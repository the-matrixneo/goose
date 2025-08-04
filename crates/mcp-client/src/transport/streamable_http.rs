use crate::oauth::{authenticate_service, ServiceConfig};
use crate::transport::Error;
use async_trait::async_trait;
use eventsource_client::{Client, SSE};
use futures::TryStreamExt;
use mcp_core::protocol::{JsonRpcMessage, JsonRpcRequest};
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::Duration;
use tracing::{error, info, warn};
use url::Url;

use super::{serialize_and_send, Transport, TransportHandle};

// Default timeout for HTTP requests
const HTTP_TIMEOUT_SECS: u64 = 30;

/// The Streamable HTTP transport actor that handles:
/// - HTTP POST requests to send messages to the server
/// - Optional streaming responses for receiving multiple responses and server-initiated messages
/// - Session management with session IDs
pub struct StreamableHttpActor {
    /// Receives messages (requests/notifications) from the handle
    receiver: mpsc::Receiver<String>,
    /// Sends messages (responses) back to the handle
    sender: mpsc::Sender<JsonRpcMessage>,
    /// MCP endpoint URL
    mcp_endpoint: String,
    /// HTTP client for sending requests
    http_client: HttpClient,
    /// Optional session ID for stateful connections
    session_id: Arc<RwLock<Option<String>>>,
    /// Environment variables to set
    env: HashMap<String, String>,
    /// Custom headers to include in requests
    headers: HashMap<String, String>,
}

