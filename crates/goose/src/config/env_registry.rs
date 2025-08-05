use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

/// Centralized registry for environment variables loaded at process start.
/// This eliminates just-in-time environment variable access and provides
/// a structured way to manage all environment variables used by Goose.
pub struct EnvRegistry {
    /// All environment variables loaded at startup
    env_vars: HashMap<String, String>,
    /// Parsed values cache to avoid re-parsing
    parsed_cache: RwLock<HashMap<String, Value>>,
}

/// Global environment registry instance
pub static ENV_REGISTRY: Lazy<EnvRegistry> = Lazy::new(|| EnvRegistry::new());

/// Categories of environment variables used by Goose
#[derive(Debug, Clone)]
pub enum EnvCategory {
    /// Provider configuration (API keys, endpoints, etc.)
    Provider,
    /// Goose core configuration
    Core,
    /// UI/CLI configuration
    Interface,
    /// Development/debugging
    Debug,
    /// System environment (HOME, PATH, etc.)
    System,
    /// Scheduler configuration
    Scheduler,
    /// Extension/MCP configuration
    Extension,
    /// Tracing/observability
    Tracing,
}

/// Known environment variables with their categories and whether they're secrets
pub struct EnvVarSpec {
    pub name: &'static str,
    pub category: EnvCategory,
    pub is_secret: bool,
    pub description: &'static str,
}

