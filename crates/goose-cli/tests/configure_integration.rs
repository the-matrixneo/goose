use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

fn goose_bin() -> Command {
    let mut cmd = Command::cargo_bin("goose").expect("binary built");
    cmd
}

fn with_isolated_home(mut cmd: Command, tmp: &TempDir) -> Command {
    // Isolate HOME and XDG to temp dir so etcetera resolves under it
    let home = tmp.path().to_str().unwrap();
    cmd.env("HOME", home)
        .env("USERPROFILE", home)
        .env("XDG_CONFIG_HOME", format!("{home}/.config"))
        .env("GOOSE_DISABLE_KEYRING", "1");
    cmd
}

#[test]
fn configure_show_json_and_table() {
    let tmp = TempDir::new().unwrap();
    // First, ensure we can run `goose configure --show --format json`
    let mut cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .args(["configure", "--show", "--format", "json"]) // default, redacted secrets
        .assert();
    assert.success().stdout(predicate::str::contains("\"key\""));

    // Table with sources
    let mut cmd2 = goose_bin();
    let assert2 = with_isolated_home(cmd2, &tmp)
        .args(["configure", "--show", "--format", "table", "--sources"]) // includes headers
        .assert();
    assert2.success().stdout(predicate::str::contains("Key"));
}

#[test]
fn configure_set_get_unset_persists_non_secret() {
    let tmp = TempDir::new().unwrap();

    // set a non-secret value
    let mut set_cmd = goose_bin();
    let assert = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "llm.model=unit-model"]) // non-secret
        .assert();
    assert
        .success()
        .stdout(predicate::str::contains("set llm.model"));

    // get should return the value (redacted not applied for non-secret)
    let mut get_cmd = goose_bin();
    let assert2 = with_isolated_home(get_cmd, &tmp)
        .args(["configure", "--get", "llm.model", "--raw"]) // print raw only value
        .assert();
    assert2.success().stdout("unit-model\n");

    // unset the value
    let mut unset_cmd = goose_bin();
    let assert3 = with_isolated_home(unset_cmd, &tmp)
        .args(["configure", "--unset", "llm.model"]) // remove
        .assert();
    assert3
        .success()
        .stdout(predicate::str::contains("unset llm.model"));
}

#[test]
fn configure_secret_roundtrip_file_based() {
    let tmp = TempDir::new().unwrap();

    // set secret
    let mut set_cmd = goose_bin();
    let assert = with_isolated_home(set_cmd, &tmp)
        .args([
            "configure",
            "--set",
            "providers.openai.api_key=sk-secret",
            "--secret",
        ])
        .assert();
    assert.success();

    // get without show-secret should redact
    let mut get_redacted = goose_bin();
    let assert2 = with_isolated_home(get_redacted, &tmp)
        .args(["configure", "--get", "providers.openai.api_key", "--raw"]) // raw prints ***
        .assert();
    assert2.success().stdout("***\n");

    // get with show-secret should reveal
    let mut get_reveal = goose_bin();
    let assert3 = with_isolated_home(get_reveal, &tmp)
        .args([
            "configure",
            "--get",
            "providers.openai.api_key",
            "--raw",
            "--show-secret",
        ])
        .assert();
    assert3.success().stdout("sk-secret\n");
}

#[test]
fn overlay_flag_takes_precedence_without_persist() {
    let tmp = TempDir::new().unwrap();

    // Persist a baseline value
    let mut set_cmd = goose_bin();
    let _ = with_isolated_home(set_cmd, &tmp)
        .args(["configure", "--set", "llm.provider=baseline"]) // non-secret
        .assert()
        .success();

    // Run a command with overlay that reads llm.provider indirectly, e.g., 'configure --get'
    let mut get_cmd_overlay = goose_bin();
    let assert = with_isolated_home(get_cmd_overlay, &tmp)
        .args([
            "--set",
            "llm.provider=overlay",
            "configure",
            "--get",
            "llm.provider",
            "--raw",
        ])
        .assert();
    assert.success().stdout("overlay\n");

    // A subsequent run without overlay should still show baseline (not overlay)
    let mut get_cmd_no_overlay = goose_bin();
    let assert2 = with_isolated_home(get_cmd_no_overlay, &tmp)
        .args(["configure", "--get", "llm.provider", "--raw"])
        .assert();
    assert2.success().stdout("baseline\n");
}

#[test]
fn configure_show_filter_only_changed_and_sources_with_alias() {
    let tmp = TempDir::new().unwrap();

    // Use env alias for llm.model and verify sources/alias-used are reported
    let mut cmd = goose_bin();
    let assert = with_isolated_home(cmd, &tmp)
        .env("GOOSE_MODEL", "alias-model")
        .args([
            "configure",
            "--show",
            "--format",
            "json",
            "--filter",
            "llm.",
            "--only-changed",
            "--sources",
        ])
        .assert();

    // Ensure success without consuming assert
    let out = assert.get_output().clone();
    assert!(out.status.success());

    // Parse JSON and assert expectations
    let output = out.stdout.clone();
    let items: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let arr = items.as_array().expect("array");

    // should include llm.model (changed by env), and NOT include llm.provider (unchanged default)
    assert!(arr
        .iter()
        .any(|e| e.get("key") == Some(&serde_json::Value::String("llm.model".into()))));
    assert!(!arr
        .iter()
        .any(|e| e.get("key") == Some(&serde_json::Value::String("llm.provider".into()))));

    let model_entry = arr
        .iter()
        .find(|e| e.get("key") == Some(&serde_json::Value::String("llm.model".into())))
        .unwrap();

    assert_eq!(
        model_entry.get("value").unwrap(),
        &serde_json::Value::String("alias-model".into())
    );
    assert_eq!(
        model_entry.get("is_secret").unwrap(),
        &serde_json::Value::Bool(false)
    );
    // Source info
    assert_eq!(
        model_entry.get("source").unwrap(),
        &serde_json::Value::String("Env".into())
    );
    assert_eq!(
        model_entry.get("env_name").unwrap(),
        &serde_json::Value::String("GOOSE_MODEL".into())
    );
    assert_eq!(
        model_entry.get("alias_used").unwrap(),
        &serde_json::Value::Bool(true)
    );
}
