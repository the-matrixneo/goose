use serde_json::Value;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::agents::extension_manager::ExtensionManager;
use crate::agents::sub_recipe_execution_tool::task_execution_tracker::TaskExecutionTracker;
use crate::agents::sub_recipe_execution_tool::task_types::{Task, TaskResult, TaskStatus};
use crate::agents::subagent_handler::run_complete_subagent_task;
use crate::providers::base::Provider;
use mcp_core::protocol::JsonRpcMessage;
use tokio::sync::RwLock;

const DEFAULT_TASK_TIMEOUT_SECONDS: u64 = 300;

pub async fn process_task(
    task: &Task,
    task_execution_tracker: Arc<TaskExecutionTracker>,
    mcp_tx: mpsc::Sender<JsonRpcMessage>,
    provider: Option<Arc<dyn Provider>>,
    extension_manager: Option<Arc<RwLock<ExtensionManager>>>,
) -> TaskResult {
    let timeout_in_seconds = task
        .timeout_in_seconds
        .unwrap_or(DEFAULT_TASK_TIMEOUT_SECONDS);
    let task_clone = task.clone();
    let timeout_duration = Duration::from_secs(timeout_in_seconds);

    let task_execution_tracker_clone = task_execution_tracker.clone();
    match timeout(
        timeout_duration,
        get_task_result(
            task_clone,
            task_execution_tracker,
            mcp_tx,
            provider,
            extension_manager,
        ),
    )
    .await
    {
        Ok(Ok(data)) => TaskResult {
            task_id: task.id.clone(),
            status: TaskStatus::Completed,
            data: Some(data),
            error: None,
        },
        Ok(Err(error)) => TaskResult {
            task_id: task.id.clone(),
            status: TaskStatus::Failed,
            data: None,
            error: Some(error),
        },
        Err(_) => {
            let current_output = task_execution_tracker_clone
                .get_current_output(&task.id)
                .await
                .unwrap_or_default();

            TaskResult {
                task_id: task.id.clone(),
                status: TaskStatus::Failed,
                data: Some(serde_json::json!({
                    "partial_output": current_output
                })),
                error: Some(format!("Task timed out after {}s", timeout_in_seconds)),
            }
        }
    }
}

async fn get_task_result(
    task: Task,
    task_execution_tracker: Arc<TaskExecutionTracker>,
    mcp_tx: mpsc::Sender<JsonRpcMessage>,
    provider: Option<Arc<dyn Provider>>,
    extension_manager: Option<Arc<RwLock<ExtensionManager>>>,
) -> Result<Value, String> {
    if task.task_type == "text_instruction" {
        // Handle text_instruction tasks using subagent system
        handle_text_instruction_task(
            task,
            task_execution_tracker,
            mcp_tx,
            provider,
            extension_manager,
        )
        .await
    } else {
        // Handle sub_recipe tasks using command execution
        let (command, output_identifier) = build_command(&task)?;
        let (stdout_output, stderr_output, success) = run_command(
            command,
            &output_identifier,
            &task.id,
            task_execution_tracker,
        )
        .await?;

        if success {
            process_output(stdout_output)
        } else {
            Err(format!("Command failed:\n{}", stderr_output))
        }
    }
}

