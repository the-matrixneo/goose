// goose/src/config/compat.rs
// Compatibility layer for seamless migration from existing config patterns

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Drop-in replacement functions
// ============================================================================

/// Drop-in replacement for std::env::var()
///
/// This function provides a compatible interface with std::env::var() but adds:
/// - Integration with the config system
/// - Fallback to config parameters if env var not found
pub fn var(key: &str) -> Result<String> {
    // Try environment variable first
    if let Ok(value) = std::env::var(key) {
        return Ok(value);
    }

    // Fallback to config parameter
    use crate::config::Config;
    if let Ok(value) = Config::global().get_param::<String>(key) {
        return Ok(value);
    }

    bail!(
        "Environment variable or config parameter '{}' not found",
        key
    )
}

/// Drop-in replacement for Config::global().get_param()
///
/// This function provides seamless access to configuration parameters
/// with automatic type conversion support.
pub fn get<T>(key: &str) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    use crate::config::Config;
    Config::global().get_param::<T>(key).ok()
}

/// Drop-in replacement for Config::global().set_param()
///
/// This function provides seamless setting of configuration parameters
/// with automatic serialization support.
pub fn set<T>(key: &str, value: T) -> Result<()>
where
    T: Serialize,
{
    use crate::config::Config;
    let json_value = serde_json::to_value(value)
        .with_context(|| format!("Failed to serialize value for key: {}", key))?;

    Config::global()
        .set_param(key, json_value)
        .map_err(|e| anyhow::anyhow!("Failed to set param '{}': {}", key, e))
}

/// Get a secret from the keyring
///
/// This function provides secure storage integration for sensitive data
/// like API keys, passwords, and tokens.
pub fn get_secret(key: &str) -> Result<String> {
    use crate::config::Config;
    Config::global()
        .get_secret::<String>(key)
        .map_err(|e| anyhow::anyhow!("Failed to get secret '{}': {}", key, e))
}

/// Set a secret in the keyring
pub fn set_secret(key: &str, value: &str) -> Result<()> {
    use crate::config::Config;
    Config::global()
        .set_secret(key, Value::String(value.to_string()))
        .map_err(|e| anyhow::anyhow!("Failed to set secret '{}': {}", key, e))
}

/// Delete a secret from the keyring
pub fn delete_secret(key: &str) -> Result<()> {
    use crate::config::Config;
    Config::global()
        .delete_secret(key)
        .map_err(|e| anyhow::anyhow!("Failed to delete secret '{}': {}", key, e))
}

// ============================================================================
// Convenience functions
// ============================================================================

/// Get a parameter with default value
///
/// This is the primary convenience function for getting config values with defaults.
/// It supports automatic type conversion and provides a clean API for migration.
pub fn get_or<T>(key: &str, default: T) -> T
where
    T: for<'de> Deserialize<'de>,
{
    get::<T>(key).unwrap_or(default)
}

/// Get a string parameter with default value
pub fn get_string(key: &str, default: &str) -> String {
    get::<String>(key).unwrap_or_else(|| default.to_string())
}

/// Get a boolean parameter with default value
pub fn get_bool(key: &str, default: bool) -> bool {
    get::<bool>(key).unwrap_or(default)
}

/// Get an integer parameter with default value
pub fn get_int(key: &str, default: i64) -> i64 {
    get::<i64>(key).unwrap_or(default)
}

/// Get a float parameter with default value
pub fn get_float(key: &str, default: f64) -> f64 {
    get::<f64>(key).unwrap_or(default)
}

/// Check if a parameter exists
pub fn has(key: &str) -> bool {
    use crate::config::Config;
    Config::global().get_param::<Value>(key).is_ok()
}

/// Remove a parameter
pub fn remove(key: &str) -> Result<()> {
    use crate::config::Config;
    Config::global()
        .delete(key)
        .map_err(|e| anyhow::anyhow!("Failed to remove key '{}': {}", key, e))
}

/// Get all parameters
pub fn get_all() -> HashMap<String, Value> {
    use crate::config::Config;
    Config::global().load_values().unwrap_or_default()
}

/// Check if the configuration file exists
pub fn exists() -> bool {
    use crate::config::Config;
    Config::global().exists()
}

/// Clear all configuration (both params and secrets)
pub fn clear() -> Result<()> {
    use crate::config::Config;
    Config::global()
        .clear()
        .map_err(|e| anyhow::anyhow!("Failed to clear config: {}", e))
}

/// Reset global config state for testing
#[cfg(test)]
pub fn reset_for_test() {
    // Clear environment variables that might affect config
    let goose_env_vars = [
        "GOOSE_PROVIDER",
        "GOOSE_MODEL",
        "GOOSE_TEMPERATURE",
        "GOOSE_CONTEXT_LIMIT",
        "GOOSE_TOOLSHIM",
        "GOOSE_TOOLSHIM_OLLAMA_MODEL",
        "GOOSE_MODE",
    ];

    for var in &goose_env_vars {
        std::env::remove_var(var);
    }

    // Clear the config file
    let _ = clear();
}

