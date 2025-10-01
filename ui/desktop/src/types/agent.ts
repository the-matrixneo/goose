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
 * Context required for agent initialization
 */
export interface InitializationContext {
  /** Recipe configuration to initialize with */
  recipeConfig?: Recipe;
  /** Session ID to resume from */
  resumeSessionId?: string;
  /** Callback to set waiting message during initialization */
  setAgentWaitingMessage: (msg: string | null) => void;
  /** Callback to set extensions loading state */
  setIsExtensionsLoading?: (isLoading: boolean) => void;
  /** Force reset of existing session */
  forceReset?: boolean;
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
  /** Reset chat and agent state */
  resetChat: () => void;
  /** Load current chat with initialization context */
  loadCurrentChat: (context: InitializationContext) => Promise<import('../types/chat').ChatType>;
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
