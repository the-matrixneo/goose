//! Configuration manager with layered configuration support
//! 
//! This module implements the new configuration system with:
//! - Layered configuration (defaults -> system -> user -> profile -> env -> runtime)
//! - Type-safe access to configuration values
//! - Automatic secret resolution
//! - Environment variable overrides based on schema annotations

use super::schema::*;
use super::secrets::{SecretsManager, SecretValue};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration manager with layered configuration support
pub struct ConfigManager {
    /// Base layer: Built-in defaults
    defaults: ConfigSchema,
    
    /// System layer: System-wide config (e.g., /etc/goose/config.yaml)
    system: Option<ConfigSchema>,
    
    /// User layer: User config (~/.config/goose/config.yaml)
    user: Option<ConfigSchema>,
    
    /// Profile layer: Active profile config
    profile: Option<ConfigSchema>,
    
    /// Environment layer: Environment variable overrides
    environment: HashMap<String, Value>,
    
    /// Runtime layer: Programmatic overrides
    runtime: Arc<RwLock<HashMap<String, Value>>>,
    
    /// Secrets manager
    secrets: SecretsManager,
    
    /// Active profile name
    active_profile: Option<String>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub async fn new() -> Result<Self> {
        let secrets = SecretsManager::new();
        
        // Load user config if it exists
        let user = Self::load_user_config().await?;
        
        // Load environment overrides based on schema
        let environment = Self::load_environment_overrides();
        
        Ok(Self {
            defaults: ConfigSchema::default(),
            system: None, // TODO: Implement system config loading
            user,
            profile: None,
            environment,
            runtime: Arc::new(RwLock::new(HashMap::new())),
            secrets,
            active_profile: None,
        })
    }
    
    /// Load user configuration from file
    async fn load_user_config() -> Result<Option<ConfigSchema>> {
        let config_path = Self::get_user_config_path()?;
        if !config_path.exists() {
            return Ok(None);
        }
        
        let content = tokio::fs::read_to_string(&config_path).await?;
        let config: ConfigSchema = serde_yaml::from_str(&content)?;
        Ok(Some(config))
    }
    
