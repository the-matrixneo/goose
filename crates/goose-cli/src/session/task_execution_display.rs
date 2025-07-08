use serde_json::Value;

const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const MOVE_TO_PROGRESS_LINE: &str = "\x1b[4;1H";
const CLEAR_TO_EOL: &str = "\x1b[K";
const CLEAR_BELOW: &str = "\x1b[J";
pub const TASK_EXECUTION_NOTIFICATION_TYPE: &str = "task_execution";

pub fn format_tasks_update(data: &Value) -> String {
    let mut display = String::new();

    // Determine if this is initial display or update
    static mut INITIAL_SHOWN: bool = false;
    unsafe {
        if !INITIAL_SHOWN {
            display.push_str(CLEAR_SCREEN);
            display.push_str("🎯 Task Execution Dashboard\n");
            display.push_str("═══════════════════════════\n\n");
            INITIAL_SHOWN = true;
        } else {
            display.push_str(MOVE_TO_PROGRESS_LINE);
        }
    }

    if let Some(stats) = data.get("stats") {
        let total = stats.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
        let pending = stats.get("pending").and_then(|v| v.as_u64()).unwrap_or(0);
        let running = stats.get("running").and_then(|v| v.as_u64()).unwrap_or(0);
        let completed = stats.get("completed").and_then(|v| v.as_u64()).unwrap_or(0);
        let failed = stats.get("failed").and_then(|v| v.as_u64()).unwrap_or(0);

        display.push_str(&format!(
            "📊 Progress: {} total | ⏳ {} pending | 🏃 {} running | ✅ {} completed | ❌ {} failed", 
            total, pending, running, completed, failed
        ));
        display.push_str(&format!("{}\n\n", CLEAR_TO_EOL));
    }

    if let Some(tasks) = data.get("tasks").and_then(|t| t.as_array()) {
        for task in tasks {
            let id = task.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let status = task
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let task_type = task
                .get("task_type")
                .and_then(|v| v.as_str())
                .unwrap_or("task");

            let status_icon = match status {
                "Pending" => "⏳",
                "Running" => "🏃",
                "Completed" => "✅",
                "Failed" => "❌",
                _ => "◯",
            };

            display.push_str(&format!(
                "{} {} ({}): {}\n",
                status_icon, id, task_type, status
            ));

            if status == "Running" {
                if let Some(output) = task.get("current_output").and_then(|v| v.as_str()) {
                    if !output.trim().is_empty() {
                        let lines: Vec<&str> = output.lines().collect();
                        if lines.len() > 3 {
                            display.push_str("   ...\n");
                            for line in lines.iter().rev().take(3).rev() {
                                display.push_str(&format!("   {}\n", line));
                            }
                        } else {
                            for line in lines {
                                display.push_str(&format!("   {}\n", line));
                            }
                        }
                    }
                }
            }

            if status == "Failed" {
                if let Some(error) = task.get("error").and_then(|v| v.as_str()) {
                    display.push_str(&format!("   Error: {}\n", error));
                }
            }
        }
    }

    display.push_str(CLEAR_BELOW);
    display
}

pub fn format_tasks_complete(data: &Value) -> String {
    let mut summary = String::new();
    summary.push_str("Execution Complete!\n");
    summary.push_str("═══════════════════════\n");

    if let Some(stats) = data.get("stats") {
        let total = stats.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
        let completed = stats.get("completed").and_then(|v| v.as_u64()).unwrap_or(0);
        let failed = stats.get("failed").and_then(|v| v.as_u64()).unwrap_or(0);
        let success_rate = stats
            .get("success_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        summary.push_str(&format!("Total Tasks: {}\n", total));
        summary.push_str(&format!("✅ Completed: {}\n", completed));
        summary.push_str(&format!("❌ Failed: {}\n", failed));
        summary.push_str(&format!("📈 Success Rate: {:.1}%\n", success_rate));
    }

    if let Some(failed_tasks) = data.get("failed_tasks").and_then(|t| t.as_array()) {
        if !failed_tasks.is_empty() {
            summary.push_str("\n❌ Failed Tasks:\n");
            for task in failed_tasks {
                let name = task
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                summary.push_str(&format!("   • {}\n", name));
                if let Some(error) = task.get("error").and_then(|v| v.as_str()) {
                    summary.push_str(&format!("     Error: {}\n", error));
                }
            }
        }
    }

    summary.push_str("\n📝 Generating summary...\n");
    summary
}

pub fn format_task_execution_notification(
    data: &Value,
) -> Option<(String, Option<String>, Option<String>)> {
    if let Value::Object(o) = data {
        if o.get("type").and_then(|t| t.as_str()) == Some(TASK_EXECUTION_NOTIFICATION_TYPE) {
            return Some(match o.get("subtype").and_then(|t| t.as_str()) {
                Some("line_output") => {
                    if let Some(Value::String(line_output)) = o.get("output") {
                        (
                            format!("{}\n", line_output),
                            None,
                            Some(TASK_EXECUTION_NOTIFICATION_TYPE.to_string()),
                        )
                    } else {
                        (data.to_string(), None, None)
                    }
                }
                Some("tasks_update") => {
                    let data_value = Value::Object(o.clone());
                    let formatted_display = format_tasks_update(&data_value);
                    (
                        formatted_display,
                        None,
                        Some(TASK_EXECUTION_NOTIFICATION_TYPE.to_string()),
                    )
                }
                Some("tasks_complete") => {
                    let data_value = Value::Object(o.clone());
                    let formatted_summary = format_tasks_complete(&data_value);
                    (
                        formatted_summary,
                        None,
                        Some(TASK_EXECUTION_NOTIFICATION_TYPE.to_string()),
                    )
                }
                _ => (data.to_string(), None, None),
            });
        }
    }
    None
}