/// Registry of all known environment variables used by Goose
pub const KNOWN_ENV_VARS: &[EnvVarSpec] = &[
    // Provider secrets
    EnvVarSpec {
        name: "OPENAI_API_KEY",
        category: EnvCategory::Provider,
        is_secret: true,
        description: "OpenAI API key",
    },
    EnvVarSpec {
        name: "ANTHROPIC_API_KEY",
        category: EnvCategory::Provider,
        is_secret: true,
        description: "Anthropic API key",
    },
    EnvVarSpec {
        name: "GOOGLE_API_KEY",
        category: EnvCategory::Provider,
        is_secret: true,
        description: "Google API key",
    },
    
    // Provider configuration
    EnvVarSpec {
        name: "GOOSE_MODEL",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Default model to use",
    },
    EnvVarSpec {
        name: "GOOSE_LEAD_MODEL",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Lead model for multi-model setup",
    },
    EnvVarSpec {
        name: "GOOSE_LEAD_PROVIDER",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Lead provider for multi-model setup",
    },
    EnvVarSpec {
        name: "GOOSE_LEAD_TURNS",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Number of turns for lead model",
    },
    EnvVarSpec {
        name: "GOOSE_LEAD_FAILURE_THRESHOLD",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Failure threshold for lead model",
    },
    EnvVarSpec {
        name: "GOOSE_LEAD_FALLBACK_TURNS",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Fallback turns for lead model",
    },
    
    // Core configuration
    EnvVarSpec {
        name: "GOOSE_CONTEXT_LIMIT",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Context limit for models",
    },
    EnvVarSpec {
        name: "GOOSE_WORKER_CONTEXT_LIMIT",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Context limit for worker models",
    },
    EnvVarSpec {
        name: "GOOSE_TEMPERATURE",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Temperature setting for models",
    },
    EnvVarSpec {
        name: "GOOSE_MODE",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Operating mode",
    },
    EnvVarSpec {
        name: "GOOSE_DISABLE_KEYRING",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Disable keyring for secret storage",
    },
    EnvVarSpec {
        name: "GOOSE_CACHE_DIR",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Cache directory path",
    },
    EnvVarSpec {
        name: "GOOSE_WORKING_DIR",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Working directory path",
    },
    
    // Toolshim configuration
    EnvVarSpec {
        name: "GOOSE_TOOLSHIM",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Enable toolshim functionality",
    },
    EnvVarSpec {
        name: "GOOSE_TOOLSHIM_OLLAMA_MODEL",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Ollama model for toolshim",
    },
    
    // Router configuration
    EnvVarSpec {
        name: "GOOSE_ROUTER_TOOL_SELECTION_STRATEGY",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Tool selection strategy for router",
    },
    EnvVarSpec {
        name: "GOOSE_EMBEDDING_MODEL_PROVIDER",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Embedding model provider",
    },
    EnvVarSpec {
        name: "GOOSE_EMBEDDING_MODEL",
        category: EnvCategory::Provider,
        is_secret: false,
        description: "Embedding model name",
    },
    
    // Interface configuration
    EnvVarSpec {
        name: "GOOSE_CLI_THEME",
        category: EnvCategory::Interface,
        is_secret: false,
        description: "CLI theme",
    },
    EnvVarSpec {
        name: "GOOSE_CLI_SHOW_THINKING",
        category: EnvCategory::Interface,
        is_secret: false,
        description: "Show thinking in CLI",
    },
    EnvVarSpec {
        name: "GOOSE_CLI_MIN_PRIORITY",
        category: EnvCategory::Interface,
        is_secret: false,
        description: "Minimum priority for CLI display",
    },
    
    // Scheduler configuration
    EnvVarSpec {
        name: "GOOSE_SCHEDULER_TYPE",
        category: EnvCategory::Scheduler,
        is_secret: false,
        description: "Scheduler type (temporal/legacy)",
    },
    EnvVarSpec {
        name: "GOOSE_TEMPORAL_BIN",
        category: EnvCategory::Scheduler,
        is_secret: false,
        description: "Path to temporal binary",
    },
    
    // Debug configuration
    EnvVarSpec {
        name: "CLAUDE_THINKING_ENABLED",
        category: EnvCategory::Debug,
        is_secret: false,
        description: "Enable Claude thinking mode",
    },
    EnvVarSpec {
        name: "CLAUDE_THINKING_BUDGET",
        category: EnvCategory::Debug,
        is_secret: false,
        description: "Claude thinking token budget",
    },
    EnvVarSpec {
        name: "GOOSE_GEMINI_CLI_DEBUG",
        category: EnvCategory::Debug,
        is_secret: false,
        description: "Enable Gemini CLI debug mode",
    },
    EnvVarSpec {
        name: "GOOSE_CLAUDE_CODE_DEBUG",
        category: EnvCategory::Debug,
        is_secret: false,
        description: "Enable Claude Code debug mode",
    },
    
    // Tracing configuration
    EnvVarSpec {
        name: "LANGFUSE_PUBLIC_KEY",
        category: EnvCategory::Tracing,
        is_secret: true,
        description: "Langfuse public key",
    },
    EnvVarSpec {
        name: "LANGFUSE_SECRET_KEY",
        category: EnvCategory::Tracing,
        is_secret: true,
        description: "Langfuse secret key",
    },
    EnvVarSpec {
        name: "LANGFUSE_URL",
        category: EnvCategory::Tracing,
        is_secret: false,
        description: "Langfuse URL",
    },
    EnvVarSpec {
        name: "LANGFUSE_INIT_PROJECT_PUBLIC_KEY",
        category: EnvCategory::Tracing,
        is_secret: true,
        description: "Langfuse init project public key",
    },
    EnvVarSpec {
        name: "LANGFUSE_INIT_PROJECT_SECRET_KEY",
        category: EnvCategory::Tracing,
        is_secret: true,
        description: "Langfuse init project secret key",
    },
    EnvVarSpec {
        name: "OTEL_EXPORTER_OTLP_ENDPOINT",
        category: EnvCategory::Tracing,
        is_secret: false,
        description: "OTLP exporter endpoint",
    },
    EnvVarSpec {
        name: "OTEL_EXPORTER_OTLP_TIMEOUT",
        category: EnvCategory::Tracing,
        is_secret: false,
        description: "OTLP exporter timeout",
    },
    
    // System environment
    EnvVarSpec {
        name: "HOME",
        category: EnvCategory::System,
        is_secret: false,
        description: "User home directory",
    },
    EnvVarSpec {
        name: "USER",
        category: EnvCategory::System,
        is_secret: false,
        description: "Current user name",
    },
    EnvVarSpec {
        name: "USERNAME",
        category: EnvCategory::System,
        is_secret: false,
        description: "Current user name (Windows)",
    },
    EnvVarSpec {
        name: "PATH",
        category: EnvCategory::System,
        is_secret: false,
        description: "System PATH",
    },
    EnvVarSpec {
        name: "TEMP",
        category: EnvCategory::System,
        is_secret: false,
        description: "Temporary directory",
    },
    EnvVarSpec {
        name: "DISPLAY",
        category: EnvCategory::System,
        is_secret: false,
        description: "X11 display",
    },
    EnvVarSpec {
        name: "WAYLAND_DISPLAY",
        category: EnvCategory::System,
        is_secret: false,
        description: "Wayland display",
    },
    
    // Extension/MCP configuration
    EnvVarSpec {
        name: "GOOGLE_DRIVE_OAUTH_PATH",
        category: EnvCategory::Extension,
        is_secret: false,
        description: "Google Drive OAuth path",
    },
    EnvVarSpec {
        name: "GOOGLE_DRIVE_CREDENTIALS_PATH",
        category: EnvCategory::Extension,
        is_secret: false,
        description: "Google Drive credentials path",
    },
    EnvVarSpec {
        name: "GOOGLE_DRIVE_OAUTH_CONFIG",
        category: EnvCategory::Extension,
        is_secret: true,
        description: "Google Drive OAuth config",
    },
    EnvVarSpec {
        name: "GOOSE_EDITOR_API_KEY",
        category: EnvCategory::Extension,
        is_secret: true,
        description: "Goose editor API key",
    },
    EnvVarSpec {
        name: "GOOSE_EDITOR_HOST",
        category: EnvCategory::Extension,
        is_secret: false,
        description: "Goose editor host",
    },
    EnvVarSpec {
        name: "GOOSE_EDITOR_MODEL",
        category: EnvCategory::Extension,
        is_secret: false,
        description: "Goose editor model",
    },
    EnvVarSpec {
        name: "CONTEXT_FILE_NAMES",
        category: EnvCategory::Extension,
        is_secret: false,
        description: "Context file names for developer extension",
    },
    
    // Testing configuration
    EnvVarSpec {
        name: "GOOSE_TEST_PROVIDER",
        category: EnvCategory::Debug,
        is_secret: false,
        description: "Provider for testing",
    },
    EnvVarSpec {
        name: "GITHUB_ACTIONS",
        category: EnvCategory::System,
        is_secret: false,
        description: "GitHub Actions environment indicator",
    },
    
    // Recipe configuration
    EnvVarSpec {
        name: "GOOSE_RECIPE_PATH",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Recipe search path",
    },
    
    // Subagent configuration
    EnvVarSpec {
        name: "GOOSE_SUBAGENT_MAX_TURNS",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Maximum turns for subagents",
    },
    
    // Server configuration
    EnvVarSpec {
        name: "GOOSE_SERVER__SECRET_KEY",
        category: EnvCategory::Core,
        is_secret: true,
        description: "Server secret key",
    },
    EnvVarSpec {
        name: "GOOSE_SERVER__MEMORY",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Enable server memory",
    },
    EnvVarSpec {
        name: "GOOSE_SERVER__COMPUTER_CONTROLLER",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Enable computer controller",
    },
    EnvVarSpec {
        name: "PORT",
        category: EnvCategory::System,
        is_secret: false,
        description: "Server port",
    },
    EnvVarSpec {
        name: "GOOSE_PORT",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Goose server port",
    },
    EnvVarSpec {
        name: "GOOSE_API_HOST",
        category: EnvCategory::Core,
        is_secret: false,
        description: "Goose API host",
    },
];

