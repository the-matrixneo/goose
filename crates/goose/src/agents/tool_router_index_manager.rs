use anyhow::{anyhow, Result};
use std::sync::Arc;
use tracing;

use crate::agents::extension_manager::ExtensionManager;
use crate::agents::platform_tools;
use crate::agents::recipe_tools::dynamic_task_tools::create_dynamic_task_tool;
use crate::agents::router_tool_selector::RouterToolSelector;
use crate::agents::subagent_execution_tool::subagent_execute_task_tool;

/// Manages tool indexing operations for the router when LLM routing is enabled
pub struct ToolRouterIndexManager;

impl ToolRouterIndexManager {
    /// Updates the LLM index for tools when extensions are added or removed
    pub async fn update_extension_tools(
        selector: &Arc<Box<dyn RouterToolSelector>>,
        extension_manager: &ExtensionManager,
        extension_name: &str,
        action: &str,
    ) -> Result<()> {
        match action {
            "add" => {
                // Get tools for specific extension
                let tools = extension_manager
                    .get_prefixed_tools(Some(extension_name.to_string()))
                    .await?;

                if !tools.is_empty() {
                    // Index all tools at once
                    selector
                        .index_tools(&tools, extension_name)
                        .await
                        .map_err(|e| {
                            anyhow!(
                                "Failed to index tools for extension {}: {}",
                                extension_name,
                                e
                            )
                        })?;

                    tracing::info!(
                        "Indexed {} tools for extension {}",
                        tools.len(),
                        extension_name
                    );
                }
            }
            "remove" => {
                // Remove all tools for this extension
                let tools = extension_manager
                    .get_prefixed_tools(Some(extension_name.to_string()))
                    .await?;

                for tool in &tools {
                    selector.remove_tool(&tool.name).await.map_err(|e| {
                        anyhow!(
                            "Failed to remove tool {} for extension {}: {}",
                            tool.name,
                            extension_name,
                            e
                        )
                    })?;
                }

                tracing::info!(
                    "Removed {} tools for extension {}",
                    tools.len(),
                    extension_name
                );
            }
            _ => {
                return Err(anyhow!("Invalid action: {}", action));
            }
        }

        Ok(())
    }

    /// Indexes platform tools (search_available_extensions, manage_extensions, subagent, etc.)
    pub async fn index_platform_tools(
        selector: &Arc<Box<dyn RouterToolSelector>>,
        extension_manager: &ExtensionManager,
    ) -> Result<()> {
        let mut tools = Vec::new();

        // Add the standard platform tools
        tools.push(platform_tools::search_available_extensions_tool());
        tools.push(platform_tools::manage_extensions_tool());
        tools.push(platform_tools::manage_schedule_tool());

        // Add subagent execution tool - critical for subagent functionality
        tools.push(subagent_execute_task_tool::create_subagent_execute_task_tool());

        // Add dynamic task tool
        tools.push(create_dynamic_task_tool());

        // Add resource tools if supported
        if extension_manager.supports_resources().await {
            tools.push(platform_tools::read_resource_tool());
            tools.push(platform_tools::list_resources_tool());
        }

        // Index all platform tools at once
        selector
            .index_tools(&tools, "platform")
            .await
            .map_err(|e| anyhow!("Failed to index platform tools: {}", e))?;

        tracing::info!("Indexed platform tools (including subagent) for LLM search");
        Ok(())
    }
}
