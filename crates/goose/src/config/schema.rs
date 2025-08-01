//! Configuration schema for Goose
//! 
//! This module defines the complete configuration structure for Goose, including:
//! - Type-safe configuration with serde
//! - Environment variable mappings
//! - Secret handling
//! - Default values

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// The root configuration schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigSchema {
    /// Core Goose settings
    pub core: CoreConfig,
    
    /// Provider configurations
    pub providers: ProvidersConfig,
    
    /// Extension configurations
    pub extensions: ExtensionsConfig,
    
    /// UI settings (CLI and desktop)
    pub ui: UIConfig,
    
    /// Developer/debugging settings
    pub developer: DeveloperConfig,
    
    /// Scheduler settings
    pub scheduler: SchedulerConfig,
    
    /// Tracing/observability settings
    pub tracing: TracingConfig,
}

/// Core Goose configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// Active provider (e.g., "openai", "anthropic")
    #[serde(default = "default_provider")]
    pub provider: String,
    
    /// Active model (e.g., "gpt-4o", "claude-3-5-sonnet")
    #[serde(default = "default_model")]
    pub model: String,
    
    /// Goose mode (auto, fast_apply, etc.)
    #[serde(default = "default_mode")]
    pub mode: GooseMode,
    
    /// Context limit override
    #[serde(default)]
    pub context_limit: Option<usize>,
    
    /// Temperature override
    #[serde(default)]
    pub temperature: Option<f32>,
    
    /// Maximum turns in a conversation
    #[serde(default = "default_max_turns")]
    pub max_turns: u32,
    
    /// Context management strategy
    #[serde(default = "default_context_strategy")]
    pub context_strategy: String,
    
    /// Auto-compaction threshold
    #[serde(default = "default_auto_compact_threshold")]
    pub auto_compact_threshold: f64,
    
    /// System prompt file path
    #[serde(default)]
    pub system_prompt_file_path: Option<PathBuf>,
    
    /// Toolshim settings
    #[serde(default)]
    pub toolshim: ToolshimConfig,
    
    /// Lead-worker configuration
    #[serde(default)]
    pub lead_worker: LeadWorkerConfig,
    
    /// Router configuration
    #[serde(default)]
    pub router: RouterConfig,
    
    /// Recipe configuration
    #[serde(default)]
    pub recipe: RecipeConfig,
    
    /// Subagent configuration
    #[serde(default)]
    pub subagent: SubagentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolshimConfig {
    /// Whether toolshim is enabled
    #[serde(default)]
    pub enabled: bool,
    
    /// Ollama model for toolshim
    #[serde(default)]
    pub ollama_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeadWorkerConfig {
    /// Lead model provider
    #[serde(default)]
    pub provider: Option<String>,
    
    /// Lead model name
    #[serde(default)]
    pub model: Option<String>,
    
    /// Number of turns for lead model
    #[serde(default)]
    pub turns: Option<usize>,
    
    /// Failure threshold before switching
    #[serde(default)]
    pub failure_threshold: Option<usize>,
    
    /// Fallback turns
    #[serde(default)]
    pub fallback_turns: Option<usize>,
    
    /// Worker context limit
    #[serde(default)]
    pub worker_context_limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    /// Tool selection strategy
    #[serde(default = "default_router_strategy")]
    pub tool_selection_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecipeConfig {
    /// GitHub repo for recipes
    #[serde(default)]
    pub github_repo: Option<String>,
    
    /// Recipe path
    #[serde(default)]
    pub path: Option<String>,
    
    /// Retry timeout in seconds
    #[serde(default)]
    pub retry_timeout_seconds: Option<u64>,
    
    /// On failure timeout in seconds
    #[serde(default)]
    pub on_failure_timeout_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubagentConfig {
    /// Maximum turns for subagents
    #[serde(default)]
    pub max_turns: Option<u32>,
}

/// Goose execution modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GooseMode {
    Auto,
    FastApply,
    // Add other modes as needed
}

impl Default for GooseMode {
    fn default() -> Self {
        GooseMode::Auto
    }
}

/// Provider configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersConfig {
    #[serde(default)]
    pub openai: Option<OpenAIConfig>,
    
    #[serde(default)]
    pub anthropic: Option<AnthropicConfig>,
    
    #[serde(default)]
    pub azure: Option<AzureConfig>,
    
    #[serde(default)]
    pub google: Option<GoogleConfig>,
    
    #[serde(default)]
    pub groq: Option<GroqConfig>,
    
    #[serde(default)]
    pub ollama: Option<OllamaConfig>,
    
    #[serde(default)]
    pub xai: Option<XAIConfig>,
    
    #[serde(default)]
    pub databricks: Option<DatabricksConfig>,
    
    #[serde(default)]
    pub snowflake: Option<SnowflakeConfig>,
    
    #[serde(default)]
    pub litellm: Option<LiteLLMConfig>,
    
    #[serde(default)]
    pub openrouter: Option<OpenRouterConfig>,
    
    #[serde(default)]
    pub venice: Option<VeniceConfig>,
    
    #[serde(default)]
    pub gemini_cli: Option<GeminiCliConfig>,
    
    #[serde(default)]
    pub github_copilot: Option<GitHubCopilotConfig>,
    
    #[serde(default)]
    pub sagemaker: Option<SageMakerConfig>,
    
    #[serde(default)]
    pub planner: Option<PlannerConfig>,
    
    #[serde(default)]
    pub editor: Option<EditorConfig>,
}

/// OpenAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// API key (stored as SecretString)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_openai_host")]
    pub host: String,
    
    /// Base path for API
    #[serde(default = "default_openai_base_path")]
    pub base_path: String,
    
    /// Organization ID
    #[serde(default)]
    pub organization: Option<String>,
    
    /// Project ID
    #[serde(default)]
    pub project: Option<String>,
    
    /// Custom headers (can contain secrets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<SecretString>,
    
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

/// Anthropic provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_anthropic_host")]
    pub host: String,
}

