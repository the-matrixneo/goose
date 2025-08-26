use std::sync::{Arc, RwLock};

use super::{
    anthropic::AnthropicProvider,
    azure::AzureProvider,
    base::{Provider, ProviderMetadata},
    bedrock::BedrockProvider,
    claude_code::ClaudeCodeProvider,
    cursor_agent::CursorAgentProvider,
    databricks::DatabricksProvider,
    gcpvertexai::GcpVertexAIProvider,
    gemini_cli::GeminiCliProvider,
    githubcopilot::GithubCopilotProvider,
    google::GoogleProvider,
    groq::GroqProvider,
    lead_worker::LeadWorkerProvider,
    litellm::LiteLLMProvider,
    ollama::OllamaProvider,
    openai::OpenAiProvider,
    openrouter::OpenRouterProvider,
    provider_registry::ProviderRegistry,
    sagemaker_tgi::SageMakerTgiProvider,
    snowflake::SnowflakeProvider,
    tetrate::TetrateProvider,
    venice::VeniceProvider,
    xai::XaiProvider,
};
use crate::config::custom_providers::{custom_providers_dir, register_custom_providers};
use crate::model::ModelConfig;
use anyhow::Result;
use once_cell::sync::Lazy;

#[cfg(test)]
use super::errors::ProviderError;
#[cfg(test)]
use rmcp::model::Tool;

const DEFAULT_LEAD_TURNS: usize = 3;
const DEFAULT_FAILURE_THRESHOLD: usize = 2;
const DEFAULT_FALLBACK_TURNS: usize = 2;

static REGISTRY: Lazy<RwLock<ProviderRegistry>> = Lazy::new(|| {
    let registry = ProviderRegistry::new().with_providers(|registry| {
        registry.register::<AnthropicProvider, _>(AnthropicProvider::from_env);
        registry.register::<AzureProvider, _>(AzureProvider::from_env);
        registry.register::<BedrockProvider, _>(BedrockProvider::from_env);
        registry.register::<ClaudeCodeProvider, _>(ClaudeCodeProvider::from_env);
        registry.register::<CursorAgentProvider, _>(CursorAgentProvider::from_env);
        registry.register::<DatabricksProvider, _>(DatabricksProvider::from_env);
        registry.register::<GcpVertexAIProvider, _>(GcpVertexAIProvider::from_env);
        registry.register::<GeminiCliProvider, _>(GeminiCliProvider::from_env);
        registry.register::<GithubCopilotProvider, _>(GithubCopilotProvider::from_env);
        registry.register::<GoogleProvider, _>(GoogleProvider::from_env);
        registry.register::<GroqProvider, _>(GroqProvider::from_env);
        registry.register::<LiteLLMProvider, _>(LiteLLMProvider::from_env);
        registry.register::<OllamaProvider, _>(OllamaProvider::from_env);
        registry.register::<OpenAiProvider, _>(OpenAiProvider::from_env);
        registry.register::<OpenRouterProvider, _>(OpenRouterProvider::from_env);
        registry.register::<SageMakerTgiProvider, _>(SageMakerTgiProvider::from_env);
        registry.register::<SnowflakeProvider, _>(SnowflakeProvider::from_env);
        registry.register::<TetrateProvider, _>(TetrateProvider::from_env);
        registry.register::<VeniceProvider, _>(VeniceProvider::from_env);
        registry.register::<XaiProvider, _>(XaiProvider::from_env);

        if let Err(e) = load_custom_providers_into_registry(registry) {
            tracing::warn!("Failed to load custom providers: {}", e);
        }
    });
    RwLock::new(registry)
});

fn load_custom_providers_into_registry(registry: &mut ProviderRegistry) -> Result<()> {
    let config_dir = custom_providers_dir();
    register_custom_providers(registry, &config_dir)
}

pub fn providers() -> Vec<ProviderMetadata> {
    REGISTRY.read().unwrap().all_metadata()
}

pub fn refresh_custom_providers() -> Result<()> {
    let mut registry = REGISTRY.write().unwrap();
    registry.remove_custom_providers();

    if let Err(e) = load_custom_providers_into_registry(&mut registry) {
        tracing::warn!("Failed to refresh custom providers: {}", e);
        return Err(e);
    }

    tracing::info!("Custom providers refreshed");
    Ok(())
}

