# Phase 3: Complete goosed Integration & Testing

## Context
You've just completed Phase 2 of the Agent Architecture Overhaul. The AgentManager has been created and integrated into goose-server (goosed). Each session now gets its own isolated agent instance, fixing the critical multi-user bug.

## Current State
- ✅ **Phase 1**: AgentManager created with tests
- ✅ **Phase 2**: goose-server updated to use AgentManager
- 🔄 **Phase 3**: Final integration and testing needed

## Your Previous Work
Read these files to understand what's been done:
1. `AGENT_OVERHAUL_IMPLEMENTATION.md` - The full implementation plan
2. `PHASE2_COMPLETE.md` - Detailed record of Phase 2 changes
3. `crates/goose/src/agents/manager.rs` - The AgentManager implementation
4. Review git diff to see all changes made

## Remaining Tasks for goosed Integration

### 1. Move Tests to Dedicated Test File
**CLEANUP**: Tests should be in integration tests, not inline
- [ ] Create `crates/goose/tests/agent_manager_test.rs`
- [ ] Move all tests from the `#[cfg(test)]` module in `manager.rs`
- [ ] Ensure tests still pass after move
- [ ] Remove the inline test module from `manager.rs`

### 2. Provider and Scheduler Integration
**CRITICAL**: Each agent needs proper initialization
- [ ] Ensure each new agent gets a provider configured
- [ ] Verify scheduler access for each agent (currently commented out)
- [ ] Check if agents inherit configuration from environment/settings

### 3. Extension Persistence
**ISSUE**: Extensions are per-agent, not global anymore
- [ ] Verify extension state persists within a session
- [ ] Check if extensions need to be re-added after agent creation
- [ ] Test extension isolation between sessions

### 4. Session Lifecycle Management
- [ ] Implement periodic cleanup of idle agents (currently manual)
- [ ] Add cleanup task to goosed startup
- [ ] Configure cleanup intervals (default 5 minutes in config)

### 5. Client Compatibility
**IMPORTANT**: The desktop app needs session_id in all requests
- [ ] Verify all client requests include session_id
- [ ] Check fallback behavior for missing session_id
- [ ] Test with actual desktop app if possible

### 6. Testing Checklist
Manual testing needed:
- [ ] Start goosed and test with multiple curl/API clients
- [ ] Verify each session maintains separate state
- [ ] Test extension add/remove isolation
- [ ] Check memory usage with multiple sessions
- [ ] Verify agent cleanup works

### 7. Configuration & Initialization Issues
Check these potential problems:
```rust
// In AgentManager::create_agent_for_session
let agent = Agent::new();
// TODO: Agent needs:
// - Provider configuration
// - Scheduler reference  
// - Initial extensions from config?
// - Environment settings?
```

## Commands to Run

```bash
# Build and start goosed
cargo build --release
./target/release/goosed

# In another terminal, test with multiple sessions
# Session 1
curl -X POST http://localhost:9000/agent/start \
  -H "x-secret-key: test" \
  -H "Content-Type: application/json" \
  -d '{"working_dir": "/tmp/session1"}'

# Session 2  
curl -X POST http://localhost:9000/agent/start \
  -H "x-secret-key: test" \
  -H "Content-Type: application/json" \
  -d '{"working_dir": "/tmp/session2"}'

# Test isolation by adding extensions to each session
```

## Key Questions to Answer

1. **Provider Configuration**: How should each agent get its provider?
   - From environment variables?
   - From a default configuration?
   - Passed through the API?

2. **Scheduler Access**: The scheduler is in AppState, but each agent needs it
   - Should we pass scheduler reference when creating agents?
   - Or set it after creation?

3. **Extension Defaults**: Should agents start with default extensions?
   - Developer extension is commonly needed
   - Should this be configurable?

4. **Cleanup Strategy**: When should idle agents be removed?
   - After X minutes of inactivity?
   - On session end?
   - Periodic background task?

## Success Criteria

Phase 3 is complete when:
1. goosed starts and accepts multiple concurrent sessions
2. Each session has a fully functional agent (provider, extensions work)
3. Sessions are properly isolated (no state leakage)
4. Memory is managed (idle cleanup works)
5. Desktop app can connect and work normally

## DO NOT Focus On (Yet)
- SubAgent system changes
- Recipe unification  
- Scheduler migration
- These are Phase 4+ tasks

## Start Here
1. Run `cargo build --release` and verify it still compiles
2. Start goosed and test basic session creation
3. Identify what's broken (likely provider/extension initialization)
4. Fix initialization issues in AgentManager::create_agent_for_session
5. Test multi-session scenarios thoroughly

Remember: The goal is to make goosed fully functional with per-session agents before moving on to the larger architectural changes.
