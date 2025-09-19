# Unified Recipe Infrastructure Planning (URIP)

## TODO List

- [x] Read and analyze the four reports to understand current systems
  - [x] Read AGENT_SESSION_REPORT.md
  - [x] Read DYNAMIC_TASK_REPORT.md
  - [x] Read RECIPE_REPORT.md
  - [x] Read SCHEDULER_REPORT.md
- [x] Deep dive into the actual code implementation
  - [x] Examine Agent structure and lifecycle
  - [x] Understand TasksManager and task execution
  - [x] Study SubRecipeManager and sub-recipe tools
  - [x] Analyze scheduler implementations
  - [x] Review session management and storage
  - [x] Investigate extension management
- [x] Identify commonalities and differences
  - [x] Map out shared infrastructure
  - [x] Document divergent execution paths
  - [x] Note configuration differences
  - [x] Identify redundant code
- [x] Design unified system architecture
  - [x] Define core abstractions
  - [x] Plan execution model
  - [x] Design configuration system
  - [x] Plan migration path
- [x] Generate final report with recommendations

## Proposed Unified Architecture

### 1. Single Agent Server with Session-Based Agents

Instead of the current mixed approach, have:
- **One goose server** that manages everything
- **One agent per session** (not shared across sessions)
- **Unified execution pipeline** for all task types

### 2. Recipe as Universal Task Definition

Convert everything to recipes internally:

```rust
impl From<String> for Recipe {
    fn from(text: String) -> Self {
        Recipe::builder()
            .title("Dynamic Task")
            .instructions(text)
            .build()
    }
}
```

### 3. Unified Execution Pipeline

```
Request → Recipe → Agent → Session → Result
```

All execution paths follow this pattern:
1. Convert request to Recipe (if not already)
2. Get or create Agent for session
3. Execute recipe with agent
4. Store results in session
5. Return response

### 4. Agent Lifecycle Management

```rust
struct AgentManager {
    agents: HashMap<SessionId, Arc<Agent>>,
    
    async fn get_or_create_agent(&self, session_id: SessionId, config: AgentConfig) -> Arc<Agent> {
        // Reuse existing or create new
    }
    
    async fn cleanup_idle_agents(&self) {
        // Remove agents not used recently
    }
}
```

### 5. Task Execution Modes

```rust
enum TaskMode {
    Interactive {
        streaming: bool,
        user_confirmations: bool,
    },
    Background {
        scheduled: Option<CronSchedule>,
        retry_config: Option<RetryConfig>,
    },
    SubTask {
        parent_session: SessionId,
        inherit_extensions: bool,
    },
}
```

### Migration Path

#### Phase 1: Standardize Interfaces
1. Create unified `AgentExecutor` trait
2. Implement for current systems
3. Add adapter layers

#### Phase 2: Consolidate Agent Creation
1. Create `AgentFactory` with modes
2. Replace ad-hoc agent creation
3. Centralize configuration

#### Phase 3: Unify Task System
1. Convert dynamic tasks to inline recipes
2. Merge SubAgent into main Agent with mode flag
3. Standardize task storage and retrieval

#### Phase 4: Session-Based Architecture
1. Implement agent-per-session in goose-server
2. Add agent pooling and lifecycle management
3. Migrate scheduler to use agent pool

#### Phase 5: Cleanup and Optimization
1. Remove redundant code paths
2. Optimize for common cases
3. Add monitoring and metrics

### Benefits of Unified System

1. **Simplicity**: One way to create and manage agents
2. **Consistency**: Same execution model everywhere
3. **Isolation**: Better multi-user support
4. **Efficiency**: Agent pooling and reuse
5. **Maintainability**: Less code duplication
6. **Testability**: Unified testing approach
7. **Scalability**: Clear resource management

### Challenges to Address

1. **Memory Usage**: More agents = more memory
   - Solution: Agent pooling with limits
   
2. **Migration Complexity**: Existing code depends on current structure
   - Solution: Phased migration with adapters
   
3. **Performance**: Agent creation overhead
   - Solution: Pre-warming and caching
   
4. **Backwards Compatibility**: Existing APIs and tools
   - Solution: Compatibility layer during transition

## Commonalities Across Systems

### Shared Infrastructure
1. **Session Storage**: All systems use the same JSONL session storage
2. **Extension System**: MCP extensions with tool dispatch
3. **Provider Interface**: Same provider abstraction for LLM access
4. **Message Format**: Unified Conversation and Message types
5. **Tool Execution**: Similar tool dispatch patterns

### Redundant Code Identified
1. **Agent Creation**: Each system has its own way to create agents
2. **Extension Loading**: Duplicated logic for adding extensions
3. **Recipe Execution**: Similar patterns in scheduler and sub-recipes
4. **Task Execution**: Overlapping code between dynamic tasks and sub-recipes
5. **Provider Setup**: Repeated provider configuration logic

## Configuration Differences

### Main Agent (Interactive)
- Provider: Shared, configured once
- Extensions: Loaded on demand, persisted
- Session: Lightweight, file-based
- Execution: Streaming, interactive

### Scheduled Recipes
- Provider: Fresh per execution
- Extensions: From recipe definition
- Session: Full lifecycle with metadata
- Execution: Autonomous, background

