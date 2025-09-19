# PR #4542 Review: Initial POC of Agent Manager

## Executive Summary

This PR implements an Agent Manager to provide per-session agent isolation in goose-server, addressing the shared agent concurrency issues outlined in discussion #4389. While the implementation follows many existing patterns in the codebase, it has significant issues that need addressing before it can be merged.

## Strengths

### 1. Follows Established Patterns
- **Error Handling**: Uses `thiserror::Error` with custom `AgentError` enum, consistent with other modules
- **Concurrency**: Uses `Arc<RwLock<HashMap>>` pattern found throughout the codebase (router_tool_selector, subagent_execution_tool)
- **API Design**: Returns `anyhow::Error` from public APIs, matching existing AppState patterns
- **Testing**: Comprehensive test coverage with good scenarios including concurrent access

### 2. Good Architecture Decisions
- Clear separation between AgentManager and AppState
- Metrics tracking for observability
- Session-based agent isolation solving the core problem
- Execution modes for future extensibility

### 3. Backward Compatibility
- Makes session_id optional with auto-generation
- Preserves existing API surface
- Handles missing working_dir gracefully

## Critical Issues

### 1. Incomplete Implementation (POC Status)
The PR is explicitly marked as "Rough POC" and only implements 1 of 4 execution paths:
- ✅ User sessions on goosed
- ❌ Scheduler integration
- ❌ Dynamic tasks
- ❌ Sub-recipes

The TODO comment in `commands/agent.rs` confirms this is incomplete.

### 2. Error Handling Inconsistencies

#### Provider Initialization
```rust
// Swallows errors - violates fail-fast principle
if let Err(e) = Self::initialize_agent_provider(&agent).await {
    tracing::warn!("Failed to initialize provider for new agent: {}", e);
    // Continue without provider - it can be set later via API
}
```
This pattern differs from the codebase norm of propagating errors. Compare with `session/storage.rs` which properly propagates all errors.

#### Mixed Error Types
The conversion from `AgentError` to `anyhow::Error` in AppState loses type information:
```rust
pub async fn get_agent(&self, session_id: session::Identifier) -> Result<AgentRef, anyhow::Error> {
    self.agent_manager.get_agent(session_id).await
        .map_err(|e| anyhow::anyhow!("Failed to get agent: {}", e))
}
```

### 3. Resource Management Issues

#### No Automatic Cleanup
Unlike `session/storage.rs` which has background cleanup, AgentManager requires manual cleanup:
```rust
pub async fn cleanup(&self) -> Result<usize, AgentError> {
    self.cleanup_idle(self.config.max_idle_duration).await
}
```
No background task is spawned to call this periodically.

#### Unbounded Growth
The `max_agents` configuration is never enforced:
```rust
pub struct AgentManagerConfig {
    pub max_agents: usize,  // Never checked!
}
```

### 4. Concurrency Concerns

#### Double-Checked Locking Anti-Pattern
```rust
// First check with read lock
{
    let agents = self.agents.read().await;
    if let Some(session_agent) = agents.get(&session_id) {
        return Ok(Arc::clone(&session_agent.agent));
    }
}
// Then check again with write lock
let mut agents = self.agents.write().await;
if let Some(session_agent) = agents.get_mut(&session_id) {
    // ...
}
```
While this works, it's unnecessarily complex. Consider using `entry` API or `DashMap`.

#### Metrics Lock Contention
```rust
metrics: Arc<RwLock<AgentMetrics>>
```
Metrics use RwLock but could use atomic counters for better performance:
```rust
pub struct AgentMetrics {
    pub agents_created: AtomicUsize,
    pub cache_hits: AtomicUsize,
    // ...
}
```

### 5. API Design Issues

#### Async Constructor
```rust
pub async fn new(config: AgentManagerConfig) -> Self
```
This is unusual in Rust. The constructor doesn't do any async work, so it should be synchronous.

#### Session Metadata Side Effects
The reply handler creates session metadata as a side effect:
```rust
// New session - create metadata
let new_metadata = session::SessionMetadata { ... };
if let Err(e) = session::storage::save_messages_with_metadata(...) {
    // Error handling
}
```
This violates single responsibility principle.

### 6. Testing Gaps

Missing test coverage for:
- Provider initialization failures
- Max agents enforcement
- Cleanup interval configuration
- Error propagation paths
- Integration with actual scheduler

### 7. Over-Engineering

#### Unused Complexity
```rust
pub enum ExecutionMode {
    Interactive,
    Background,
    SubTask {
        parent: session::Identifier,
        inherit: InheritConfig,
        approval_mode: ApprovalMode,
    },
}
```
The `SubTask` variant with `InheritConfig` and `ApprovalMode` is never used and adds unnecessary complexity for a POC.

#### Agent Pooling Stub
```rust
struct AgentPool {
    _placeholder: std::marker::PhantomData<()>,
}
```
Premature abstraction that should be removed until needed.

## Recommendations

### Immediate Fixes Required

1. **Complete the Implementation**: Integrate scheduler, dynamic tasks, and sub-recipes
2. **Fix Error Handling**: Propagate provider initialization errors properly
3. **Add Background Cleanup**: Spawn a task to periodically clean idle agents
4. **Enforce Resource Limits**: Check and enforce `max_agents` configuration
5. **Simplify Concurrency**: Use `DashMap` or simplify the locking strategy
6. **Make Constructor Sync**: Remove async from `new()` method
7. **Use Atomic Metrics**: Replace RwLock with atomic counters for metrics

### Code Quality Improvements

1. **Remove Over-Engineering**: Simplify ExecutionMode, remove agent pooling stub
2. **Extract Constants**: Replace magic numbers (3600, 300) with named constants
3. **Improve Documentation**: Add examples and error condition documentation
4. **Consistent Error Messages**: Standardize error message formatting

### Testing Improvements

1. Add integration tests with scheduler
2. Test error propagation paths
3. Add stress tests for concurrent agent creation
4. Test resource limit enforcement

## Verdict

**Status: Not Ready for Merge**

This PR successfully demonstrates the concept of per-session agents and solves the core isolation problem. However, as a self-described "Rough POC," it needs significant work before production readiness:

1. Complete the remaining 3 execution paths
2. Fix critical resource management issues
3. Simplify over-engineered components
4. Add proper error handling and cleanup

The foundation is solid and follows many good patterns from the codebase, but the implementation needs refinement. I recommend:
1. Opening this as a draft PR
2. Creating tracking issues for the remaining work
3. Breaking it into smaller, reviewable chunks
4. Adding integration tests before marking as ready

## Positive Aspects Worth Preserving

- The core AgentManager design with session mapping
- Comprehensive unit test suite
- Metrics tracking approach
- Backward compatibility handling
- Use of established error types (thiserror)

The PR shows good understanding of the problem space and the solution direction is correct. With the recommended improvements, this will be a valuable addition to Goose.
