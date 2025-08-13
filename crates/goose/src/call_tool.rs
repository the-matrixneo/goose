use rmcp::model::{CallToolRequest, CallToolRequestParam};

/// Helper to construct a CallToolRequest from a tool name and JSON arguments (as Value).
/// Arguments must be a JSON object; if not, arguments will be set to None.
pub fn make_call_tool_request<N: Into<String>>(name: N, arguments: serde_json::Value) -> CallToolRequest {
    let arguments = match arguments {
        serde_json::Value::Object(map) => Some(map),
        _ => None,
    };
    CallToolRequest {
        params: CallToolRequestParam {
            name: name.into().into(),
            arguments,
        },
        method: Default::default(),
        extensions: Default::default(),
    }
}

/// Convert CallToolRequest params.arguments into a serde_json::Value::Object map
/// with default empty object when None.
pub fn args_value(req: &CallToolRequest) -> serde_json::Value {
    serde_json::Value::Object(req.params.arguments.clone().unwrap_or_default())
}

/// Get name reference
pub fn name(req: &CallToolRequest) -> &str {
    &req.params.name
}
