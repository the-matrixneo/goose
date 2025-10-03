import { Recipe } from '../api';

/**
 * Agent initialization state enum
 */
export enum AgentState {
  UNINITIALIZED = 'uninitialized',
  INITIALIZING = 'initializing',
  NO_PROVIDER = 'no_provider',
  INITIALIZED = 'initialized',
  ERROR = 'error',
}

/**
 * Reset behavior options for chat initialization
 */
export interface ResetOptions {
  /** Reset the agent session (creates new session ID) */
  resetSession?: boolean;
  /** Clear all messages from chat */
  clearMessages?: boolean;
  /** Clear recipe configuration */
  clearRecipe?: boolean;
  /** Clear recipe parameters */
  clearRecipeParameters?: boolean;
}

/**
 * Context required for agent initialization
 */
export interface InitializationContext {
  /** Recipe configuration to initialize with */
  recipe?: Recipe;
  /** Session ID to resume from */
  resumeSessionId?: string;
  /** Callback to set waiting message during initialization */
  setAgentWaitingMessage: (msg: string | null) => void;
  /** Callback to set extensions loading state */
  setIsExtensionsLoading?: (isLoading: boolean) => void;
  /** Reset behavior options */
  resetOptions?: ResetOptions;
}

/**
 * Agent initialization hook return type
 */
export interface UseAgentInitializationReturn {
  /** Current agent state */
  agentState: AgentState;
  /** Current session ID if initialized */
  sessionId: string | null;
  /** Reset initialization state */
  resetInitialization: () => void;
  /** Initialize agent with given context */
  initializeAgent: (context: InitializationContext) => Promise<import('../types/chat').ChatType>;
}

/**
 * Main agent hook return type
 */
export interface UseAgentReturn {
  /** Current agent state */
  agentState: AgentState;
  /** Load current chat with initialization context */
  loadCurrentChat: (context: InitializationContext) => Promise<import('../types/chat').ChatType>;
  /** Reset chat for new conversation (clears messages, keeps recipe) */
  resetForNewConversation: (
    context: Omit<InitializationContext, 'resetOptions'>
  ) => Promise<import('../types/chat').ChatType>;
  /** Reset chat for new recipe (clears everything, loads new recipe) */
  resetForNewRecipe: (
    context: Omit<InitializationContext, 'resetOptions'>
  ) => Promise<import('../types/chat').ChatType>;
}

/**
 * Agent-related error types
 */
export class NoProviderOrModelError extends Error {
  constructor() {
    super('No provider or model configured');
    this.name = this.constructor.name;
  }
}

export class AgentInitializationError extends Error {
  constructor(
    message: string,
    public readonly cause?: Error
  ) {
    super(message);
    this.name = this.constructor.name;
  }
}

export class ConfigurationError extends Error {
  constructor(
    message: string,
    public readonly cause?: Error
  ) {
    super(message);
    this.name = this.constructor.name;
  }
}
