use anyhow::Result;
use rmcp::model::{CreateMessageRequestParam, CreateMessageResult};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use uuid::Uuid;

/// Request sent from MCP extension to UI for user confirmation
#[derive(Debug, Clone)]
pub struct SamplingRequest {
    /// Unique identifier for this request
    pub request_id: String,
    /// Name of the extension making the request
    pub extension_name: String,
    /// The original MCP sampling parameters
    pub params: CreateMessageRequestParam,
}

/// Response from UI for a sampling request
#[derive(Debug, Clone)]
pub struct SamplingResponse {
    /// The request ID this response is for
    pub request_id: String,
    /// Whether the request was approved
    pub approved: bool,
    /// The sampling result if approved
    pub result: Option<CreateMessageResult>,
    /// Error message if denied or failed
    pub error_message: Option<String>,
}

/// Trait for handling sampling requests that need UI confirmation
#[async_trait::async_trait]
pub trait SamplingInterface: Send + Sync {
    /// Send a sampling request to the UI and wait for response
    async fn request_sampling(
        &self,
        extension_name: String,
        params: CreateMessageRequestParam,
    ) -> Result<CreateMessageResult>;
}

/// Default implementation that uses channels to communicate with UI
pub struct ChannelSamplingInterface {
    /// Channel for sending sampling requests to the UI
    sampling_tx: Arc<Mutex<Option<mpsc::Sender<SamplingRequest>>>>,
    /// Map of pending sampling requests waiting for UI response
    pending_requests: Arc<Mutex<std::collections::HashMap<String, oneshot::Sender<SamplingResponse>>>>,
}

impl ChannelSamplingInterface {
    pub fn new(
        sampling_tx: Arc<Mutex<Option<mpsc::Sender<SamplingRequest>>>>,
        pending_requests: Arc<Mutex<std::collections::HashMap<String, oneshot::Sender<SamplingResponse>>>>,
    ) -> Self {
        Self {
            sampling_tx,
            pending_requests,
        }
    }
}

#[async_trait::async_trait]
impl SamplingInterface for ChannelSamplingInterface {
    async fn request_sampling(
        &self,
        extension_name: String,
        params: CreateMessageRequestParam,
    ) -> Result<CreateMessageResult> {
        // Generate unique request ID
        let request_id = Uuid::new_v4().to_string();
        
        // Create oneshot channel for response
        let (response_tx, response_rx) = oneshot::channel::<SamplingResponse>();
        
        // Store the response sender
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id.clone(), response_tx);
        }
        
        // Create the request
        let request = SamplingRequest {
            request_id: request_id.clone(),
            extension_name,
            params,
        };
        
        // Send request to UI
        {
            let sampling_tx = self.sampling_tx.lock().await;
            if let Some(ref tx) = *sampling_tx {
                if let Err(e) = tx.send(request).await {
                    // Clean up pending request on send failure
                    let mut pending = self.pending_requests.lock().await;
                    pending.remove(&request_id);
                    return Err(anyhow::anyhow!("Failed to send sampling request to UI: {}", e));
                }
            } else {
                return Err(anyhow::anyhow!("No UI connected to handle sampling requests"));
            }
        }
        
        // Wait for response
        match response_rx.await {
            Ok(response) => {
                if response.approved {
                    response.result.ok_or_else(|| {
                        anyhow::anyhow!("Sampling approved but no result provided")
                    })
                } else {
                    Err(anyhow::anyhow!(
                        "Sampling request denied: {}",
                        response.error_message.unwrap_or_else(|| "User denied the request".to_string())
                    ))
                }
            }
            Err(_) => {
                // Clean up pending request on receive failure
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                Err(anyhow::anyhow!("Failed to receive sampling response - channel closed"))
            }
        }
    }
}

/// No-op implementation for when UI sampling is not available
pub struct NoOpSamplingInterface;

#[async_trait::async_trait]
impl SamplingInterface for NoOpSamplingInterface {
    async fn request_sampling(
        &self,
        _extension_name: String,
        _params: CreateMessageRequestParam,
    ) -> Result<CreateMessageResult> {
        Err(anyhow::anyhow!("Sampling requests are not supported in this context"))
    }
}
