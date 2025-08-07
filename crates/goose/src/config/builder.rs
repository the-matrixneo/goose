use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Error types for configuration building
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Source not found: {0}")]
    SourceNotFound(String),

    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Environment variable error: {0}")]
    EnvVar(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Priority levels for configuration sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Lowest priority - default values
    Default = 0,
    /// System-wide configuration
    System = 1,
    /// User configuration files
    User = 2,
    /// Project-specific configuration
    Project = 3,
    /// Environment variables
    Environment = 4,
    /// Command-line arguments
    CommandLine = 5,
    /// Runtime overrides - highest priority
    Runtime = 6,
}

/// Configuration source trait for different source types
#[async_trait::async_trait]
pub trait ConfigSource: Send + Sync {
    /// Load configuration from this source
    async fn load(&self) -> ConfigResult<Value>;

    /// Get the name of this source for debugging
    fn name(&self) -> &str;
}

/// File-based configuration source
pub struct FileSource {
    path: PathBuf,
    format: FileFormat,
}

#[derive(Debug, Clone)]
pub enum FileFormat {
    Json,
    Toml,
    Yaml,
}

impl FileSource {
    pub fn new(path: impl AsRef<Path>, format: FileFormat) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            format,
        }
    }

    pub fn auto_detect(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();
        let format = match path.extension().and_then(|s| s.to_str()) {
            Some("json") => FileFormat::Json,
            Some("toml") => FileFormat::Toml,
            Some("yaml") | Some("yml") => FileFormat::Yaml,
            _ => {
                return Err(ConfigError::Parse(format!(
                    "Cannot detect format for file: {}",
                    path.display()
                )))
            }
        };
        Ok(Self::new(path, format))
    }
}

#[async_trait::async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> ConfigResult<Value> {
        let content = tokio::fs::read_to_string(&self.path).await?;

        match self.format {
            FileFormat::Json => Ok(serde_json::from_str(&content)?),
            FileFormat::Toml => {
                let value: toml::Value = toml::from_str(&content)?;
                Ok(serde_json::to_value(value)?)
            }
            FileFormat::Yaml => Ok(serde_yaml::from_str(&content)?),
        }
    }

    fn name(&self) -> &str {
        self.path.to_str().unwrap_or("unknown")
    }
}

/// Environment variable configuration source
pub struct EnvSource {
    prefix: String,
    separator: String,
}

impl EnvSource {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            separator: "_".to_string(),
        }
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

#[async_trait::async_trait]
impl ConfigSource for EnvSource {
    async fn load(&self) -> ConfigResult<Value> {
        let mut config = serde_json::Map::new();

        for (key, value) in std::env::vars() {
            if key.starts_with(&self.prefix) {
                let key = key
                    .strip_prefix(&self.prefix)
                    .unwrap()
                    .trim_start_matches(&self.separator)
                    .to_lowercase()
                    .replace(&self.separator, ".");

                if key.is_empty() {
                    continue;
                }

                // Try to parse as JSON first, then as string
                let parsed_value =
                    serde_json::from_str(&value).unwrap_or_else(|_| Value::String(value));

                // Handle nested keys
                let parts: Vec<&str> = key.split('.').collect();
                Self::set_nested(&mut config, &parts, parsed_value);
            }
        }

        Ok(Value::Object(config))
    }

    fn name(&self) -> &str {
        "environment"
    }
}

impl EnvSource {
    fn set_nested(map: &mut serde_json::Map<String, Value>, keys: &[&str], value: Value) {
        if keys.is_empty() {
            return;
        }

        if keys.len() == 1 {
            map.insert(keys[0].to_string(), value);
            return;
        }

        let key = keys[0].to_string();
        let rest = &keys[1..];

        let entry = map
            .entry(key)
            .or_insert_with(|| Value::Object(serde_json::Map::new()));

        if let Value::Object(nested_map) = entry {
            Self::set_nested(nested_map, rest, value);
        }
    }
}