/// Get the configuration file path
pub fn path() -> String {
    use crate::config::Config;
    Config::global().path()
}

/// Delete a configuration parameter
pub fn delete(key: &str) -> Result<()> {
    use crate::config::Config;
    Config::global()
        .delete(key)
        .map_err(|e| anyhow::anyhow!("Failed to delete key '{}': {}", key, e))
}

/// Load all configuration values from file
pub fn load_values() -> Result<HashMap<String, Value>> {
    use crate::config::Config;
    Config::global()
        .load_values()
        .map_err(|e| anyhow::anyhow!("Failed to load values: {}", e))
}

/// Load all secrets from secure storage
pub fn load_secrets() -> Result<HashMap<String, Value>> {
    use crate::config::Config;
    Config::global()
        .load_secrets()
        .map_err(|e| anyhow::anyhow!("Failed to load secrets: {}", e))
}

// ============================================================================
// Macro support for even more convenient usage
// ============================================================================

/// Macro for getting config values with automatic type inference
#[macro_export]
macro_rules! config_get {
    ($key:expr) => {
        $crate::config::compat::get($key)
    };
    ($key:expr, $default:expr) => {
        $crate::config::compat::get($key).unwrap_or($default)
    };
}

/// Macro for setting config values
#[macro_export]
macro_rules! config_set {
    ($key:expr, $value:expr) => {
        $crate::config::compat::set($key, $value)
    };
}

/// Macro for getting environment variables with fallback
#[macro_export]
macro_rules! env_or_config {
    ($key:expr) => {
        $crate::config::compat::var($key)
    };
    ($key:expr, $default:expr) => {
        $crate::config::compat::var($key).unwrap_or_else(|_| $default.to_string())
    };
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env;

    #[test]
    fn test_var_function() {
        // Set an environment variable
        std::env::set_var("TEST_VAR", "test_value");

        // Should get the env var
        assert_eq!(var("TEST_VAR").unwrap(), "test_value");

        // Clean up
        std::env::remove_var("TEST_VAR");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_set_functions() {
        // Reset global state for clean test environment
        reset_for_test();

        temp_env::with_vars(
            [
                ("GOOSE_PROVIDER", None::<&str>),
                ("GOOSE_MODEL", None::<&str>),
                ("GOOSE_TEMPERATURE", None::<&str>),
                ("GOOSE_CONTEXT_LIMIT", None::<&str>),
                ("GOOSE_TOOLSHIM", None::<&str>),
                ("GOOSE_TOOLSHIM_OLLAMA_MODEL", None::<&str>),
                ("GOOSE_MODE", None::<&str>),
            ],
            || {
                // Test with environment variable override to verify precedence
                temp_env::with_var("TEST_KEY", Some("env_value"), || {
                    // Set a string value in config
                    set("test_key", "test_value").unwrap();
                    // Environment variable should take precedence
                    assert_eq!(get::<String>("test_key"), Some("env_value".to_string()));
                });

                // Test without environment variable
                temp_env::with_var("TEST_KEY", None::<&str>, || {
                    // Set a string value
                    set("test_key", "test_value").unwrap();
                    assert_eq!(get::<String>("test_key"), Some("test_value".to_string()));

                    // Set a boolean value
                    set("bool_key", true).unwrap();
                    assert_eq!(get::<bool>("bool_key"), Some(true));

                    // Set a number value
                    set("num_key", 42).unwrap();
                    assert_eq!(get::<i32>("num_key"), Some(42));
                });
            },
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_convenience_functions() {
        // Reset global state for clean test environment
        reset_for_test();

        temp_env::with_vars(
            [
                ("GOOSE_PROVIDER", None::<&str>),
                ("GOOSE_MODEL", None::<&str>),
                ("GOOSE_TEMPERATURE", None::<&str>),
                ("GOOSE_CONTEXT_LIMIT", None::<&str>),
                ("GOOSE_TOOLSHIM", None::<&str>),
                ("GOOSE_TOOLSHIM_OLLAMA_MODEL", None::<&str>),
                ("GOOSE_MODE", None::<&str>),
            ],
            || {
                // Test with defaults
                assert_eq!(get_string("missing_key_str", "default"), "default");
                assert_eq!(get_bool("missing_key_bool", true), true);
                assert_eq!(get_int("missing_key_int", 100), 100);
                assert_eq!(get_float("missing_key_float", 3.14), 3.14);

                // Test has function
                set("exists", "yes").unwrap();
                assert!(has("exists"));
                assert!(!has("not_exists"));
            },
        );
    }
}
