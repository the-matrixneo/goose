use rmcp::{
    model::{
        CallToolRequest, CallToolRequestParam, CallToolResult, ClientCapabilities, ClientInfo,
        ClientRequest, GetPromptRequest, GetPromptRequestParam, GetPromptResult, Implementation,
        InitializeResult, ListPromptsRequest, ListPromptsResult, ListResourcesRequest,
        ListResourcesResult, ListToolsRequest, ListToolsResult, LoggingMessageNotification,
        LoggingMessageNotificationMethod, PaginatedRequestParam, ProgressNotification,
        ProgressNotificationMethod, ProtocolVersion, ReadResourceRequest, ReadResourceRequestParam,
        ReadResourceResult, ServerNotification, ServerResult,
    },
    service::{ClientInitializeError, PeerRequestOptions, RunningService},
    transport::IntoTransport,
    ClientHandler, RoleClient, ServiceError, ServiceExt,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};

pub type BoxError = Box<dyn std::error::Error + Sync + Send>;

<<<<<<< Updated upstream
pub type Error = rmcp::ServiceError;
||||||| Stash base
/// Error type for MCP client operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(#[from] super::transport::Error),

    #[error("RPC error: code={code}, message={message}")]
    RpcError { code: i32, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unexpected response from server: {0}")]
    UnexpectedResponse(String),

    #[error("Not initialized")]
    NotInitialized,

    #[error("Timeout or service not ready")]
    NotReady,

    #[error("Request timed out")]
    Timeout(#[from] tower::timeout::error::Elapsed),

    #[error("Error from mcp-server: {0}")]
    ServerBoxError(BoxError),

    #[error("Call to '{server}' failed for '{method}'. {source}")]
    McpServerError {
        method: String,
        server: String,
        #[source]
        source: BoxError,
    },
}

// BoxError from mcp-server gets converted to our Error type
impl From<BoxError> for Error {
    fn from(err: BoxError) -> Self {
        Error::ServerBoxError(err)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ClientCapabilities {
    // Add fields as needed. For now, empty capabilities are fine.
}

#[derive(Serialize, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}
=======
/// Error type for MCP client operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(#[from] super::transport::Error),

    #[error("RPC error: code={code}, message={message}")]
    RpcError { code: i32, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unexpected response from server: {0}")]
    UnexpectedResponse(String),

    #[error("Not initialized")]
    NotInitialized,

    #[error("Timeout or service not ready")]
    NotReady,

    #[error("Request timed out")]
    Timeout(#[from] tower::timeout::error::Elapsed),

    #[error("Error from mcp-server: {0}")]
    ServerBoxError(BoxError),

    #[error("Call to '{server}' failed for '{method}'. {source}")]
    McpServerError {
        method: String,
        server: String,
        #[source]
        source: BoxError,
    },
}

// BoxError from mcp-server gets converted to our Error type
impl From<BoxError> for Error {
    fn from(err: BoxError) -> Self {
        Error::ServerBoxError(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ClientCapabilities {
    // Add fields as needed. For now, empty capabilities are fine.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}
>>>>>>> Stashed changes

#[async_trait::async_trait]
pub trait McpClientTrait: Send + Sync {
    async fn list_resources(
        &self,
        next_cursor: Option<String>,
    ) -> Result<ListResourcesResult, Error>;

    async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult, Error>;

    async fn list_tools(&self, next_cursor: Option<String>) -> Result<ListToolsResult, Error>;

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult, Error>;

    async fn list_prompts(&self, next_cursor: Option<String>) -> Result<ListPromptsResult, Error>;

    async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult, Error>;

    async fn subscribe(&self) -> mpsc::Receiver<ServerNotification>;

    fn get_info(&self) -> Option<&InitializeResult>;
}

pub struct GooseClient {
    notification_handlers: Arc<Mutex<Vec<Sender<ServerNotification>>>>,
}

impl GooseClient {
    pub fn new(handlers: Arc<Mutex<Vec<Sender<ServerNotification>>>>) -> Self {
        GooseClient {
            notification_handlers: handlers,
        }
    }
}

impl ClientHandler for GooseClient {
    async fn on_progress(
        &self,
        params: rmcp::model::ProgressNotificationParam,
        context: rmcp::service::NotificationContext<rmcp::RoleClient>,
    ) {
        self.notification_handlers
            .lock()
            .await
            .iter()
            .for_each(|handler| {
                let _ = handler.try_send(ServerNotification::ProgressNotification(
                    ProgressNotification {
                        params: params.clone(),
                        method: ProgressNotificationMethod,
                        extensions: context.extensions.clone(),
                    },
                ));
            });
    }

    async fn on_logging_message(
        &self,
        params: rmcp::model::LoggingMessageNotificationParam,
        context: rmcp::service::NotificationContext<rmcp::RoleClient>,
    ) {
        self.notification_handlers
            .lock()
            .await
            .iter()
            .for_each(|handler| {
                let _ = handler.try_send(ServerNotification::LoggingMessageNotification(
                    LoggingMessageNotification {
                        params: params.clone(),
                        method: LoggingMessageNotificationMethod,
                        extensions: context.extensions.clone(),
                    },
                ));
            });
    }

    fn get_info(&self) -> ClientInfo {
        ClientInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ClientCapabilities::builder().build(),
            client_info: Implementation {
                name: "goose".to_string(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            },
        }
    }
}

/// The MCP client is the interface for MCP operations.
pub struct McpClient {
    client: Mutex<RunningService<RoleClient, GooseClient>>,
    notification_subscribers: Arc<Mutex<Vec<mpsc::Sender<ServerNotification>>>>,
    server_info: Option<InitializeResult>,
    timeout: std::time::Duration,
}

<<<<<<< Updated upstream
impl McpClient {
    pub async fn connect<T, E, A>(
        transport: T,
        timeout: std::time::Duration,
    ) -> Result<Self, ClientInitializeError>
    where
        T: IntoTransport<RoleClient, E, A>,
        E: std::error::Error + From<std::io::Error> + Send + Sync + 'static,
    {
||||||| Stash base
impl<T> McpClient<T>
where
    T: TransportHandle + Send + Sync + 'static,
{
    pub async fn connect(transport: T, timeout: std::time::Duration) -> Result<Self, Error> {
        let service = McpService::new(transport.clone());
        let service_ptr = service.clone();
=======
impl<T> McpClient<T>
where
    T: TransportHandle + Send + Sync + 'static,
{
    pub async fn connect(transport: T, timeout: std::time::Duration) -> Result<Self, Error> {
        tracing::info!("Connecting MCP client with timeout: {:?}", timeout);
        let service = McpService::new(transport.clone());
        let service_ptr = service.clone();
>>>>>>> Stashed changes
        let notification_subscribers =
<<<<<<< Updated upstream
            Arc::new(Mutex::new(Vec::<mpsc::Sender<ServerNotification>>::new()));
||||||| Stash base
            Arc::new(Mutex::new(Vec::<mpsc::Sender<JsonRpcMessage>>::new()));
        let subscribers_ptr = notification_subscribers.clone();
=======
            Arc::new(Mutex::new(Vec::<mpsc::Sender<JsonRpcMessage>>::new()));
        let subscribers_ptr = notification_subscribers.clone();
        
        tracing::info!("Starting transport message receive loop");
>>>>>>> Stashed changes

<<<<<<< Updated upstream
        let client = GooseClient::new(notification_subscribers.clone());
        let client: rmcp::service::RunningService<rmcp::RoleClient, GooseClient> =
            client.serve(transport).await?;
        let server_info = client.peer_info().cloned();

||||||| Stash base
        tokio::spawn(async move {
            loop {
                match transport.receive().await {
                    Ok(message) => {
                        tracing::info!("Received message: {:?}", message);
                        match message {
                            JsonRpcMessage::Response(JsonRpcResponse { id: Some(id), .. })
                            | JsonRpcMessage::Error(JsonRpcError { id: Some(id), .. }) => {
                                service_ptr.respond(&id.to_string(), Ok(message)).await;
                            }
                            _ => {
                                let mut subs = subscribers_ptr.lock().await;
                                subs.retain(|sub| sub.try_send(message.clone()).is_ok());
                            }
                        }
                    }
                    Err(e) => {
                        service_ptr.hangup(e).await;
                        subscribers_ptr.lock().await.clear();
                        break;
                    }
                }
            }
        });

        let middleware = TimeoutLayer::new(timeout);

=======
        tokio::spawn(async move {
            loop {
                match transport.receive().await {
                    Ok(message) => {
                        // Log message type without full content to avoid spam
                        let msg_type = match &message {
                            JsonRpcMessage::Request(req) => format!("Request({})", req.method),
                            JsonRpcMessage::Response(resp) => format!("Response(id={})", resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string())),
                            JsonRpcMessage::Error(err) => format!("Error(id={})", err.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string())),
                            JsonRpcMessage::Notification(notif) => format!("Notification({})", notif.method),
                            JsonRpcMessage::Nil => "Nil".to_string(),
                        };
                        tracing::info!("MCP client received message from transport: {}", msg_type);
                        match message {
                            JsonRpcMessage::Response(JsonRpcResponse { id: Some(id), .. })
                            | JsonRpcMessage::Error(JsonRpcError { id: Some(id), .. }) => {
                                tracing::info!("Forwarding response/error message with ID {} to service", id);
                                service_ptr.respond(&id.to_string(), Ok(message)).await;
                            }
                            _ => {
                                tracing::info!("Broadcasting notification/server-initiated message to {} subscribers", subscribers_ptr.lock().await.len());
                                let mut subs = subscribers_ptr.lock().await;
                                let initial_count = subs.len();
                                subs.retain(|sub| sub.try_send(message.clone()).is_ok());
                                if subs.len() < initial_count {
                                    tracing::info!("Removed {} inactive subscribers", initial_count - subs.len());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Transport receive error, shutting down message loop: {}", e);
                        service_ptr.hangup(e).await;
                        subscribers_ptr.lock().await.clear();
                        break;
                    }
                }
            }
        });

        let middleware = TimeoutLayer::new(timeout);
        
        tracing::info!("MCP client connected successfully");
>>>>>>> Stashed changes
        Ok(Self {
            client: Mutex::new(client),
            notification_subscribers,
            server_info,
            timeout,
        })
    }

<<<<<<< Updated upstream
    fn get_request_options(&self) -> PeerRequestOptions {
        PeerRequestOptions {
            timeout: Some(self.timeout),
            meta: None,
||||||| Stash base
    /// Send a JSON-RPC request and check we don't get an error response.
    async fn send_request<R>(&self, method: &str, params: Value) -> Result<R, Error>
    where
        R: for<'de> Deserialize<'de>,
    {
        let mut service = self.service.lock().await;
        service.ready().await.map_err(|_| Error::NotReady)?;
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let mut params = params.clone();
        params["_meta"] = json!({
            "progressToken": format!("prog-{}", id),
        });

        let request = JsonRpcMessage::Request(JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: method.to_string(),
            params: Some(params),
        });

        let response_msg = service
            .call(request)
            .await
            .map_err(|e| Error::McpServerError {
                server: self
                    .server_info
                    .as_ref()
                    .map(|s| s.name.clone())
                    .unwrap_or("".to_string()),
                method: method.to_string(),
                // we don't need include params because it can be really large
                source: Box::<Error>::new(e.into()),
            })?;

        match response_msg {
            JsonRpcMessage::Response(JsonRpcResponse {
                id, result, error, ..
            }) => {
                // Verify id matches
                if id != Some(self.next_id.load(Ordering::SeqCst) - 1) {
                    return Err(Error::UnexpectedResponse(
                        "id mismatch for JsonRpcResponse".to_string(),
                    ));
                }
                if let Some(err) = error {
                    Err(Error::RpcError {
                        code: err.code,
                        message: err.message,
                    })
                } else if let Some(r) = result {
                    Ok(serde_json::from_value(r)?)
                } else {
                    Err(Error::UnexpectedResponse("missing result".to_string()))
                }
            }
            JsonRpcMessage::Error(JsonRpcError { id, error, .. }) => {
                if id != Some(self.next_id.load(Ordering::SeqCst) - 1) {
                    return Err(Error::UnexpectedResponse(
                        "id mismatch for JsonRpcError".to_string(),
                    ));
                }
                Err(Error::RpcError {
                    code: error.code,
                    message: error.message,
                })
            }
            _ => {
                // Requests/notifications not expected as a response
                Err(Error::UnexpectedResponse(
                    "unexpected message type".to_string(),
                ))
            }
=======
    /// Send a JSON-RPC request and check we don't get an error response.
    async fn send_request<R>(&self, method: &str, params: Value) -> Result<R, Error>
    where
        R: for<'de> Deserialize<'de>,
    {
        let request_start = std::time::Instant::now();
        tracing::info!("🚀 [REQUEST] Starting request: method={}", method);
        
        // Only show params for non-verbose methods
        if !matches!(method, "tools/list" | "resources/list") {
            tracing::info!("🚀 [REQUEST] Request params: {}", params);
        }
        
        let service_lock_start = std::time::Instant::now();
        let mut service = self.service.lock().await;
        tracing::info!("🔒 [REQUEST] Acquired service lock in {}ms for method={}", 
                      service_lock_start.elapsed().as_millis(), method);
        
        let ready_check_start = std::time::Instant::now();
        service.ready().await.map_err(|_| Error::NotReady)?;
        tracing::info!("✅ [REQUEST] Service ready in {}ms for method={}", 
                      ready_check_start.elapsed().as_millis(), method);
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        tracing::info!("🏷️ [REQUEST] Assigned request ID: {} for method={} ({}ms elapsed)", 
                      id, method, request_start.elapsed().as_millis());
        tracing::info!("Assigned request ID: {}", id);

        let mut params = params.clone();
        params["_meta"] = json!({
            "progressToken": format!("prog-{}", id),
        });

        let request = JsonRpcMessage::Request(JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: method.to_string(),
            params: Some(params),
        });

        // Log request without full params to avoid verbosity
        let req_summary = match &request {
            JsonRpcMessage::Request(req) => format!("Request(id={}, method={})", req.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()), req.method),
            _ => format!("{:?}", request)
        };
        let service_call_start = std::time::Instant::now();
        tracing::info!("⏳ [REQUEST] About to call service with: {} ({}ms elapsed since start)", 
                      req_summary, request_start.elapsed().as_millis());
        let response_msg = service
            .call(request)
            .await
            .map_err(|e| {
                let total_elapsed = request_start.elapsed();
                tracing::error!("❌ [REQUEST] MCP service call failed for method {} after {}ms: {}", 
                              method, total_elapsed.as_millis(), e);
                Error::McpServerError {
                    server: self
                        .server_info
                        .as_ref()
                        .map(|s| s.name.clone())
                        .unwrap_or("".to_string()),
                    method: method.to_string(),
                    // we don't need include params because it can be really large
                    source: Box::<Error>::new(e.into()),
                }
            })?;
        
        let service_call_elapsed = service_call_start.elapsed();
        tracing::info!("✅ [REQUEST] Service call completed in {}ms for method={}", 
                      service_call_elapsed.as_millis(), method);
        
        // Log response type without full content
        let resp_summary = match &response_msg {
            JsonRpcMessage::Response(resp) => {
                if let Some(result) = &resp.result {
                    if result.get("tools").is_some() {
                        format!("Response(tools list with {} items)", result.get("tools").and_then(|t| t.as_array()).map(|a| a.len()).unwrap_or(0))
                    } else {
                        format!("Response(id={})", resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()))
                    }
                } else {
                    format!("Response(id={})", resp.id.as_ref().map(|i| i.to_string()).unwrap_or("none".to_string()))
                }
            },
            JsonRpcMessage::Error(err) => format!("Error(code={}, message={})", err.error.code, err.error.message),
            _ => format!("{:?}", response_msg)
        };
        tracing::info!("Received response from MCP service: {}", resp_summary);

        match response_msg {
            JsonRpcMessage::Response(JsonRpcResponse {
                id, result, error, ..
            }) => {
                // Verify id matches
                if id != Some(self.next_id.load(Ordering::SeqCst) - 1) {
                    return Err(Error::UnexpectedResponse(
                        "id mismatch for JsonRpcResponse".to_string(),
                    ));
                }
                if let Some(err) = error {
                    Err(Error::RpcError {
                        code: err.code,
                        message: err.message,
                    })
                } else if let Some(r) = result {
                    Ok(serde_json::from_value(r)?)
                } else {
                    Err(Error::UnexpectedResponse("missing result".to_string()))
                }
            }
            JsonRpcMessage::Error(JsonRpcError { id, error, .. }) => {
                if id != Some(self.next_id.load(Ordering::SeqCst) - 1) {
                    return Err(Error::UnexpectedResponse(
                        "id mismatch for JsonRpcError".to_string(),
                    ));
                }
                Err(Error::RpcError {
                    code: error.code,
                    message: error.message,
                })
            }
            _ => {
                // Requests/notifications not expected as a response
                Err(Error::UnexpectedResponse(
                    "unexpected message type".to_string(),
                ))
            }
>>>>>>> Stashed changes
        }
    }
<<<<<<< Updated upstream
||||||| Stash base

    /// Send a JSON-RPC notification.
    async fn send_notification(&self, method: &str, params: Value) -> Result<(), Error> {
        let mut service = self.service.lock().await;
        service.ready().await.map_err(|_| Error::NotReady)?;

        let notification = JsonRpcMessage::Notification(JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: Some(params.clone()),
        });

        service
            .call(notification)
            .await
            .map_err(|e| Error::McpServerError {
                server: self
                    .server_info
                    .as_ref()
                    .map(|s| s.name.clone())
                    .unwrap_or("".to_string()),
                method: method.to_string(),
                // we don't need include params because it can be really large
                source: Box::<Error>::new(e.into()),
            })?;

        Ok(())
    }

    // Check if the client has completed initialization
    fn completed_initialization(&self) -> bool {
        self.server_capabilities.is_some()
    }
=======

    /// Send a JSON-RPC notification.
    async fn send_notification(&self, method: &str, params: Value) -> Result<(), Error> {
        let timing_start = std::time::Instant::now();
        tracing::info!("📢 [NOTIFICATION] Sending notification: method={}, params={}", method, params);
        let calling_context = std::backtrace::Backtrace::capture();
        tracing::info!("📋 [NOTIFICATION] Call stack for send_notification({}):\n{}", method, calling_context);
        
        let mut service = self.service.lock().await;
        service.ready().await.map_err(|_| Error::NotReady)?;

        let notification = JsonRpcMessage::Notification(JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: Some(params.clone()),
        });
        
        tracing::info!("📢 [NOTIFICATION] About to call service with notification: method={} ({}ms elapsed)", 
                      method, timing_start.elapsed().as_millis());

        let _result = service
            .call(notification)
            .await
            .map_err(|e| {
                tracing::error!("📢 [NOTIFICATION] Failed to send notification {}: {}", method, e);
                Error::McpServerError {
                    server: self
                        .server_info
                        .as_ref()
                        .map(|s| s.name.clone())
                        .unwrap_or("".to_string()),
                    method: method.to_string(),
                    // we don't need include params because it can be really large
                    source: Box::<Error>::new(e.into()),
                }
            })?;
        
        let total_elapsed = timing_start.elapsed();
        tracing::info!("✅ [NOTIFICATION] Successfully sent notification: method={} ({}ms total)", 
                      method, total_elapsed.as_millis());
        Ok(())
    }

    // Check if the client has completed initialization
    fn completed_initialization(&self) -> bool {
        self.server_capabilities.is_some()
    }
>>>>>>> Stashed changes
}

#[async_trait::async_trait]
<<<<<<< Updated upstream
impl McpClientTrait for McpClient {
    fn get_info(&self) -> Option<&InitializeResult> {
        self.server_info.as_ref()
||||||| Stash base
impl<T> McpClientTrait for McpClient<T>
where
    T: TransportHandle + Send + Sync + 'static,
{
    async fn initialize(
        &mut self,
        info: ClientInfo,
        capabilities: ClientCapabilities,
    ) -> Result<InitializeResult, Error> {
        let params = InitializeParams {
            protocol_version: "2025-03-26".to_string(),
            client_info: info,
            capabilities,
        };
        let result: InitializeResult = self
            .send_request("initialize", serde_json::to_value(params)?)
            .await?;

        self.send_notification("notifications/initialized", serde_json::json!({}))
            .await?;

        self.server_capabilities = Some(result.capabilities.clone());

        self.server_info = Some(result.server_info.clone());

        Ok(result)
=======
impl<T> McpClientTrait for McpClient<T>
where
    T: TransportHandle + Send + Sync + 'static,
{
    async fn initialize(
        &mut self,
        info: ClientInfo,
        capabilities: ClientCapabilities,
    ) -> Result<InitializeResult, Error> {
        tracing::info!("Initializing MCP client with info: {:?}", info);
        let params = InitializeParams {
            protocol_version: "2025-03-26".to_string(),
            client_info: info,
            capabilities,
        };
        tracing::info!("Sending initialize request to MCP server");
        let result: InitializeResult = self
            .send_request("initialize", serde_json::to_value(params)?)
            .await?;

        tracing::info!("Initialize request successful, server: {} v{}", 
                      result.server_info.name, result.server_info.version);
        
        // Log capabilities concisely
        let mut caps = Vec::new();
        if result.capabilities.tools.is_some() { caps.push("tools"); }
        if result.capabilities.resources.is_some() { caps.push("resources"); }
        if result.capabilities.prompts.is_some() { caps.push("prompts"); }
        tracing::info!("Server capabilities: {}", caps.join(", "));
        
        tracing::info!("🚀 [INIT] About to send notifications/initialized notification");
        let init_notification_time = std::time::Instant::now();
        self.send_notification("notifications/initialized", serde_json::json!({}))
            .await?;
        tracing::info!("✅ [INIT] notifications/initialized sent successfully in {}ms", 
                      init_notification_time.elapsed().as_millis());

        self.server_capabilities = Some(result.capabilities.clone());
        self.server_info = Some(result.server_info.clone());
        
        tracing::info!("✅ [INIT] MCP client initialization completed successfully");
        Ok(result)
>>>>>>> Stashed changes
    }

    async fn list_resources(&self, cursor: Option<String>) -> Result<ListResourcesResult, Error> {
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::ListResourcesRequest(ListResourcesRequest {
                    params: Some(PaginatedRequestParam { cursor }),
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::ListResourcesResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
        }
    }

    async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult, Error> {
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::ReadResourceRequest(ReadResourceRequest {
                    params: ReadResourceRequestParam {
                        uri: uri.to_string(),
                    },
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::ReadResourceResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
        }
    }

<<<<<<< Updated upstream
    async fn list_tools(&self, cursor: Option<String>) -> Result<ListToolsResult, Error> {
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::ListToolsRequest(ListToolsRequest {
                    params: Some(PaginatedRequestParam { cursor }),
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::ListToolsResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
||||||| Stash base
    async fn list_tools(&self, next_cursor: Option<String>) -> Result<ListToolsResult, Error> {
        if !self.completed_initialization() {
            return Err(Error::NotInitialized);
=======
    async fn list_tools(&self, next_cursor: Option<String>) -> Result<ListToolsResult, Error> {
        tracing::info!("Listing tools, cursor: {:?}", next_cursor);
        if !self.completed_initialization() {
            return Err(Error::NotInitialized);
>>>>>>> Stashed changes
        }
<<<<<<< Updated upstream
||||||| Stash base
        // If tools is not supported, return an empty list
        if self.server_capabilities.as_ref().unwrap().tools.is_none() {
            return Ok(ListToolsResult {
                tools: vec![],
                next_cursor: None,
            });
        }

        let payload = next_cursor
            .map(|cursor| serde_json::json!({"cursor": cursor}))
            .unwrap_or_else(|| serde_json::json!({}));

        self.send_request("tools/list", payload).await
=======
        // If tools is not supported, return an empty list
        if self.server_capabilities.as_ref().unwrap().tools.is_none() {
            tracing::info!("Server does not support tools capability, returning empty list");
            return Ok(ListToolsResult {
                tools: vec![],
                next_cursor: None,
            });
        }

        let payload = next_cursor
            .map(|cursor| serde_json::json!({"cursor": cursor}))
            .unwrap_or_else(|| serde_json::json!({}));

        tracing::info!("Sending tools/list request with payload: {}", payload);
        let result: ListToolsResult = self.send_request("tools/list", payload).await?;
        let tool_names: Vec<&str> = result.tools.iter().map(|t| t.name.as_str()).collect();
        tracing::info!("Received {} tools: [{}]", result.tools.len(), tool_names.join(", "));
        Ok(result)
>>>>>>> Stashed changes
    }

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult, Error> {
        let arguments = match arguments {
            Value::Object(map) => Some(map),
            _ => None,
        };
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::CallToolRequest(CallToolRequest {
                    params: CallToolRequestParam {
                        name: name.to_string().into(),
                        arguments,
                    },
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::CallToolResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
        }
    }

    async fn list_prompts(&self, cursor: Option<String>) -> Result<ListPromptsResult, Error> {
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::ListPromptsRequest(ListPromptsRequest {
                    params: Some(PaginatedRequestParam { cursor }),
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::ListPromptsResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
        }
    }

    async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult, Error> {
        let arguments = match arguments {
            Value::Object(map) => Some(map),
            _ => None,
        };
        let res = self
            .client
            .lock()
            .await
            .send_request_with_option(
                ClientRequest::GetPromptRequest(GetPromptRequest {
                    params: GetPromptRequestParam {
                        name: name.to_string(),
                        arguments,
                    },
                    method: Default::default(),
                    extensions: Default::default(),
                }),
                self.get_request_options(),
            )
            .await?
            .await_response()
            .await?;
        match res {
            ServerResult::GetPromptResult(result) => Ok(result),
            _ => Err(ServiceError::UnexpectedResponse),
        }
    }

    async fn subscribe(&self) -> mpsc::Receiver<ServerNotification> {
        let (tx, rx) = mpsc::channel(16);
        self.notification_subscribers.lock().await.push(tx);
        rx
    }
}
