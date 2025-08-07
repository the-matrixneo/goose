use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur when loading configuration from sources
#[derive(Debug, Error)]
pub enum ConfigSourceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Merge conflict: {0}")]
    MergeConflict(String),
}

pub type Result<T> = std::result::Result<T, ConfigSourceError>;

/// Priority levels for configuration sources
/// Higher values take precedence over lower values
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Default = 0,
    File = 1,
    Environment = 2,
    Cli = 3,
}

/// Partial configuration that can have missing fields
/// This allows each source to provide only the values it has
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartialConfig {
    // API Configuration
    pub api_key: Option<String>,
    pub api_endpoint: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,

    // System Configuration
    pub log_level: Option<String>,
    pub log_file: Option<PathBuf>,
    pub cache_dir: Option<PathBuf>,
    pub config_dir: Option<PathBuf>,

    // Feature Flags
    pub enable_telemetry: Option<bool>,
    pub enable_cache: Option<bool>,
    pub enable_plugins: Option<bool>,

    // Network Configuration
    pub timeout_seconds: Option<u64>,
    pub max_retries: Option<u32>,
    pub proxy_url: Option<String>,

    // Custom fields stored as raw values
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl PartialConfig {
    /// Merge another partial config into this one
    /// Values from `other` will override values in `self` if they are Some
    pub fn merge(mut self, other: PartialConfig) -> Self {
        if other.api_key.is_some() {
            self.api_key = other.api_key;
        }
        if other.api_endpoint.is_some() {
            self.api_endpoint = other.api_endpoint;
        }
        if other.model.is_some() {
            self.model = other.model;
        }
        if other.temperature.is_some() {
            self.temperature = other.temperature;
        }
        if other.max_tokens.is_some() {
            self.max_tokens = other.max_tokens;
        }
        if other.log_level.is_some() {
            self.log_level = other.log_level;
        }
        if other.log_file.is_some() {
            self.log_file = other.log_file;
        }
        if other.cache_dir.is_some() {
            self.cache_dir = other.cache_dir;
        }
        if other.config_dir.is_some() {
            self.config_dir = other.config_dir;
        }
        if other.enable_telemetry.is_some() {
            self.enable_telemetry = other.enable_telemetry;
        }
        if other.enable_cache.is_some() {
            self.enable_cache = other.enable_cache;
        }
        if other.enable_plugins.is_some() {
            self.enable_plugins = other.enable_plugins;
        }
        if other.timeout_seconds.is_some() {
            self.timeout_seconds = other.timeout_seconds;
        }
        if other.max_retries.is_some() {
            self.max_retries = other.max_retries;
        }
        if other.proxy_url.is_some() {
            self.proxy_url = other.proxy_url;
        }

        // Merge custom fields
        if let Some(other_custom) = other.custom {
            match &mut self.custom {
                Some(self_custom) => {
                    self_custom.extend(other_custom);
                }
                None => {
                    self.custom = Some(other_custom);
                }
            }
        }

        self
    }

    /// Check if this config is completely empty (all fields are None)
    pub fn is_empty(&self) -> bool {
        self.api_key.is_none()
            && self.api_endpoint.is_none()
            && self.model.is_none()
            && self.temperature.is_none()
            && self.max_tokens.is_none()
            && self.log_level.is_none()
            && self.log_file.is_none()
            && self.cache_dir.is_none()
            && self.config_dir.is_none()
            && self.enable_telemetry.is_none()
            && self.enable_cache.is_none()
            && self.enable_plugins.is_none()
            && self.timeout_seconds.is_none()
            && self.max_retries.is_none()
            && self.proxy_url.is_none()
            && self.custom.as_ref().is_none_or(|c| c.is_empty())
    }
}

/// Trait for configuration sources
#[async_trait]
pub trait ConfigSource: Send + Sync {
    /// Load configuration from this source
    async fn load(&self) -> Result<PartialConfig>;

    /// Get the priority of this source
    fn priority(&self) -> Priority;

    /// Get a human-readable name for this source
    fn name(&self) -> &str;
}

/// File-based configuration source supporting YAML and TOML
pub struct FileSource {
    path: PathBuf,
    format: FileFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Yaml,
    Toml,
}

impl FileSource {
    /// Create a new file source with automatic format detection
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let format = Self::detect_format(&path)?;
        Ok(Self { path, format })
    }

    /// Create a new file source with explicit format
    pub fn with_format(path: impl AsRef<Path>, format: FileFormat) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            format,
        }
    }

    /// Detect file format from extension
    fn detect_format(path: &Path) -> Result<FileFormat> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| ConfigSourceError::UnsupportedFormat("No file extension".to_string()))?;

        match extension.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            "toml" => Ok(FileFormat::Toml),
            ext => Err(ConfigSourceError::UnsupportedFormat(ext.to_string())),
        }
    }
}

