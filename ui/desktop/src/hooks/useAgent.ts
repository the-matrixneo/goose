import { useState, useRef, useCallback } from 'react';
import { useConfig } from '../components/ConfigContext';
import { generateSessionId } from '../sessions';
import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { initializeSystem } from '../utils/providerUtils';
import { initializeCostDatabase } from '../utils/costDatabase';
import {
  type ExtensionConfig,
  type FixedExtensionEntry,
  MalformedConfigError,
} from '../components/ConfigContext';
import { backupConfig, initConfig, readAllConfig, recoverConfig, validateConfig } from '../api';
import { COST_TRACKING_ENABLED } from '../updates';

type InitializationMode = 'new' | 'recipe' | 'resume';

interface InitializationContext {
  mode: InitializationMode;
  recipeConfig?: Recipe;
  resumedChat?: ChatType;
}

interface UseAgentReturn {
  isAgentInitialized: boolean;
  isInitializing: boolean;
  initializeAgentIfNeeded: (context: InitializationContext) => Promise<void>;
  error: string | null;
}

export function useAgent(setPairChat: (chat: ChatType) => void): UseAgentReturn {
  const [isAgentInitialized, setIsAgentInitialized] = useState(false);
  const [isInitializing, setIsInitializing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const initAttemptedRef = useRef(false);

  const { getExtensions, addExtension, read } = useConfig();

  const initializeAgentIfNeeded = useCallback(
    async (initContext: InitializationContext) => {
      if (isAgentInitialized || isInitializing || initAttemptedRef.current) {
        return;
      }

      initAttemptedRef.current = true;
      setIsInitializing(true);
      setError(null);

      try {
        const config = window.electron.getConfig();
        const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
        const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

        let chatToSet: ChatType;

        switch (initContext.mode) {
          case 'resume':
            if (!initContext.resumedChat) {
              throw new Error('Resume mode requires resumedChat');
            }
            chatToSet = initContext.resumedChat;
            break;

          case 'recipe':
            chatToSet = {
              sessionId: generateSessionId(),
              title: initContext.recipeConfig?.title || 'Recipe Chat',
              messages: [],
              messageHistoryIndex: 0,
              recipeConfig: initContext.recipeConfig,
              recipeParameters: null,
            };
            break;

          case 'new':
          default:
            chatToSet = {
              sessionId: generateSessionId(),
              title: 'New Chat',
              messages: [],
              messageHistoryIndex: 0,
              recipeConfig: null,
              recipeParameters: null,
            };
            break;
        }

        setPairChat(chatToSet);

        await initializeAgentCore(
          chatToSet.sessionId,
          getExtensions,
          addExtension,
          provider as string,
          model as string
        );

        setIsAgentInitialized(true);
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error occurred';
        console.error('Fatal error during agent initialization:', err);
        setError(errorMessage);
        initAttemptedRef.current = false;
      } finally {
        setIsInitializing(false);
      }
    },
    [isAgentInitialized, isInitializing, getExtensions, addExtension, read, setPairChat]
  );

  return {
    isAgentInitialized,
    isInitializing,
    initializeAgentIfNeeded,
    error,
  };
}

const initializeAgentCore = async (
  sessionId: string,
  getExtensions: (b: boolean) => Promise<FixedExtensionEntry[]>,
  addExtension: (name: string, config: ExtensionConfig, enabled: boolean) => Promise<void>,
  provider: string,
  model: string
) => {
  console.log(`Initializing agent core with provider: ${provider}, model: ${model}`);

  // Initialize cost database if enabled
  const costDbPromise = COST_TRACKING_ENABLED
    ? initializeCostDatabase().catch((error) => {
        console.error('Failed to initialize cost database:', error);
      })
    : await (() => {
        console.log('Cost tracking disabled, skipping cost database initialization');
        return Promise.resolve();
      })();

  await initConfig();

  try {
    await readAllConfig({ throwOnError: true });
  } catch (error) {
    console.warn('Initial config read failed, attempting recovery:', error);
    await handleConfigRecovery();
  }

  // Initialize the provider system and extensions
  if (provider && model) {
    try {
      const initPromises = [
        initializeSystem(sessionId, provider, model, {
          getExtensions,
          addExtension,
        }),
      ];

      if (COST_TRACKING_ENABLED && costDbPromise) {
        initPromises.push(costDbPromise);
      }

      await Promise.all(initPromises);
      console.log('Agent core initialization completed successfully');
    } catch (error) {
      console.error('Error in system initialization:', error);
      if (error instanceof MalformedConfigError) {
        throw error;
      }
      throw error;
    }
  } else {
    throw new Error('Provider and model are required for agent initialization');
  }
};

const handleConfigRecovery = async () => {
  const configVersion = localStorage.getItem('configVersion');
  const shouldMigrateExtensions = !configVersion || parseInt(configVersion, 10) < 3;

  if (shouldMigrateExtensions) {
    console.log('Performing extension migration...');
    try {
      await backupConfig({ throwOnError: true });
      await initConfig();
    } catch (migrationError) {
      console.error('Migration failed:', migrationError);
    }
  }

  console.log('Attempting config recovery...');
  try {
    await validateConfig({ throwOnError: true });
    await readAllConfig({ throwOnError: true });
  } catch {
    console.log('Config validation failed, attempting recovery...');
    try {
      await recoverConfig({ throwOnError: true });
      await readAllConfig({ throwOnError: true });
    } catch {
      console.warn('Config recovery failed, reinitializing...');
      await initConfig();
    }
  }
};
