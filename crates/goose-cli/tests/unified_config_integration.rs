//! Integration tests for unified configuration in CLI context
//! Tests security allowlist, CLI theme, and precedence behavior

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

fn goose_bin() -> Command {
    Command::cargo_bin("goose").expect("binary built")
}

fn with_isolated_home(mut cmd: Command, tmp: &TempDir) -> Command {
    let home = tmp.path().to_str().unwrap();
    cmd.env("HOME", home)
        .env("USERPROFILE", home)
        .env("XDG_CONFIG_HOME", format!("{home}/.config"))
        .env("GOOSE_DISABLE_KEYRING", "1");
    cmd
}

#[test]
fn test_security_allowlist_url_resolution_from_env() {
    let tmp = TempDir::new().unwrap();

    // Test canonical env variable
    let cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .env(
            "GOOSE_SECURITY_ALLOWLIST_URL",
            "https://example.com/allowlist",
        )
        .args(["configure", "--get", "security.allowlist.url", "--raw"])
        .assert();

    assert.success().stdout("https://example.com/allowlist\n");

    // Test alias env variable
    let cmd2 = goose_bin();
    let assert2 = with_isolated_home(cmd2, &tmp)
        .env("GOOSE_ALLOWLIST", "https://alias.example.com/list")
        .args(["configure", "--get", "security.allowlist.url", "--raw"])
        .assert();

    assert2.success().stdout("https://alias.example.com/list\n");
}

#[test]
fn test_security_allowlist_bypass_resolution() {
    let tmp = TempDir::new().unwrap();

    // Test setting via configure
    let set_cmd = goose_bin();
    let assert = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "security.allowlist.bypass=true"])
        .assert();

    assert.success();

    // Verify it was set
    let get_cmd = goose_bin();
    let assert2 = with_isolated_home(get_cmd, &tmp)
        .args(["configure", "--get", "security.allowlist.bypass", "--raw"])
        .assert();

    assert2.success().stdout("true\n");

    // Test env variable override
    let env_cmd = goose_bin();
    let assert3 = with_isolated_home(env_cmd, &tmp)
        .env("GOOSE_ALLOWLIST_BYPASS", "false")
        .args(["configure", "--get", "security.allowlist.bypass", "--raw"])
        .assert();

    assert3.success().stdout("false\n");
}

#[test]
fn test_cli_theme_precedence_integration() {
    let tmp = TempDir::new().unwrap();

    // Set file value
    let set_cmd = goose_bin();
    let _ = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "cli.theme=file-theme"])
        .assert()
        .success();

    // Test env overrides file
    let env_cmd = goose_bin();
    let assert = with_isolated_home(env_cmd, &tmp)
        .env("GOOSE_CLI_THEME", "env-theme")
        .args(["configure", "--get", "cli.theme", "--raw"])
        .assert();

    assert.success().stdout("env-theme\n");

    // Test CLI overlay overrides env
    let cli_cmd = goose_bin();
    let assert2 = with_isolated_home(cli_cmd, &tmp)
        .env("GOOSE_CLI_THEME", "env-theme")
        .args([
            "--set",
            "cli.theme=cli-theme",
            "configure",
            "--get",
            "cli.theme",
            "--raw",
        ])
        .assert();

    assert2.success().stdout("cli-theme\n");

    // Test file value is still there when no env/cli
    let file_cmd = goose_bin();
    let assert3 = with_isolated_home(file_cmd, &tmp)
        .args(["configure", "--get", "cli.theme", "--raw"])
        .assert();

    assert3.success().stdout("file-theme\n");
}

#[test]
fn test_cli_theme_default_value() {
    let tmp = TempDir::new().unwrap();

    // With no configuration, should get default
    let cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .args(["configure", "--get", "cli.theme", "--raw"])
        .assert();

    assert.success().stdout("dark\n");
}