/// Azure OpenAI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// Azure endpoint
    pub endpoint: Option<String>,
    
    /// Deployment name
    pub deployment_name: Option<String>,
    
    /// API version
    #[serde(default = "default_azure_api_version")]
    pub api_version: String,
}

/// Google provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_google_host")]
    pub host: String,
}

/// Groq provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_groq_host")]
    pub host: String,
}

/// Ollama provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// API host
    #[serde(default = "default_ollama_host")]
    pub host: String,
    
    /// Request timeout
    #[serde(default = "default_ollama_timeout")]
    pub timeout: u64,
}

/// xAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XAIConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_xai_host")]
    pub host: String,
}

/// Databricks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabricksConfig {
    /// API token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretString>,
    
    /// Host URL
    pub host: Option<String>,
}

/// Snowflake configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnowflakeConfig {
    /// API token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretString>,
    
    /// Host URL
    pub host: Option<String>,
}

/// LiteLLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteLLMConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_litellm_host")]
    pub host: String,
    
    /// Base path
    #[serde(default = "default_litellm_base_path")]
    pub base_path: String,
}

/// OpenRouter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_openrouter_host")]
    pub host: String,
}

/// Venice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeniceConfig {
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// API host
    #[serde(default = "default_venice_host")]
    pub host: String,
    
    /// Base path
    #[serde(default = "default_venice_base_path")]
    pub base_path: String,
    
    /// Models path
    #[serde(default = "default_venice_models_path")]
    pub models_path: String,
}

/// Gemini CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiCliConfig {
    /// CLI command
    #[serde(default = "default_gemini_cli_command")]
    pub command: String,
}

/// GitHub Copilot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCopilotConfig {
    /// Access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretString>,
}

/// SageMaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SageMakerConfig {
    /// Endpoint name
    pub endpoint_name: Option<String>,
}

/// Planner-specific provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlannerConfig {
    /// Planner provider override
    #[serde(default)]
    pub provider: Option<String>,
    
    /// Planner model override
    #[serde(default)]
    pub model: Option<String>,
    
    /// Planner context limit
    #[serde(default)]
    pub context_limit: Option<usize>,
}

/// Editor model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorConfig {
    /// Editor API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    /// Editor host
    #[serde(default)]
    pub host: Option<String>,
    
    /// Editor model
    #[serde(default)]
    pub model: Option<String>,
}

/// Extension configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionsConfig {
    /// Map of extension configurations
    #[serde(default)]
    pub configs: HashMap<String, ExtensionEntry>,
}

