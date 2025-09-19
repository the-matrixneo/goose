# Agent Manager Integration - Continuation Prompt

## Context
You are continuing work on PR #4542 (https://github.com/block/goose/pull/4542) which implements the Agent Manager to solve the shared agent concurrency issues described in Discussion #4389 (https://github.com/block/goose/discussions/4389).

## Current Status
The Agent Manager POC is functionally complete with the following accomplished:
1. ✅ Core `AgentManager` implementation in `crates/goose/src/agents/manager.rs`
2. ✅ Per-session agent isolation (each session gets its own agent)
3. ✅ Cleanup task with graceful shutdown (fixed in commit 0116cb79b2)
4. ✅ Session identifier documentation improvements
5. ✅ Comprehensive test coverage (4 test files)
6. ✅ Integration with goose-server routes

## The Problem
The branch `unified_execution` needs to be merged with `origin/main`, but there are significant conflicts due to authentication changes in main:
- PR #4338 removed `verify_secret_key` function in favor of middleware-based auth
- `AppState::new()` no longer takes parameters (was taking `secret_key`)
- Routes no longer individually verify authentication

## Key Documents to Review
1. **MERGE_STRATEGY.md** - Complete strategy for resolving conflicts
2. **PR_REVIEW2.md** - Analysis of which concerns are valid/invalid
3. **PR_CONCERNS2.md** - Initial concerns about the implementation
4. **Discussion #4389** - Original requirements for unified agent execution

## Work to Complete

### Phase 1: Merge Resolution (CRITICAL)
1. **Start with clean state**:
   ```bash
   git checkout unified_execution
   git merge origin/main --no-commit
   ```

2. **Resolve each conflict following MERGE_STRATEGY.md**:
   - `state.rs`: Keep AgentManager, remove secret_key param, add recipe_session_tracker
   - `commands/agent.rs`: Use `AppState::new().await` (no params)
   - Route files: Remove all `verify_secret_key` calls and `HeaderMap` params
   - `reply.rs`: Keep session-based agent retrieval, remove auth verification

3. **Key preservation points**:
   - ALL code in `crates/goose/src/agents/manager.rs` must be preserved
   - ALL test files in `crates/goose/tests/agent_manager_*.rs`
   - Session-based agent retrieval pattern in all routes
   - Graceful shutdown improvements

### Phase 2: Testing & Validation
1. **Compile and fix any remaining issues**:
   ```bash
   cargo build --package goose --package goose-server
   cargo fmt --all
   ./scripts/clippy-lint.sh
   ```

2. **Run critical tests**:
   ```bash
   cargo test -p goose agent_manager
   cargo test -p goose-server multi_session
   ```

3. **Manual testing with goosed**:
   - Start two chat sessions
   - Enable different extensions in each
   - Verify no cross-contamination
   - Check cleanup task runs

### Phase 3: PR Preparation
1. **Update PR description** with:
   - Summary of conflicts resolved
   - How we adapted to new auth pattern
   - Test results showing isolation works

2. **Create comprehensive commit message**:
   ```
   merge: integrate main branch with Agent Manager changes
   
   - Adapt to middleware-based authentication from PR #4338
   - Remove verify_secret_key in favor of middleware auth
   - Update AppState initialization to match new signature
   - Preserve all Agent Manager functionality for per-session isolation
   - Keep graceful shutdown improvements for cleanup task
   
   This completes the Agent Manager POC addressing Discussion #4389:
   - Each session gets its own isolated agent
   - Extensions don't interfere between sessions
   - Cleanup task manages memory with graceful shutdown
   - Foundation laid for scheduler/recipe integration
   ```

## Rationale for This Approach

### Why Agent Manager Matters
The shared agent in goose-server was causing:
- Extension state leaking between sessions
- Race conditions in multi-user scenarios
- Inability to run parallel sessions safely
- Blocked path to proper multi-user support

### Why the Merge is Complex
Main branch made a fundamental change to authentication:
- Old: Each route verified auth with `verify_secret_key(&headers, &state)`
- New: Middleware handles auth before routes are called
- Impact: Every route we modified needs adaptation

### Why We Must Preserve Our Work
The Agent Manager solves critical architectural issues:
- Enables true multi-session support (requirement for production)
- Unblocks scheduler integration (each job gets clean agent)
- Foundation for recipe/subagent unification (as per Discussion #4389)
- Already has comprehensive test coverage proving it works

## Success Criteria
1. ✅ All tests pass after merge
2. ✅ Two concurrent sessions can use different extensions without interference
3. ✅ Cleanup task properly manages idle agents
4. ✅ No regressions in existing functionality
5. ✅ PR ready for review with clear documentation of changes

## Important Notes
- Do NOT lose the `AgentManager` implementation - it's the core value
- Do NOT accept any merge resolution that breaks per-session isolation
- The auth changes are just mechanical updates - the architecture remains sound
- Focus on preserving functionality while adapting to new patterns

## First Commands to Run
```bash
cd /Users/tlongwell/Development/goose
git status  # Verify on unified_execution branch
git log --oneline -5  # Verify our commits are there
gh pr view 4542  # Review PR status
cat MERGE_STRATEGY.md  # Review merge strategy
```

Then begin the merge resolution following the strategy document.