#[test]
#[ignore] // TODO: Fix file persistence issue in test environment
fn test_scheduler_type_precedence_integration() {
    let tmp = TempDir::new().unwrap();

    // Set file value - use a non-default value
    let set_cmd = goose_bin();
    let _ = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "scheduler.type=temporal"])
        .assert()
        .success();

    // Verify file value was set
    let get_cmd = goose_bin();
    let assert = with_isolated_home(get_cmd, &tmp)
        .args(["configure", "--get", "scheduler.type", "--raw"])
        .assert();

    assert.success().stdout("temporal\n");

    // Test env overrides file
    let env_cmd = goose_bin();
    let assert2 = with_isolated_home(env_cmd, &tmp)
        .env("GOOSE_SCHEDULER_TYPE", "legacy")
        .args(["configure", "--get", "scheduler.type", "--raw"])
        .assert();

    assert2.success().stdout("legacy\n");

    // Test CLI overlay overrides all
    let cli_cmd = goose_bin();
    let assert3 = with_isolated_home(cli_cmd, &tmp)
        .env("GOOSE_SCHEDULER_TYPE", "legacy")
        .args([
            "--set",
            "scheduler.type=temporal",
            "configure",
            "--get",
            "scheduler.type",
            "--raw",
        ])
        .assert();

    assert3.success().stdout("temporal\n");
}

#[test]
fn test_temperature_validation_integration() {
    let tmp = TempDir::new().unwrap();

    // Valid temperature
    let valid_cmd = goose_bin();
    let assert = with_isolated_home(valid_cmd, &tmp)
        .args(["configure", "--set", "model.temperature=1.5"])
        .assert();

    assert.success();

    // Invalid temperature (too high)
    let invalid_cmd = goose_bin();
    let assert2 = with_isolated_home(invalid_cmd, &tmp)
        .args(["configure", "--set", "model.temperature=2.5"])
        .assert();

    assert2
        .failure()
        .stderr(predicate::str::contains("must be between 0.0 and 2.0"));

    // Invalid temperature (negative)
    let negative_cmd = goose_bin();
    let assert3 = with_isolated_home(negative_cmd, &tmp)
        .args(["configure", "--set", "model.temperature=-0.5"])
        .assert();

    assert3
        .failure()
        .stderr(predicate::str::contains("must be between 0.0 and 2.0"));
}

#[test]
fn test_otlp_protocol_validation_integration() {
    let tmp = TempDir::new().unwrap();

    // Valid protocols
    for protocol in &["grpc", "http", "GRPC", "HTTP"] {
        let cmd = goose_bin();
        let assert = with_isolated_home(cmd, &tmp)
            .args([
                "configure",
                "--set",
                &format!("tracing.otlp.protocol={}", protocol),
            ])
            .assert();

        assert.success();
    }

    // Invalid protocol
    let invalid_cmd = goose_bin();
    let assert = with_isolated_home(invalid_cmd, &tmp)
        .args(["configure", "--set", "tracing.otlp.protocol=https"])
        .assert();

    assert
        .failure()
        .stderr(predicate::str::contains("must be one of: grpc, http"));
}

#[test]
fn test_toolshim_enabled_bool_integration() {
    let tmp = TempDir::new().unwrap();

    // Test setting via canonical env
    let cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .env("GOOSE_TOOLSHIM_ENABLED", "true")
        .args(["configure", "--get", "toolshim.enabled", "--raw"])
        .assert();

    assert.success().stdout("true\n");

    // Test setting via alias env
    let alias_cmd = goose_bin();
    let assert2 = with_isolated_home(alias_cmd, &tmp)
        .env("GOOSE_TOOLSHIM", "false")
        .args(["configure", "--get", "toolshim.enabled", "--raw"])
        .assert();

    assert2.success().stdout("false\n");

    // Test setting via configure
    let set_cmd = goose_bin();
    let _ = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "toolshim.enabled=true"])
        .assert()
        .success();

    let get_cmd = goose_bin();
    let assert3 = with_isolated_home(get_cmd, &tmp)
        .args(["configure", "--get", "toolshim.enabled", "--raw"])
        .assert();

    assert3.success().stdout("true\n");
}

