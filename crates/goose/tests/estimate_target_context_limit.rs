use std::sync::Arc;

use goose::context_mgmt::estimate_target_context_limit;
use goose::model::ModelConfig;
use goose::providers::base::{Provider, ProviderMetadata, ProviderUsage};
use goose::providers::errors::ProviderError;
use goose::conversation::message::Message;
use rmcp::model::Tool;
use async_trait::async_trait;

#[derive(Clone)]
struct MockProvider {
    cfg: ModelConfig,
}

#[async_trait]
impl Provider for MockProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::empty()
    }

    async fn complete_with_model(
        &self,
        _model_config: &ModelConfig,
        _system: &str,
        _messages: &[Message],
        _tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        Err(ProviderError::NotImplemented("not needed in test".to_string()))
    }

    fn get_model_config(&self) -> ModelConfig {
        self.cfg.clone()
    }
}

#[test]
fn test_estimate_target_context_limit_various_cases() {
    // Case 1: Known model "gpt-4o" with default limit 128k
    // target = floor(128000 * 0.7) = 89600; minus overhead (3000+5000) = 81600
    let provider1 = Arc::new(MockProvider { cfg: ModelConfig::new_or_fail("gpt-4o") });
    let est1 = estimate_target_context_limit(provider1);
    assert_eq!(est1, 81_600);

    // Case 2: Explicit small context limit where overhead exceeds target
    // context_limit = 6000 -> target = floor(6000 * 0.7) = 4200
    // overhead = 8000, overhead > target -> return max(target/2, 1000) = 2100
    let small_cfg = ModelConfig::new_or_fail("unknown-model").with_context_limit(Some(6000));
    let provider2 = Arc::new(MockProvider { cfg: small_cfg });
    let est2 = estimate_target_context_limit(provider2);
    assert_eq!(est2, 2100);

    // Case 3: Model with 8192 limit (e.g., gemma-2-27b)
    // target = floor(8192 * 0.7) = 5734; overhead 8000 > target -> max(5734/2, 1000) = 2867
    let provider3 = Arc::new(MockProvider { cfg: ModelConfig::new_or_fail("gemma-2-27b") });
    let est3 = estimate_target_context_limit(provider3);
    assert_eq!(est3, 2867);
}
