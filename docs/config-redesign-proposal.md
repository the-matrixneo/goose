# Goose Configuration Redesign Proposal

## Overview

This proposal outlines a comprehensive redesign of the Goose configuration system to address current issues with environment variable management, configuration structure, and user experience.

## Current Issues

1. **Environment Variable Chaos**
   - Environment variables are read directly using `std::env::var()` throughout the codebase
   - No centralized management or validation
   - Confusing for desktop users when environment gets messed up
   - Examples found:
     - `GOOSE_PROVIDER`, `GOOSE_MODEL`, `GOOSE_CONTEXT_LIMIT`
     - `GOOSE_TEMPERATURE`, `GOOSE_TOOLSHIM`, `GOOSE_MODE`
     - `OPENAI_API_KEY`, `ANTHROPIC_HOST`, `AZURE_OPENAI_ENDPOINT`
     - Many more scattered throughout providers

2. **Mixed Configuration Structure**
   - Tool configs, KV variables, and provider configs are nested together
   - No clear separation of concerns
   - Extensions have their own nested structure

3. **No Configuration Schema**
   - Configuration keys are defined ad-hoc
   - No central registry of all possible configuration options
   - No defaults defined in one place

4. **Desktop User Experience**
   - Environment variables are not user-friendly for GUI applications
   - No clear way to manage profiles or switch between configurations

## Proposed Solution

### 1. Configuration Schema Definition

Create a centralized configuration schema that defines all possible configuration options:

```rust
// crates/goose/src/config/schema.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSchema {
    pub core: CoreConfig,
    pub providers: ProvidersConfig,
    pub extensions: ExtensionsConfig,
    pub ui: UIConfig,
    pub developer: DeveloperConfig,
    pub scheduler: SchedulerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    
    #[serde(default = "default_model")]
    pub model: String,
    
    #[serde(default = "default_mode")]
    pub mode: GooseMode, // enum: Auto, FastApply, etc.
    
    #[serde(default)]
    pub context_limit: Option<usize>,
    
    #[serde(default)]
    pub temperature: Option<f32>,
    
    #[serde(default)]
    pub system_prompt_file: Option<PathBuf>,
    
    #[serde(default = "default_context_strategy")]
    pub context_strategy: ContextStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersConfig {
    pub openai: Option<OpenAIConfig>,
    pub anthropic: Option<AnthropicConfig>,
    pub azure: Option<AzureConfig>,
    // ... other providers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    // Secret fields use SecretString type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    #[serde(default)]
    pub host: Option<String>,
    
    #[serde(default)]
    pub organization: Option<String>,
    
    #[serde(default)]
    pub project: Option<String>,
    
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretString>,
    
    #[serde(default)]
    pub host: Option<String>,
    
    #[serde(default = "default_anthropic_version")]
    pub version: String,
}

// Usage example showing how secrets are resolved
impl ConfigManager {
    pub async fn get_provider_config(&self, provider: &str) -> Result<ProviderConfig, ConfigError> {
        let config = self.get_merged_config()?;
        
        match provider {
            "openai" => {
                let mut cfg = config.providers.openai.ok_or(ConfigError::NotFound)?;
                
                // Resolve the API key secret
                if cfg.api_key.is_none() {
                    // Try to get from secrets manager using conventional key name
                    cfg.api_key = Some(SecretString::from_keyring("openai_api_key"));
                }
                
                Ok(cfg.into())
            }
            // ... other providers
        }
    }
}
```

### 2. Configuration Layers

Implement a layered configuration system with clear precedence:

```rust
// crates/goose/src/config/layers.rs

pub struct ConfigManager {
    // Base layer: Built-in defaults
    defaults: ConfigSchema,
    
    // System layer: System-wide config (e.g., /etc/goose/config.yaml)
    system: Option<ConfigSchema>,
    
    // User layer: User config (~/.config/goose/config.yaml)
    user: Option<ConfigSchema>,
    
    // Profile layer: Active profile config
    profile: Option<ConfigSchema>,
    
    // Environment layer: Environment variable overrides
    environment: Option<ConfigSchema>,
    
    // Runtime layer: Programmatic overrides
    runtime: Option<ConfigSchema>,
}

impl ConfigManager {
    /// Get a configuration value with proper precedence
    pub fn get<T>(&self, path: &str) -> Result<T, ConfigError> {
        // Check layers in order: runtime > env > profile > user > system > defaults
        // Use a path-based lookup system like "core.provider" or "providers.openai.api_key"
    }
}
```

