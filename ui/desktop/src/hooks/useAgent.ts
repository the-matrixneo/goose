import { useCallback } from 'react';
import { ChatType } from '../types/chat';
import { useAgentInitialization } from './useAgentInitialization';
import {
  AgentState,
  InitializationContext,
  UseAgentReturn,
  NoProviderOrModelError,
  AgentInitializationError,
  ConfigurationError,
} from '../types/agent';

export { AgentState, NoProviderOrModelError, AgentInitializationError, ConfigurationError };
export type { InitializationContext };

export function useAgent(): UseAgentReturn {
  const { agentState, initializeAgent } = useAgentInitialization();

  const loadCurrentChat = useCallback(
    async (context: InitializationContext): Promise<ChatType> => {
      return initializeAgent(context);
    },
    [initializeAgent]
  );

  const resetForNewConversation = useCallback(
    async (context: Omit<InitializationContext, 'resetOptions'>): Promise<ChatType> => {
      return initializeAgent({
        ...context,
        resetOptions: {
          resetSession: true,
          clearMessages: true,
          clearRecipeParameters: true,
        },
      });
    },
    [initializeAgent]
  );

  const resetForNewRecipe = useCallback(
    async (context: Omit<InitializationContext, 'resetOptions'>): Promise<ChatType> => {
      return initializeAgent({
        ...context,
        resetOptions: {
          resetSession: true,
          clearMessages: true,
          clearRecipe: true,
          clearRecipeParameters: true,
        },
      });
    },
    [initializeAgent]
  );

  return {
    agentState,
    loadCurrentChat,
    resetForNewConversation,
    resetForNewRecipe,
  };
}
