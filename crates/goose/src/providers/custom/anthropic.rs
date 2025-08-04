use anyhow::Result;
use async_trait::async_trait;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use crate::message::Message;
use crate::model::ModelConfig;
use crate::providers::base::{MessageStream, Provider, ProviderMetadata, ProviderUsage};
use crate::providers::errors::ProviderError;
use crate::providers::formats::anthropic::{
    create_request, get_usage, response_to_message, response_to_streaming_message,
};
use crate::providers::utils::get_model;
use rmcp::model::Tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCompatibleConfig {
    pub id: String,
    pub display_name: String,
    pub api_url: String,
    pub api_key: String,
    pub models: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct AnthropicCompatibleProvider {
    client: Client,
    config: AnthropicCompatibleConfig,
    api_key: String,
    model: ModelConfig,
}

impl AnthropicCompatibleProvider {
    pub fn from_env(config: AnthropicCompatibleConfig, model: ModelConfig) -> Result<Self> {
        let global_config = crate::config::Config::global();
        let api_key: String = global_config.get_secret(&config.api_key)?;

        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        Ok(Self {
            client,
            config,
            api_key,
            model,
        })
    }
}

#[async_trait]
impl Provider for AnthropicCompatibleProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::empty()
    }

    fn get_model_config(&self) -> ModelConfig {
        self.model.clone()
    }

    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        let payload = create_request(&self.model, system, messages, tools)?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&self.api_key)
                .map_err(|e| ProviderError::RequestFailed(e.to_string()))?,
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let response = self
            .client
            .post(&self.config.api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::RequestFailed(format!(
                "Request failed with status {}: {}",
                status, text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;

        let message = response_to_message(&response_json)?;
        let usage = get_usage(&response_json)?;
        let model = get_model(&response_json);
        let provider_usage = ProviderUsage::new(model, usage);

        Ok((message, provider_usage))
    }

    async fn fetch_supported_models_async(&self) -> Result<Option<Vec<String>>, ProviderError> {
        Ok(Some(self.config.models.clone()))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    async fn stream(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<MessageStream, ProviderError> {
        let payload = create_request(&self.model, system, messages, tools)?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&self.api_key)
                .map_err(|e| ProviderError::RequestFailed(e.to_string()))?,
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let mut request_payload = payload.clone();
        request_payload
            .as_object_mut()
            .unwrap()
            .insert("stream".to_string(), Value::Bool(true));

        let response = self
            .client
            .post(&self.config.api_url)
            .headers(headers)
            .json(&request_payload)
            .send()
            .await
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::RequestFailed(format!(
                "Streaming request failed with status: {}. Error: {}",
                status, error_text
            )));
        }

        // Map reqwest error to io::Error
        use async_stream::try_stream;
        use futures::StreamExt;
        use futures::TryStreamExt;
        use std::io;
        use tokio::pin;
        use tokio_util::codec::{FramedRead, LinesCodec};
        use tokio_util::io::StreamReader;

        let stream = response.bytes_stream().map_err(io::Error::other);
        let model_config = self.model.clone();

        // Wrap in a line decoder and yield lines inside the stream
        Ok(Box::pin(try_stream! {
            let stream_reader = StreamReader::new(stream);
            let framed = FramedRead::new(stream_reader, LinesCodec::new()).map_err(anyhow::Error::from);

            let message_stream = response_to_streaming_message(framed);
            pin!(message_stream);
            while let Some(message) = message_stream.next().await {
                let (message, usage) = message.map_err(|e| ProviderError::RequestFailed(format!("Stream decode error: {}", e)))?;
                crate::providers::utils::emit_debug_trace(&model_config, &payload, &message, &usage.as_ref().map(|f| f.usage).unwrap_or_default());
                yield (message, usage);
            }
        }))
    }

    fn supports_cache_control(&self) -> bool {
        false
    }
}