#[async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> Result<PartialConfig> {
        // Check if file exists
        if !self.path.exists() {
            return Err(ConfigSourceError::FileNotFound(self.path.clone()));
        }

        // Read file contents
        let contents = tokio::fs::read_to_string(&self.path).await?;

        // Parse based on format
        let config = match self.format {
            FileFormat::Yaml => serde_yaml::from_str(&contents)?,
            FileFormat::Toml => toml::from_str(&contents)?,
        };

        Ok(config)
    }

    fn priority(&self) -> Priority {
        Priority::File
    }

    fn name(&self) -> &str {
        "file"
    }
}

/// Environment variable configuration source
pub struct EnvSource {
    prefix: String,
}

impl EnvSource {
    /// Create a new environment source with the default GOOSE_ prefix
    pub fn new() -> Self {
        Self::with_prefix("GOOSE")
    }

    /// Create a new environment source with a custom prefix
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }

    /// Parse an environment variable value based on its name
    fn parse_env_value(&self, _key: &str, value: &str) -> Result<Option<serde_json::Value>> {
        // Try to parse as JSON first (for complex types)
        if let Ok(json_value) = serde_json::from_str(value) {
            return Ok(Some(json_value));
        }

        // Try to parse as boolean
        if let Ok(bool_value) = value.parse::<bool>() {
            return Ok(Some(serde_json::Value::Bool(bool_value)));
        }

        // Try to parse as number
        if let Ok(int_value) = value.parse::<i64>() {
            return Ok(Some(serde_json::Value::Number(int_value.into())));
        }
        if let Ok(float_value) = value.parse::<f64>() {
            if let Some(num) = serde_json::Number::from_f64(float_value) {
                return Ok(Some(serde_json::Value::Number(num)));
            }
        }

        // Default to string
        Ok(Some(serde_json::Value::String(value.to_string())))
    }
}

impl Default for EnvSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConfigSource for EnvSource {
    async fn load(&self) -> Result<PartialConfig> {
        let mut config = PartialConfig::default();
        let prefix_with_underscore = format!("{}_", self.prefix);

        // Iterate through environment variables
        for (key, value) in std::env::vars() {
            if !key.starts_with(&prefix_with_underscore) {
                continue;
            }

            // Remove prefix and convert to lowercase
            let config_key = key[prefix_with_underscore.len()..]
                .to_lowercase()
                .replace('_', ".");

            // Map environment variables to config fields
            match config_key.as_str() {
                "api.key" | "api_key" => config.api_key = Some(value),
                "api.endpoint" | "api_endpoint" => config.api_endpoint = Some(value),
                "model" => config.model = Some(value),
                "temperature" => {
                    config.temperature = value.parse().ok();
                }
                "max.tokens" | "max_tokens" => {
                    config.max_tokens = value.parse().ok();
                }
                "log.level" | "log_level" => config.log_level = Some(value),
                "log.file" | "log_file" => config.log_file = Some(PathBuf::from(value)),
                "cache.dir" | "cache_dir" => config.cache_dir = Some(PathBuf::from(value)),
                "config.dir" | "config_dir" => config.config_dir = Some(PathBuf::from(value)),
                "enable.telemetry" | "enable_telemetry" => {
                    config.enable_telemetry = value.parse().ok();
                }
                "enable.cache" | "enable_cache" => {
                    config.enable_cache = value.parse().ok();
                }
                "enable.plugins" | "enable_plugins" => {
                    config.enable_plugins = value.parse().ok();
                }
                "timeout.seconds" | "timeout_seconds" => {
                    config.timeout_seconds = value.parse().ok();
                }
                "max.retries" | "max_retries" => {
                    config.max_retries = value.parse().ok();
                }
                "proxy.url" | "proxy_url" => config.proxy_url = Some(value),
                _ => {
                    // Store unknown keys in custom map
                    if let Some(parsed_value) = self.parse_env_value(&config_key, &value)? {
                        config
                            .custom
                            .get_or_insert_with(HashMap::new)
                            .insert(config_key, parsed_value);
                    }
                }
            }
        }

        Ok(config)
    }

