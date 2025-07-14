// =======================================
// Module: Dynamic Task Tools
// Handles creation of tasks dynamically without sub-recipes
// =======================================
use crate::agents::recipe_tools::sub_recipe_tools::{
    EXECUTION_MODE_PARALLEL, EXECUTION_MODE_SEQUENTIAL,
};
use crate::agents::sub_recipe_execution_tool::lib::Task;
use crate::agents::tool_execution::ToolCallResult;
use mcp_core::{tool::ToolAnnotations, Content, Tool, ToolError};
use serde_json::{json, Value};

pub const DYNAMIC_TASK_TOOL_NAME_PREFIX: &str = "dynamic_task__create_task";

pub fn create_dynamic_task_tool() -> Tool {
    Tool::new(
        format!("{}", DYNAMIC_TASK_TOOL_NAME_PREFIX),
        format!(
            "Creates a dynamic task object(s) based on textual instructions. \
            Provide an array of parameter sets in the 'task_parameters' field:\n\
            - For a single task: provide an array with one parameter set\n\
            - For multiple tasks: provide an array with multiple parameter sets, each with different values\n\n\
            Each task will run the same text instruction but with different parameter values. \
            This is useful when you need to execute the same instruction multiple times with varying inputs. \
            After creating the task list, pass it to the task executor to run all tasks."
        ),
        json!({
            "type": "object",
            "properties": {
                "task_parameters": {
                    "type": "array",
                    "description": "Array of parameter sets for creating tasks. \
                        For a single task, provide an array with one element. \
                        For multiple tasks, provide an array with multiple elements, each with different parameter values. \
                        If there is no parameter set, provide an empty array.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "text_instruction": {
                                "type": "string",
                                "description": "The text instruction to execute"
                            },
                            "timeout_seconds": {
                                "type": "integer",
                                "description": "Optional timeout for the task in seconds (default: 300)",
                                "minimum": 1
                            }
                        },
                        "required": ["text_instruction"]
                    }
                }
            }
        }),
        Some(ToolAnnotations {
            title: Some(format!("Dynamic Task Creation")),
            read_only_hint: false,
            destructive_hint: true,
            idempotent_hint: false,
            open_world_hint: true,
        }),
    )
}

fn extract_task_parameters(params: &Value) -> Vec<Value> {
    params
        .get("task_parameters")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

fn create_text_instruction_tasks_from_params(task_params: &[Value]) -> Vec<Task> {
    task_params
        .iter()
        .map(|task_param| {
            let text_instruction = task_param
                .get("text_instruction")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let timeout_seconds = task_param
                .get("timeout_seconds")
                .and_then(|v| v.as_u64())
                .unwrap_or(300);

            let payload = json!({
                "text_instruction": text_instruction
            });

            Task {
                id: uuid::Uuid::new_v4().to_string(),
                task_type: "text_instruction".to_string(),
                timeout_in_seconds: Some(timeout_seconds),
                payload,
            }
        })
        .collect()
}

fn create_task_execution_payload(tasks: Vec<Task>, execution_mode: &str) -> Value {
    json!({
        "tasks": tasks,
        "execution_mode": execution_mode
    })
}

pub async fn create_dynamic_task(params: Value) -> ToolCallResult {
    let task_params_array = extract_task_parameters(&params);

    if task_params_array.is_empty() {
        return ToolCallResult::from(Err(ToolError::ExecutionError(
            "No task parameters provided".to_string(),
        )));
    }

    let tasks = create_text_instruction_tasks_from_params(&task_params_array);

    // Use parallel execution if there are multiple tasks, sequential for single task
    let execution_mode = if tasks.len() > 1 {
        EXECUTION_MODE_PARALLEL
    } else {
        EXECUTION_MODE_SEQUENTIAL
    };

    let task_execution_payload = create_task_execution_payload(tasks, execution_mode);

    let tasks_json = match serde_json::to_string(&task_execution_payload) {
        Ok(json) => json,
        Err(e) => {
            return ToolCallResult::from(Err(ToolError::ExecutionError(format!(
                "Failed to serialize task list: {}",
                e
            ))))
        }
    };
    ToolCallResult::from(Ok(vec![Content::text(tasks_json)]))
}