    /// Get the user config path
    fn get_user_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join("goose");
        Ok(config_dir.join("config.yaml"))
    }
    
    /// Load environment variable overrides based on schema annotations
    fn load_environment_overrides() -> HashMap<String, Value> {
        let mut overrides = HashMap::new();
        
        // This is a simplified version - in reality we'd use proc macros
        // to extract the #[env_var] annotations from the schema
        let env_mappings = vec![
            // Core
            ("GOOSE_PROVIDER", "core.provider", false),
            ("GOOSE_MODEL", "core.model", false),
            ("GOOSE_MODE", "core.mode", false),
            ("GOOSE_CONTEXT_LIMIT", "core.context_limit", false),
            ("GOOSE_TEMPERATURE", "core.temperature", false),
            ("GOOSE_MAX_TURNS", "core.max_turns", false),
            ("GOOSE_CONTEXT_STRATEGY", "core.context_strategy", false),
            ("GOOSE_AUTO_COMPACT_THRESHOLD", "core.auto_compact_threshold", false),
            ("GOOSE_SYSTEM_PROMPT_FILE_PATH", "core.system_prompt_file_path", false),
            
            // Toolshim
            ("GOOSE_TOOLSHIM", "core.toolshim.enabled", false),
            ("GOOSE_TOOLSHIM_OLLAMA_MODEL", "core.toolshim.ollama_model", false),
            
            // Lead-worker
            ("GOOSE_LEAD_PROVIDER", "core.lead_worker.provider", false),
            ("GOOSE_LEAD_MODEL", "core.lead_worker.model", false),
            ("GOOSE_LEAD_TURNS", "core.lead_worker.turns", false),
            ("GOOSE_LEAD_FAILURE_THRESHOLD", "core.lead_worker.failure_threshold", false),
            ("GOOSE_LEAD_FALLBACK_TURNS", "core.lead_worker.fallback_turns", false),
            ("GOOSE_WORKER_CONTEXT_LIMIT", "core.lead_worker.worker_context_limit", false),
            
            // Router
            ("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY", "core.router.tool_selection_strategy", false),
            
            // Recipe
            ("GOOSE_RECIPE_GITHUB_REPO", "core.recipe.github_repo", false),
            ("GOOSE_RECIPE_PATH", "core.recipe.path", false),
            ("GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS", "core.recipe.retry_timeout_seconds", false),
            ("GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS", "core.recipe.on_failure_timeout_seconds", false),
            
            // Subagent
            ("GOOSE_SUBAGENT_MAX_TURNS", "core.subagent.max_turns", false),
            
            // OpenAI
            ("OPENAI_API_KEY", "providers.openai.api_key", true),
            ("OPENAI_HOST", "providers.openai.host", false),
            ("OPENAI_BASE_PATH", "providers.openai.base_path", false),
            ("OPENAI_ORGANIZATION", "providers.openai.organization", false),
            ("OPENAI_PROJECT", "providers.openai.project", false),
            ("OPENAI_CUSTOM_HEADERS", "providers.openai.custom_headers", true),
            ("OPENAI_TIMEOUT", "providers.openai.timeout", false),
            
            // Anthropic
            ("ANTHROPIC_API_KEY", "providers.anthropic.api_key", true),
            ("ANTHROPIC_HOST", "providers.anthropic.host", false),
            
            // Azure
            ("AZURE_OPENAI_API_KEY", "providers.azure.api_key", true),
            ("AZURE_OPENAI_ENDPOINT", "providers.azure.endpoint", false),
            ("AZURE_OPENAI_DEPLOYMENT_NAME", "providers.azure.deployment_name", false),
            ("AZURE_OPENAI_API_VERSION", "providers.azure.api_version", false),
            
            // Google
            ("GOOGLE_API_KEY", "providers.google.api_key", true),
            ("GOOGLE_HOST", "providers.google.host", false),
            
            // Groq
            ("GROQ_API_KEY", "providers.groq.api_key", true),
            ("GROQ_HOST", "providers.groq.host", false),
            
            // Ollama
            ("OLLAMA_HOST", "providers.ollama.host", false),
            ("OLLAMA_TIMEOUT", "providers.ollama.timeout", false),
            
            // xAI
            ("XAI_API_KEY", "providers.xai.api_key", true),
            ("XAI_HOST", "providers.xai.host", false),
            
            // CLI
            ("GOOSE_CLI_THEME", "ui.cli.theme", false),
            ("GOOSE_CLI_SHOW_COST", "ui.cli.show_cost", false),
            ("GOOSE_CLI_SHOW_THINKING", "ui.cli.show_thinking", false),
            ("GOOSE_CLI_MIN_PRIORITY", "ui.cli.min_priority", false),
            ("GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH", "ui.cli.tool_params_truncation_max_length", false),
            
            // Developer
            ("GOOSE_CACHE_DIR", "developer.cache_dir", false),
            ("GOOSE_WORKING_DIR", "developer.working_dir", false),
            ("GOOSE_VECTOR_DB_PATH", "developer.vector_db_path", false),
            ("GOOSE_EMBEDDING_MODEL", "developer.embedding.model", false),
            ("GOOSE_EMBEDDING_MODEL_PROVIDER", "developer.embedding.provider", false),
            ("GOOSE_CLAUDE_CODE_DEBUG", "developer.debug.claude_code", false),
            ("GOOSE_GEMINI_CLI_DEBUG", "developer.debug.gemini_cli", false),
            ("GOOSE_TEST_PROVIDER", "developer.test.provider", false),
            ("GOOSE_SERVER__SECRET_KEY", "developer.server.secret_key", true),
            ("GOOSE_ALLOWLIST", "developer.server.allowlist", false),
            ("GOOSE_ALLOWLIST_BYPASS", "developer.server.allowlist_bypass", false),
            
            // Scheduler
            ("GOOSE_SCHEDULER_TYPE", "scheduler.type", false),
            ("GOOSE_TEMPORAL_BIN", "scheduler.temporal_bin", false),
            
            // Tracing
            ("LANGFUSE_URL", "tracing.langfuse.url", false),
            ("LANGFUSE_SECRET_KEY", "tracing.langfuse.secret_key", true),
            ("LANGFUSE_INIT_PROJECT_SECRET_KEY", "tracing.langfuse.init_project_secret_key", true),
        ];
        
        for (env_var, path, _is_secret) in env_mappings {
            if let Ok(value) = env::var(env_var) {
                let parsed_value = Self::parse_env_value(&value);
                overrides.insert(path.to_string(), parsed_value);
            }
        }
        
        overrides
    }
    
    /// Parse environment variable value based on the expected type
    fn parse_env_value(value: &str) -> Value {
        // Try to parse as JSON first
        if let Ok(json_value) = serde_json::from_str(value) {
            return json_value;
        }
        
        // Handle booleans
        match value.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => return Value::Bool(true),
            "false" | "0" | "no" | "off" => return Value::Bool(false),
            _ => {}
        }
        
        // Try to parse as number
        if let Ok(int_val) = value.parse::<i64>() {
            return Value::Number(int_val.into());
        }
        
        if let Ok(float_val) = value.parse::<f64>() {
            if let Some(num) = serde_json::Number::from_f64(float_val) {
                return Value::Number(num);
            }
        }
        
        // Default to string
        Value::String(value.to_string())
    }
    
    /// Get a configuration value with proper precedence
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Check layers in order: runtime > env > profile > user > system > defaults
        
        // 1. Runtime overrides
        {
            let runtime = self.runtime.read().await;
            if let Some(value) = runtime.get(path) {
                return Ok(serde_json::from_value(value.clone())?);
            }
        }
        
        // 2. Environment overrides
        if let Some(value) = self.environment.get(path) {
            return Ok(serde_json::from_value(value.clone())?);
        }
        
        // 3. Profile config
        if let Some(ref profile) = self.profile {
            if let Ok(value) = self.get_from_schema(profile, path) {
                return Ok(value);
            }
        }
        
        // 4. User config
        if let Some(ref user) = self.user {
            if let Ok(value) = self.get_from_schema(user, path) {
                return Ok(value);
            }
        }
        
        // 5. System config
        if let Some(ref system) = self.system {
            if let Ok(value) = self.get_from_schema(system, path) {
                return Ok(value);
            }
        }
        
        // 6. Defaults
        self.get_from_schema(&self.defaults, path)
    }
    
    /// Get a value from a ConfigSchema using a path
    fn get_from_schema<T>(&self, schema: &ConfigSchema, path: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Convert schema to JSON for path-based access
        let json = serde_json::to_value(schema)?;
        
        // Navigate the path
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &json;
        
        for part in parts {
            current = current.get(part)
                .ok_or_else(|| anyhow!("Path not found: {}", path))?;
        }
        
        Ok(serde_json::from_value(current.clone())?)
    }
    
    /// Set a runtime override
    pub async fn set_runtime(&self, path: &str, value: Value) -> Result<()> {
        let mut runtime = self.runtime.write().await;
        runtime.insert(path.to_string(), value);
        Ok(())
    }
    
    /// Load a profile
    pub async fn load_profile(&mut self, profile_name: &str) -> Result<()> {
        let profile_path = Self::get_profile_path(profile_name)?;
        if !profile_path.exists() {
            return Err(anyhow!("Profile '{}' not found", profile_name));
        }
        
        let content = tokio::fs::read_to_string(&profile_path).await?;
        let profile: ConfigSchema = serde_yaml::from_str(&content)?;
        
        self.profile = Some(profile);
        self.active_profile = Some(profile_name.to_string());
        
        Ok(())
    }
    
    /// Get the path to a profile
    fn get_profile_path(profile_name: &str) -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join("goose")
            .join("profiles");
        Ok(config_dir.join(format!("{}.yaml", profile_name)))
    }
    
    /// Get a secret value
    pub async fn get_secret(&self, path: &str) -> Result<String> {
        // Check if there's an environment override first
        if let Some(value) = self.environment.get(path) {
            if let Value::String(s) = value {
                return Ok(s.clone());
            }
        }
        
        // Extract the key name from the path
        // e.g., "providers.openai.api_key" -> "openai_api_key"
        let key = self.path_to_secret_key(path);
        
        // Use secrets manager to resolve
        self.secrets.get_secret(&key).await
    }
    
    /// Convert a config path to a secret key name
    fn path_to_secret_key(&self, path: &str) -> String {
        let parts: Vec<&str> = path.split('.').collect();
        match parts.as_slice() {
            ["providers", provider, field] => format!("{}_{}", provider, field),
            _ => path.replace('.', "_"),
        }
    }
    
    /// Get the merged configuration as a single schema
    pub async fn get_merged_config(&self) -> Result<ConfigSchema> {
        // Start with defaults
        let mut config = self.defaults.clone();
        
        // TODO: Implement proper merging logic
        // For now, just return the defaults with environment overrides applied
        
        Ok(config)
    }
    
    /// Save the current user configuration
    pub async fn save_user_config(&self) -> Result<()> {
        if let Some(ref user_config) = self.user {
            let config_path = Self::get_user_config_path()?;
            let content = serde_yaml::to_string(user_config)?;
            
            // Ensure directory exists
            if let Some(parent) = config_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            
            tokio::fs::write(&config_path, content).await?;
        }
        
        Ok(())
    }
    
    /// Get a provider configuration with resolved secrets
    pub async fn get_provider_config(&self, provider: &str) -> Result<Value> {
        let base_path = format!("providers.{}", provider);
        
        // Get the provider config section
        let mut config: Value = self.get(&base_path).await?;
        
        // Resolve secrets for known secret fields
        if let Value::Object(ref mut map) = config {
            // Check for api_key field
            if map.contains_key("api_key") {
                let secret_path = format!("{}.api_key", base_path);
                if let Ok(secret) = self.get_secret(&secret_path).await {
                    map.insert("api_key".to_string(), Value::String(secret));
                }
            }
            
            // Check for token field
            if map.contains_key("token") {
                let secret_path = format!("{}.token", base_path);
                if let Ok(secret) = self.get_secret(&secret_path).await {
                    map.insert("token".to_string(), Value::String(secret));
                }
            }
            
            // Check for secret_key field
            if map.contains_key("secret_key") {
                let secret_path = format!("{}.secret_key", base_path);
                if let Ok(secret) = self.get_secret(&secret_path).await {
                    map.insert("secret_key".to_string(), Value::String(secret));
                }
            }
            
            // Check for custom_headers field (marked as secret in schema)
            if map.contains_key("custom_headers") {
                let secret_path = format!("{}.custom_headers", base_path);
                if let Ok(secret) = self.get_secret(&secret_path).await {
                    map.insert("custom_headers".to_string(), Value::String(secret));
                }
            }
        }
        
        Ok(config)
    }
    
    /// Check if a field is marked as secret in the schema
    pub fn is_secret_field(&self, path: &str) -> bool {
        // This would be determined by the #[secret] attribute in the schema
        // For now, we'll hardcode known secret fields
        let secret_paths = vec![
            "providers.openai.api_key",
            "providers.openai.custom_headers",
            "providers.anthropic.api_key",
            "providers.azure.api_key",
            "providers.google.api_key",
            "providers.groq.api_key",
            "providers.xai.api_key",
            "developer.server.secret_key",
            "tracing.langfuse.secret_key",
            "tracing.langfuse.init_project_secret_key",
        ];
        
        secret_paths.contains(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_config_manager_defaults() {
        let manager = ConfigManager::new().await.unwrap();
        
        let provider: String = manager.get("core.provider").await.unwrap();
        assert_eq!(provider, "openai");
        
        let model: String = manager.get("core.model").await.unwrap();
        assert_eq!(model, "gpt-4o");
        
        let max_turns: u32 = manager.get("core.max_turns").await.unwrap();
        assert_eq!(max_turns, 1000);
    }
    
    #[tokio::test]
    async fn test_runtime_override() {
        let manager = ConfigManager::new().await.unwrap();
        
        // Set runtime override
        manager.set_runtime("core.provider", Value::String("anthropic".to_string())).await.unwrap();
        
        // Should get the override value
        let provider: String = manager.get("core.provider").await.unwrap();
        assert_eq!(provider, "anthropic");
    }
    
    #[test]
    fn test_env_var_parsing() {
        assert_eq!(
            ConfigManager::parse_env_value("true"),
            Value::Bool(true)
        );
        assert_eq!(
            ConfigManager::parse_env_value("false"),
            Value::Bool(false)
        );
        assert_eq!(
            ConfigManager::parse_env_value("42"),
            Value::Number(42.into())
        );
        assert_eq!(
            ConfigManager::parse_env_value("3.14"),
            Value::Number(serde_json::Number::from_f64(3.14).unwrap())
        );
        assert_eq!(
            ConfigManager::parse_env_value("hello"),
            Value::String("hello".to_string())
        );
    }
    
    #[test]
    fn test_is_secret_field() {
        let manager = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(ConfigManager::new())
            .unwrap();
        
        assert!(manager.is_secret_field("providers.openai.api_key"));
        assert!(manager.is_secret_field("developer.server.secret_key"));
        assert!(!manager.is_secret_field("core.provider"));
        assert!(!manager.is_secret_field("providers.openai.host"));
    }
}
