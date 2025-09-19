# Review Implementation TODO

## Task 1: Remove Legacy text_instruction Path
- [x] Remove legacy path in dynamic_task_tools.rs (lines 240-260)
- [x] Update test_backward_compatibility test to expect the new behavior
- [x] Verify no other code depends on this legacy path

## Task 2: Convert task_type to Enum
- [x] Create TaskType enum in task_types.rs
- [x] Add Display implementation for TaskType
- [x] Update Task struct to use TaskType enum
- [x] Update Task helper methods (get_sub_recipe, get_text_instruction)
- [x] Update tasks.rs to use enum in match statement
- [x] Update dynamic_task_tools.rs to use TaskType::InlineRecipe
- [x] Update sub_recipe_tools.rs to use TaskType::SubRecipe
- [x] Update notification_events.rs to use TaskType enum
- [x] Update CLI display tests to work with enum (TaskInfo still uses String for display)
- [x] Update utils tests to use enum
- [x] Add serialization/deserialization tests for TaskType

## Testing & Verification
- [x] Run cargo build
- [x] Run cargo test
- [x] Run cargo fmt
- [x] Run ./scripts/clippy-lint.sh (running)
- [x] Verify JSON backward compatibility
- [ ] Test task execution still works