### Dynamic Tasks/Sub-Recipes
- Provider: Inherited from parent
- Extensions: Copied or specified
- Session: Not persisted (subagent internal)
- Execution: Synchronous, returns result

## Unified System Design

### Core Concept: Universal Agent Execution Context

Instead of having different agent creation patterns, we could have a single, unified agent execution context that can be configured for different modes:

```rust
enum ExecutionMode {
    Interactive,      // Main agent sessions
    Scheduled,        // Cron-based recipes
    SubTask,          // Dynamic tasks and sub-recipes
    Standalone,       // CLI one-shot execution
}

struct AgentContext {
    mode: ExecutionMode,
    provider: Arc<dyn Provider>,
    extensions: Vec<ExtensionConfig>,
    session_config: Option<SessionConfig>,
    parent_agent: Option<Arc<Agent>>,
}
```

### Unified Task System

All tasks (dynamic, sub-recipe, scheduled) could be represented as recipes:

```rust
enum TaskDefinition {
    TextInstruction(String),           // Dynamic task
    Recipe(Recipe),                     // Full recipe
    RecipeReference(String, Params),   // Sub-recipe with params
}

struct UnifiedTask {
    id: String,
    definition: TaskDefinition,
    execution_context: AgentContext,
}
```

### Agent Pool Architecture

Instead of creating agents ad-hoc, use an agent pool:

```rust
struct AgentPool {
    interactive_agents: HashMap<SessionId, Arc<Agent>>,
    task_agents: Vec<Arc<Agent>>,  // Reusable pool
    max_agents: usize,
}
```

## Initial Observations from Reports

### Common Themes
1. **Agent-based execution**: All systems create and manage Agent instances
2. **Task abstraction**: Dynamic tasks and sub-recipes both use Task struct
3. **Session management**: All create sessions with metadata and storage
4. **Extension support**: All systems need to manage MCP extensions
5. **Async execution**: Everything uses tokio for async operations

### Key Differences
1. **Agent lifecycle**:
   - Main agent: Single shared instance for all sessions
   - Scheduled recipes: Fresh agent per job execution
   - Sub-recipes/dynamic tasks: Use subagent system or CLI spawning

2. **Execution methods**:
   - Dynamic tasks: Direct subagent invocation
   - Sub-recipes: CLI command execution (goose run --recipe)
   - Scheduled recipes: Full agent lifecycle with recipe

3. **Configuration**:
   - Dynamic tasks: Minimal, text-only
   - Sub-recipes: Full recipe with parameters
   - Scheduled recipes: Complete recipe with cron schedule

### Potential Unification Points
1. All could use the same agent creation/management system
2. Task abstraction could be standardized
3. Session storage is already shared
4. Extension management could be unified

## Research Notes

### Agent Architecture
- **Agent struct**: Contains provider, extension_manager, sub_recipe_manager, tasks_manager, scheduler_service, etc.
- **Single shared instance** in main goose-server for all sessions
- **Fresh instances** created for scheduled jobs and subagents
- Heavy use of Mutex for shared state protection

### Task Execution Systems
1. **TasksManager**: Simple HashMap storage for tasks, shared across agent
2. **SubAgent**: Separate agent instances with own extension managers
3. **Scheduled jobs**: Use `run_scheduled_job_internal` to create fresh agents

### Key Findings
- All systems create Agent instances but in different ways
- Extension management is duplicated across systems
- Session storage is already unified
- Provider management varies (shared vs per-instance)

### Next Steps
1. ✅ Look at Agent struct implementation
2. ✅ Examine TasksManager in detail  
3. ✅ Study subagent execution system
4. ✅ Understand how recipes are loaded and executed (scheduler)
5. ✅ Map out the different execution paths
6. ✅ Look at how extensions are managed in each system
7. ✅ Examine session creation and management

## Execution Path Analysis

### Main Agent (goose-server)
1. **Single shared Agent instance** in AppState
2. Handles multiple concurrent sessions via `reply()` method
3. Sessions differentiated by SessionConfig
4. Shared state protected by Mutexes
5. Extensions loaded once, shared across sessions

### Scheduled Recipes (Scheduler)
1. **Fresh Agent per job** created in `run_scheduled_job_internal`
2. Loads recipe from file (YAML/JSON)
3. Configures agent with recipe extensions
4. Creates new provider instance
5. Executes with SessionConfig containing schedule_id
6. Full agent lifecycle per execution

### Dynamic Tasks
1. Creates tasks with text instructions
2. Tasks stored in TasksManager
3. Executed via `run_complete_subagent_task`
4. **Creates SubAgent** with own extension manager
5. Uses provider from parent agent

### Sub-Recipes  
1. Creates tasks from recipe definitions
2. Tasks stored in TasksManager
3. Two execution paths:
   - CLI command: `goose run --recipe`
   - Subagent system (like dynamic tasks)
4. Parameter validation and passing

### SubAgent System
1. **Fresh SubAgent instance** per task
2. Own ExtensionManager (copies from parent or uses specified)
3. Conversation management
4. Tool dispatch through own extension_manager
5. Status tracking (Ready, Processing, Completed)
