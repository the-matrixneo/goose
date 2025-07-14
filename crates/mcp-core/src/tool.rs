/// Tools represent a routine that a server can execute
/// Tool calls represent requests from the client to execute one
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// Additional properties describing a tool to clients.
///
/// NOTE: all properties in ToolAnnotations are **hints**.
/// They are not guaranteed to provide a faithful description of
/// tool behavior (including descriptive properties like `title`).
///
/// Clients should never make tool use decisions based on ToolAnnotations
/// received from untrusted servers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    /// A human-readable title for the tool.
    pub title: Option<String>,

    /// If true, the tool does not modify its environment.
    ///
    /// Default: false
    #[serde(default)]
    pub read_only_hint: bool,

    /// If true, the tool may perform destructive updates to its environment.
    /// If false, the tool performs only additive updates.
    ///
    /// (This property is meaningful only when `read_only_hint == false`)
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub destructive_hint: bool,

    /// If true, calling the tool repeatedly with the same arguments
    /// will have no additional effect on its environment.
    ///
    /// (This property is meaningful only when `read_only_hint == false`)
    ///
    /// Default: false
    #[serde(default)]
    pub idempotent_hint: bool,

    /// If true, this tool may interact with an "open world" of external
    /// entities. If false, the tool's domain of interaction is closed.
    /// For example, the world of a web search tool is open, whereas that
    /// of a memory tool is not.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub open_world_hint: bool,
}

impl Default for ToolAnnotations {
    fn default() -> Self {
        ToolAnnotations {
            title: None,
            read_only_hint: false,
            destructive_hint: true,
            idempotent_hint: false,
            open_world_hint: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Implement builder methods for `ToolAnnotations`
impl ToolAnnotations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only_hint = read_only;
        self
    }

    pub fn with_destructive(mut self, destructive: bool) -> Self {
        self.destructive_hint = destructive;
        self
    }

    pub fn with_idempotent(mut self, idempotent: bool) -> Self {
        self.idempotent_hint = idempotent;
        self
    }

    pub fn with_open_world(mut self, open_world: bool) -> Self {
        self.open_world_hint = open_world;
        self
    }
}

/// A tool that can be used by a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    /// The name of the tool
    pub name: String,
    /// A description of what the tool does
    pub description: String,
    /// A JSON Schema object defining the expected parameters for the tool
    pub input_schema: Value,
    /// Optional additional tool information.
    pub annotations: Option<ToolAnnotations>,
}

impl Tool {
    /// Create a new tool with the given name and description
    pub fn new<N, D>(
        name: N,
        description: D,
        input_schema: Value,
        annotations: Option<ToolAnnotations>,
    ) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Tool {
            name: name.into(),
            description: description.into(),
            input_schema,
            annotations,
        }
    }
}

/// A tool call request that an extension can execute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    /// The name of the tool to execute
    pub name: String,
    /// The parameters for the execution
    pub arguments: Value,
}

impl ToolCall {
    /// Create a new ToolUse with the given name and parameters
    pub fn new<S: Into<String>>(name: S, arguments: Value) -> Self {
        Self {
            name: name.into(),
            arguments,
        }
    }
}

// RMCP compatibility conversions
impl Tool {
    /// Convert to RMCP Tool when feature flag is enabled
    pub fn to_rmcp(&self) -> crate::rmcp::model::Tool {
        use serde_json::Value;
        use std::sync::Arc;

        // Convert Value to JsonObject (Map<String, Value>)
        let input_schema = match &self.input_schema {
            Value::Object(map) => Arc::new(map.clone()),
            _ => Arc::new(serde_json::Map::new()),
        };

        crate::rmcp::model::Tool {
            name: self.name.clone().into(),
            description: Some(self.description.clone().into()),
            input_schema,
            annotations: self
                .annotations
                .as_ref()
                .map(|ann| crate::rmcp::model::ToolAnnotations {
                    title: ann.title.clone(),
                    read_only_hint: Some(ann.read_only_hint),
                    destructive_hint: Some(ann.destructive_hint),
                    idempotent_hint: Some(ann.idempotent_hint),
                    open_world_hint: Some(ann.open_world_hint),
                }),
        }
    }

