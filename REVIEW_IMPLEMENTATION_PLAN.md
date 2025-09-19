# Review Implementation Plan

## Overview
Implementing two changes from PR #4311 review:
1. Remove legacy `text_instruction` path (lines 240-260 in dynamic_task_tools.rs)
2. Convert `task_type` from String to enum

## 1. Remove Legacy text_instruction Path

### Current State
- `dynamic_task_tools.rs` has a legacy path that checks for `text_instruction` field
- This creates a task with `task_type: "text_instruction"` 
- The `text_instruction` task type is still used elsewhere in the codebase
- **IMPORTANT**: We're NOT removing the `text_instruction` task type itself, just the legacy compatibility path in dynamic_task_tools

### Changes Required
- Remove lines 240-260 in `dynamic_task_tools.rs` (the if block checking for `text_instruction`)
- Update test `test_backward_compatibility` in `dynamic_task_tools_tests.rs` to expect failure
- Ensure all new tasks use `instructions` or `prompt` fields

### Files to Modify
- `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs`
- `crates/goose/tests/dynamic_task_tools_tests.rs`

## 2. Convert task_type to Enum

### Current State
- `task_type` is a String field in `Task` struct
- Three valid values: `"text_instruction"`, `"inline_recipe"`, `"sub_recipe"`
- Used in multiple places with string matching
- Serialized/deserialized as JSON strings

### Design Decision
Create `TaskType` enum with serde attributes for backward compatibility:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    TextInstruction,
    InlineRecipe,
    SubRecipe,
}
```

### Files to Modify

#### Core Changes
1. `crates/goose/src/agents/subagent_execution_tool/task_types.rs`
   - Define `TaskType` enum
   - Update `Task` struct to use `TaskType` instead of String
   - Update helper methods to use enum

2. `crates/goose/src/agents/subagent_execution_tool/tasks.rs`
   - Update match statement to use enum variants
   - Remove `.as_str()` call

3. `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs`
   - Update task creation to use `TaskType::InlineRecipe`
   - Remove the legacy text_instruction path

4. `crates/goose/src/agents/recipe_tools/sub_recipe_tools.rs`
   - Update task creation to use `TaskType::SubRecipe`

#### Display/UI Changes
5. `crates/goose/src/agents/subagent_execution_tool/notification_events.rs`
   - Update `TaskExecutionEvent` to use `TaskType` enum
   - May need Display impl for formatting

6. `crates/goose-cli/src/session/task_execution_display/tests.rs`
   - Update test data to use enum (with serde it should still accept strings)

#### Test Updates
7. `crates/goose/src/agents/subagent_execution_tool/utils/tests.rs`
   - Update test task creation to use enum

8. `crates/goose/tests/dynamic_task_tools_tests.rs`
   - Update backward compatibility test

### Serialization Considerations
- Use `#[serde(rename_all = "snake_case")]` to maintain JSON compatibility
- Existing JSON with string values should still deserialize correctly
- New serialization will use the same string values

### Display Implementation
Add Display trait for user-facing output:
```rust
impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::TextInstruction => write!(f, "text_instruction"),
            TaskType::InlineRecipe => write!(f, "inline_recipe"),
            TaskType::SubRecipe => write!(f, "sub_recipe"),
        }
    }
}
```

## Testing Strategy

1. Run existing tests to ensure backward compatibility
2. Update tests that create tasks directly
3. Add new tests for enum serialization/deserialization
4. Test that JSON with string values still works

## Order of Implementation

1. Create TaskType enum in task_types.rs
2. Update Task struct and methods
3. Update all task creation sites to use enum
4. Remove legacy text_instruction path
5. Update tests
6. Run tests and fix any issues
7. Run linter and formatter

## Potential Issues

1. **JSON Compatibility**: Need to ensure existing JSON payloads still work
2. **Display formatting**: CLI display needs to show human-readable format
3. **Test data**: Many tests use string literals that need updating
4. **Notification events**: Need to ensure events still serialize correctly

## Verification Steps

1. `cargo build` - Ensure compilation
2. `cargo test` - Run all tests
3. `cargo fmt` - Format code
4. `./scripts/clippy-lint.sh` - Run linter
5. Manual testing of task execution
