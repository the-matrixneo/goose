use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use anyhow::Result;

use serde_json::Value;
use mcp_core::
    handler::ToolError
;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtData {
    pub thought: String,
    pub thought_number: u32,
    pub total_thoughts: u32,
    pub next_thought_needed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_more_thoughts: Option<bool>,
}

pub struct SequentialThinkingState {
    pub thought_history: Vec<ThoughtData>,
    pub branches: HashMap<String, Vec<ThoughtData>>,
}

pub fn validate_thought_data(params: Value) -> Result<ThoughtData, ToolError> {
    // Parse the JSON data into ThoughtData
    let thought_data: ThoughtData = serde_json::from_value(params)
        .map_err(|e| ToolError::InvalidParameters(format!("Invalid thought data: {}", e)))?;

    // Validate required fields
    if thought_data.thought.is_empty() {
        return Err(ToolError::InvalidParameters(
            "Invalid thought: must be a non-empty string".into(),
        ));
    }

    if thought_data.thought_number == 0 {
        return Err(ToolError::InvalidParameters(
            "Invalid thoughtNumber: must be a positive number".into(),
        ));
    }

    if thought_data.total_thoughts == 0 {
        return Err(ToolError::InvalidParameters(
            "Invalid totalThoughts: must be a positive number".into(),
        ));
    }

    Ok(thought_data)
}

// Format thought for display
pub fn format_thought(thought_data: &ThoughtData) -> String {
    let (prefix, context) = if thought_data.is_revision.unwrap_or(false) {
        (
            "🔄 Revision",
            format!(
                " (revising thought {})",
                thought_data.revises_thought.unwrap_or(0)
            ),
        )
    } else if thought_data.branch_from_thought.is_some() {
        (
            "🌿 Branch",
            format!(
                " (from thought {}, ID: {})",
                thought_data.branch_from_thought.unwrap_or(0),
                thought_data.branch_id.as_deref().unwrap_or("unknown")
            ),
        )
    } else {
        ("💭 Thought", String::new())
    };

    let header = format!(
        "{} {}/{}{}",
        prefix, thought_data.thought_number, thought_data.total_thoughts, context
    );
    
    let thought_len = thought_data.thought.len();
    let border_len = std::cmp::max(100, thought_len) + 4;
    let border = "─".repeat(border_len);

    format!(
        "\n┌{}┐\n│ {} │\n├{}┤\n│ {} │\n└{}┘",
        border,
        header,
        border,
        format!("{:<width$}", thought_data.thought, width = border_len - 2),
        border
    )
    
}

