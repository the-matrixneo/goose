use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::state::AppState;

/// Request sent from MCP extension to UI for user confirmation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SamplingConfirmationRequest {
    /// Unique identifier for this request
    pub request_id: String,
    /// Name of the extension making the request
    pub extension_name: String,
    /// Messages to be sent to the model
    pub messages: Vec<SamplingMessage>,
    /// Optional model preferences
    pub model_preferences: Option<ModelPreferences>,
    /// System prompt if provided
    pub system_prompt: Option<String>,
}

/// A message in the sampling request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SamplingMessage {
    /// Role of the message sender
    pub role: String,
    /// Content of the message
    pub content: String,
}

/// Model preferences for sampling
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModelPreferences {
    /// Hints for model selection
    pub hints: Option<Vec<String>>,
}

/// Response from UI for a sampling request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SamplingConfirmationResponse {
    /// The request ID this response is for
    pub request_id: String,
    /// Whether the request was approved
    pub approved: bool,
    /// Optional response content if approved
    pub response_content: Option<String>,
}

/// SSE endpoint for streaming sampling requests to the UI
pub async fn sampling_stream(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<SamplingConfirmationRequest>(32);

    // Store the sender in app state for other routes to use
    {
        let mut sampling_tx = state.sampling_tx.lock().await;
        *sampling_tx = Some(tx);
    }

    let stream = ReceiverStream::new(rx).map(|request| {
        Ok(Event::default()
            .event("sampling_request")
            .json_data(request)
            .unwrap_or_else(|e| {
                error!("Failed to serialize sampling request: {}", e);
                Event::default()
            }))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Endpoint to respond to a sampling request
pub async fn respond_to_sampling(
    State(state): State<Arc<AppState>>,
    Json(response): Json<SamplingConfirmationResponse>,
) -> impl IntoResponse {
    // Find the pending request and send response
    let mut pending_requests = state.pending_sampling_requests.lock().await;

    if let Some(sender) = pending_requests.remove(&response.request_id) {
        // Send the response back to the waiting MCP handler
        if let Err(_e) = sender.send(response.clone()) {
            error!("Failed to send sampling response - receiver dropped");
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send response",
            );
        }

        (axum::http::StatusCode::OK, "Response sent")
    } else {
        (
            axum::http::StatusCode::NOT_FOUND,
            "Request not found or already processed",
        )
    }
}

/// Test endpoint to simulate a sampling request (for development)
#[cfg(debug_assertions)]
pub async fn test_sampling_request(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let request = SamplingConfirmationRequest {
        request_id: Uuid::new_v4().to_string(),
        extension_name: "test_extension".to_string(),
        messages: vec![SamplingMessage {
            role: "user".to_string(),
            content: "What is the capital of France?".to_string(),
        }],
        model_preferences: Some(ModelPreferences {
            hints: Some(vec!["fast".to_string(), "accurate".to_string()]),
        }),
        system_prompt: Some("You are a helpful assistant.".to_string()),
    };

    // Send to UI via SSE
    if let Some(ref tx) = *state.sampling_tx.lock().await {
        if let Err(e) = tx.send(request.clone()).await {
            error!("Failed to send test sampling request: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to send request"
                })),
            );
        }
    } else {
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "No UI connected to receive sampling requests"
            })),
        );
    }

    (
        axum::http::StatusCode::OK,
        Json(serde_json::json!({
            "request_id": request.request_id,
            "message": "Test sampling request sent"
        })),
    )
}

/// Configure sampling routes
pub fn routes(state: Arc<AppState>) -> Router {
    let mut router = Router::new()
        .route("/api/sampling/stream", get(sampling_stream))
        .route("/api/sampling/respond", post(respond_to_sampling));

    // Add test endpoint in debug mode
    #[cfg(debug_assertions)]
    {
        router = router.route("/api/sampling/test", post(test_sampling_request));
    }

    router.with_state(state)
}
