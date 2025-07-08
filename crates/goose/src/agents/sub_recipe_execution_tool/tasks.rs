use serde_json::Value;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

use crate::agents::sub_recipe_execution_tool::dashboard::TaskDashboard;
use crate::agents::sub_recipe_execution_tool::types::{Task, TaskResult, TaskStatus};

pub async fn process_task(
    task: &Task,
    timeout_seconds: u64,
    dashboard: Arc<TaskDashboard>,
) -> TaskResult {
    let task_clone = task.clone();
    let timeout_duration = Duration::from_secs(timeout_seconds);

    match timeout(timeout_duration, get_task_result(task_clone, dashboard)).await {
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
        Err(_) => TaskResult {
            task_id: task.id.clone(),
            status: TaskStatus::Failed,
            data: None,
            error: Some("Task timeout".to_string()),
        },
    }
}

async fn get_task_result(task: Task, dashboard: Arc<TaskDashboard>) -> Result<Value, String> {
    let (command, output_identifier) = build_command(&task)?;
    let (stdout_output, stderr_output, success) =
        run_command(command, &output_identifier, &task.id, dashboard).await?;

    if success {
        process_output(stdout_output)
    } else {
        Err(format!("Command failed:\n{}", stderr_output))
    }
}

fn build_command(task: &Task) -> Result<(Command, String), String> {
    let mut output_identifier = task.id.clone();
    let mut command = if task.task_type == "sub_recipe" {
        let sub_recipe_name = task
            .get_sub_recipe_name()
            .ok_or("Missing sub_recipe name")?;
        let path = task
            .get_sub_recipe_path()
            .ok_or("Missing sub_recipe path")?;
        let command_parameters = task
            .get_command_parameters()
            .ok_or("Missing command_parameters")?;

        output_identifier = format!("sub-recipe {}", sub_recipe_name);
        let mut cmd = Command::new("goose");
        cmd.arg("run").arg("--recipe").arg(path);

        for (key, value) in command_parameters {
            let key_str = key.to_string();
            let value_str = value.as_str().unwrap_or(&value.to_string()).to_string();
            cmd.arg("--params")
                .arg(format!("{}={}", key_str, value_str));
        }
        cmd
    } else {
        let text = task
            .get_text_instruction()
            .ok_or("Missing text_instruction")?;
        let mut cmd = Command::new("goose");
        cmd.arg("run").arg("--text").arg(text);
        cmd
    };

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    Ok((command, output_identifier))
}

async fn run_command(
    mut command: Command,
    output_identifier: &str,
    task_id: &str,
    dashboard: Arc<TaskDashboard>,
) -> Result<(String, String, bool), String> {
    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn goose: {}", e))?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_task =
        spawn_output_reader(stdout, output_identifier, false, task_id, dashboard.clone());
    let stderr_task =
        spawn_output_reader(stderr, output_identifier, true, task_id, dashboard.clone());

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
    dashboard: Arc<TaskDashboard>,
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
                // Use dashboard's smart output handling based on display mode
                dashboard.send_live_output(&task_id, &line).await;
            } else {
                eprintln!("[stderr for {}] {}", output_identifier, line);
            }
        }
        buffer
    })
}

fn process_output(stdout_output: String) -> Result<Value, String> {
    let last_line = stdout_output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .next_back()
        .unwrap_or("");

    if let (Some(start), Some(end)) = (last_line.find('{'), last_line.rfind('}')) {
        if start < end {
            let potential_json = &last_line[start..=end];

            if serde_json::from_str::<Value>(potential_json).is_ok() {
                return Ok(Value::String(potential_json.to_string()));
            }
        }
    }
    Ok(Value::String(stdout_output))
}
