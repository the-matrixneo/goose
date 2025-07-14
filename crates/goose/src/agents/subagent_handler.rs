use anyhow::Result;
use mcp_core::{Content, ToolError};
use mcp_core::protocol::JsonRpcMessage;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use crate::agents::extension_manager::ExtensionManager;
use crate::providers::base::Provider;
use crate::agents::subagent::{SubAgent, SubAgentConfig};

/// Standalone function to run a complete subagent task
pub async fn run_complete_subagent_task(
    arguments: Value,
    mcp_tx: mpsc::Sender<JsonRpcMessage>,
    provider: Arc<dyn Provider>,
    extension_manager: Option<Arc<RwLock<ExtensionManager>>>,
) -> Result<Vec<Content>, ToolError> {
    // Parse arguments - using "task" as the main message parameter
    let message = arguments
        .get("task")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::ExecutionError("Missing task parameter".to_string()))?
        .to_string();

    // Get instructions from arguments
    let instructions = arguments
        .get("instructions")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::ExecutionError("Missing instructions parameter".to_string()))?
        .to_string();

    // Set max_turns with default of 10
    let max_turns = arguments
        .get("max_turns")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let timeout = arguments.get("timeout_seconds").and_then(|v| v.as_u64());

    // Create subagent config with instructions
    let mut config = SubAgentConfig::new_with_instructions(instructions);
    config = config.with_max_turns(max_turns);
    if let Some(timeout) = timeout {
        config = config.with_timeout(timeout);
    }

    // Create the subagent with the parent agent's provider
    let extension_manager_clone = extension_manager.clone();
    let (subagent, handle) = SubAgent::new(
        config,
        Arc::clone(&provider),
        extension_manager,
        mcp_tx,
    )
    .await
    .map_err(|e| ToolError::ExecutionError(format!("Failed to create subagent: {}", e)))?;

    // Run the complete conversation
    let mut conversation_result = String::new();
    let turn_count = 0;

    // Execute the subagent task
    match subagent
        .reply_subagent(
            message,
            Arc::clone(&provider),
            extension_manager_clone,
        )
        .await
    {
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
            conversation_result
                .push_str(&format!("\n[Error after {} turns: {}]", turn_count, e));
        }
    }

    // Clean up the subagent handle
    if let Err(e) = handle.await {
        tracing::debug!("Subagent handle cleanup error: {}", e);
    }

    // Return the complete conversation result
    Ok(vec![Content::text(format!("Subagent task completed:\n{}", conversation_result))])
}
