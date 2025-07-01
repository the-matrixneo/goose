use anyhow::{Result, anyhow};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;
use lazy_static::lazy_static;

use crate::providers::errors::ProviderError;

/// Default timeout for HTTP requests
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;

/// Common retry configuration for providers
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 32000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Authentication type for providers
#[derive(Debug, Clone)]
pub enum AuthType {
    Bearer,
    ApiKey,
    Custom(String),
}

/// Common headers builder for providers
pub struct HeaderBuilder {
    auth_token: String,
    auth_type: AuthType,
    custom_headers: HashMap<String, String>,
}

impl HeaderBuilder {
    pub fn new(auth_token: String, auth_type: AuthType) -> Self {
        Self {
            auth_token,
            auth_type,
            custom_headers: HashMap::new(),
        }
    }

    pub fn add_custom_header(mut self, key: String, value: String) -> Self {
        self.custom_headers.insert(key, value);
        self
    }

    pub fn build(self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        
        // Add authorization header
        match self.auth_type {
            AuthType::Bearer => {
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", self.auth_token).parse().unwrap(),
                );
            }
            AuthType::ApiKey => {
                headers.insert(
                    "X-API-Key",
                    self.auth_token.parse().unwrap(),
                );
            }
            AuthType::Custom(header_name) => {
                if let Ok(name) = reqwest::header::HeaderName::from_bytes(header_name.as_bytes()) {
                    headers.insert(
                        name,
                        self.auth_token.parse().unwrap(),
                    );
                }
            }
        }
        
        // Add custom headers
        for (key, value) in self.custom_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                value.parse()
            ) {
                headers.insert(header_name, header_value);
            }
        }
        
        headers
    }
}

/// Connection pool configuration
pub struct ConnectionPoolConfig {
    /// Maximum idle connections per host
    pub max_idle_per_host: usize,
    /// Time before idle connections are closed
    pub idle_timeout_secs: u64,
    /// Maximum number of connections per host
    pub max_connections_per_host: Option<usize>,
    /// Enable HTTP/2
    pub http2_enabled: bool,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_idle_per_host: 10,
            idle_timeout_secs: 90,
            max_connections_per_host: Some(50),
            http2_enabled: true,
        }
    }
}

/// Create a default HTTP client with common settings
pub fn create_default_client(timeout_secs: Option<u64>) -> Result<Client> {
    create_client_with_config(timeout_secs, ConnectionPoolConfig::default())
}

/// Create an HTTP client with custom configuration
pub fn create_client_with_config(
    timeout_secs: Option<u64>,
    pool_config: ConnectionPoolConfig,
) -> Result<Client> {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS)))
        .pool_idle_timeout(Duration::from_secs(pool_config.idle_timeout_secs))
        .pool_max_idle_per_host(pool_config.max_idle_per_host);
    
    if pool_config.http2_enabled {
        builder = builder.http2_prior_knowledge();
    }
    
    builder
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
}

// Global shared HTTP client for providers that want to share connections
lazy_static! {
    static ref SHARED_CLIENT: Arc<Client> = Arc::new(
        create_default_client(None)
            .expect("Failed to create shared HTTP client")
    );
}

/// Get the shared HTTP client instance
pub fn get_shared_client() -> Arc<Client> {
    SHARED_CLIENT.clone()
}

/// Build endpoint URL from base and path
pub fn build_endpoint_url(base: &str, path: &str) -> Result<Url, ProviderError> {
    let base_url = Url::parse(base)
        .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;
    base_url.join(path)
        .map_err(|e| ProviderError::RequestFailed(format!("Failed to construct endpoint URL: {e}")))
}

/// Check if an error is retryable
pub trait IsRetryable {
    fn is_retryable(&self) -> bool;
}

impl IsRetryable for ProviderError {
    fn is_retryable(&self) -> bool {
        matches!(
            self,
            ProviderError::RateLimitExceeded(_) | 
            ProviderError::ServerError(_) |
            ProviderError::RequestFailed(_)
        )
    }
}

/// Retry an async operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    config: &RetryConfig,
    mut operation: F,
) -> Result<T, ProviderError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, ProviderError>>,
{
    let mut attempts = 0;
    let mut delay_ms = config.initial_delay_ms;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if e.is_retryable() && attempts < config.max_retries => {
                attempts += 1;
                tracing::warn!(
                    "Retryable error (attempt {}/{}): {}. Retrying in {}ms...",
                    attempts,
                    config.max_retries,
                    e,
                    delay_ms
                );
                
                sleep(Duration::from_millis(delay_ms)).await;
                
                // Update delay with exponential backoff
                delay_ms = ((delay_ms as f64) * config.backoff_multiplier) as u64;
                delay_ms = delay_ms.min(config.max_delay_ms);
            }
            Err(e) => return Err(e),
        }
    }
}

