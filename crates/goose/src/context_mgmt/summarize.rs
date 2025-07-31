use crate::message::Message;
use crate::prompt_template::render_global_file;
use crate::providers::base::Provider;
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

    // Get the token count from the provider usage for the output tokens
    // For now, we'll use the output tokens as an approximation for the summary token count
    let input_tokens = provider_usage.usage.input_tokens.unwrap_or(0) as usize;
    let output_tokens = provider_usage.usage.output_tokens.unwrap_or(0) as usize;

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
                ProviderUsage::new("mock".to_string(), Usage::default()),
            ))
        }
    }

    fn create_mock_provider() -> Result<Arc<dyn Provider>> {
        let mock_model_config = ModelConfig::new("test-model")?.with_context_limit(200_000.into());

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
        let provider = create_mock_provider();
        let messages = create_test_messages();

        let result = summarize_messages(Arc::clone(&provider), &messages).await;

        assert!(result.is_ok(), "The function should return Ok.");
        let summary_result = result.unwrap();

        assert!(
            summary_result.is_some(),
            "The summary should contain a result."
        );
        let (summarized_message, _token_count) = summary_result.unwrap();

        assert_eq!(
            summarized_message.role,
            Role::User,
            "The summarized message should be from the user."
        );
    }

    #[tokio::test]
    async fn test_summarize_messages_empty_input() {
        let provider = create_mock_provider();
        let messages: Vec<Message> = Vec::new();

        let result = summarize_messages(Arc::clone(&provider), &messages).await;

        assert!(result.is_ok(), "The function should return Ok.");
        let summary_result = result.unwrap();

        assert!(
            summary_result.is_none(),
            "The summary should be None for empty input."
        );
        assert_eq!(
            summarized_messages[0].role,
            Role::User,
            "Summary should be from user role for context."
        );
        assert_eq!(
            token_counts.len(),
            1,
            "Should have token count for the summary."
        );
    }

    #[tokio::test]
    async fn test_summarize_messages_chunked_direct_call() {
        let provider = create_mock_provider().expect("failed to create mock provider");
        let token_counter = TokenCounter::new();
        let context_limit = 10_000; // Higher limit to avoid underflow
        let messages = create_test_messages();

        let result = summarize_messages_chunked(
            Arc::clone(&provider),
            &messages,
            &token_counter,
            context_limit,
        )
        .await;

        assert!(
            result.is_ok(),
            "Chunked summarization should work directly."
        );
        let (summarized_messages, token_counts) = result.unwrap();

        assert_eq!(
            summarized_messages.len(),
            1,
            "Chunked should return a single final summary."
        );
        assert_eq!(
            summarized_messages[0].role,
            Role::User,
            "Summary should be from user role for context."
        );
        assert_eq!(
            token_counts.len(),
            1,
            "Should have token count for the summary."
        );
    }

    #[tokio::test]
    async fn test_absolute_token_threshold_calculation() {
        let provider = create_mock_provider().expect("failed to create mock provider");
        let token_counter = TokenCounter::new();

        // Test with a context limit where absolute token calculation matters
        let context_limit = 10_000;
        let system_prompt_overhead = 1000;
        let response_overhead = 4000;
        let safety_buffer = 1000;
        let max_message_tokens =
            context_limit - system_prompt_overhead - response_overhead - safety_buffer; // 4000 tokens

        // Create messages that are just under the absolute threshold
        let mut large_messages = Vec::new();
        let base_message = set_up_text_message("x".repeat(50).as_str(), Role::User);

        // Add enough messages to approach but not exceed the absolute threshold
        let message_tokens = token_counter.count_tokens(&format!("{:?}", base_message));
        let num_messages = (max_message_tokens / message_tokens).saturating_sub(1);

        for i in 0..num_messages {
            large_messages.push(set_up_text_message(&format!("Message {}", i), Role::User));
        }

        let result = summarize_messages(
            Arc::clone(&provider),
            &large_messages,
        )
        .await;

        assert!(
            result.is_ok(),
            "Should handle absolute threshold calculation correctly."
        );
        let (summarized_messages, _) = result.unwrap();
        assert_eq!(summarized_messages.len(), 1, "Should produce a summary.");
    }
}