impl StreamableHttpActor {
    pub fn new(
        receiver: mpsc::Receiver<String>,
        sender: mpsc::Sender<JsonRpcMessage>,
        mcp_endpoint: String,
        session_id: Arc<RwLock<Option<String>>>,
        env: HashMap<String, String>,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            receiver,
            sender,
            mcp_endpoint,
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
                .build()
                .unwrap(),
            session_id,
            env,
            headers,
        }
    }

    /// Main entry point for the actor
    pub async fn run(mut self) {
        info!("Starting StreamableHttpActor for endpoint: {}", self.mcp_endpoint);
        
        // Set environment variables
        for (key, value) in &self.env {
            info!("Setting environment variable: {}={}", key, value);
            std::env::set_var(key, value);
        }

        // Handle outgoing messages
        while let Some(message_str) = self.receiver.recv().await {
            // Log message type without full content to reduce verbosity
        let msg_preview = if message_str.len() > 100 {
            format!("{}... ({} chars)", &message_str[..100], message_str.len())
        } else {
            message_str.clone()
        };
        info!("StreamableHttpActor received outgoing message: {}", msg_preview);
            if let Err(e) = self.handle_outgoing_message(message_str).await {
                error!("Error handling outgoing message: {}", e);
                break;
            }
        }

        info!("StreamableHttpActor shut down for endpoint: {}", self.mcp_endpoint);
    }

    /// Handle an outgoing message by sending it via HTTP POST
    async fn handle_outgoing_message(&mut self, message_str: String) -> Result<(), Error> {
        let message_start_time = std::time::Instant::now();
        
        // Extract method from message for better logging
        let method = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&message_str) {
            parsed.get("method")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown")
                .to_string()
        } else {
            "invalid-json".to_string()
        };
        
        info!("⏰ [MESSAGE] Starting to handle outgoing message for {}: method={} ({}ms since start)", 
              self.mcp_endpoint, method, message_start_time.elapsed().as_millis());

        // Parse the message to determine if it's a request that expects a response
        let parsed_message: JsonRpcMessage =
            serde_json::from_str(&message_str).map_err(Error::Serialization)?;

        let expects_response = matches!(
            parsed_message,
            JsonRpcMessage::Request(JsonRpcRequest { id: Some(_), .. })
        );

        // Try to send the request
        let http_send_start = std::time::Instant::now();
        info!("🚀 [MESSAGE] Attempting to send HTTP request to {} for method={} ({}ms elapsed)", 
              self.mcp_endpoint, method, message_start_time.elapsed().as_millis());
        match self.send_request(&message_str, expects_response).await {
            Ok(()) => {
                let total_elapsed = message_start_time.elapsed();
                let http_elapsed = http_send_start.elapsed();
                info!("✅ [MESSAGE] Successfully sent request to {} for method={} (HTTP: {}ms, Total: {}ms)", 
                      self.mcp_endpoint, method, http_elapsed.as_millis(), total_elapsed.as_millis());
                
                // Log timing anomalies
                if total_elapsed.as_millis() > 1000 {
                    warn!("⚠️ [TIMING] Slow message processing detected! method={}, total={}ms, http={}ms", 
                          method, total_elapsed.as_millis(), http_elapsed.as_millis());
                }
                Ok(())
            },
            Err(Error::HttpError { status, .. }) if status == 401 || status == 403 => {
                // Authentication challenge - try to authenticate and retry
                let auth_start = std::time::Instant::now();
                info!(
                    "🔐 [AUTH] Received authentication challenge ({}) from {} for method={}, attempting OAuth flow... ({}ms elapsed)",
                    status, self.mcp_endpoint, method, message_start_time.elapsed().as_millis()
                );

                if let Some(token) = self.attempt_authentication().await? {
                    let auth_elapsed = auth_start.elapsed();
                    info!("🔐 [AUTH] Authentication successful for {} in {}ms, retrying request for method={}...", 
                          self.mcp_endpoint, auth_elapsed.as_millis(), method);
                    self.headers
                        .insert("Authorization".to_string(), format!("Bearer {}", token));
                    
                    let retry_start = std::time::Instant::now();
                    let result = self.send_request(&message_str, expects_response).await;
                    let retry_elapsed = retry_start.elapsed();
                    let total_elapsed = message_start_time.elapsed();
                    
                    info!("🔐 [AUTH] Retry completed for method={}: retry={}ms, auth={}ms, total={}ms", 
                          method, retry_elapsed.as_millis(), auth_elapsed.as_millis(), total_elapsed.as_millis());
                    result
                } else {
                    let auth_elapsed = auth_start.elapsed();
                    error!("🔐 [AUTH] Authentication failed for {} after {}ms - service not supported or OAuth flow failed", 
                           self.mcp_endpoint, auth_elapsed.as_millis());
                    Err(Error::StreamableHttpError(
                        "Authentication failed - service not supported or OAuth flow failed"
                            .to_string(),
                    ))
                }
            }
            Err(e) => {
                let total_elapsed = message_start_time.elapsed();
                error!("❌ [MESSAGE] Failed to send request to {} for method={} after {}ms: {}", 
                       self.mcp_endpoint, method, total_elapsed.as_millis(), e);
                Err(e)
            },
        }
    }

    /// Send an HTTP request to the MCP endpoint
    async fn send_request(
        &mut self,
        message_str: &str,
        expects_response: bool,
    ) -> Result<(), Error> {
        info!("Building HTTP POST request to {}, expects_response: {}", self.mcp_endpoint, expects_response);
        
        // Build the HTTP request
        let mut request = self
            .http_client
            .post(&self.mcp_endpoint)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", "2025-06-18") // Required protocol version header
            .body(message_str.to_string());

        // Add session ID header if we have one
        let session_check_time = std::time::Instant::now();
        if let Some(session_id) = self.session_id.read().await.as_ref() {
            info!("🔑 [SESSION] Adding session ID header: {} (acquired in {}ms)", 
                  session_id, session_check_time.elapsed().as_millis());
            request = request.header("Mcp-Session-Id", session_id);
        } else {
            info!("⚠️ [SESSION] No session ID available, sending without session header (checked in {}ms)", 
                  session_check_time.elapsed().as_millis());
        }

        // Add custom headers
        for (key, value) in &self.headers {
            info!("Adding custom header: {}={}", key, value);
            request = request.header(key, value);
        }

        // Send the request
        info!("Sending HTTP request to {}", self.mcp_endpoint);
        let response = request
            .send()
            .await
            .map_err(|e| {
                error!("HTTP request to {} failed: {}", self.mcp_endpoint, e);
                Error::StreamableHttpError(format!("HTTP request failed: {}", e))
            })?;
        
        info!("Received HTTP response from {} with status: {}", self.mcp_endpoint, response.status());

        // Handle HTTP error status codes
        if !response.status().is_success() {
            let status = response.status();
            if status.as_u16() == 404 {
                // Session not found - clear our session ID
                *self.session_id.write().await = None;
                return Err(Error::SessionError(
                    "Session expired or not found".to_string(),
                ));
            }
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::HttpError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Check for session ID in response headers
        let session_timing = std::time::Instant::now();
        if let Some(session_id_header) = response.headers().get("Mcp-Session-Id") {
            if let Ok(session_id) = session_id_header.to_str() {
                let existing_session = self.session_id.read().await.clone();
                if existing_session.is_none() {
                    info!("🆆 [SESSION] First session ID received from {}: {} ({}ms after request)", 
                          self.mcp_endpoint, session_id, session_timing.elapsed().as_millis());
                } else if existing_session.as_deref() != Some(session_id) {
                    info!("🔄 [SESSION] Session ID changed from {:?} to {} for {} ({}ms after request)", 
                          existing_session, session_id, self.mcp_endpoint, session_timing.elapsed().as_millis());
                } else {
                    info!("✅ [SESSION] Session ID confirmed: {} for {} ({}ms after request)", 
                          session_id, self.mcp_endpoint, session_timing.elapsed().as_millis());
                }
                *self.session_id.write().await = Some(session_id.to_string());
            }
        } else {
            let existing_session = self.session_id.read().await.clone();
            if existing_session.is_some() {
                info!("⚠️ [SESSION] No session ID in response headers from {} (had: {:?})", 
                      self.mcp_endpoint, existing_session);
            } else {
                info!("🛡️ [SESSION] No session ID in response headers from {} (none expected)", self.mcp_endpoint);
            }
        }

        // Handle the response based on content type
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        
        let response_analysis_time = std::time::Instant::now();
        info!("📊 [RESPONSE] Analyzing response type for {}: content-type='{}'", self.mcp_endpoint, content_type);
        info!("📊 [RESPONSE] Full response headers: {:?}", response.headers());
        info!("📊 [RESPONSE] Response status: {}, expects_response: {}", response.status(), expects_response);

        if content_type.starts_with("text/event-stream") {
            // Handle streaming HTTP response (server chose to stream multiple messages back)
            info!("🌊 [RESPONSE] Handling as STREAMING response from {}, content-type: {}", self.mcp_endpoint, content_type);
            if expects_response {
                info!("🌊 [RESPONSE] Processing streaming response (expects_response=true)");
                self.handle_streaming_response(response).await?;
            } else {
                info!("⚠️ [RESPONSE] Ignoring streaming response (expects_response=false)");
            }
        } else if content_type.starts_with("application/json") || expects_response {
            // Handle single JSON response
            info!("📋 [RESPONSE] Handling as JSON response from {}, content-type: '{}' (fallback: expects_response={})", 
                  self.mcp_endpoint, content_type, expects_response);
            let response_text = response.text().await.map_err(|e| {
                Error::StreamableHttpError(format!("Failed to read response: {}", e))
            })?;

            if !response_text.is_empty() {
                info!("📋 [RESPONSE] Parsing JSON response from {} ({} chars): {}", 
                      self.mcp_endpoint, response_text.len(), 
                      if response_text.len() > 300 { &response_text[..300] } else { &response_text });
                      
                let json_message: JsonRpcMessage =
                    serde_json::from_str(&response_text).map_err(|e| {
                        error!("📋 [RESPONSE] JSON parsing failed for {}: {}. Raw text: {}", 
                               self.mcp_endpoint, e, response_text);
                        Error::Serialization(e)
                    })?;

                // Special logging for tools responses
                if let JsonRpcMessage::Response(ref resp) = json_message {
                    if let Some(result) = &resp.result {
                        if let Some(tools) = result.get("tools") {
                            if let Some(tools_array) = tools.as_array() {
                                info!("🔧 [RESPONSE] JSON tools response: {} tools found", tools_array.len());
                                if tools_array.is_empty() {
                                    warn!("⚠️ [RESPONSE] Empty tools array in JSON response! Full result: {}", result);
                                }
                            }
                        }
                    }
                }

                info!("📋 [RESPONSE] Forwarding parsed JSON message from {} to client", self.mcp_endpoint);
                let _ = self.sender.send(json_message).await;
            } else {
                warn!("⚠️ [RESPONSE] Received empty JSON response body from {}", self.mcp_endpoint);
            }
        } else {
            info!("🚫 [RESPONSE] Ignoring response from {} (content-type: '{}', expects_response: {})", 
                  self.mcp_endpoint, content_type, expects_response);
        }
        
        let response_processing_time = response_analysis_time.elapsed();
        info!("✅ [RESPONSE] Response processing completed in {}ms for {}", 
              response_processing_time.as_millis(), self.mcp_endpoint);
        // For notifications and responses, we get 202 Accepted with no body

        Ok(())
    }

    /// Attempt to authenticate with the service
    async fn attempt_authentication(&self) -> Result<Option<String>, Error> {
        info!("Attempting to authenticate with service at: {}", self.mcp_endpoint);

        // Create a generic OAuth configuration from the MCP endpoint
        info!("Creating OAuth configuration from MCP endpoint: {}", self.mcp_endpoint);
        match ServiceConfig::from_mcp_endpoint(&self.mcp_endpoint) {
            Ok(config) => {
                info!("Created OAuth config for endpoint {}: oauth_host={}, redirect_uri={}", 
                      self.mcp_endpoint, config.oauth_host, config.redirect_uri);

                info!("Starting OAuth authentication flow for {}", self.mcp_endpoint);
                match authenticate_service(config, &self.mcp_endpoint).await {
                    Ok(token) => {
                        info!("OAuth authentication successful for {}! Token length: {}", 
                              self.mcp_endpoint, token.len());
                        Ok(Some(token))
                    }
                    Err(e) => {
                        warn!("OAuth authentication failed for {}: {}", self.mcp_endpoint, e);
                        Err(Error::StreamableHttpError(format!("OAuth failed: {}", e)))
                    }
                }
            }
            Err(e) => {
                warn!(
                    "Could not create OAuth config from MCP endpoint {}: {}",
                    self.mcp_endpoint, e
                );
                Ok(None)
            }
        }
    }

    /// Handle streaming HTTP response that uses Server-Sent Events format
    ///
    /// This is called when the server responds to an HTTP POST with `text/event-stream`
    /// content-type, indicating it wants to stream multiple JSON-RPC messages back
    /// rather than sending a single response. This is part of the Streamable HTTP
    /// specification, not a separate SSE transport.
    async fn handle_streaming_response(
        &mut self,
        response: reqwest::Response,
    ) -> Result<(), Error> {
        use futures::StreamExt;
        use tokio::io::AsyncBufReadExt;
        use tokio_util::io::StreamReader;
        
        let start_time = std::time::Instant::now();
        info!("🌊 [STREAMING] Starting streaming response handler for {}", self.mcp_endpoint);
        info!("🌊 [STREAMING] Response headers: {:?}", response.headers());
        
        // Convert the response body to a stream reader
        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(std::io::Error::other));
        info!("🌊 [STREAMING] Created byte stream for {}", self.mcp_endpoint);
        let reader = StreamReader::new(stream);
        let mut lines = tokio::io::BufReader::new(reader).lines();

        let mut event_type = String::new();
        let mut event_data = String::new();
        let mut event_id = String::new();
        let mut bytes_received = 0usize;
        let mut lines_processed = 0usize;
        let mut events_processed = 0usize;
        
        info!("🌊 [STREAMING] Starting to read SSE lines from {}", self.mcp_endpoint);

        while let Ok(Some(line)) = lines.next_line().await {
            lines_processed += 1;
            bytes_received += line.len() + 1; // +1 for newline
            let elapsed = start_time.elapsed();
            
            if lines_processed % 10 == 1 || !line.is_empty() { // Log every 10th line or non-empty lines
                info!("🌊 [STREAMING] Line {}: '{}' (total {} bytes, {}ms elapsed)", 
                      lines_processed, line, bytes_received, elapsed.as_millis());
            }
            if line.is_empty() {
                // Empty line indicates end of event
                events_processed += 1;
                let elapsed = start_time.elapsed();
                
                if !event_data.is_empty() {
                    info!("🌊 [STREAMING] Event {} complete after {}ms. Data length: {} chars. Data preview: {}", 
                          events_processed, elapsed.as_millis(), event_data.len(), 
                          if event_data.len() > 200 { &event_data[..200] } else { &event_data });
                    
                    // Parse the streamed data as JSON-RPC message
                    match serde_json::from_str::<JsonRpcMessage>(&event_data) {
                        Ok(message) => {
                            // Log message type without full content
                            let msg_type = match &message {
                                JsonRpcMessage::Request(req) => format!("Request({})", req.method),
                                JsonRpcMessage::Response(resp) => {
                                    // Special handling for tools/list responses to debug empty arrays
                                    if let Some(result) = &resp.result {
                                        if let Some(tools) = result.get("tools") {
                                            if let Some(tools_array) = tools.as_array() {
                                                format!("Response(id={}, tools_count={})", 
                                                       resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()), 
                                                       tools_array.len())
                                            } else {
                                                format!("Response(id={}, tools=non-array)", 
                                                       resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()))
                                            }
                                        } else {
                                            format!("Response(id={}, no_tools_field)", 
                                                   resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()))
                                        }
                                    } else {
                                        format!("Response(id={}, no_result)", 
                                               resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()))
                                    }
                                },
                                JsonRpcMessage::Error(err) => format!("Error(id={})", err.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string())),
                                JsonRpcMessage::Notification(notif) => format!("Notification({})", notif.method),
                                JsonRpcMessage::Nil => "Nil".to_string(),
                            };
                            info!("🌊 [STREAMING] Successfully parsed and sending: {}", msg_type);
                            let _ = self.sender.send(message).await;
                        }
                        Err(err) => {
                            warn!("🌊 [STREAMING] Failed to parse event data as JSON-RPC: {}. Raw data: '{}'", err, event_data);
                        }
                    }
                } else {
                    info!("🌊 [STREAMING] Empty event {} after {}ms (keepalive?)", events_processed, elapsed.as_millis());
                }
                // Reset for next event
                event_type.clear();
                event_data.clear();
                event_id.clear();
            } else if let Some(field_data) = line.strip_prefix("data: ") {
                if !event_data.is_empty() {
                    event_data.push('\n');
                }
                event_data.push_str(field_data);
                info!("🌊 [STREAMING] Added data field: '{}' (total event data length: {})", field_data, event_data.len());
            } else if let Some(field_data) = line.strip_prefix("event: ") {
                event_type = field_data.to_string();
                info!("🌊 [STREAMING] Set event type: '{}'", event_type);
            } else if let Some(field_data) = line.strip_prefix("id: ") {
                event_id = field_data.to_string();
                info!("🌊 [STREAMING] Set event ID: '{}'", event_id);
            } else if !line.trim().is_empty() {
                info!("🌊 [STREAMING] Unknown SSE field: '{}'", line);
            }
            // Ignore other fields (retry, etc.) - we only care about data
        }
        
        let total_elapsed = start_time.elapsed();
        info!("🌊 [STREAMING] Streaming completed for {} after {}ms. Total: {} lines, {} bytes, {} events", 
              self.mcp_endpoint, total_elapsed.as_millis(), lines_processed, bytes_received, events_processed);

        Ok(())
    }
}

