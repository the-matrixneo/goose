use crate::agents::a2a::agent_card::{AgentCard, AuthInfo};
use crate::agents::a2a::protocol::{
    A2ARequest, A2AResponse, JsonRpcMessage, utils,
};
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use url::Url;

/// A2A Communication manager for handling agent-to-agent communication
pub struct A2ACommunication {
    /// HTTP client for standard requests
    http_client: Client,
    /// WebSocket connections for streaming
    ws_connections: Arc<RwLock<HashMap<String, WebSocketConnection>>>,
    /// Configuration for communication
    config: CommunicationConfig,
}

/// Configuration for A2A communication
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Default timeout for requests
    pub timeout: Duration,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Enable automatic reconnection for WebSockets
    pub auto_reconnect: bool,
    /// Connection keep-alive interval
    pub keep_alive_interval: Duration,
}

/// WebSocket connection wrapper
struct WebSocketConnection {
    stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    agent_id: String,
    last_used: std::time::SystemTime,
}

/// Communication mode for A2A requests
#[derive(Debug, Clone)]
pub enum CommunicationMode {
    /// Synchronous HTTP request/response
    Synchronous,
    /// Asynchronous HTTP with callback
    Asynchronous { callback_url: Url },
    /// Streaming via WebSocket
    Streaming,
}

/// A2A communication request
#[derive(Debug, Clone)]
pub struct CommunicationRequest {
    /// Target agent information
    pub target_agent: AgentCard,
    /// A2A request payload
    pub request: A2ARequest,
    /// Communication mode
    pub mode: CommunicationMode,
    /// Additional headers for HTTP requests
    pub headers: Option<HeaderMap>,
}

/// A2A communication response
#[derive(Debug, Clone)]
pub struct CommunicationResponse {
    /// A2A response payload
    pub response: A2AResponse,
    /// Response metadata
    pub metadata: HashMap<String, Value>,
}

/// Streaming response handler
pub struct StreamingResponse {
    connection: WebSocketConnection,
    request_id: String,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            auto_reconnect: true,
            keep_alive_interval: Duration::from_secs(30),
        }
    }
}