/// Common response handler for providers
pub async fn handle_provider_response(
    response: reqwest::Response,
    provider_name: &str,
) -> Result<serde_json::Value, ProviderError> {
    let status = response.status();
    let response_text = response.text().await
        .map_err(|e| ProviderError::RequestFailed(format!("Failed to read response: {}", e)))?;
    
    if status.is_success() {
        serde_json::from_str(&response_text)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid JSON response: {}", e)))
    } else {
        let error_msg = format!("{} API error ({}): {}", provider_name, status, response_text);
        
        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ProviderError::Authentication(error_msg))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(ProviderError::RateLimitExceeded(error_msg))
            }
            StatusCode::BAD_REQUEST => {
                // Check if it's a context length error
                if response_text.to_lowercase().contains("context") || 
                   response_text.to_lowercase().contains("too long") ||
                   response_text.to_lowercase().contains("exceeds") {
                    Err(ProviderError::ContextLengthExceeded(error_msg))
                } else {
                    Err(ProviderError::RequestFailed(error_msg))
                }
            }
            s if s.is_server_error() => {
                Err(ProviderError::ServerError(error_msg))
            }
            _ => {
                Err(ProviderError::RequestFailed(error_msg))
            }
        }
    }
}

/// Configuration builder for providers
pub struct ProviderConfigBuilder<'a> {
    config: &'a crate::config::Config,
    prefix: String,
}

impl<'a> ProviderConfigBuilder<'a> {
    pub fn new(config: &'a crate::config::Config, prefix: &str) -> Self {
        Self {
            config,
            prefix: prefix.to_uppercase(),
        }
    }
    
    pub fn get_api_key(&self) -> Result<String> {
        self.config.get_secret(&format!("{}_API_KEY", self.prefix))
            .map_err(|e| anyhow!("Failed to get API key: {}", e))
    }
    
    pub fn get_host(&self, default: &str) -> String {
        self.config.get_param(&format!("{}_HOST", self.prefix))
            .unwrap_or_else(|_| default.to_string())
    }
    
    pub fn get_model(&self, default: &str) -> String {
        self.config.get_param(&format!("{}_MODEL", self.prefix))
            .unwrap_or_else(|_| default.to_string())
    }
    
    pub fn get_param(&self, param: &str, default: Option<&str>) -> Option<String> {
        self.config.get_param(&format!("{}_{}", self.prefix, param))
            .ok()
            .or_else(|| default.map(|s| s.to_string()))
    }
}

/// Base provider struct that others can compose with
pub struct BaseProvider {
    pub client: Client,
    pub host: String,
    pub retry_config: RetryConfig,
}

impl BaseProvider {
    pub fn new(host: String, retry_config: Option<RetryConfig>) -> Result<Self> {
        Ok(Self {
            client: create_default_client(None)?,
            host,
            retry_config: retry_config.unwrap_or_default(),
        })
    }
    
    /// Make a POST request with retry logic
    pub async fn post_json<T: Serialize>(
        &self,
        endpoint: &str,
        headers: reqwest::header::HeaderMap,
        payload: &T,
    ) -> Result<serde_json::Value, ProviderError> {
        let url = build_endpoint_url(&self.host, endpoint)?;
        
        retry_with_backoff(&self.retry_config, || async {
            let response = self.client
                .post(url.clone())
                .headers(headers.clone())
                .json(payload)
                .send()
                .await
                .map_err(|e| ProviderError::RequestFailed(format!("Request failed: {}", e)))?;
            
            handle_provider_response(response, "Provider").await
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_builder() {
        let headers = HeaderBuilder::new("test-token".to_string(), AuthType::Bearer)
            .add_custom_header("X-Custom".to_string(), "value".to_string())
            .build();
        
        assert_eq!(
            headers.get("authorization").unwrap(),
            "Bearer test-token"
        );
        assert_eq!(
            headers.get("x-custom").unwrap(),
            "value"
        );
    }
    
    #[test]
    fn test_build_endpoint_url() {
        let url = build_endpoint_url("https://api.example.com", "/v1/chat").unwrap();
        assert_eq!(url.as_str(), "https://api.example.com/v1/chat");
        
        let url = build_endpoint_url("https://api.example.com/", "v1/chat").unwrap();
        assert_eq!(url.as_str(), "https://api.example.com/v1/chat");
    }
    
    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 1000);
        assert_eq!(config.backoff_multiplier, 2.0);
    }
}