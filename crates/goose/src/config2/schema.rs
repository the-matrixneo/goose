//! Configuration schema with integrated environment variable mappings
//! 
//! This module defines the complete configuration structure for Goose with:
//! - All configuration options in one place
//! - Explicit secret marking
//! - Environment variable mappings built into the schema

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Macro to define a configuration field with optional environment variable mapping
macro_rules! config_field {
    ($field:ident: $type:ty) => {
        pub $field: $type
    };
    ($field:ident: $type:ty, env: $env:expr) => {
        #[serde(default)]
        #[env_var($env)]
        pub $field: $type
    };
    ($field:ident: $type:ty, env: $env:expr, default: $default:expr) => {
        #[serde(default = $default)]
        #[env_var($env)]
        pub $field: $type
    };
    ($field:ident: Secret<$type:ty>, env: $env:expr) => {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[env_var($env)]
        #[secret]
        pub $field: Option<Secret<$type>>
    };
}

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
    #[serde(default = "default_provider")]
    #[env_var("GOOSE_PROVIDER")]
    pub provider: String,
    
    #[serde(default = "default_model")]
    #[env_var("GOOSE_MODEL")]
    pub model: String,
    
    #[serde(default = "default_mode")]
    #[env_var("GOOSE_MODE")]
    pub mode: GooseMode,
    
    #[serde(default)]
    #[env_var("GOOSE_CONTEXT_LIMIT")]
    pub context_limit: Option<usize>,
    
    #[serde(default)]
    #[env_var("GOOSE_TEMPERATURE")]
    pub temperature: Option<f32>,
    
    #[serde(default = "default_max_turns")]
    #[env_var("GOOSE_MAX_TURNS")]
    pub max_turns: u32,
    
    #[serde(default = "default_context_strategy")]
    #[env_var("GOOSE_CONTEXT_STRATEGY")]
    pub context_strategy: String,
    
    #[serde(default = "default_auto_compact_threshold")]
    #[env_var("GOOSE_AUTO_COMPACT_THRESHOLD")]
    pub auto_compact_threshold: f64,
    
    #[serde(default)]
    #[env_var("GOOSE_SYSTEM_PROMPT_FILE_PATH")]
    pub system_prompt_file_path: Option<PathBuf>,
    
    #[serde(default)]
    pub toolshim: ToolshimConfig,
    
    #[serde(default)]
    pub lead_worker: LeadWorkerConfig,
    
    #[serde(default)]
    pub router: RouterConfig,
    
    #[serde(default)]
    pub recipe: RecipeConfig,
    
    #[serde(default)]
    pub subagent: SubagentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolshimConfig {
    #[serde(default)]
    #[env_var("GOOSE_TOOLSHIM")]
    pub enabled: bool,
    
    #[serde(default)]
    #[env_var("GOOSE_TOOLSHIM_OLLAMA_MODEL")]
    pub ollama_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeadWorkerConfig {
    #[serde(default)]
    #[env_var("GOOSE_LEAD_PROVIDER")]
    pub provider: Option<String>,
    
    #[serde(default)]
    #[env_var("GOOSE_LEAD_MODEL")]
    pub model: Option<String>,
    
    #[serde(default)]
    #[env_var("GOOSE_LEAD_TURNS")]
    pub turns: Option<usize>,
    
    #[serde(default)]
    #[env_var("GOOSE_LEAD_FAILURE_THRESHOLD")]
    pub failure_threshold: Option<usize>,
    
    #[serde(default)]
    #[env_var("GOOSE_LEAD_FALLBACK_TURNS")]
    pub fallback_turns: Option<usize>,
    
    #[serde(default)]
    #[env_var("GOOSE_WORKER_CONTEXT_LIMIT")]
    pub worker_context_limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    #[serde(default = "default_router_strategy")]
    #[env_var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")]
    pub tool_selection_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecipeConfig {
    #[serde(default)]
    #[env_var("GOOSE_RECIPE_GITHUB_REPO")]
    pub github_repo: Option<String>,
    
    #[serde(default)]
    #[env_var("GOOSE_RECIPE_PATH")]
    pub path: Option<String>,
    
    #[serde(default)]
    #[env_var("GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS")]
    pub retry_timeout_seconds: Option<u64>,
    
    #[serde(default)]
    #[env_var("GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS")]
    pub on_failure_timeout_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubagentConfig {
    #[serde(default)]
    #[env_var("GOOSE_SUBAGENT_MAX_TURNS")]
    pub max_turns: Option<u32>,
}

/// Goose execution modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GooseMode {
    Auto,
    FastApply,
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
    
    // Add other providers as needed
}

/// OpenAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("OPENAI_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_openai_host")]
    #[env_var("OPENAI_HOST")]
    pub host: String,
    
    #[serde(default = "default_openai_base_path")]
    #[env_var("OPENAI_BASE_PATH")]
    pub base_path: String,
    
    #[serde(default)]
    #[env_var("OPENAI_ORGANIZATION")]
    pub organization: Option<String>,
    
    #[serde(default)]
    #[env_var("OPENAI_PROJECT")]
    pub project: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("OPENAI_CUSTOM_HEADERS")]
    #[secret]
    pub custom_headers: Option<Secret<String>>,
    
    #[serde(default = "default_timeout")]
    #[env_var("OPENAI_TIMEOUT")]
    pub timeout: u64,
}