### 3. Profile Support

Add profile management for easy configuration switching:

```yaml
# ~/.config/goose/profiles/work.yaml
core:
  provider: azure
  model: gpt-4o

providers:
  azure:
    endpoint: https://company.openai.azure.com/
    deployment_name: gpt-4o-deployment

# ~/.config/goose/profiles/personal.yaml  
core:
  provider: anthropic
  model: claude-3-5-sonnet-latest

providers:
  anthropic:
    api_key: ${ANTHROPIC_API_KEY}  # Support env var references
```

### 4. Secrets Management

Implement a robust secrets management system with multiple fallback options:

```rust
// crates/goose/src/config/secrets.rs

#[derive(Debug, Clone)]
pub enum SecretValue {
    /// Direct value (only for non-production use)
    Direct(String),
    /// Reference to keyring
    Keyring { service: String, key: String },
    /// Reference to environment variable
    EnvVar(String),
    /// Reference to file
    File(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretString {
    /// Plain string for serialization (never contains actual secret)
    Reference(String), // e.g., "keyring:goose/openai_api_key" or "env:OPENAI_API_KEY"
}

impl SecretString {
    /// Resolve the secret value with fallback chain
    pub async fn resolve(&self) -> Result<String, ConfigError> {
        match &self.0 {
            SecretValue::Keyring { service, key } => {
                // Try keyring first
                match self.get_from_keyring(service, key).await {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        // Fallback to environment variable
                        if let Ok(value) = env::var(key.to_uppercase()) {
                            Ok(value)
                        } else {
                            Err(ConfigError::SecretNotFound(key.clone()))
                        }
                    }
                }
            }
            SecretValue::EnvVar(var) => {
                env::var(var).map_err(|_| ConfigError::SecretNotFound(var.clone()))
            }
            SecretValue::File(path) => {
                fs::read_to_string(path).map_err(|e| ConfigError::FileError(e))
            }
            SecretValue::Direct(value) => {
                // Log warning about insecure storage
                tracing::warn!("Using direct secret value - not recommended for production");
                Ok(value.clone())
            }
        }
    }
}

/// Secret resolution priority:
/// 1. Keyring (if available and not disabled)
/// 2. Environment variable
/// 3. Secrets file (if keyring disabled)
/// 4. Config file reference (least secure)
pub struct SecretsManager {
    keyring_enabled: bool,
    keyring_service: String,
    secrets_file: Option<PathBuf>,
    env_prefix: String,
}

impl SecretsManager {
    pub fn new() -> Self {
        Self {
            keyring_enabled: env::var("GOOSE_DISABLE_KEYRING").is_err(),
            keyring_service: "goose".to_string(),
            secrets_file: Some(Config::global().config_path.parent().unwrap().join("secrets.yaml")),
            env_prefix: "GOOSE_".to_string(),
        }
    }
    
    /// Get a secret with automatic fallback
    pub async fn get_secret(&self, key: &str) -> Result<String, ConfigError> {
        // 1. Try keyring first (if enabled)
        if self.keyring_enabled {
            if let Ok(value) = self.get_from_keyring(key).await {
                return Ok(value);
            }
        }
        
        // 2. Try environment variable
        let env_key = self.to_env_var_name(key);
        if let Ok(value) = env::var(&env_key) {
            return Ok(value);
        }
        
        // 3. Try secrets file (if keyring disabled)
        if !self.keyring_enabled {
            if let Some(ref secrets_file) = self.secrets_file {
                if let Ok(value) = self.get_from_secrets_file(secrets_file, key).await {
                    return Ok(value);
                }
            }
        }
        
        Err(ConfigError::SecretNotFound(key.to_string()))
    }
}
```

### 5. Environment Variable Mapping

Create a clear, documented mapping between environment variables and config paths:

