use crate::message::Message;
use crate::prompt_template::render_global_file;
use crate::providers::base::Provider;
use crate::token_counter::create_async_token_counter;
use anyhow::Result;
use rmcp::model::Role;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct SummarizeContext {
    messages: String,
}

/// Summarization function that uses the detailed prompt from the markdown template
pub async fn summarize_messages(
    provider: Arc<dyn Provider>,
    messages: &[Message],
) -> Result<Option<(Message, usize, usize)>, anyhow::Error> {
    if messages.is_empty() {
        return Ok(None);
    }

    // Format all messages as a single string for the summarization prompt
    let messages_text = messages
        .iter()
        .map(|msg| format!("{:?}", msg))
        .collect::<Vec<_>>()
        .join("\n\n");

    let context = SummarizeContext {
        messages: messages_text,
    };

    // Render the one-shot summarization prompt
    let system_prompt = render_global_file("summarize_oneshot.md", &context)?;

    // Create a simple user message requesting summarization
    let user_message = Message::user()
        .with_text("Please summarize the conversation history provided in the system prompt.");
    let summarization_request = vec![user_message];

    // Send the request to the provider and fetch the response
    let (mut response, provider_usage) = provider
        .complete(&system_prompt, &summarization_request, &[])
        .await?;

    // Set role to user as it will be used in following conversation as user content
    response.role = Role::User;

    // Get the token count from the provider usage, with token counter as fallback
    let (input_tokens, output_tokens) = if provider_usage.usage.input_tokens.is_some() && provider_usage.usage.output_tokens.is_some() {
        // Use provider usage if available
        (
            provider_usage.usage.input_tokens.unwrap() as usize,
            provider_usage.usage.output_tokens.unwrap() as usize,
        )
    } else {
        // Fallback to token counter when provider usage is not available
        let token_counter = create_async_token_counter().await
            .map_err(|e| anyhow::anyhow!("Failed to create token counter: {}", e))?;
        
        let input_count = token_counter.count_chat_tokens(&system_prompt, &summarization_request, &[]);
        let response_text = response.content.iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join(" ");
        let output_count = token_counter.count_tokens(&response_text);
        
        (input_count, output_count)
    };

    Ok(Some((response, input_tokens, output_tokens)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{Message, MessageContent};
    use crate::model::ModelConfig;
    use crate::providers::base::{Provider, ProviderMetadata, ProviderUsage, Usage};
    use crate::providers::errors::ProviderError;
    use chrono::Utc;
    use rmcp::model::Role;
    use rmcp::model::Tool;
    use rmcp::model::{AnnotateAble, RawTextContent};
    use std::sync::Arc;

    #[derive(Clone)]
    struct MockProvider {
        model_config: ModelConfig,
    }

    #[async_trait::async_trait]
    impl Provider for MockProvider {
        fn metadata() -> ProviderMetadata {
            ProviderMetadata::empty()
        }

        fn get_model_config(&self) -> ModelConfig {
            self.model_config.clone()
        }

        async fn complete(
            &self,
            _system: &str,
            _messages: &[Message],
            _tools: &[Tool],
        ) -> Result<(Message, ProviderUsage), ProviderError> {
            Ok((
                Message::new(
                    Role::Assistant,
                    Utc::now().timestamp(),
                    vec![MessageContent::Text(
                        RawTextContent {
                            text: "Summarized content".to_string(),
                        }
                        .no_annotation(),
                    )],
                ),
                ProviderUsage::new(
                    "mock".to_string(),
                    Usage {
                        input_tokens: Some(100),
                        output_tokens: Some(50),
                        total_tokens: Some(150),
                    },
                ),
            ))
        }
    }

    fn create_mock_provider() -> Result<Arc<dyn Provider>> {
        let mock_model_config = ModelConfig::new("test-model")?.with_context_limit(Some(200_000));

        Ok(Arc::new(MockProvider {
            model_config: mock_model_config,
        }))
    }

    fn create_test_messages() -> Vec<Message> {
        vec![
            set_up_text_message("Message 1", Role::User),
            set_up_text_message("Message 2", Role::Assistant),
            set_up_text_message("Message 3", Role::User),
        ]
    }

    fn set_up_text_message(text: &str, role: Role) -> Message {
        Message::new(role, 0, vec![MessageContent::text(text.to_string())])
    }

    #[tokio::test]
    async fn test_summarize_messages_basic() {
        let provider = create_mock_provider().expect("failed to create mock provider");
        let messages = create_test_messages();

        let result = summarize_messages(Arc::clone(&provider), &messages).await;

        assert!(result.is_ok(), "The function should return Ok.");
        let summary_result = result.unwrap();

        assert!(
            summary_result.is_some(),
            "The summary should contain a result."
        );
        let (summarized_message, input_tokens, output_tokens) = summary_result.unwrap();

        assert_eq!(
            summarized_message.role,
            Role::User,
            "The summarized message should be from the user."
        );
        assert!(input_tokens > 0, "Should have input token count");
        assert!(output_tokens > 0, "Should have output token count");
    }

    #[tokio::test]
    async fn test_summarize_messages_empty_input() {
        let provider = create_mock_provider().expect("failed to create mock provider");
        let messages: Vec<Message> = Vec::new();

        let result = summarize_messages(Arc::clone(&provider), &messages).await;

        assert!(result.is_ok(), "The function should return Ok.");
        let summary_result = result.unwrap();

        assert!(
            summary_result.is_none(),
            "The summary should be None for empty input."
        );
    }
}
