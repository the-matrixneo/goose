//! Unified Configuration System (Phase 1)
//! Unified Configuration System (Phase 1)
//!
//! Minimal, additive config overlay that unifies env/cli/config-file with clear precedence.
//! Public, drop-in API mirrors the kickoff brief.
//!
//! Overview
//! - Canonical keys use dot-notation (e.g., "llm.model").
//! - Env mapping: dots → underscores, uppercase, prefixed with GOOSE_.
//!   Example: "llm.model" → GOOSE_LLM_MODEL.
//! - Precedence (highest → lowest):
//!   1) CLI overlay (--set KEY=VALUE)
//!   2) Env (canonical GOOSE_* first, then aliases from the registry)
//!   3) File (config.yaml / secrets via base::Config)
//!   4) Registry default
//! - Secrets: registry marks keys as secret; reads/writes delegate to keyring or file-secrets.
//!
//! API
//! - resolve::<T>(key) → ValueWithSource<T> including provenance and redaction flag
//! - get::<T>(key), get_or::<T>(key, default)
//! - get_secret::<T>(key)
//! - set(key, value), set_secret(key, value), unset(key)
//! - effective_config(filter, only_changed, include_sources) → Vec<EffectiveEntry>
//!
//! Examples
//! ```rust
//! use goose::config::unified as config;
//! // Read values with sane defaults
//! let provider: String = config::get_or("llm.provider", "openai".to_string());
//! let port: u16 = config::get_or("server.port", 3000);
//!
//! // Resolve with provenance
//! let r = config::resolve::<String>("llm.model").unwrap();
//! match r.source {
//!     config::Source::Cli => { /* came from --set overlay */ }
//!     config::Source::Env { name, alias_used } => { /* from env, maybe alias */ }
//!     config::Source::File => { /* from config/secrets file */ }
//!     config::Source::Default => { /* from registry default */ }
//! }
//!
//! // Persist values
//! let _ = config::set("session.max_turns", serde_json::json!(200));
//! let _ = config::set_secret("providers.openai.api_key", serde_json::json!("sk-..."));
//! let _ = config::unset("session.max_turns");
//!
//! // Programmatic effective config for UI/observability
//! let entries = config::effective_config(Some("llm."), false, true);
//! ```

//! Unified Configuration System (Phase 1)
//!
//! Minimal, additive config overlay that unifies env/cli/config-file with clear precedence.
//! Public, drop-in API mirrors the kickoff brief.
//!
//! Overview
//! - Canonical keys use dot-notation (e.g., "llm.model").
//! - Env mapping: dots → underscores, uppercase, prefixed with GOOSE_.
//!   Example: "llm.model" → GOOSE_LLM_MODEL.
//! - Precedence (highest → lowest):
//!   1) CLI overlay (--set KEY=VALUE)
//!   2) Env (canonical GOOSE_* first, then aliases from the registry)
//!   3) File (config.yaml / secrets via base::Config)
//!   4) Registry default
//! - Secrets: registry marks keys as secret; reads/writes delegate to keyring or file-secrets.
//!
//! API
//! - resolve::<T>(key) → ValueWithSource<T> including provenance and redaction flag
//! - get::<T>(key), get_or::<T>(key, default)
//! - get_secret::<T>(key)
//! - set(key, value), set_secret(key, value), unset(key)
//!
//! Example
//! ```rust
//! use goose::config::unified as config;
//! let provider = config::get_or::<String>("llm.provider", "openai".into());
//! let port = config::get_or::<u16>("server.port", 3000);
//! ```

//!
//! Minimal, additive config overlay that unifies env/cli/config-file with clear precedence.
//! Public, drop-in API mirrors the kickoff brief.

pub mod registry;

#[cfg(test)]
mod tests;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use utoipa::ToSchema;

