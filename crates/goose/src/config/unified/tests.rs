//! Unit tests for unified configuration system
//! Tests validators, precedence, and resolution logic

use super::*;
use crate::config::unified::registry::find_spec;
use serde_json::json;
use serial_test::serial;
use std::collections::HashMap;

// Helper to clean up environment variables
fn cleanup_env(keys: &[&str]) {
    for key in keys {
        std::env::remove_var(key);
    }
}

// Helper to set up a test with isolated environment
fn with_clean_env<F, R>(env_keys: &[&str], f: F) -> R
where
    F: FnOnce() -> R,
{
    cleanup_env(env_keys);
    let result = f();
    cleanup_env(env_keys);
    result
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_temperature_validator() {
        let spec = find_spec("model.temperature").expect("temperature key should exist");
        let validator = spec.validator.expect("temperature should have validator");

        // Valid cases
        assert!(validator(&json!(0.0)).is_ok(), "0.0 should be valid");
        assert!(validator(&json!(1.0)).is_ok(), "1.0 should be valid");
        assert!(validator(&json!(2.0)).is_ok(), "2.0 should be valid");
        assert!(validator(&json!(0.5)).is_ok(), "0.5 should be valid");
        assert!(validator(&json!(1.5)).is_ok(), "1.5 should be valid");

        // Invalid cases
        assert!(
            validator(&json!(-0.1)).is_err(),
            "negative should be invalid"
        );
        assert!(
            validator(&json!(2.1)).is_err(),
            "over 2.0 should be invalid"
        );
        assert!(validator(&json!(3.0)).is_err(), "3.0 should be invalid");
        assert!(
            validator(&json!("1.0")).is_err(),
            "string should be invalid"
        );
        assert!(validator(&json!(null)).is_err(), "null should be invalid");
    }

    #[test]
    fn test_toolshim_enabled_bool_type() {
        let spec = find_spec("toolshim.enabled").expect("toolshim.enabled key should exist");

        // Verify it's a bool type
        assert!(matches!(
            spec.ty,
            crate::config::unified::registry::ValueType::Bool
        ));

        // No validator needed for bool - type system handles it
        assert!(spec.validator.is_none());
    }

    #[test]
    fn test_otlp_protocol_enum_validator() {
        let spec = find_spec("tracing.otlp.protocol").expect("otlp.protocol key should exist");
        let validator = spec.validator.expect("otlp.protocol should have validator");

        // Valid cases (case-insensitive)
        assert!(validator(&json!("grpc")).is_ok(), "grpc should be valid");
        assert!(validator(&json!("http")).is_ok(), "http should be valid");
        assert!(validator(&json!("GRPC")).is_ok(), "GRPC should be valid");
        assert!(validator(&json!("HTTP")).is_ok(), "HTTP should be valid");
        assert!(validator(&json!("Http")).is_ok(), "Http should be valid");

        // Invalid cases
        assert!(
            validator(&json!("https")).is_err(),
            "https should be invalid"
        );
        assert!(validator(&json!("tcp")).is_err(), "tcp should be invalid");
        assert!(
            validator(&json!("")).is_err(),
            "empty string should be invalid"
        );
        assert!(validator(&json!(123)).is_err(), "number should be invalid");
        assert!(validator(&json!(null)).is_err(), "null should be invalid");

        // Check error message
        let err = validator(&json!("invalid")).unwrap_err();
        assert!(err.contains("must be one of: grpc, http"));
    }

    #[test]
    fn test_scheduler_type_enum_validator() {
        let spec = find_spec("scheduler.type").expect("scheduler.type key should exist");
        let validator = spec
            .validator
            .expect("scheduler.type should have validator");

        // Valid cases
        assert!(
            validator(&json!("legacy")).is_ok(),
            "legacy should be valid"
        );
        assert!(
            validator(&json!("temporal")).is_ok(),
            "temporal should be valid"
        );
        assert!(
            validator(&json!("LEGACY")).is_ok(),
            "LEGACY should be valid"
        );
        assert!(
            validator(&json!("Temporal")).is_ok(),
            "Temporal should be valid"
        );

        // Invalid cases
        assert!(
            validator(&json!("invalid")).is_err(),
            "invalid should be invalid"
        );
        assert!(
            validator(&json!("")).is_err(),
            "empty string should be invalid"
        );
        assert!(validator(&json!(true)).is_err(), "bool should be invalid");

        // Check error message
        let err = validator(&json!("other")).unwrap_err();
        assert!(err.contains("must be one of: legacy, temporal"));
    }

    #[test]
    fn test_url_validator() {
        let spec = find_spec("security.allowlist.url").expect("allowlist.url key should exist");
        let validator = spec.validator.expect("allowlist.url should have validator");

        // Valid cases
        assert!(validator(&json!("http://example.com")).is_ok());
        assert!(validator(&json!("https://example.com")).is_ok());
        assert!(validator(&json!("http://localhost:8080")).is_ok());
        assert!(validator(&json!("https://api.example.com/path")).is_ok());

        // Invalid cases
        assert!(validator(&json!("not-a-url")).is_err());
        assert!(
            validator(&json!("ftp://example.com")).is_err(),
            "non-http(s) scheme"
        );
        assert!(validator(&json!("")).is_err(), "empty string");
        assert!(validator(&json!("   ")).is_err(), "whitespace only");
        assert!(validator(&json!(123)).is_err(), "number");
    }

    #[test]
    fn test_port_validator_edge_cases() {
        let spec = find_spec("server.port").expect("server.port key should exist");
        let validator = spec.validator.expect("server.port should have validator");

        // Edge cases
        assert!(validator(&json!(1)).is_ok(), "port 1 should be valid");
        assert!(
            validator(&json!(65535)).is_ok(),
            "port 65535 should be valid"
        );
        assert!(validator(&json!(0)).is_err(), "port 0 should be invalid");
        assert!(
            validator(&json!(65536)).is_err(),
            "port 65536 should be invalid"
        );
        assert!(
            validator(&json!(-1)).is_err(),
            "negative port should be invalid"
        );
        assert!(validator(&json!(1.5)).is_err(), "float should be invalid");
    }
}