    fn priority(&self) -> Priority {
        Priority::Environment
    }

    fn name(&self) -> &str {
        "environment"
    }
}

/// Command-line argument configuration source
pub struct CliSource {
    args: CliArgs,
}

/// Command-line arguments structure
#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub api_key: Option<String>,
    pub api_endpoint: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub log_level: Option<String>,
    pub log_file: Option<PathBuf>,
    pub cache_dir: Option<PathBuf>,
    pub config_dir: Option<PathBuf>,
    pub enable_telemetry: Option<bool>,
    pub enable_cache: Option<bool>,
    pub enable_plugins: Option<bool>,
    pub timeout_seconds: Option<u64>,
    pub max_retries: Option<u32>,
    pub proxy_url: Option<String>,
    pub custom: Option<HashMap<String, String>>,
}

impl CliSource {
    /// Create a new CLI source from parsed arguments
    pub fn new(args: CliArgs) -> Self {
        Self { args }
    }

    /// Parse command-line arguments using clap (example implementation)
    pub fn from_clap() -> Self {
        // This would be implemented when clap feature is enabled
        // For now, return default CLI args
        Self::new(CliArgs::default())

        /* Example implementation with clap:
        use clap::Parser;

        #[derive(Parser)]
        #[command(name = "goose")]
        #[command(about = "Goose configuration CLI")]
        struct Cli {
            #[arg(long, env = "GOOSE_API_KEY")]
            api_key: Option<String>,

            #[arg(long)]
            api_endpoint: Option<String>,

            #[arg(long)]
            model: Option<String>,

            #[arg(long)]
            temperature: Option<f32>,

            #[arg(long)]
            max_tokens: Option<usize>,

            #[arg(long)]
            log_level: Option<String>,

            #[arg(long)]
            log_file: Option<PathBuf>,

            #[arg(long)]
            cache_dir: Option<PathBuf>,

            #[arg(long)]
            config_dir: Option<PathBuf>,

            #[arg(long)]
            enable_telemetry: Option<bool>,

            #[arg(long)]
            enable_cache: Option<bool>,

            #[arg(long)]
            enable_plugins: Option<bool>,

            #[arg(long)]
            timeout_seconds: Option<u64>,

            #[arg(long)]
            max_retries: Option<u32>,

            #[arg(long)]
            proxy_url: Option<String>,
        }

        let cli = Cli::parse();

        Self::new(CliArgs {
            api_key: cli.api_key,
            api_endpoint: cli.api_endpoint,
            model: cli.model,
            temperature: cli.temperature,
            max_tokens: cli.max_tokens,
            log_level: cli.log_level,
            log_file: cli.log_file,
            cache_dir: cli.cache_dir,
            config_dir: cli.config_dir,
            enable_telemetry: cli.enable_telemetry,
            enable_cache: cli.enable_cache,
            enable_plugins: cli.enable_plugins,
            timeout_seconds: cli.timeout_seconds,
            max_retries: cli.max_retries,
            proxy_url: cli.proxy_url,
            custom: None,
        })
        */
    }
}

