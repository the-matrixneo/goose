# Merge Strategy for Agent Manager PR with Main

## Current Situation
The Agent Manager PR (#4542) has conflicts with main due to authentication changes:
- Main removed `verify_secret_key` function in favor of middleware-based auth (PR #4338)
- Main removed the `secret_key` parameter from `AppState::new()`
- Routes no longer need to verify secret keys individually

## Key Changes in Main That Affect Us
1. **Authentication**: Now handled by middleware in `commands/agent.rs`
2. **AppState**: No longer takes `secret_key` parameter
3. **Routes**: No longer need `HeaderMap` for auth verification

## Agent Manager Changes to Preserve
1. **Core Agent Manager**: All code in `crates/goose/src/agents/manager.rs` ✅
2. **Per-session agents**: Replace single agent with AgentManager in AppState ✅
3. **Session-based routing**: All routes use session_id to get correct agent ✅
4. **Tests**: All new test files for agent manager functionality ✅
5. **Graceful shutdown**: Our improvements to cleanup task ✅

## Resolution Strategy

### Step 1: Manual Conflict Resolution
For each conflicted file:

#### `crates/goose-server/src/state.rs`
- Keep AgentManager instead of single agent
- Remove `secret_key` parameter
- Add `recipe_session_tracker` from main
- Keep our new methods (get_agent, cleanup_idle_agents, etc.)

#### `crates/goose-server/src/commands/agent.rs`
- Remove all Agent creation code
- Use `AppState::new().await` (no parameters)
- Keep middleware auth from main

#### Route files (agent.rs, audio.rs, config_management.rs, etc.)
- Remove all `verify_secret_key` calls
- Remove `HeaderMap` parameters (change to `_headers` if needed for signature)
- Keep all session-based agent retrieval logic
- Remove `state.reset()` and `state.get_agent()` (no params) calls

#### `crates/goose-server/src/routes/reply.rs`
- Keep session-based agent retrieval
- Remove verify_secret_key
- Keep message visibility filtering from main

#### Test files
- Update `AppState::new()` calls to not pass parameters
- Keep all agent manager specific tests

### Step 2: Testing After Merge
1. Run agent manager tests
2. Run multi-session extension tests
3. Test with goosed to ensure sessions are isolated
4. Verify cleanup task works

## Commands to Execute

```bash
# Start fresh
git checkout unified_execution
git reset --hard HEAD

# Create a clean merge commit
git merge origin/main --no-commit

# Manually resolve each file according to strategy above
# Then:
git add .
git commit -m "merge: integrate main branch with Agent Manager changes

- Adapt to middleware-based authentication from PR #4338
- Remove verify_secret_key in favor of middleware auth
- Update AppState initialization to match new signature
- Preserve all Agent Manager functionality for per-session isolation
- Keep graceful shutdown improvements"
```

## Files to Carefully Review
1. `crates/goose-server/src/state.rs` - Core AppState changes
2. `crates/goose-server/src/routes/reply.rs` - Session handling
3. `crates/goose-server/src/commands/agent.rs` - Startup logic
4. All test files - Ensure they compile with new signatures
