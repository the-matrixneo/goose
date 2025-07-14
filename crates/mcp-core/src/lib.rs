// Re-export RMCP types for compatibility during migration
pub use rmcp;

// Feature flag configuration for RMCP migration
pub mod config {
    /// Feature flag to control whether RMCP or legacy implementation is used.
    ///
    /// When `false` (default): Uses the legacy internal MCP implementation
    /// When `true`: Uses the RMCP (official Rust SDK) implementation
    ///
    /// This allows for gradual migration and easy rollback during development.
    pub const USE_RMCP: bool = false;

    /// Check if RMCP implementation should be used
    pub fn use_rmcp() -> bool {
        USE_RMCP
    }

    /// Check if legacy implementation should be used
    pub fn use_legacy() -> bool {
        !USE_RMCP
    }
}

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

#[cfg(test)]
mod tests {
    use super::config;

    #[test]
    fn test_feature_flag_default_state() {
        // By default, RMCP should be disabled (false)
        assert!(!config::USE_RMCP);
        assert!(config::use_legacy());
        assert!(!config::use_rmcp());
    }

    #[test]
    fn test_feature_flag_functions() {
        // Test that the helper functions work correctly
        assert_eq!(config::use_rmcp(), config::USE_RMCP);
        assert_eq!(config::use_legacy(), !config::USE_RMCP);
    }
}
