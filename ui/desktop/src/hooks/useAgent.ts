import { useState, useRef, useCallback } from 'react';
import { useConfig } from '../components/ConfigContext';
import { initializeAgent } from '../utils/appInitialization';
import { generateSessionId } from '../sessions';
import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';

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

        // Set up the pair chat based on the context
        let chatToSet: ChatType;

        switch (initContext.mode) {
          case 'resume':
            if (!initContext.resumedChat) {
              throw new Error('Resume mode requires resumedChat');
            }
            chatToSet = initContext.resumedChat;
            break;

          case 'recipe':
            if (!initContext.recipeConfig) {
              throw new Error('Recipe mode requires recipeConfig');
            }
            chatToSet = {
              sessionId: generateSessionId(),
              title: initContext.recipeConfig.title || 'Recipe Chat',
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

        await initializeAgent({
          getExtensions,
          addExtension,
          setPairChat,
          provider: provider as string,
          model: model as string,
        });

        setIsAgentInitialized(true);
        console.log('Agent initialized successfully with context:', initContext);
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error occurred';
        console.error('Fatal error during agent initialization:', err);
        setError(errorMessage);
        // Reset the attempt flag so user can retry
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
