# PR #4311 Analysis: Dynamic Tasks Already Unified

## Summary
PR #4311 (merged 2025-09-03) already unified dynamic tasks with recipes by converting them to inline recipes. This means **Phase 3 of the proposed unification is already complete**.

## What Was Done

### 1. Dynamic Tasks Now Use Inline Recipes
- Dynamic tasks are converted to `Recipe` objects internally
- They use `TaskType::InlineRecipe` instead of the old text instruction approach
- Full recipe capabilities available (extensions, settings, retry, response schema, etc.)

### 2. Extension Control
Dynamic tasks now support precise extension control:
- **Omit field**: Use all current extensions (backward compatible)
- **Empty array `[]`**: No extensions (sandboxed)
- **Array with names**: Only specified extensions

### 3. Key Code Changes

#### Task Creation (`dynamic_task_tools.rs`)
```rust
pub fn task_params_to_inline_recipe(
    task_param: &Value,
    loaded_extensions: &[String],
) -> Result<Recipe> {
    // Converts dynamic task parameters to a Recipe object
    // Handles instructions/prompt, extensions, settings, etc.
}
```

#### Task Execution (`tasks.rs`)
```rust
async fn handle_inline_recipe_task(
    task: Task,
    mut task_config: TaskConfig,
    cancellation_token: CancellationToken,
) -> Result<Value, String> {
    // Executes inline recipe using SubAgent
    // Respects extension configuration
    // Supports return_last_only flag
}
```

#### Task Types (`task_types.rs`)
```rust
pub enum TaskType {
    InlineRecipe,  // Dynamic tasks use this
    SubRecipe,     // Traditional sub-recipes
}
```

### 4. Return Control
Added `return_last_only` flag:
- `true`: Return only the last message (reduces token usage)
- `false`: Return full conversation (default)

## Implications for Unified Architecture

### What's Already Done
- ✅ Dynamic tasks unified with recipe system
- ✅ Extension control implemented
- ✅ Inline recipe execution working
- ✅ SubAgent respects extension configuration

### What Still Needs Work
1. **Shared Agent Problem**: goose-server still uses single shared agent
2. **Scheduler Integration**: Still creates fresh agents per job
3. **Sub-Recipe Execution**: Still has dual paths (CLI vs SubAgent)
4. **Tool Approval Bubbling**: Not implemented for subtasks

## Tool Approval Bubbling Requirements

Currently, SubAgents execute autonomously without bubbling tool approvals to parent. For the unified architecture, we need:

### 1. Optional Approval Bubbling
```rust
pub struct InheritConfig {
    pub extensions: ExtensionInheritance,
    pub provider: bool,
    pub approval_bubbling: ApprovalBubbling,  // NEW
}

pub enum ApprovalBubbling {
    None,           // Autonomous (current default)
    All,            // Bubble all approvals to parent
    Filtered(Vec<String>),  // Only specific tools
}
```

### 2. Communication Channel
SubAgents need a way to communicate with parent:
```rust
pub struct SubAgent {
    // ... existing fields ...
    parent_channel: Option<ParentChannel>,  // NEW
}

pub struct ParentChannel {
    approval_tx: mpsc::Sender<ApprovalRequest>,
    approval_rx: mpsc::Receiver<ApprovalResponse>,
}
```

### 3. Approval Flow
```
SubAgent needs approval
    ↓
Send to parent via channel
    ↓
Parent Agent receives request
    ↓
Parent checks if user-facing
    ↓
If yes: Bubble to user
If no: Auto-approve or policy
    ↓
Send response to SubAgent
    ↓
SubAgent continues
```

## Updated Migration Path

### Phase 1: Create AgentManager ✅
### Phase 2: Update goose-server 
### Phase 3: ~~Unify Dynamic Tasks~~ ✅ ALREADY DONE
### Phase 4: Add Approval Bubbling (NEW)
### Phase 5: Migrate Scheduler
### Phase 6: Complete Integration
