use std::sync::Arc;

use async_trait::async_trait;
use mcp_client::client::{Error, McpClientTrait};
use rmcp::model::{
    CallToolResult, Content, GetPromptResult, InitializeResult, ListPromptsResult,
    ListResourcesResult, ListToolsResult, ReadResourceResult, ServerNotification, Tool,
};
use serde_json::Value;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;

use crate::agents::todo_tools::{todo_read_tool, todo_write_tool};
use crate::agents::types::SessionConfig;
use crate::session::extension_data::ExtensionState;
use crate::session::{self, TodoState};

/// Storage trait for TODO persistence
#[async_trait]
pub trait TodoStorage: Send + Sync {
    async fn read(&self) -> Result<String, String>;
    async fn write(&self, content: String) -> Result<String, String>;
}

/// Session-based TODO storage implementation
pub struct SessionTodoStorage {
    session_id: session::Identifier,
}

impl SessionTodoStorage {
    pub fn new(session_id: session::Identifier) -> Self {
        Self { session_id }
    }
}

#[async_trait]
impl TodoStorage for SessionTodoStorage {
    async fn read(&self) -> Result<String, String> {
        let session_path = session::storage::get_path(self.session_id.clone())
            .map_err(|e| format!("Failed to get session path: {}", e))?;

        let metadata = session::storage::read_metadata(&session_path)
            .map_err(|e| format!("Failed to read session metadata: {}", e))?;

        let content = TodoState::from_extension_data(&metadata.extension_data)
            .map(|state| state.content)
            .unwrap_or_default();

        Ok(content)
    }

    async fn write(&self, content: String) -> Result<String, String> {
        // Character limit validation
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

        let session_path = session::storage::get_path(self.session_id.clone())
            .map_err(|e| format!("Failed to get session path: {}", e))?;

        let mut metadata = session::storage::read_metadata(&session_path)
            .map_err(|e| format!("Failed to read session metadata: {}", e))?;

        let todo_state = TodoState::new(content);
        todo_state
            .to_extension_data(&mut metadata.extension_data)
            .map_err(|e| format!("Failed to update extension data: {}", e))?;

        session::storage::update_metadata(&session_path, &metadata)
            .await
            .map_err(|e| format!("Failed to update session metadata: {}", e))?;

        Ok(format!("Updated ({} chars)", char_count))
    }
}

/// In-memory TODO storage for sessions without persistence
pub struct MemoryTodoStorage {
    content: Arc<Mutex<String>>,
}

impl MemoryTodoStorage {
    pub fn new() -> Self {
        Self {
            content: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl Default for MemoryTodoStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TodoStorage for MemoryTodoStorage {
    async fn read(&self) -> Result<String, String> {
        Ok(self.content.lock().await.clone())
    }

    async fn write(&self, content: String) -> Result<String, String> {
        let char_count = content.chars().count();
        *self.content.lock().await = content;
        Ok(format!("Updated ({} chars)", char_count))
    }
}

/// Internal MCP client for TODO functionality
pub struct TodoMcpClient {
    storage: Arc<dyn TodoStorage>,
    server_info: InitializeResult,
}

impl TodoMcpClient {
    pub fn new(storage: Arc<dyn TodoStorage>) -> Self {
        let instructions = r#"The todo extension provides persistent task management throughout your session.

These tools help you track multi-step work, maintain context between interactions, and ensure systematic task completion.

## Task Management Guidelines

**Required Usage:**
- Use `todo__read` and `todo__write` for any task with 2+ steps, multiple files/components, or uncertain scope
- Skipping these tools when needed is considered an error

**Workflow:**
1. Start: Always `todo__read` first, then `todo__write` a brief checklist using Markdown checkboxes
2. During: After each major action, reread via `todo__read`, then update via `todo__write` - mark completed items, add new discoveries, note blockers
3. Finish: Ensure every item is checked, or clearly list what remains

**Critical:** `todo__write` replaces the entire list. Always read before writing - not doing so is an error.

**Best Practices:**
- Keep items short, specific, and action-oriented
- Use nested checkboxes for subtasks
- Include context about blockers or dependencies

Example format:
```markdown
- [x] Analyze request fully
- [ ] Create implementation plan
  - [x] General guidelines
  - [ ] Sample work
- [ ] Begin on implementation plan
```"#;

        let server_info = InitializeResult {
            protocol_version: rmcp::model::ProtocolVersion::V_2025_03_26,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: None }),
                prompts: None,
                resources: None,
                logging: None,
                experimental: None,
                completions: None,
            },
            server_info: rmcp::model::Implementation {
                name: "todo".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(instructions.to_string()),
        };

        Self {
            storage,
            server_info,
        }
    }

    pub fn with_session(session: &SessionConfig) -> Self {
        // Use the session ID directly - the storage layer will handle it properly
        let storage = Arc::new(SessionTodoStorage::new(session.id.clone()));
        Self::new(storage)
    }

    pub fn memory_only() -> Self {
        let storage = Arc::new(MemoryTodoStorage::new());
        Self::new(storage)
    }
}

#[async_trait]
impl McpClientTrait for TodoMcpClient {
    fn get_info(&self) -> Option<&InitializeResult> {
        Some(&self.server_info)
    }

