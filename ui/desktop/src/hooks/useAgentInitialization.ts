import { useCallback, useRef, useState } from 'react';
import { useConfig } from '../components/ConfigContext';
import { initializeSystem } from '../utils/providerUtils';
import { initializeCostDatabase } from '../utils/costDatabase';
import {
  backupConfig,
  initConfig,
  readAllConfig,
  Recipe,
  recoverConfig,
  resumeAgent,
  startAgent,
  validateConfig,
} from '../api';
import { COST_TRACKING_ENABLED } from '../updates';
import { convertApiMessageToFrontendMessage } from '../components/context_management';
import { ChatType } from '../types/chat';
import {
  AgentState,
  InitializationContext,
  UseAgentInitializationReturn,
  NoProviderOrModelError,
  AgentInitializationError,
  ConfigurationError,
} from '../types/agent';

/**
 * Hook for managing agent initialization state and process
 * Extracted from useAgent to reduce complexity and improve testability
 */
export function useAgentInitialization(): UseAgentInitializationReturn {
  const [agentState, setAgentState] = useState<AgentState>(AgentState.UNINITIALIZED);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const initPromiseRef = useRef<Promise<ChatType> | null>(null);
  const [recipeFromAppConfig, setRecipeFromAppConfig] = useState<Recipe | null>(
    (window.appConfig.get('recipe') as Recipe) || null
  );

  const { getExtensions, addExtension, read } = useConfig();

  const resetInitialization = useCallback(() => {
    setSessionId(null);
    setAgentState(AgentState.UNINITIALIZED);
    setRecipeFromAppConfig(null);
    initPromiseRef.current = null;
  }, []);

  const performInitialization = useCallback(
    async (initContext: InitializationContext): Promise<ChatType> => {
      try {
        setAgentState(AgentState.INITIALIZING);
        initContext.setAgentWaitingMessage('Agent is initializing');

        // Validate initialization context
        if (!initContext.setAgentWaitingMessage) {
          throw new AgentInitializationError(
            'Invalid initialization context: missing setAgentWaitingMessage'
          );
        }

        // Check provider and model configuration
        const config = window.electron.getConfig();
        const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
        const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

        if (!provider || !model) {
          setAgentState(AgentState.NO_PROVIDER);
          throw new NoProviderOrModelError();
        }

        // Start or resume agent session
        let agentResponse;
        try {
          agentResponse = initContext.resumeSessionId
            ? await resumeAgent({
                body: { session_id: initContext.resumeSessionId },
                throwOnError: true,
              })
            : await startAgent({
                body: {
                  working_dir: window.appConfig.get('GOOSE_WORKING_DIR') as string,
                  recipe: recipeFromAppConfig ?? initContext.recipeConfig,
                },
                throwOnError: true,
              });
        } catch (error) {
          throw new AgentInitializationError(
            `Failed to ${initContext.resumeSessionId ? 'resume' : 'start'} agent session`,
            error as Error
          );
        }

        const agentSession = agentResponse.data;
        if (!agentSession) {
          throw new AgentInitializationError('Failed to get session info from agent response');
        }
        setSessionId(agentSession.id);

        // Initialize configuration with proper error handling
        initContext.setAgentWaitingMessage('Agent is loading config');
        try {
          await initConfig();
          await readAllConfig({ throwOnError: true });
        } catch (error) {
          console.warn('Initial config read failed, attempting recovery:', error);
          try {
            await handleConfigRecovery();
          } catch (recoveryError) {
            throw new ConfigurationError(
              'Failed to initialize or recover configuration',
              recoveryError as Error
            );
          }
        }

        // Initialize system and extensions with error boundary
        initContext.setAgentWaitingMessage('Extensions are loading');
        try {
          const recipeConfigForInit = initContext.recipeConfig || agentSession.recipe || undefined;
          await initializeSystem(agentSession.id, provider as string, model as string, {
            getExtensions,
            addExtension,
            setIsExtensionsLoading: initContext.setIsExtensionsLoading,
            recipeParameters: agentSession.recipe_parameters,
            recipeConfig: recipeConfigForInit,
          });
        } catch (error) {
          throw new AgentInitializationError(
            'Failed to initialize system and extensions',
            error as Error
          );
        }

        // Initialize cost tracking if enabled (non-critical, don't fail initialization)
        if (COST_TRACKING_ENABLED) {
          try {
            await initializeCostDatabase();
          } catch (error) {
            console.error('Failed to initialize cost database (non-critical):', error);
          }
        }

        // Prepare chat data - trust what the agent returns
        // For start_agent: agent returns the recipe we provided + empty conversation
        // For resume_agent: agent returns existing recipe + existing conversation
        const conversation = agentSession.conversation || [];
        const messages = conversation.map(convertApiMessageToFrontendMessage);

        const initChat: ChatType = {
          sessionId: agentSession.id,
          title: agentSession.recipe?.title || agentSession.description || 'New Session',
          messageHistoryIndex: 0,
          messages: messages,
          recipeConfig: agentSession.recipe, // Always trust what the agent returns
          recipeParameters: agentSession.recipe_parameters || null,
        };

        setAgentState(AgentState.INITIALIZED);
        return initChat;
      } catch (error) {
        // Improved error classification
        if (error instanceof NoProviderOrModelError) {
          setAgentState(AgentState.NO_PROVIDER);
        } else if (
          error instanceof AgentInitializationError ||
          error instanceof ConfigurationError
        ) {
          setAgentState(AgentState.ERROR);
        } else if ((error + '').includes('Failed to create provider')) {
          setAgentState(AgentState.NO_PROVIDER);
        } else {
          setAgentState(AgentState.ERROR);
        }
        throw error;
      } finally {
        initContext.setAgentWaitingMessage(null);
        initPromiseRef.current = null;
      }
    },
    [read, recipeFromAppConfig, getExtensions, addExtension]
  );

  const initializeAgent = useCallback(
    async (initContext: InitializationContext): Promise<ChatType> => {
      // Handle force reset
      if (initContext.forceReset) {
        resetInitialization();
      }

      // Return existing session if already initialized and not forcing reset
      if (agentState === AgentState.INITIALIZED && sessionId && !initContext.forceReset) {
        const agentResponse = await resumeAgent({
          body: { session_id: sessionId },
          throwOnError: true,
        });

        const agentSession = agentResponse.data;
        const messages = agentSession.conversation || [];
        return {
          sessionId: agentSession.id,
          title: agentSession.recipe?.title || agentSession.description,
          messageHistoryIndex: 0,
          messages: messages?.map(convertApiMessageToFrontendMessage),
          recipeConfig: agentSession.recipe,
          recipeParameters: agentSession.recipe_parameters || null,
        };
      }

      // Return existing initialization promise if in progress
      if (initPromiseRef.current) {
        return initPromiseRef.current;
      }

      // Start new initialization
      const initPromise = performInitialization(initContext);
      initPromiseRef.current = initPromise;
      return initPromise;
    },
    [agentState, sessionId, resetInitialization, performInitialization]
  );

  return {
    agentState,
    sessionId,
    resetInitialization,
    initializeAgent,
  };
}

const handleConfigRecovery = async () => {
  const configVersion = localStorage.getItem('configVersion');
  const shouldMigrateExtensions = !configVersion || parseInt(configVersion, 10) < 3;

  if (shouldMigrateExtensions) {
    try {
      await backupConfig({ throwOnError: true });
      await initConfig();
    } catch (migrationError) {
      console.error('Migration failed:', migrationError);
    }
  }

  try {
    await validateConfig({ throwOnError: true });
    await readAllConfig({ throwOnError: true });
  } catch {
    try {
      await recoverConfig({ throwOnError: true });
      await readAllConfig({ throwOnError: true });
    } catch {
      console.warn('Config recovery failed, reinitializing...');
      await initConfig();
    }
  }
};
