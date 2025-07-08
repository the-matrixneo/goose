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
                serde_json::to_value(response)
                    .map_err(|e| format!("Failed to serialize response: {}", e))
            } else {
                Err("Sequential execution mode requires exactly one task".to_string())
            }
        }
        "parallel" => {
            let response = execute_tasks_in_parallel(tasks, config, notifier).await;
            serde_json::to_value(response)
                .map_err(|e| format!("Failed to serialize response: {}", e))
        }
        _ => Err("Invalid execution mode".to_string()),
    }
}
