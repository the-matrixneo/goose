//! Unified configuration management for Goose
//!
//! This module provides a centralized configuration system that unifies
//! environment variables, configuration files, and CLI arguments.

pub mod base;
pub mod builder;
pub mod compat;
pub mod experiments;
pub mod extensions;
pub mod permission;
pub mod schema;
pub mod signup_openrouter;
pub mod sources;

// #[cfg(test)]
// mod tests;

// Re-export core configuration types
pub use base::{Config, ConfigError, APP_STRATEGY};
pub use experiments::ExperimentManager;
pub use extensions::{
    name_to_key, ExtensionConfigManager, ExtensionEntry, DEFAULT_EXTENSION,
    DEFAULT_EXTENSION_DESCRIPTION,
};
pub use permission::{PermissionLevel, PermissionManager};

// Re-export builder and schema types
pub use builder::ConfigBuilder;
pub use compat::{get, get_or, get_secret, set, var};
pub use schema::{GooseConfig, ModelConfig, ProviderConfig};
pub use sources::{CliSource, ConfigSource, EnvSource, FileSource};

// Re-export the compatibility layer functions for easier migration
pub use compat::{
    clear, delete, exists, get_all, get_bool, get_float, get_int, get_string, has, load_values,
    path, remove, set_secret,
};

// Re-export signup_openrouter functionality
pub use signup_openrouter::configure_openrouter;

// Constants
pub const DEFAULT_DISPLAY_NAME: &str = "Developer Tools";
pub const DEFAULT_EXTENSION_TIMEOUT: u64 = 180;
