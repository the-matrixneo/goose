# Agent Structure Analysis

## Core Agent Struct (agent.rs)

The `Agent` struct is the central orchestrator of the Goose system. It contains:

### Key Components
1. **Provider Management**: `provider: Mutex<Option<Arc<dyn Provider>>>`
   - Manages the LLM provider (GPT-4, Claude, etc.)
   - Shared reference counted for concurrent access
   
2. **Extension System**: `extension_manager: ExtensionManager`
   - Manages MCP extensions (developer, browser, etc.)
   - Handles tool dispatch and resource management
   
3. **Recipe & Task Management**:
   - `sub_recipe_manager: Mutex<SubRecipeManager>` - Manages sub-recipes
   - `tasks_manager: TasksManager` - Stores and retrieves tasks
   
4. **Tool Systems**:
   - `final_output_tool: Arc<Mutex<Option<FinalOutputTool>>>` - Response validation
   - `frontend_tools: Mutex<HashMap<String, FrontendTool>>` - UI-based tools
   - `tool_route_manager: ToolRouteManager` - LLM-based tool routing
   - `tool_monitor: Arc<Mutex<Option<ToolMonitor>>>` - Monitors tool repetition
   
5. **Communication Channels**:
   - `confirmation_tx/rx` - For permission confirmations
   - `tool_result_tx/rx` - For tool execution results
   
6. **Other Systems**:
   - `prompt_manager: Mutex<PromptManager>` - System prompt construction
   - `scheduler_service: Mutex<Option<Arc<dyn SchedulerTrait>>>` - Scheduled jobs
   - `retry_manager: RetryManager` - Retry logic
   - `autopilot: Mutex<AutoPilot>` - Automatic model switching

## Agent Creation Flow

```rust
Agent::new() -> Self {
    // Creates channels for communication
    // Initializes all managers
    // Sets up tool monitor with retry manager
}
```

## Main Reply Loop (reply method)

The core execution loop follows this pattern:

1. **Auto-compaction Check**: Handle context length management
2. **Context Preparation**: Fix conversation, prepare tools and prompts
3. **Turn Loop** (up to max_turns):
   - Check cancellation
   - Check final output
   - AutoPilot model switching
   - Stream response from provider
   - Process tool calls
   - Handle permissions
   - Execute tools
   - Update messages
   - Retry logic if needed

## Tool Execution Pipeline

1. **Tool Categorization**: Split into frontend/backend tools
2. **Permission Checking**: Determine approved/denied/needs-approval
3. **Tool Dispatch**: Route to appropriate handler
4. **Result Aggregation**: Collect and format responses

## Extension Points

The Agent is designed to be extended through:
- Extensions (MCP servers)
- Frontend tools
- Sub-recipes
- Dynamic tasks
- Custom prompts
