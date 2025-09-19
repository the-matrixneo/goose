# Unified Agent Architecture Design

## Core Concept: Agent Per Session

Every execution context gets its own Agent instance, managed centrally.

## Unified Architecture Components

### 1. AgentManager (New Component)

```rust
pub struct AgentManager {
    /// Maps session ID to agent instance
    agents: Arc<RwLock<HashMap<SessionId, SessionAgent>>>,
    /// Configuration for agent creation
    config: AgentManagerConfig,
    /// Agent pool for reuse
    pool: Option<AgentPool>,
    /// Metrics and monitoring
    metrics: AgentMetrics,
}

pub struct SessionAgent {
    agent: Arc<Agent>,
    session_id: SessionId,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    execution_mode: ExecutionMode,
    state: SessionState,
}

impl AgentManager {
    /// Get or create an agent for a session
    pub async fn get_agent(&self, session_id: SessionId) -> Arc<Agent>;
    
    /// Execute a recipe in a session context
    pub async fn execute(
        &self,
        session_id: SessionId,
        source: RecipeSource,
        mode: ExecutionMode,
    ) -> Result<ExecutionResult>;
    
    /// Clean up idle agents
    pub async fn cleanup_idle(&self, max_idle: Duration);
    
    /// Get session statistics
    pub async fn stats(&self) -> ManagerStats;
}
```

### 2. Unified Recipe Model

Everything becomes a recipe internally:

```rust
pub enum RecipeSource {
    /// Traditional recipe file
    File(PathBuf),
    /// Programmatically created recipe
    Inline(Recipe),
    /// Simple text instruction (converted to minimal recipe)
    Text(String),
    /// Reference to another recipe
    Reference(String),
}

impl From<String> for Recipe {
    fn from(text: String) -> Self {
        Recipe::minimal()
            .with_instructions(text)
            .build()
    }
}
```

### 3. Execution Modes

Different behaviors, same infrastructure:

```rust
pub enum ExecutionMode {
    /// Interactive chat with user
    Interactive {
        streaming: bool,
        confirmations: bool,
    },
    /// Background/scheduled execution
    Background {
        scheduled: Option<ScheduleInfo>,
        retry: Option<RetryConfig>,
    },
    /// Sub-task of another session
    SubTask {
        parent: SessionId,
        inherit: InheritConfig,
    },
}
```

### 4. Session Lifecycle

```rust
pub enum SessionState {
    Active,
    Idle(Duration),
    Executing(TaskId),
    Completed,
    Failed(Error),
}

pub struct Session {
    id: SessionId,
    agent: Arc<Agent>,
    messages: Conversation,
    metadata: SessionMetadata,
    mode: ExecutionMode,
    state: SessionState,
}
```

## Migration Path

### Phase 1: Create AgentManager

1. Implement AgentManager with basic functionality
2. Add session-to-agent mapping
3. Create agent lifecycle management
4. Add metrics and monitoring

### Phase 2: Update goose-server

Replace shared agent with AgentManager:

```rust
// OLD
pub struct AppState {
    agent: Arc<RwLock<AgentRef>>,
}

// NEW
pub struct AppState {
    agent_manager: Arc<AgentManager>,
}

// Usage
let agent = state.agent_manager.get_agent(session_id).await;
```

### Phase 3: Unify Dynamic Tasks

Convert dynamic tasks to inline recipes:

```rust
// OLD
create_dynamic_task(text_instruction)

// NEW
let recipe = Recipe::from(text_instruction);
agent_manager.execute(
    session_id,
    RecipeSource::Inline(recipe),
    ExecutionMode::SubTask { ... }
).await
```

### Phase 4: Unify Scheduler

Update scheduler to use AgentManager:

```rust
// OLD
let agent = Agent::new();
// ... configure agent ...

// NEW
let session_id = generate_session_id();
agent_manager.execute(
    session_id,
    RecipeSource::File(recipe_path),
    ExecutionMode::Background { ... }
).await
```

### Phase 5: Unify Sub-Recipes

Execute sub-recipes through same pipeline:

```rust
// OLD
// Either spawn CLI or create SubAgent

// NEW
agent_manager.execute(
    sub_session_id,
    RecipeSource::Reference(sub_recipe_name),
    ExecutionMode::SubTask { parent: session_id, ... }
).await
```

## Benefits

### Immediate Benefits
1. **Session Isolation**: Each session has its own agent
2. **No Shared State**: Eliminates concurrency issues
3. **Consistent Behavior**: One execution path
4. **Better Testing**: Single code path to test

### Long-term Benefits
1. **Agent Pooling**: Reuse warmed agents
2. **Resource Management**: Centralized control
3. **Better Metrics**: Per-session tracking
4. **Easier Debugging**: Consistent execution model

## Backward Compatibility

### Maintain Existing APIs
- Keep tool interfaces unchanged
- Preserve session storage format
- Maintain recipe structure
- Keep CLI commands working

### Adapter Pattern
```rust
// Adapter for old dynamic task interface
pub async fn create_dynamic_task_compat(
    text: String,
    manager: &AgentManager,
) -> Result<Task> {
    let recipe = Recipe::from(text);
    let session_id = generate_temp_session();
    manager.execute(
        session_id,
        RecipeSource::Inline(recipe),
        ExecutionMode::SubTask { ... }
    ).await
}
```

## Implementation Timeline

### Week 1-2: Foundation
- Create AgentManager structure
- Implement basic lifecycle management
- Add session mapping

### Week 3-4: Server Migration
- Update AppState to use AgentManager
- Migrate reply endpoint
- Test session isolation

### Week 5-6: Task Unification
- Convert dynamic tasks to recipes
- Update TasksManager integration
- Migrate sub-recipe execution

### Week 7-8: Scheduler Integration
- Update scheduler to use AgentManager
- Test scheduled job isolation
- Verify session persistence

### Week 9-10: Optimization
- Implement agent pooling
- Add resource limits
- Performance tuning

### Week 11-12: Cleanup
- Remove old code paths
- Update documentation
- Final testing

## Risk Mitigation

### Memory Usage
- Implement agent limits (max agents per manager)
- Automatic cleanup of idle agents
- Agent pooling for reuse

### Performance
- Pre-warm agents in pool
- Lazy extension loading
- Connection reuse where possible

### Complexity
- Phased rollout with feature flags
- Extensive testing at each phase
- Maintain backward compatibility

## Success Criteria

1. **Functional**: All existing features work
2. **Performance**: No regression in benchmarks
3. **Isolation**: Sessions fully isolated
4. **Consistency**: Single execution model
5. **Maintainability**: Reduced code complexity

## Open Questions

1. **Agent Pool Size**: What's the optimal pool size?
2. **Idle Timeout**: How long to keep idle agents?
3. **Resource Limits**: Per-session or global?
4. **Extension Caching**: Share MCP connections?
5. **Migration Strategy**: Big bang or gradual?
