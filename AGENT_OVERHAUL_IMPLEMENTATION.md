# Agent Overhaul Implementation Plan

## Overview

This document outlines the implementation plan for unifying Goose's agent execution model, transitioning from four parallel execution paths to a single, session-based architecture managed by an `AgentManager`. This change will eliminate ~3,400 lines of code while keeping 85-90% of the core Agent unchanged.

## Background & Motivation

### The Problem
Currently, Goose has four different ways to create and run agents:
1. **Shared agent in goose-server** - All sessions share one agent (causes concurrency bugs)
2. **Fresh agents in scheduler** - Each job creates new agent (good isolation)
3. **Dynamic tasks via inline recipes** - Uses SubAgent (already unified in PR #4311)
4. **Sub-recipes** - Dual path: CLI spawning or SubAgent

This has led to:
- **Critical bugs**: Race conditions, state bleeding across users ([Discussion #4389](https://github.com/block/goose/discussions/4389))
- **Code duplication**: Same logic implemented 4 different ways
- **Maintenance burden**: Features must be added to each system separately
- **Poor multi-user support**: Shared agent prevents proper isolation

### The Solution
Implement "one agent per session" model with centralized `AgentManager`, treating all execution as recipes internally.

### Expected Outcomes
- **3,400 lines removed** (30-35% reduction)
- **Complete session isolation** 
- **True multi-user support**
- **Single execution model**
- **85-90% of Agent code unchanged**

## Required Reading

### Core Documentation
1. **This Report Suite**:
   - `AGENT_REPORT.md` - Comprehensive agent analysis
   - `DYNAMIC_TASK_REPORT.md` - Dynamic task system analysis
   - `RECIPE_REPORT.md` - Recipe system deep dive
   - `SCHEDULER_REPORT.md` - Scheduler implementation details
   - `UNIFICATION_REPORT.md` - Unification strategy

2. **GitHub Discussions & PRs**:
   - [Discussion #4389](https://github.com/block/goose/discussions/4389) - Unify Agent Execution proposal
   - [PR #4311](https://github.com/block/goose/pull/4311) - Dynamic tasks unified with recipes (COMPLETED)
   - [PR #4216](https://github.com/block/goose/pull/4216) - Session-aware agents (in progress)

3. **Source Code to Study**:
   - `crates/goose/src/agents/agent.rs` - Core Agent implementation
   - `crates/goose/src/session/storage.rs` - Session persistence patterns
   - `crates/goose/src/recipe/mod.rs` - Recipe structure and validation
   - `crates/goose-server/src/state.rs` - Current shared agent problem
   - `crates/goose/src/scheduler.rs` - Good isolation pattern to follow

## Implementation Phases

### Phase 1: AgentManager Foundation (Week 1-2)

#### Goals
- Create centralized agent lifecycle management
- Implement session-to-agent mapping
- Add agent pooling infrastructure

#### Implementation

**File: `crates/goose/src/agents/manager.rs`**

```rust
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::agents::Agent;
use crate::session;

/// Manages agent lifecycle and session mapping
/// 
/// This is the central component that ensures each session gets its own
/// isolated agent instance, solving the shared agent concurrency issues.
pub struct AgentManager {
    /// Maps session IDs to their dedicated agents
    agents: Arc<RwLock<HashMap<session::Identifier, SessionAgent>>>,
    
    /// Optional pool for agent reuse (future optimization)
    pool: Option<AgentPool>,
    
    /// Configuration for agent creation and management
    config: AgentManagerConfig,
    
    /// Metrics for monitoring and debugging
    metrics: Arc<RwLock<AgentMetrics>>,
}

struct SessionAgent {
    agent: Arc<Agent>,
    session_id: session::Identifier,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    execution_mode: ExecutionMode,
    state: SessionState,
}

// Follow existing async patterns from session::storage
impl AgentManager {
    pub async fn new(config: AgentManagerConfig) -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            pool: None, // Start without pooling, add later
            config,
            metrics: Arc::new(RwLock::new(AgentMetrics::default())),
        }
    }
    
    /// Get or create an agent for a session
    /// 
    /// This ensures each session has exactly one agent, providing
    /// complete isolation between users/sessions.
    pub async fn get_agent(
        &self, 
        session_id: session::Identifier
    ) -> Result<Arc<Agent>, AgentError> {
        let mut agents = self.agents.write().await;
        
        // Check if agent already exists for this session
        if let Some(session_agent) = agents.get_mut(&session_id) {
            session_agent.last_used = Utc::now();
            self.metrics.write().await.record_cache_hit();
            return Ok(Arc::clone(&session_agent.agent));
        }
        
        // Create new agent for this session
        let agent = self.create_agent_for_session(session_id.clone()).await?;
        
        agents.insert(session_id.clone(), SessionAgent {
            agent: Arc::clone(&agent),
            session_id,
            created_at: Utc::now(),
            last_used: Utc::now(),
            execution_mode: ExecutionMode::Interactive,
            state: SessionState::Active,
        });
        
        self.metrics.write().await.record_agent_created();
        Ok(agent)
    }
    
    /// Clean up idle agents to manage memory
    /// 
    /// Following the pattern from session::storage::cleanup_old_sessions
    pub async fn cleanup_idle(&self, max_idle: Duration) -> Result<usize, AgentError> {
        let mut agents = self.agents.write().await;
        let now = Utc::now();
        let mut removed = 0;
        
        agents.retain(|_, session_agent| {
            let idle_time = now.signed_duration_since(session_agent.last_used);
            if idle_time > chrono::Duration::from_std(max_idle).unwrap() {
                removed += 1;
                false
            } else {
                true
            }
        });
        
        self.metrics.write().await.record_cleanup(removed);
        Ok(removed)
    }
}
```

#### Tests Required
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_per_session() {
        // Verify each session gets unique agent
        let manager = AgentManager::new(Default::default()).await;
        let session1 = session::Identifier::new();
        let session2 = session::Identifier::new();
        
        let agent1 = manager.get_agent(session1.clone()).await.unwrap();
        let agent2 = manager.get_agent(session2.clone()).await.unwrap();
        
        // Different sessions get different agents
        assert!(!Arc::ptr_eq(&agent1, &agent2));
        
        // Same session gets same agent
        let agent1_again = manager.get_agent(session1).await.unwrap();
        assert!(Arc::ptr_eq(&agent1, &agent1_again));
    }
    
    #[tokio::test]
    async fn test_cleanup_idle_agents() {
        // Verify idle agents are cleaned up
        let manager = AgentManager::new(Default::default()).await;
        let session = session::Identifier::new();
        
        let _agent = manager.get_agent(session.clone()).await.unwrap();
        
        // Immediately cleanup with 0 idle time
        let removed = manager.cleanup_idle(Duration::from_secs(0)).await.unwrap();
        assert_eq!(removed, 1);
        
        // Agent should be recreated on next access
        let _agent_new = manager.get_agent(session).await.unwrap();
    }
}
```

### Phase 2: Update goose-server (Week 3-4)

#### Goals
- Replace shared agent with AgentManager
- Maintain API compatibility
- Fix concurrency issues

#### Implementation

**File: `crates/goose-server/src/state.rs`**

```rust
// BEFORE (problematic shared agent):
// pub struct AppState {
//     agent: Arc<RwLock<AgentRef>>,
// }

// AFTER (session-isolated agents):
use goose::agents::manager::AgentManager;

pub struct AppState {
    /// Manages per-session agents instead of sharing one
    agent_manager: Arc<AgentManager>,
    pub secret_key: String,
    pub scheduler: Arc<RwLock<Option<Arc<dyn SchedulerTrait>>>>,
    pub recipe_file_hash_map: Arc<Mutex<HashMap<String, PathBuf>>>,
    pub session_counter: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new(secret_key: String) -> Arc<AppState> {
        let agent_manager = Arc::new(
            tokio::runtime::Handle::current()
                .block_on(AgentManager::new(Default::default()))
        );
        
        Arc::new(Self {
            agent_manager,
            secret_key,
            scheduler: Arc::new(RwLock::new(None)),
            recipe_file_hash_map: Arc::new(Mutex::new(HashMap::new())),
            session_counter: Arc::new(AtomicUsize::new(0)),
        })
    }
    
    /// Get agent for a specific session
    pub async fn get_agent(&self, session_id: session::Identifier) -> Result<Arc<Agent>> {
        self.agent_manager.get_agent(session_id).await
    }
}
```

**File: `crates/goose-server/src/routes/reply.rs`**

```rust
// Update reply handler to use session-specific agent
async fn reply_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<ChatRequest>,
) -> Result<SseResponse, StatusCode> {
    verify_secret_key(&headers, &state)?;
    
    let session_id = request.session_id
        .ok_or_else(|| {
            tracing::error!("session_id is required");
            StatusCode::BAD_REQUEST
        })?;
    
    // CHANGE: Get session-specific agent instead of shared
    let agent = state.get_agent(
        session::Identifier::Name(session_id.clone())
    ).await.map_err(|e| {
        tracing::error!("Failed to get agent for session {}: {}", session_id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Rest of the handler remains unchanged
    // ...
}
```

### Phase 3: Tool Approval Bubbling (Week 5-6)

#### Goals
- Enable optional approval bubbling from subtasks to parents
- Maintain backward compatibility (default: autonomous)
- Support flexible bubbling strategies

#### Implementation

**File: `crates/goose/src/agents/approval.rs`**

```rust
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

/// Defines how subtask tool approvals are handled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalMode {
    /// Subtask handles all approvals locally (default, backward compatible)
    Autonomous,
    
    /// All approval requests bubble to parent
    BubbleAll,
    
    /// Selective bubbling based on tool names
    BubbleFiltered {
        tools: Vec<String>,
        default_action: ApprovalAction,
    },
}

impl Default for ApprovalMode {
    fn default() -> Self {
        Self::Autonomous // Maintain backward compatibility
    }
}

/// Channel for parent-child approval communication
pub struct ApprovalChannel {
    request_tx: mpsc::Sender<ApprovalRequest>,
    response_rx: mpsc::Receiver<ApprovalResponse>,
}

impl ApprovalChannel {
    pub fn new(buffer_size: usize) -> (Self, Self) {
        // Create bidirectional channels following existing pattern
        // from Agent::new() confirmation channels
        let (req_tx, req_rx) = mpsc::channel(buffer_size);
        let (resp_tx, resp_rx) = mpsc::channel(buffer_size);
        
        let parent = Self {
            request_tx: resp_tx,
            response_rx: req_rx,
        };
        
        let child = Self {
            request_tx: req_tx,
            response_rx: resp_rx,
        };
        
        (parent, child)
    }
}
```

**File: `crates/goose/src/agents/agent.rs` (minimal changes)**

```rust
// Add to Agent struct (only 4 new fields):
pub struct Agent {
    // ... ALL EXISTING FIELDS UNCHANGED ...
    
    /// Session this agent belongs to (for isolation)
    session_id: Option<session::Identifier>,
    
    /// Execution mode (interactive, background, subtask)
    execution_mode: ExecutionMode,
    
    /// Channel for receiving approval requests from subtasks
    subtask_approval_tx: Option<mpsc::Sender<ApprovalRequest>>,
    
    /// Channel for sending approval responses to subtasks
    subtask_approval_rx: Option<mpsc::Receiver<ApprovalResponse>>,
}

// Add new constructor variant (reuses existing new()):
impl Agent {
    /// Create agent for a specific session with execution mode
    pub fn new_with_session(
        session_id: session::Identifier,
        mode: ExecutionMode,
    ) -> Self {
        let mut agent = Self::new(); // Reuse ALL existing initialization
        agent.session_id = Some(session_id);
        agent.execution_mode = mode;
        agent
    }
    
    /// Handle approval request from subtask
    /// 
    /// This reuses the existing confirmation channel infrastructure
    /// to bubble approvals from subtasks through parents to users
    pub async fn handle_subtask_approval(
        &self,
        request: ApprovalRequest,
    ) -> ApprovalResponse {
        // Check if this needs to go to user
        if self.should_bubble_to_user(&request.tool_name) {
            // Reuse existing confirmation channel
            let _ = self.confirmation_tx.send((
                request.id.clone(),
                PermissionConfirmation {
                    principal_type: PrincipalType::Tool,
                    permission: Permission::Pending,
                }
            )).await;
            
            // Wait for user response using existing channel
            let mut rx = self.confirmation_rx.lock().await;
            while let Some((id, confirmation)) = rx.recv().await {
                if id == request.id {
                    return ApprovalResponse {
                        request_id: request.id,
                        decision: match confirmation.permission {
                            Permission::AllowOnce => ApprovalDecision::Approved,
                            _ => ApprovalDecision::Denied,
                        },
                        reason: None,
                    };
                }
            }
        }
        
        // Apply local policy
        self.apply_local_approval_policy(request).await
    }
}
```

### Phase 4: Migrate Scheduler (Week 7-8)

#### Goals
- Update scheduler to use AgentManager
- Remove duplicate agent creation logic
- Maintain job isolation

#### Implementation

**File: `crates/goose/src/scheduler.rs`**

```rust
// BEFORE: Creates new agent for each job
// let agent: Agent = Agent::new();

// AFTER: Use AgentManager
async fn run_scheduled_job_internal(
    job: ScheduledJob,
    agent_manager: Arc<AgentManager>,
    provider_override: Option<Arc<dyn GooseProvider>>,
) -> Result<String, JobExecutionError> {
    // Generate session ID for this job run
    let session_id = session::Identifier::Name(
        format!("schedule_{}_{}", job.id, Utc::now().timestamp())
    );
    
    // Get dedicated agent for this job execution
    let agent = agent_manager.get_agent(session_id.clone()).await
        .map_err(|e| JobExecutionError::Setup(e.to_string()))?;
    
    // Configure agent with recipe extensions
    if let Some(provider) = provider_override {
        agent.update_provider(provider).await?;
    }
    
    // Load and execute recipe (rest unchanged)
    let recipe = load_recipe(&job.source)?;
    
    // Execute using existing reply logic
    let session_config = SessionConfig {
        id: session_id,
        working_dir: std::env::current_dir()?,
        schedule_id: Some(job.id.clone()),
        execution_mode: Some("background".to_string()),
        max_turns: None,
        retry_config: recipe.retry.clone(),
    };
    
    // Rest of execution remains the same
    // ...
}
```

### Phase 5: Remove SubAgent (Week 9-10)

#### Goals
- Delete SubAgent implementation
- Convert SubAgent usage to regular Agent with ExecutionMode::SubTask
- Update tests

#### Files to Delete
- `crates/goose/src/agents/subagent.rs` (350 lines)
- `crates/goose/src/agents/subagent_handler.rs` (100 lines)
- `crates/goose/src/agents/subagent_task_config.rs` (50 lines)

#### Update Task Execution

**File: `crates/goose/src/agents/subagent_execution_tool/tasks.rs`**

```rust
// BEFORE: Uses SubAgent
// let subagent = SubAgent::new(task_config).await?;

// AFTER: Use regular Agent with SubTask mode
async fn handle_inline_recipe_task(
    task: Task,
    agent_manager: Arc<AgentManager>,
    parent_session: session::Identifier,
    cancellation_token: CancellationToken,
) -> Result<Value, String> {
    // Create session for this subtask
    let subtask_session = session::Identifier::Name(
        format!("subtask_{}", task.id)
    );
    
    // Get agent configured as subtask
    let agent = agent_manager.get_agent(subtask_session).await
        .map_err(|e| format!("Failed to create subtask agent: {}", e))?;
    
    // Configure as subtask with parent reference
    agent.set_execution_mode(ExecutionMode::SubTask {
        parent: parent_session,
        inherit: InheritConfig::default(),
        approval_mode: ApprovalMode::default(), // Autonomous by default
    });
    
    // Execute recipe using standard reply
    let recipe = task.payload.get("recipe")
        .ok_or("Missing recipe in payload")?;
    
    // Rest of execution using standard agent.reply()
    // ...
}
```

## Code Style Guidelines

### Follow Existing Patterns

1. **Async/Await Usage** (follow `session::storage` patterns):
```rust
// Good - follows existing pattern
pub async fn get_agent(&self, id: Identifier) -> Result<Arc<Agent>>

// Bad - inconsistent with codebase
pub fn get_agent(&self, id: Identifier) -> impl Future<Output = Result<Arc<Agent>>>
```

2. **Error Handling** (follow existing patterns):
```rust
// Good - descriptive errors with context
.map_err(|e| AgentError::CreationFailed(format!("Failed to create agent: {}", e)))?

// Bad - generic errors
.map_err(|_| AgentError::Failed)?
```

3. **Naming Conventions**:
```rust
// Good - descriptive names following existing patterns
pub struct AgentManager     // Not: AM, AgentMgr
pub async fn get_agent()    // Not: get(), fetch_agent()
session_id: Identifier      // Not: sid, sess_id

// Follow existing type aliases
type SessionId = session::Identifier;  // Reuse existing types
```

4. **Comments** (only for non-trivial logic):
```rust
// Good - explains why, not what
/// Clean up idle agents to manage memory
/// 
/// Following the pattern from session::storage::cleanup_old_sessions
pub async fn cleanup_idle(&self, max_idle: Duration) -> Result<usize>

// Bad - obvious comment
// Get the agent for the session
pub async fn get_agent(&self, session_id: SessionId)
```

### Reuse Existing Components

1. **Session Management**: Use `crates/goose/src/session/` types and patterns
2. **Async Patterns**: Follow tokio usage from existing code
3. **Channel Patterns**: Follow Agent's existing confirmation channel design
4. **Storage Patterns**: Follow session::storage for persistence
5. **Error Types**: Extend existing error types where possible

## Testing Strategy

### Unit Tests
- Each new component gets comprehensive unit tests
- Follow existing test patterns from `agent.rs` and `session/storage.rs`
- Use `tokio::test` for async tests

### Integration Tests
```rust
// crates/goose/tests/agent_manager_integration.rs
#[tokio::test]
async fn test_multi_session_isolation() {
    // Verify sessions don't interfere
}

#[tokio::test]
async fn test_approval_bubbling() {
    // Verify approvals flow correctly
}
```

### Edge Cases to Test
1. **Concurrent session access** - Multiple threads accessing same session
2. **Agent cleanup during execution** - Idle cleanup while agent is active
3. **Approval timeout** - What happens when parent doesn't respond
4. **Circular approval chains** - Subtask of subtask approval loops
5. **Memory pressure** - Behavior when too many agents exist
6. **Provider switching** - Changing provider mid-execution
7. **Extension conflicts** - Loading/unloading extensions concurrently

## Migration Strategy

### Backward Compatibility
1. **Phase approach** - Each phase maintains working system
2. **Feature flags** - Can toggle between old/new behavior
3. **Adapter layers** - Old APIs continue working during transition
4. **Gradual rollout** - Test in staging before production

### Rollback Plan
Each phase can be rolled back independently:
- Phase 1: Remove AgentManager, no impact
- Phase 2: Revert to shared agent (known issues remain)
- Phase 3: Disable approval bubbling (autonomous only)
- Phase 4: Revert scheduler changes
- Phase 5: Keep SubAgent (more code but works)

## Success Criteria

### Functional Requirements
- [ ] Each session gets unique agent
- [ ] No state bleeding between sessions
- [ ] Approval bubbling works when enabled
- [ ] All existing tests pass
- [ ] Performance benchmarks maintained

### Code Quality Metrics
- [ ] 3,400+ lines removed
- [ ] 85-90% of Agent unchanged
- [ ] Test coverage > 80%
- [ ] No new clippy warnings
- [ ] Documentation complete

### Performance Targets
- [ ] Agent creation < 10ms
- [ ] Memory per agent < 10MB
- [ ] Cleanup runs < 100ms
- [ ] No mutex contention hotspots

## Timeline

| Week | Phase | Deliverable |
|------|-------|-------------|
| 1-2 | Phase 1 | AgentManager implementation |
| 3-4 | Phase 2 | goose-server migration |
| 5-6 | Phase 3 | Approval bubbling |
| 7-8 | Phase 4 | Scheduler migration |
| 9-10 | Phase 5 | SubAgent removal |
| 11-12 | Cleanup | Documentation, optimization |

## Risk Mitigation

### Technical Risks
1. **Memory usage increase**
   - Mitigation: Agent pooling, aggressive cleanup
   - Monitoring: Track memory per session

2. **Performance regression**
   - Mitigation: Benchmark before/after each phase
   - Monitoring: Response time metrics

3. **Breaking changes**
   - Mitigation: Adapter layers, gradual migration
   - Testing: Comprehensive integration tests

### Process Risks
1. **Scope creep**
   - Mitigation: Strict phase boundaries
   - Review: Weekly progress checks

2. **Merge conflicts**
   - Mitigation: Small, focused PRs
   - Strategy: Rebase frequently

## References

### Key Documents
- [Discussion #4389](https://github.com/block/goose/discussions/4389) - Original proposal
- [PR #4311](https://github.com/block/goose/pull/4311) - Dynamic task unification (completed)
- [PR #4216](https://github.com/block/goose/pull/4216) - Session-aware agents (in progress)

### Related Code
- `crates/goose/src/agents/` - Agent implementation
- `crates/goose/src/session/` - Session management patterns
- `crates/goose/src/recipe/` - Recipe system
- `crates/goose-server/src/` - Server implementation

## Conclusion

This implementation plan provides a clear, phased approach to unifying Goose's agent execution model. By following existing patterns, reusing proven code, and maintaining backward compatibility, we can achieve a 30-35% code reduction while fixing critical bugs and enabling true multi-user support.

The key to success is keeping changes minimal - 85-90% of the Agent stays unchanged, we're just wrapping it better and removing duplicates. Each phase is independently valuable and can be rolled back if needed, reducing risk while delivering continuous improvement.

The end result will be a simpler, more maintainable, and more capable system that provides a solid foundation for Goose's future growth.
