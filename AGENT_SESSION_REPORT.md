# Agent-Per-Session Architecture Analysis Report

## Executive Summary

This report analyzes the current Goose architecture where a single Agent instance handles multiple concurrent sessions, and explores the implications of moving to a one-agent-per-session model. The investigation reveals significant concurrency challenges in the current design and presents a comprehensive analysis of the tradeoffs involved in architectural changes.

## Current Architecture Overview

### Single Agent, Multiple Sessions Model

The current Goose architecture employs a singleton Agent pattern where:
- **One Agent instance** (`Arc<Agent>`) is shared across all sessions in goose-server/goosed
- **AppState** holds this single agent reference and passes it to all request handlers
- Each incoming request spawns an async task that calls the shared agent's `reply()` method
- Sessions are differentiated only by their `SessionConfig` parameters

### Agent Structure

The Agent struct contains numerous fields protected by Mutex locks:

```rust
pub struct Agent {
    provider: Mutex<Option<Arc<dyn Provider>>>,           // LLM provider
    extension_manager: ExtensionManager,                  // MCP extensions
    sub_recipe_manager: Mutex<SubRecipeManager>,         // Sub-recipes
    tasks_manager: TasksManager,                         // Task management
    final_output_tool: Arc<Mutex<Option<FinalOutputTool>>>, // Recipe output
    frontend_tools: Mutex<HashMap<String, FrontendTool>>, // Frontend tools
    prompt_manager: Mutex<PromptManager>,                // System prompts
    confirmation_tx/rx: mpsc channels,                   // Permissions
    tool_result_tx/rx: mpsc channels,                    // Tool results
    tool_monitor: Arc<Mutex<Option<ToolMonitor>>>,       // Tool monitoring
    tool_route_manager: ToolRouteManager,                // Tool routing
    scheduler_service: Mutex<Option<Arc<dyn SchedulerTrait>>>, // Scheduler
    retry_manager: RetryManager,                         // Retry logic
}
```

### Session Management

Sessions are lightweight and file-based:
- **SessionConfig**: Contains session ID, working directory, and execution parameters
- **Session Storage**: JSONL files with metadata header and message history
- **Persistence**: Messages saved after each interaction completes
- **Isolation**: Session data is isolated, but agent state is shared

## Identified Concurrency Issues

### 1. Extension State Conflicts

The `ExtensionManager` maintains a single set of MCP client connections shared across all sessions:

```rust
pub struct ExtensionManager {
    extensions: Mutex<HashMap<String, Extension>>,
}
```

**Problems:**
- Adding/removing extensions affects all active sessions
- Extension state changes (e.g., authentication) impact all users
- Resource limits are shared (file handles, memory, connections)
- One session's extension failure can affect others

### 2. Tool Monitor Interference

The tool monitor tracks repetition to prevent infinite loops, but it's shared:

```rust
tool_monitor: Arc<Mutex<Option<ToolMonitor>>>,
```

**Problems:**
- Tool usage counts from one session affect another's limits
- Reset operations impact all sessions
- False positives for repetition detection across different contexts

### 3. Channel Message Interleaving

Permission confirmations and tool results use shared channels:

```rust
confirmation_tx/rx: mpsc channels,
tool_result_tx/rx: mpsc channels,
```

**Problems:**
- Messages from different sessions can interleave
- Race conditions in permission handling
- Tool results may be delivered to wrong session handlers
- Difficult to trace message flow in concurrent scenarios

### 4. Recipe and Sub-Recipe Conflicts

Recipe-related state is global to the agent:

```rust
final_output_tool: Arc<Mutex<Option<FinalOutputTool>>>,
sub_recipe_manager: Mutex<SubRecipeManager>,
```

**Problems:**
- One session's recipe execution affects another
- Final output tools persist across sessions
- Sub-recipe registration is global
- Recipe state leaks between sessions

### 5. Provider and Configuration Changes

The LLM provider and configuration are shared:

```rust
provider: Mutex<Option<Arc<dyn Provider>>>,
```

**Problems:**
- Provider updates affect all active sessions mid-conversation
- Model changes impact ongoing interactions
- Temperature and other settings are global
- No per-session provider customization possible

## One-Agent-Per-Session Design Analysis

### Proposed Architecture

Instead of a single shared Agent, each session would create its own Agent instance:

```rust
pub struct AppState {
    agents: Arc<Mutex<HashMap<SessionId, Arc<Agent>>>>,
    // ... other fields
}
```