/// Anthropic provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("ANTHROPIC_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_anthropic_host")]
    #[env_var("ANTHROPIC_HOST")]
    pub host: String,
}

/// Azure OpenAI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("AZURE_OPENAI_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default)]
    #[env_var("AZURE_OPENAI_ENDPOINT")]
    pub endpoint: Option<String>,
    
    #[serde(default)]
    #[env_var("AZURE_OPENAI_DEPLOYMENT_NAME")]
    pub deployment_name: Option<String>,
    
    #[serde(default = "default_azure_api_version")]
    #[env_var("AZURE_OPENAI_API_VERSION")]
    pub api_version: String,
}

/// Google provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("GOOGLE_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_google_host")]
    #[env_var("GOOGLE_HOST")]
    pub host: String,
}

/// Groq provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("GROQ_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_groq_host")]
    #[env_var("GROQ_HOST")]
    pub host: String,
}

/// Ollama provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    #[serde(default = "default_ollama_host")]
    #[env_var("OLLAMA_HOST")]
    pub host: String,
    
    #[serde(default = "default_ollama_timeout")]
    #[env_var("OLLAMA_TIMEOUT")]
    pub timeout: u64,
}

/// xAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XAIConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("XAI_API_KEY")]
    #[secret]
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_xai_host")]
    #[env_var("XAI_HOST")]
    pub host: String,
}

/// Extension configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionsConfig {
    // Keep this flexible for now
    #[serde(default)]
    pub configs: HashMap<String, serde_json::Value>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UIConfig {
    #[serde(default)]
    pub cli: CLIConfig,
    
    #[serde(default)]
    pub desktop: DesktopConfig,
}

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIConfig {
    #[serde(default = "default_cli_theme")]
    #[env_var("GOOSE_CLI_THEME")]
    pub theme: String,
    
    #[serde(default = "default_show_cost")]
    #[env_var("GOOSE_CLI_SHOW_COST")]
    pub show_cost: bool,
    
    #[serde(default)]
    #[env_var("GOOSE_CLI_SHOW_THINKING")]
    pub show_thinking: bool,
    
    #[serde(default = "default_min_priority")]
    #[env_var("GOOSE_CLI_MIN_PRIORITY")]
    pub min_priority: f32,
    
    #[serde(default = "default_tool_params_truncation")]
    #[env_var("GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH")]
    pub tool_params_truncation_max_length: usize,
}

/// Desktop configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesktopConfig {
    #[serde(default)]
    pub default_profile: Option<String>,
    
    #[serde(default = "default_auto_save")]
    pub auto_save_sessions: bool,
}

/// Developer/debugging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperConfig {
    #[serde(default = "default_cache_dir")]
    #[env_var("GOOSE_CACHE_DIR")]
    pub cache_dir: PathBuf,
    
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    #[serde(default)]
    #[env_var("GOOSE_WORKING_DIR")]
    pub working_dir: Option<PathBuf>,
    
    #[serde(default)]
    #[env_var("GOOSE_VECTOR_DB_PATH")]
    pub vector_db_path: Option<PathBuf>,
    
    #[serde(default)]
    pub embedding: EmbeddingConfig,
    
    #[serde(default)]
    pub debug: DebugConfig,
    
    #[serde(default)]
    pub test: TestConfig,
    
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingConfig {
    #[serde(default = "default_embedding_model")]
    #[env_var("GOOSE_EMBEDDING_MODEL")]
    pub model: String,
    
    #[serde(default = "default_embedding_provider")]
    #[env_var("GOOSE_EMBEDDING_MODEL_PROVIDER")]
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugConfig {
    #[serde(default)]
    #[env_var("GOOSE_CLAUDE_CODE_DEBUG")]
    pub claude_code: bool,
    
    #[serde(default)]
    #[env_var("GOOSE_GEMINI_CLI_DEBUG")]
    pub gemini_cli: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestConfig {
    #[serde(default)]
    #[env_var("GOOSE_TEST_PROVIDER")]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("GOOSE_SERVER__SECRET_KEY")]
    #[secret]
    pub secret_key: Option<Secret<String>>,
    
    #[serde(default)]
    #[env_var("GOOSE_ALLOWLIST")]
    pub allowlist: Option<String>,
    
    #[serde(default)]
    #[env_var("GOOSE_ALLOWLIST_BYPASS")]
    pub allowlist_bypass: Option<bool>,
}

/// Scheduler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    #[serde(default = "default_scheduler_type")]
    #[env_var("GOOSE_SCHEDULER_TYPE")]
    pub r#type: String,
    
    #[serde(default = "default_max_concurrent_jobs")]
    pub max_concurrent_jobs: usize,
    
    #[serde(default)]
    #[env_var("GOOSE_TEMPORAL_BIN")]
    pub temporal_bin: Option<PathBuf>,
}

/// Tracing/observability configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    #[serde(default)]
    pub langfuse: Option<LangfuseConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangfuseConfig {
    #[serde(default = "default_langfuse_url")]
    #[env_var("LANGFUSE_URL")]
    pub url: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("LANGFUSE_SECRET_KEY")]
    #[secret]
    pub secret_key: Option<Secret<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("LANGFUSE_INIT_PROJECT_SECRET_KEY")]
    #[secret]
    pub init_project_secret_key: Option<Secret<String>>,
}