/// Creates a provider with pre-validation of models.
/// If the fast_model is invalid, it will be cleared and the provider recreated without it.
pub async fn create(name: &str, mut model: ModelConfig) -> Result<Arc<dyn Provider>> {
    let config = crate::config::Config::global();

    // Check for lead/worker configuration first
    if let Ok(lead_model_name) = config.get_param::<String>("GOOSE_LEAD_MODEL") {
        tracing::info!("Creating lead/worker provider from environment variables");
        return create_lead_worker_from_env(name, &model, &lead_model_name);
    }

    // Create the provider with the original config
    let provider = REGISTRY.read().unwrap().create(name, model.clone())?;

    // Validate the main model
    match provider.validate_model(&model.model_name).await {
        Ok(()) => {
            tracing::debug!("Main model '{}' validated successfully", model.model_name);
        }
        Err(super::errors::ProviderError::ConfigurationError(msg)) => {
            return Err(anyhow::anyhow!(
                "Model '{}' is not available for provider '{}': {}",
                model.model_name,
                name,
                msg
            ));
        }
        Err(e) => {
            tracing::warn!(
                "Could not validate main model '{}': {}",
                model.model_name,
                e
            );
            // Continue anyway - provider might not support validation
        }
    }

    // Validate fast_model if present
    if let Some(ref fast_model_name) = model.fast_model {
        match provider.validate_model(fast_model_name).await {
            Ok(()) => {
                tracing::debug!("Fast model '{}' validated successfully", fast_model_name);
                Ok(provider)
            }
            Err(super::errors::ProviderError::ConfigurationError(msg)) => {
                tracing::warn!(
                    "Fast model '{}' is not available for provider '{}': {}. Disabling fast model.",
                    fast_model_name,
                    name,
                    msg
                );
                // Clear the fast_model and recreate the provider without validation
                // (to avoid infinite recursion)
                model.fast_model = None;
                Ok(REGISTRY.read().unwrap().create(name, model)?)
            }
            Err(e) => {
                tracing::warn!("Could not validate fast model '{}': {}", fast_model_name, e);
                // Continue with fast_model - provider might not support validation
                Ok(provider)
            }
        }
    } else {
        Ok(provider)
    }
}

fn create_lead_worker_from_env(
    default_provider_name: &str,
    default_model: &ModelConfig,
    lead_model_name: &str,
) -> Result<Arc<dyn Provider>> {
    let config = crate::config::Config::global();

    let lead_provider_name = config
        .get_param::<String>("GOOSE_LEAD_PROVIDER")
        .unwrap_or_else(|_| default_provider_name.to_string());

    let lead_turns = config
        .get_param::<usize>("GOOSE_LEAD_TURNS")
        .unwrap_or(DEFAULT_LEAD_TURNS);
    let failure_threshold = config
        .get_param::<usize>("GOOSE_LEAD_FAILURE_THRESHOLD")
        .unwrap_or(DEFAULT_FAILURE_THRESHOLD);
    let fallback_turns = config
        .get_param::<usize>("GOOSE_LEAD_FALLBACK_TURNS")
        .unwrap_or(DEFAULT_FALLBACK_TURNS);

    let lead_model_config = ModelConfig::new_with_context_env(
        lead_model_name.to_string(),
        Some("GOOSE_LEAD_CONTEXT_LIMIT"),
    )?;

    let worker_model_config = create_worker_model_config(default_model)?;

    let lead_provider = REGISTRY
        .read()
        .unwrap()
        .create(&lead_provider_name, lead_model_config)?;
    let worker_provider = REGISTRY
        .read()
        .unwrap()
        .create(default_provider_name, worker_model_config)?;

    Ok(Arc::new(LeadWorkerProvider::new_with_settings(
        lead_provider,
        worker_provider,
        lead_turns,
        failure_threshold,
        fallback_turns,
    )))
}