/// In-memory configuration source
pub struct MemorySource {
    data: Value,
    name: String,
}

impl MemorySource {
    pub fn new(data: Value) -> Self {
        Self {
            data,
            name: "memory".to_string(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

#[async_trait::async_trait]
impl ConfigSource for MemorySource {
    async fn load(&self) -> ConfigResult<Value> {
        Ok(self.data.clone())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Validation function type
pub type ValidationFn = Box<dyn Fn(&Value) -> ConfigResult<()> + Send + Sync>;

/// Configuration source with priority
struct PrioritizedSource {
    source: Box<dyn ConfigSource>,
    priority: Priority,
}

/// The main configuration structure (placeholder - should be defined elsewhere)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GooseConfig {
    // This would contain actual configuration fields
    pub data: Value,
}

/// Configuration builder for constructing configuration with multiple sources
pub struct ConfigBuilder {
    sources: Arc<RwLock<Vec<PrioritizedSource>>>,
    validators: Arc<RwLock<Vec<ValidationFn>>>,
    defaults: Option<Value>,
    merge_strategy: MergeStrategy,
}

/// Strategy for merging configuration values
#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    /// Replace entire values (default)
    Replace,
    /// Deep merge objects, replace other types
    Deep,
    /// Deep merge objects and arrays
    DeepWithArrays,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            sources: Arc::new(RwLock::new(Vec::new())),
            validators: Arc::new(RwLock::new(Vec::new())),
            defaults: None,
            merge_strategy: MergeStrategy::Deep,
        }
    }

    /// Set default configuration values
    pub fn with_defaults(mut self, defaults: Value) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// Set the merge strategy
    pub fn with_merge_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.merge_strategy = strategy;
        self
    }

    /// Add a configuration source with specified priority
    pub async fn add_source(self, source: impl ConfigSource + 'static, priority: Priority) -> Self {
        let mut sources = self.sources.write().await;
        sources.push(PrioritizedSource {
            source: Box::new(source),
            priority,
        });
        drop(sources);
        self
    }

    /// Add a file source with auto-detected format
    pub async fn add_file(self, path: impl AsRef<Path>, priority: Priority) -> ConfigResult<Self> {
        let source = FileSource::auto_detect(path)?;
        Ok(self.add_source(source, priority).await)
    }

    /// Add environment variables as a source
    pub async fn add_env(self, prefix: impl Into<String>, priority: Priority) -> Self {
        let source = EnvSource::new(prefix);
        self.add_source(source, priority).await
    }

    /// Add in-memory configuration
    pub async fn add_memory(self, data: Value, priority: Priority) -> Self {
        let source = MemorySource::new(data);
        self.add_source(source, priority).await
    }

    /// Add a validation function
    pub async fn add_validator<F>(self, validator: F) -> Self
    where
        F: Fn(&Value) -> ConfigResult<()> + Send + Sync + 'static,
    {
        let mut validators = self.validators.write().await;
        validators.push(Box::new(validator));
        drop(validators);
        self
    }

    /// Build the final configuration
    pub async fn build(self) -> ConfigResult<Arc<GooseConfig>> {
        // Start with defaults if provided
        let mut merged = self
            .defaults
            .clone()
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));

        // Sort sources by priority
        let mut sources = self.sources.write().await;
        sources.sort_by_key(|s| s.priority);

        // Load and merge all sources
        for source in sources.iter() {
            match source.source.load().await {
                Ok(value) => {
                    merged = self.merge_values(merged, value)?;
                }
                Err(e) => {
                    // Log error but continue with other sources
                    eprintln!(
                        "Warning: Failed to load source '{}': {}",
                        source.source.name(),
                        e
                    );
                }
            }
        }

        // Run validators
        let validators = self.validators.read().await;
        for validator in validators.iter() {
            validator(&merged)?;
        }

