//! Environment variable mapping for configuration
//! 
//! This module provides explicit mappings between environment variables
//! and configuration paths, making it clear what environment variables
//! are supported and where they map in the configuration structure.

use super::schema::*;
use std::collections::HashMap;

/// Mapping between environment variables and configuration paths
#[derive(Debug, Clone)]
pub struct EnvMapping {
    /// The environment variable name
    pub env_var: &'static str,
    
    /// The configuration path (e.g., "core.provider")
    pub config_path: &'static str,
    
    /// Whether this is a secret value
    pub is_secret: bool,
    
    /// Optional default value
    pub default: Option<&'static str>,
}

impl EnvMapping {
    pub const fn new(env_var: &'static str, config_path: &'static str, is_secret: bool) -> Self {
        Self {
            env_var,
            config_path,
            is_secret,
            default: None,
        }
    }
    
    pub const fn with_default(mut self, default: &'static str) -> Self {
        self.default = Some(default);
        self
    }
}

/// All environment variable mappings
pub const ENV_MAPPINGS: &[EnvMapping] = &[
    // Core settings
    EnvMapping::new("GOOSE_PROVIDER", "core.provider", false),
    EnvMapping::new("GOOSE_MODEL", "core.model", false),
    EnvMapping::new("GOOSE_MODE", "core.mode", false),
    EnvMapping::new("GOOSE_CONTEXT_LIMIT", "core.context_limit", false),
    EnvMapping::new("GOOSE_TEMPERATURE", "core.temperature", false),
    EnvMapping::new("GOOSE_MAX_TURNS", "core.max_turns", false),
    EnvMapping::new("GOOSE_CONTEXT_STRATEGY", "core.context_strategy", false),
    EnvMapping::new("GOOSE_AUTO_COMPACT_THRESHOLD", "core.auto_compact_threshold", false),
    EnvMapping::new("GOOSE_SYSTEM_PROMPT_FILE_PATH", "core.system_prompt_file_path", false),
    
    // Toolshim
    EnvMapping::new("GOOSE_TOOLSHIM", "core.toolshim.enabled", false),
    EnvMapping::new("GOOSE_TOOLSHIM_OLLAMA_MODEL", "core.toolshim.ollama_model", false),
    
    // Lead-worker
    EnvMapping::new("GOOSE_LEAD_PROVIDER", "core.lead_worker.provider", false),
    EnvMapping::new("GOOSE_LEAD_MODEL", "core.lead_worker.model", false),
    EnvMapping::new("GOOSE_LEAD_TURNS", "core.lead_worker.turns", false),
    EnvMapping::new("GOOSE_LEAD_FAILURE_THRESHOLD", "core.lead_worker.failure_threshold", false),
    EnvMapping::new("GOOSE_LEAD_FALLBACK_TURNS", "core.lead_worker.fallback_turns", false),
    EnvMapping::new("GOOSE_WORKER_CONTEXT_LIMIT", "core.lead_worker.worker_context_limit", false),
    
    // Router
    EnvMapping::new("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY", "core.router.tool_selection_strategy", false),
    
    // Recipe
    EnvMapping::new("GOOSE_RECIPE_GITHUB_REPO", "core.recipe.github_repo", false),
    EnvMapping::new("GOOSE_RECIPE_PATH", "core.recipe.path", false),
    EnvMapping::new("GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS", "core.recipe.retry_timeout_seconds", false),
    EnvMapping::new("GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS", "core.recipe.on_failure_timeout_seconds", false),
    
    // Subagent
    EnvMapping::new("GOOSE_SUBAGENT_MAX_TURNS", "core.subagent.max_turns", false),
    
    // OpenAI provider
    EnvMapping::new("OPENAI_API_KEY", "providers.openai.api_key", true),
    EnvMapping::new("OPENAI_HOST", "providers.openai.host", false)
        .with_default("https://api.openai.com"),
    EnvMapping::new("OPENAI_BASE_PATH", "providers.openai.base_path", false)
        .with_default("v1/chat/completions"),
    EnvMapping::new("OPENAI_ORGANIZATION", "providers.openai.organization", false),
    EnvMapping::new("OPENAI_PROJECT", "providers.openai.project", false),
    EnvMapping::new("OPENAI_CUSTOM_HEADERS", "providers.openai.custom_headers", true),
    EnvMapping::new("OPENAI_TIMEOUT", "providers.openai.timeout", false)
        .with_default("600"),
    
    // Anthropic provider
    EnvMapping::new("ANTHROPIC_API_KEY", "providers.anthropic.api_key", true),
    EnvMapping::new("ANTHROPIC_HOST", "providers.anthropic.host", false)
        .with_default("https://api.anthropic.com"),
    
    // Azure provider
    EnvMapping::new("AZURE_OPENAI_API_KEY", "providers.azure.api_key", true),
    EnvMapping::new("AZURE_OPENAI_ENDPOINT", "providers.azure.endpoint", false),
    EnvMapping::new("AZURE_OPENAI_DEPLOYMENT_NAME", "providers.azure.deployment_name", false),
    EnvMapping::new("AZURE_OPENAI_API_VERSION", "providers.azure.api_version", false)
        .with_default("2024-02-15-preview"),
    
    // Google provider
    EnvMapping::new("GOOGLE_API_KEY", "providers.google.api_key", true),
    EnvMapping::new("GOOGLE_HOST", "providers.google.host", false)
        .with_default("https://generativelanguage.googleapis.com"),
    
    // Groq provider
    EnvMapping::new("GROQ_API_KEY", "providers.groq.api_key", true),
    EnvMapping::new("GROQ_HOST", "providers.groq.host", false)
        .with_default("https://api.groq.com"),
    
    // Ollama provider
    EnvMapping::new("OLLAMA_HOST", "providers.ollama.host", false)
        .with_default("http://localhost:11434"),
    EnvMapping::new("OLLAMA_TIMEOUT", "providers.ollama.timeout", false)
        .with_default("300"),
    
    // xAI provider
    EnvMapping::new("XAI_API_KEY", "providers.xai.api_key", true),
    EnvMapping::new("XAI_HOST", "providers.xai.host", false)
        .with_default("https://api.x.ai"),
    
    // Databricks provider
    EnvMapping::new("DATABRICKS_TOKEN", "providers.databricks.token", true),
    EnvMapping::new("DATABRICKS_HOST", "providers.databricks.host", false),
    
    // Snowflake provider
    EnvMapping::new("SNOWFLAKE_TOKEN", "providers.snowflake.token", true),
    EnvMapping::new("SNOWFLAKE_HOST", "providers.snowflake.host", false),
    
    // LiteLLM provider
    EnvMapping::new("LITELLM_API_KEY", "providers.litellm.api_key", true),
    EnvMapping::new("LITELLM_HOST", "providers.litellm.host", false)
        .with_default("http://localhost:4000"),
    EnvMapping::new("LITELLM_BASE_PATH", "providers.litellm.base_path", false)
        .with_default("v1/chat/completions"),
    
    // OpenRouter provider
    EnvMapping::new("OPENROUTER_API_KEY", "providers.openrouter.api_key", true),
    EnvMapping::new("OPENROUTER_HOST", "providers.openrouter.host", false)
        .with_default("https://openrouter.ai"),
    
    // Venice provider
    EnvMapping::new("VENICE_API_KEY", "providers.venice.api_key", true),
    EnvMapping::new("VENICE_HOST", "providers.venice.host", false)
        .with_default("https://api.venice.ai"),
    EnvMapping::new("VENICE_BASE_PATH", "providers.venice.base_path", false)
        .with_default("api/v1/chat/completions"),
    EnvMapping::new("VENICE_MODELS_PATH", "providers.venice.models_path", false)
        .with_default("api/v1/models"),
    
    // Gemini CLI provider
    EnvMapping::new("GEMINI_CLI_COMMAND", "providers.gemini_cli.command", false)
        .with_default("gemini"),
    
    // GitHub Copilot provider
    EnvMapping::new("GITHUB_COPILOT_TOKEN", "providers.github_copilot.token", true),
    
    // SageMaker provider
    EnvMapping::new("SAGEMAKER_ENDPOINT_NAME", "providers.sagemaker.endpoint_name", false),
    
    // Planner provider
    EnvMapping::new("GOOSE_PLANNER_PROVIDER", "providers.planner.provider", false),
    EnvMapping::new("GOOSE_PLANNER_MODEL", "providers.planner.model", false),
    EnvMapping::new("GOOSE_PLANNER_CONTEXT_LIMIT", "providers.planner.context_limit", false),
    
    // Editor provider
    EnvMapping::new("GOOSE_EDITOR_API_KEY", "providers.editor.api_key", true),
    EnvMapping::new("GOOSE_EDITOR_HOST", "providers.editor.host", false),
    EnvMapping::new("GOOSE_EDITOR_MODEL", "providers.editor.model", false),
    
    // UI settings
    EnvMapping::new("GOOSE_CLI_THEME", "ui.cli.theme", false),
    EnvMapping::new("GOOSE_CLI_SHOW_COST", "ui.cli.show_cost", false),
    EnvMapping::new("GOOSE_CLI_SHOW_THINKING", "ui.cli.show_thinking", false),
    EnvMapping::new("GOOSE_CLI_MIN_PRIORITY", "ui.cli.min_priority", false),
    EnvMapping::new("GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH", "ui.cli.tool_params_truncation_max_length", false),
    
    // Developer settings
    EnvMapping::new("GOOSE_CACHE_DIR", "developer.cache_dir", false),
    EnvMapping::new("GOOSE_WORKING_DIR", "developer.working_dir", false),
    EnvMapping::new("GOOSE_VECTOR_DB_PATH", "developer.vector_db_path", false),
    EnvMapping::new("GOOSE_EMBEDDING_MODEL", "developer.embedding.model", false)
        .with_default("text-embedding-3-small"),
    EnvMapping::new("GOOSE_EMBEDDING_MODEL_PROVIDER", "developer.embedding.provider", false)
        .with_default("openai"),
    EnvMapping::new("GOOSE_CLAUDE_CODE_DEBUG", "developer.debug.claude_code", false),
    EnvMapping::new("GOOSE_GEMINI_CLI_DEBUG", "developer.debug.gemini_cli", false),
    EnvMapping::new("GOOSE_TEST_PROVIDER", "developer.test.provider", false),
    EnvMapping::new("GOOSE_SERVER__SECRET_KEY", "developer.server.secret_key", true),
    EnvMapping::new("GOOSE_ALLOWLIST", "developer.server.allowlist", false),
    EnvMapping::new("GOOSE_ALLOWLIST_BYPASS", "developer.server.allowlist_bypass", false),
    
    // Scheduler settings
    EnvMapping::new("GOOSE_SCHEDULER_TYPE", "scheduler.type", false)
        .with_default("temporal"),
    EnvMapping::new("GOOSE_TEMPORAL_BIN", "scheduler.temporal_bin", false),
    
    // Tracing settings
    EnvMapping::new("LANGFUSE_URL", "tracing.langfuse.url", false)
        .with_default("https://cloud.langfuse.com"),
    EnvMapping::new("LANGFUSE_SECRET_KEY", "tracing.langfuse.secret_key", true),
    EnvMapping::new("LANGFUSE_INIT_PROJECT_SECRET_KEY", "tracing.langfuse.init_project_secret_key", true),
    
    // Special cases - these need custom handling
    EnvMapping::new("GOOSE_DISABLE_KEYRING", "_internal.disable_keyring", false),
    EnvMapping::new("ELEVENLABS_API_KEY", "_external.elevenlabs_api_key", true),
    EnvMapping::new("GOOGLE_DRIVE_CREDENTIALS_PATH", "_external.google_drive_credentials_path", false),
    EnvMapping::new("GOOGLE_DRIVE_OAUTH_PATH", "_external.google_drive_oauth_path", false),
];

