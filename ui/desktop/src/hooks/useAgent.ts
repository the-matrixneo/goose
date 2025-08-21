import { useState, useCallback, useRef } from 'react';
import { useConfig } from '../components/ConfigContext';
import { generateSessionId } from '../sessions';
import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { initializeSystem } from '../utils/providerUtils';
import { initializeCostDatabase } from '../utils/costDatabase';
import { backupConfig, initConfig, readAllConfig, recoverConfig, validateConfig } from '../api';
import { COST_TRACKING_ENABLED } from '../updates';

export enum AgentState {
  UNINITIALIZED = 'uninitialized',
  INITIALIZING = 'initializing',
  NO_PROVIDER = 'no_provider',
  INITIALIZED = 'initialized',
  ERROR = 'error',
}

export interface InitializationContext {
  recipeConfig?: Recipe | null;
  resumedChat?: ChatType | null;
  initialMessage?: string | null;
  resetChat?: boolean;
}

interface UseAgentReturn {
  agentState: AgentState;
  initializeAgentIfNeeded: (context: InitializationContext) => Promise<void>;
}

export function useAgent(setChat: (chat: ChatType) => void): UseAgentReturn {
  const [agentState, setAgentState] = useState<AgentState>(AgentState.UNINITIALIZED);
  const initPromiseRef = useRef<Promise<void> | null>(null);

  const { getExtensions, addExtension, read } = useConfig();

  const initializeAgentIfNeeded = useCallback(
    async (initContext: InitializationContext) => {
      if (agentState === AgentState.INITIALIZED) {
        return;
      }

      if (initPromiseRef.current) {
        return initPromiseRef.current;
      }

      const initPromise = (async () => {
        setAgentState(AgentState.INITIALIZING);

        try {
          const config = window.electron.getConfig();
          const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
          const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

          if (!provider || !model) {
            setAgentState(AgentState.NO_PROVIDER);
            return;
          }

          let chatToSet: ChatType;
          if (initContext.resumedChat) {
            chatToSet = initContext.resumedChat;
          } else if (initContext.resetChat && initContext.recipeConfig) {
            chatToSet = {
              sessionId: generateSessionId(),
              title: initContext.recipeConfig.title || 'Recipe Chat',
              messages: [],
              messageHistoryIndex: 0,
              recipeConfig: initContext.recipeConfig,
              recipeParameters: null,
            };
          } else if (initContext.resetChat && initContext.initialMessage) {
            chatToSet = {
              sessionId: generateSessionId(),
              title: 'New Chat',
              messages: [],
              messageHistoryIndex: 0,
              recipeConfig: null,
              recipeParameters: null,
            };
          } else if (initContext.recipeConfig) {
            chatToSet = {
              sessionId: generateSessionId(),
              title: initContext.recipeConfig.title || 'Recipe Chat',
              messages: [],
              messageHistoryIndex: 0,
              recipeConfig: initContext.recipeConfig,
              recipeParameters: null,
            };
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

          setChat(chatToSet);

          const costDbPromise = COST_TRACKING_ENABLED
            ? initializeCostDatabase().catch((error) => {
                console.error('Failed to initialize cost database:', error);
              })
            : Promise.resolve();

          await initConfig();

          try {
            await readAllConfig({ throwOnError: true });
          } catch (error) {
            console.warn('Initial config read failed, attempting recovery:', error);
            await handleConfigRecovery();
          }

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

          setAgentState(AgentState.INITIALIZED);
        } catch (error) {
          if ((error + '').includes('Failed to create provider')) {
            // This is not ideal, but otherwise we end up showing a fatal error instead of
            // allowing the user to fix their config:
            setAgentState(AgentState.NO_PROVIDER);
            return;
          }
          setAgentState(AgentState.ERROR);
          throw error;
        } finally {
          initPromiseRef.current = null;
        }
      })();

      initPromiseRef.current = initPromise;
      return initPromise;
    },
    [agentState, getExtensions, addExtension, read, setChat]
  );

  return {
    agentState,
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
