use mcp_core::protocol::{JsonRpcMessage, JsonRpcNotification};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration, Instant};

use crate::agents::sub_recipe_execution_tool::types::{Task, TaskInfo, TaskResult, TaskStatus};
use crate::agents::sub_recipe_execution_tool::utils::{count_by_status, get_task_name};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMode {
    MultipleTasksOutput,
    SingleTaskOutput,
}

const THROTTLE_INTERVAL_MS: u64 = 1000;

fn format_task_metadata(task_info: &TaskInfo) -> String {
    if let Some(params) = task_info.task.get_command_parameters() {
        if params.is_empty() {
            return String::new();
        }

        params
            .iter()
            .map(|(key, value)| {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                format!("{}={}", key, value_str)
            })
            .collect::<Vec<_>>()
            .join(",")
    } else {
        String::new()
    }
}

pub struct TaskExecutionTracker {
    tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    last_refresh: Arc<RwLock<Instant>>,
    notifier: mpsc::Sender<JsonRpcMessage>,
    display_mode: DisplayMode,
}

impl TaskExecutionTracker {
    pub fn new(
        tasks: Vec<Task>,
        display_mode: DisplayMode,
        notifier: mpsc::Sender<JsonRpcMessage>,
    ) -> Self {
        let task_map = tasks
            .into_iter()
            .map(|task| {
                let task_id = task.id.clone();
                (
                    task_id,
                    TaskInfo {
                        task,
                        status: TaskStatus::Pending,
                        start_time: None,
                        end_time: None,
                        result: None,
                        current_output: String::new(),
                    },
                )
            })
            .collect();

        Self {
            tasks: Arc::new(RwLock::new(task_map)),
            last_refresh: Arc::new(RwLock::new(Instant::now())),
            notifier,
            display_mode,
        }
    }

    pub async fn start_task(&self, task_id: &str) {
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(task_id) {
            task_info.status = TaskStatus::Running;
            task_info.start_time = Some(Instant::now());
        }
        drop(tasks);
        self.refresh_display().await;
    }

    pub async fn complete_task(&self, task_id: &str, result: TaskResult) {
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(task_id) {
            task_info.status = result.status.clone();
            task_info.end_time = Some(Instant::now());
            task_info.result = Some(result);
        }
        drop(tasks);
        self.refresh_display().await;
    }

    pub async fn get_current_output(&self, task_id: &str) -> Option<String> {
        let tasks = self.tasks.read().await;
        tasks
            .get(task_id)
            .map(|task_info| task_info.current_output.clone())
    }

    pub async fn send_live_output(&self, task_id: &str, line: &str) {
        match self.display_mode {
            DisplayMode::SingleTaskOutput => {
                // Send raw output data - let subscriber format it
                let _ = self
                    .notifier
                    .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                        jsonrpc: "2.0".to_string(),
                        method: "notifications/message".to_string(),
                        params: Some(json!({
                            "data": {
                                "type": "task_execution",
                                "subtype": "line_output",
                                "task_id": task_id,
                                "output": line
                            }
                        })),
                    }));
            }
            DisplayMode::MultipleTasksOutput => {
                let mut tasks = self.tasks.write().await;
                if let Some(task_info) = tasks.get_mut(task_id) {
                    task_info.current_output.push_str(line);
                    task_info.current_output.push('\n');
                }
                drop(tasks);

                if !self.should_throttle_refresh().await {
                    self.refresh_display().await;
                }
            }
        }
    }

    async fn should_throttle_refresh(&self) -> bool {
        let now = Instant::now();
        let mut last_refresh = self.last_refresh.write().await;

        if now.duration_since(*last_refresh) > Duration::from_millis(THROTTLE_INTERVAL_MS) {
            *last_refresh = now;
            false
        } else {
            true
        }
    }

    async fn send_tasks_update(&self) {
        let tasks = self.tasks.read().await;
        let task_list: Vec<_> = tasks.values().collect();
        let (total, pending, running, completed, failed) = count_by_status(&tasks);

        let _ = self
            .notifier
            .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                jsonrpc: "2.0".to_string(),
                method: "notifications/message".to_string(),
                params: Some(json!({
                    "data": {
                        "type": "task_execution",
                        "subtype": "tasks_update",
                        "stats": {
                            "total": total,
                            "pending": pending,
                            "running": running,
                            "completed": completed,
                            "failed": failed
                        },
                        "tasks": task_list.iter().map(|task_info| {
                            let now = Instant::now();
                            json!({
                                "id": task_info.task.id,
                                "status": task_info.status,
                                "duration_secs": task_info.start_time.map(|start| {
                                    if let Some(end) = task_info.end_time {
                                        end.duration_since(start).as_secs_f64()
                                    } else {
                                        now.duration_since(start).as_secs_f64()
                                    }
                                }),
                                "current_output": task_info.current_output,
                                "task_type": task_info.task.task_type,
                                "task_name": get_task_name(task_info),
                                "task_metadata": format_task_metadata(task_info),
                                "error": task_info.error()
                            })
                        }).collect::<Vec<_>>()
                    }
                })),
            }));
    }

    pub async fn refresh_display(&self) {
        match self.display_mode {
            DisplayMode::MultipleTasksOutput => {
                self.send_tasks_update().await;
            }
            DisplayMode::SingleTaskOutput => {
                // No dashboard display needed for single task output mode
                // Live output is handled via send_live_output method
            }
        }
    }

    pub async fn send_tasks_complete(&self) {
        let tasks = self.tasks.read().await;
        let (total, _, _, completed, failed) = count_by_status(&tasks);

        // Send structured summary data only
        let failed_tasks: Vec<_> = tasks
            .values()
            .filter(|task_info| matches!(task_info.status, TaskStatus::Failed))
            .map(|task_info| {
                json!({
                    "id": task_info.task.id,
                    "name": get_task_name(task_info),
                    "error": task_info.error()
                })
            })
            .collect();

        let _ = self
            .notifier
            .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                jsonrpc: "2.0".to_string(),
                method: "notifications/message".to_string(),
                params: Some(json!({
                    "data": {
                        "type": "task_execution",
                        "subtype": "tasks_complete",
                        "stats": {
                            "total": total,
                            "completed": completed,
                            "failed": failed,
                            "success_rate": (completed as f64 / total as f64) * 100.0
                        },
                        "failed_tasks": failed_tasks
                    }
                })),
            }));

        sleep(Duration::from_millis(500)).await;
    }
}