/// Build a lookup map from environment variable to mapping
pub fn build_env_map() -> HashMap<&'static str, &'static EnvMapping> {
    ENV_MAPPINGS.iter()
        .map(|mapping| (mapping.env_var, mapping))
        .collect()
}

/// Build a lookup map from config path to mapping
pub fn build_path_map() -> HashMap<&'static str, &'static EnvMapping> {
    ENV_MAPPINGS.iter()
        .map(|mapping| (mapping.config_path, mapping))
        .collect()
}

/// Get all environment variables that are secrets
pub fn get_secret_env_vars() -> Vec<&'static str> {
    ENV_MAPPINGS.iter()
        .filter(|m| m.is_secret)
        .map(|m| m.env_var)
        .collect()
}

/// Get all environment variables for a specific provider
pub fn get_provider_env_vars(provider: &str) -> Vec<&'static EnvMapping> {
    let prefix = format!("providers.{}", provider);
    ENV_MAPPINGS.iter()
        .filter(|m| m.config_path.starts_with(&prefix))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_env_mappings() {
        let env_map = build_env_map();
        assert!(env_map.contains_key("GOOSE_PROVIDER"));
        assert!(env_map.contains_key("OPENAI_API_KEY"));
        
        let mapping = env_map.get("OPENAI_API_KEY").unwrap();
        assert_eq!(mapping.config_path, "providers.openai.api_key");
        assert!(mapping.is_secret);
    }
    
    #[test]
    fn test_secret_env_vars() {
        let secrets = get_secret_env_vars();
        assert!(secrets.contains(&"OPENAI_API_KEY"));
        assert!(secrets.contains(&"ANTHROPIC_API_KEY"));
        assert!(!secrets.contains(&"GOOSE_PROVIDER"));
    }
    
    #[test]
    fn test_provider_env_vars() {
        let openai_vars = get_provider_env_vars("openai");
        assert!(openai_vars.iter().any(|m| m.env_var == "OPENAI_API_KEY"));
        assert!(openai_vars.iter().any(|m| m.env_var == "OPENAI_HOST"));
    }
}
