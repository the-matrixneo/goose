// goose/src/config/schema.rs
//! Type-safe configuration schema for Goose
//!
//! This module provides strongly-typed configuration structures for all Goose configuration options.
//! It includes custom types like SecretString that prevent sensitive data from being logged.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

/// Custom type for secret strings that won't leak in Debug output
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(s: impl Into<String>) -> Self {
        SecretString(s.into())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretString(***)")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}

/// Main configuration structure for Goose
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GooseConfig {
    /// Provider configuration
    pub provider: ProviderConfig,

    /// Model configuration
    pub model: ModelConfig,

    /// Session configuration
    pub session: SessionConfig,

    /// CLI-specific configuration
    pub cli: CliConfig,

    /// Server configuration
    pub server: ServerConfig,

    /// Extension configurations
    pub extensions: ExtensionConfig,

    /// Experimental features configuration
    pub experiments: ExperimentConfig,

    /// Permission configuration
    pub permissions: PermissionConfig,

    /// System configuration
    pub system: SystemConfig,

    /// Context management configuration
    pub context: ContextConfig,

    /// Scheduler configuration
    pub scheduler: SchedulerConfig,

    /// Recipe configuration
    pub recipe: RecipeConfig,

    /// Router configuration
    pub router: RouterConfig,
}

/// Provider configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderConfig {
    /// Active provider name (e.g., "openai", "anthropic", "groq")
    pub goose_provider: Option<String>,

    /// OpenAI configuration
    pub openai: OpenAIConfig,

    /// Anthropic configuration
    pub anthropic: AnthropicConfig,

    /// Groq configuration
    pub groq: GroqConfig,

    /// XAI configuration
    pub xai: XAIConfig,

    /// Snowflake configuration
    pub snowflake: SnowflakeConfig,

    /// LiteLLM configuration
    pub litellm: LiteLLMConfig,

    /// Venice configuration
    pub venice: VeniceConfig,

    /// Azure OpenAI configuration
    pub azure_openai: AzureOpenAIConfig,

    /// Google/Gemini configuration
    pub google: GoogleConfig,

    /// Databricks configuration
    pub databricks: DatabricksConfig,

    /// OpenRouter configuration
    pub openrouter: OpenRouterConfig,

    /// Ollama configuration
    pub ollama: OllamaConfig,

    /// SageMaker configuration
    pub sagemaker: SageMakerConfig,
}

/// OpenAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenAIConfig {
    pub api_key: SecretString,
    pub host: String,
    pub base_path: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub custom_headers: Option<HashMap<String, String>>,
    pub timeout: u64,
}

/// Anthropic provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AnthropicConfig {
    pub api_key: SecretString,
    pub host: String,
    pub api_version: Option<String>,
}

/// Groq provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GroqConfig {
    pub api_key: SecretString,
    pub host: String,
}

/// XAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct XAIConfig {
    pub api_key: SecretString,
    pub host: String,
}

/// Snowflake provider configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SnowflakeConfig {
    pub host: SecretString,
    pub token: SecretString,
}

/// LiteLLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LiteLLMConfig {
    pub api_key: SecretString,
    pub host: String,
    pub base_path: String,
    pub custom_headers: Option<HashMap<String, String>>,
    pub timeout: u64,
}

/// Venice provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VeniceConfig {
    pub api_key: SecretString,
    pub host: String,
    pub base_path: String,
    pub models_path: String,
}

/// Azure OpenAI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AzureOpenAIConfig {
    pub api_key: SecretString,
    pub endpoint: String,
    pub deployment_name: String,
    pub api_version: String,
}

/// Google/Gemini configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GoogleConfig {
    pub api_key: SecretString,
    pub host: String,
    pub project_id: Option<String>,
    pub location: Option<String>,
}

/// Databricks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabricksConfig {
    pub host: SecretString,
    pub token: SecretString,
    pub max_retries: u32,
    pub initial_retry_interval_ms: u64,
    pub max_retry_interval_ms: u64,
    pub backoff_multiplier: f32,
}

/// OpenRouter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenRouterConfig {
    pub api_key: SecretString,
    pub host: String,
}

/// Ollama configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OllamaConfig {
    pub host: String,
    pub timeout: u64,
}