/// Extension entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionEntry {
    pub enabled: bool,
    #[serde(flatten)]
    pub config: serde_json::Value, // Keep flexible for now
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UIConfig {
    /// CLI-specific settings
    #[serde(default)]
    pub cli: CLIConfig,
    
    /// Desktop-specific settings
    #[serde(default)]
    pub desktop: DesktopConfig,
}

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIConfig {
    /// Color theme
    #[serde(default = "default_cli_theme")]
    pub theme: String,
    
    /// Show cost information
    #[serde(default = "default_show_cost")]
    pub show_cost: bool,
    
    /// Show thinking messages
    #[serde(default)]
    pub show_thinking: bool,
    
    /// Minimum priority for messages
    #[serde(default = "default_min_priority")]
    pub min_priority: f32,
    
    /// Tool params truncation length
    #[serde(default = "default_tool_params_truncation")]
    pub tool_params_truncation_max_length: usize,
}

/// Desktop configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesktopConfig {
    /// Default profile
    #[serde(default)]
    pub default_profile: Option<String>,
    
    /// Auto-save sessions
    #[serde(default = "default_auto_save")]
    pub auto_save_sessions: bool,
}

/// Developer/debugging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperConfig {
    /// Cache directory
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
    
    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    /// Working directory
    #[serde(default)]
    pub working_dir: Option<PathBuf>,
    
    /// Vector DB path
    #[serde(default)]
    pub vector_db_path: Option<PathBuf>,
    
    /// Embedding configuration
    #[serde(default)]
    pub embedding: EmbeddingConfig,
    
    /// Debug flags
    #[serde(default)]
    pub debug: DebugConfig,
    
    /// Test configuration
    #[serde(default)]
    pub test: TestConfig,
    
    /// Server configuration
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingConfig {
    /// Embedding model
    #[serde(default = "default_embedding_model")]
    pub model: String,
    
    /// Embedding model provider
    #[serde(default = "default_embedding_provider")]
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugConfig {
    /// Enable Claude Code debug
    #[serde(default)]
    pub claude_code: bool,
    
    /// Enable Gemini CLI debug
    #[serde(default)]
    pub gemini_cli: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestConfig {
    /// Test provider override
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    /// Server secret key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<SecretString>,
    
    /// Allowlist configuration
    #[serde(default)]
    pub allowlist: Option<String>,
    
    /// Allowlist bypass
    #[serde(default)]
    pub allowlist_bypass: Option<bool>,
}

/// Scheduler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Scheduler type (temporal, simple, etc.)
    #[serde(default = "default_scheduler_type")]
    pub r#type: String,
    
    /// Maximum concurrent jobs
    #[serde(default = "default_max_concurrent_jobs")]
    pub max_concurrent_jobs: usize,
    
    /// Temporal binary path
    #[serde(default)]
    pub temporal_bin: Option<PathBuf>,
}

/// Tracing/observability configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// Langfuse configuration
    #[serde(default)]
    pub langfuse: Option<LangfuseConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangfuseConfig {
    /// Langfuse URL
    #[serde(default = "default_langfuse_url")]
    pub url: String,
    
    /// Langfuse secret key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<SecretString>,
    
    /// Langfuse init project secret key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_project_secret_key: Option<SecretString>,
}

/// Secret string type that never serializes actual values
#[derive(Debug, Clone)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        SecretString(value)
    }
    
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Never serialize actual secret values
        serializer.serialize_str("***")
    }
}

impl<'de> Deserialize<'de> for SecretString {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Secrets should never be deserialized from config files
        // They come from keyring or environment variables
        Ok(SecretString::new(String::new()))
    }
}

// Default value functions
fn default_provider() -> String { "openai".to_string() }
fn default_model() -> String { "gpt-4o".to_string() }
fn default_mode() -> GooseMode { GooseMode::Auto }
fn default_max_turns() -> u32 { 1000 }
fn default_context_strategy() -> String { "auto".to_string() }
fn default_auto_compact_threshold() -> f64 { 0.8 }
fn default_router_strategy() -> String { "auto".to_string() }
fn default_timeout() -> u64 { 600 }

