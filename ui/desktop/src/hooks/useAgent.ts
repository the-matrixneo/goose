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
  const { agentState, resetInitialization, initializeAgent } = useAgentInitialization();

  const resetChat = useCallback(() => {
    resetInitialization();
  }, [resetInitialization]);

  const loadCurrentChat = useCallback(
    async (context: InitializationContext): Promise<ChatType> => {
      return initializeAgent(context);
    },
    [initializeAgent]
  );

  return {
    agentState,
    resetChat,
    loadCurrentChat,
  };
}