/// SageMaker configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SageMakerConfig {
    pub endpoint_name: String,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModelConfig {
    /// Active model name
    pub goose_model: Option<String>,

    /// Context window limit
    pub context_limit: Option<usize>,

    /// Temperature for generation
    pub temperature: Option<f32>,

    /// Maximum tokens to generate
    pub max_tokens: Option<i32>,

    /// Enable tool shimming for models without native tool support
    pub toolshim: bool,

    /// Ollama model to use for tool shimming
    pub toolshim_ollama_model: Option<String>,

    /// Embedding model name
    pub embedding_model: Option<String>,

    /// Embedding model provider
    pub embedding_model_provider: Option<String>,

    /// Lead model configuration for multi-agent scenarios
    pub lead_model: Option<String>,
    pub lead_provider: Option<String>,
    pub lead_context_limit: Option<usize>,
    pub lead_turns: Option<u32>,
    pub lead_fallback_turns: Option<u32>,
    pub lead_failure_threshold: Option<u32>,

    /// Planner model configuration
    pub planner_model: Option<String>,
    pub planner_provider: Option<String>,
    pub planner_context_limit: Option<usize>,

    /// Worker model configuration
    pub worker_context_limit: Option<usize>,
}

/// Session configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionConfig {
    /// Session name or identifier
    pub session_name: Option<String>,

    /// Working directory for sessions
    pub working_dir: Option<PathBuf>,

    /// Maximum number of turns in a session
    pub max_turns: Option<u32>,

    /// Maximum turns for sub-agents
    pub subagent_max_turns: Option<u32>,

    /// Resume previous session
    pub resume: bool,

    /// Run without saving session
    pub no_session: bool,
}

/// CLI-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CliConfig {
    /// Color theme (light, dark, ansi)
    pub theme: String,

    /// Show cost information
    pub show_cost: bool,

    /// Show thinking/reasoning output
    pub show_thinking: bool,

    /// Minimum priority for messages to display
    pub min_priority: f32,

    /// Maximum length for tool parameter truncation
    pub tool_params_truncation_max_length: usize,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// Server host
    pub host: String,

    /// Server port
    pub port: u16,

    /// Secret key for authentication
    pub secret_key: SecretString,
}

/// Extension configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtensionConfig {
    /// List of enabled extensions
    pub enabled: Vec<String>,

    /// Extension-specific configurations
    pub configs: HashMap<String, ExtensionInstanceConfig>,

    /// Override extensions list
    pub extensions_override: Option<Vec<String>>,
}

/// Individual extension instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtensionInstanceConfig {
    pub name: String,
    pub enabled: bool,
    pub command: Option<String>,
    pub uri: Option<String>,
    pub env_vars: HashMap<String, String>,
    pub timeout: u64,
}

/// Experimental features configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExperimentConfig {
    /// Enable Claude thinking/reasoning
    pub claude_thinking_enabled: bool,

    /// Enable Claude code debugging
    pub claude_code_debug: bool,

    /// Enable Gemini CLI debugging
    pub gemini_cli_debug: bool,

    /// Random thinking messages for testing
    pub random_thinking_messages: bool,
}

/// Permission configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PermissionConfig {
    /// Tools that are always allowed
    pub always_allow: Vec<String>,

    /// Tools that require user confirmation
    pub ask_before: Vec<String>,

    /// Tools that are never allowed
    pub never_allow: Vec<String>,

    /// Allowlist of allowed operations
    pub allowlist: Option<Vec<String>>,

    /// Bypass allowlist checking
    pub allowlist_bypass: bool,
}

/// System configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemConfig {
    /// Custom system prompt file path
    pub system_prompt_file_path: Option<PathBuf>,

    /// Additional system prompt text
    pub additional_system_prompt: Option<String>,

    /// Cache directory
    pub cache_dir: Option<PathBuf>,

    /// Vector database path
    pub vector_db_path: Option<PathBuf>,

    /// Disable keyring usage
    pub disable_keyring: bool,

    /// Operating mode
    pub mode: Option<String>,
}

/// Context management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextConfig {
    /// Context management strategy
    pub strategy: String,

    /// Auto-compaction threshold (0.0 to 1.0)
    pub auto_compact_threshold: f32,
}

/// Scheduler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SchedulerConfig {
    /// Scheduler type (e.g., "temporal", "simple")
    pub scheduler_type: String,

    /// Path to Temporal binary
    pub temporal_bin: Option<PathBuf>,
}

/// Recipe configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RecipeConfig {
    /// Recipe path or directory
    pub recipe_path: Option<PathBuf>,

    /// GitHub repository for recipes
    pub github_repo: Option<String>,

    /// Retry timeout in seconds
    pub retry_timeout_seconds: u64,

    /// On-failure timeout in seconds
    pub on_failure_timeout_seconds: u64,
}

