use crate::agents::a2a::{
    agent_card::{AgentCard, Capability},
    communication::{A2ACommunication, CommunicationConfig, CommunicationRequest, CommunicationMode},
    discovery::{AgentDiscovery, DiscoveryConfig, DiscoveryRequest},
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

/// A2A Protocol version
pub const A2A_VERSION: &str = "1.0.0";

/// JSON-RPC 2.0 message structure for A2A communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// A2A Request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2ARequest {
    /// The capability ID being invoked
    pub capability_id: String,
    /// Input parameters for the capability
    pub input: Value,
    /// Optional metadata for the request
    pub metadata: Option<HashMap<String, Value>>,
    /// Whether streaming response is requested
    pub streaming: Option<bool>,
}

/// A2A Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AResponse {
    /// The output data from the capability
    pub output: Value,
    /// Optional metadata about the response
    pub metadata: Option<HashMap<String, Value>>,
    /// Whether this is a partial response in a stream
    pub partial: Option<bool>,
}

/// A2A Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AError {
    /// Error code following JSON-RPC conventions
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Additional error data
    pub data: Option<Value>,
}

/// Main A2A Protocol handler that coordinates discovery, communication, and agent management
pub struct A2AProtocol {
    /// This agent's card
    agent_card: Arc<RwLock<AgentCard>>,
    /// Agent discovery service
    discovery: AgentDiscovery,
    /// Communication manager
    communication: A2ACommunication,
    /// Configuration
    config: A2AConfig,
}

/// Configuration for the A2A Protocol
#[derive(Debug, Clone)]
pub struct A2AConfig {
    /// This agent's information
    pub agent_id: String,
    pub agent_name: String,
    pub agent_version: String,
    pub base_url: Url,
    /// Discovery configuration
    pub discovery: DiscoveryConfig,
    /// Communication configuration
    pub communication: CommunicationConfig,
    /// Whether to auto-register with discovery services
    pub auto_register: bool,
}

// Standard A2A error codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    pub const CAPABILITY_NOT_FOUND: i32 = -32000;
    pub const CAPABILITY_ERROR: i32 = -32001;
    pub const AUTHENTICATION_ERROR: i32 = -32002;
    pub const AUTHORIZATION_ERROR: i32 = -32003;
    pub const RATE_LIMIT_ERROR: i32 = -32004;
}

impl JsonRpcMessage {
    /// Create a new request message
    pub fn request(id: Value, method: String, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: Some(method),
            params,
            result: None,
            error: None,
        }
    }

    /// Create a new response message
    pub fn response(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }

    /// Create a new error response message
    pub fn error_response(id: Option<Value>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: None,
            error: Some(error),
        }
    }

    /// Create a notification message (no response expected)
    pub fn notification(method: String, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some(method),
            params,
            result: None,
            error: None,
        }
    }

    /// Check if this is a request message
    pub fn is_request(&self) -> bool {
        self.method.is_some() && self.id.is_some()
    }

    /// Check if this is a notification message
    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }

    /// Check if this is a response message
    pub fn is_response(&self) -> bool {
        self.method.is_none() && (self.result.is_some() || self.error.is_some())
    }
}

impl JsonRpcError {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn parse_error() -> Self {
        Self::new(error_codes::PARSE_ERROR, "Parse error".to_string())
    }

    pub fn invalid_request() -> Self {
        Self::new(error_codes::INVALID_REQUEST, "Invalid Request".to_string())
    }

    pub fn method_not_found() -> Self {
        Self::new(error_codes::METHOD_NOT_FOUND, "Method not found".to_string())
    }

    pub fn invalid_params() -> Self {
        Self::new(error_codes::INVALID_PARAMS, "Invalid params".to_string())
    }

    pub fn internal_error() -> Self {
        Self::new(error_codes::INTERNAL_ERROR, "Internal error".to_string())
    }

    pub fn capability_not_found(capability_id: &str) -> Self {
        Self::new(
            error_codes::CAPABILITY_NOT_FOUND,
            format!("Capability '{}' not found", capability_id),
        )
    }

