# Scheduler Research In Progress

## Research Goals
- Understand the theory behind Goose's scheduler system
- Analyze how agents run outside the main agent loop
- Document recipe execution mechanisms
- Map the complete architecture and flow

## TODO List

### Discovery Phase
- [x] Find scheduler-related files and modules
- [x] Identify main scheduler components
- [x] Locate recipe execution code
- [x] Find agent spawning/management code

### Code Analysis
- [x] Analyze scheduler core logic
- [x] Understand recipe data structures
- [x] Document agent lifecycle management
- [x] Map communication between scheduler and agents
- [x] Understand job/session management
- [x] Analyze scheduling mechanisms (cron, triggers)

### Architecture Documentation
- [x] Create architecture diagrams (conceptual)
- [x] Document data flow
- [x] List key abstractions and interfaces
- [x] Document configuration options

### Implementation Details
- [x] Database/storage mechanisms
- [x] Error handling strategies
- [x] Concurrency model
- [x] State management

## Notes

### Initial Observations
- Starting research at: 2025-08-27 14:41:30
- Project structure: Rust project with multiple crates
- Key crates to investigate: goose, goose-server, goose-cli

### Key Files Discovered
1. **Core Scheduler Files:**
   - `crates/goose/src/scheduler_trait.rs` - Trait definition for scheduler implementations
   - `crates/goose/src/scheduler.rs` - Legacy/default scheduler implementation
   - `crates/goose/src/temporal_scheduler.rs` - Temporal-based scheduler (advanced)
   - `crates/goose/src/scheduler_factory.rs` - Factory for creating scheduler instances

2. **Recipe System:**
   - `crates/goose/src/recipe/mod.rs` - Recipe data structures and parsing
   - Recipe files contain prompts, instructions, extensions, and configuration

3. **Agent Integration:**
   - `crates/goose/src/agents/schedule_tool.rs` - Tool for scheduling from within agents
   - `crates/goose/src/agents/agent.rs` - Core agent implementation

### Research Completed
- ✅ All TODO items completed
- ✅ Comprehensive report generated: SCHEDULER_REPORT.md
- ✅ Deep analysis of both theory and implementation
- ✅ Complete understanding of how agents run outside main loop

### Key Findings Summary
1. **Agent Isolation**: Each scheduled job creates a completely new Agent instance
2. **Execution Model**: Jobs spawn as Tokio tasks (Legacy) or Temporal workflows
3. **Recipe-Driven**: Recipes define all agent behavior and configuration
4. **State Management**: Comprehensive tracking via mutexes and persistent storage
5. **Dual Implementation**: Legacy (in-process) and Temporal (external service)
6. **Session Tracking**: Each execution creates a unique session with full history
7. **Graceful Degradation**: Falls back to Legacy if Temporal unavailable
8. **Complete Lifecycle**: From recipe loading → agent creation → execution → persistence
