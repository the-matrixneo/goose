# Code Reduction Analysis for Unified Agent Architecture

## Executive Summary
**Expected Net Reduction: 30-40% of agent-related code (~3,000-4,000 lines)**

## Areas of Significant Reduction

### 1. Eliminated Duplicate Agent Creation Logic (~500 lines)

**Current State**: Agent creation logic repeated in 4+ places:
- `goose-server/src/state.rs` - Shared agent setup
- `goose/src/scheduler.rs` - Per-job agent creation
- `goose/src/agents/subagent.rs` - SubAgent creation
- Various test files

**After Unification**: Single creation path in AgentManager
```rust
// ONE place instead of FOUR
impl AgentManager {
    async fn create_agent(&self, config: AgentConfig) -> Arc<Agent> {
        // Single, unified creation logic
    }
}
```

### 2. Consolidated Extension Loading (~400 lines)

**Current Duplication**:
```rust
// In scheduler.rs
for extension in recipe.extensions {
    agent.add_extension(extension).await?;
}

// In subagent.rs
for extension in extensions_to_add {
    extension_manager.add_extension(extension).await?;
}

// In server routes
if let Some(extensions) = request.extensions {
    for ext in extensions {
        agent.add_extension(ext).await?;
    }
}
```

**After**: Single extension configuration in AgentManager

### 3. Removed SubAgent Implementation (~800 lines)

The entire `SubAgent` struct and its implementation can be removed:
- `agents/subagent.rs` - ~350 lines
- `agents/subagent_handler.rs` - ~100 lines  
- `agents/subagent_task_config.rs` - ~50 lines
- Related test files - ~300 lines

SubAgents become regular Agents with `ExecutionMode::SubTask`.

### 4. Simplified Task Execution (~600 lines)

**Current**: Multiple execution paths in `tasks.rs`:
```rust
match task.task_type {
    TaskType::InlineRecipe => {
        // Special handling for inline recipes
        handle_inline_recipe_task(...)
    }
    TaskType::SubRecipe => {
        // Different handling for sub-recipes
        build_command(...) // CLI spawning logic
    }
}
```

**After**: Single execution through AgentManager:
```rust
agent_manager.execute(session_id, recipe_source, mode).await
```

### 5. Unified Provider Management (~300 lines)

**Current**: Provider configuration scattered:
- Scheduler creates its own provider
- SubAgent inherits from parent
- Server manages shared provider
- Tests create mock providers differently

**After**: Provider management centralized in AgentManager

### 6. Consolidated Session Management (~400 lines)

**Current**: Different session handling for:
- Interactive sessions (server)
- Scheduled sessions (scheduler)
- No sessions for dynamic tasks
- CLI sessions for sub-recipes

**After**: Unified session model for all execution types

### 7. Removed Dual-Path Sub-Recipe Execution (~500 lines)

**Current**: Two paths for sub-recipes:
1. CLI spawning with command building
2. SubAgent execution

**After**: Single path through unified execution pipeline

Removes:
- Command building logic
- Process spawning code
- Output parsing
- Error handling duplication

### 8. Simplified Tool Dispatch (~200 lines)

**Current**: Different tool routing for different agent types:
```rust
// In Agent
if tool == "dynamic_task" { ... }
else if tool == "subagent_execute" { ... }
else if is_sub_recipe_tool { ... }

// In SubAgent
if needs_approval { 
    // Different approval logic
}
```

**After**: Single tool dispatch with unified approval flow

### 9. Reduced Test Complexity (~800 lines)

**Current**: Separate tests for:
- Agent tests
- SubAgent tests
- Scheduler agent tests
- Dynamic task tests
- Sub-recipe tests

**After**: Single set of comprehensive tests for unified system

### 10. Eliminated Workarounds (~300 lines)

Remove various workarounds for:
- Shared agent mutex contention
- Extension state management
- Cross-session interference
- Resource cleanup hacks

## Code That Will Be Added

### New Components (~1,000 lines)
- `AgentManager` implementation - ~400 lines
- Approval bubbling infrastructure - ~300 lines
- Agent pooling logic - ~200 lines
- Migration adapters - ~100 lines

## Net Reduction Calculation

| Component | Lines Removed | Lines Added | Net Change |
|-----------|--------------|-------------|------------|
| Agent Creation | -500 | +100 | -400 |
| Extension Loading | -400 | +50 | -350 |
| SubAgent | -800 | 0 | -800 |
| Task Execution | -600 | +100 | -500 |
| Provider Management | -300 | +50 | -250 |
| Session Management | -400 | +100 | -300 |
| Sub-Recipe Execution | -500 | 0 | -500 |
| Tool Dispatch | -200 | +100 | -100 |
| Tests | -800 | +200 | -600 |
| Workarounds | -300 | 0 | -300 |
| AgentManager | 0 | +400 | +400 |
| Approval Bubbling | 0 | +300 | +300 |
| **TOTAL** | **-4,800** | **+1,400** | **-3,400** |

## Additional Benefits Beyond Line Count

### 1. Cognitive Load Reduction
- One mental model instead of four
- Single execution path to understand
- Consistent patterns throughout

### 2. Maintenance Efficiency
- Fix bugs in one place, not four
- Single test suite to maintain
- Unified documentation

### 3. Feature Development Speed
- New features added once, work everywhere
- No need to implement for each execution type
- Consistent behavior guaranteed

### 4. Code Quality Improvements
- Better separation of concerns
- Cleaner interfaces
- Reduced coupling

## Real Example: Adding a New Feature

### Current Approach (Multiple Implementations)
```rust
// 1. Add to Agent (~50 lines)
impl Agent {
    pub async fn new_feature(&self) { ... }
}

// 2. Add to SubAgent (~50 lines)
impl SubAgent {
    pub async fn new_feature(&self) { ... }
}

// 3. Add to Scheduler path (~30 lines)
fn setup_agent_with_feature() { ... }

// 4. Add to dynamic tasks (~30 lines)
fn configure_task_with_feature() { ... }

// Total: ~160 lines
```

### After Unification (Single Implementation)
```rust
// Add once to AgentManager (~40 lines)
impl AgentManager {
    pub async fn new_feature(&self) { ... }
}
// Total: ~40 lines (75% reduction)
```

## Specific Files That Can Be Deleted

1. `crates/goose/src/agents/subagent.rs` - 350 lines
2. `crates/goose/src/agents/subagent_handler.rs` - 100 lines
3. `crates/goose/src/agents/subagent_task_config.rs` - 50 lines
4. Parts of `crates/goose/src/agents/subagent_execution_tool/tasks.rs` - 200 lines
5. CLI spawning code in sub-recipes - 300 lines

## Conclusion

The unified architecture will result in:
- **~3,400 lines net reduction** (30-35% of agent-related code)
- **Simpler codebase** with one execution model
- **Faster feature development** (75% less code for new features)
- **Easier maintenance** with centralized logic
- **Better testability** with single test suite

This is a rare case where we can simultaneously:
1. Add significant new capabilities (proper multi-user support, approval bubbling)
2. Fix major architectural issues (shared agent problems)
3. **Reduce the codebase size by ~30%**

The reduction comes from eliminating the parallel implementations that evolved organically as the system grew. By unifying to a single execution model, we remove massive duplication while making the system more powerful and maintainable.
