use serde_json::Value;
use std::collections::HashMap;

use goose::providers::base::{Provider as GooseProvider, ProviderUsage};
use goose::model::ModelConfig;
use goose::message::{Message, MessageContent};
use mcp_core::tool::Tool;
use mcp_core::Content;

use crate::error::BitMortarError;
use crate::types::{ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo, Choice, Usage};

/// Wrapper around Goose providers to provide Databricks-compatible API
pub struct BitMortarProvider {
    provider: Box<dyn GooseProvider>,
    name: String,
}

impl BitMortarProvider {
    pub fn new(provider_name: &str, config: HashMap<String, String>) -> Result<Self, BitMortarError> {
        // Set up environment variables from config for Goose to pick up
        for (key, value) in &config {
            std::env::set_var(key, value);
        }

        // Get default model for this provider type
        let default_model = match provider_name {
            "openai" => goose::providers::openai::OPEN_AI_DEFAULT_MODEL,
            "anthropic" => goose::providers::anthropic::ANTHROPIC_DEFAULT_MODEL,
            "databricks" => goose::providers::databricks::DATABRICKS_DEFAULT_MODEL,
            _ => "gpt-4o", // fallback
        };

        let model_config = ModelConfig::new(default_model.to_string());

        // Create the Goose provider manually since factory is private
        let provider: Box<dyn GooseProvider> = match provider_name {
            "openai" => Box::new(goose::providers::openai::OpenAiProvider::from_env(model_config)
                .map_err(|e| BitMortarError::ConfigError(format!("Failed to create OpenAI provider: {}", e)))?),
            "anthropic" => Box::new(goose::providers::anthropic::AnthropicProvider::from_env(model_config)
                .map_err(|e| BitMortarError::ConfigError(format!("Failed to create Anthropic provider: {}", e)))?),
            "databricks" => Box::new(goose::providers::databricks::DatabricksProvider::from_env(model_config)
                .map_err(|e| BitMortarError::ConfigError(format!("Failed to create Databricks provider: {}", e)))?),
            _ => return Err(BitMortarError::ConfigError(format!("Unsupported provider: {}", provider_name))),
        };

        Ok(Self {
            provider,
            name: provider_name.to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn health_check(&self) -> bool {
        // Try to fetch models as a health check
        self.provider.fetch_supported_models_async().await.is_ok()
    }

    pub async fn chat_completion(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse, BitMortarError> {
        // Convert request to Goose format
        let (system, messages, tools) = self.convert_request_to_goose(request)?;

        // Call Goose provider
        let (response_message, usage) = self
            .provider
            .complete(&system, &messages, &tools)
            .await
            .map_err(|e| match e {
                goose::providers::errors::ProviderError::ContextLengthExceeded(msg) => {
                    BitMortarError::ContextLengthExceeded(msg)
                }
                goose::providers::errors::ProviderError::RateLimitExceeded(msg) => {
                    BitMortarError::RateLimitExceeded(msg)
                }
                goose::providers::errors::ProviderError::Authentication(msg) => {
                    BitMortarError::AuthenticationError(msg)
                }
                _ => BitMortarError::ProviderError(format!("Provider error: {}", e)),
            })?;

        // Convert response back to Databricks format
        self.convert_response_from_goose(response_message, usage)
    }

    pub async fn create_embeddings(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse, BitMortarError> {
        if !self.provider.supports_embeddings() {
            return Err(BitMortarError::ProviderError(
                format!("Provider {} does not support embeddings", self.name),
            ));
        }

        let embeddings = self
            .provider
            .create_embeddings(request.input.clone())
            .await
            .map_err(|e| BitMortarError::ProviderError(format!("Embedding error: {}", e)))?;

        let data = embeddings
            .into_iter()
            .enumerate()
            .map(|(index, embedding)| crate::types::EmbeddingData {
                index: index as u32,
                embedding,
            })
            .collect();

        Ok(EmbeddingResponse {
            data,
            usage: Usage {
                prompt_tokens: 0, // Goose doesn't provide detailed embedding usage
                completion_tokens: 0,
                total_tokens: 0,
            },
            model: request.model,
        })
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, BitMortarError> {
        match self.provider.fetch_supported_models_async().await {
            Ok(Some(models)) => {
                let model_infos = models
                    .into_iter()
                    .map(|model| ModelInfo {
                        id: model,
                        object: "model".to_string(),
                        created: chrono::Utc::now().timestamp() as u64,
                        owned_by: self.name.clone(),
                    })
                    .collect();
                Ok(model_infos)
            }
            Ok(None) => {
                // Return known models from metadata based on provider type
                let known_models = match self.name.as_str() {
                    "openai" => goose::providers::openai::OpenAiProvider::metadata().known_models,
                    "anthropic" => goose::providers::anthropic::AnthropicProvider::metadata().known_models,
                    "databricks" => goose::providers::databricks::DatabricksProvider::metadata().known_models,
                    _ => vec![],
                };
                
                let model_infos = known_models
                    .into_iter()
                    .map(|model| ModelInfo {
                        id: model.name,
                        object: "model".to_string(),
                        created: chrono::Utc::now().timestamp() as u64,
                        owned_by: self.name.clone(),
                    })
                    .collect();
                Ok(model_infos)
            }
            Err(e) => Err(BitMortarError::ProviderError(format!(
                "Failed to fetch models: {}",
                e
            ))),
        }
    }

    pub fn supports_embeddings(&self) -> bool {
        self.provider.supports_embeddings()
    }

    // Convert Databricks/OpenAI-style request to Goose format
    fn convert_request_to_goose(&self, request: ChatCompletionRequest) -> Result<(String, Vec<Message>, Vec<Tool>), BitMortarError> {
        let mut system = String::new();
        let mut goose_messages = Vec::new();

        // Process messages
        for msg in request.messages {
            match msg.role.as_str() {
                "system" => {
                    if let crate::types::MessageContent::Text(text) = msg.content {
                        system = text;
                    }
                }
                "user" => {
                    let mut message = Message::user();
                    match msg.content {
                        crate::types::MessageContent::Text(text) => {
                            message = message.with_text(&text);
                        }
                        crate::types::MessageContent::Array(parts) => {
                            for part in parts {
                                match part {
                                    crate::types::ContentPart::Text { text } => {
                                        message = message.with_text(&text);
                                    }
                                    crate::types::ContentPart::ImageUrl { image_url } => {
                                        // Handle image content - this would need proper image handling
                                        message = message.with_text(&format!("Image: {}", image_url.url));
                                    }
                                    crate::types::ContentPart::Reasoning { .. } => {
                                        // Skip reasoning parts for user messages
                                    }
                                }
                            }
                        }
                    }
                    goose_messages.push(message);
                }
                "assistant" => {
                    let mut message = Message::assistant();
                    match msg.content {
                        crate::types::MessageContent::Text(text) => {
                            message = message.with_text(&text);
                        }
                        crate::types::MessageContent::Array(parts) => {
                            for part in parts {
                                match part {
                                    crate::types::ContentPart::Text { text } => {
                                        message = message.with_text(&text);
                                    }
                                    crate::types::ContentPart::Reasoning { summary } => {
                                        for reasoning in summary {
                                            match reasoning {
                                                crate::types::ReasoningSummary::Text { text, signature } => {
                                                    message = message.with_thinking(&text, &signature);
                                                }
                                                crate::types::ReasoningSummary::Encrypted { data } => {
                                                    message = message.with_redacted_thinking(&data);
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    
                    // Handle tool calls
                    if let Some(tool_calls) = msg.tool_calls {
                        for tool_call in tool_calls {
                            if let Ok(args) = serde_json::from_str::<Value>(&tool_call.function.arguments) {
                                let tool_call_obj = mcp_core::ToolCall::new(&tool_call.function.name, args);
                                message = message.with_tool_request(&tool_call.id, Ok(tool_call_obj));
                            } else {
                                let error = mcp_core::ToolError::InvalidParameters(
                                    format!("Invalid arguments: {}", tool_call.function.arguments)
                                );
                                message = message.with_tool_request(&tool_call.id, Err(error));
                            }
                        }
                    }
                    
                    goose_messages.push(message);
                }
                "tool" => {
                    if let (Some(tool_call_id), crate::types::MessageContent::Text(content)) = 
                        (msg.tool_call_id, msg.content) {
                        let message = Message::user().with_tool_response(
                            tool_call_id,
                            Ok(vec![Content::text(&content)])
                        );
                        goose_messages.push(message);
                    }
                }
                _ => {
                    // Skip unknown roles
                }
            }
        }

        // Convert tools
        let tools = if let Some(tools) = request.tools {
            tools
                .into_iter()
                .map(|tool| {
                    Tool::new(
                        &tool.function.name,
                        &tool.function.description,
                        tool.function.parameters,
                        None,
                    )
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok((system, goose_messages, tools))
    }

    // Convert Goose response to Databricks format
    fn convert_response_from_goose(&self, message: Message, usage: ProviderUsage) -> Result<ChatCompletionResponse, BitMortarError> {
        let mut content_parts = Vec::new();
        let mut tool_calls = Vec::new();

        // Process message content
        for content in &message.content {
            match content {
                MessageContent::Text(text) => {
                    if !text.text.is_empty() {
                        content_parts.push(crate::types::ContentPart::Text {
                            text: text.text.clone(),
                        });
                    }
                }
                MessageContent::Thinking(thinking) => {
                    content_parts.push(crate::types::ContentPart::Reasoning {
                        summary: vec![crate::types::ReasoningSummary::Text {
                            text: thinking.thinking.clone(),
                            signature: thinking.signature.clone(),
                        }],
                    });
                }
                MessageContent::RedactedThinking(redacted) => {
                    content_parts.push(crate::types::ContentPart::Reasoning {
                        summary: vec![crate::types::ReasoningSummary::Encrypted {
                            data: redacted.data.clone(),
                        }],
                    });
                }
                MessageContent::ToolRequest(request) => {
                    if let Ok(tool_call) = &request.tool_call {
                        tool_calls.push(crate::types::ToolCall {
                            id: request.id.clone(),
                            tool_type: "function".to_string(),
                            function: crate::types::FunctionCall {
                                name: tool_call.name.clone(),
                                arguments: tool_call.arguments.to_string(),
                            },
                        });
                    }
                }
                _ => {
                    // Skip other content types for now
                }
            }
        }

        // Create message content
        let message_content = if content_parts.len() == 1 {
            if let crate::types::ContentPart::Text { text } = &content_parts[0] {
                crate::types::MessageContent::Text(text.clone())
            } else {
                crate::types::MessageContent::Array(content_parts)
            }
        } else if content_parts.is_empty() && !tool_calls.is_empty() {
            crate::types::MessageContent::Text(String::new())
        } else {
            crate::types::MessageContent::Array(content_parts)
        };

        let chat_message = crate::types::ChatMessage {
            role: "assistant".to_string(),
            content: message_content,
            tool_calls: if tool_calls.is_empty() { None } else { Some(tool_calls) },
            tool_call_id: None,
        };

        let response = ChatCompletionResponse {
            choices: vec![Choice {
                index: 0,
                message: chat_message,
                finish_reason: "stop".to_string(),
            }],
            usage: Usage {
                prompt_tokens: usage.usage.input_tokens.unwrap_or(0) as u32,
                completion_tokens: usage.usage.output_tokens.unwrap_or(0) as u32,
                total_tokens: usage.usage.total_tokens.unwrap_or(0) as u32,
            },
            model: Some(usage.model),
        };

        Ok(response)
    }
}