    pub fn capability_error(message: String) -> Self {
        Self::new(error_codes::CAPABILITY_ERROR, message)
    }

    pub fn authentication_error() -> Self {
        Self::new(
            error_codes::AUTHENTICATION_ERROR,
            "Authentication required".to_string(),
        )
    }

    pub fn authorization_error() -> Self {
        Self::new(
            error_codes::AUTHORIZATION_ERROR,
            "Not authorized".to_string(),
        )
    }

    pub fn rate_limit_error() -> Self {
        Self::new(
            error_codes::RATE_LIMIT_ERROR,
            "Rate limit exceeded".to_string(),
        )
    }
}

impl A2ARequest {
    pub fn new(capability_id: String, input: Value) -> Self {
        Self {
            capability_id,
            input,
            metadata: None,
            streaming: None,
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.streaming = Some(streaming);
        self
    }
}

impl A2AResponse {
    pub fn new(output: Value) -> Self {
        Self {
            output,
            metadata: None,
            partial: None,
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn partial(mut self) -> Self {
        self.partial = Some(true);
        self
    }
}

impl A2AProtocol {
    /// Create a new A2A Protocol instance
    pub async fn new(config: A2AConfig) -> Result<Self> {
        // Create agent card
        let agent_card = AgentCard::new(
            config.agent_id.clone(),
            config.agent_name.clone(),
            config.agent_version.clone(),
            config.base_url.clone(),
        );

        // Initialize discovery and communication
        let discovery = AgentDiscovery::new(config.discovery.clone());
        let communication = A2ACommunication::new(config.communication.clone())?;

        let protocol = Self {
            agent_card: Arc::new(RwLock::new(agent_card)),
            discovery,
            communication,
            config,
        };

        // Auto-register if configured
        if protocol.config.auto_register {
            protocol.register_self().await?;
        }

        Ok(protocol)
    }

    /// Get this agent's card
    pub async fn get_agent_card(&self) -> AgentCard {
        self.agent_card.read().await.clone()
    }

    /// Update this agent's capabilities
    pub async fn update_capabilities(&self, capabilities: Vec<Capability>) -> Result<()> {
        {
            let mut card = self.agent_card.write().await;
            card.capabilities = capabilities;
        }

        // Re-register with updated capabilities
        if self.config.auto_register {
            self.register_self().await?;
        }

        Ok(())
    }

    /// Add a capability to this agent
    pub async fn add_capability(&self, capability: Capability) -> Result<()> {
        {
            let mut card = self.agent_card.write().await;
            card.capabilities.push(capability);
        }

        // Re-register with updated capabilities
        if self.config.auto_register {
            self.register_self().await?;
        }

        Ok(())
    }

    /// Discover other agents
    pub async fn discover_agents(&self, request: DiscoveryRequest) -> Result<Vec<AgentCard>> {
        let response = self.discovery.discover_agents(request).await?;
        Ok(response.agents)
    }

    /// Find agents with specific capabilities
    pub async fn find_agents_with_capability(&self, capability_id: &str) -> Result<Vec<AgentCard>> {
        let request = DiscoveryRequest {
            query: None,
            capabilities: Some(vec![capability_id.to_string()]),
            tags: None,
            limit: None,
        };

        self.discover_agents(request).await
    }

    /// Invoke a capability on another agent
    pub async fn invoke_capability(
        &self,
        target_agent_id: &str,
        capability_id: &str,
        input: Value,
        streaming: bool,
    ) -> Result<A2AResponse> {
        // Get target agent info
        let target_agent = self.discovery.get_agent(target_agent_id).await?
            .ok_or_else(|| anyhow!("Agent '{}' not found", target_agent_id))?;

        // Create communication request
        let request = CommunicationRequest {
            target_agent,
            request: A2ARequest::new(capability_id.to_string(), input).with_streaming(streaming),
            mode: if streaming {
                CommunicationMode::Streaming
            } else {
                CommunicationMode::Synchronous
            },
            headers: None,
        };

        // Send request
        let response = self.communication.send_request(request).await?;
        Ok(response.response)
    }

    /// Check if an agent is available
    pub async fn ping_agent(&self, agent_id: &str) -> Result<bool> {
        let agent = self.discovery.get_agent(agent_id).await?
            .ok_or_else(|| anyhow!("Agent '{}' not found", agent_id))?;

        self.communication.ping_agent(&agent).await
    }

    /// Register this agent with discovery services
    async fn register_self(&self) -> Result<()> {
        let card = self.agent_card.read().await;
        self.discovery.register_agent(&card).await
    }

    /// Unregister this agent from discovery services
    pub async fn unregister_self(&self) -> Result<()> {
        self.discovery.unregister_agent(&self.config.agent_id).await
    }

    /// Clean up resources
    pub async fn shutdown(&self) -> Result<()> {
        // Unregister from discovery
        self.unregister_self().await?;
        
        // Clean up connections
        self.communication.cleanup_connections().await;
        
        Ok(())
    }
}

impl From<anyhow::Error> for JsonRpcError {
    fn from(error: anyhow::Error) -> Self {
        JsonRpcError::internal_error().with_data(serde_json::json!({
            "details": error.to_string()
        }))
    }
}

/// Utility functions for A2A protocol
pub mod utils {
    use super::*;

    /// Validate that a JSON-RPC message is properly formatted
    pub fn validate_jsonrpc_message(msg: &JsonRpcMessage) -> Result<()> {
        if msg.jsonrpc != "2.0" {
            return Err(anyhow!("Invalid JSON-RPC version: {}", msg.jsonrpc));
        }

        // Request validation
        if msg.method.is_some() {
            if msg.result.is_some() || msg.error.is_some() {
                return Err(anyhow!("Request message cannot have result or error"));
            }
        }

        // Response validation
        if msg.method.is_none() {
            if msg.result.is_none() && msg.error.is_none() {
                return Err(anyhow!("Response message must have result or error"));
            }
            if msg.result.is_some() && msg.error.is_some() {
                return Err(anyhow!("Response message cannot have both result and error"));
            }
        }

        Ok(())
    }

    /// Generate a unique request ID
    pub fn generate_request_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonrpc_request() {
        let msg = JsonRpcMessage::request(
            json!("123"),
            "invoke_capability".to_string(),
            Some(json!({"capability_id": "search", "input": {"query": "test"}})),
        );

        assert!(msg.is_request());
        assert!(!msg.is_response());
        assert!(!msg.is_notification());
        assert_eq!(msg.method.unwrap(), "invoke_capability");
    }

    #[test]
    fn test_jsonrpc_response() {
        let msg = JsonRpcMessage::response(json!("123"), json!({"output": "result"}));

        assert!(!msg.is_request());
        assert!(msg.is_response());
        assert!(!msg.is_notification());
        assert_eq!(msg.result, Some(json!({"output": "result"})));
    }

    #[test]
    fn test_jsonrpc_notification() {
        let msg = JsonRpcMessage::notification(
            "agent_status_changed".to_string(),
            Some(json!({"status": "online"})),
        );

        assert!(!msg.is_request());
        assert!(!msg.is_response());
        assert!(msg.is_notification());
        assert_eq!(msg.method.unwrap(), "agent_status_changed");
    }

    #[test]
    fn test_a2a_request() {
        let request = A2ARequest::new("search".to_string(), json!({"query": "test"}))
            .with_streaming(true);

        assert_eq!(request.capability_id, "search");
        assert_eq!(request.streaming, Some(true));
    }

    #[test]
    fn test_error_codes() {
        let error = JsonRpcError::capability_not_found("missing_cap");
        assert_eq!(error.code, error_codes::CAPABILITY_NOT_FOUND);
        assert!(error.message.contains("missing_cap"));
    }

    #[test]
    fn test_message_validation() {
        let valid_request = JsonRpcMessage::request(
            json!("1"),
            "test".to_string(),
            None,
        );
        assert!(utils::validate_jsonrpc_message(&valid_request).is_ok());

        let invalid_msg = JsonRpcMessage {
            jsonrpc: "1.0".to_string(), // Wrong version
            id: Some(json!("1")),
            method: Some("test".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(utils::validate_jsonrpc_message(&invalid_msg).is_err());
    }
}