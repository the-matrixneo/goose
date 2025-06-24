use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitMortarError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Context length exceeded: {0}")]
    ContextLengthExceeded(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl IntoResponse for BitMortarError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            BitMortarError::ProviderNotFound(_) => (StatusCode::NOT_FOUND, "provider_not_found", self.to_string()),
            BitMortarError::ModelNotFound(_) => (StatusCode::NOT_FOUND, "model_not_found", self.to_string()),
            BitMortarError::AuthenticationError(_) => (StatusCode::UNAUTHORIZED, "authentication_error", self.to_string()),
            BitMortarError::RateLimitExceeded(_) => (StatusCode::TOO_MANY_REQUESTS, "rate_limit_exceeded", self.to_string()),
            BitMortarError::ContextLengthExceeded(_) => (StatusCode::BAD_REQUEST, "context_length_exceeded", self.to_string()),
            BitMortarError::RequestError(_) => (StatusCode::BAD_REQUEST, "request_error", self.to_string()),
            BitMortarError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "config_error", self.to_string()),
            BitMortarError::ProviderError(_) => (StatusCode::BAD_GATEWAY, "provider_error", self.to_string()),
            BitMortarError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "server_error", self.to_string()),
            BitMortarError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", self.to_string()),
            BitMortarError::SerializationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "serialization_error", self.to_string()),
        };

        let body = Json(json!({
            "error": {
                "type": error_type,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}