### Benefits

#### 1. Complete Session Isolation
- No shared state between sessions
- Clean extension state per session
- Independent tool monitoring
- Isolated failure domains

#### 2. Elimination of Lock Contention
- No mutex contention between sessions
- True parallel execution
- Better CPU utilization
- Improved response times under load

#### 3. Enhanced Security
- Session-specific permissions
- No cross-session data leaks
- Better audit trails
- Isolated security contexts

#### 4. Improved Flexibility
- Per-session provider configuration
- Session-specific extensions
- Custom tool sets per user
- Dynamic resource allocation

#### 5. Better Multi-User Support
- True multi-tenancy
- User-specific configurations
- Isolated resource quotas
- Fair scheduling

### Challenges

#### 1. Increased Memory Usage

**Estimated overhead per agent:**
- Base Agent struct: ~500 bytes
- Extension connections: 1-10MB per extension
- Provider instance: 1-5MB
- Channels and buffers: ~100KB
- **Total: 5-20MB per agent instance**

For 100 concurrent sessions: 500MB - 2GB additional memory

#### 2. Resource Multiplication

**MCP Connections:**
- Each session needs separate extension connections
- More file descriptors (limit: typically 1024-4096)
- More network connections
- More subprocess instances

**Example with 10 extensions, 50 sessions:**
- Current: 10 connections
- Per-session: 500 connections

#### 3. Slower Session Initialization

**Startup overhead:**
- Extension initialization: 100-500ms per extension
- Provider setup: 50-100ms
- Channel creation: ~1ms
- **Total: 200ms - 5s depending on extensions**

#### 4. Complex Lifecycle Management

**New requirements:**
- Agent pool management
- Garbage collection for inactive sessions
- Resource limit enforcement
- Connection pooling strategies

### Implementation Considerations

#### 1. Agent Pool Pattern

```rust
pub struct AgentPool {
    max_agents: usize,
    idle_timeout: Duration,
    agents: HashMap<SessionId, (Arc<Agent>, Instant)>,
}
```

- Limit maximum concurrent agents
- Reuse agents when possible
- Clean up idle agents
- Pre-warm agents for faster startup

#### 2. Shared Resource Optimization

Some resources could remain shared:
- Provider connections (with request routing)
- Read-only extension data
- Static configuration
- Tool definitions

#### 3. Hybrid Approach

Consider a hybrid model:
- Shared extensions for stateless tools
- Per-session extensions for stateful operations
- Copy-on-write for configuration
- Lazy initialization of resources

#### 4. Migration Strategy

Phased implementation:
1. Refactor Agent to be cloneable
2. Implement agent pool management
3. Add per-session agent creation
4. Migrate extensions to per-session model
5. Optimize resource sharing

## Recommendations

### Short Term (Current Architecture Improvements)

1. **Fix Channel Interleaving**
   - Add session ID to channel messages
   - Implement message routing layer
   - Use separate channels per session

2. **Improve Lock Granularity**
   - Break up large mutex-protected structures
   - Use RwLock where appropriate
   - Reduce lock hold times

3. **Add Session Context**
   - Pass session context through tool calls
   - Implement session-aware tool monitoring
   - Add session ID to log entries

### Long Term (Architecture Evolution)

1. **Implement Agent Pool**
   - Start with a configurable pool size
   - Add metrics for pool utilization
   - Implement agent recycling

2. **Gradual Migration**
   - Begin with stateless components
   - Move to per-session extensions
   - Finally migrate provider instances

3. **Resource Management**
   - Implement connection pooling
   - Add resource quotas per session
   - Monitor and enforce limits

4. **Configuration Options**
   - Make architecture configurable
   - Allow single-agent mode for resource-constrained environments
   - Support both models during transition

## Conclusion

The current single-agent architecture presents significant concurrency challenges that limit Goose's ability to serve multiple users simultaneously without interference. While moving to a one-agent-per-session model would resolve these issues and provide better isolation, security, and scalability, it comes with increased resource costs and implementation complexity.

The recommended approach is a phased migration that:
1. First addresses the most critical concurrency issues in the current architecture
2. Implements an agent pool pattern for gradual transition
3. Provides configuration options to support different deployment scenarios
4. Optimizes resource sharing where possible while maintaining session isolation

This evolution would enable Goose to better support multi-user scenarios, improve reliability, and provide a foundation for future scaling requirements while managing resource consumption effectively.
