//! Configuration manager with layered configuration support
//! 
//! This module implements the new configuration system with:
//! - Layered configuration (defaults -> system -> user -> profile -> env -> runtime)
//! - Type-safe access to configuration values
//! - Automatic secret resolution
//! - Environment variable overrides

use super::base::{Config as LegacyConfig, ConfigError};
use super::env_mapping::{build_env_map, build_path_map, ENV_MAPPINGS};
use super::schema::*;
use super::secrets::SecretsManager;
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
    
    /// Legacy config for backwards compatibility
    legacy_config: Option<LegacyConfig>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub async fn new() -> Result<Self> {
        let legacy_config = LegacyConfig::global();
        let secrets = SecretsManager::new();
        
        // Load user config if it exists
        let user = Self::load_user_config().await?;
        
        // Load environment overrides
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
            legacy_config: Some(legacy_config.clone()),
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
    
    /// Load environment variable overrides
    fn load_environment_overrides() -> HashMap<String, Value> {
        let mut overrides = HashMap::new();
        let env_map = build_env_map();
        
        for (env_var, mapping) in env_map {
            if let Ok(value) = env::var(env_var) {
                // Parse the value appropriately
                let parsed_value = Self::parse_env_value(&value, mapping);
                overrides.insert(mapping.config_path.to_string(), parsed_value);
            }
        }
        
        overrides
    }
    
    /// Parse environment variable value based on the expected type
    fn parse_env_value(value: &str, mapping: &super::env_mapping::EnvMapping) -> Value {
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
            ["providers", provider, "api_key"] => format!("{}_api_key", provider),
            ["providers", provider, "token"] => format!("{}_token", provider),
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
        let secret_fields = ["api_key", "token", "secret_key", "custom_headers"];
        
        if let Value::Object(ref mut map) = config {
            for field in &secret_fields {
                let secret_path = format!("{}.{}", base_path, field);
                if map.contains_key(*field) {
                    if let Ok(secret) = self.get_secret(&secret_path).await {
                        map.insert(field.to_string(), Value::String(secret));
                    }
                }
            }
        }
        
        Ok(config)
    }
}

/// Backwards compatibility layer
impl ConfigManager {
    /// Get a parameter using the legacy config system
    pub fn get_param_legacy<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(ref legacy) = self.legacy_config {
            legacy.get_param(key)
        } else {
            Err(ConfigError::NotFound(key.to_string()))
        }
    }
    
    /// Get a secret using the legacy config system
    pub fn get_secret_legacy<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(ref legacy) = self.legacy_config {
            legacy.get_secret(key)
        } else {
            Err(ConfigError::NotFound(key.to_string()))
        }
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
    
    #[tokio::test]
    async fn test_env_var_parsing() {
        // Test boolean parsing
        let mapping = super::super::env_mapping::EnvMapping::new("TEST", "test", false);
        
        assert_eq!(
            ConfigManager::parse_env_value("true", &mapping),
            Value::Bool(true)
        );
        assert_eq!(
            ConfigManager::parse_env_value("false", &mapping),
            Value::Bool(false)
        );
        assert_eq!(
            ConfigManager::parse_env_value("1", &mapping),
            Value::Bool(true)
        );
        assert_eq!(
            ConfigManager::parse_env_value("0", &mapping),
            Value::Bool(false)
        );
        
        // Test number parsing
        assert_eq!(
            ConfigManager::parse_env_value("42", &mapping),
            Value::Number(42.into())
        );
        assert_eq!(
            ConfigManager::parse_env_value("3.14", &mapping),
            Value::Number(serde_json::Number::from_f64(3.14).unwrap())
        );
        
        // Test string parsing
        assert_eq!(
            ConfigManager::parse_env_value("hello", &mapping),
            Value::String("hello".to_string())
        );
    }
}
