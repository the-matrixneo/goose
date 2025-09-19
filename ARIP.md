# Agent Research In Progress (ARIP)

## Todo List

- [x] Examine current Agent struct implementation in goose crate
  - [x] Identify core Agent struct and its fields
  - [x] Understand lifecycle management
  - [x] Document current state management approach
- [ ] Analyze Session management
  - [x] How sessions are created and managed
  - [x] Relationship between Agent and Session
  - [x] Session state and persistence
- [x] Study goose-server/goosed integration
  - [x] How Agent is instantiated in server
  - [x] Request routing to sessions
  - [x] Concurrency and locking mechanisms
- [x] Research MCP/Extension architecture
  - [x] How extensions are loaded and managed
  - [x] Per-agent vs per-session extension state
  - [x] Resource sharing implications
- [x] Identify concurrency/locking issues
  - [x] Current mutex/lock usage
  - [x] Shared state between sessions
  - [x] Potential race conditions
- [x] Design considerations for one-agent-per-session
  - [x] Memory implications
  - [x] Resource management
  - [x] Extension lifecycle
  - [x] Performance impact
- [x] Write comprehensive report

## Notes

### Initial Observations
- Starting research on agent architecture in goose codebase
- Focus on understanding current single-agent-multiple-sessions model
- Need to identify all shared state and concurrency controls

### Agent Structure (crates/goose/src/agents/agent.rs)
- **Core Agent struct** contains multiple Mutex-protected fields:
  - `provider: Mutex<Option<Arc<dyn Provider>>>` - LLM provider
  - `extension_manager: ExtensionManager` - Manages MCP extensions
  - `sub_recipe_manager: Mutex<SubRecipeManager>` - Sub-recipe handling
  - `tasks_manager: TasksManager` - Task/subagent management
  - `final_output_tool: Arc<Mutex<Option<FinalOutputTool>>>` - Recipe output
  - `frontend_tools: Mutex<HashMap<String, FrontendTool>>` - Frontend tools
  - `frontend_instructions: Mutex<Option<String>>` - Frontend prompts
  - `prompt_manager: Mutex<PromptManager>` - System prompts
  - `confirmation_tx/rx: mpsc channels` - Permission confirmations
  - `tool_result_tx/rx: mpsc channels` - Tool results
  - `tool_monitor: Arc<Mutex<Option<ToolMonitor>>>` - Tool repetition monitoring
  - `tool_route_manager: ToolRouteManager` - Tool routing/indexing
  - `scheduler_service: Mutex<Option<Arc<dyn SchedulerTrait>>>` - Scheduler
  - `retry_manager: RetryManager` - Retry logic

### Session Management (crates/goose/src/session/)
- **SessionConfig** struct defines session parameters:
  - `id: Identifier` - Session ID
  - `working_dir: PathBuf` - Working directory
  - `schedule_id: Option<String>` - For scheduled jobs
  - `execution_mode: Option<String>` - foreground/background
  - `max_turns: Option<u32>` - Max iterations
  - `retry_config: Option<RetryConfig>` - Retry settings
  
- **SessionMetadata** stored in JSONL files:
  - Description, message count, token usage
  - Working directory
  - TODO list content (session-scoped)
  - Schedule ID if triggered by scheduler

- Sessions are file-based (JSONL format) with:
  - First line: metadata
  - Subsequent lines: messages
  - Atomic file operations for safety
  - Corruption recovery mechanisms

### Server Integration (crates/goose-server/src/state.rs)
- **AppState** holds single Agent instance:
  - `agent: Option<AgentRef>` where `AgentRef = Arc<Agent>`
  - Single agent serves all sessions/requests
  - Scheduler is also stored in AppState

### Request Routing (crates/goose-server/src/routes/reply.rs)
- Each `/reply` request creates a new async task
- SessionConfig created per request with:
  - Session ID (generated or provided)
  - Working directory
  - Optional schedule ID
- Agent's `reply()` method called with session config
- Messages persisted to session file after completion
- SSE (Server-Sent Events) used for streaming responses

