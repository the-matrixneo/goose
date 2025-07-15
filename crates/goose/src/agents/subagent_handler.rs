use anyhow::Result;
use mcp_core::{Content, ToolError};
use serde_json::Value;

use crate::agents::subagent::SubAgent;
use crate::agents::task::TaskConfig;

/// Standalone function to run a complete subagent task
pub async fn run_complete_subagent_task(
    task_arguments: Value,
    task_config: TaskConfig,
) -> Result<Vec<Content>, ToolError> {
    // Parse arguments - using "task" as the main message parameter
    let text_instruction = task_arguments
        .get("text_instruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::ExecutionError("Missing text_instruction parameter".to_string()))?
        .to_string();

    // Create the subagent with the parent agent's provider
    let (subagent, handle) = SubAgent::new(task_config.clone())
        .await
        .map_err(|e| ToolError::ExecutionError(format!("Failed to create subagent: {}", e)))?;

    // Run the complete conversation
    let mut conversation_result = String::new();
    let turn_count = 0;

    println!("Subagent created, executing task...");
    // Execute the subagent task
    match subagent.reply_subagent(text_instruction, task_config).await {
        Ok(response) => {
            let response_text = response.as_concat_text();
            conversation_result.push_str(&format!(
                "\n--- Turn {} ---\n{}",
                turn_count + 1,
                response_text
            ));
            conversation_result.push_str(&format!(
                "\n[Task completed after {} turns]",
                turn_count + 1
            ));
        }
        Err(e) => {
            conversation_result.push_str(&format!("\n[Error after {} turns: {}]", turn_count, e));
        }
    }

    // Clean up the subagent handle
    if let Err(e) = handle.await {
        tracing::debug!("Subagent handle cleanup error: {}", e);
    }

    // Return the complete conversation result
    Ok(vec![Content::text(format!(
        "Subagent task completed:\n{}",
        conversation_result
    ))])
}