fn create_worker_model_config(default_model: &ModelConfig) -> Result<ModelConfig> {
    let mut worker_config = ModelConfig::new_or_fail(&default_model.model_name)
        .with_context_limit(default_model.context_limit)
        .with_temperature(default_model.temperature)
        .with_max_tokens(default_model.max_tokens)
        .with_toolshim(default_model.toolshim)
        .with_toolshim_model(default_model.toolshim_model.clone());

    let global_config = crate::config::Config::global();

    if let Ok(limit_str) = global_config.get_param::<String>("GOOSE_WORKER_CONTEXT_LIMIT") {
        if let Ok(limit) = limit_str.parse::<usize>() {
            worker_config = worker_config.with_context_limit(Some(limit));
        }
    } else if let Ok(limit_str) = global_config.get_param::<String>("GOOSE_CONTEXT_LIMIT") {
        if let Ok(limit) = limit_str.parse::<usize>() {
            worker_config = worker_config.with_context_limit(Some(limit));
        }
    }

    Ok(worker_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::message::{Message, MessageContent};
    use crate::providers::base::{ProviderMetadata, ProviderUsage, Usage};
    use chrono::Utc;
    use rmcp::model::{AnnotateAble, RawTextContent, Role};
    use std::env;

    #[derive(Clone)]
    struct MockTestProvider {
        name: String,
        model_config: ModelConfig,
    }

    #[async_trait::async_trait]
    impl Provider for MockTestProvider {
        fn metadata() -> ProviderMetadata {
            ProviderMetadata::new(
                "mock_test",
                "Mock Test Provider",
                "A mock provider for testing",
                "mock-model",
                vec!["mock-model"],
                "",
                vec![],
            )
        }

        fn get_model_config(&self) -> ModelConfig {
            self.model_config.clone()
        }

        async fn complete_with_model(
            &self,
            _model_config: &ModelConfig,
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
                            text: format!(
                                "Response from {} with model {}",
                                self.name, self.model_config.model_name
                            ),
                        }
                        .no_annotation(),
                    )],
                ),
                ProviderUsage::new(self.model_config.model_name.clone(), Usage::default()),
            ))
        }
    }

    struct EnvVarGuard {
        vars: Vec<(String, Option<String>)>,
    }

    impl EnvVarGuard {
        fn new(vars: &[&str]) -> Self {
            let saved_vars = vars
                .iter()
                .map(|&var| (var.to_string(), env::var(var).ok()))
                .collect();

            for &var in vars {
                env::remove_var(var);
            }

            Self { vars: saved_vars }
        }

        fn set(&self, key: &str, value: &str) {
            env::set_var(key, value);
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            for (key, value) in &self.vars {
                match value {
                    Some(val) => env::set_var(key, val),
                    None => env::remove_var(key),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_create_lead_worker_provider() {
        let _guard = EnvVarGuard::new(&[
            "GOOSE_LEAD_MODEL",
            "GOOSE_LEAD_PROVIDER",
            "GOOSE_LEAD_TURNS",
        ]);

        _guard.set("GOOSE_LEAD_MODEL", "gpt-4o");

        let gpt4mini_config = ModelConfig::new_or_fail("gpt-4o-mini");
        let result = create("openai", gpt4mini_config.clone()).await;

        match result {
            Ok(_) => {}
            Err(error) => {
                let error_msg = error.to_string();
                assert!(error_msg.contains("OPENAI_API_KEY") || error_msg.contains("secret"));
            }
        }

        _guard.set("GOOSE_LEAD_PROVIDER", "anthropic");
        _guard.set("GOOSE_LEAD_TURNS", "5");

        let _result = create("openai", gpt4mini_config).await;
    }

    #[tokio::test]
    async fn test_lead_model_env_vars_with_defaults() {
        let _guard = EnvVarGuard::new(&[
            "GOOSE_LEAD_MODEL",
            "GOOSE_LEAD_PROVIDER",
            "GOOSE_LEAD_TURNS",
            "GOOSE_LEAD_FAILURE_THRESHOLD",
            "GOOSE_LEAD_FALLBACK_TURNS",
        ]);

        _guard.set("GOOSE_LEAD_MODEL", "grok-3");

        let result = create("openai", ModelConfig::new_or_fail("gpt-4o-mini")).await;

        match result {
            Ok(_) => {}
            Err(error) => {
                let error_msg = error.to_string();
                assert!(error_msg.contains("OPENAI_API_KEY") || error_msg.contains("secret"));
            }
        }

        _guard.set("GOOSE_LEAD_TURNS", "7");
        _guard.set("GOOSE_LEAD_FAILURE_THRESHOLD", "4");
        _guard.set("GOOSE_LEAD_FALLBACK_TURNS", "3");

        let _result = create("openai", ModelConfig::new_or_fail("gpt-4o-mini")).await;
    }

    #[tokio::test]
    async fn test_create_regular_provider_without_lead_config() {
        let _guard = EnvVarGuard::new(&[
            "GOOSE_LEAD_MODEL",
            "GOOSE_LEAD_PROVIDER",
            "GOOSE_LEAD_TURNS",
            "GOOSE_LEAD_FAILURE_THRESHOLD",
            "GOOSE_LEAD_FALLBACK_TURNS",
        ]);

        let result = create("openai", ModelConfig::new_or_fail("gpt-4o-mini")).await;

        match result {
            Ok(_) => {}
            Err(error) => {
                let error_msg = error.to_string();
                assert!(error_msg.contains("OPENAI_API_KEY") || error_msg.contains("secret"));
            }
        }
    }

    #[test]
    fn test_worker_model_preserves_original_context_limit() {
        let _guard = EnvVarGuard::new(&[
            "GOOSE_LEAD_MODEL",
            "GOOSE_WORKER_CONTEXT_LIMIT",
            "GOOSE_CONTEXT_LIMIT",
        ]);

        _guard.set("GOOSE_LEAD_MODEL", "gpt-4o");

        let default_model =
            ModelConfig::new_or_fail("gpt-3.5-turbo").with_context_limit(Some(16_000));

        let result = create_lead_worker_from_env("openai", &default_model, "gpt-4o");

        _guard.set("GOOSE_WORKER_CONTEXT_LIMIT", "32000");
        let _result = create_lead_worker_from_env("openai", &default_model, "gpt-4o");

        _guard.set("GOOSE_CONTEXT_LIMIT", "64000");
        let _result = create_lead_worker_from_env("openai", &default_model, "gpt-4o");

        match result {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn test_fast_model_validation() {
        // Test with a mock provider that simulates validation failures
        struct MockValidatingProvider {
            model_config: ModelConfig,
            valid_models: Vec<String>,
        }

        #[async_trait::async_trait]
        impl Provider for MockValidatingProvider {
            fn metadata() -> ProviderMetadata {
                ProviderMetadata::new(
                    "mock_validating",
                    "Mock Validating Provider",
                    "A mock provider for testing model validation",
                    "valid-model",
                    vec!["valid-model", "another-valid-model"],
                    "",
                    vec![],
                )
            }

            fn get_model_config(&self) -> ModelConfig {
                self.model_config.clone()
            }

            async fn validate_model(&self, model_name: &str) -> Result<(), ProviderError> {
                if self.valid_models.contains(&model_name.to_string()) {
                    Ok(())
                } else {
                    Err(ProviderError::ConfigurationError(format!(
                        "Model '{}' is not available",
                        model_name
                    )))
                }
            }

            async fn complete_with_model(
                &self,
                _model_config: &ModelConfig,
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
                                text: "Mock response".to_string(),
                            }
                            .no_annotation(),
                        )],
                    ),
                    ProviderUsage::new(self.model_config.model_name.clone(), Usage::default()),
                ))
            }
        }

        // Register the mock provider temporarily
        {
            let mut registry = REGISTRY.write().unwrap();
            registry.register::<MockValidatingProvider, _>(|model: ModelConfig| {
                Ok(MockValidatingProvider {
                    model_config: model,
                    valid_models: vec![
                        "valid-model".to_string(),
                        "another-valid-model".to_string(),
                    ],
                })
            });
        }

        // Test 1: Valid main model with invalid fast model - fast model should be cleared
        let model_config =
            ModelConfig::new_or_fail("valid-model").with_fast("invalid-fast-model".to_string());

        let result = create("mock_validating", model_config).await;
        assert!(result.is_ok(), "Should succeed with cleared fast model");

        let provider = result.unwrap();
        let config = provider.get_model_config();
        assert!(
            config.fast_model.is_none(),
            "Fast model should be cleared when invalid"
        );

        // Test 2: Valid main model with valid fast model - both should be preserved
        let model_config =
            ModelConfig::new_or_fail("valid-model").with_fast("another-valid-model".to_string());

        let result = create("mock_validating", model_config).await;
        assert!(result.is_ok(), "Should succeed with valid fast model");

        let provider = result.unwrap();
        let config = provider.get_model_config();
        assert_eq!(
            config.fast_model,
            Some("another-valid-model".to_string()),
            "Valid fast model should be preserved"
        );

        // Test 3: Invalid main model - should fail
        let model_config = ModelConfig::new_or_fail("invalid-main-model");

        let result = create("mock_validating", model_config).await;
        assert!(result.is_err(), "Should fail with invalid main model");
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("not available"),
                "Error should mention model not available, got: {}",
                error_msg
            );
        }

        // Clean up - remove the mock provider
        {
            let mut registry = REGISTRY.write().unwrap();
            registry.remove_custom_providers();
        }
    }
}
