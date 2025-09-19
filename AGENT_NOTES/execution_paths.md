# Agent Execution Paths Analysis

## Current Execution Paths

### 1. Interactive Chat (goose-server)

**Entry Point**: `/reply` endpoint in `goose-server/src/routes/reply.rs`

**Flow**:
```
HTTP Request → AppState.get_agent() → agent.reply() → Stream responses
```

**Key Issues**:
- Uses shared agent from AppState
- All sessions share same agent instance
- Extension changes affect all users

### 2. Scheduled Jobs (Scheduler)

**Entry Point**: `run_scheduled_job_internal` in `crates/goose/src/scheduler.rs`

**Flow**:
```
Cron trigger → Create new Agent → Load recipe → Configure extensions → Execute → Save session
```

**Key Characteristics**:
- Fresh agent per execution
- Complete isolation
- Recipe-driven configuration

### 3. Dynamic Tasks

**Entry Point**: `dynamic_task__create_task` tool

**Flow**:
```
Tool call → Create tasks → Store in TasksManager → Execute via SubAgent
```

**Execution**:
```rust
// In subagent_handler.rs
SubAgent::new(task_config) → subagent.reply_subagent(instruction)
```

### 4. Sub-Recipes

**Two Paths**:

**Path A - CLI Spawning**:
```
Tool call → Build CLI command → Spawn process → Parse output
```

**Path B - SubAgent**:
```
Tool call → Create SubAgent → Execute inline
```

## Execution Configurations

### SessionConfig
Used in interactive and scheduled execution:
```rust
pub struct SessionConfig {
    pub id: Identifier,
    pub working_dir: PathBuf,
    pub schedule_id: Option<String>,
    pub execution_mode: Option<String>,  // "foreground" or "background"
    pub max_turns: Option<u32>,
    pub retry_config: Option<RetryConfig>,
}
```

### TaskConfig
Used in dynamic tasks and sub-recipes:
```rust
pub struct TaskConfig {
    pub id: String,
    pub provider: Option<Arc<dyn Provider>>,
    pub extensions: Option<Vec<ExtensionConfig>>,
    pub max_turns: Option<u32>,
}
```

## Provider Management

### Interactive (Shared Provider)
- Provider set once on agent creation
- Shared across all sessions
- Updated via `agent.update_provider()`

### Scheduled (Fresh Provider)
- Creates new provider per job
- Configured from recipe settings
- No sharing between jobs

### Dynamic Tasks (Inherited Provider)
- Uses parent agent's provider
- Passed via TaskConfig
- No independent provider creation

## Extension Management

### Interactive (Global Extensions)
- Extensions loaded into shared agent
- Changes affect all sessions
- Managed via platform tools

### Scheduled (Recipe Extensions)
- Extensions specified in recipe
- Loaded fresh per job
- Isolated from other jobs

### Dynamic Tasks (Filtered Extensions)
- Can specify subset of extensions
- Or inherit all from parent
- Managed via TaskConfig

## Session Storage

### Interactive Sessions
- Stored in `~/.config/goose/sessions/`
- Named by session ID
- Persisted after each turn

### Scheduled Sessions
- Same storage location
- Include `schedule_id` in metadata
- Named with timestamp pattern

### Dynamic Task Results
- Not persisted as sessions
- Results returned to parent
- Parent session updated

## Concurrency Handling

### Interactive (Problematic)
- Mutex contention on shared agent
- Race conditions in extension management
- Channel confusion for confirmations

### Scheduled (Good)
- Complete isolation per job
- No shared state
- Independent execution

### Dynamic Tasks (Partial)
- New agent instance
- But shares parent's provider
- Results aggregated in parent

## Key Differences

| Aspect | Interactive | Scheduled | Dynamic Tasks | Sub-Recipes |
|--------|------------|-----------|---------------|-------------|
| Agent Creation | Shared | Fresh per job | Fresh per task | Fresh or CLI |
| Provider | Shared | Fresh | Inherited | Fresh or inherited |
| Extensions | Global | Recipe-defined | Filtered | Recipe-defined |
| Session Storage | Yes | Yes | No | Depends |
| Isolation | None | Complete | Partial | Varies |
| Concurrency | Poor | Good | Good | Good |

## Unification Opportunities

All paths could be unified to:
1. Create or get session-specific agent
2. Configure with appropriate settings
3. Execute through same pipeline
4. Store results consistently

The key insight is that **all execution types are fundamentally the same** - they just differ in:
- How the agent is configured
- Where the instructions come from
- How results are returned
- Whether sessions are persisted