#[async_trait]
impl ConfigSource for CliSource {
    async fn load(&self) -> Result<PartialConfig> {
        let mut config = PartialConfig {
            api_key: self.args.api_key.clone(),
            api_endpoint: self.args.api_endpoint.clone(),
            model: self.args.model.clone(),
            temperature: self.args.temperature,
            max_tokens: self.args.max_tokens,
            log_level: self.args.log_level.clone(),
            log_file: self.args.log_file.clone(),
            cache_dir: self.args.cache_dir.clone(),
            config_dir: self.args.config_dir.clone(),
            enable_telemetry: self.args.enable_telemetry,
            enable_cache: self.args.enable_cache,
            enable_plugins: self.args.enable_plugins,
            timeout_seconds: self.args.timeout_seconds,
            max_retries: self.args.max_retries,
            proxy_url: self.args.proxy_url.clone(),
            custom: None,
        };

        // Convert custom string map to JSON values
        if let Some(custom_args) = &self.args.custom {
            let mut custom_map = HashMap::new();
            for (key, value) in custom_args {
                // Try to parse as JSON, otherwise use as string
                let json_value = serde_json::from_str(value)
                    .unwrap_or_else(|_| serde_json::Value::String(value.clone()));
                custom_map.insert(key.clone(), json_value);
            }
            config.custom = Some(custom_map);
        }

        Ok(config)
    }

    fn priority(&self) -> Priority {
        Priority::Cli
    }

    fn name(&self) -> &str {
        "cli"
    }
}

/// Configuration loader that combines multiple sources with priority
pub struct ConfigLoader {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl ConfigLoader {
    /// Create a new config loader
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Add a configuration source
    pub fn add_source(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.sources.push(source);
        self
    }

    /// Load configuration from all sources and merge based on priority
    pub async fn load(&self) -> Result<PartialConfig> {
        // Sort sources by priority (lowest first)
        let mut source_configs: Vec<(Priority, PartialConfig)> = Vec::new();

        for source in &self.sources {
            match source.load().await {
                Ok(config) => {
                    if !config.is_empty() {
                        source_configs.push((source.priority(), config));
                    }
                }
                Err(e) => {
                    // Log error but continue with other sources
                    eprintln!(
                        "Warning: Failed to load config from {}: {}",
                        source.name(),
                        e
                    );
                }
            }
        }

        // Sort by priority (lowest first)
        source_configs.sort_by_key(|(priority, _)| *priority);

        // Merge configurations
        let mut result = PartialConfig::default();
        for (_, config) in source_configs {
            result = result.merge(config);
        }

        Ok(result)
    }

    /// Create a standard loader with file, env, and CLI sources
    pub fn standard(config_file: Option<PathBuf>, cli_args: Option<CliArgs>) -> Result<Self> {
        let mut loader = Self::new();

        // Add file source if provided
        if let Some(file_path) = config_file {
            if file_path.exists() {
                loader = loader.add_source(Box::new(FileSource::new(file_path)?));
            }
        }

        // Add environment source
        loader = loader.add_source(Box::new(EnvSource::new()));

        // Add CLI source if provided
        if let Some(args) = cli_args {
            loader = loader.add_source(Box::new(CliSource::new(args)));
        }

        Ok(loader)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio;

    #[tokio::test]
    async fn test_partial_config_merge() {
        let mut config1 = PartialConfig::default();
        config1.api_key = Some("key1".to_string());
        config1.model = Some("model1".to_string());

        let mut config2 = PartialConfig::default();
        config2.api_key = Some("key2".to_string());
        config2.temperature = Some(0.7);

        let merged = config1.merge(config2);
        assert_eq!(merged.api_key, Some("key2".to_string()));
        assert_eq!(merged.model, Some("model1".to_string()));
        assert_eq!(merged.temperature, Some(0.7));
    }

    #[tokio::test]
    async fn test_file_source_yaml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.yaml");

        let yaml_content = r#"
api_key: test_key
model: gpt-4
temperature: 0.8
enable_cache: true
"#;

        std::fs::write(&file_path, yaml_content).unwrap();

        let source = FileSource::new(&file_path).unwrap();
        let config = source.load().await.unwrap();

        assert_eq!(config.api_key, Some("test_key".to_string()));
        assert_eq!(config.model, Some("gpt-4".to_string()));
        assert_eq!(config.temperature, Some(0.8));
        assert_eq!(config.enable_cache, Some(true));
    }

