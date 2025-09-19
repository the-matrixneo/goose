# Agent Architecture Overhaul - Implementation Start

## Session Prompt for Implementation

You are about to begin implementing a major architectural improvement to Goose that will unify the agent execution model, eliminate ~3,400 lines of code, and fix critical multi-user support issues.

## Essential Documents to Read First

Read these documents in order to understand the full context:

1. **`AGENT_OVERHAUL_IMPLEMENTATION.md`** - Your implementation roadmap
2. **`AGENT_REPORT.md`** - Comprehensive analysis of current Agent architecture  
3. **GitHub Discussion #4389** - Original unification proposal: https://github.com/block/goose/discussions/4389
4. **PR #4311** - Shows how dynamic tasks were already unified (completed work to reference)

## Quick Overview of the Work

### The Problem
Goose currently has 4 different ways to create and run agents:
- **goose-server**: Uses a SINGLE shared agent for ALL sessions (causes race conditions)
- **scheduler**: Creates fresh agent per job (good pattern)
- **dynamic tasks**: Already unified with recipes in PR #4311
- **sub-recipes**: Dual execution paths

This causes critical bugs where users interfere with each other and requires implementing every feature 4 times.

### The Solution
Implement "one agent per session" model with a central `AgentManager` that:
- Gives each session its own isolated Agent instance
- Treats everything as recipes internally
- Removes the entire SubAgent subsystem
- Unifies all execution paths

### Expected Outcome
- **3,400 lines of code removed** (30-35% reduction)
- **85-90% of Agent code unchanged** (low risk)
- **Complete session isolation** (fixes multi-user bugs)
- **Single execution model** (implement features once)

## Phase 1 Starting Point (Weeks 1-2)

Begin with creating the `AgentManager` foundation:

### Step 1: Create the AgentManager
Create `crates/goose/src/agents/manager.rs` with:
- Session-to-agent mapping
- Agent lifecycle management  
- Cleanup for idle agents
- Metrics tracking

### Step 2: Write Tests
Create comprehensive tests in `crates/goose/src/agents/manager.rs` (in `#[cfg(test)]` module):
- Verify each session gets unique agent
- Verify same session reuses same agent
- Test cleanup of idle agents
- Test concurrent access

### Step 3: Integration
Add the module to `crates/goose/src/agents/mod.rs`:
```rust
pub mod manager;
```

## Key Implementation Guidelines

### Code Style
- Follow existing patterns from `session::storage` for async/await
- Use descriptive names (not abbreviations)
- Comments only for non-obvious logic (explain "why" not "what")
- Reuse existing types like `session::Identifier`

### Testing
- Every new function needs tests
- Use `#[tokio::test]` for async tests
- Test edge cases explicitly

### Minimal Changes
- The existing Agent only needs 4 new fields
- 85-90% of Agent code stays exactly the same
- We're wrapping Agent in AgentManager, not rewriting it

## Success Criteria for Phase 1

- [ ] AgentManager compiles and passes all tests
- [ ] Each session gets a unique agent
- [ ] Same session reuses the same agent  
- [ ] Idle agents are cleaned up correctly
- [ ] No performance regression
- [ ] Code follows existing patterns

## Important Context

This is fixing a CRITICAL bug where all users share one agent in goose-server, causing:
- Extension changes by one user affect all users
- Tool monitor counts bleed across sessions
- Race conditions in concurrent requests
- No proper multi-user support

The beauty of this solution is that we keep all the complex, working Agent code and just fix how agents are created and managed.

## Next Steps After Phase 1

Once AgentManager is working:
- Phase 2: Update goose-server to use AgentManager (weeks 3-4)
- Phase 3: Add approval bubbling for subtasks (weeks 5-6)  
- Phase 4: Migrate scheduler (weeks 7-8)
- Phase 5: Remove SubAgent entirely (weeks 9-10)

## Begin Implementation

Start by creating `crates/goose/src/agents/manager.rs` following the implementation plan in `AGENT_OVERHAUL_IMPLEMENTATION.md`. Focus on getting the basic session-to-agent mapping working first, then add cleanup and metrics.

Remember: We're doing minimal surgery on well-tested code. The Agent works great - we're just fixing how it's created and shared.
