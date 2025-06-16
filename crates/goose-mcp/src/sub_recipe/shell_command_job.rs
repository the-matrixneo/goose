use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::{collections::HashMap, process::Stdio};
use tokio::{
    process::Command,
    sync::{mpsc, Mutex, Semaphore},
};
use uuid::Uuid;

// Define job and status types
#[derive(Debug)]
pub struct Job {
    id: Uuid,
    command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum JobStatus {
    Queued,
    Running,
    Completed { output: String },
    Failed { error: String },
}

// Type aliases for convenience
pub type JobMap = Arc<Mutex<HashMap<Uuid, JobStatus>>>;
pub type JobSender = mpsc::Sender<Job>;

/// Submit a new job into the system
pub async fn submit_job(command: String, sender: &JobSender, job_map: &JobMap) -> Uuid {
    let job_id = Uuid::new_v4();
    let job = Job {
        id: job_id,
        command,
    };

    job_map.lock().await.insert(job_id, JobStatus::Queued);
    sender.send(job).await.unwrap(); // Ignore send errors for now

    job_id
}

/// Run the command and capture result
async fn run_shell_command(cmd: String) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to execute: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Start the central dispatcher
pub async fn start_dispatcher(
    mut receiver: mpsc::Receiver<Job>,
    job_map: JobMap,
    max_concurrent_jobs: usize,
) {
    let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));

    tokio::spawn(async move {
        while let Some(job) = receiver.recv().await {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let job_map = job_map.clone();

            // Mark as running
            {
                let mut map = job_map.lock().await;
                map.insert(job.id, JobStatus::Running);
            }

            // Spawn a task per job
            tokio::spawn(async move {
                let result = run_shell_command(job.command).await;

                let mut map = job_map.lock().await;
                match result {
                    Ok(output) => {
                        map.insert(job.id, JobStatus::Completed { output });
                    }
                    Err(error) => {
                        map.insert(job.id, JobStatus::Failed { error });
                    }
                }

                // Drop the permit to allow another job to run
                drop(permit);
            });
        }
    });
}

// Schema for the shell command job tool
pub const SHELL_COMMAND_JOB_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "action": {
            "type": "string",
            "enum": ["submit", "status", "list"],
            "description": "Action to perform: submit a new job, check status of a job, or list all jobs"
        },
        "command": {
            "type": "string",
            "description": "Shell command to execute (required when action is 'submit')"
        },
        "job_id": {
            "type": "string",
            "description": "UUID of the job to check (required when action is 'status')"
        },
        "task_id": {
            "type": "string",
            "description": "ID of the task to run the command for"
        }
    },
    "required": ["action"],
    "additionalProperties": false
}"#;

// Description for the shell command job tool
pub const SHELL_COMMAND_JOB_DESCRIPTION: &str = r#"
A tool for running shell commands asynchronously as jobs. You can submit commands to be executed,
check the status of running jobs, and list all jobs in the system.

- Use 'submit' action with a 'command' parameter and an optional 'task_id' parameter to run a shell command
- Use 'status' action with a 'job_id' parameter to check a specific job's status
- Use 'list' action to see all jobs in the system

Example usage:
1. Submit a job: {"action": "submit", "command": "sleep 5 && echo 'Hello world'", "task_id": "first_task"}
2. Check status: {"action": "status", "job_id": "123e4567-e89b-12d3-a456-426614174000"}
3. List all jobs: {"action": "list"}
"#;

pub fn validate_shell_job_params(
    params: Value,
) -> Result<(String, Option<String>, Option<String>), String> {
    let action = params["action"]
        .as_str()
        .ok_or_else(|| "Missing 'action' parameter".to_string())?
        .to_string();

    let command = params["command"].as_str().map(|s| s.to_string());
    let job_id = params["job_id"].as_str().map(|s| s.to_string());

    match action.as_str() {
        "submit" => {
            if command.is_none() {
                return Err("'command' parameter is required for 'submit' action".to_string());
            }
        }
        "status" => {
            if job_id.is_none() {
                return Err("'job_id' parameter is required for 'status' action".to_string());
            }
        }
        "list" => {} // No additional parameters needed
        _ => return Err(format!("Unknown action: {}", action)),
    }

    Ok((action, command, job_id))
}
