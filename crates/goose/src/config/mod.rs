pub mod base;
mod experiments;
pub mod extensions;
pub mod permission;
pub mod signup_openrouter;

// New configuration modules
pub mod schema;
pub mod env_mapping;
pub mod manager;
pub mod secrets;

pub use crate::agents::ExtensionConfig;
pub use base::{Config, ConfigError, APP_STRATEGY};
pub use experiments::ExperimentManager;
pub use extensions::{ExtensionConfigManager, ExtensionEntry};
pub use permission::PermissionManager;
pub use signup_openrouter::configure_openrouter;

pub use extensions::DEFAULT_DISPLAY_NAME;
pub use extensions::DEFAULT_EXTENSION;
pub use extensions::DEFAULT_EXTENSION_DESCRIPTION;
pub use extensions::DEFAULT_EXTENSION_TIMEOUT;

// Export new configuration types
pub use schema::{ConfigSchema, CoreConfig, ProvidersConfig, SecretString};
pub use env_mapping::{ENV_MAPPINGS, EnvMapping};
pub use manager::ConfigManager;
pub use secrets::{SecretsManager, SecretRef};
