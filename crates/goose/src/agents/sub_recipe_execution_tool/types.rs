use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::agents::sub_recipe_execution_tool::dashboard::TaskDashboard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: String,
    pub payload: Value,
}

impl Task {
    pub fn get_sub_recipe(&self) -> Option<&Map<String, Value>> {
        if self.task_type == "sub_recipe" {
            self.payload.get("sub_recipe").and_then(|sr| sr.as_object())
        } else {
            None
        }
    }

    pub fn get_command_parameters(&self) -> Option<&Map<String, Value>> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("command_parameters"))
            .and_then(|cp| cp.as_object())
    }

    pub fn get_sub_recipe_name(&self) -> Option<&str> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("name"))
            .and_then(|name| name.as_str())
    }

    pub fn get_sub_recipe_path(&self) -> Option<&str> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("recipe_path"))
            .and_then(|path| path.as_str())
    }

    pub fn get_text_instruction(&self) -> Option<&str> {
        if self.task_type != "sub_recipe" {
            self.payload
                .get("text_instruction")
                .and_then(|text| text.as_str())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub task: Task,
    pub status: TaskStatus,
    pub start_time: Option<tokio::time::Instant>,
    pub end_time: Option<tokio::time::Instant>,
    pub result: Option<TaskResult>,
    pub current_output: String,
}

impl TaskInfo {
    pub fn error(&self) -> Option<&String> {
        self.result.as_ref().and_then(|r| r.error.as_ref())
    }

    pub fn data(&self) -> Option<&Value> {
        self.result.as_ref().and_then(|r| r.data.as_ref())
    }
}

pub struct SharedState {
    pub task_receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<Task>>>,
    pub result_sender: mpsc::Sender<TaskResult>,
    pub active_workers: Arc<AtomicUsize>,
    pub dashboard: Arc<TaskDashboard>,
}

impl SharedState {
    pub fn increment_active_workers(&self) {
        self.active_workers.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_active_workers(&self) {
        self.active_workers.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_max_workers")]
    pub max_workers: usize,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_initial_workers")]
    pub initial_workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_workers: default_max_workers(),
            timeout_seconds: default_timeout(),
            initial_workers: default_initial_workers(),
        }
    }
}

fn default_max_workers() -> usize {
    10
}
fn default_timeout() -> u64 {
    300
}
fn default_initial_workers() -> usize {
    2
}

#[derive(Debug, Serialize)]
pub struct ExecutionStats {
    pub total_tasks: usize,
    pub completed: usize,
    pub failed: usize,
    pub execution_time_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResponse {
    pub status: String,
    pub results: Vec<TaskResult>,
    pub stats: ExecutionStats,
}