    async fn list_resources(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListResourcesResult, Error> {
        Ok(ListResourcesResult {
            resources: vec![],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        _uri: &str,
        _cancellation_token: CancellationToken,
    ) -> Result<ReadResourceResult, Error> {
        Err(Error::UnexpectedResponse)
    }

    async fn list_tools(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListToolsResult, Error> {
        // Return unprefixed tools - the extension manager will add the prefix
        Ok(ListToolsResult {
            tools: vec![
                Tool {
                    name: "read".into(),
                    description: todo_read_tool().description,
                    input_schema: todo_read_tool().input_schema,
                    annotations: todo_read_tool().annotations,
                    output_schema: todo_read_tool().output_schema,
                },
                Tool {
                    name: "write".into(),
                    description: todo_write_tool().description,
                    input_schema: todo_write_tool().input_schema,
                    annotations: todo_write_tool().annotations,
                    output_schema: todo_write_tool().output_schema,
                },
            ],
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Value,
        _cancellation_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        match name {
            "read" => {
                let content = self.storage.read().await.map_err(|e| {
                    Error::McpError(rmcp::model::ErrorData::new(
                        rmcp::model::ErrorCode::INTERNAL_ERROR,
                        e,
                        None,
                    ))
                })?;
                Ok(CallToolResult {
                    content: vec![Content::text(content)],
                    is_error: None,
                    structured_content: None,
                })
            }
            "write" => {
                let content = arguments
                    .get("content")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::McpError(rmcp::model::ErrorData::new(
                            rmcp::model::ErrorCode::INVALID_PARAMS,
                            "Missing 'content' parameter".to_string(),
                            None,
                        ))
                    })?
                    .to_string();

                let result = self.storage.write(content).await.map_err(|e| {
                    Error::McpError(rmcp::model::ErrorData::new(
                        rmcp::model::ErrorCode::INTERNAL_ERROR,
                        e,
                        None,
                    ))
                })?;

                Ok(CallToolResult {
                    content: vec![Content::text(result)],
                    is_error: None,
                    structured_content: None,
                })
            }
            _ => Err(Error::UnexpectedResponse),
        }
    }

    async fn list_prompts(
        &self,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListPromptsResult, Error> {
        Ok(ListPromptsResult {
            prompts: vec![],
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        _name: &str,
        _arguments: Value,
        _cancellation_token: CancellationToken,
    ) -> Result<GetPromptResult, Error> {
        Err(Error::UnexpectedResponse)
    }

    async fn subscribe(&self) -> mpsc::Receiver<ServerNotification> {
        // Return a receiver that will never send anything
        let (_tx, rx) = mpsc::channel(1);
        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryTodoStorage::new();

        // Test initial read
        let content = storage.read().await.unwrap();
        assert_eq!(content, "");

        // Test write
        let result = storage.write("- [ ] Test task".to_string()).await.unwrap();
        assert!(result.contains("Updated"));

        // Test read after write
        let content = storage.read().await.unwrap();
        assert_eq!(content, "- [ ] Test task");
    }

    #[tokio::test]
    async fn test_todo_client_list_tools() {
        let client = TodoMcpClient::memory_only();

        let tools = client
            .list_tools(None, CancellationToken::default())
            .await
            .unwrap();

        assert_eq!(tools.tools.len(), 2);
        assert_eq!(tools.tools[0].name, "read");
        assert_eq!(tools.tools[1].name, "write");
    }

    #[tokio::test]
    async fn test_todo_client_read_write() {
        let client = TodoMcpClient::memory_only();

        // Test read empty
        let result = client
            .call_tool("read", serde_json::json!({}), CancellationToken::default())
            .await
            .unwrap();
        assert_eq!(result.content[0], Content::text(""));

        // Test write
        let result = client
            .call_tool(
                "write",
                serde_json::json!({"content": "- [ ] Task 1"}),
                CancellationToken::default(),
            )
            .await
            .unwrap();
        assert!(result.content[0]
            .as_text()
            .unwrap()
            .text
            .contains("Updated"));

        // Test read after write
        let result = client
            .call_tool("read", serde_json::json!({}), CancellationToken::default())
            .await
            .unwrap();
        assert_eq!(result.content[0], Content::text("- [ ] Task 1"));
    }

    #[tokio::test]
    async fn test_todo_client_invalid_tool() {
        let client = TodoMcpClient::memory_only();

        let result = client
            .call_tool(
                "invalid",
                serde_json::json!({}),
                CancellationToken::default(),
            )
            .await;

        assert!(matches!(result, Err(Error::UnexpectedResponse)));
    }

    #[tokio::test]
    async fn test_todo_client_missing_content_param() {
        let client = TodoMcpClient::memory_only();

        let result = client
            .call_tool("write", serde_json::json!({}), CancellationToken::default())
            .await;

        assert!(matches!(result, Err(Error::McpError(_))));
    }
}