impl EnvRegistry {
    /// Create a new environment registry by loading all environment variables at startup
    pub fn new() -> Self {
        let env_vars = env::vars().collect();
        
        Self {
            env_vars,
            parsed_cache: RwLock::new(HashMap::new()),
        }
    }

    /// Refresh the environment registry by reloading all environment variables
    /// This is primarily for testing purposes when environment variables change
    #[cfg(test)]
    pub fn refresh(&mut self) {
        self.env_vars = env::vars().collect();
        if let Ok(mut cache) = self.parsed_cache.write() {
            cache.clear();
        }
    }

    /// Get an environment variable value as a string
    pub fn get_raw(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    /// Get an environment variable value with case-insensitive lookup
    /// First tries exact match, then uppercase
    pub fn get_raw_flexible(&self, key: &str) -> Option<&String> {
        // First try exact match
        if let Some(value) = self.env_vars.get(key) {
            return Some(value);
        }
        
        // Then try uppercase
        let upper_key = key.to_uppercase();
        self.env_vars.get(&upper_key)
    }

    /// Parse an environment variable value into a JSON Value with caching
    pub fn get_parsed(&self, key: &str) -> Option<Value> {
        // Check cache first
        {
            let cache = self.parsed_cache.read().ok()?;
            if let Some(cached) = cache.get(key) {
                return Some(cached.clone());
            }
        }

        // Get raw value and parse
        let raw_value = self.get_raw_flexible(key)?;
        let parsed = self.parse_env_value(raw_value).ok()?;

        // Cache the result
        {
            let mut cache = self.parsed_cache.write().ok()?;
            cache.insert(key.to_string(), parsed.clone());
        }

        Some(parsed)
    }

    /// Check if an environment variable exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.env_vars.contains_key(key) || self.env_vars.contains_key(&key.to_uppercase())
    }

    /// Get all environment variables in a specific category
    pub fn get_by_category(&self, _category: EnvCategory) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for spec in KNOWN_ENV_VARS {
            if std::mem::discriminant(&spec.category) == std::mem::discriminant(&_category) {
                if let Some(value) = self.get_raw(spec.name) {
                    result.insert(spec.name.to_string(), value.clone());
                }
            }
        }
        
