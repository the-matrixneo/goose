use super::common::get_messages_token_counts;
use crate::message::Message;
use crate::prompt_template::render_global_file;
use crate::providers::base::Provider;
use crate::token_counter::TokenCounter;
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
    token_counter: &TokenCounter,
    _context_limit: usize,
) -> Result<(Vec<Message>, Vec<usize>), anyhow::Error> {
    if messages.is_empty() {
        return Ok((vec![], vec![]));
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

    // Render the detailed summarization prompt
    let system_prompt = render_global_file("summarize_oneshot.md", &context)?;

    // Create a simple user message requesting summarization
    let user_message = Message::user()
        .with_text("Please summarize the conversation history provided in the system prompt.");
    let summarization_request = vec![user_message];

    // Send the request to the provider and fetch the response
    let mut response = provider
        .complete(&system_prompt, &summarization_request, &[])
        .await?
        .0;

    // Set role to user as it will be used in following conversation as user content
    response.role = Role::User;

    let final_summary = vec![response];

    Ok((
        final_summary.clone(),
        get_messages_token_counts(token_counter, &final_summary),
    ))
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

    fn create_mock_provider() -> Arc<dyn Provider> {
        let mock_model_config =
            ModelConfig::new("test-model".to_string()).with_context_limit(200_000.into());
        Arc::new(MockProvider {
            model_config: mock_model_config,
        })
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
        let token_counter = TokenCounter::new();
        let context_limit = 10_000;
        let messages = create_test_messages();

        let result = summarize_messages(
            Arc::clone(&provider),
            &messages,
            &token_counter,
            context_limit,
        )
        .await;

        assert!(result.is_ok(), "The function should return Ok.");
        let (summarized_messages, token_counts) = result.unwrap();

        assert_eq!(
            summarized_messages.len(),
            1,
            "The summary should contain one message."
        );
        assert_eq!(
            summarized_messages[0].role,
            Role::User,
            "The summarized message should be from the user."
        );

        assert_eq!(
            token_counts.len(),
            1,
            "Token counts should match the number of summarized messages."
        );
    }

    #[tokio::test]
    async fn test_summarize_messages_empty_input() {
        let provider = create_mock_provider();
        let token_counter = TokenCounter::new();
        let context_limit = 10_000;
        let messages: Vec<Message> = Vec::new();

        let result = summarize_messages(
            Arc::clone(&provider),
            &messages,
            &token_counter,
            context_limit,
        )
        .await;

        assert!(result.is_ok(), "The function should return Ok.");
        let (summarized_messages, token_counts) = result.unwrap();

        assert_eq!(
            summarized_messages.len(),
            0,
            "The summary should be empty for an empty input."
        );
        assert!(
            token_counts.is_empty(),
            "Token counts should be empty for an empty input."
        );
    }
}
