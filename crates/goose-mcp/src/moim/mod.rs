use chrono::Local;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorCode, ErrorData, Implementation, ServerCapabilities,
        ServerInfo,
    },
    schemars::JsonSchema,
    tool, tool_handler, tool_router, ServerHandler,
};
use serde::{Deserialize, Serialize};

/// Parameters for the moim__read tool
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MoimReadParams {
    /// Optional session ID to get TODO content from
    #[serde(default)]
    pub session_id: Option<String>,
}

/// MOIM (Minus One Info Message) MCP Server
/// Provides system context updates including timestamp and TODO content
#[derive(Clone)]
pub struct MoimServer {
    tool_router: ToolRouter<Self>,
    instructions: String,
}

impl Default for MoimServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl MoimServer {
    pub fn new() -> Self {
        let instructions = "MOIM provides system context updates including current timestamp and TODO content. \
                           The moim__read tool is automatically called periodically to refresh context.".to_string();

        Self {
            tool_router: Self::tool_router(),
            instructions,
        }
    }

    /// Reads system context including timestamp and TODO content
    #[tool(
        name = "moim__read",
        description = "Refresh system context including timestamp and TODO content"
    )]
    pub async fn moim_read(
        &self,
        params: Parameters<MoimReadParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;
        let mut content = String::new();

        // Add header
        content.push_str("=== System Context Update ===\n");

        // Add timestamp
        content.push_str(&format!(
            "Current date and time: {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        // Add TODO content if session_id provided
        if let Some(session_id) = params.session_id {
            match self.get_todo_content(&session_id).await {
                Ok(Some(todo_content)) => {
                    content.push_str("\nCurrent TODO list:\n");
                    content.push_str(&todo_content);
                    content.push('\n');
                }
                Ok(None) => {
                    // No TODO content
                }
                Err(e) => {
                    tracing::debug!("Could not read TODO for MOIM: {}", e);
                }
            }
        }

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    async fn get_todo_content(&self, session_id: &str) -> Result<Option<String>, ErrorData> {
        // This is a placeholder - in the actual implementation, this would
        // access the session storage to get TODO content
        // For now, we'll return None to avoid circular dependencies

        // In the future, this could be passed via a different mechanism
        // or the Agent could include TODO content in the tool parameters
        let _ = session_id;
        Ok(None)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for MoimServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "goose-moim".to_string(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(self.instructions.clone()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_moim_read_without_session() {
        let server = MoimServer::new();
        let params = Parameters(MoimReadParams { session_id: None });

        let result = server.moim_read(params).await;
        assert!(result.is_ok());

        let content = &result.unwrap().content[0];
        if let Content::Text(text) = content {
            assert!(text.text.contains("System Context Update"));
            assert!(text.text.contains("Current date and time:"));
            assert!(!text.text.contains("TODO"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_moim_read_with_session() {
        let server = MoimServer::new();
        let params = Parameters(MoimReadParams {
            session_id: Some("test_session".to_string()),
        });

        let result = server.moim_read(params).await;
        assert!(result.is_ok());

        let content = &result.unwrap().content[0];
        if let Content::Text(text) = content {
            assert!(text.text.contains("System Context Update"));
            assert!(text.text.contains("Current date and time:"));
        } else {
            panic!("Expected text content");
        }
    }
}
