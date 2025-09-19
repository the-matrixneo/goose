# Tool Approval Bubbling Design for SubTasks

## Problem Statement

Currently, SubAgents (used for dynamic tasks and sub-recipes) execute autonomously without the ability to request tool approvals from their parent agent or the user. This limits their usefulness in scenarios where:
1. Sensitive operations need user confirmation
2. Parent agents want to maintain control over certain tools
3. Security policies require approval for specific actions

## Design Goals

1. **Optional**: Approval bubbling should be opt-in, with autonomous execution as default
2. **Flexible**: Support different bubbling strategies (all, none, filtered)
3. **Transparent**: Clear chain of approval from subtask → parent → user
4. **Non-blocking**: Async communication between agents
5. **Backward Compatible**: Existing code continues to work unchanged

## Proposed Architecture

### 1. Execution Mode Enhancement

```rust
pub enum ExecutionMode {
    Interactive {
        streaming: bool,
        confirmations: bool,
    },
    Background {
        scheduled: Option<ScheduleInfo>,
        retry: Option<RetryConfig>,
    },
    SubTask {
        parent: SessionId,
        inherit: InheritConfig,
        approval_mode: ApprovalMode,  // NEW
    },
}

pub enum ApprovalMode {
    /// Subtask handles all approvals autonomously (default)
    Autonomous,
    
    /// Bubble all approval requests to parent
    BubbleAll,
    
    /// Bubble only specific tools to parent
    BubbleFiltered {
        tools: Vec<String>,
        default_action: ApprovalAction,
    },
    
    /// Use parent's approval policy
    InheritPolicy,
}

pub enum ApprovalAction {
    Approve,
    Deny,
    Bubble,  // Continue bubbling up
}
```

### 2. Parent-Child Communication

```rust
/// Channel for approval requests from child to parent
pub struct ApprovalChannel {
    /// Send approval requests to parent
    request_tx: mpsc::Sender<ApprovalRequest>,
    /// Receive approval responses from parent
    response_rx: mpsc::Receiver<ApprovalResponse>,
}

pub struct ApprovalRequest {
    /// Unique ID for this approval request
    pub id: String,
    /// ID of the requesting subtask
    pub subtask_id: SessionId,
    /// Tool being requested
    pub tool_name: String,
    /// Tool arguments
    pub tool_args: Value,
    /// Chain of agents (for debugging/audit)
    pub approval_chain: Vec<SessionId>,
}

pub struct ApprovalResponse {
    /// ID matching the request
    pub request_id: String,
    /// Approval decision
    pub decision: ApprovalDecision,
    /// Optional reason/message
    pub reason: Option<String>,
}

pub enum ApprovalDecision {
    Approved,
    Denied,
    /// Defer to another policy (e.g., timeout)
    Deferred,
}
```

### 3. Agent Modifications

#### Parent Agent
```rust
impl Agent {
    /// Handle approval requests from subtasks
    pub async fn handle_subtask_approval(
        &self,
        request: ApprovalRequest,
    ) -> ApprovalResponse {
        // Check if this should bubble to user
        if self.should_bubble_to_user(&request.tool_name) {
            // Send to user via existing confirmation channel
            self.send_user_confirmation(request).await
        } else {
            // Apply local policy
            self.apply_approval_policy(request).await
        }
    }
    
    /// Monitor for subtask approval requests
    pub async fn monitor_subtask_approvals(&self) {
        while let Some(request) = self.subtask_approval_rx.recv().await {
            let response = self.handle_subtask_approval(request).await;
            self.subtask_approval_tx.send(response).await;
        }
    }
}
```

