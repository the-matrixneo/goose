import { useState, useCallback, useEffect } from 'react';
import { useConfig } from '../components/ConfigContext';
import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { initializeSystem } from '../utils/providerUtils';
import { initializeCostDatabase } from '../utils/costDatabase';
import {
  backupConfig,
  initConfig,
  Message as ApiMessage,
  readAllConfig,
  recoverConfig,
  resumeAgent,
  startAgent,
  validateConfig,
} from '../api';
import { COST_TRACKING_ENABLED } from '../updates';
import { convertApiMessageToFrontendMessage } from '../components/context_management';
import { fetchSessionDetails } from '../sessions';

export enum AgentState {
  UNINITIALIZED = 'uninitialized',
  INITIALIZING = 'initializing',
  NO_PROVIDER = 'no_provider',
  INITIALIZED = 'initialized',
  ERROR = 'error',
}

export interface InitializationContext {
  recipeConfig?: Recipe;
  resumeSessionId?: string;
}

interface UseAgentReturn {
  agentState: AgentState;
  agentStateMessage: string | null;
  resetChat: () => void;
  chat: ChatType | null;
}

function wrapWithAbortSignal<T extends Array<unknown>, U>(
  func: (...args: T) => U,
  signal: AbortController['signal']
) {
  return (...args: T): U => {
    if (signal.aborted) {
      throw new Error('Aborted');
    }
    return func(...args);
  };
}

export function useAgent({ recipeConfig, resumeSessionId }: InitializationContext): UseAgentReturn {
  const [agentState, setAgentState] = useState<AgentState>(AgentState.UNINITIALIZED);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [chat, setChat] = useState<ChatType | null>(null);
  const [agentStateMessage, setAgentStateMessage] = useState<string | null>(null);

  const { getExtensions, addExtension, read } = useConfig();

  const resetChat = useCallback(() => {
    setSessionId(null);
    setAgentState(AgentState.UNINITIALIZED);
  }, []);

  const agentIsInitialized = agentState === AgentState.INITIALIZED;
  const recipeTitle = recipeConfig?.title;

  useEffect(() => {
    const abortController = new AbortController();
    (async function () {
      try {
        if (agentIsInitialized && sessionId) {
          const sessionDetails = await fetchSessionDetails(sessionId, abortController.signal);
          const chat: ChatType = {
            sessionId: sessionDetails.sessionId,
            title: sessionDetails.metadata.description || 'Chat Session',
            messageHistoryIndex: 0,
            messages: sessionDetails.messages,
          };

          chat.title = recipeTitle || chat.title;
          setChat(chat);
          return;
        }

        setAgentState(AgentState.INITIALIZING);
        setAgentStateMessage('Agent is initializing');

        const config = window.electron.getConfig();
        const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
        const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

        if (!provider || !model) {
          setAgentState(AgentState.NO_PROVIDER);
          throw new Error('No provider or model configured');
        }

        const agentResponse = resumeSessionId
          ? await resumeAgent({
              body: {
                session_id: resumeSessionId,
              },
              throwOnError: true,
              signal: abortController.signal,
            })
          : await startAgent({
              body: {
                working_dir: window.appConfig.get('GOOSE_WORKING_DIR') as string,
              },
              throwOnError: true,
              signal: abortController.signal,
            });

        const agentSessionInfo = agentResponse.data;
        if (!agentSessionInfo) {
          throw Error('Failed to get session info');
        }
        setSessionId(agentSessionInfo.session_id);

        setAgentStateMessage('Agent is loading config');

        await initConfig();

        try {
          await readAllConfig({ throwOnError: true });
        } catch (error) {
          console.warn('Initial config read failed, attempting recovery:', error);
          await handleConfigRecovery();
        }

        setAgentStateMessage('Extensions are loading');
        console.log(`calling initializeSystem with sessionId: ${agentSessionInfo.session_id}`);
        await initializeSystem(agentSessionInfo.session_id, provider as string, model as string, {
          getExtensions,
          addExtension: wrapWithAbortSignal(addExtension, abortController.signal),
        });
        console.log('init system done!!');

        if (COST_TRACKING_ENABLED) {
          try {
            await initializeCostDatabase();
          } catch (error) {
            console.error('Failed to initialize cost database:', error);
          }
        }

        let initChat: ChatType = {
          sessionId: agentSessionInfo.session_id,
          title: recipeTitle ?? agentSessionInfo.metadata.description,
          messageHistoryIndex: 0,
          messages: agentSessionInfo.messages.map((message: ApiMessage) =>
            convertApiMessageToFrontendMessage(message, true, true)
          ),
        };
        setAgentState(AgentState.INITIALIZED);

        setChat(initChat);
      } catch (error) {
        if ((error + '').startsWith('Abort')) {
          // react aborted this effect
        } else if ((error + '').includes('Failed to create provider')) {
          setAgentState(AgentState.NO_PROVIDER);
          throw error;
        } else {
          setAgentState(AgentState.ERROR);
          throw error;
        }
      } finally {
        setAgentStateMessage(null);
      }
    })();

    return () => {
      abortController.abort('Aborted: useEffect cleanup');
    };
  }, [
    getExtensions,
    addExtension,
    read,
    agentIsInitialized,
    sessionId,
    resumeSessionId,
    recipeTitle,
  ]);

  // TODO(Douwe/Jack): we should store the recipe config on the server so not needed here:
  const chatWithRecipe: ChatType | null = chat && {
    ...chat,
    recipeConfig,
  };

  return {
    agentState,
    agentStateMessage,
    resetChat,
    chat: chatWithRecipe,
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
