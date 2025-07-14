// Re-export RMCP types for compatibility during migration
pub use rmcp;

// Legacy module structure - maintained for backward compatibility
pub mod content;
pub use content::{Annotations, Content, ImageContent, TextContent};
pub mod handler;
pub mod role;
pub use role::Role;
pub mod tool;
pub use tool::{Tool, ToolCall};
pub mod resource;
pub use resource::{Resource, ResourceContents};
pub mod protocol;
pub use handler::{ToolError, ToolResult};
pub mod prompt;

// RMCP compatibility re-exports
// These provide a bridge between the old API and the new RMCP types
pub mod rmcp_compat {
    pub use rmcp::model::ErrorData;
    pub use rmcp::model::*;
    pub use rmcp::Service;
    pub use rmcp::ServiceExt;

    // Type aliases for easier migration
    pub type RmcpContent = rmcp::model::Content;
    pub type RmcpTool = rmcp::model::Tool;
    pub type RmcpResource = rmcp::model::Resource;
    pub type RmcpPrompt = rmcp::model::Prompt;
    pub type RmcpJsonRpcMessage = rmcp::model::JsonRpcMessage;
    pub type RmcpErrorData = rmcp::model::ErrorData;
}
