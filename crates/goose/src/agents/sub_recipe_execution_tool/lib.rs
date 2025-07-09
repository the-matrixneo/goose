use crate::agents::sub_recipe_execution_tool::executor::{
    execute_single_task, execute_tasks_in_parallel,
};
pub use crate::agents::sub_recipe_execution_tool::types::{
    Config, ExecutionResponse, ExecutionStats, SharedState, Task, TaskResult, TaskStatus,
};

use mcp_core::protocol::JsonRpcMessage;
use serde_json::Value;
use tokio::sync::mpsc;

pub async fn execute_tasks(
    input: Value,
    execution_mode: &str,
    notifier: mpsc::Sender<JsonRpcMessage>,
) -> Result<Value, String> {
    let tasks: Vec<Task> =
        serde_json::from_value(input.get("tasks").ok_or("Missing tasks field")?.clone())
            .map_err(|e| format!("Failed to parse tasks: {}", e))?;

    let config: Config = if let Some(config_value) = input.get("config") {
        serde_json::from_value(config_value.clone())
            .map_err(|e| format!("Failed to parse config: {}", e))?
    } else {
        Config::default()
    };

    let task_count = tasks.len();
    match execution_mode {
        "sequential" => {
            if task_count == 1 {
                let response = execute_single_task(&tasks[0], notifier).await;
                handle_response(response)
            } else {
                Err("Sequential execution mode requires exactly one task".to_string())
            }
        }
        "parallel" => {
            let response: ExecutionResponse =
                execute_tasks_in_parallel(tasks, config, notifier).await;
            handle_response(response)
        }
        _ => Err("Invalid execution mode".to_string()),
    }
}

fn handle_response(response: ExecutionResponse) -> Result<Value, String> {
    if response.stats.failed > 0 {
        let failed_tasks: Vec<String> = response
            .results
            .iter()
            .filter(|r| matches!(r.status, TaskStatus::Failed))
            .map(|r| {
                let error_msg = r.error.as_deref().unwrap_or("Unknown error");
                let partial_output = r
                    .data
                    .as_ref()
                    .and_then(|d| d.get("partial_output"))
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or("No output captured");

                format!(
                    "Task '{}' ({}): {}, \noutput: {}",
                    r.task_id,
                    get_task_description(r),
                    error_msg,
                    partial_output
                )
            })
            .collect();

        let error_summary = format!(
            "{}/{} tasks failed:\n{}",
            response.stats.failed,
            response.stats.total_tasks,
            failed_tasks.join("\n")
        );

        return Err(error_summary);
    }
    serde_json::to_value(response).map_err(|e| format!("Failed to serialize response: {}", e))
}

fn get_task_description(result: &TaskResult) -> String {
    // We'd need to reconstruct task info from the result or pass it through
    // For now, just use the task_id as placeholder
    format!("ID: {}", result.task_id)
}