#[derive(Clone)]
pub struct StreamableHttpTransportHandle {
    sender: mpsc::Sender<String>,
    receiver: Arc<Mutex<mpsc::Receiver<JsonRpcMessage>>>,
    session_id: Arc<RwLock<Option<String>>>,
    mcp_endpoint: String,
    http_client: HttpClient,
    headers: HashMap<String, String>,
}

#[async_trait::async_trait]
impl TransportHandle for StreamableHttpTransportHandle {
    async fn send(&self, message: JsonRpcMessage) -> Result<(), Error> {
        serialize_and_send(&self.sender, message).await
    }

    async fn receive(&self) -> Result<JsonRpcMessage, Error> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await.ok_or(Error::ChannelClosed)
    }
}

impl StreamableHttpTransportHandle {
    /// Manually terminate the session by sending HTTP DELETE
    pub async fn terminate_session(&self) -> Result<(), Error> {
        if let Some(session_id) = self.session_id.read().await.as_ref() {
            let mut request = self
                .http_client
                .delete(&self.mcp_endpoint)
                .header("Mcp-Session-Id", session_id)
                .header("MCP-Protocol-Version", "2025-06-18"); // Required protocol version header

            // Add custom headers
            for (key, value) in &self.headers {
                request = request.header(key, value);
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().as_u16() == 405 {
                        // Method not allowed - server doesn't support session termination
                        info!("Server doesn't support session termination");
                    }
                }
                Err(e) => {
                    warn!("Failed to terminate session: {}", e);
                }
            }
        }
        Ok(())
    }

    /// Create a GET request to establish a streaming connection for server-initiated messages
    pub async fn listen_for_server_messages(&self) -> Result<(), Error> {
        let mut request = self
            .http_client
            .get(&self.mcp_endpoint)
            .header("Accept", "text/event-stream")
            .header("MCP-Protocol-Version", "2025-06-18"); // Required protocol version header

        // Add session ID header if we have one
        if let Some(session_id) = self.session_id.read().await.as_ref() {
            request = request.header("Mcp-Session-Id", session_id);
        }

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await.map_err(|e| {
            Error::StreamableHttpError(format!("Failed to start GET streaming connection: {}", e))
        })?;

        if !response.status().is_success() {
            if response.status().as_u16() == 405 {
                // Method not allowed - server doesn't support GET streaming connections
                info!("Server doesn't support GET streaming connections");
                return Ok(());
            }
            return Err(Error::HttpError {
                status: response.status().as_u16(),
                message: "Failed to establish GET streaming connection".to_string(),
            });
        }

        // Handle the streaming connection in a separate task
        let receiver = self.receiver.clone();
        let url = response.url().clone();

        tokio::spawn(async move {
            let client = match eventsource_client::ClientBuilder::for_url(url.as_str()) {
                Ok(builder) => builder.build(),
                Err(e) => {
                    error!(
                        "Failed to create streaming client for GET connection: {}",
                        e
                    );
                    return;
                }
            };

            let mut stream = client.stream();
            while let Ok(Some(event)) = stream.try_next().await {
                match event {
                    SSE::Event(e) if e.event_type == "message" || e.event_type.is_empty() => {
                        match serde_json::from_str::<JsonRpcMessage>(&e.data) {
                            Ok(message) => {
                                // Log message type without full content
                                let msg_type = match &message {
                                    JsonRpcMessage::Request(req) => format!("Request({})", req.method),
                                    JsonRpcMessage::Response(resp) => format!("Response(id={})", resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string())),
                                    JsonRpcMessage::Error(err) => format!("Error(id={})", err.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string())),
                                    JsonRpcMessage::Notification(notif) => format!("Notification({})", notif.method),
                                    JsonRpcMessage::Nil => "Nil".to_string(),
                                };
                                info!("Received GET streaming message: {}", msg_type);
                                let receiver_guard = receiver.lock().await;
                                // We can't send through the receiver since it's for outbound messages
                                // This would need a different channel for server-initiated messages
                                drop(receiver_guard);
                            }
                            Err(err) => {
                                warn!("Failed to parse GET streaming message: {}", err);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}

#[derive(Clone)]
pub struct StreamableHttpTransport {
    mcp_endpoint: String,
    env: HashMap<String, String>,
    headers: HashMap<String, String>,
}

impl StreamableHttpTransport {
    pub fn new<S: Into<String>>(mcp_endpoint: S, env: HashMap<String, String>) -> Self {
        Self {
            mcp_endpoint: mcp_endpoint.into(),
            env,
            headers: HashMap::new(),
        }
    }

    pub fn with_headers<S: Into<String>>(
        mcp_endpoint: S,
        env: HashMap<String, String>,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            mcp_endpoint: mcp_endpoint.into(),
            env,
            headers,
        }
    }

    /// Validate that the URL is a valid MCP endpoint
    pub fn validate_endpoint(endpoint: &str) -> Result<(), Error> {
        Url::parse(endpoint)
            .map_err(|e| Error::StreamableHttpError(format!("Invalid MCP endpoint URL: {}", e)))?;
        Ok(())
    }
}

#[async_trait]
impl Transport for StreamableHttpTransport {
    type Handle = StreamableHttpTransportHandle;

    async fn start(&self) -> Result<Self::Handle, Error> {
        // Validate the endpoint URL
        Self::validate_endpoint(&self.mcp_endpoint)?;

        // Create channels for communication
        let (tx, rx) = mpsc::channel(32);
        let (otx, orx) = mpsc::channel(32);

        let session_id: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let session_id_clone = Arc::clone(&session_id);

        // Create and spawn the actor
        let actor = StreamableHttpActor::new(
            rx,
            otx,
            self.mcp_endpoint.clone(),
            session_id,
            self.env.clone(),
            self.headers.clone(),
        );

        tokio::spawn(actor.run());

        // Create the handle
        let handle = StreamableHttpTransportHandle {
            sender: tx,
            receiver: Arc::new(Mutex::new(orx)),
            session_id: session_id_clone,
            mcp_endpoint: self.mcp_endpoint.clone(),
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
                .build()
                .unwrap(),
            headers: self.headers.clone(),
        };

        Ok(handle)
    }

    async fn close(&self) -> Result<(), Error> {
        // The transport is closed when the actor task completes
        // No additional cleanup needed
        Ok(())
    }
}
