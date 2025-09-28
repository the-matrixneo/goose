use crate::agents::extension::PlatformExtensionContext;
use crate::session::extension_data::ExtensionState;
use crate::session::{extension_data, SessionManager};
use anyhow::Result;
use async_trait::async_trait;
use mcp_client::client::{Error, McpClientTrait};
use rmcp::model::{
    CallToolResult, Content, GetPromptResult, Implementation, InitializeResult, ListPromptsResult,
    ListResourcesResult, ListToolsResult, ProtocolVersion, ReadResourceResult, ServerCapabilities,
    ServerNotification, Tool, ToolsCapability,
};
use serde_json::{json, Map, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub static EXTENSION_NAME: &str = "todo";

pub struct TodoClient {
    info: InitializeResult,
    context: PlatformExtensionContext,
}

impl TodoClient {
    pub fn new(context: PlatformExtensionContext) -> Result<Self> {
        let info = InitializeResult {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: None,
                prompts: None,
                completions: None,
                experimental: None,
                logging: None,
            },
            server_info: Implementation {
                name: "todo".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(
                "Manage TODO lists - read and write task planning content.".to_string(),
            ),
        };

        Ok(Self { info, context })
    }

    async fn handle_read_todo(&self) -> Result<Vec<Content>, String> {
        match SessionManager::get_session(&self.context.session_id, false).await {
            Ok(metadata) => {
                let content =
                    extension_data::TodoState::from_extension_data(&metadata.extension_data)
                        .map(|state| state.content)
                        .unwrap_or_default();
                Ok(vec![Content::text(content)])
            }
            Err(_) => Ok(vec![Content::text(String::new())]),
        }
    }

    async fn handle_write_todo(&self, arguments: Value) -> Result<Vec<Content>, String> {
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: content")?
            .to_string();

        let char_count = content.chars().count();
        let max_chars = std::env::var("GOOSE_TODO_MAX_CHARS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50_000);

        if max_chars > 0 && char_count > max_chars {
            return Err(format!(
                "Todo list too large: {} chars (max: {})",
                char_count, max_chars
            ));
        }

        match SessionManager::get_session(&self.context.session_id, false).await {
            Ok(mut session) => {
                let todo_state = extension_data::TodoState::new(content);
                if todo_state
                    .to_extension_data(&mut session.extension_data)
                    .is_ok()
                {
                    match SessionManager::update_session(&self.context.session_id)
                        .extension_data(session.extension_data)
                        .apply()
                        .await
                    {
                        Ok(_) => Ok(vec![Content::text(format!(
                            "Updated ({} chars)",
                            char_count
                        ))]),
                        Err(_) => Err("Failed to update session metadata".to_string()),
                    }
                } else {
                    Err("Failed to serialize TODO state".to_string())
                }
            }
            Err(_) => Err("Failed to read session metadata".to_string()),
        }
    }

    fn get_tools() -> Vec<Tool> {
        fn create_schema(json_value: Value) -> Arc<Map<String, Value>> {
            Arc::new(json_value.as_object().unwrap().clone())
        }

        vec![
            Tool {
                name: "todo_read".into(),
                description: Some("Read the current TODO list content".into()),
                input_schema: create_schema(json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                })),
                annotations: None,
                output_schema: None,
            },
            Tool {
                name: "todo_write".into(),
                description: Some("Write/update the TODO list content".into()),
                input_schema: create_schema(json!({
                    "type": "object",
                    "properties": {
                        "content": {
                            "type": "string",
                            "description": "The TODO list content to save"
                        }
                    },
                    "required": ["content"]
                })),
                annotations: None,
                output_schema: None,
            },
        ]
    }
}

#[async_trait]
impl McpClientTrait for TodoClient {
    async fn list_resources(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListResourcesResult, Error> {
        Err(Error::TransportClosed)
    }

    async fn read_resource(
        &self,
        _uri: &str,
        _cancellation_token: CancellationToken,
    ) -> Result<ReadResourceResult, Error> {
        Err(Error::TransportClosed)
    }

    async fn list_tools(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListToolsResult, Error> {
        Ok(ListToolsResult {
            tools: Self::get_tools(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Value,
        _cancellation_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        let content = match name {
            "todo_read" => self.handle_read_todo().await,
            "todo_write" => self.handle_write_todo(arguments).await,
            _ => Err(format!("Unknown tool: {}", name)),
        };

        match content {
            Ok(content) => Ok(CallToolResult::success(content)),
            Err(error) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                error
            ))])),
        }
    }

    async fn list_prompts(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListPromptsResult, Error> {
        Err(Error::TransportClosed)
    }

    async fn get_prompt(
        &self,
        _name: &str,
        _arguments: Value,
        _cancellation_token: CancellationToken,
    ) -> Result<GetPromptResult, Error> {
        Err(Error::TransportClosed)
    }

    async fn subscribe(&self) -> mpsc::Receiver<ServerNotification> {
        mpsc::channel(1).1
    }

    fn get_info(&self) -> Option<&InitializeResult> {
        Some(&self.info)
    }
}