        // Create the final configuration
        let config = GooseConfig { data: merged };
        Ok(Arc::new(config))
    }

    /// Build with strict mode - all sources must load successfully
    pub async fn build_strict(self) -> ConfigResult<Arc<GooseConfig>> {
        let mut merged = self
            .defaults
            .clone()
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));

        let mut sources = self.sources.write().await;
        sources.sort_by_key(|s| s.priority);

        for source in sources.iter() {
            let value = source.source.load().await?;
            merged = self.merge_values(merged, value)?;
        }

        let validators = self.validators.read().await;
        for validator in validators.iter() {
            validator(&merged)?;
        }

        let config = GooseConfig { data: merged };
        Ok(Arc::new(config))
    }

    /// Merge two configuration values according to the merge strategy
    fn merge_values(&self, base: Value, overlay: Value) -> ConfigResult<Value> {
        match self.merge_strategy {
            MergeStrategy::Replace => Ok(overlay),
            MergeStrategy::Deep => Self::deep_merge(base, overlay, false),
            MergeStrategy::DeepWithArrays => Self::deep_merge(base, overlay, true),
        }
    }

    /// Perform deep merge of two JSON values
    fn deep_merge(base: Value, overlay: Value, merge_arrays: bool) -> ConfigResult<Value> {
        match (base, overlay) {
            (Value::Object(mut base_map), Value::Object(overlay_map)) => {
                for (key, overlay_value) in overlay_map {
                    match base_map.get(&key) {
                        Some(base_value) => {
                            let merged =
                                Self::deep_merge(base_value.clone(), overlay_value, merge_arrays)?;
                            base_map.insert(key, merged);
                        }
                        None => {
                            base_map.insert(key, overlay_value);
                        }
                    }
                }
                Ok(Value::Object(base_map))
            }
            (Value::Array(mut base_arr), Value::Array(overlay_arr)) if merge_arrays => {
                base_arr.extend(overlay_arr);
                Ok(Value::Array(base_arr))
            }
            (_, overlay) => Ok(overlay),
        }
    }

    /// Clear all sources
    pub async fn clear_sources(self) -> Self {
        let mut sources = self.sources.write().await;
        sources.clear();
        drop(sources);
        self
    }

    /// Clear all validators
    pub async fn clear_validators(self) -> Self {
        let mut validators = self.validators.write().await;
        validators.clear();
        drop(validators);
        self
    }

    /// Get the number of registered sources
    pub async fn source_count(&self) -> usize {
        self.sources.read().await.len()
    }

    /// Get the number of registered validators
    pub async fn validator_count(&self) -> usize {
        self.validators.read().await.len()
    }
}

/// Helper functions for common validation scenarios
pub mod validators {
    use super::*;

    /// Validate that required fields exist
    pub fn required_fields(fields: Vec<String>) -> ValidationFn {
        Box::new(move |value| {
            if let Value::Object(map) = value {
                for field in &fields {
                    if !map.contains_key(field) {
                        return Err(ConfigError::Validation(format!(
                            "Required field '{}' is missing",
                            field
                        )));
                    }
                }
            } else {
                return Err(ConfigError::Validation(
                    "Configuration must be an object".to_string(),
                ));
            }
            Ok(())
        })
    }

    /// Validate that a field matches a specific type
    pub fn field_type(field: String, expected_type: &'static str) -> ValidationFn {
        Box::new(move |value| {
            if let Value::Object(map) = value {
                if let Some(field_value) = map.get(&field) {
                    let actual_type = match field_value {
                        Value::Null => "null",
                        Value::Bool(_) => "boolean",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Array(_) => "array",
                        Value::Object(_) => "object",
                    };

                    if actual_type != expected_type {
                        return Err(ConfigError::Validation(format!(
                            "Field '{}' must be of type '{}', got '{}'",
                            field, expected_type, actual_type
                        )));
                    }
                }
            }
            Ok(())
        })
    }

