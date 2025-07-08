use serde_json::{Map, Value};
use std::collections::HashMap;
use tokio::time::Instant;

use crate::agents::sub_recipe_execution_tool::types::{TaskInfo, TaskStatus};

// Constants for display formatting
const MAX_OUTPUT_LINES: usize = 2;
const OUTPUT_PREVIEW_LENGTH: usize = 100;
const ERROR_PREVIEW_LENGTH: usize = 80;
const CLEAR_TO_EOL: &str = "\x1b[K";

pub fn get_task_name(task_info: &TaskInfo) -> &str {
    task_info
        .task
        .get_sub_recipe_name()
        .unwrap_or(&task_info.task.id)
}

pub fn get_command_parameters(task_info: &TaskInfo) -> Option<&Map<String, Value>> {
    task_info.task.get_command_parameters()
}

pub fn truncate_with_ellipsis(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    } else {
        text.to_string()
    }
}

pub fn count_by_status(tasks: &HashMap<String, TaskInfo>) -> (usize, usize, usize, usize, usize) {
    let total = tasks.len();
    let (pending, running, completed, failed) = tasks.values().fold(
        (0, 0, 0, 0),
        |(pending, running, completed, failed), task| match task.status {
            TaskStatus::Pending => (pending + 1, running, completed, failed),
            TaskStatus::Running => (pending, running + 1, completed, failed),
            TaskStatus::Completed => (pending, running, completed + 1, failed),
            TaskStatus::Failed => (pending, running, completed, failed + 1),
        },
    );
    (total, pending, running, completed, failed)
}

pub fn strip_ansi_codes(text: &str) -> String {
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


pub fn get_status_icon(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Pending => "⏳",
        TaskStatus::Running => "🏃",
        TaskStatus::Completed => "✅",
        TaskStatus::Failed => "❌",
    }
}

pub fn process_output_lines(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let recent_lines = if lines.len() > MAX_OUTPUT_LINES {
        &lines[lines.len() - MAX_OUTPUT_LINES..]
    } else {
        &lines
    };

    let clean_output = recent_lines.join("\n");
    strip_ansi_codes(&clean_output)
}

/// Format task timing information
pub fn format_task_timing(task_info: &TaskInfo, current_time: Instant) -> Option<String> {
    task_info.start_time.map(|start_time| {
        let duration = if let Some(end_time) = task_info.end_time {
            end_time.duration_since(start_time)
        } else {
            current_time.duration_since(start_time)
        };
        format!(
            "   ⏱️  {:.1}s{}
",
            duration.as_secs_f64(),
            CLEAR_TO_EOL
        )
    })
}

/// Format task output preview
pub fn format_task_output(task_info: &TaskInfo) -> Option<String> {
    if matches!(task_info.status, TaskStatus::Running) && !task_info.current_output.is_empty() {
        let output_preview =
            truncate_with_ellipsis(&task_info.current_output, OUTPUT_PREVIEW_LENGTH);
        Some(format!(
            "   💬 {}{}
",
            output_preview.replace('\n', " | "),
            CLEAR_TO_EOL
        ))
    } else {
        None
    }
}

/// Format task error information
pub fn format_task_error(task_info: &TaskInfo) -> Option<String> {
    task_info.error().map(|error| {
        let error_preview = truncate_with_ellipsis(error, ERROR_PREVIEW_LENGTH);
        format!(
            "   ⚠️  {}{}
",
            error_preview.replace('\n', " "),
            CLEAR_TO_EOL
        )
    })
}

pub fn format_command_parameters(task_info: &TaskInfo) -> Option<String> {
    get_command_parameters(task_info).map(|params| {
        if params.is_empty() {
            return format!("   📋 Parameters: (none){}\n", CLEAR_TO_EOL);
        }

        let params_str = params
            .iter()
            .map(|(key, value)| {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                format!("{}={}", key, value_str)
            })
            .collect::<Vec<_>>()
            .join(", ");

        let params_preview = truncate_with_ellipsis(&params_str, OUTPUT_PREVIEW_LENGTH);
        format!("   📋 Parameters: {}{}\n", params_preview, CLEAR_TO_EOL)
    })
}

pub fn format_task_display(task_info: &TaskInfo, current_time: Instant) -> String {
    let mut display = String::new();

    let status_icon = get_status_icon(&task_info.status);
    let task_name = get_task_name(task_info);

    // Task status line
    display.push_str(&format!(
        "{} {} ({}){}
",
        status_icon, task_name, task_info.task.task_type, CLEAR_TO_EOL
    ));

    if let Some(params) = format_command_parameters(task_info) {
        display.push_str(&params);
    }

    if let Some(timing) = format_task_timing(task_info, current_time) {
        display.push_str(&timing);
    }

    if let Some(output) = format_task_output(task_info) {
        display.push_str(&output);
    }

    // Task error (if failed)
    if let Some(error) = format_task_error(task_info) {
        display.push_str(&error);
    }

    display.push_str(&format!(
        "{}
",
        CLEAR_TO_EOL
    ));

    display
}

#[cfg(test)]
mod tests;
