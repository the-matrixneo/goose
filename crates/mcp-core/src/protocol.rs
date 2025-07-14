/// The protocol messages exchanged between client and server
use crate::{
    config,
    content::Content,
    prompt::{Prompt, PromptMessage},
    resource::Resource,
    resource::ResourceContents,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JsonRpcError {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub error: ErrorData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged, try_from = "JsonRpcRaw")]
pub enum JsonRpcMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
    Error(JsonRpcError),
    Nil, // used to respond to notifications
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRaw {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorData>,
}

impl TryFrom<JsonRpcRaw> for JsonRpcMessage {
    type Error = String;

    fn try_from(raw: JsonRpcRaw) -> Result<Self, <Self as TryFrom<JsonRpcRaw>>::Error> {
        // If it has an error field, it's an error response
        if raw.error.is_some() {
            return Ok(JsonRpcMessage::Error(JsonRpcError {
                jsonrpc: raw.jsonrpc,
                id: raw.id,
                error: raw.error.unwrap(),
            }));
        }

        // If it has a result field, it's a response
        if raw.result.is_some() {
            return Ok(JsonRpcMessage::Response(JsonRpcResponse {
                jsonrpc: raw.jsonrpc,
                id: raw.id,
                result: raw.result,
                error: None,
            }));
        }

        // If we have a method, it's either a notification or request
        if let Some(method) = raw.method {
            if raw.id.is_none() {
                return Ok(JsonRpcMessage::Notification(JsonRpcNotification {
                    jsonrpc: raw.jsonrpc,
                    method,
                    params: raw.params,
                }));
            }

            return Ok(JsonRpcMessage::Request(JsonRpcRequest {
                jsonrpc: raw.jsonrpc,
                id: raw.id,
                method,
                params: raw.params,
            }));
        }

        // If we have no method and no result/error, it's a nil response
        if raw.id.is_none() && raw.result.is_none() && raw.error.is_none() {
            return Ok(JsonRpcMessage::Nil);
        }

        // If we get here, something is wrong with the message
        Err(format!(
            "Invalid JSON-RPC message format: id={:?}, method={:?}, result={:?}, error={:?}",
            raw.id, raw.method, raw.result, raw.error
        ))
    }
}

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

/// Error information for JSON-RPC error responses.
///
/// This type provides compatibility between the legacy implementation and RMCP.
/// When USE_RMCP is true, it uses RMCP's ErrorData internally.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorData {
    /// The error type that occurred.
    pub code: i32,

    /// A short description of the error. The message SHOULD be limited to a concise single sentence.
    pub message: String,

    /// Additional information about the error. The value of this member is defined by the
    /// sender (e.g. detailed error information, nested errors etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ErrorData {
    /// Create a new ErrorData instance
    pub fn new(code: i32, message: String, data: Option<Value>) -> Self {
        if config::use_rmcp() {
            // Convert to RMCP ErrorData and back to maintain compatibility
            let rmcp_error = crate::rmcp::model::ErrorData::new(
                crate::rmcp::model::ErrorCode(code),
                message.clone(),
                data.clone(),
            );
            Self {
                code: rmcp_error.code.0,
                message: rmcp_error.message.to_string(),
                data: rmcp_error.data,
            }
        } else {
            Self {
                code,
                message,
                data,
            }
        }
    }

    /// Create a resource not found error
    pub fn resource_not_found(message: String, data: Option<Value>) -> Self {
        Self::new(
            crate::rmcp::model::ErrorCode::RESOURCE_NOT_FOUND.0,
            message,
            data,
        )
    }

    /// Create a parse error
    pub fn parse_error(message: String, data: Option<Value>) -> Self {
        Self::new(PARSE_ERROR, message, data)
    }

    /// Create an invalid request error
    pub fn invalid_request(message: String, data: Option<Value>) -> Self {
        Self::new(INVALID_REQUEST, message, data)
    }

    /// Create a method not found error
    pub fn method_not_found(message: String, data: Option<Value>) -> Self {
        Self::new(METHOD_NOT_FOUND, message, data)
    }

    /// Create an invalid params error
    pub fn invalid_params(message: String, data: Option<Value>) -> Self {
        Self::new(INVALID_PARAMS, message, data)
    }

    /// Create an internal error
    pub fn internal_error(message: String, data: Option<Value>) -> Self {
        Self::new(INTERNAL_ERROR, message, data)
    }
}

// Conversion functions for RMCP compatibility
impl From<crate::rmcp::model::ErrorData> for ErrorData {
    fn from(rmcp_error: crate::rmcp::model::ErrorData) -> Self {
        Self {
            code: rmcp_error.code.0,
            message: rmcp_error.message.to_string(),
            data: rmcp_error.data,
        }
    }
}

impl From<ErrorData> for crate::rmcp::model::ErrorData {
    fn from(error: ErrorData) -> Self {
        crate::rmcp::model::ErrorData::new(
            crate::rmcp::model::ErrorCode(error.code),
            error.message,
            error.data,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: Implementation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Implementation {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    // Add other capabilities as needed
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PromptsCapability {
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesCapability {
    pub subscribe: Option<bool>,
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCapability {
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContents>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListPromptsResult {
    pub prompts: Vec<Prompt>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GetPromptResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResult {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_notification_conversion() {
        let raw = JsonRpcRaw {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("notify".to_string()),
            params: Some(json!({"key": "value"})),
            result: None,
            error: None,
        };

        let message = JsonRpcMessage::try_from(raw).unwrap();
        match message {
            JsonRpcMessage::Notification(n) => {
                assert_eq!(n.jsonrpc, "2.0");
                assert_eq!(n.method, "notify");
                assert_eq!(n.params.unwrap(), json!({"key": "value"}));
            }
            _ => panic!("Expected Notification"),
        }
    }

    #[test]
    fn test_request_conversion() {
        let raw = JsonRpcRaw {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("request".to_string()),
            params: Some(json!({"key": "value"})),
            result: None,
            error: None,
        };

        let message = JsonRpcMessage::try_from(raw).unwrap();
        match message {
            JsonRpcMessage::Request(r) => {
                assert_eq!(r.jsonrpc, "2.0");
                assert_eq!(r.id, Some(1));
                assert_eq!(r.method, "request");
                assert_eq!(r.params.unwrap(), json!({"key": "value"}));
            }
            _ => panic!("Expected Request"),
        }
    }
}
