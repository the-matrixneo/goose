use serde_json::Value;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

use crate::agents::sub_recipe_execution_tool::task_execution_tracker::TaskExecutionTracker;
use crate::agents::sub_recipe_execution_tool::types::{Task, TaskResult, TaskStatus};
use crate::agents::subagent_types::SpawnSubAgentArgs;

const DEFAULT_TASK_TIMEOUT_SECONDS: u64 = 300;

// Type for subagent execution callback
pub type SubagentExecutor = Box<dyn Fn(SpawnSubAgentArgs) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>> + Send + Sync>;

pub async fn process_task(
    task: &Task,
    task_execution_tracker: Arc<TaskExecutionTracker>,
    subagent_executor: Option<SubagentExecutor>,
) -> TaskResult {
    let timeout_in_seconds = task
        .timeout_in_seconds
        .unwrap_or(DEFAULT_TASK_TIMEOUT_SECONDS);
    let task_clone = task.clone();
    let timeout_duration = Duration::from_secs(timeout_in_seconds);

    let task_execution_tracker_clone = task_execution_tracker.clone();
    match timeout(
        timeout_duration,
        get_task_result(task_clone, task_execution_tracker, subagent_executor),
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
    subagent_executor: Option<SubagentExecutor>,
) -> Result<Value, String> {
    match task.task_type.as_str() {
        "subagent" => execute_subagent_task(task, task_execution_tracker, subagent_executor).await,
        _ => {
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
}

async fn execute_subagent_task(
    task: Task,
    task_execution_tracker: Arc<TaskExecutionTracker>,
    subagent_executor: Option<SubagentExecutor>,
) -> Result<Value, String> {
    // Get subagent parameters
    let message = task
        .get_subagent_message()
        .ok_or_else(|| "Missing subagent message".to_string())?
        .to_string();

    let recipe_name = task.get_subagent_recipe_name().map(|s| s.to_string());
    let instructions = task.get_subagent_instructions().map(|s| s.to_string());

    // Create subagent arguments
    let mut args = if let Some(recipe_name) = recipe_name {
        SpawnSubAgentArgs::new_with_recipe(recipe_name, message.clone())
    } else if let Some(instructions) = instructions {
        SpawnSubAgentArgs::new_with_instructions(instructions, message.clone())
    } else {
        return Err("Either recipe_name or instructions must be provided for subagent task".to_string());
    };

    // Set optional parameters
    if let Some(max_turns) = task.get_subagent_max_turns() {
        args = args.with_max_turns(max_turns);
    }

    if let Some(timeout) = task.get_subagent_timeout() {
        args = args.with_timeout(timeout);
    }

    // Execute the subagent task
    let executor = subagent_executor.ok_or_else(|| {
        "Subagent executor not provided. Cannot execute subagent tasks.".to_string()
    })?;

    let result = executor(args).await?;
    
    // Return the result as JSON
    Ok(serde_json::json!({
        "subagent_result": result,
        "task_id": task.id
    }))
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
    } else if task.task_type == "text_instruction" {
        let text = task
            .get_text_instruction()
            .ok_or_else(|| task_error("text_instruction"))?;
        let mut cmd = Command::new("goose");
        cmd.arg("run").arg("--text").arg(text);
        cmd
    } else {
        return Err(format!("Unsupported task type: {}", task.task_type));
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
