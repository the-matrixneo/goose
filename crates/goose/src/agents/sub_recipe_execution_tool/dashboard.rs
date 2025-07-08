use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

use crate::agents::sub_recipe_execution_tool::types::{Task, TaskInfo, TaskResult, TaskStatus};
use crate::agents::sub_recipe_execution_tool::utils::{
    count_by_status, get_task_name, strip_ansi_codes, truncate_with_ellipsis,
};

pub struct TaskDashboard {
    tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    last_display: Arc<RwLock<String>>,
    last_refresh: Arc<RwLock<Instant>>,
    initial_display_shown: Arc<RwLock<bool>>,
}

impl TaskDashboard {
    pub fn new(tasks: Vec<Task>) -> Self {
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

    pub async fn update_task_output(&self, task_id: &str, output: &str) {
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(task_id) {
            // Keep only the last few lines to avoid overwhelming display
            let lines: Vec<&str> = output.lines().collect();
            let recent_lines = if lines.len() > 2 {
                &lines[lines.len() - 2..]
            } else {
                &lines
            };

            // Strip ANSI escape sequences to prevent color flashing
            let clean_output = recent_lines.join("\n");
            task_info.current_output = strip_ansi_codes(&clean_output);
        }
        drop(tasks);

        // Throttle refreshes to avoid overwhelming the display (max 1 per second)
        let now = Instant::now();
        let mut last_refresh = self.last_refresh.write().await;
        if now.duration_since(*last_refresh) > Duration::from_millis(1000) {
            *last_refresh = now;
            drop(last_refresh);
            self.refresh_display().await;
        }
    }

    pub async fn refresh_display(&self) {
        let tasks = self.tasks.read().await;
        let mut display = String::new();

        let mut initial_shown = self.initial_display_shown.write().await;
        if !*initial_shown {
            // Clear screen and show header only on first display
            display.push_str("\x1b[2J\x1b[H");
            display.push_str("🎯 Task Execution Dashboard\n");
            display.push_str("═══════════════════════════\n\n");
            *initial_shown = true;
        } else {
            // Move cursor to beginning of progress line (line 4)
            display.push_str("\x1b[4;1H");
        }
        drop(initial_shown);

        // Summary stats (this line gets updated in-place)
        let (total, pending, running, completed, failed) = count_by_status(&tasks);
        display.push_str(&format!("📊 Progress: {} total | ⏳ {} pending | 🏃 {} running | ✅ {} completed | ❌ {} failed", 
            total, pending, running, completed, failed));

        // Clear to end of line and add newlines
        display.push_str("\x1b[K\n\n");

        // Task list (update in-place)
        let mut task_list: Vec<_> = tasks.values().collect();
        task_list.sort_by_key(|t| &t.task.id);

        for task_info in task_list {
            let status_icon = match task_info.status {
                TaskStatus::Pending => "⏳",
                TaskStatus::Running => "🏃",
                TaskStatus::Completed => "✅",
                TaskStatus::Failed => "❌",
            };

            let task_name = get_task_name(task_info);

            display.push_str(&format!(
                "{} {} ({})",
                status_icon, task_name, task_info.task.task_type
            ));
            display.push_str("\x1b[K\n"); // Clear to end of line

            if let Some(start_time) = task_info.start_time {
                let duration = if let Some(end_time) = task_info.end_time {
                    end_time.duration_since(start_time)
                } else {
                    Instant::now().duration_since(start_time)
                };
                display.push_str(&format!("   ⏱️  {:.1}s", duration.as_secs_f64()));
                display.push_str("\x1b[K\n"); // Clear to end of line
            }

            if matches!(task_info.status, TaskStatus::Running)
                && !task_info.current_output.is_empty()
            {
                let output_preview = truncate_with_ellipsis(&task_info.current_output, 100);
                display.push_str(&format!("   💬 {}", output_preview.replace('\n', " | ")));
                display.push_str("\x1b[K\n"); // Clear to end of line
            }

            if let Some(error) = task_info.error() {
                let error_preview = truncate_with_ellipsis(error, 80);
                display.push_str(&format!("   ⚠️  {}", error_preview.replace('\n', " ")));
                display.push_str("\x1b[K\n"); // Clear to end of line
            }

            display.push_str("\x1b[K\n"); // Clear to end of line and add blank line
        }

        // Clear any remaining lines below
        display.push_str("\x1b[J");

        // Only update display if it changed
        let mut last_display = self.last_display.write().await;
        if *last_display != display {
            print!("{}", display);
            io::stdout().flush().unwrap();
            *last_display = display;
        }
    }

    pub async fn show_final_summary(&self) {
        let tasks = self.tasks.read().await;

        println!("\n🎉 Execution Complete!");
        println!("═══════════════════════");

        let (total, _, _, completed, failed) = count_by_status(&tasks);

        println!("📊 Final Results:");
        println!("   Total Tasks: {}", total);
        println!("   ✅ Completed: {}", completed);
        println!("   ❌ Failed: {}", failed);
        println!(
            "   📈 Success Rate: {:.1}%",
            (completed as f64 / total as f64) * 100.0
        );

        if failed > 0 {
            println!("\n❌ Failed Tasks:");
            for task_info in tasks.values() {
                if matches!(task_info.status, TaskStatus::Failed) {
                    let task_name = get_task_name(task_info);
                    println!("   • {}", task_name);
                    if let Some(error) = task_info.error() {
                        println!("     Error: {}", error);
                    }
                }
            }
        }
    }
}