use super::base::{Config as FileConfig, ConfigError};
use registry::{canonical_to_env, find_spec};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum Source {
    Cli,
    Env { name: String, alias_used: bool },
    File,
    Default,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueWithSource<T> {
    pub key: String,
    pub value: T,
    pub source: Source,
    pub is_secret: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum UnifiedConfigError {
    #[error("unknown configuration key: {0}")]
    UnknownKey(String),
    #[error("type mismatch for key '{key}': expected {expected}")]
    TypeMismatch { key: String, expected: &'static str },
    #[error("invalid value for '{key}': {reason}")]
    InvalidValue { key: String, reason: String },
    #[error(transparent)]
    Base(#[from] ConfigError),
}

// Runtime overlay passed from CLI --set KEY=VALUE
// Using Mutex for test compatibility - allows resetting in tests
static CLI_OVERLAY: Mutex<Option<HashMap<String, Value>>> = Mutex::new(None);

pub fn set_cli_overlay(map: HashMap<String, Value>) {
    *CLI_OVERLAY.lock().unwrap() = Some(map);
}

#[cfg(test)]
pub(crate) fn reset_cli_overlay_for_tests() {
    *CLI_OVERLAY.lock().unwrap() = None;
}

fn snapshot_env() -> HashMap<String, String> {
    std::env::vars().collect()
}

fn parse_value_for_t<T: DeserializeOwned>(key: &str, v: Value) -> Result<T, UnifiedConfigError> {
    serde_json::from_value::<T>(v).map_err(|_| UnifiedConfigError::TypeMismatch {
        key: key.into(),
        expected: std::any::type_name::<T>(),
    })
}

fn resolve_raw(key: &str) -> Result<(Value, Source, bool), UnifiedConfigError> {
    let spec = find_spec(key).ok_or_else(|| UnifiedConfigError::UnknownKey(key.to_string()))?;

    // 1) CLI overlay
    if let Some(ref cli) = *CLI_OVERLAY.lock().unwrap() {
        if let Some(v) = cli.get(key) {
            return Ok((v.clone(), Source::Cli, spec.secret));
        }
    }

    // 2) Env: prefer canonical GOOSE_* form then aliases
    let env = snapshot_env();
    let canonical = canonical_to_env(key);
    if let Some(raw) = env.get(&canonical) {
        let v = FileConfig::parse_env_value(raw).map_err(UnifiedConfigError::Base)?;
        return Ok((
            v,
            Source::Env {
                name: canonical,
                alias_used: false,
            },
            spec.secret,
        ));
    }
    for &alias in spec.env_aliases {
        if let Some(raw) = env.get(alias) {
            let v = FileConfig::parse_env_value(raw).map_err(UnifiedConfigError::Base)?;
            return Ok((
                v,
                Source::Env {
                    name: alias.to_string(),
                    alias_used: true,
                },
                spec.secret,
            ));
        }
    }

    // 3) File: Config::global().load_values/load_secrets
    let file = FileConfig::global();
    if spec.secret {
        if let Ok(map) = file.load_secrets() {
            if let Some(v) = map.get(key) {
                return Ok((v.clone(), Source::File, true));
            }
        }
    } else if let Ok(map) = file.load_values() {
        if let Some(v) = map.get(key) {
            return Ok((v.clone(), Source::File, false));
        }
    }

    // 4) Default
    if let Some(def) = &spec.default {
        return Ok((def.clone(), Source::Default, spec.secret));
    }

    Err(UnifiedConfigError::Base(ConfigError::NotFound(
        key.to_string(),
    )))
}

pub fn resolve<T: DeserializeOwned>(key: &str) -> Result<ValueWithSource<T>, UnifiedConfigError> {
    let (raw, source, is_secret) = resolve_raw(key)?;
    Ok(ValueWithSource {
        key: key.to_string(),
        value: parse_value_for_t(key, raw)?,
        source,
        is_secret,
    })
}

pub fn get<T: DeserializeOwned>(key: &str) -> Result<T, UnifiedConfigError> {
    Ok(resolve::<T>(key)?.value)
}

pub fn get_or<T>(key: &str, default: T) -> T
where
    T: DeserializeOwned + Clone,
{
    match get::<T>(key) {
        Ok(v) => v,
        Err(_) => default,
    }
}

pub fn get_secret<T: DeserializeOwned>(key: &str) -> Result<T, UnifiedConfigError> {
    // delegate to resolve so precedence is consistent
    get::<T>(key)
}

pub fn set(key: &str, value: Value) -> Result<(), UnifiedConfigError> {
    let spec = find_spec(key).ok_or_else(|| UnifiedConfigError::UnknownKey(key.to_string()))?;
    if let Some(validator) = spec.validator {
        if let Err(reason) = validator(&value) {
            return Err(UnifiedConfigError::InvalidValue {
                key: key.to_string(),
                reason,
            });
        }
    }
    if spec.secret {
        return set_secret(key, value);
    }
    let file = FileConfig::global();
    file.set_param(key, value).map_err(UnifiedConfigError::Base)
}

pub fn set_secret(key: &str, value: Value) -> Result<(), UnifiedConfigError> {
    let spec = find_spec(key).ok_or_else(|| UnifiedConfigError::UnknownKey(key.to_string()))?;
    if let Some(validator) = spec.validator {
        if let Err(reason) = validator(&value) {
            return Err(UnifiedConfigError::InvalidValue {
                key: key.to_string(),
                reason,
            });
        }
    }
    let file = FileConfig::global();
    file.set_secret(key, value)
        .map_err(UnifiedConfigError::Base)
}

pub fn unset(key: &str) -> Result<(), UnifiedConfigError> {
    let spec = find_spec(key).ok_or_else(|| UnifiedConfigError::UnknownKey(key.to_string()))?;
    let file = FileConfig::global();
    if spec.secret {
        file.delete_secret(key).map_err(UnifiedConfigError::Base)
    } else {
        file.delete(key).map_err(UnifiedConfigError::Base)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EffectiveEntry {
    pub key: String,
    pub value: serde_json::Value, // redacted if secret
    pub redacted: bool,
    pub is_secret: bool,
    pub source: Source,
    pub env_name: Option<String>,
    pub alias_used: Option<bool>,
    pub has_default: bool,
}

pub fn effective_config(
    filter: Option<&str>,
    only_changed: bool,
    include_sources: bool,
) -> Vec<EffectiveEntry> {
    use crate::config::unified::registry::REGISTRY;

    let mut out: Vec<EffectiveEntry> = Vec::new();
    for spec in REGISTRY.iter() {
        if let Some(prefix) = filter {
            if !spec.key.starts_with(prefix) {
                continue;
            }
        }
        match resolve::<serde_json::Value>(spec.key) {
            Ok(ValueWithSource {
                value,
                source,
                is_secret,
                ..
            }) => {
                if only_changed && matches!(source, Source::Default) {
                    continue;
                }
                let (val, redacted) = if is_secret {
                    (serde_json::json!("***"), true)
                } else {
                    (value, false)
                };
                let (env_name, alias_used) = match &source {
                    Source::Env { name, alias_used } if include_sources => {
                        (Some(name.clone()), Some(*alias_used))
                    }
                    _ => (None, None),
                };
                out.push(EffectiveEntry {
                    key: spec.key.to_string(),
                    value: val,
                    redacted,
                    is_secret,
                    source,
                    env_name,
                    alias_used,
                    has_default: spec.default.is_some(),
                });
            }
            Err(UnifiedConfigError::Base(ConfigError::NotFound(_))) => {
                if !only_changed {
                    out.push(EffectiveEntry {
                        key: spec.key.to_string(),
                        value: serde_json::Value::Null,
                        redacted: false,
                        is_secret: spec.secret,
                        source: Source::Default, // treat as none/default
                        env_name: None,
                        alias_used: None,
                        has_default: spec.default.is_some(),
                    });
                }
            }
            Err(_e) => {
                // Skip invalid/unknown; future: expose errors channel
            }
        }
    }
    out
}

#[cfg(test)]
mod unified_unit {
    use super::*;
    use crate::config::unified::registry::REGISTRY;
    use serial_test::serial;

    fn unset_env(keys: &[&str]) {
        for k in keys {
            std::env::remove_var(k);
        }
    }

    #[test]
    #[serial]
    fn registry_contains_expected_keys_and_flags() {
        let keys: Vec<&str> = REGISTRY.iter().map(|s| s.key).collect();
        assert!(keys.contains(&"llm.provider"));
        assert!(keys.contains(&"llm.model"));
        assert!(keys.contains(&"server.port"));
        assert!(keys.contains(&"session.max_turns"));
        assert!(keys.contains(&"session.max_tool_repetitions"));
        assert!(keys.contains(&"tracing.langfuse.url"));
        assert!(keys.contains(&"providers.openai.api_key"));

        let openai = REGISTRY
            .iter()
            .find(|s| s.key == "providers.openai.api_key")
            .unwrap();
        assert!(openai.secret, "openai api key must be secret");

        let model = REGISTRY.iter().find(|s| s.key == "llm.model").unwrap();
        assert!(model.default.is_some());
    }

    #[test]
    #[serial]
    fn env_precedence_canonical_over_alias() {
        // Set both canonical GOOSE_* and alias env; canonical should win
        std::env::set_var("GOOSE_LLM_MODEL", "canonical-model");
        std::env::set_var("GOOSE_MODEL", "alias-model");

        let r = resolve::<String>("llm.model").expect("should resolve");
        assert_eq!(r.value, "canonical-model");
        match r.source {
            Source::Env { name, alias_used } => {
                assert_eq!(name, "GOOSE_LLM_MODEL");
                assert!(!alias_used);
            }
            other => panic!("unexpected source: {:?}", other),
        }

        unset_env(&["GOOSE_LLM_MODEL", "GOOSE_MODEL"]);
    }

    #[test]
    #[serial]
    fn env_alias_used_when_canonical_missing() {
        std::env::remove_var("GOOSE_LLM_MODEL");
        std::env::set_var("GOOSE_MODEL", "alias-model");

        let r = resolve::<String>("llm.model").expect("should resolve");
        assert_eq!(r.value, "alias-model");
        match r.source {
            Source::Env { name, alias_used } => {
                assert_eq!(name, "GOOSE_MODEL");
                assert!(alias_used);
            }
            other => panic!("unexpected source: {:?}", other),
        }

        unset_env(&["GOOSE_MODEL"]);
    }

    #[test]
    #[serial]
    fn env_parsing_numbers_and_bools() {
        std::env::set_var("GOOSE_SESSION_MAX_TURNS", "42");
        let max_turns = get::<u32>("session.max_turns").unwrap();
        assert_eq!(max_turns, 42);
        std::env::remove_var("GOOSE_SESSION_MAX_TURNS");

        // Type mismatch path
        std::env::set_var("GOOSE_LLM_MODEL", "true");
        let err = get::<u16>("llm.model").unwrap_err();
        match err {
            UnifiedConfigError::TypeMismatch { key, expected } => {
                assert_eq!(key, "llm.model");
                assert!(expected.contains("u16"));
            }
            other => panic!("unexpected error: {:?}", other),
        }
        std::env::remove_var("GOOSE_LLM_MODEL");
    }

    #[test]
    fn unknown_key_errors() {
        let err = get::<String>("does.not.exist").unwrap_err();
        match err {
            UnifiedConfigError::UnknownKey(k) => assert_eq!(k, "does.not.exist"),
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    #[serial]
    fn secret_env_alias_resolves_with_metadata() {
        // Use legacy alias for secret
        std::env::set_var("OPENAI_API_KEY", "sk-test");
        let r = resolve::<String>("providers.openai.api_key").expect("resolve");
        assert_eq!(r.value, "sk-test");
        assert!(r.is_secret);
        match r.source {
            Source::Env { name, alias_used } => {
                assert_eq!(name, "OPENAI_API_KEY");
                assert!(alias_used);
            }
            other => panic!("unexpected source: {:?}", other),
        }
        std::env::remove_var("OPENAI_API_KEY");
    }
}