/// Router configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RouterConfig {
    /// Router strategy
    pub strategy: String,

    /// Tool selection strategy
    pub tool_selection_strategy: String,
}

// Default implementations

impl Default for OpenAIConfig {
    fn default() -> Self {
        OpenAIConfig {
            api_key: SecretString::default(),
            host: "https://api.openai.com".to_string(),
            base_path: "v1/chat/completions".to_string(),
            organization: None,
            project: None,
            custom_headers: None,
            timeout: 600,
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        AnthropicConfig {
            api_key: SecretString::default(),
            host: "https://api.anthropic.com".to_string(),
            api_version: None,
        }
    }
}

impl Default for GroqConfig {
    fn default() -> Self {
        GroqConfig {
            api_key: SecretString::default(),
            host: "https://api.groq.com".to_string(),
        }
    }
}

impl Default for XAIConfig {
    fn default() -> Self {
        XAIConfig {
            api_key: SecretString::default(),
            host: "https://api.x.ai".to_string(),
        }
    }
}

impl Default for LiteLLMConfig {
    fn default() -> Self {
        LiteLLMConfig {
            api_key: SecretString::default(),
            host: "https://api.litellm.ai".to_string(),
            base_path: "v1/chat/completions".to_string(),
            custom_headers: None,
            timeout: 600,
        }
    }
}

impl Default for VeniceConfig {
    fn default() -> Self {
        VeniceConfig {
            api_key: SecretString::default(),
            host: "https://api.venice.ai".to_string(),
            base_path: "api".to_string(),
            models_path: "api/models".to_string(),
        }
    }
}

impl Default for AzureOpenAIConfig {
    fn default() -> Self {
        AzureOpenAIConfig {
            api_key: SecretString::default(),
            endpoint: String::new(),
            deployment_name: String::new(),
            api_version: "2024-02-01".to_string(),
        }
    }
}

impl Default for GoogleConfig {
    fn default() -> Self {
        GoogleConfig {
            api_key: SecretString::default(),
            host: "https://generativelanguage.googleapis.com".to_string(),
            project_id: None,
            location: None,
        }
    }
}

impl Default for DatabricksConfig {
    fn default() -> Self {
        DatabricksConfig {
            host: SecretString::default(),
            token: SecretString::default(),
            max_retries: 3,
            initial_retry_interval_ms: 1000,
            max_retry_interval_ms: 60000,
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        OpenRouterConfig {
            api_key: SecretString::default(),
            host: "https://openrouter.ai".to_string(),
        }
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        OllamaConfig {
            host: "http://localhost:11434".to_string(),
            timeout: 600,
        }
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            goose_model: None,
            context_limit: None,
            temperature: None,
            max_tokens: None,
            toolshim: false,
            toolshim_ollama_model: None,
            embedding_model: Some("text-embedding-3-small".to_string()),
            embedding_model_provider: None,
            lead_model: None,
            lead_provider: None,
            lead_context_limit: None,
            lead_turns: None,
            lead_fallback_turns: None,
            lead_failure_threshold: None,
            planner_model: None,
            planner_provider: None,
            planner_context_limit: None,
            worker_context_limit: None,
        }
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            theme: "dark".to_string(),
            show_cost: false,
            show_thinking: false,
            min_priority: 0.5,
            tool_params_truncation_max_length: 40,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            secret_key: SecretString::default(),
        }
    }
}

impl Default for ExtensionInstanceConfig {
    fn default() -> Self {
        ExtensionInstanceConfig {
            name: String::new(),
            enabled: false,
            command: None,
            uri: None,
            env_vars: HashMap::new(),
            timeout: 30,
        }
    }
}

impl Default for ContextConfig {
    fn default() -> Self {
        ContextConfig {
            strategy: "auto".to_string(),
            auto_compact_threshold: 0.8,
        }
    }
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        SchedulerConfig {
            scheduler_type: "simple".to_string(),
            temporal_bin: None,
        }
    }
}

impl Default for RecipeConfig {
    fn default() -> Self {
        RecipeConfig {
            recipe_path: None,
            github_repo: None,
            retry_timeout_seconds: 300,
            on_failure_timeout_seconds: 600,
        }
    }
}

impl Default for RouterConfig {
    fn default() -> Self {
        RouterConfig {
            strategy: "default".to_string(),
            tool_selection_strategy: "default".to_string(),
        }
    }
}