async fn handle_text_instruction_task(
    task: Task,
    task_execution_tracker: Arc<TaskExecutionTracker>,
    mcp_tx: mpsc::Sender<JsonRpcMessage>,
    provider: Option<Arc<dyn Provider>>,
    extension_manager: Option<Arc<RwLock<ExtensionManager>>>,
) -> Result<Value, String> {
    let text_instruction = task
        .get_text_instruction()
        .ok_or_else(|| format!("Task {}: Missing text_instruction", task.id))?;

    // Check if we have the required dependencies for subagent execution
    let (provider, extension_manager) = match (provider, extension_manager) {
        (Some(p), Some(em)) => (p, em),
        _ => {
            return Err(
                "Text instruction tasks require provider and extension_manager".to_string(),
            );
        }
    };

    // Create arguments for the subagent task
    let arguments = serde_json::json!({
        "task": text_instruction,
        "instructions": "You are a helpful assistant. Execute the given task and provide a clear, concise response.",
        "max_turns": 5,
        "timeout_seconds": task.timeout_in_seconds.unwrap_or(300)
    });

    // Execute the text instruction using the subagent system
    match run_complete_subagent_task(arguments, mcp_tx, provider, Some(extension_manager)).await {
        Ok(contents) => {
            // Extract the text content from the result
            let result_text = contents
                .into_iter()
                .filter_map(|content| match content {
                    mcp_core::Content::Text(text) => Some(text.text),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");

            Ok(serde_json::json!({
                "result": result_text
            }))
        }
        Err(e) => Err(format!("Subagent execution failed: {}", e)),
    }
}

fn build_command(task: &Task) -> Result<(Command, String), String> {
    let task_error = |field: &str| format!("Task {}: Missing {}", task.id, field);

    let mut output_identifier = task.id.clone();
    let mut command = if task.task_type == "sub_recipe" {
        let sub_recipe_name = task
            .get_sub_recipe_name()
            .ok_or_else(|| task_error("sub_recipe name"))?;
        let path = task
            .get_sub_recipe_path()
            .ok_or_else(|| task_error("sub_recipe path"))?;
        let command_parameters = task
            .get_command_parameters()
            .ok_or_else(|| task_error("command_parameters"))?;

        output_identifier = format!("sub-recipe {}", sub_recipe_name);
        let mut cmd = Command::new("goose");
        cmd.arg("run").arg("--recipe").arg(path).arg("--no-session");

        for (key, value) in command_parameters {
            let key_str = key.to_string();
            let value_str = value.as_str().unwrap_or(&value.to_string()).to_string();
            cmd.arg("--params")
                .arg(format!("{}={}", key_str, value_str));
        }
        cmd
    } else {
        // This branch should not be reached for text_instruction tasks anymore
        // as they are handled in handle_text_instruction_task
        return Err("Text instruction tasks are handled separately".to_string());
    };

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    Ok((command, output_identifier))
}

async fn run_command(
    mut command: Command,
    output_identifier: &str,
    task_id: &str,
    task_execution_tracker: Arc<TaskExecutionTracker>,
) -> Result<(String, String, bool), String> {
    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn goose: {}", e))?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_task = spawn_output_reader(
        stdout,
        output_identifier,
        false,
        task_id,
        task_execution_tracker.clone(),
    );
    let stderr_task = spawn_output_reader(
        stderr,
        output_identifier,
        true,
        task_id,
        task_execution_tracker.clone(),
    );

    let status = child
        .wait()
        .await
        .map_err(|e| format!("Failed to wait for process: {}", e))?;

    let stdout_output = stdout_task.await.unwrap();
    let stderr_output = stderr_task.await.unwrap();

    Ok((stdout_output, stderr_output, status.success()))
}

fn spawn_output_reader(
    reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
    output_identifier: &str,
    is_stderr: bool,
    task_id: &str,
    task_execution_tracker: Arc<TaskExecutionTracker>,
) -> tokio::task::JoinHandle<String> {
    let output_identifier = output_identifier.to_string();
    let task_id = task_id.to_string();
    tokio::spawn(async move {
        let mut buffer = String::new();
        let mut lines = BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            buffer.push_str(&line);
            buffer.push('\n');

            if !is_stderr {
                task_execution_tracker
                    .send_live_output(&task_id, &line)
                    .await;
            } else {
                tracing::warn!("Task stderr [{}]: {}", output_identifier, line);
            }
        }
        buffer
    })
}

fn extract_json_from_line(line: &str) -> Option<String> {
    let start = line.find('{')?;
    let end = line.rfind('}')?;

    if start >= end {
        return None;
    }

    let potential_json = &line[start..=end];
    if serde_json::from_str::<Value>(potential_json).is_ok() {
        Some(potential_json.to_string())
    } else {
        None
    }
}

fn process_output(stdout_output: String) -> Result<Value, String> {
    let last_line = stdout_output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .next_back()
        .unwrap_or("");

    if let Some(json_string) = extract_json_from_line(last_line) {
        Ok(Value::String(json_string))
    } else {
        Ok(Value::String(stdout_output))
    }
}
