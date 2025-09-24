use indoc::indoc;
use rmcp::model::{Tool, ToolAnnotations};
use rmcp::object;

use std::sync::Arc;
use async_trait::async_trait;
use mcp_client::client::{McpClientTrait, Error};
use rmcp::model::{
    CallToolResult, Content, GetPromptResult, InitializeResult, ListPromptsResult,
    ListResourcesResult, ListToolsResult, ReadResourceResult, ServerNotification,
};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use super::extension_manager::ExtensionManager;
use super::tool_route_manager::ToolRouteManager;
use super::tool_router_index_manager::ToolRouterIndexManager;
use crate::config::ExtensionConfigManager;

pub const PLATFORM_READ_RESOURCE_TOOL_NAME: &str = "read_resource";
pub const PLATFORM_LIST_RESOURCES_TOOL_NAME: &str = "list_resources";
pub const PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME: &str = "search_available_extensions";
pub const PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME: &str = "manage_extensions";
pub const PLATFORM_MANAGE_SCHEDULE_TOOL_NAME: &str = "platform__manage_schedule";

pub fn read_resource_tool() -> Tool {
    Tool::new(
        PLATFORM_READ_RESOURCE_TOOL_NAME.to_string(),
        indoc! {r#"
            Read a resource from an extension.

            Resources allow extensions to share data that provide context to LLMs, such as
            files, database schemas, or application-specific information. This tool searches for the
            resource URI in the provided extension, and reads in the resource content. If no extension
            is provided, the tool will search all extensions for the resource.
        "#}.to_string(),
        object!({
            "type": "object",
            "required": ["uri"],
            "properties": {
                "uri": {"type": "string", "description": "Resource URI"},
                "extension_name": {"type": "string", "description": "Optional extension name"}
            }
        })
    ).annotate(ToolAnnotations {
        title: Some("Read a resource".to_string()),
        read_only_hint: Some(true),
        destructive_hint: Some(false),
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}

pub fn list_resources_tool() -> Tool {
    Tool::new(
        PLATFORM_LIST_RESOURCES_TOOL_NAME.to_string(),
        indoc! {r#"
            List resources from an extension(s).

            Resources allow extensions to share data that provide context to LLMs, such as
            files, database schemas, or application-specific information. This tool lists resources
            in the provided extension, and returns a list for the user to browse. If no extension
            is provided, the tool will search all extensions for the resource.
        "#}
        .to_string(),
        object!({
            "type": "object",
            "properties": {
                "extension_name": {"type": "string", "description": "Optional extension name"}
            }
        }),
    )
    .annotate(ToolAnnotations {
        title: Some("List resources".to_string()),
        read_only_hint: Some(true),
        destructive_hint: Some(false),
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}

pub fn search_available_extensions_tool() -> Tool {
    Tool::new(
        PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME.to_string(),
        "Searches for additional extensions available to help complete tasks.
        Use this tool when you're unable to find a specific feature or functionality you need to complete your task, or when standard approaches aren't working.
        These extensions might provide the exact tools needed to solve your problem.
        If you find a relevant one, consider using your tools to enable it.".to_string(),
        object!({
            "type": "object",
            "required": [],
            "properties": {}
        })
    ).annotate(ToolAnnotations {
        title: Some("Discover extensions".to_string()),
        read_only_hint: Some(true),
        destructive_hint: Some(false),
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}

pub fn manage_extensions_tool() -> Tool {
    Tool::new(
        PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME.to_string(),
        "Tool to manage extensions and tools in goose context.
            Enable or disable extensions to help complete tasks.
            Enable or disable an extension by providing the extension name.
            "
        .to_string(),
        object!({
            "type": "object",
            "required": ["action", "extension_name"],
            "properties": {
                "action": {"type": "string", "description": "The action to perform", "enum": ["enable", "disable"]},
                "extension_name": {"type": "string", "description": "The name of the extension to enable"}
            }
        }),
    ).annotate(ToolAnnotations {
        title: Some("Enable or disable an extension".to_string()),
        read_only_hint: Some(false),
        destructive_hint: Some(false),
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}

pub fn manage_schedule_tool() -> Tool {
    Tool::new(
        PLATFORM_MANAGE_SCHEDULE_TOOL_NAME.to_string(),
        indoc! {r#"
            Manage scheduled recipe execution for this Goose instance.
            
            Actions:
            - "list": List all scheduled jobs
            - "create": Create a new scheduled job from a recipe file
            - "run_now": Execute a scheduled job immediately  
            - "pause": Pause a scheduled job
            - "unpause": Resume a paused job
            - "delete": Remove a scheduled job
            - "kill": Terminate a currently running job
            - "inspect": Get details about a running job
            - "sessions": List execution history for a job
            - "session_content": Get the full content (messages) of a specific session
        "#}
        .to_string(),
        object!({
            "type": "object",
            "required": ["action"],
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "create", "run_now", "pause", "unpause", "delete", "kill", "inspect", "sessions", "session_content"]
                },
                "job_id": {"type": "string", "description": "Job identifier for operations on existing jobs"},
                "recipe_path": {"type": "string", "description": "Path to recipe file for create action"},
                "cron_expression": {"type": "string", "description": "A cron expression for create action. Supports both 5-field (minute hour day month weekday) and 6-field (second minute hour day month weekday) formats. 5-field expressions are automatically converted to 6-field by prepending '0' for seconds."},
                "execution_mode": {"type": "string", "description": "Execution mode for create action: 'foreground' or 'background'", "enum": ["foreground", "background"], "default": "background"},
                "limit": {"type": "integer", "description": "Limit for sessions list", "default": 50},
                "session_id": {"type": "string", "description": "Session identifier for session_content action"}
            }
        }),
    ).annotate(ToolAnnotations {
        title: Some("Manage scheduled recipes".to_string()),
        read_only_hint: Some(false),
        destructive_hint: Some(true), // Can kill jobs
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}

/// Platform tools client that provides access to goose platform functionality
pub struct PlatformTools {
    extension_manager: Arc<ExtensionManager>,
    tool_route_manager: Arc<ToolRouteManager>,
}

impl PlatformTools {
    pub fn new(extension_manager: Arc<ExtensionManager>, tool_route_manager: Arc<ToolRouteManager>) -> Self {
        Self { extension_manager, tool_route_manager }
    }

    async fn handle_manage_extensions(&self, arguments: Value) -> Result<CallToolResult, Error> {
        // Extract parameters from arguments
        let extension_name = arguments
            .get("extension_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if extension_name.is_empty() || action.is_empty() {
            return Ok(CallToolResult {
                content: vec![Content::text("Error: Both extension_name and action are required")],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            });
        }

        // Handle tool router index updates if router is functional
        if self.tool_route_manager.is_router_functional().await {
            let selector = self.tool_route_manager.get_router_tool_selector().await;
            if let Some(selector) = selector {
                let selector_action = if action == "disable" { "remove" } else { "add" };
                let selector = Arc::new(selector);
                if let Err(e) = ToolRouterIndexManager::update_extension_tools(
                    &selector,
                    &self.extension_manager,
                    &extension_name,
                    selector_action,
                )
                .await
                {
                    return Ok(CallToolResult {
                        content: vec![Content::text(format!(
                            "Failed to update LLM index: {}",
                            e
                        ))],
                        is_error: Some(true),
                        meta: None,
                        structured_content: None,
                    });
                }
            }
        }

        // Handle disable action
        if action == "disable" {
            match self.extension_manager.remove_extension(&extension_name).await {
                Ok(_) => {
                    let content = vec![Content::text(format!(
                        "The extension '{}' has been disabled successfully",
                        extension_name
                    ))];
                    return Ok(CallToolResult {
                        content,
                        is_error: Some(false),
                        meta: None,
                        structured_content: None,
                    });
                }
                Err(e) => {
                    return Ok(CallToolResult {
                        content: vec![Content::text(format!(
                            "Failed to disable extension '{}': {}",
                            extension_name, e
                        ))],
                        is_error: Some(true),
                        meta: None,
                        structured_content: None,
                    });
                }
            }
        }

        // Handle enable action
        if action == "enable" {
            let config = match ExtensionConfigManager::get_config_by_name(&extension_name) {
                Ok(Some(config)) => config,
                Ok(None) => {
                    return Ok(CallToolResult {
                        content: vec![Content::text(format!(
                            "Extension '{}' not found. Please check the extension name and try again.",
                            extension_name
                        ))],
                        is_error: Some(true),
                        meta: None,
                        structured_content: None,
                    });
                }
                Err(e) => {
                    return Ok(CallToolResult {
                        content: vec![Content::text(format!(
                            "Failed to get extension config: {}",
                            e
                        ))],
                        is_error: Some(true),
                        meta: None,
                        structured_content: None,
                    });
                }
            };

            let result = self.extension_manager.add_extension(config).await;
            
            // Handle post-enable tool router updates if the extension was successfully added
            if result.is_ok() && self.tool_route_manager.is_router_functional().await {
                let selector = self.tool_route_manager.get_router_tool_selector().await;
                if let Some(selector) = selector {
                    let selector = Arc::new(selector);
                    if let Err(e) = ToolRouterIndexManager::update_extension_tools(
                        &selector,
                        &self.extension_manager,
                        &extension_name,
                        "add",
                    )
                    .await
                    {
                        return Ok(CallToolResult {
                            content: vec![Content::text(format!(
                                "Extension enabled but failed to update LLM index: {}",
                                e
                            ))],
                            is_error: Some(true),
                            meta: None,
                            structured_content: None,
                        });
                    }
                }
            }

            match result {
                Ok(_) => {
                    let content = vec![Content::text(format!(
                        "The extension '{}' has been installed successfully",
                        extension_name
                    ))];
                    return Ok(CallToolResult {
                        content,
                        is_error: Some(false),
                        meta: None,
                        structured_content: None,
                    });
                }
                Err(e) => {
                    return Ok(CallToolResult {
                        content: vec![Content::text(format!(
                            "Failed to enable extension '{}': {}",
                            extension_name, e
                        ))],
                        is_error: Some(true),
                        meta: None,
                        structured_content: None,
                    });
                }
            }
        }

        // Invalid action
        Ok(CallToolResult {
            content: vec![Content::text(format!(
                "Invalid action '{}'. Valid actions are 'enable' or 'disable'",
                action
            ))],
            is_error: Some(true),
            meta: None,
            structured_content: None,
        })
    }

    async fn handle_search_available_extensions(&self) -> Result<CallToolResult, Error> {
        match self.extension_manager.search_available_extensions().await {
            Ok(content) => Ok(CallToolResult {
                content,
                is_error: Some(false),
                meta: None,
                structured_content: None,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to search available extensions: {}", e))],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            }),
        }
    }

    async fn handle_read_resource(
        &self,
        arguments: Value,
        cancellation_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        match self.extension_manager.read_resource(arguments, cancellation_token).await {
            Ok(content) => Ok(CallToolResult {
                content,
                is_error: Some(false),
                meta: None,
                structured_content: None,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to read resource: {}", e))],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            }),
        }
    }

    async fn handle_list_resources(
        &self,
        arguments: Value,
        cancellation_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        // Fix parameter name mismatch: tool expects "extension_name" but extension_manager expects "extension"
        let mut fixed_arguments = arguments.clone();
        if let Some(extension_name) = arguments.get("extension_name") {
            fixed_arguments["extension"] = extension_name.clone();
            // Remove the old parameter name to avoid confusion
            if let Some(obj) = fixed_arguments.as_object_mut() {
                obj.remove("extension_name");
            }
        }

        match self.extension_manager.list_resources(fixed_arguments, cancellation_token).await {
            Ok(content) => Ok(CallToolResult {
                content,
                is_error: Some(false),
                meta: None,
                structured_content: None,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to list resources: {}", e))],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            }),
        }
    }
}

#[async_trait]
impl McpClientTrait for PlatformTools {
    async fn list_resources(
        &self,
        _next_cursor: Option<String>,
        _cancel_token: CancellationToken,
    ) -> Result<ListResourcesResult, Error> {
        // Default implementation - to be implemented later
        Ok(ListResourcesResult {
            resources: vec![],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        _uri: &str,
        _cancel_token: CancellationToken,
    ) -> Result<ReadResourceResult, Error> {
        // Default implementation - to be implemented later
        Err(Error::UnexpectedResponse)
    }

    async fn list_tools(
        &self,
        _next_cursor: Option<String>,
        _cancel_token: CancellationToken,
    ) -> Result<ListToolsResult, Error> {
        // Return platform tools that are managed by this client
        let tools = vec![
            manage_extensions_tool(),
            search_available_extensions_tool(),
            read_resource_tool(),
            list_resources_tool(),
        ];
        
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Value,
        cancel_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        match name {
            tool_name if tool_name == PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME => {
                self.handle_manage_extensions(arguments).await
            }
            tool_name if tool_name == PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME => {
                self.handle_search_available_extensions().await
            }
            tool_name if tool_name == PLATFORM_READ_RESOURCE_TOOL_NAME => {
                self.handle_read_resource(arguments, cancel_token).await
            }
            tool_name if tool_name == PLATFORM_LIST_RESOURCES_TOOL_NAME => {
                self.handle_list_resources(arguments, cancel_token).await
            }
            _ => {
                // Tool not handled by this client
                Err(Error::UnexpectedResponse)
            }
        }
    }

    async fn list_prompts(
        &self,
        _next_cursor: Option<String>,
        _cancel_token: CancellationToken,
    ) -> Result<ListPromptsResult, Error> {
        // Default implementation - to be implemented later
        Ok(ListPromptsResult {
            prompts: vec![],
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        _name: &str,
        _arguments: Value,
        _cancel_token: CancellationToken,
    ) -> Result<GetPromptResult, Error> {
        // Default implementation - to be implemented later
        Err(Error::UnexpectedResponse)
    }

    async fn subscribe(&self) -> mpsc::Receiver<ServerNotification> {
        // Default implementation - to be implemented later
        let (_tx, rx) = mpsc::channel(1);
        rx
    }

    fn get_info(&self) -> Option<&InitializeResult> {
        // Default implementation - to be implemented later
        None
    }
}
