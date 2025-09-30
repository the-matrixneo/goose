use async_trait::async_trait;
use goose::conversation::message::Message;
use goose::model::ModelConfig;
use goose::providers::base::{Provider, ProviderMetadata, ProviderUsage};
use goose::providers::errors::ProviderError;
use rmcp::model::Tool;

#[derive(Clone)]
struct DummyProvider;

#[async_trait]
impl Provider for DummyProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "dummy",
            "Dummy Provider",
            "A dummy provider for testing",
            "dummy-model",
            vec!["dummy-model"],
            "",
            vec![],
        )
    }

    async fn complete_with_model(
        &self,
        _model_config: &ModelConfig,
        _system: &str,
        _messages: &[Message],
        _tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        Err(ProviderError::NotImplemented(
            "not used in this test".to_string(),
        ))
    }

    fn get_model_config(&self) -> ModelConfig {
        ModelConfig::new_or_fail("dummy-model")
    }
}

#[test]
fn test_create_session_name_prompt_includes_context_and_instructions() {
    let provider = DummyProvider;
    let context = vec![
        "User: Help me summarize a report".to_string(),
        "User: The report is about Q3 sales".to_string(),
    ];

    let prompt = provider.create_session_name_prompt(&context);

    let expected_instructions = "Based on the conversation so far, provide a concise description of this session in 4 words or less. This will be used for finding the session later in a UI with limited space - reply *ONLY* with the description";
    let expected = format!(
        "Here are the first few user messages:\n{}\n\n{}",
        context.join("\n"),
        expected_instructions
    );

    assert_eq!(prompt, expected);
}
