# Current Agent Usage Patterns

## 1. Shared Agent in goose-server (PROBLEMATIC)

### Location: `crates/goose-server/src/state.rs`
```rust
pub struct AppState {
    agent: Arc<RwLock<AgentRef>>,  // SINGLE SHARED AGENT
    // ...
}
```

### Problems:
- **Single agent for ALL sessions**: All users share the same Agent instance
- **Mutex contention**: RwLock around agent causes bottleneck
- **Extension conflicts**: Enabling/disabling extensions affects all users
- **Tool monitor shared**: Repetition detection bleeds across sessions
- **Prompt manager shared**: System prompt changes affect everyone

### Usage in reply.rs:
```rust
let agent = state.get_agent().await;  // Gets the SAME agent for everyone
```

## 2. Fresh Agent per Scheduled Job (GOOD)

### Location: `crates/goose/src/scheduler.rs`
```rust
async fn run_scheduled_job_internal(...) {
    let agent: Agent = Agent::new();  // Fresh agent per job
    // Configure agent with recipe extensions
    // Execute with isolation
}
```

### Benefits:
- Complete isolation between jobs
- No shared state
- Clean extension configuration
- Independent provider connections

## 3. SubAgent for Dynamic Tasks (PARTIAL ISOLATION)

### Location: `crates/goose/src/agents/subagent.rs`
```rust
impl SubAgent {
    pub async fn new(task_config: TaskConfig) -> Result<Self> {
        let agent = Agent::new();  // Fresh agent for subagent
        // Configure with parent's provider
    }
}
```

### Characteristics:
- Creates new Agent instance
- Inherits provider from parent
- Limited extension control
- Returns results to parent

## 4. Sub-Recipe Execution (TWO PATHS)

### Path 1: CLI Spawning
```rust
// In tasks.rs
let command = format!("goose run --recipe {} ...", recipe_path);
// Spawns separate process
```

### Path 2: SubAgent
```rust
// Also uses SubAgent::new() for inline execution
```

## Key Observations

### Concurrency Issues in Shared Agent Model

1. **Extension Manager Conflicts**:
   - User A enables "developer" extension
   - User B's session suddenly has developer tools
   - Race conditions in extension loading/unloading

2. **Tool Monitor Interference**:
   - User A's repetitive tool calls affect User B's limits
   - Shared counter across all sessions

3. **Prompt Manager Conflicts**:
   - System prompt changes by one session affect others
   - Frontend instructions shared globally

4. **Channel Confusion**:
   - Single confirmation channel for all sessions
   - Tool results can be misrouted

### Resource Inefficiency

1. **No Agent Pooling**: 
   - Scheduler creates new agent every run
   - No reuse of warmed-up agents

2. **Duplicated Extension Loading**:
   - Each scheduled job reloads same extensions
   - No caching of MCP connections

3. **Provider Duplication**:
   - Each agent creates new provider connection
   - No connection pooling

## Migration Requirements

### Must Maintain:
1. Existing API compatibility
2. Tool interfaces
3. Session storage format
4. Recipe execution behavior

### Must Fix:
1. Session isolation in server
2. Resource efficiency
3. Consistent execution model
4. Multi-user support

### Nice to Have:
1. Agent pooling
2. Connection reuse
3. Pre-warmed agents
4. Better metrics per session