#[test]
fn test_allowlist_url_validation_integration() {
    let tmp = TempDir::new().unwrap();

    // Valid URLs
    let valid_urls = vec![
        "http://example.com",
        "https://example.com",
        "http://localhost:8080",
        "https://api.example.com/path",
    ];

    for url in valid_urls {
        let cmd = goose_bin();
        let assert = with_isolated_home(cmd, &tmp)
            .args([
                "configure",
                "--set",
                &format!("security.allowlist.url={}", url),
            ])
            .assert();

        assert.success();
    }

    // Invalid URLs
    let invalid_urls = vec![
        ("not-a-url", "invalid URL"),
        ("ftp://example.com", "must start with http"),
        ("", "non-empty URL"),
    ];

    for (url, expected_error) in invalid_urls {
        let cmd = goose_bin();
        let assert = with_isolated_home(cmd, &tmp)
            .args([
                "configure",
                "--set",
                &format!("security.allowlist.url={}", url),
            ])
            .assert();

        assert
            .failure()
            .stderr(predicate::str::contains(expected_error));
    }
}

#[test]
fn test_env_alias_precedence_integration() {
    let tmp = TempDir::new().unwrap();

    // Test canonical wins over alias
    let cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .env("GOOSE_MODEL_TEMPERATURE", "1.2") // canonical
        .env("GOOSE_TEMPERATURE", "0.8") // alias
        .args(["configure", "--get", "model.temperature", "--raw"])
        .assert();

    assert.success().stdout("1.2\n");

    // Test alias used when canonical missing
    let alias_cmd = goose_bin();
    let assert2 = with_isolated_home(alias_cmd, &tmp)
        .env("GOOSE_TEMPERATURE", "0.8")
        .args(["configure", "--get", "model.temperature", "--raw"])
        .assert();

    assert2.success().stdout("0.8\n");
}

#[test]
fn test_effective_config_shows_sources() {
    let tmp = TempDir::new().unwrap();

    // Set various sources
    let set_cmd = goose_bin();
    let _ = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "cli.show_cost=true"])
        .assert()
        .success();

    // Show effective config with sources
    let show_cmd = goose_bin();
    let assert = with_isolated_home(show_cmd, &tmp)
        .env("GOOSE_CLI_THEME", "light")
        .args([
            "configure",
            "--show",
            "--format",
            "json",
            "--filter",
            "cli.",
            "--sources",
            "--only-changed",
        ])
        .assert();

    // Ensure success without consuming assert
    let output = assert.get_output().clone();
    assert!(output.status.success());

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    let entries = json.as_array().expect("array");

    // Check theme from env
    let theme_entry = entries
        .iter()
        .find(|e| e.get("key") == Some(&serde_json::json!("cli.theme")))
        .expect("cli.theme should be present");

    assert_eq!(theme_entry.get("value"), Some(&serde_json::json!("light")));
    assert_eq!(theme_entry.get("source"), Some(&serde_json::json!("Env")));
    assert_eq!(
        theme_entry.get("env_name"),
        Some(&serde_json::json!("GOOSE_CLI_THEME"))
    );
    assert_eq!(
        theme_entry.get("alias_used"),
        Some(&serde_json::json!(false))
    );

    // Check show_cost from file
    let cost_entry = entries
        .iter()
        .find(|e| e.get("key") == Some(&serde_json::json!("cli.show_cost")))
        .expect("cli.show_cost should be present");

    assert_eq!(cost_entry.get("value"), Some(&serde_json::json!(true)));
    assert_eq!(cost_entry.get("source"), Some(&serde_json::json!("File")));
}

#[test]
fn test_scheduler_type_enum_validation() {
    let tmp = TempDir::new().unwrap();

    // Valid values
    for value in &["legacy", "temporal", "LEGACY", "Temporal"] {
        let cmd = goose_bin();
        let assert = with_isolated_home(cmd, &tmp)
            .args(["configure", "--set", &format!("scheduler.type={}", value)])
            .assert();

        assert.success();
    }

    // Invalid value
    let invalid_cmd = goose_bin();
    let assert = with_isolated_home(invalid_cmd, &tmp)
        .args(["configure", "--set", "scheduler.type=invalid"])
        .assert();

    assert
        .failure()
        .stderr(predicate::str::contains("must be one of: legacy, temporal"));
}