// Helper methods for GooseConfig
impl GooseConfig {
    /// Load configuration from a YAML file
    pub fn from_file(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: GooseConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to a YAML file
    pub fn to_file(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_yaml::to_string(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Merge with environment variables
    /// Environment variables take precedence over file configuration
    pub fn merge_env(&mut self) {
        // This would be implemented to check for environment variables
        // and override the corresponding fields in the config
        // For example:
        // if let Ok(provider) = std::env::var("GOOSE_PROVIDER") {
        //     self.provider.goose_provider = Some(provider);
        // }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate temperature if set
        if let Some(temp) = self.model.temperature {
            if !(0.0..=2.0).contains(&temp) {
                errors.push(format!(
                    "Temperature must be between 0.0 and 2.0, got {}",
                    temp
                ));
            }
        }

        // Validate context limit if set
        if let Some(limit) = self.model.context_limit {
            if limit == 0 {
                errors.push("Context limit must be greater than 0".to_string());
            }
        }

        // Validate auto-compact threshold
        if self.context.auto_compact_threshold < 0.0 || self.context.auto_compact_threshold > 1.0 {
            errors.push(format!(
                "Auto-compact threshold must be between 0.0 and 1.0, got {}",
                self.context.auto_compact_threshold
            ));
        }

        // Validate CLI min priority
        if self.cli.min_priority < 0.0 || self.cli.min_priority > 1.0 {
            errors.push(format!(
                "CLI min priority must be between 0.0 and 1.0, got {}",
                self.cli.min_priority
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get the active provider configuration
    pub fn get_active_provider(&self) -> Option<&str> {
        self.provider.goose_provider.as_deref()
    }

    /// Get the active model name
    pub fn get_active_model(&self) -> Option<&str> {
        self.model.goose_model.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_string_debug() {
        let secret = SecretString::new("sensitive_data");
        let debug_output = format!("{:?}", secret);
        assert_eq!(debug_output, "SecretString(***)");
        assert!(!debug_output.contains("sensitive_data"));
    }

    #[test]
    fn test_secret_string_display() {
        let secret = SecretString::new("sensitive_data");
        let display_output = format!("{}", secret);
        assert_eq!(display_output, "***");
        assert!(!display_output.contains("sensitive_data"));
    }

    #[test]
    fn test_default_config() {
        let config = GooseConfig::default();
        assert!(config.provider.goose_provider.is_none());
        assert!(config.model.goose_model.is_none());
        assert_eq!(config.cli.theme, "dark");
        assert_eq!(config.cli.min_priority, 0.5);
        assert_eq!(config.context.auto_compact_threshold, 0.8);
    }

    #[test]
    fn test_config_validation() {
        let mut config = GooseConfig::default();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // Invalid temperature
        config.model.temperature = Some(3.0);
        let result = config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Temperature")));

        // Reset and test invalid context limit
        config.model.temperature = Some(1.0);
        config.model.context_limit = Some(0);
        let result = config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Context limit")));

        // Reset and test invalid auto-compact threshold
        config.model.context_limit = Some(1000);
        config.context.auto_compact_threshold = 1.5;
        let result = config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Auto-compact threshold")));
    }

    #[test]
    fn test_serialization_deserialization() {
        let config = GooseConfig::default();

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&config).unwrap();

        // Deserialize back
        let deserialized: GooseConfig = serde_yaml::from_str(&yaml).unwrap();

        // Check some fields match
        assert_eq!(config.cli.theme, deserialized.cli.theme);
        assert_eq!(
            config.context.auto_compact_threshold,
            deserialized.context.auto_compact_threshold
        );
        assert_eq!(
            config.model.embedding_model,
            deserialized.model.embedding_model
        );
    }

    #[test]
    fn test_partial_deserialization() {
        // Test that we can deserialize partial configs
        let yaml = r#"
provider:
  goose_provider: openai
  openai:
    api_key: test_key
model:
  goose_model: gpt-4
  temperature: 0.7
"#;

        let config: GooseConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.provider.goose_provider, Some("openai".to_string()));
        assert_eq!(config.model.goose_model, Some("gpt-4".to_string()));
        assert_eq!(config.model.temperature, Some(0.7));

        // Check defaults are still applied
        assert_eq!(config.cli.theme, "dark");
        assert_eq!(config.context.auto_compact_threshold, 0.8);
    }
}