```rust
// crates/goose/src/config/env_mapping.rs

pub struct EnvMapping {
    pub env_var: &'static str,
    pub config_path: &'static str,
    pub is_secret: bool,
}

pub const ENV_MAPPINGS: &[EnvMapping] = &[
    // Core mappings
    EnvMapping { env_var: "GOOSE_PROVIDER", config_path: "core.provider", is_secret: false },
    EnvMapping { env_var: "GOOSE_MODEL", config_path: "core.model", is_secret: false },
    EnvMapping { env_var: "GOOSE_MODE", config_path: "core.mode", is_secret: false },
    EnvMapping { env_var: "GOOSE_CONTEXT_LIMIT", config_path: "core.context_limit", is_secret: false },
    
    // Provider secrets
    EnvMapping { env_var: "OPENAI_API_KEY", config_path: "providers.openai.api_key", is_secret: true },
    EnvMapping { env_var: "ANTHROPIC_API_KEY", config_path: "providers.anthropic.api_key", is_secret: true },
    EnvMapping { env_var: "AZURE_OPENAI_API_KEY", config_path: "providers.azure.api_key", is_secret: true },
    
    // Provider configs
    EnvMapping { env_var: "OPENAI_HOST", config_path: "providers.openai.host", is_secret: false },
    EnvMapping { env_var: "AZURE_OPENAI_ENDPOINT", config_path: "providers.azure.endpoint", is_secret: false },
    
    // UI mappings
    EnvMapping { env_var: "GOOSE_CLI_THEME", config_path: "ui.cli.theme", is_secret: false },
    EnvMapping { env_var: "GOOSE_CLI_SHOW_COST", config_path: "ui.cli.show_cost", is_secret: false },
];
```

### 6. Migration Strategy

1. **Phase 1: Build New System**
   - Implement new config schema and manager
   - Add compatibility layer that reads from old config
   - All new code uses new system

2. **Phase 2: Migrate Existing Code**
   - Replace all `std::env::var()` calls with config manager
   - Replace all `config.get_param()` calls with typed config access
   - Update providers to use structured config

3. **Phase 3: Desktop Integration**
   - Add profile management UI
   - Add configuration editor UI
   - Remove environment variable dependencies for desktop

### 7. Configuration File Structure

New configuration file structure with secrets handling:

```yaml
# ~/.config/goose/config.yaml
version: "2.0"

core:
  provider: anthropic
  model: claude-3-5-sonnet-latest
  mode: auto
  context_strategy: summarize

providers:
  anthropic:
    # API key stored in keyring, referenced by convention
    # System will automatically look for:
    # 1. Keyring: service="goose", key="anthropic_api_key"
    # 2. Env var: ANTHROPIC_API_KEY
    # 3. Secrets file (if keyring disabled)
    
  openai:
    # Non-secret configs in plain text
    organization: my-org
    project: my-project
    # API key handled same as above
    
  azure:
    endpoint: https://company.openai.azure.com/
    deployment_name: gpt-4o-deployment
    # Can explicitly reference secrets if needed
    api_key: "${env:AZURE_OPENAI_API_KEY}"
    
extensions:
  - name: developer
    enabled: true
    config:
      show_hidden_files: false
      
  - name: github
    enabled: false
    # Extension secrets also follow same pattern
    
ui:
  cli:
    theme: dark
    show_cost: true
    show_thinking: false
    
  desktop:
    default_profile: work
    auto_save_sessions: true

developer:
  cache_dir: ~/.cache/goose
  log_level: info
  
scheduler:
  type: temporal
  max_concurrent_jobs: 5

# Secrets are NEVER stored in this file
# They are managed separately through:
# - System keyring (preferred)
# - Environment variables (fallback)
# - Secrets file (when keyring disabled)
```

Example secrets file (only used when keyring is disabled):
```yaml
# ~/.config/goose/secrets.yaml
# This file should have restricted permissions (600)
anthropic_api_key: sk-ant-...
openai_api_key: sk-...
github_token: ghp_...
```

### 8. Benefits

1. **Clear Structure**: Separation of concerns with dedicated sections
2. **Type Safety**: Strongly typed configuration with serde
3. **Profile Support**: Easy switching between configurations
4. **Desktop Friendly**: No reliance on environment variables for desktop users
5. **Backwards Compatible**: Migration path preserves existing functionality
6. **Centralized**: All configuration options defined in one place
7. **Validated**: Schema validation ensures configuration correctness
8. **Discoverable**: Users can see all available options
9. **Secure Secrets**: Multi-layered secrets management with keyring priority

### 9. Implementation Priority

1. **High Priority**
   - Core config schema definition
   - Config manager with layering
   - Secrets management system
   - Environment variable mapping
   - Migration of provider configs

2. **Medium Priority**
   - Profile support
   - Desktop UI integration
   - Config validation
   - Secrets migration from current system

3. **Low Priority**
   - System-wide config support
   - Advanced templating features
   - Config hot-reloading

## Next Steps

1. Review and refine this proposal
2. Create detailed implementation plan
3. Start with core schema definition
4. Build compatibility layer
5. Gradually migrate existing code