#### SubAgent
```rust
impl SubAgent {
    /// Create with optional parent channel
    pub async fn new_with_parent(
        task_config: TaskConfig,
        parent_channel: Option<ApprovalChannel>,
    ) -> Result<Arc<Self>> {
        // ... existing creation logic ...
        
        Ok(Arc::new(SubAgent {
            // ... existing fields ...
            parent_channel,
            approval_mode: task_config.approval_mode,
        }))
    }
    
    /// Override dispatch_tool_call to handle approvals
    async fn dispatch_tool_call(&self, tool_call: ToolCall) -> ToolResult {
        // Check if approval needed
        if self.needs_approval(&tool_call) {
            match &self.approval_mode {
                ApprovalMode::Autonomous => {
                    // Handle locally (current behavior)
                    self.handle_local_approval(tool_call).await
                }
                ApprovalMode::BubbleAll => {
                    // Send to parent
                    self.request_parent_approval(tool_call).await
                }
                ApprovalMode::BubbleFiltered { tools, default_action } => {
                    if tools.contains(&tool_call.name) {
                        self.request_parent_approval(tool_call).await
                    } else {
                        match default_action {
                            ApprovalAction::Approve => self.execute_tool(tool_call).await,
                            ApprovalAction::Deny => Err("Tool denied by policy"),
                            ApprovalAction::Bubble => self.request_parent_approval(tool_call).await,
                        }
                    }
                }
                ApprovalMode::InheritPolicy => {
                    // Use parent's decision
                    self.request_parent_approval(tool_call).await
                }
            }
        } else {
            // No approval needed, execute directly
            self.execute_tool(tool_call).await
        }
    }
    
    async fn request_parent_approval(&self, tool_call: ToolCall) -> ToolResult {
        if let Some(ref channel) = self.parent_channel {
            let request = ApprovalRequest {
                id: Uuid::new_v4().to_string(),
                subtask_id: self.id.clone(),
                tool_name: tool_call.name.clone(),
                tool_args: tool_call.arguments.clone(),
                approval_chain: vec![self.id.clone()],
            };
            
            // Send request
            channel.request_tx.send(request).await?;
            
            // Wait for response (with timeout)
            match timeout(Duration::from_secs(60), channel.response_rx.recv()).await {
                Ok(Some(response)) if response.decision == ApprovalDecision::Approved => {
                    self.execute_tool(tool_call).await
                }
                _ => {
                    Err("Tool approval denied or timed out")
                }
            }
        } else {
            // No parent channel, handle locally
            self.handle_local_approval(tool_call).await
        }
    }
}
```

### 4. Dynamic Task Creation Update

```rust
// In dynamic_task_tools.rs
pub fn create_dynamic_task_tool() -> Tool {
    Tool::new(
        DYNAMIC_TASK_TOOL_NAME_PREFIX.to_string(),
        "... approval_mode: optional, controls how tool approvals are handled ...",
        object!({
            // ... existing fields ...
            "approval_mode": {
                "type": "string",
                "enum": ["autonomous", "bubble_all", "inherit_policy"],
                "default": "autonomous",
                "description": "How to handle tool approvals in subtasks"
            },
            "approval_tools": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Specific tools to bubble approvals for (when approval_mode is filtered)"
            }
        })
    )
}
```

## Implementation Phases

### Phase 1: Infrastructure
1. Add `ApprovalChannel` and related types
2. Add `approval_mode` to `TaskConfig`
3. Update `SubAgent::new()` to accept parent channel

### Phase 2: Communication
1. Implement approval request/response flow
2. Add timeout handling
3. Add approval chain tracking

### Phase 3: Integration
1. Update dynamic task creation
2. Modify sub-recipe execution
3. Update parent agent to monitor subtask approvals

### Phase 4: Testing
1. Test autonomous mode (backward compatibility)
2. Test bubble all mode
3. Test filtered bubbling
4. Test approval chains (subtask → subtask → parent → user)

## Example Usage

### Autonomous (Default)
```rust
// Current behavior - subtask handles everything
let task = create_dynamic_task(
    "Update the database",
    // No approval_mode specified, defaults to autonomous
);
```

### Bubble All Approvals
```rust
let task = create_dynamic_task(
    "Perform system maintenance",
    approval_mode: "bubble_all",  // All tool approvals go to parent
);
```

### Selective Bubbling
```rust
let task = create_dynamic_task(
    "Process user data",
    approval_mode: "bubble_filtered",
    approval_tools: ["database_write", "file_delete"],  // Only these bubble up
);
```

## Benefits

1. **Security**: Sensitive operations can require user approval even in subtasks
2. **Control**: Parent agents maintain oversight of subtask operations
3. **Flexibility**: Different approval strategies for different use cases
4. **Audit Trail**: Approval chain provides clear accountability
5. **Backward Compatible**: Existing code works unchanged

## Considerations

1. **Performance**: Approval requests add latency
2. **Deadlock**: Need to prevent circular approval dependencies
3. **Timeout**: What happens if parent doesn't respond?
4. **UI Impact**: How to show approval chains to users?
5. **Testing**: Complex approval chains need thorough testing

## Alternative Approaches Considered

1. **Shared Confirmation Channel**: All agents share one channel
   - Pros: Simple
   - Cons: No hierarchy, routing complexity

2. **Policy Objects**: Pass policy objects down
   - Pros: Declarative
   - Cons: Less flexible, harder to update

3. **Event Bus**: Central event system
   - Pros: Decoupled
   - Cons: Over-engineered for this use case

The proposed channel-based approach balances simplicity, flexibility, and maintainability.
