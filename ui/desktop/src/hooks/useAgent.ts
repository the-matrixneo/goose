import { useState, useCallback } from 'react';
import { useConfig } from '../components/ConfigContext';
import { generateSessionId } from '../sessions';
import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { initializeSystem } from '../utils/providerUtils';
import { initializeCostDatabase } from '../utils/costDatabase';
import { backupConfig, initConfig, readAllConfig, recoverConfig, validateConfig } from '../api';
import { COST_TRACKING_ENABLED } from '../updates';

interface InitializationContext {
  recipeConfig?: Recipe;
  resumedChat?: ChatType;
}

interface UseAgentReturn {
  isAgentInitialized: boolean;
  isInitializing: boolean;
  initializeAgentIfNeeded: (context: InitializationContext) => Promise<void>;
}

export function useAgent(setPairChat: (chat: ChatType) => void): UseAgentReturn {
  const [isAgentInitialized, setIsAgentInitialized] = useState(false);
  const [isInitializing, setIsInitializing] = useState(false);

  const { getExtensions, addExtension, read } = useConfig();

  const initializeAgentIfNeeded = useCallback(
    async (initContext: InitializationContext) => {
      if (isAgentInitialized || isInitializing) {
        return;
      }

      setIsInitializing(true);

      try {
        const config = window.electron.getConfig();
        const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
        const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

        let chatToSet: ChatType;

        if (initContext.recipeConfig) {
          chatToSet = {
            sessionId: generateSessionId(),
            title: initContext.recipeConfig?.title || 'Recipe Chat',
            messages: [],
            messageHistoryIndex: 0,
            recipeConfig: initContext.recipeConfig,
            recipeParameters: null,
          };
        } else if (initContext.resumedChat) {
          chatToSet = initContext.resumedChat;
        } else {
          chatToSet = {
            sessionId: generateSessionId(),
            title: 'New Chat',
            messages: [],
            messageHistoryIndex: 0,
            recipeConfig: null,
            recipeParameters: null,
          };
        }

        setPairChat(chatToSet);

        // Initialize cost database if enabled
        const costDbPromise = COST_TRACKING_ENABLED
          ? initializeCostDatabase().catch((error) => {
              console.error('Failed to initialize cost database:', error);
            })
          : (() => {
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
          const initPromises = [
            initializeSystem(chatToSet.sessionId, provider as string, model as string, {
              getExtensions,
              addExtension,
            }),
          ];

          if (COST_TRACKING_ENABLED && costDbPromise) {
            initPromises.push(costDbPromise);
          }

          await Promise.all(initPromises);
          console.log('Agent core initialization completed successfully');
        } else {
          throw new Error('Provider and model are required for agent initialization');
        }

        setIsAgentInitialized(true);
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
  };
}

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