    /// Validate numeric range
    pub fn numeric_range(field: String, min: f64, max: f64) -> ValidationFn {
        Box::new(move |value| {
            if let Value::Object(map) = value {
                if let Some(Value::Number(n)) = map.get(&field) {
                    if let Some(v) = n.as_f64() {
                        if v < min || v > max {
                            return Err(ConfigError::Validation(format!(
                                "Field '{}' must be between {} and {}",
                                field, min, max
                            )));
                        }
                    }
                }
            }
            Ok(())
        })
    }

    /// Validate string pattern using regex
    pub fn string_pattern(field: String, pattern: regex::Regex) -> ValidationFn {
        Box::new(move |value| {
            if let Value::Object(map) = value {
                if let Some(Value::String(s)) = map.get(&field) {
                    if !pattern.is_match(s) {
                        return Err(ConfigError::Validation(format!(
                            "Field '{}' does not match required pattern",
                            field
                        )));
                    }
                }
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_builder_basic() {
        let config = ConfigBuilder::new()
            .with_defaults(json!({
                "app": {
                    "name": "goose",
                    "version": "1.0.0"
                }
            }))
            .build()
            .await
            .unwrap();

        assert_eq!(config.data["app"]["name"], json!("goose"));
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let low_priority = json!({
            "setting": "low"
        });

        let high_priority = json!({
            "setting": "high"
        });

        let config = ConfigBuilder::new()
            .add_memory(low_priority, Priority::Default)
            .await
            .add_memory(high_priority, Priority::Runtime)
            .await
            .build()
            .await
            .unwrap();

        assert_eq!(config.data["setting"], json!("high"));
    }

    #[tokio::test]
    async fn test_deep_merge() {
        let base = json!({
            "database": {
                "host": "localhost",
                "port": 5432
            }
        });

        let overlay = json!({
            "database": {
                "port": 3306,
                "user": "admin"
            }
        });

        let config = ConfigBuilder::new()
            .with_merge_strategy(MergeStrategy::Deep)
            .add_memory(base, Priority::Default)
            .await
            .add_memory(overlay, Priority::User)
            .await
            .build()
            .await
            .unwrap();

        let db = &config.data["database"];
        assert_eq!(db["host"], json!("localhost"));
        assert_eq!(db["port"], json!(3306));
        assert_eq!(db["user"], json!("admin"));
    }

    #[tokio::test]
    async fn test_validation() {
        let data = json!({
            "required_field": "value"
        });

        let result = ConfigBuilder::new()
            .add_memory(data, Priority::Default)
            .await
            .add_validator(validators::required_fields(vec![
                "required_field".to_string()
            ]))
            .await
            .build()
            .await;

        assert!(result.is_ok());

        let data_missing = json!({
            "other_field": "value"
        });

        let result = ConfigBuilder::new()
            .add_memory(data_missing, Priority::Default)
            .await
            .add_validator(validators::required_fields(vec![
                "required_field".to_string()
            ]))
            .await
            .build()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_env_source() {
        std::env::set_var("GOOSE_DATABASE_HOST", "production.db");
        std::env::set_var("GOOSE_DATABASE_PORT", "5432");

        let env_source = EnvSource::new("GOOSE");
        let value = env_source.load().await.unwrap();

        assert_eq!(value["database"]["host"], json!("production.db"));
        assert_eq!(value["database"]["port"], json!(5432));

        std::env::remove_var("GOOSE_DATABASE_HOST");
        std::env::remove_var("GOOSE_DATABASE_PORT");
    }

    #[tokio::test]
    async fn test_thread_safety() {
        let builder = Arc::new(ConfigBuilder::new());
        let builder1 = Arc::clone(&builder);
        let builder2 = Arc::clone(&builder);

        let handle1 = tokio::spawn(async move { builder1.source_count().await });

        let handle2 = tokio::spawn(async move { builder2.validator_count().await });

        let count1 = handle1.await.unwrap();
        let count2 = handle2.await.unwrap();

        assert_eq!(count1, 0);
        assert_eq!(count2, 0);
    }
}
