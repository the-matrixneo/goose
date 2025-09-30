use std::sync::Arc;

use async_trait::async_trait;
use goose::context_mgmt::common::estimate_target_context_limit;
use goose::providers::base::{Provider, ProviderMetadata, ProviderUsage};
use goose::providers::errors::ProviderError;
use goose::model::ModelConfig;
use goose::conversation::message::Message;
use rmcp::model::Tool;

#[derive(Clone)]
struct DummyProvider(ModelConfig);

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
        self.0.clone()
    }
}

#[test]
fn test_estimate_target_context_limit_with_overhead() {
    // Model with 128k context limit
    let config = ModelConfig::new_or_fail("gpt-4o");
    let provider = Arc::new(DummyProvider(config));

    // 70% of 128000 is 89600; minus overhead (8000) = 81600
    let estimate = estimate_target_context_limit(provider);
    assert_eq!(estimate, 81_600);
}

#[test]
fn test_estimate_target_context_limit_minimum_when_small_limit() {
    // Create a model with a small explicit context limit (e.g., 6000)
    let config = ModelConfig::new_or_fail("gemma-2-2b").with_context_limit(Some(6_000));
    let provider = Arc::new(DummyProvider(config));

    // 70% of 6000 = 4200; overhead is 8000, so target<=overhead -> return max(target/2, 1000)
    // target/2 = 2100, which is > 1000, so expect 2100
    let estimate = estimate_target_context_limit(provider);
    assert_eq!(estimate, 2_100);
}