#[cfg(test)]
mod precedence_tests {
    use super::*;
    use serde_json::json;

    #[test]
    #[serial]
    fn test_cli_theme_precedence_cli_wins() {
        with_clean_env(&["GOOSE_CLI_THEME"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            // Set up precedence layers
            std::env::set_var("GOOSE_CLI_THEME", "env-theme");

            // Simulate CLI overlay
            let mut cli_overlay = HashMap::new();
            cli_overlay.insert("cli.theme".to_string(), json!("cli-theme"));
            set_cli_overlay(cli_overlay);

            // CLI should win
            let result = resolve::<String>("cli.theme").expect("should resolve");
            assert_eq!(result.value, "cli-theme");
            assert!(matches!(result.source, Source::Cli));

            // Clean up CLI overlay for other tests
            reset_cli_overlay_for_tests();
        });
    }

    #[test]
    #[serial]
    fn test_cli_theme_precedence_env_over_file() {
        with_clean_env(&["GOOSE_CLI_THEME"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            // Set env variable
            std::env::set_var("GOOSE_CLI_THEME", "env-theme");

            // No CLI overlay, env should win over file/default
            let result = resolve::<String>("cli.theme").expect("should resolve");
            assert_eq!(result.value, "env-theme");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_CLI_THEME");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }

    #[test]
    #[serial]
    fn test_cli_theme_precedence_default_fallback() {
        with_clean_env(&["GOOSE_CLI_THEME"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            // No CLI, no env, no file - should use default
            let result = resolve::<String>("cli.theme").expect("should resolve");
            assert_eq!(result.value, "dark"); // default value from registry
            assert!(matches!(result.source, Source::Default));
        });
    }

    #[test]
    #[serial]
    fn test_scheduler_type_precedence_cli_over_env() {
        with_clean_env(&["GOOSE_SCHEDULER_TYPE"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            // Set env
            std::env::set_var("GOOSE_SCHEDULER_TYPE", "temporal");

            // Set CLI overlay
            let mut cli_overlay = HashMap::new();
            cli_overlay.insert("scheduler.type".to_string(), json!("legacy"));
            set_cli_overlay(cli_overlay);

            // CLI should win
            let result = resolve::<String>("scheduler.type").expect("should resolve");
            assert_eq!(result.value, "legacy");
            assert!(matches!(result.source, Source::Cli));

            // Clean up
            reset_cli_overlay_for_tests();
        });
    }

    #[test]
    #[serial]
    fn test_scheduler_type_precedence_env_wins() {
        with_clean_env(&["GOOSE_SCHEDULER_TYPE"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            std::env::set_var("GOOSE_SCHEDULER_TYPE", "temporal");

            let result = resolve::<String>("scheduler.type").expect("should resolve");
            assert_eq!(result.value, "temporal");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_SCHEDULER_TYPE");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }

    #[test]
    #[serial]
    fn test_scheduler_type_precedence_default() {
        with_clean_env(&["GOOSE_SCHEDULER_TYPE"], || {
            // Reset CLI overlay from any previous tests
            reset_cli_overlay_for_tests();

            let result = resolve::<String>("scheduler.type").expect("should resolve");
            assert_eq!(result.value, "legacy"); // default from registry
            assert!(matches!(result.source, Source::Default));
        });
    }

    #[test]
    #[serial]
    fn test_complex_precedence_chain() {
        with_clean_env(&["GOOSE_LLM_MODEL", "GOOSE_MODEL", "MODEL"], || {
            // Test canonical env wins over aliases
            std::env::set_var("GOOSE_LLM_MODEL", "canonical");
            std::env::set_var("GOOSE_MODEL", "alias1");
            std::env::set_var("MODEL", "alias2");

            let result = resolve::<String>("llm.model").expect("should resolve");
            assert_eq!(result.value, "canonical");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_LLM_MODEL");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }
}

#[cfg(test)]
mod alias_tests {
    use super::*;

    #[test]
    #[serial]
    fn test_canonical_and_alias_env_names() {
        with_clean_env(&["GOOSE_LLM_MODEL", "GOOSE_MODEL", "MODEL"], || {
            // Test canonical name
            std::env::set_var("GOOSE_LLM_MODEL", "from-canonical");
            let result = resolve::<String>("llm.model").expect("should resolve");
            assert_eq!(result.value, "from-canonical");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_LLM_MODEL");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
            std::env::remove_var("GOOSE_LLM_MODEL");

            // Test first alias
            std::env::set_var("GOOSE_MODEL", "from-alias1");
            let result = resolve::<String>("llm.model").expect("should resolve");
            assert_eq!(result.value, "from-alias1");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_MODEL");
                    assert!(alias_used);
                }
                _ => panic!("Expected env source"),
            }
            std::env::remove_var("GOOSE_MODEL");

            // Test second alias
            std::env::set_var("MODEL", "from-alias2");
            let result = resolve::<String>("llm.model").expect("should resolve");
            assert_eq!(result.value, "from-alias2");
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "MODEL");
                    assert!(alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }

    #[test]
    #[serial]
    fn test_temperature_env_aliases() {
        with_clean_env(&["GOOSE_MODEL_TEMPERATURE", "GOOSE_TEMPERATURE"], || {
            // Test canonical
            std::env::set_var("GOOSE_MODEL_TEMPERATURE", "1.5");
            let result = resolve::<f64>("model.temperature").expect("should resolve");
            assert_eq!(result.value, 1.5);
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_MODEL_TEMPERATURE");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
            std::env::remove_var("GOOSE_MODEL_TEMPERATURE");

            // Test alias
            std::env::set_var("GOOSE_TEMPERATURE", "0.7");
            let result = resolve::<f64>("model.temperature").expect("should resolve");
            assert_eq!(result.value, 0.7);
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_TEMPERATURE");
                    assert!(alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }

    #[test]
    #[serial]
    fn test_toolshim_env_aliases() {
        with_clean_env(&["GOOSE_TOOLSHIM_ENABLED", "GOOSE_TOOLSHIM"], || {
            // Test canonical
            std::env::set_var("GOOSE_TOOLSHIM_ENABLED", "true");
            let result = resolve::<bool>("toolshim.enabled").expect("should resolve");
            assert_eq!(result.value, true);
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_TOOLSHIM_ENABLED");
                    assert!(!alias_used);
                }
                _ => panic!("Expected env source"),
            }
            std::env::remove_var("GOOSE_TOOLSHIM_ENABLED");

            // Test alias
            std::env::set_var("GOOSE_TOOLSHIM", "false");
            let result = resolve::<bool>("toolshim.enabled").expect("should resolve");
            assert_eq!(result.value, false);
            match result.source {
                Source::Env { name, alias_used } => {
                    assert_eq!(name, "GOOSE_TOOLSHIM");
                    assert!(alias_used);
                }
                _ => panic!("Expected env source"),
            }
        });
    }

    #[test]
    #[serial]
    fn test_secret_key_env_aliases() {
        with_clean_env(
            &["OPENAI_API_KEY", "GOOSE_PROVIDERS_OPENAI_API_KEY"],
            || {
                // Test alias (common pattern)
                std::env::set_var("OPENAI_API_KEY", "sk-test-123");
                let result = resolve::<String>("providers.openai.api_key").expect("should resolve");
                assert_eq!(result.value, "sk-test-123");
                assert!(result.is_secret);
                match result.source {
                    Source::Env { name, alias_used } => {
                        assert_eq!(name, "OPENAI_API_KEY");
                        assert!(alias_used);
                    }
                    _ => panic!("Expected env source"),
                }
            },
        );
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_invalid_value_with_validator() {
        let result = set("model.temperature", json!(3.0));
        assert!(result.is_err());
        match result.unwrap_err() {
            UnifiedConfigError::InvalidValue { key, reason } => {
                assert_eq!(key, "model.temperature");
                assert!(reason.contains("must be between 0.0 and 2.0"));
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_type_mismatch_error() {
        with_clean_env(&["GOOSE_SERVER_PORT"], || {
            std::env::set_var("GOOSE_SERVER_PORT", "not-a-number");
            let result = get::<u16>("server.port");
            assert!(result.is_err());
            match result.unwrap_err() {
                UnifiedConfigError::TypeMismatch { key, expected } => {
                    assert_eq!(key, "server.port");
                    assert!(expected.contains("u16"));
                }
                _ => panic!("Expected TypeMismatch error"),
            }
        });
    }

    #[test]
    fn test_unknown_key_error() {
        let result = get::<String>("this.key.does.not.exist");
        assert!(result.is_err());
        match result.unwrap_err() {
            UnifiedConfigError::UnknownKey(key) => {
                assert_eq!(key, "this.key.does.not.exist");
            }
            _ => panic!("Expected UnknownKey error"),
        }
    }

    #[test]
    fn test_set_with_invalid_enum_value() {
        let result = set("scheduler.type", json!("invalid-type"));
        assert!(result.is_err());
        match result.unwrap_err() {
            UnifiedConfigError::InvalidValue { key, reason } => {
                assert_eq!(key, "scheduler.type");
                assert!(reason.contains("must be one of: legacy, temporal"));
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }
}

#[cfg(test)]
mod effective_config_tests {
    use super::*;

    #[test]
    #[serial]
    fn test_effective_config_with_filter() {
        with_clean_env(&["GOOSE_CLI_THEME"], || {
            std::env::set_var("GOOSE_CLI_THEME", "light");

            let entries = effective_config(Some("cli."), false, true);

            // Should only include cli.* keys
            assert!(entries.iter().all(|e| e.key.starts_with("cli.")));

            // Should include cli.theme with env source
            let theme_entry = entries
                .iter()
                .find(|e| e.key == "cli.theme")
                .expect("cli.theme should be present");
            assert_eq!(theme_entry.value, json!("light"));
            assert!(matches!(theme_entry.source, Source::Env { .. }));
            assert_eq!(theme_entry.env_name, Some("GOOSE_CLI_THEME".to_string()));
            assert_eq!(theme_entry.alias_used, Some(false));
        });
    }

    #[test]
    #[serial]
    fn test_effective_config_only_changed() {
        with_clean_env(&["GOOSE_LLM_MODEL"], || {
            std::env::set_var("GOOSE_LLM_MODEL", "custom-model");

            let entries = effective_config(Some("llm."), true, false);

            // Should include llm.model (changed)
            assert!(entries.iter().any(|e| e.key == "llm.model"));

            // Should NOT include llm.provider (default)
            assert!(!entries.iter().any(|e| e.key == "llm.provider"));
        });
    }

    #[test]
    #[serial]
    fn test_effective_config_secrets_redacted() {
        with_clean_env(&["OPENAI_API_KEY"], || {
            std::env::set_var("OPENAI_API_KEY", "sk-secret-key");

            let entries = effective_config(Some("providers."), false, true);

            let api_key_entry = entries
                .iter()
                .find(|e| e.key == "providers.openai.api_key")
                .expect("api key should be present");

            assert_eq!(api_key_entry.value, json!("***"));
            assert!(api_key_entry.redacted);
            assert!(api_key_entry.is_secret);
        });
    }
}

#[cfg(test)]
mod registry_coverage_tests {
    use super::*;
    use crate::config::unified::registry::REGISTRY;

    #[test]
    fn test_all_phase3_keys_present() {
        // Verify all new Phase 3 keys are in registry
        let keys_to_check = vec![
            "model.temperature",
            "toolshim.enabled",
            "toolshim.model",
            "tracing.otlp.headers",
            "tracing.otlp.protocol",
            "cli.theme",
            "scheduler.type",
            "security.allowlist.url",
            "security.allowlist.bypass",
        ];

        for key in keys_to_check {
            assert!(
                find_spec(key).is_some(),
                "Key '{}' should be in registry",
                key
            );
        }
    }

    #[test]
    fn test_validator_assignments() {
        // Verify validators are assigned correctly
        let validators_expected = vec![
            ("model.temperature", true),
            ("tracing.otlp.protocol", true),
            ("scheduler.type", true),
            ("security.allowlist.url", true),
            ("toolshim.enabled", false), // bool doesn't need validator
            ("cli.theme", false),        // string without constraints
        ];

        for (key, should_have_validator) in validators_expected {
            let spec = find_spec(key).expect(&format!("Key {} should exist", key));
            assert_eq!(
                spec.validator.is_some(),
                should_have_validator,
                "Key '{}' validator presence mismatch",
                key
            );
        }
    }
}
