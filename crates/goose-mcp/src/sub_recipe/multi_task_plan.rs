use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use mcp_core::handler::ToolError;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub task: String,
    pub task_number: u32,
    pub total_tasks: u32,
    pub task_id: String,
    pub next_task_needed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_task: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_task: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_more_tasks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
}

pub struct TasksState {
    pub task_history: Vec<TaskData>,
    pub branches: HashMap<String, Vec<TaskData>>,
}

pub fn validate_task_data(params: Value) -> Result<TaskData, ToolError> {
    let task_data: TaskData = serde_json::from_value(params)
        .map_err(|e| ToolError::InvalidParameters(format!("Invalid task data: {}", e)))?;

    if task_data.task.is_empty() {
        return Err(ToolError::InvalidParameters(
            "Invalid task: must be a non-empty string".into(),
        ));
    }

    if task_data.task_number == 0 {
        return Err(ToolError::InvalidParameters(
            "Invalid taskNumber: must be a positive number".into(),
        ));
    }

    if task_data.total_tasks == 0 {
        return Err(ToolError::InvalidParameters(
            "Invalid totalTasks: must be a positive number".into(),
        ));
    }

    Ok(task_data)
}

pub fn format_task(task_data: &TaskData) -> String {
    let (prefix, context) = if task_data.is_revision.unwrap_or(false) {
        (
            "🔄 Revision",
            format!(" (revising task {})", task_data.revises_task.unwrap_or(0)),
        )
    } else if task_data.branch_from_task.is_some() {
        (
            "🌿 Branch",
            format!(
                " (from task {}, ID: {})",
                task_data.branch_from_task.unwrap_or(0),
                task_data.branch_id.as_deref().unwrap_or("unknown")
            ),
        )
    } else {
        ("💭 Task", String::new())
    };

    let header = format!(
        "{} {}/{}{}",
        prefix, task_data.task_number, task_data.total_tasks, context
    );

    let task_len = task_data.task.len();
    let border_len = std::cmp::max(100, task_len) + 4;
    let border = "─".repeat(border_len);

    let task_content = format!("{:<width$}", task_data.task, width = border_len - 2);

    format!(
        "\n┌{}┐\n│ {} │\n├{}┤\n│ {} │\n└{}┘",
        border, header, border, task_content, border
    )
}