        result
    }

    /// Get all secret environment variables
    pub fn get_secrets(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for spec in KNOWN_ENV_VARS {
            if spec.is_secret {
                if let Some(value) = self.get_raw(spec.name) {
                    result.insert(spec.name.to_string(), value.clone());
                }
            }
        }
        
        result
    }

    /// Get all non-secret environment variables
    pub fn get_params(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for spec in KNOWN_ENV_VARS {
            if !spec.is_secret {
                if let Some(value) = self.get_raw(spec.name) {
                    result.insert(spec.name.to_string(), value.clone());
                }
            }
        }
        
        result
    }

    /// Parse an environment variable value into a JSON Value
    /// This mirrors the logic from Config::parse_env_value
    fn parse_env_value(&self, val: &str) -> Result<Value, serde_json::Error> {
        // First try JSON parsing - this handles quoted strings, objects, arrays, etc.
        if let Ok(json_value) = serde_json::from_str(val) {
            return Ok(json_value);
        }

        let trimmed = val.trim();

        match trimmed.to_lowercase().as_str() {
            "true" => return Ok(Value::Bool(true)),
            "false" => return Ok(Value::Bool(false)),
            _ => {}
        }

        if let Ok(int_val) = trimmed.parse::<i64>() {
            return Ok(Value::Number(int_val.into()));
        }

        if let Ok(float_val) = trimmed.parse::<f64>() {
            if let Some(num) = serde_json::Number::from_f64(float_val) {
                return Ok(Value::Number(num));
            }
        }

        Ok(Value::String(val.to_string()))
    }

    /// Get diagnostic information about environment variables
    pub fn get_diagnostics(&self) -> EnvDiagnostics {
        let mut known_found = Vec::new();
        let mut known_missing = Vec::new();
        let mut unknown_goose_vars = Vec::new();

        // Check known variables
        for spec in KNOWN_ENV_VARS {
            if self.contains_key(spec.name) {
                known_found.push(spec.name);
            } else {
                known_missing.push(spec.name);
            }
        }

        // Find unknown GOOSE_* variables
        for (key, _) in &self.env_vars {
            if key.starts_with("GOOSE_") && !KNOWN_ENV_VARS.iter().any(|spec| spec.name == key) {
                unknown_goose_vars.push(key.clone());
            }
        }

        EnvDiagnostics {
            total_env_vars: self.env_vars.len(),
            known_found,
            known_missing,
            unknown_goose_vars,
        }
    }
}

/// Diagnostic information about environment variables
#[derive(Debug)]
pub struct EnvDiagnostics {
    pub total_env_vars: usize,
    pub known_found: Vec<&'static str>,
    pub known_missing: Vec<&'static str>,
    pub unknown_goose_vars: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_env_registry_creation() {
        let registry = EnvRegistry::new();
        assert!(!registry.env_vars.is_empty());
    }

    #[test]
    fn test_flexible_lookup() {
        env::set_var("TEST_FLEXIBLE_UPPER", "upper_value");
        let registry = EnvRegistry::new();
        
        // Should find with exact match
        assert_eq!(registry.get_raw_flexible("TEST_FLEXIBLE_UPPER"), Some(&"upper_value".to_string()));
        
        // Should find with case conversion
        assert_eq!(registry.get_raw_flexible("test_flexible_upper"), Some(&"upper_value".to_string()));
        
        env::remove_var("TEST_FLEXIBLE_UPPER");
    }

    #[test]
    fn test_value_parsing() {
        let registry = EnvRegistry::new();
        
        // Test boolean parsing
        let parsed = registry.parse_env_value("true").unwrap();
        assert_eq!(parsed, Value::Bool(true));
        
        // Test number parsing
        let parsed = registry.parse_env_value("42").unwrap();
        assert_eq!(parsed, Value::Number(42.into()));
        
        // Test string parsing
        let parsed = registry.parse_env_value("hello").unwrap();
        assert_eq!(parsed, Value::String("hello".to_string()));
    }

    #[test]
    fn test_category_filtering() {
        env::set_var("GOOSE_MODEL", "test_model");
        let registry = EnvRegistry::new();
        
        let provider_vars = registry.get_by_category(EnvCategory::Provider);
        assert!(provider_vars.contains_key("GOOSE_MODEL"));
        
        env::remove_var("GOOSE_MODEL");
    }

    #[test]
    fn test_secrets_filtering() {
        env::set_var("OPENAI_API_KEY", "test_key");
        let registry = EnvRegistry::new();
        
        let secrets = registry.get_secrets();
        assert!(secrets.contains_key("OPENAI_API_KEY"));
        
        let params = registry.get_params();
        assert!(!params.contains_key("OPENAI_API_KEY"));
        
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_diagnostics() {
        env::set_var("GOOSE_MODEL", "test_model");
        env::set_var("GOOSE_UNKNOWN_VAR", "unknown");
        let registry = EnvRegistry::new();
        
        let diagnostics = registry.get_diagnostics();
        assert!(diagnostics.known_found.contains(&"GOOSE_MODEL"));
        assert!(diagnostics.unknown_goose_vars.iter().any(|s| s == "GOOSE_UNKNOWN_VAR"));
        
        env::remove_var("GOOSE_MODEL");
        env::remove_var("GOOSE_UNKNOWN_VAR");
    }
}