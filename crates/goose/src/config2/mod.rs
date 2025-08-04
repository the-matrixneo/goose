//! New configuration system for Goose
//! 
//! This module provides a complete redesign of the configuration system with:
//! - Type-safe schema with all configuration options
//! - Explicit secret marking
//! - Environment variable mappings in the schema
//! - Layered configuration support

pub mod schema;
pub mod manager;
pub mod secrets;

pub use schema::{ConfigSchema, CoreConfig, ProvidersConfig, ConfigValue};
pub use manager::ConfigManager;
pub use secrets::{SecretsManager, SecretValue};