// Provider defaults
fn default_openai_host() -> String { "https://api.openai.com".to_string() }
fn default_openai_base_path() -> String { "v1/chat/completions".to_string() }
fn default_anthropic_host() -> String { "https://api.anthropic.com".to_string() }
fn default_azure_api_version() -> String { "2024-02-15-preview".to_string() }
fn default_google_host() -> String { "https://generativelanguage.googleapis.com".to_string() }
fn default_groq_host() -> String { "https://api.groq.com".to_string() }
fn default_ollama_host() -> String { "http://localhost:11434".to_string() }
fn default_ollama_timeout() -> u64 { 300 }
fn default_xai_host() -> String { "https://api.x.ai".to_string() }
fn default_litellm_host() -> String { "http://localhost:4000".to_string() }
fn default_litellm_base_path() -> String { "v1/chat/completions".to_string() }
fn default_openrouter_host() -> String { "https://openrouter.ai".to_string() }
fn default_venice_host() -> String { "https://api.venice.ai".to_string() }
fn default_venice_base_path() -> String { "api/v1/chat/completions".to_string() }
fn default_venice_models_path() -> String { "api/v1/models".to_string() }
fn default_gemini_cli_command() -> String { "gemini".to_string() }

// UI defaults
fn default_cli_theme() -> String { "dark".to_string() }
fn default_show_cost() -> bool { true }
fn default_min_priority() -> f32 { 0.5 }
fn default_tool_params_truncation() -> usize { 40 }
fn default_auto_save() -> bool { true }

// Developer defaults
fn default_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("goose")
}
fn default_log_level() -> String { "info".to_string() }
fn default_embedding_model() -> String { "text-embedding-3-small".to_string() }
fn default_embedding_provider() -> String { "openai".to_string() }

// Scheduler defaults
fn default_scheduler_type() -> String { "temporal".to_string() }
fn default_max_concurrent_jobs() -> usize { 5 }

// Tracing defaults
fn default_langfuse_url() -> String { "https://cloud.langfuse.com".to_string() }

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: default_model(),
            mode: default_mode(),
            context_limit: None,
            temperature: None,
            max_turns: default_max_turns(),
            context_strategy: default_context_strategy(),
            auto_compact_threshold: default_auto_compact_threshold(),
            system_prompt_file_path: None,
            toolshim: ToolshimConfig::default(),
            lead_worker: LeadWorkerConfig::default(),
            router: RouterConfig::default(),
            recipe: RecipeConfig::default(),
            subagent: SubagentConfig::default(),
        }
    }
}

impl Default for CLIConfig {
    fn default() -> Self {
        Self {
            theme: default_cli_theme(),
            show_cost: default_show_cost(),
            show_thinking: false,
            min_priority: default_min_priority(),
            tool_params_truncation_max_length: default_tool_params_truncation(),
        }
    }
}

impl Default for DeveloperConfig {
    fn default() -> Self {
        Self {
            cache_dir: default_cache_dir(),
            log_level: default_log_level(),
            working_dir: None,
            vector_db_path: None,
            embedding: EmbeddingConfig::default(),
            debug: DebugConfig::default(),
            test: TestConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            r#type: default_scheduler_type(),
            max_concurrent_jobs: default_max_concurrent_jobs(),
            temporal_bin: None,
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            host: default_openai_host(),
            base_path: default_openai_base_path(),
            organization: None,
            project: None,
            custom_headers: None,
            timeout: default_timeout(),
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            host: default_anthropic_host(),
        }
    }
}

// Add similar Default implementations for other provider configs...

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_schema_serialization() {
        let config = ConfigSchema::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        assert!(yaml.contains("core:"));
        assert!(yaml.contains("providers:"));
        
        // Secrets should not be serialized
        let mut config = ConfigSchema::default();
        config.providers.openai = Some(OpenAIConfig {
            api_key: Some(SecretString::new("sk-test".to_string())),
            ..Default::default()
        });
        let yaml = serde_yaml::to_string(&config).unwrap();
        assert!(!yaml.contains("sk-test"));
        assert!(yaml.contains("***"));
    }
    
    #[test]
    fn test_defaults() {
        let config = ConfigSchema::default();
        assert_eq!(config.core.provider, "openai");
        assert_eq!(config.core.model, "gpt-4o");
        assert_eq!(config.core.max_turns, 1000);
        assert_eq!(config.ui.cli.theme, "dark");
        assert_eq!(config.scheduler.r#type, "temporal");
    }
}