    #[tokio::test]
    async fn test_file_source_toml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        let toml_content = r#"
api_key = "test_key"
model = "gpt-4"
temperature = 0.8
enable_cache = true
"#;

        std::fs::write(&file_path, toml_content).unwrap();

        let source = FileSource::new(&file_path).unwrap();
        let config = source.load().await.unwrap();

        assert_eq!(config.api_key, Some("test_key".to_string()));
        assert_eq!(config.model, Some("gpt-4".to_string()));
        assert_eq!(config.temperature, Some(0.8));
        assert_eq!(config.enable_cache, Some(true));
    }

    #[tokio::test]
    async fn test_env_source() {
        std::env::set_var("GOOSE_API_KEY", "env_key");
        std::env::set_var("GOOSE_MODEL", "claude");
        std::env::set_var("GOOSE_TEMPERATURE", "0.5");
        std::env::set_var("GOOSE_ENABLE_CACHE", "false");

        let source = EnvSource::new();
        let config = source.load().await.unwrap();

        assert_eq!(config.api_key, Some("env_key".to_string()));
        assert_eq!(config.model, Some("claude".to_string()));
        assert_eq!(config.temperature, Some(0.5));
        assert_eq!(config.enable_cache, Some(false));

        // Clean up
        std::env::remove_var("GOOSE_API_KEY");
        std::env::remove_var("GOOSE_MODEL");
        std::env::remove_var("GOOSE_TEMPERATURE");
        std::env::remove_var("GOOSE_ENABLE_CACHE");
    }

    #[tokio::test]
    async fn test_cli_source() {
        let args = CliArgs {
            api_key: Some("cli_key".to_string()),
            model: Some("llama".to_string()),
            temperature: Some(0.3),
            enable_telemetry: Some(true),
            ..Default::default()
        };

        let source = CliSource::new(args);
        let config = source.load().await.unwrap();

        assert_eq!(config.api_key, Some("cli_key".to_string()));
        assert_eq!(config.model, Some("llama".to_string()));
        assert_eq!(config.temperature, Some(0.3));
        assert_eq!(config.enable_telemetry, Some(true));
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        assert!(Priority::Default < Priority::File);
        assert!(Priority::File < Priority::Environment);
        assert!(Priority::Environment < Priority::Cli);
    }

    #[tokio::test]
    async fn test_config_loader_priority() {
        // Set up environment variable
        std::env::set_var("GOOSE_API_KEY", "env_key");
        std::env::set_var("GOOSE_MODEL", "env_model");

        // Create CLI args
        let cli_args = CliArgs {
            api_key: Some("cli_key".to_string()),
            ..Default::default()
        };

        // Create loader with env and CLI sources
        let loader = ConfigLoader::new()
            .add_source(Box::new(EnvSource::new()))
            .add_source(Box::new(CliSource::new(cli_args)));

        let config = loader.load().await.unwrap();

        // CLI should override env for api_key
        assert_eq!(config.api_key, Some("cli_key".to_string()));
        // But env value should be used for model
        assert_eq!(config.model, Some("env_model".to_string()));

        // Clean up
        std::env::remove_var("GOOSE_API_KEY");
        std::env::remove_var("GOOSE_MODEL");
    }

    #[tokio::test]
    async fn test_custom_fields() {
        let mut custom = HashMap::new();
        custom.insert("custom_field".to_string(), "custom_value".to_string());

        let args = CliArgs {
            custom: Some(custom),
            ..Default::default()
        };

        let source = CliSource::new(args);
        let config = source.load().await.unwrap();

        assert!(config.custom.is_some());
        let custom_map = config.custom.unwrap();
        assert_eq!(
            custom_map.get("custom_field"),
            Some(&serde_json::Value::String("custom_value".to_string()))
        );
    }

    #[tokio::test]
    async fn test_empty_config() {
        let config = PartialConfig::default();
        assert!(config.is_empty());

        let mut config2 = PartialConfig::default();
        config2.api_key = Some("key".to_string());
        assert!(!config2.is_empty());
    }
}
