use anyhow::{anyhow, Result};
use std::sync::Arc;
use tracing;

use crate::agents::extension_manager::ExtensionManager;
use crate::agents::platform_tools;
use crate::agents::router_tool_selector::{RouterToolSelectionStrategy, RouterToolSelector};
use crate::config::extensions::ExtensionConfigManager;
use crate::agents::extension::ExtensionConfig;

/// Manages tool indexing operations for the router when vector routing is enabled
pub struct ToolRouterIndexManager;

impl ToolRouterIndexManager {
    /// Updates the vector index for tools when extensions are added or removed
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
                    // Get extension description from config
                    let extension_config = ExtensionConfigManager::get_config_by_name(extension_name)
                        .map_err(|e| anyhow!("Failed to get extension config: {}", e))?
                        .ok_or_else(|| anyhow!("Extension {} not found in config", extension_name))?;

                    let extension_description = match &extension_config {
                        ExtensionConfig::Builtin { display_name, .. } => {
                            display_name.as_deref().unwrap_or(extension_name)
                        }
                        ExtensionConfig::Sse { description, .. } | ExtensionConfig::Stdio { description, .. } => {
                            description.as_deref().unwrap_or(extension_name)
                        }
                        ExtensionConfig::Frontend { .. } => extension_name,
                    };

                    // Index all tools at once with extension description
                    selector.index_tools(&tools, Some(extension_description)).await.map_err(|e| {
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
                // Get tool names for the extension to remove them
                let tools = extension_manager
                    .get_prefixed_tools(Some(extension_name.to_string()))
                    .await?;

                for tool in &tools {
                    selector
                        .remove_tool(&tool.name)
                        .await
                        .map_err(|e| anyhow!("Failed to remove tool {}: {}", tool.name, e))?;
                }

                tracing::info!(
                    "Removed {} tools for extension {}",
                    tools.len(),
                    extension_name
                );
            }
            _ => {
                anyhow::bail!("Invalid action '{}' for tool indexing", action);
            }
        }

        Ok(())
    }

    /// Indexes platform tools (search_available_extensions, manage_extensions, etc.)
    pub async fn index_platform_tools(
        selector: &Arc<Box<dyn RouterToolSelector>>,
        extension_manager: &ExtensionManager,
    ) -> Result<()> {
        let mut tools = Vec::new();

        // Add the standard platform tools
        tools.push(platform_tools::search_available_extensions_tool());
        tools.push(platform_tools::manage_extensions_tool());

        // Add resource tools if supported
        if extension_manager.supports_resources() {
            tools.push(platform_tools::read_resource_tool());
            tools.push(platform_tools::list_resources_tool());
        }

        // Index all platform tools at once
        selector
            .index_tools(&tools, None)
            .await
            .map_err(|e| anyhow!("Failed to index platform tools: {}", e))?;

        tracing::info!("Indexed platform tools for vector search");
        Ok(())
    }

    /// Helper to check if vector tool router is enabled
    pub fn vector_tool_router_enabled(selector: &Option<Arc<Box<dyn RouterToolSelector>>>) -> bool {
        if let Some(selector) = selector {
            selector.selector_type() == RouterToolSelectionStrategy::Vector
        } else {
            false
        }
    }
}