### Concurrency Analysis
- **Single Agent, Multiple Sessions**: One Agent instance handles all concurrent sessions
- **Mutex Protection**: Most Agent fields protected by Mutex locks
  - Provider, sub_recipe_manager, frontend_tools, prompt_manager, etc.
  - Each field locked independently when accessed
- **Shared State Issues**:
  - Extensions loaded globally in ExtensionManager
  - Tool monitors shared across sessions
  - Provider instance shared (Arc<dyn Provider>)
  - Scheduler service shared
  - Frontend tools and instructions shared

### MCP/Extension Architecture
- **ExtensionManager** (crates/goose/src/agents/extension_manager.rs):
  - Contains `extensions: Mutex<HashMap<String, Extension>>`
  - Each Extension holds:
    - MCP client connection (Arc<Mutex<Box<dyn McpClientTrait>>>)
    - Server info and configuration
    - Optional temp directory for inline Python extensions
  - Extensions are agent-wide, not per-session
  - All sessions share the same extension connections

- **Extension Types**:
  - SSE (Server-Sent Events)
  - StreamableHttp (with OAuth support)
  - Stdio (subprocess)
  - Builtin (goose mcp subcommands)
  - InlinePython (dynamic Python scripts)
  - Frontend (browser-executed tools)

### Potential Concurrency Issues
1. **Extension State Conflicts**:
   - Multiple sessions may call same extension tools simultaneously
   - Extension state changes affect all sessions
   - Adding/removing extensions impacts running sessions

2. **Tool Monitor Conflicts**:
   - Tool repetition monitoring shared across sessions
   - One session's tool usage affects another's limits

3. **Provider Changes**:
   - Updating provider affects all active sessions
   - Model configuration changes mid-session

4. **Channel Conflicts**:
   - confirmation_tx/rx channels shared
   - tool_result_tx/rx channels shared
   - Messages from different sessions could interleave

5. **Recipe/Sub-recipe State**:
   - final_output_tool shared
   - sub_recipe_manager shared
   - One session's recipe affects another

### Design Considerations for One-Agent-Per-Session

#### Memory Implications
- **Current**: ~1 Agent instance regardless of sessions
- **Per-Session**: N agents for N concurrent sessions
- **Overhead per Agent**:
  - Base Agent struct: ~500 bytes
  - Extension connections: Variable (1-10MB per extension)
  - Provider instance: ~1-5MB
  - Channels and buffers: ~100KB
  - **Estimated**: 5-20MB per agent instance

#### Resource Management
- **MCP Connections**:
  - Current: Shared connections, resource efficient
  - Per-Session: Duplicate connections per session
  - Impact: More file descriptors, network connections, subprocesses

- **File Handles**:
  - Session files already per-session
  - Extension processes would multiply

#### Extension Lifecycle
- **Current Issues**:
  - Extensions persist across sessions
  - State contamination possible
  - Resource leaks accumulate

- **Per-Session Benefits**:
  - Clean extension state per session
  - Isolated failures
  - Better resource cleanup

#### Performance Impact
- **Pros**:
  - No lock contention between sessions
  - True parallel execution
  - Isolated failures
  - Better scaling for multi-user

- **Cons**:
  - Higher memory usage
  - Slower session startup (extension init)
  - More OS resources (processes, connections)
  - Potential for resource exhaustion

## Summary

Research complete. The current architecture uses a single Agent instance shared across all sessions, protected by multiple Mutex locks. This creates several concurrency issues:

1. **Shared extension state** - All sessions share MCP connections
2. **Tool monitor conflicts** - Usage limits affect all sessions  
3. **Channel interleaving** - Messages can mix between sessions
4. **Recipe state leaks** - Recipe tools persist across sessions
5. **Provider changes** - Updates affect all active sessions

Moving to one-agent-per-session would provide:
- **Benefits**: Complete isolation, no lock contention, better security, true multi-user support
- **Costs**: 5-20MB per session, more connections/processes, slower startup

Recommendation: Implement a phased migration with an agent pool pattern, fixing critical issues first while gradually moving to per-session agents with resource optimization.
