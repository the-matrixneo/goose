use mcp_core::protocol::{JsonRpcMessage, JsonRpcNotification};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration, Instant};

use crate::agents::sub_recipe_execution_tool::types::{Task, TaskInfo, TaskResult, TaskStatus};
use crate::agents::sub_recipe_execution_tool::utils::{
    count_by_status, format_task_display, get_task_name,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMode {
    Dashboard,
    SingleTaskOutput,
}

const THROTTLE_INTERVAL_MS: u64 = 1000;
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const MOVE_TO_PROGRESS_LINE: &str = "\x1b[4;1H";
const CLEAR_TO_EOL: &str = "\x1b[K";
const CLEAR_BELOW: &str = "\x1b[J";

pub struct TaskDashboard {
    tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    last_display: Arc<RwLock<String>>,
    last_refresh: Arc<RwLock<Instant>>,
    initial_display_shown: Arc<RwLock<bool>>,
    notifier: mpsc::Sender<JsonRpcMessage>,
    display_mode: DisplayMode,
}

impl TaskDashboard {
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
            last_display: Arc::new(RwLock::new(String::new())),
            last_refresh: Arc::new(RwLock::new(Instant::now())),
            initial_display_shown: Arc::new(RwLock::new(false)),
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

    pub async fn send_live_output(&self, task_id: &str, line: &str) {
        match self.display_mode {
            DisplayMode::SingleTaskOutput => {
                let _ = self
                    .notifier
                    .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                        jsonrpc: "2.0".to_string(),
                        method: "notifications/message".to_string(),
                        params: Some(json!({
                            "data": {
                                "type": "dashboard",
                                "display": format!("{}\n", line)
                            }
                        })),
                    }));
            }
            DisplayMode::Dashboard => {
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

    fn render_header(&self, display: &mut String, initial_shown: &mut bool) {
        if !*initial_shown {
            display.push_str(CLEAR_SCREEN);
            display.push_str("🎯 Task Execution Dashboard\n");
            display.push_str("═══════════════════════════\n\n");
            *initial_shown = true;
        } else {
            display.push_str(MOVE_TO_PROGRESS_LINE);
        }
    }

    fn render_progress_line(&self, display: &mut String, tasks: &HashMap<String, TaskInfo>) {
        let (total, pending, running, completed, failed) = count_by_status(tasks);
        display.push_str(&format!(
            "📊 Progress: {} total | ⏳ {} pending | 🏃 {} running | ✅ {} completed | ❌ {} failed", 
            total, pending, running, completed, failed
        ));
        display.push_str(&format!("{}\n\n", CLEAR_TO_EOL));
    }

    fn render_task(&self, display: &mut String, task_info: &TaskInfo) {
        let task_display = format_task_display(task_info, Instant::now());
        display.push_str(&task_display);
    }

    async fn update_display_if_changed(&self, display: String) {
        let mut last_display = self.last_display.write().await;
        if *last_display != display {
            let _ = self
                .notifier
                .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                    jsonrpc: "2.0".to_string(),
                    method: "notifications/message".to_string(),
                    params: Some(json!({
                        "data": {
                            "type": "dashboard",
                            "display": display.clone()
                        }
                    })),
                }));
            *last_display = display;
        }
    }

    pub async fn refresh_display(&self) {
        match self.display_mode {
            DisplayMode::Dashboard => {
                let tasks = self.tasks.read().await;
                let mut display = String::new();

                let mut initial_shown = self.initial_display_shown.write().await;
                self.render_header(&mut display, &mut initial_shown);
                drop(initial_shown);

                self.render_progress_line(&mut display, &tasks);

                let mut task_list: Vec<_> = tasks.values().collect();
                task_list.sort_by_key(|t| &t.task.id);

                for task_info in task_list {
                    self.render_task(&mut display, task_info);
                }

                display.push_str(CLEAR_BELOW);

                self.update_display_if_changed(display).await;
            }
            DisplayMode::SingleTaskOutput => {
                // No dashboard display needed for single task output mode
                // Live output is handled via send_live_output method
            }
        }
    }

    pub async fn show_final_summary(&self) {
        let tasks = self.tasks.read().await;

        let mut summary = String::new();
        summary.push_str("Execution Complete!\n");
        summary.push_str("═══════════════════════\n");

        let (total, _, _, completed, failed) = count_by_status(&tasks);

        summary.push_str(&format!("Total Tasks: {}\n", total));
        summary.push_str(&format!("✅ Completed: {}\n", completed));
        summary.push_str(&format!("❌ Failed: {}\n", failed));
        summary.push_str(&format!(
            "📈 Success Rate: {:.1}%\n",
            (completed as f64 / total as f64) * 100.0
        ));

        if failed > 0 {
            summary.push_str("\n❌ Failed Tasks:\n");
            for task_info in tasks.values() {
                if matches!(task_info.status, TaskStatus::Failed) {
                    let task_name = get_task_name(task_info);
                    summary.push_str(&format!("   • {}\n", task_name));
                    if let Some(error) = task_info.error() {
                        summary.push_str(&format!("     Error: {}\n", error));
                    }
                }
            }
        }

        summary.push_str("\n📝 Generating summary...\n");

        // Send the final summary via notification
        let _ = self
            .notifier
            .try_send(JsonRpcMessage::Notification(JsonRpcNotification {
                jsonrpc: "2.0".to_string(),
                method: "notifications/message".to_string(),
                params: Some(json!({
                    "data": {
                        "type": "dashboard",
                        "display": summary
                    }
                })),
            }));

        sleep(Duration::from_millis(500)).await;
    }
}