    /// Create from RMCP Tool
    pub fn from_rmcp(rmcp_tool: crate::rmcp::model::Tool) -> Self {
        Tool {
            name: rmcp_tool.name.to_string(),
            description: rmcp_tool
                .description
                .map(|d| d.to_string())
                .unwrap_or_default(),
            input_schema: Value::Object(rmcp_tool.input_schema.as_ref().clone()),
            annotations: rmcp_tool.annotations.map(|ann| ToolAnnotations {
                title: ann.title,
                read_only_hint: ann.read_only_hint.unwrap_or(false),
                destructive_hint: ann.destructive_hint.unwrap_or(true),
                idempotent_hint: ann.idempotent_hint.unwrap_or(false),
                open_world_hint: ann.open_world_hint.unwrap_or(true),
            }),
        }
    }
}

impl From<crate::rmcp::model::Tool> for Tool {
    fn from(rmcp_tool: crate::rmcp::model::Tool) -> Self {
        Tool::from_rmcp(rmcp_tool)
    }
}

impl From<Tool> for crate::rmcp::model::Tool {
    fn from(tool: Tool) -> Self {
        tool.to_rmcp()
    }
}

impl ToolCall {
    /// Convert to RMCP CallToolRequestParam when feature flag is enabled
    pub fn to_rmcp(&self) -> crate::rmcp::model::CallToolRequestParam {
        crate::rmcp::model::CallToolRequestParam {
            name: self.name.clone().into(),
            arguments: if self.arguments.is_null() {
                None
            } else {
                Some(self.arguments.as_object().cloned().unwrap_or_default())
            },
        }
    }

    /// Create from RMCP CallToolRequestParam
    pub fn from_rmcp(rmcp_call: crate::rmcp::model::CallToolRequestParam) -> Self {
        ToolCall {
            name: rmcp_call.name.to_string(),
            arguments: rmcp_call
                .arguments
                .map(Value::Object)
                .unwrap_or(Value::Null),
        }
    }
}

impl From<crate::rmcp::model::CallToolRequestParam> for ToolCall {
    fn from(rmcp_call: crate::rmcp::model::CallToolRequestParam) -> Self {
        ToolCall::from_rmcp(rmcp_call)
    }
}

impl From<ToolCall> for crate::rmcp::model::CallToolRequestParam {
    fn from(tool_call: ToolCall) -> Self {
        tool_call.to_rmcp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_rmcp_conversion() {
        let original = Tool::new(
            "test_tool",
            "A test tool",
            json!({
                "type": "object",
                "properties": {
                    "param": {"type": "string"}
                }
            }),
            Some(ToolAnnotations::new().with_title("Test Tool")),
        );

        let rmcp_tool = original.to_rmcp();
        let converted_back = Tool::from_rmcp(rmcp_tool);

        assert_eq!(original.name, converted_back.name);
        assert_eq!(original.description, converted_back.description);
        assert_eq!(original.input_schema, converted_back.input_schema);
        assert_eq!(
            original.annotations.as_ref().unwrap().title,
            converted_back.annotations.as_ref().unwrap().title
        );
    }

    #[test]
    fn test_tool_call_rmcp_conversion() {
        let original = ToolCall::new(
            "test_tool",
            json!({
                "param": "value"
            }),
        );

        let rmcp_call = original.to_rmcp();
        let converted_back = ToolCall::from_rmcp(rmcp_call);

        assert_eq!(original.name, converted_back.name);
        assert_eq!(original.arguments, converted_back.arguments);
    }

    #[test]
    fn test_tool_call_with_null_arguments() {
        let original = ToolCall::new("test_tool", serde_json::Value::Null);

        let rmcp_call = original.to_rmcp();
        let converted_back = ToolCall::from_rmcp(rmcp_call);

        assert_eq!(original.name, converted_back.name);
        assert_eq!(converted_back.arguments, serde_json::Value::Null);
    }

    #[test]
    fn test_tool_annotations_conversion() {
        let annotations = ToolAnnotations::new()
            .with_title("Test Tool")
            .with_read_only(true)
            .with_destructive(false)
            .with_idempotent(true)
            .with_open_world(false);

        let tool = Tool::new(
            "test",
            "Test tool",
            json!({"type": "object"}),
            Some(annotations),
        );

        let rmcp_tool = tool.to_rmcp();
        let converted_back = Tool::from_rmcp(rmcp_tool);

        let converted_annotations = converted_back.annotations.unwrap();
        assert_eq!(converted_annotations.title, Some("Test Tool".to_string()));
        assert_eq!(converted_annotations.read_only_hint, true);
        assert_eq!(converted_annotations.destructive_hint, false);
        assert_eq!(converted_annotations.idempotent_hint, true);
        assert_eq!(converted_annotations.open_world_hint, false);
    }
}