/// Secret wrapper type that never serializes actual values
#[derive(Debug, Clone)]
pub struct Secret<T>(T);

impl<T> Secret<T> {
    pub fn new(value: T) -> Self {
        Secret(value)
    }
    
    pub fn expose(&self) -> &T {
        &self.0
    }
    
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Serialize for Secret<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Never serialize actual secret values
        serializer.serialize_str("***")
    }
}

impl<'de, T> Deserialize<'de> for Secret<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Secrets should never be deserialized from config files
        // They come from keyring or environment variables
        Ok(Secret(T::deserialize(_deserializer)?))
    }
}

/// Configuration value that can be either a direct value or a reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue<T> {
    /// Direct value
    Value(T),
    /// Reference to another source
    Reference(String), // e.g., "${env:SOME_VAR}" or "${file:/path/to/file}"
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

// Implement defaults for structs
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

impl Default for AzureConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            endpoint: None,
            deployment_name: None,
            api_version: default_azure_api_version(),
        }
    }
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            host: default_google_host(),
        }
    }
}

impl Default for GroqConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            host: default_groq_host(),
        }
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: default_ollama_host(),
            timeout: default_ollama_timeout(),
        }
    }
}

impl Default for XAIConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            host: default_xai_host(),
        }
    }
}

impl Default for LangfuseConfig {
    fn default() -> Self {
        Self {
            url: default_langfuse_url(),
            secret_key: None,
            init_project_secret_key: None,
        }
    }
}

/// Trait to extract environment variable mappings from the schema
pub trait EnvVarMapping {
    fn get_env_mappings(&self) -> Vec<(&'static str, String, bool)>;
}

// Helper macro to implement environment variable extraction
macro_rules! impl_env_mapping {
    ($struct_name:ident, $($field:ident => $env:expr, $secret:expr),*) => {
        impl EnvVarMapping for $struct_name {
            fn get_env_mappings(&self) -> Vec<(&'static str, String, bool)> {
                vec![
                    $(($env, stringify!($field).to_string(), $secret),)*
                ]
            }
        }
    };
}

// Implement for each config struct
impl_env_mapping!(CoreConfig,
    provider => "GOOSE_PROVIDER", false,
    model => "GOOSE_MODEL", false,
    mode => "GOOSE_MODE", false,
    context_limit => "GOOSE_CONTEXT_LIMIT", false,
    temperature => "GOOSE_TEMPERATURE", false,
    max_turns => "GOOSE_MAX_TURNS", false,
    context_strategy => "GOOSE_CONTEXT_STRATEGY", false,
    auto_compact_threshold => "GOOSE_AUTO_COMPACT_THRESHOLD", false,
    system_prompt_file_path => "GOOSE_SYSTEM_PROMPT_FILE_PATH", false
);

impl_env_mapping!(OpenAIConfig,
    api_key => "OPENAI_API_KEY", true,
    host => "OPENAI_HOST", false,
    base_path => "OPENAI_BASE_PATH", false,
    organization => "OPENAI_ORGANIZATION", false,
    project => "OPENAI_PROJECT", false,
    custom_headers => "OPENAI_CUSTOM_HEADERS", true,
    timeout => "OPENAI_TIMEOUT", false
);

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
            api_key: Some(Secret::new("sk-test".to_string())),
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
    
    #[test]
    fn test_secret_type() {
        let secret = Secret::new("my-secret-value".to_string());
        assert_eq!(secret.expose(), "my-secret-value");
        
        // Test serialization
        let json = serde_json::to_string(&secret).unwrap();
        assert_eq!(json, "\"***\"");
    }
}