impl A2ACommunication {
    /// Create a new A2A communication manager
    pub fn new(config: CommunicationConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .build()?;

        Ok(Self {
            http_client,
            ws_connections: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Send a request to another agent
    pub async fn send_request(
        &self, 
        request: CommunicationRequest
    ) -> Result<CommunicationResponse> {
        match request.mode.clone() {
            CommunicationMode::Synchronous => {
                self.send_synchronous_request(request).await
            }
            CommunicationMode::Asynchronous { callback_url } => {
                self.send_asynchronous_request(request, callback_url).await
            }
            CommunicationMode::Streaming => {
                Err(anyhow!("Use send_streaming_request for streaming mode"))
            }
        }
    }

    /// Send a streaming request to another agent
    pub async fn send_streaming_request(
        &self,
        request: CommunicationRequest,
    ) -> Result<StreamingResponse> {
        let _agent_id = request.target_agent.id.clone();
        
        // Get or create WebSocket connection
        let mut connection = self.get_or_create_ws_connection(&request.target_agent).await?;
        
        // Send the request
        let request_id = utils::generate_request_id();
        let jsonrpc_request = JsonRpcMessage::request(
            serde_json::json!(request_id),
            "invoke_capability".to_string(),
            Some(serde_json::to_value(&request.request)?),
        );

        let message = WsMessage::Text(serde_json::to_string(&jsonrpc_request)?);
        connection.stream.send(message).await?;

        Ok(StreamingResponse {
            connection,
            request_id,
        })
    }

    /// Invoke a specific capability on a target agent
    pub async fn invoke_capability(
        &self,
        target_agent: &AgentCard,
        capability_id: &str,
        input: Value,
        streaming: bool,
    ) -> Result<CommunicationResponse> {
        let request = CommunicationRequest {
            target_agent: target_agent.clone(),
            request: A2ARequest::new(capability_id.to_string(), input),
            mode: if streaming {
                CommunicationMode::Streaming
            } else {
                CommunicationMode::Synchronous
            },
            headers: None,
        };

        if streaming {
            return Err(anyhow!("Use send_streaming_request for streaming capabilities"));
        }

        self.send_request(request).await
    }

    /// Check agent health/availability
    pub async fn ping_agent(&self, agent: &AgentCard) -> Result<bool> {
        let ping_url = agent.connection.base_url.join("v1/ping")?;
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        // Add authentication if required
        if let Some(auth) = &agent.connection.auth {
            self.add_auth_header(&mut headers, auth)?;
        }

        let response = self.http_client
            .get(ping_url)
            .headers(headers)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// Get agent capabilities dynamically
    pub async fn get_agent_capabilities(&self, agent: &AgentCard) -> Result<Vec<crate::agents::a2a::agent_card::Capability>> {
        let capabilities_url = agent.connection.base_url.join("v1/capabilities")?;
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        if let Some(auth) = &agent.connection.auth {
            self.add_auth_header(&mut headers, auth)?;
        }

        let response = self.http_client
            .get(capabilities_url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get capabilities: {}", response.status()));
        }

        let capabilities: Vec<crate::agents::a2a::agent_card::Capability> = response.json().await?;
        Ok(capabilities)
    }

    /// Close connection to an agent
    pub async fn close_connection(&self, agent_id: &str) -> Result<()> {
        let mut connections = self.ws_connections.write().await;
        if let Some(mut connection) = connections.remove(agent_id) {
            let _ = connection.stream.close(None).await;
        }
        Ok(())
    }

    /// Clean up idle connections
    pub async fn cleanup_connections(&self) {
        let mut connections = self.ws_connections.write().await;
        let now = std::time::SystemTime::now();
        
        connections.retain(|_id, connection| {
            now.duration_since(connection.last_used)
                .map(|age| age < self.config.keep_alive_interval * 2)
                .unwrap_or(false)
        });
    }

    /// Send synchronous HTTP request
    async fn send_synchronous_request(
        &self,
        request: CommunicationRequest,
    ) -> Result<CommunicationResponse> {
        let invoke_url = request.target_agent.connection.base_url.join("v1/invoke")?;
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        // Add custom headers
        if let Some(custom_headers) = &request.headers {
            headers.extend(custom_headers.clone());
        }
        
        // Add authentication if required
        if let Some(auth) = &request.target_agent.connection.auth {
            self.add_auth_header(&mut headers, auth)?;
        }

        let request_id = utils::generate_request_id();
        let jsonrpc_request = JsonRpcMessage::request(
            serde_json::json!(request_id),
            "invoke_capability".to_string(),
            Some(serde_json::to_value(&request.request)?),
        );

        let response = self.http_client
            .post(invoke_url)
            .headers(headers)
            .json(&jsonrpc_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {}", response.status()));
        }

        let jsonrpc_response: JsonRpcMessage = response.json().await?;
        
        if let Some(error) = jsonrpc_response.error {
            return Err(anyhow!("A2A Error: {} - {}", error.code, error.message));
        }

        let result = jsonrpc_response.result
            .ok_or_else(|| anyhow!("No result in response"))?;
        
        let a2a_response: A2AResponse = serde_json::from_value(result)?;
        
        Ok(CommunicationResponse {
            response: a2a_response,
            metadata: HashMap::new(),
        })
    }

    /// Send asynchronous HTTP request
    async fn send_asynchronous_request(
        &self,
        request: CommunicationRequest,
        _callback_url: Url,
    ) -> Result<CommunicationResponse> {
        // For now, treat as synchronous - in a full implementation,
        // this would set up callback handling
        self.send_synchronous_request(request).await
    }

    /// Get or create WebSocket connection
    async fn get_or_create_ws_connection(
        &self,
        agent: &AgentCard,
    ) -> Result<WebSocketConnection> {
        let agent_id = &agent.id;
        
        // Check existing connections
        {
            let mut connections = self.ws_connections.write().await;
            if let Some(connection) = connections.get_mut(agent_id) {
                connection.last_used = std::time::SystemTime::now();
                // Note: In real implementation, we'd need to handle moving the connection out
                // This is a simplified version
            }
        }

        // Create new connection
        let ws_url = agent.connection.base_url.join("v1/ws")?;
        let ws_url_str = ws_url.to_string().replace("http", "ws");
        let ws_url = Url::parse(&ws_url_str)?;

        let (ws_stream, _) = connect_async(ws_url.as_str()).await?;
        
        let connection = WebSocketConnection {
            stream: ws_stream,
            agent_id: agent_id.clone(),
            last_used: std::time::SystemTime::now(),
        };

        // Store connection
        {
            let mut connections = self.ws_connections.write().await;
            connections.insert(agent_id.clone(), connection);
        }

        // Return a new connection (simplified for this example)
        let (ws_stream, _) = connect_async(&ws_url_str).await?;
        Ok(WebSocketConnection {
            stream: ws_stream,
            agent_id: agent_id.clone(),
            last_used: std::time::SystemTime::now(),
        })
    }

    /// Add authentication header
    fn add_auth_header(&self, headers: &mut HeaderMap, auth: &AuthInfo) -> Result<()> {
        match auth.auth_type.as_str() {
            "bearer" => {
                if let Some(token) = auth.credentials.get("token") {
                    let header_value = HeaderValue::from_str(&format!("Bearer {}", token))?;
                    headers.insert(AUTHORIZATION, header_value);
                }
            }
            "api_key" => {
                if let Some(key) = auth.credentials.get("key") {
                    let header_value = HeaderValue::from_str(key)?;
                    headers.insert("X-API-Key", header_value);
                }
            }
            _ => {
                return Err(anyhow!("Unsupported auth type: {}", auth.auth_type));
            }
        }
        Ok(())
    }
}

impl StreamingResponse {
    /// Receive next response from the stream
    pub async fn next_response(&mut self) -> Result<Option<A2AResponse>> {
        while let Some(message) = self.connection.stream.next().await {
            match message? {
                WsMessage::Text(text) => {
                    let jsonrpc_msg: JsonRpcMessage = serde_json::from_str(&text)?;
                    
                    // Check if this is our response
                    if let Some(id) = &jsonrpc_msg.id {
                        if id.as_str() == Some(&self.request_id) {
                            if let Some(error) = jsonrpc_msg.error {
                                return Err(anyhow!("Stream error: {} - {}", error.code, error.message));
                            }
                            
                            if let Some(result) = jsonrpc_msg.result {
                                let response: A2AResponse = serde_json::from_value(result)?;
                                return Ok(Some(response));
                            }
                        }
                    }
                }
                WsMessage::Close(_) => {
                    return Ok(None);
                }
                _ => {
                    // Ignore other message types
                }
            }
        }
        
        Ok(None)
    }

    /// Close the streaming connection
    pub async fn close(mut self) -> Result<()> {
        self.connection.stream.close(None).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::a2a::agent_card::Capability;

    #[test]
    fn test_communication_config_default() {
        let config = CommunicationConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_connections, 100);
        assert!(config.auto_reconnect);
    }

    #[tokio::test]
    async fn test_communication_creation() {
        let config = CommunicationConfig::default();
        let comm = A2ACommunication::new(config);
        assert!(comm.is_ok());
    }

    #[test]
    fn test_communication_request() {
        let agent_card = AgentCard::new(
            "test_agent".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            Url::parse("https://example.com").unwrap(),
        );

        let request = CommunicationRequest {
            target_agent: agent_card,
            request: A2ARequest::new("test_capability".to_string(), serde_json::json!({})),
            mode: CommunicationMode::Synchronous,
            headers: None,
        };

        assert_eq!(request.request.capability_id, "test_capability");
    }

    #[tokio::test]
    async fn test_ping_agent() {
        let config = CommunicationConfig::default();
        let comm = A2ACommunication::new(config).unwrap();
        
        let agent_card = AgentCard::new(
            "test_agent".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            Url::parse("https://httpbin.org").unwrap(),
        );

        // This will fail because httpbin.org doesn't have our A2A endpoint,
        // but it tests the basic HTTP functionality
        let result = comm.ping_agent(&agent_card).await;
        
        // We expect this to fail, but not panic
        assert!(result.is_err() || !result.unwrap());
    }
}