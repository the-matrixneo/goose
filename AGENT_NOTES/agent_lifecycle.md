# Agent Lifecycle Analysis

## Creation and Initialization

### Agent::new()
Creates a new agent instance with:
- Communication channels (confirmation, tool results)
- Extension manager
- Sub-recipe manager
- Tasks manager
- Tool monitor
- Retry manager
- AutoPilot for model switching
- Empty provider (must be set later)

### Provider Configuration
```rust
agent.update_provider(provider) // Sets the LLM provider
```

### Extension Loading
```rust
agent.add_extension(extension) // Adds MCP extensions
```

## Reply Loop Lifecycle

### 1. Pre-processing
- Auto-compaction check
- Context preparation
- Tool and prompt preparation

### 2. Main Loop (per turn)
- Check cancellation
- Check final output
- AutoPilot model switching
- Stream response from provider
- Process tool calls
- Handle permissions
- Execute tools
- Update messages
- Retry logic

### 3. Post-processing
- Session metrics update
- Message persistence
- Cleanup

## Tool Execution Lifecycle

### 1. Tool Request
- Categorize tools (frontend/backend, readonly/regular)
- Record tool requests

### 2. Permission Check
- Check tool permissions
- Handle approvals/denials

### 3. Tool Dispatch
- Route to appropriate handler:
  - Platform tools
  - Extension tools
  - Frontend tools
  - Sub-recipe tools
  - Dynamic tasks

### 4. Result Processing
- Aggregate results
- Handle notifications
- Update message with responses

## Session Management

### Session Configuration
```rust
SessionConfig {
    id: Identifier,
    working_dir: PathBuf,
    schedule_id: Option<String>,
    execution_mode: Option<String>,
    max_turns: Option<u32>,
    retry_config: Option<RetryConfig>,
}
```

### Session Storage
- Messages saved to JSONL files
- Metadata tracked (tokens, message count, etc.)
- Extension data persisted (e.g., TODO content)

## Resource Management

### Shared Resources
- Provider (Arc<dyn Provider>)
- Tool monitor (Arc<Mutex<Option<ToolMonitor>>>)
- Final output tool (Arc<Mutex<Option<FinalOutputTool>>>)

### Cleanup
- Channels automatically cleaned up when Agent dropped
- Extensions cleaned up by ExtensionManager
- Sessions persisted to disk

## Concurrency Model

### Mutexes Used
- `provider: Mutex<Option<Arc<dyn Provider>>>`
- `sub_recipe_manager: Mutex<SubRecipeManager>`
- `frontend_tools: Mutex<HashMap<String, FrontendTool>>`
- `frontend_instructions: Mutex<Option<String>>`
- `prompt_manager: Mutex<PromptManager>`
- `confirmation_rx: Mutex<mpsc::Receiver<...>>`
- `tool_monitor: Arc<Mutex<Option<ToolMonitor>>>`
- `scheduler_service: Mutex<Option<Arc<dyn SchedulerTrait>>>`
- `autopilot: Mutex<AutoPilot>`

### Channel Communication
- Confirmation channel: UI -> Agent for tool approvals
- Tool result channel: Frontend -> Agent for tool results

## State Transitions

### Agent States
1. **Created**: Fresh instance, no provider
2. **Configured**: Provider set, extensions loaded
3. **Active**: Processing messages in reply loop
4. **Waiting**: Awaiting user confirmation or tool results
5. **Completed**: Reply loop finished

### Message Flow
1. User message received
2. Agent processes and generates response
3. Tool calls executed if needed
4. Results aggregated
5. Final message returned

## Error Handling

### Context Length Exceeded
- Triggers auto-compaction
- Falls back to summarization
- Replaces history if successful

### Provider Errors
- Returned to user with retry suggestion
- Retry manager handles automatic retries

### Tool Errors
- Individual tool failures don't stop execution
- Errors returned in tool response
- User notified of failures
