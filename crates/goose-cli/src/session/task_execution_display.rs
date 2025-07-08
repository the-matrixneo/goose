use serde_json::Value;

const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const MOVE_TO_PROGRESS_LINE: &str = "\x1b[4;1H";
const CLEAR_TO_EOL: &str = "\x1b[K";
const CLEAR_BELOW: &str = "\x1b[J";
pub const TASK_EXECUTION_NOTIFICATION_TYPE: &str = "task_execution";

pub fn format_tasks_update(data: &Value) -> String {
    let mut display = String::new();

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
        let mut sorted_tasks: Vec<_> = tasks.iter().collect();
        sorted_tasks.sort_by_key(|task| task.get("id").and_then(|v| v.as_str()).unwrap_or(""));

        for task in sorted_tasks {
            display.push_str(&format_task_from_json(task));
        }
    }

    display.push_str(CLEAR_BELOW);
    display
}

fn format_task_from_json(task: &Value) -> String {
    let mut task_display = String::new();

    let id = task.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
    let status = task
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let task_type = task
        .get("task_type")
        .and_then(|v| v.as_str())
        .unwrap_or("task");
    let task_name = task.get("task_name").and_then(|v| v.as_str()).unwrap_or(id);
    let task_metadata = task
        .get("task_metadata")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let status_icon = match status {
        "Pending" => "⏳",
        "Running" => "🏃",
        "Completed" => "✅",
        "Failed" => "❌",
        _ => "◯",
    };

    task_display.push_str(&format!(
        "{} {} ({}){}\n",
        status_icon, task_name, task_type, CLEAR_TO_EOL
    ));

    if !task_metadata.is_empty() {
        task_display.push_str(&format!(
            "   📋 Parameters: {}{}\n",
            task_metadata, CLEAR_TO_EOL
        ));
    }

    if let Some(duration_secs) = task.get("duration_secs").and_then(|v| v.as_f64()) {
        task_display.push_str(&format!("   ⏱️  {:.1}s{}\n", duration_secs, CLEAR_TO_EOL));
    }

    if status == "Running" {
        if let Some(current_output) = task.get("current_output").and_then(|v| v.as_str()) {
            if !current_output.trim().is_empty() {
                let processed_output = process_output_for_display(current_output);
                if !processed_output.is_empty() {
                    task_display.push_str(&format!("   💬 {}{}\n", processed_output, CLEAR_TO_EOL));
                }
            }
        }
    }

    if status == "Failed" {
        if let Some(error) = task.get("error").and_then(|v| v.as_str()) {
            let error_preview = truncate_with_ellipsis(error, 80);
            task_display.push_str(&format!(
                "   ⚠️  {}{}\n",
                error_preview.replace('\n', " "),
                CLEAR_TO_EOL
            ));
        }
    }

    task_display.push_str(&format!("{}\n", CLEAR_TO_EOL));
    task_display
}

fn process_output_for_display(output: &str) -> String {
    const MAX_OUTPUT_LINES: usize = 2;
    const OUTPUT_PREVIEW_LENGTH: usize = 100;

    let lines: Vec<&str> = output.lines().collect();
    let recent_lines = if lines.len() > MAX_OUTPUT_LINES {
        &lines[lines.len() - MAX_OUTPUT_LINES..]
    } else {
        &lines
    };

    let clean_output = recent_lines.join(" | ");
    let stripped = strip_ansi_codes(&clean_output);
    truncate_with_ellipsis(&stripped, OUTPUT_PREVIEW_LENGTH)
}

fn truncate_with_ellipsis(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    } else {
        text.to_string()
    }
}

fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.next() == Some('[') {
                loop {
                    match chars.next() {
                        Some(c) if c.is_ascii_alphabetic() => break,
                        Some(_) => continue,
                        None => break,
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
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
