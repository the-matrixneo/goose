# Comprehensive Goose Configuration Analysis

**Total Configuration Items Found:** 631

## Other (74 items)

### `1=1`

**Usage locations:**
- `crates/goose/src/agents/tool_vectordb.rs:165` (config_delete)

**Example context:**
```rust
// Delete all records instead of dropping the table
                table
                    .delete("1=1") // This will match all records
                    .await
                    .context("Failed to delete all records")?;
```

### `API_KEY`

**Usage locations:**
- `crates/goose/src/config/base.rs:923` (env_set_var)
- `crates/goose/src/config/base.rs:926` (env_remove_var)

**Example context:**
```rust
// Test environment variable override
        std::env::set_var("API_KEY", "env_secret");
        let value: String = config.get_secret("api_key")?;
        assert_eq!(value, "env_secret");
```

### `CLAUDE_THINKING_ENABLED`

**Usage locations:**
- `crates/goose/src/providers/formats/anthropic.rs:916` (env_set_var)
- `crates/goose/src/providers/formats/anthropic.rs:945` (env_remove_var)

**Example context:**
```rust
fn test_create_request_with_thinking() -> Result<()> {
        let original_value = std::env::var("CLAUDE_THINKING_ENABLED").ok();
        std::env::set_var("CLAUDE_THINKING_ENABLED", "true");

        let result = (|| {
```

### `CONFIG`

**Usage locations:**
- `crates/goose/src/config/base.rs:1423` (env_set_var)
- `crates/goose/src/config/base.rs:1437` (env_remove_var)

**Example context:**
```rust
// Test JSON object environment variable
        std::env::set_var("CONFIG", "{\"debug\": true, \"level\": 5}");
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestConfig {
```

### `CONTEXT_FILE_NAMES`

**Usage locations:**
- `crates/goose-mcp/src/developer/mod.rs:1714` (env_set_var)
- `crates/goose-mcp/src/developer/mod.rs:1723` (env_remove_var)

**Example context:**
```rust
let dir = TempDir::new().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        std::env::set_var("CONTEXT_FILE_NAMES", r#"["CLAUDE.md", ".goosehints"]"#);

        fs::write("CLAUDE.md", "Custom hints file content from CLAUDE.md").unwrap();
```

### `DATABRICKS_TOKEN`

**Usage locations:**
- `crates/goose/examples/databricks_oauth.rs:16` (env_remove_var)

**Example context:**
```rust
// Clear any token to force OAuth
    std::env::remove_var("DATABRICKS_TOKEN");

    // Create the provider
```

### `ELEVENLABS_API_KEY`

**Usage locations:**
- `crates/goose-server/src/routes/audio.rs:231` (config_delete)

**Example context:**
```rust
}
                            // Delete the non-secret version
                            let _ = config.delete("ELEVENLABS_API_KEY");
                            key
                        }
```

### `ENABLED`

**Usage locations:**
- `crates/goose/src/config/base.rs:1418` (env_set_var)
- `crates/goose/src/config/base.rs:1436` (env_remove_var)

**Example context:**
```rust
// Test boolean environment variable
        std::env::set_var("ENABLED", "true");
        let value: bool = config.get_param("enabled")?;
        assert_eq!(value, true);
```

### `GOOSE_ALLOWLIST`

**Usage locations:**
- `crates/goose-server/src/routes/extension.rs:1057` (env_set_var)
- `crates/goose-server/src/routes/extension.rs:1075` (env_remove_var)

**Example context:**
```rust
// Set the environment variable to point to our mock server
        env::set_var("GOOSE_ALLOWLIST", format!("{}{}", server_url, server_path));

        // Give the server a moment to start
```

### `GOOSE_ALLOWLIST_BYPASS`

**Usage locations:**
- `crates/goose-server/src/routes/extension.rs:1096` (env_set_var)
- `crates/goose-server/src/routes/extension.rs:1112` (env_remove_var)

**Example context:**
```rust
// Set the bypass environment variable
        env::set_var("GOOSE_ALLOWLIST_BYPASS", "true");

        // With bypass enabled, any command should be allowed regardless of allowlist
```

### `GOOSE_CACHE_DIR`

**Usage locations:**
- `crates/goose/tests/pricing_integration_test.rs:9` (env_set_var)
- `crates/goose/tests/pricing_integration_test.rs:101` (env_remove_var)

**Example context:**
```rust
// Use a unique cache directory for this test to avoid conflicts
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("GOOSE_CACHE_DIR", temp_dir.path());

    // Initialize the cache
```

### `GOOSE_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:423` (env_set_var)
- `crates/goose/src/providers/factory.rs:425` (env_remove_var)

**Example context:**
```rust
// Test case 3: With GOOSE_CONTEXT_LIMIT - should override original
        env::set_var("GOOSE_CONTEXT_LIMIT", "64000");
        let _result = create_lead_worker_from_env("openai", &default_model, "gpt-4o");
        env::remove_var("GOOSE_CONTEXT_LIMIT");
```

### `GOOSE_LEAD_FAILURE_THRESHOLD`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:324` (env_set_var)
- `crates/goose/src/providers/factory.rs:352` (env_remove_var)

**Example context:**
```rust
// Test with custom values
        env::set_var("GOOSE_LEAD_TURNS", "7");
        env::set_var("GOOSE_LEAD_FAILURE_THRESHOLD", "4");
        env::set_var("GOOSE_LEAD_FALLBACK_TURNS", "3");
```

### `GOOSE_LEAD_FALLBACK_TURNS`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:325` (env_set_var)
- `crates/goose/src/providers/factory.rs:353` (env_remove_var)

**Example context:**
```rust
env::set_var("GOOSE_LEAD_TURNS", "7");
        env::set_var("GOOSE_LEAD_FAILURE_THRESHOLD", "4");
        env::set_var("GOOSE_LEAD_FALLBACK_TURNS", "3");

        let _result = create("openai", ModelConfig::new_or_fail("gpt-4o-mini"));
```

### `GOOSE_LEAD_MODEL`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:241` (env_set_var)
- `crates/goose/src/providers/factory.rs:270` (env_remove_var)

**Example context:**
```rust
// Test with basic lead model configuration
        env::set_var("GOOSE_LEAD_MODEL", "gpt-4o");

        // This will try to create a lead/worker provider
```

### `GOOSE_LEAD_PROVIDER`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:261` (env_set_var)
- `crates/goose/src/providers/factory.rs:274` (env_remove_var)

**Example context:**
```rust
// Test with different lead provider
        env::set_var("GOOSE_LEAD_PROVIDER", "anthropic");
        env::set_var("GOOSE_LEAD_TURNS", "5");
```

### `GOOSE_LEAD_TURNS`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:262` (env_set_var)
- `crates/goose/src/providers/factory.rs:278` (env_remove_var)

**Example context:**
```rust
// Test with different lead provider
        env::set_var("GOOSE_LEAD_PROVIDER", "anthropic");
        env::set_var("GOOSE_LEAD_TURNS", "5");

        let _result = create("openai", gpt4mini_config);
```

### `GOOSE_MODE`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:535` (env_set_var)
- `crates/goose/src/providers/claude_code.rs:541` (env_remove_var)

**Example context:**
```rust
fn test_permission_mode_flag_construction() {
        // Test that in auto mode, the --permission-mode acceptEdits flag is added
        std::env::set_var("GOOSE_MODE", "auto");

        let config = Config::global();
```

### `GOOSE_MODEL`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1428` (env_set_var)
- `crates/goose/src/scheduler.rs:1522` (env_remove_var)

**Example context:**
```rust
// Set environment variables for the test
        env::set_var("GOOSE_PROVIDER", "test_provider");
        env::set_var("GOOSE_MODEL", "test_model");

        let temp_dir = tempdir()?;
```

### `GOOSE_PROVIDER`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1427` (env_set_var)
- `crates/goose/src/scheduler.rs:1521` (env_remove_var)

**Example context:**
```rust
async fn test_scheduled_session_has_schedule_id() -> Result<(), Box<dyn std::error::Error>> {
        // Set environment variables for the test
        env::set_var("GOOSE_PROVIDER", "test_provider");
        env::set_var("GOOSE_MODEL", "test_model");
```

### `GOOSE_VECTOR_DB_PATH`

**Usage locations:**
- `crates/goose/src/agents/tool_vectordb.rs:554` (env_set_var)
- `crates/goose/src/agents/tool_vectordb.rs:559` (env_remove_var)

**Example context:**
```rust
let custom_path = temp_dir.path().join("custom_vector_db");

        env::set_var("GOOSE_VECTOR_DB_PATH", custom_path.to_str().unwrap());

        let db_path = ToolVectorDB::get_db_path()?;
```

### `GOOSE_WORKER_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:418` (env_set_var)
- `crates/goose/src/providers/factory.rs:420` (env_remove_var)

**Example context:**
```rust
// Test case 2: With GOOSE_WORKER_CONTEXT_LIMIT - should override original
        env::set_var("GOOSE_WORKER_CONTEXT_LIMIT", "32000");
        let _result = create_lead_worker_from_env("openai", &default_model, "gpt-4o");
        env::remove_var("GOOSE_WORKER_CONTEXT_LIMIT");
```

### `HOME`

**Usage locations:**
- `crates/goose-cli/src/logging.rs:206` (env_set_var)
- `crates/goose-cli/src/session/output.rs:888` (env_set_var)
- `crates/goose-cli/src/session/output.rs:905` (env_remove_var)

**Example context:**
```rust
env::set_var("USERPROFILE", temp_dir.path());
        } else {
            env::set_var("HOME", temp_dir.path());
        }
        temp_dir
```

### `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:431` (env_set_var)
- `crates/goose/src/tracing/langfuse_layer.rs:437` (env_remove_var)
- `crates/goose-cli/src/logging.rs:490` (env_set_var)
- `crates/goose-cli/src/logging.rs:495` (env_remove_var)

**Example context:**
```rust
// Test 4: Only public key set (init project)
        env::set_var("LANGFUSE_INIT_PROJECT_PUBLIC_KEY", "test-public-key");
        let observer = create_langfuse_observer();
        assert!(
```

### `LANGFUSE_INIT_PROJECT_SECRET_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:440` (env_set_var)
- `crates/goose/src/tracing/langfuse_layer.rs:446` (env_remove_var)
- `crates/goose-cli/src/logging.rs:491` (env_set_var)

**Example context:**
```rust
// Test 5: Only secret key set (init project)
        env::set_var("LANGFUSE_INIT_PROJECT_SECRET_KEY", "test-secret-key");
        let observer = create_langfuse_observer();
        assert!(
```

### `LANGFUSE_PUBLIC_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:413` (env_set_var)
- `crates/goose/src/tracing/langfuse_layer.rs:419` (env_remove_var)
- `crates/goose-cli/src/logging.rs:483` (env_set_var)
- `crates/goose-cli/src/logging.rs:488` (env_remove_var)

**Example context:**
```rust
// Test 2: Only public key set (regular)
        env::set_var("LANGFUSE_PUBLIC_KEY", "test-public-key");
        let observer = create_langfuse_observer();
        assert!(
```

### `LANGFUSE_SECRET_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:422` (env_set_var)
- `crates/goose/src/tracing/langfuse_layer.rs:428` (env_remove_var)
- `crates/goose-cli/src/logging.rs:484` (env_set_var)
- `crates/goose-cli/src/logging.rs:489` (env_remove_var)

**Example context:**
```rust
// Test 3: Only secret key set (regular)
        env::set_var("LANGFUSE_SECRET_KEY", "test-secret-key");
        let observer = create_langfuse_observer();
        assert!(
```

### `LANGFUSE_URL`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:451` (env_set_var)

**Example context:**
```rust
env::set_var("LANGFUSE_PUBLIC_KEY", "test-public-key");
        env::set_var("LANGFUSE_SECRET_KEY", "test-secret-key");
        env::set_var("LANGFUSE_URL", fixture.mock_server_uri());
        let observer = create_langfuse_observer();
        assert!(
```

### `OTEL_EXPORTER_OTLP_ENDPOINT`

**Usage locations:**
- `crates/goose/src/tracing/otlp_layer.rs:255` (env_set_var)
- `crates/goose/src/tracing/otlp_layer.rs:252` (env_remove_var)

**Example context:**
```rust
assert!(OtlpConfig::from_env().is_none());

        env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://test:4317");
        env::set_var("OTEL_EXPORTER_OTLP_TIMEOUT", "5000");
```

### `OTEL_EXPORTER_OTLP_TIMEOUT`

**Usage locations:**
- `crates/goose/src/tracing/otlp_layer.rs:256` (env_set_var)
- `crates/goose/src/tracing/otlp_layer.rs:268` (env_remove_var)

**Example context:**
```rust
env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://test:4317");
        env::set_var("OTEL_EXPORTER_OTLP_TIMEOUT", "5000");

        let config = OtlpConfig::from_env().unwrap();
```

### `PORT`

**Usage locations:**
- `crates/goose/src/config/base.rs:1413` (env_set_var)
- `crates/goose/src/config/base.rs:1435` (env_remove_var)

**Example context:**
```rust
// Test number environment variable
        std::env::set_var("PORT", "8080");
        let value: i32 = config.get_param("port")?;
        assert_eq!(value, 8080);
```

### `PROVIDER`

**Usage locations:**
- `crates/goose/src/config/base.rs:1408` (env_set_var)
- `crates/goose/src/config/base.rs:1434` (env_remove_var)

**Example context:**
```rust
// Test string environment variable (the original issue case)
        std::env::set_var("PROVIDER", "ANTHROPIC");
        let value: String = config.get_param("provider")?;
        assert_eq!(value, "ANTHROPIC");
```

### `TEST_KEY`

**Usage locations:**
- `crates/goose/src/config/base.rs:814` (env_set_var)

**Example context:**
```rust
// Test with environment variable override
        std::env::set_var("TEST_KEY", "env_value");
        let value: String = config.get_param("test_key")?;
        assert_eq!(value, "env_value");
```

### `TEST_PRECEDENCE`

**Usage locations:**
- `crates/goose/src/config/base.rs:1455` (env_set_var)
- `crates/goose/src/config/base.rs:1462` (env_remove_var)

**Example context:**
```rust
// Set environment variable
        std::env::set_var("TEST_PRECEDENCE", "env_value");

        // Environment variable should take precedence
```

### `TMPDIR`

**Usage locations:**
- `crates/goose-cli/src/logging.rs:260` (env_set_var)

**Example context:**
```rust
env::set_var("HOME", test_dir);
            // Also set TMPDIR to prevent temp directory sharing between tests
            env::set_var("TMPDIR", test_dir);
        }
```

### `USERPROFILE`

**Usage locations:**
- `crates/goose-cli/src/logging.rs:204` (env_set_var)

**Example context:**
```rust
let temp_dir = TempDir::new().unwrap();
        if cfg!(windows) {
            env::set_var("USERPROFILE", temp_dir.path());
        } else {
            env::set_var("HOME", temp_dir.path());
```

### `key`

**Usage locations:**
- `crates/goose/src/config/base.rs:883` (config_delete)

**Example context:**
```rust
assert_eq!(value, "value");

        config.delete("key")?;

        let result: Result<String, ConfigError> = config.get_param("key");
```

## Environment Variables (390 items)

### `ANTHROPIC_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:43` (potential_env_vars)
- `crates/goose/tests/providers.rs:549` (potential_env_vars)
- `crates/goose/src/providers/anthropic.rs:52` (potential_env_vars)

**Example context:**
```rust
],
            ProviderType::OpenAi => &["OPENAI_API_KEY"],
            ProviderType::Anthropic => &["ANTHROPIC_API_KEY"],
            ProviderType::Bedrock => &["AWS_PROFILE"],
            ProviderType::Databricks => &["DATABRICKS_HOST"],
```

### `ANTHROPIC_HOST`

**Usage locations:**
- `crates/goose/src/providers/anthropic.rs:54` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get_param("ANTHROPIC_HOST")
            .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
```

### `API_KEY`

**Usage locations:**
- `crates/goose/src/config/base.rs:923` (potential_env_vars)
- `crates/goose/src/providers/base.rs:170` (potential_env_vars)
- `crates/goose-cli/src/recipes/secret_discovery.rs:236` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:714` (potential_env_vars)

**Example context:**
```rust
// Test environment variable override
        std::env::set_var("API_KEY", "env_secret");
        let value: String = config.get_secret("api_key")?;
        assert_eq!(value, "env_secret");
```

### `API_TOKEN`

**Usage locations:**
- `crates/goose-cli/src/recipes/secret_discovery.rs:272` (potential_env_vars)

**Example context:**
```rust
#[test]
    fn test_secret_requirement_creation() {
        let req = SecretRequirement::new("test-ext".to_string(), "API_TOKEN".to_string());

        assert_eq!(req.key, "API_TOKEN");
```

### `APPDATA`

**Usage locations:**
- `crates/goose-mcp/src/developer/shell.rs:82` (env_var_std)
- `crates/goose-mcp/src/developer/shell.rs:82` (env_var_unwrap)

**Example context:**
```rust
);
        // Add more Windows environment variables as needed
        with_userprofile.replace("%APPDATA%", &env::var("APPDATA").unwrap_or_default())
    } else {
        // Unix-style expansion
```

### `APPINIT_DLLS`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:94` (potential_env_vars)

**Example context:**
```rust
"GOROOT", // Go: Changes root installation directory (could lead to execution hijacking)
        // 🖥️ Windows-specific process & DLL hijacking
        "APPINIT_DLLS", // Forces Windows to load a DLL into every process
        "SESSIONNAME",  // Affects Windows session configuration
        "ComSpec",      // Determines default command interpreter (can replace `cmd.exe`)
```

### `AWS_`

**Usage locations:**
- `crates/goose/src/providers/sagemaker_tgi.rs:48` (potential_env_vars)
- `crates/goose/src/providers/bedrock.rs:48` (potential_env_vars)

**Example context:**
```rust
if let Ok(map) = res {
                map.into_iter()
                    .filter(|(key, _)| key.starts_with("AWS_"))
                    .filter_map(|(key, value)| value.as_str().map(|s| (key, s.to_string())))
                    .for_each(|(key, s)| std::env::set_var(key, s));
```

### `AWS_ACCESS_KEY_ID`

**Usage locations:**
- `crates/goose/tests/providers.rs:480` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/provider_configs.rs:62` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Bedrock",
        &["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY"],
        None,
        bedrock::BedrockProvider::default,
```

### `AWS_PROFILE`

**Usage locations:**
- `crates/goose/tests/agent.rs:44` (potential_env_vars)
- `crates/goose/tests/providers.rs:497` (potential_env_vars)
- `crates/goose/src/providers/sagemaker_tgi.rs:273` (potential_env_vars)
- `crates/goose/src/providers/bedrock.rs:146` (potential_env_vars)

**Example context:**
```rust
ProviderType::OpenAi => &["OPENAI_API_KEY"],
            ProviderType::Anthropic => &["ANTHROPIC_API_KEY"],
            ProviderType::Bedrock => &["AWS_PROFILE"],
            ProviderType::Databricks => &["DATABRICKS_HOST"],
            ProviderType::Google => &["GOOGLE_API_KEY"],
```

### `AWS_REGION`

**Usage locations:**
- `crates/goose/src/providers/sagemaker_tgi.rs:272` (potential_env_vars)

**Example context:**
```rust
vec![
                ConfigKey::new("SAGEMAKER_ENDPOINT_NAME", false, false, None),
                ConfigKey::new("AWS_REGION", true, false, Some("us-east-1")),
                ConfigKey::new("AWS_PROFILE", true, false, Some("default")),
            ],
```

### `AWS_SECRET_ACCESS_KEY`

**Usage locations:**
- `crates/goose/tests/providers.rs:480` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/provider_configs.rs:62` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Bedrock",
        &["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY"],
        None,
        bedrock::BedrockProvider::default,
```

### `AZURE_OPENAI_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:38` (potential_env_vars)
- `crates/goose/tests/providers.rs:466` (potential_env_vars)
- `crates/goose/src/providers/azure.rs:83` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/provider_configs.rs:52` (potential_env_vars)

**Example context:**
```rust
match self {
            ProviderType::Azure => &[
                "AZURE_OPENAI_API_KEY",
                "AZURE_OPENAI_ENDPOINT",
                "AZURE_OPENAI_DEPLOYMENT_NAME",
```

### `AZURE_OPENAI_API_VERSION`

**Usage locations:**
- `crates/goose/src/providers/azure.rs:79` (potential_env_vars)

**Example context:**
```rust
let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| AZURE_DEFAULT_API_VERSION.to_string());
```

### `AZURE_OPENAI_DEPLOYMENT_NAME`

**Usage locations:**
- `crates/goose/tests/agent.rs:40` (potential_env_vars)
- `crates/goose/tests/providers.rs:468` (potential_env_vars)
- `crates/goose/src/providers/azure.rs:77` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/provider_configs.rs:54` (potential_env_vars)

**Example context:**
```rust
"AZURE_OPENAI_API_KEY",
                "AZURE_OPENAI_ENDPOINT",
                "AZURE_OPENAI_DEPLOYMENT_NAME",
            ],
            ProviderType::OpenAi => &["OPENAI_API_KEY"],
```

### `AZURE_OPENAI_ENDPOINT`

**Usage locations:**
- `crates/goose/tests/agent.rs:39` (potential_env_vars)
- `crates/goose/tests/providers.rs:467` (potential_env_vars)
- `crates/goose/src/providers/azure.rs:76` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/provider_configs.rs:53` (potential_env_vars)

**Example context:**
```rust
ProviderType::Azure => &[
                "AZURE_OPENAI_API_KEY",
                "AZURE_OPENAI_ENDPOINT",
                "AZURE_OPENAI_DEPLOYMENT_NAME",
            ],
```

### `CARGO_MANIFEST_DIR`

**Usage locations:**
- `crates/goose-mcp/src/computercontroller/pdf_tool.rs:333` (potential_env_vars)
- `crates/goose-mcp/src/computercontroller/docx_tool.rs:561` (potential_env_vars)
- `crates/goose-mcp/src/computercontroller/xlsx_tool.rs:264` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/message_generator.rs:21` (potential_env_vars)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:144` (potential_env_vars)
- `crates/goose-server/src/bin/generate_schema.rs:9` (env_var_std)
- `crates/goose-server/src/bin/generate_schema.rs:9` (potential_env_vars)

**Example context:**
```rust
#[tokio::test]
    async fn test_pdf_text_extraction() {
        let test_pdf_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/computercontroller/tests/data/test.pdf");
        let cache_dir = tempfile::tempdir().unwrap().into_path();
```

### `CARGO_PKG_VERSION`

**Usage locations:**
- `crates/goose/src/tracing/otlp_layer.rs:57` (potential_env_vars)
- `crates/mcp-client/src/client.rs:131` (potential_env_vars)
- `crates/goose-cli/src/commands/info.rs:34` (potential_env_vars)
- `crates/mcp-server/src/router.rs:141` (potential_env_vars)

**Example context:**
```rust
let resource = Resource::new(vec![
        KeyValue::new("service.name", "goose"),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        KeyValue::new("service.namespace", "goose"),
    ]);
```

### `CLAUDE_CODE_COMMAND`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:36` (potential_env_vars)

**Example context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("CLAUDE_CODE_COMMAND")
            .unwrap_or_else(|_| "claude".to_string());
```

### `CLAUDE_THINKING_BUDGET`

**Usage locations:**
- `crates/goose/src/providers/formats/databricks.rs:563` (env_var_std)
- `crates/goose/src/providers/formats/databricks.rs:563` (potential_env_vars)
- `crates/goose/src/providers/formats/anthropic.rs:419` (env_var_std)
- `crates/goose/src/providers/formats/anthropic.rs:419` (potential_env_vars)

**Example context:**
```rust
if is_claude_sonnet && is_thinking_enabled {
        // Minimum budget_tokens is 1024
        let budget_tokens = std::env::var("CLAUDE_THINKING_BUDGET")
            .unwrap_or_else(|_| "16000".to_string())
            .parse()
```

### `CLAUDE_THINKING_ENABLED`

**Usage locations:**
- `crates/goose/src/providers/anthropic.rs:71` (env_var_std)
- `crates/goose/src/providers/anthropic.rs:71` (env_var_is_ok)
- `crates/goose/src/providers/anthropic.rs:71` (potential_env_vars)
- `crates/goose/src/providers/formats/databricks.rs:560` (env_var_std)
- `crates/goose/src/providers/formats/databricks.rs:560` (env_var_is_ok)
- `crates/goose/src/providers/formats/databricks.rs:560` (potential_env_vars)
- `crates/goose/src/providers/formats/anthropic.rs:416` (env_var_std)
- `crates/goose/src/providers/formats/anthropic.rs:915` (env_var_ok)
- `crates/goose/src/providers/formats/anthropic.rs:416` (env_var_is_ok)
- `crates/goose/src/providers/formats/anthropic.rs:416` (potential_env_vars)

**Example context:**
```rust
let mut headers = Vec::new();

        let is_thinking_enabled = std::env::var("CLAUDE_THINKING_ENABLED").is_ok();
        if self.model.model_name.starts_with("claude-3-7-sonnet-") {
            if is_thinking_enabled {
```

### `CONTEXT_FILE_NAMES`

**Usage locations:**
- `crates/goose-mcp/src/developer/mod.rs:406` (env_var_std)
- `crates/goose-mcp/src/developer/mod.rs:406` (potential_env_vars)

**Example context:**
```rust
};

        let hints_filenames: Vec<String> = std::env::var("CONTEXT_FILE_NAMES")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
```

### `DATABRICKS_BACKOFF_MULTIPLIER`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:163` (potential_env_vars)

**Example context:**
```rust
let backoff_multiplier = config
            .get_param("DATABRICKS_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `DATABRICKS_HOST`

**Usage locations:**
- `crates/goose/tests/agent.rs:45` (potential_env_vars)
- `crates/goose/tests/providers.rs:508` (potential_env_vars)
- `crates/goose/src/providers/databricks.rs:113` (potential_env_vars)

**Example context:**
```rust
ProviderType::Anthropic => &["ANTHROPIC_API_KEY"],
            ProviderType::Bedrock => &["AWS_PROFILE"],
            ProviderType::Databricks => &["DATABRICKS_HOST"],
            ProviderType::Google => &["GOOGLE_API_KEY"],
            ProviderType::Groq => &["GROQ_API_KEY"],
```

### `DATABRICKS_INITIAL_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:157` (potential_env_vars)

**Example context:**
```rust
let initial_interval_ms = config
            .get_param("DATABRICKS_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `DATABRICKS_MAX_RETRIES`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:151` (potential_env_vars)

**Example context:**
```rust
fn load_retry_config(config: &crate::config::Config) -> RetryConfig {
        let max_retries = config
            .get_param("DATABRICKS_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `DATABRICKS_MAX_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:169` (potential_env_vars)

**Example context:**
```rust
let max_interval_ms = config
            .get_param("DATABRICKS_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `DATABRICKS_TOKEN`

**Usage locations:**
- `crates/goose/tests/providers.rs:508` (potential_env_vars)
- `crates/goose/examples/databricks_oauth.rs:16` (potential_env_vars)
- `crates/goose/src/providers/databricks.rs:128` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Databricks",
        &["DATABRICKS_HOST", "DATABRICKS_TOKEN"],
        None,
        databricks::DatabricksProvider::default,
```

### `DISPLAY`

**Usage locations:**
- `crates/goose-mcp/src/computercontroller/platform/linux.rs:50` (env_var_std)

**Example context:**
```rust
}

        if let Ok(display) = std::env::var("DISPLAY") {
            if !display.is_empty() {
                return DisplayServer::X11;
```

### `DYLD_FRAMEWORK_PATH`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:82` (potential_env_vars)

**Example context:**
```rust
"DYLD_LIBRARY_PATH",     // Same as LD_LIBRARY_PATH but for macOS
        "DYLD_INSERT_LIBRARIES", // macOS equivalent of LD_PRELOAD
        "DYLD_FRAMEWORK_PATH",   // Overrides framework lookup paths
        // 🐍 Python / Node / Ruby / Java / Golang hijacking
        "PYTHONPATH",   // Overrides Python module resolution
```

### `DYLD_INSERT_LIBRARIES`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:81` (potential_env_vars)

**Example context:**
```rust
// 🍎 macOS dynamic linker variables
        "DYLD_LIBRARY_PATH",     // Same as LD_LIBRARY_PATH but for macOS
        "DYLD_INSERT_LIBRARIES", // macOS equivalent of LD_PRELOAD
        "DYLD_FRAMEWORK_PATH",   // Overrides framework lookup paths
        // 🐍 Python / Node / Ruby / Java / Golang hijacking
```

### `DYLD_LIBRARY_PATH`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:80` (potential_env_vars)

**Example context:**
```rust
"LD_ASSUME_KERNEL", // Tricks linker into thinking it's running on an older kernel
        // 🍎 macOS dynamic linker variables
        "DYLD_LIBRARY_PATH",     // Same as LD_LIBRARY_PATH but for macOS
        "DYLD_INSERT_LIBRARIES", // macOS equivalent of LD_PRELOAD
        "DYLD_FRAMEWORK_PATH",   // Overrides framework lookup paths
```

### `EDIT_MODE`

**Usage locations:**
- `crates/goose-cli/src/session/builder.rs:395` (potential_env_vars)

**Example context:**
```rust
// Determine editor mode
    let edit_mode = config
        .get_param::<String>("EDIT_MODE")
        .ok()
        .and_then(|edit_mode| match edit_mode.to_lowercase().as_str() {
```

### `ELEVENLABS_API_KEY`

**Usage locations:**
- `crates/goose-server/src/routes/audio.rs:212` (potential_env_vars)

**Example context:**
```rust
// First try to get it as a secret
    let api_key: String = match config.get_secret("ELEVENLABS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
```

### `GCP_BACKOFF_MULTIPLIER`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:148` (potential_env_vars)

**Example context:**
```rust
let backoff_multiplier = config
            .get_param("GCP_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `GCP_INITIAL_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:142` (potential_env_vars)

**Example context:**
```rust
let initial_interval_ms = config
            .get_param("GCP_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_LOCATION`

**Usage locations:**
- `crates/goose/tests/agent.rs:50` (potential_env_vars)
- `crates/goose/src/providers/gcpvertexai.rs:174` (potential_env_vars)

**Example context:**
```rust
ProviderType::Ollama => &[],
            ProviderType::OpenRouter => &["OPENROUTER_API_KEY"],
            ProviderType::GcpVertexAI => &["GCP_PROJECT_ID", "GCP_LOCATION"],
            ProviderType::Xai => &["XAI_API_KEY"],
        }
```

### `GCP_MAX_RETRIES`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:136` (potential_env_vars)

**Example context:**
```rust
// Load max retries for 429 rate limit errors
        let max_retries = config
            .get_param("GCP_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `GCP_MAX_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:154` (potential_env_vars)

**Example context:**
```rust
let max_interval_ms = config
            .get_param("GCP_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_PROJECT_ID`

**Usage locations:**
- `crates/goose/tests/agent.rs:50` (potential_env_vars)
- `crates/goose/src/providers/gcpvertexai.rs:108` (potential_env_vars)

**Example context:**
```rust
ProviderType::Ollama => &[],
            ProviderType::OpenRouter => &["OPENROUTER_API_KEY"],
            ProviderType::GcpVertexAI => &["GCP_PROJECT_ID", "GCP_LOCATION"],
            ProviderType::Xai => &["XAI_API_KEY"],
        }
```

### `GEMINI_CLI_COMMAND`

**Usage locations:**
- `crates/goose/src/providers/gemini_cli.rs:35` (potential_env_vars)

**Example context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("GEMINI_CLI_COMMAND")
            .unwrap_or_else(|_| "gemini".to_string());
```

### `GEM_HOME`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:89` (potential_env_vars)

**Example context:**
```rust
"RUBYOPT",      // Injects Ruby execution flags
        "GEM_PATH",     // Alters where RubyGems looks for installed packages
        "GEM_HOME",     // Changes RubyGems default install location
        "CLASSPATH",    // Java: Controls where classes are loaded from — critical for RCE attacks
        "GO111MODULE",  // Go: Forces use of module proxy or disables it
```

### `GEM_PATH`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:88` (potential_env_vars)

**Example context:**
```rust
"NODE_OPTIONS", // Injects options/scripts into every Node.js process
        "RUBYOPT",      // Injects Ruby execution flags
        "GEM_PATH",     // Alters where RubyGems looks for installed packages
        "GEM_HOME",     // Changes RubyGems default install location
        "CLASSPATH",    // Java: Controls where classes are loaded from — critical for RCE attacks
```

### `GITHUB_ACTIONS`

**Usage locations:**
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (env_var_std)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (env_var_is_ok)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (potential_env_vars)

**Example context:**
```rust
}
    } else {
        if std::env::var("GITHUB_ACTIONS").is_ok() {
            panic!(
                "Test recording is not supported on CI. \
```

### `GITHUB_API_URL`

**Usage locations:**
- `crates/goose-cli/src/recipes/secret_discovery.rs:142` (potential_env_vars)

**Example context:**
```rust
uri: "sse://example.com".to_string(),
                    envs: Envs::new(HashMap::new()),
                    env_keys: vec!["GITHUB_TOKEN".to_string(), "GITHUB_API_URL".to_string()],
                    description: None,
                    timeout: None,
```

### `GITHUB_COPILOT_TOKEN`

**Usage locations:**
- `crates/goose/src/providers/githubcopilot.rs:231` (potential_env_vars)

**Example context:**
```rust
async fn refresh_api_info(&self) -> Result<CopilotTokenInfo> {
        let config = Config::global();
        let token = match config.get_secret::<String>("GITHUB_COPILOT_TOKEN") {
            Ok(token) => token,
            Err(err) => match err {
```

### `GITHUB_TOKEN`

**Usage locations:**
- `crates/goose-cli/src/recipes/secret_discovery.rs:9` (potential_env_vars)

**Example context:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct SecretRequirement {
    /// The environment variable name (e.g., "GITHUB_TOKEN")
    pub key: String,
    /// The name of the extension that requires this secret
```

### `GOOGLE_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:46` (potential_env_vars)
- `crates/goose/tests/providers.rs:571` (potential_env_vars)
- `crates/goose/src/providers/google.rs:59` (potential_env_vars)

**Example context:**
```rust
ProviderType::Bedrock => &["AWS_PROFILE"],
            ProviderType::Databricks => &["DATABRICKS_HOST"],
            ProviderType::Google => &["GOOGLE_API_KEY"],
            ProviderType::Groq => &["GROQ_API_KEY"],
            ProviderType::Ollama => &[],
```

### `GOOGLE_APPLICATION_CREDENTIALS`

**Usage locations:**
- `crates/goose/src/providers/gcpauth.rs:213` (potential_env_vars)

**Example context:**
```rust
fn get_env_credentials_path(env_ops: &impl EnvOps) -> Result<String, AuthError> {
        env_ops
            .get_var("GOOGLE_APPLICATION_CREDENTIALS")
            .map_err(|_| {
                AuthError::Credentials("GOOGLE_APPLICATION_CREDENTIALS not set".to_string())
```

### `GOOGLE_DRIVE_CREDENTIALS_PATH`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:104` (env_var_std)
- `crates/goose-mcp/src/google_drive/mod.rs:104` (potential_env_vars)

**Example context:**
```rust
let keyfile_path_str = env::var("GOOGLE_DRIVE_OAUTH_PATH")
            .unwrap_or_else(|_| "./gcp-oauth.keys.json".to_string());
        let credentials_path_str = env::var("GOOGLE_DRIVE_CREDENTIALS_PATH")
            .unwrap_or_else(|_| "./gdrive-server-credentials.json".to_string());
```

### `GOOGLE_DRIVE_DISK_FALLBACK`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:47` (potential_env_vars)

**Example context:**
```rust
pub const KEYCHAIN_SERVICE: &str = "mcp_google_drive";
pub const KEYCHAIN_USERNAME: &str = "oauth_credentials";
pub const KEYCHAIN_DISK_FALLBACK_ENV: &str = "GOOGLE_DRIVE_DISK_FALLBACK";

const GOOGLE_DRIVE_SCOPES: Scope = Scope::Full;
```

### `GOOGLE_DRIVE_OAUTH_CONFIG`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:119` (env_var_std)
- `crates/goose-mcp/src/google_drive/mod.rs:119` (potential_env_vars)

**Example context:**
```rust
);

        if let Ok(oauth_config) = env::var("GOOGLE_DRIVE_OAUTH_CONFIG") {
            // Ensure the parent directory exists (create_dir_all is idempotent)
            if let Some(parent) = keyfile_path.parent() {
```

### `GOOGLE_DRIVE_OAUTH_PATH`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:102` (env_var_std)
- `crates/goose-mcp/src/google_drive/mod.rs:102` (potential_env_vars)

**Example context:**
```rust
Arc<CredentialsManager>,
    ) {
        let keyfile_path_str = env::var("GOOGLE_DRIVE_OAUTH_PATH")
            .unwrap_or_else(|_| "./gcp-oauth.keys.json".to_string());
        let credentials_path_str = env::var("GOOGLE_DRIVE_CREDENTIALS_PATH")
```

### `GOOGLE_HOST`

**Usage locations:**
- `crates/goose/src/providers/google.rs:61` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get_param("GOOGLE_HOST")
            .unwrap_or_else(|_| GOOGLE_API_HOST.to_string());
```

### `GOOSE_ALLOWLIST`

**Usage locations:**
- `crates/goose-server/src/routes/extension.rs:351` (env_var_std)
- `crates/goose-server/src/routes/extension.rs:351` (potential_env_vars)

**Example context:**
```rust
#[allow(dead_code)]
fn fetch_allowed_extensions() -> Option<AllowedExtensions> {
    match env::var("GOOSE_ALLOWLIST") {
        Err(_) => {
            // Environment variable not set, no allowlist to enforce
```

### `GOOSE_ALLOWLIST_BYPASS`

**Usage locations:**
- `crates/goose-server/src/routes/extension.rs:392` (env_var_std)
- `crates/goose-server/src/routes/extension.rs:392` (potential_env_vars)

**Example context:**
```rust
fn is_command_allowed(cmd: &str, args: &[String]) -> bool {
    // Check if bypass is enabled
    if let Ok(bypass_value) = env::var("GOOSE_ALLOWLIST_BYPASS") {
        if bypass_value.to_lowercase() == "true" {
            // Bypass the allowlist check
```

### `GOOSE_AUTO_COMPACT_THRESHOLD`

**Usage locations:**
- `crates/goose/src/context_mgmt/auto_compact.rs:70` (potential_env_vars)

**Example context:**
```rust
let threshold = threshold_override.unwrap_or_else(|| {
        config
            .get_param::<f64>("GOOSE_AUTO_COMPACT_THRESHOLD")
            .unwrap_or(0.3) // Default to 30%
    });
```

### `GOOSE_CACHE_DIR`

**Usage locations:**
- `crates/goose/tests/pricing_integration_test.rs:9` (potential_env_vars)
- `crates/goose/src/providers/pricing.rs:16` (env_var_std)
- `crates/goose/src/providers/pricing.rs:16` (potential_env_vars)

**Example context:**
```rust
// Use a unique cache directory for this test to avoid conflicts
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("GOOSE_CACHE_DIR", temp_dir.path());

    // Initialize the cache
```

### `GOOSE_CLAUDE_CODE_DEBUG`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:310` (env_var_std)
- `crates/goose/src/providers/claude_code.rs:310` (env_var_is_ok)
- `crates/goose/src/providers/claude_code.rs:310` (potential_env_vars)

**Example context:**
```rust
let filtered_system = self.filter_extensions_from_system_prompt(system);

        if std::env::var("GOOSE_CLAUDE_CODE_DEBUG").is_ok() {
            println!("=== CLAUDE CODE PROVIDER DEBUG ===");
            println!("Command: {}", self.command);
```

### `GOOSE_CLI_MIN_PRIORITY`

**Usage locations:**
- `crates/goose-cli/src/commands/configure.rs:1216` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:1216` (env_var_is_ok)
- `crates/goose-cli/src/commands/configure.rs:1216` (potential_env_vars)
- `crates/goose-cli/src/session/mod.rs:1125` (potential_env_vars)
- `crates/goose-cli/src/session/output.rs:270` (potential_env_vars)

**Example context:**
```rust
let config = Config::global();
    // Check if GOOSE_CLI_MIN_PRIORITY is set as an environment variable
    if std::env::var("GOOSE_CLI_MIN_PRIORITY").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_CLI_MIN_PRIORITY environment variable is set and will override the configuration here.");
    }
```

### `GOOSE_CLI_SHOW_COST`

**Usage locations:**
- `crates/goose-cli/src/session/mod.rs:1476` (potential_env_vars)

**Example context:**
```rust
let config = Config::global();
        let show_cost = config
            .get_param::<bool>("GOOSE_CLI_SHOW_COST")
            .unwrap_or(false);
```

### `GOOSE_CLI_SHOW_THINKING`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:176` (env_var_std)
- `crates/goose-cli/src/session/output.rs:176` (env_var_is_ok)
- `crates/goose-cli/src/session/output.rs:176` (potential_env_vars)

**Example context:**
```rust
}
            MessageContent::Thinking(thinking) => {
                if std::env::var("GOOSE_CLI_SHOW_THINKING").is_ok()
                    && std::io::stdout().is_terminal()
                {
```

### `GOOSE_CLI_THEME`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:58` (env_var_std)
- `crates/goose-cli/src/session/output.rs:58` (env_var_ok)
- `crates/goose-cli/src/session/output.rs:58` (potential_env_vars)

**Example context:**
```rust
thread_local! {
    static CURRENT_THEME: RefCell<Theme> = RefCell::new(
        std::env::var("GOOSE_CLI_THEME").ok()
            .map(|val| Theme::from_config_str(&val))
            .unwrap_or_else(||
```

### `GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:506` (potential_env_vars)

**Example context:**
```rust
fn get_tool_params_max_length() -> usize {
    Config::global()
        .get_param::<usize>("GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH")
        .ok()
        .unwrap_or(40)
```

### `GOOSE_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose/src/model.rs:115` (env_var_std)
- `crates/goose/src/model.rs:115` (potential_env_vars)
- `crates/goose/src/providers/factory.rs:399` (env_var_std)
- `crates/goose/src/providers/factory.rs:399` (env_var_ok)
- `crates/goose/src/providers/factory.rs:125` (potential_env_vars)

**Example context:**
```rust
}
        }
        if let Ok(val) = std::env::var("GOOSE_CONTEXT_LIMIT") {
            return Self::validate_context_limit(&val, "GOOSE_CONTEXT_LIMIT").map(Some);
        }
```

### `GOOSE_CONTEXT_STRATEGY`

**Usage locations:**
- `crates/goose-cli/src/session/mod.rs:958` (potential_env_vars)

**Example context:**
```rust
// Check for user-configured default context strategy
                                let config = Config::global();
                                let context_strategy = config.get_param::<String>("GOOSE_CONTEXT_STRATEGY")
                                    .unwrap_or_else(|_| if interactive { "prompt".to_string() } else { "summarize".to_string() });
```

### `GOOSE_DISABLE_KEYRING`

**Usage locations:**
- `crates/goose/src/config/base.rs:132` (env_var_std)
- `crates/goose/src/config/base.rs:132` (potential_env_vars)

**Example context:**
```rust
let config_path = config_dir.join("config.yaml");

        let secrets = match env::var("GOOSE_DISABLE_KEYRING") {
            Ok(_) => SecretStorage::File {
                path: config_dir.join("secrets.yaml"),
```

### `GOOSE_EDITOR_API_KEY`

**Usage locations:**
- `crates/goose-mcp/src/developer/editor_models/mod.rs:78` (env_var_std)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:78` (env_var_ok)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:78` (potential_env_vars)

**Example context:**
```rust
// Check if basic editor API variables are set
    let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;
```

### `GOOSE_EDITOR_HOST`

**Usage locations:**
- `crates/goose-mcp/src/developer/editor_models/mod.rs:79` (env_var_std)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:79` (env_var_ok)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:79` (potential_env_vars)

**Example context:**
```rust
// Check if basic editor API variables are set
    let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;
```

### `GOOSE_EDITOR_MODEL`

**Usage locations:**
- `crates/goose-mcp/src/developer/editor_models/mod.rs:80` (env_var_std)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:80` (env_var_ok)
- `crates/goose-mcp/src/developer/editor_models/mod.rs:80` (potential_env_vars)

**Example context:**
```rust
let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;

    if api_key.is_empty() || host.is_empty() || model.is_empty() {
```

### `GOOSE_EMBEDDING_MODEL`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:229` (env_var_std)
- `crates/goose/src/providers/litellm.rs:229` (potential_env_vars)
- `crates/goose/src/providers/openai.rs:268` (env_var_std)
- `crates/goose/src/providers/openai.rs:268` (potential_env_vars)
- `crates/goose/src/agents/router_tool_selector.rs:48` (env_var_std)
- `crates/goose/src/agents/router_tool_selector.rs:48` (potential_env_vars)

**Example context:**
```rust
impl EmbeddingCapable for LiteLLMProvider {
    async fn create_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, anyhow::Error> {
        let embedding_model = std::env::var("GOOSE_EMBEDDING_MODEL")
            .unwrap_or_else(|_| "text-embedding-3-small".to_string());
```

### `GOOSE_EMBEDDING_MODEL_PROVIDER`

**Usage locations:**
- `crates/goose/src/agents/router_tool_selector.rs:45` (env_var_std)
- `crates/goose/src/agents/router_tool_selector.rs:45` (env_var_is_ok)
- `crates/goose/src/agents/router_tool_selector.rs:51` (env_var_unwrap)
- `crates/goose/src/agents/router_tool_selector.rs:45` (potential_env_vars)

**Example context:**
```rust
let vector_db = ToolVectorDB::new(Some(table_name)).await?;

        let embedding_provider = if env::var("GOOSE_EMBEDDING_MODEL_PROVIDER").is_ok() {
            // If env var is set, create a new provider for embeddings
            // Get embedding model and provider from environment variables
```

### `GOOSE_GEMINI_CLI_DEBUG`

**Usage locations:**
- `crates/goose/src/providers/gemini_cli.rs:161` (env_var_std)
- `crates/goose/src/providers/gemini_cli.rs:161` (env_var_is_ok)
- `crates/goose/src/providers/gemini_cli.rs:161` (potential_env_vars)

**Example context:**
```rust
full_prompt.push_str("Assistant: ");

        if std::env::var("GOOSE_GEMINI_CLI_DEBUG").is_ok() {
            println!("=== GEMINI CLI PROVIDER DEBUG ===");
            println!("Command: {}", self.command);
```

### `GOOSE_LEAD_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:103` (potential_env_vars)

**Example context:**
```rust
let lead_model_config = ModelConfig::new_with_context_env(
        lead_model_name.to_string(),
        Some("GOOSE_LEAD_CONTEXT_LIMIT"),
    )?;
```

### `GOOSE_LEAD_FAILURE_THRESHOLD`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:291` (env_var_std)
- `crates/goose/src/providers/factory.rs:291` (env_var_ok)
- `crates/goose/src/providers/factory.rs:95` (potential_env_vars)

**Example context:**
```rust
(
                "GOOSE_LEAD_FAILURE_THRESHOLD",
                env::var("GOOSE_LEAD_FAILURE_THRESHOLD").ok(),
            ),
            (
```

### `GOOSE_LEAD_FALLBACK_TURNS`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:295` (env_var_std)
- `crates/goose/src/providers/factory.rs:295` (env_var_ok)
- `crates/goose/src/providers/factory.rs:98` (potential_env_vars)

**Example context:**
```rust
(
                "GOOSE_LEAD_FALLBACK_TURNS",
                env::var("GOOSE_LEAD_FALLBACK_TURNS").ok(),
            ),
        ];
```

### `GOOSE_LEAD_MODEL`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:236` (env_var_std)
- `crates/goose/src/providers/factory.rs:236` (env_var_ok)
- `crates/goose/src/providers/factory.rs:69` (potential_env_vars)

**Example context:**
```rust
fn test_create_lead_worker_provider() {
        // Save current env vars
        let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();
```

### `GOOSE_LEAD_PROVIDER`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:237` (env_var_std)
- `crates/goose/src/providers/factory.rs:237` (env_var_ok)
- `crates/goose/src/providers/factory.rs:87` (potential_env_vars)

**Example context:**
```rust
// Save current env vars
        let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();
```

### `GOOSE_LEAD_TURNS`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:238` (env_var_std)
- `crates/goose/src/providers/factory.rs:238` (env_var_ok)
- `crates/goose/src/providers/factory.rs:92` (potential_env_vars)

**Example context:**
```rust
let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();

        // Test with basic lead model configuration
```

### `GOOSE_MAX_TURNS`

**Usage locations:**
- `crates/goose/src/agents/agent.rs:884` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:1522` (potential_env_vars)

**Example context:**
```rust
.and_then(|s| s.max_turns)
                .unwrap_or_else(|| {
                    config.get_param("GOOSE_MAX_TURNS").unwrap_or(DEFAULT_MAX_TURNS)
                });
```

### `GOOSE_MODE`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:342` (potential_env_vars)
- `crates/goose/src/providers/ollama.rs:134` (potential_env_vars)
- `crates/goose/src/agents/prompt_manager.rs:141` (potential_env_vars)
- `crates/goose/src/agents/agent.rs:1150` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:1121` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:1121` (env_var_is_ok)
- `crates/goose-cli/src/commands/configure.rs:1121` (potential_env_vars)
- `crates/goose-cli/src/session/mod.rs:621` (potential_env_vars)
- `crates/goose-server/src/routes/agent.rs:184` (potential_env_vars)

**Example context:**
```rust
// Add permission mode based on GOOSE_MODE setting
        let config = Config::global();
        if let Ok(goose_mode) = config.get_param::<String>("GOOSE_MODE") {
            if goose_mode.as_str() == "auto" {
                cmd.arg("--permission-mode").arg("acceptEdits");
```

### `GOOSE_MODEL`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1135` (potential_env_vars)
- `crates/goose/src/config/signup_openrouter/mod.rs:171` (potential_env_vars)
- `crates/goose-bench/src/runners/model_runner.rs:78` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:442` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:442` (env_var_unwrap)
- `crates/goose-cli/src/commands/configure.rs:442` (potential_env_vars)
- `crates/goose-cli/src/commands/web.rs:95` (potential_env_vars)
- `crates/goose-cli/src/session/mod.rs:1649` (potential_env_vars)
- `crates/goose-cli/src/session/builder.rs:198` (potential_env_vars)
- `crates/goose-server/src/routes/agent.rs:249` (potential_env_vars)

**Example context:**
```rust
};
        let model_name: String =
            match global_config.get_param("GOOSE_MODEL") {
                Ok(name) => name,
                Err(_) => return Err(JobExecutionError {
```

### `GOOSE_PLANNER_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose-cli/src/session/mod.rs:1654` (potential_env_vars)

**Example context:**
```rust
let model_config =
        ModelConfig::new_with_context_env(model, Some("GOOSE_PLANNER_CONTEXT_LIMIT"))?;
    let reasoner = create(&provider, model_config)?;
```

### `GOOSE_PLANNER_MODEL`

**Usage locations:**
- `crates/goose-cli/src/session/mod.rs:1644` (potential_env_vars)

**Example context:**
```rust
// Try planner-specific model first, fallback to default model
    let model = if let Ok(model) = config.get_param::<String>("GOOSE_PLANNER_MODEL") {
        model
    } else {
```

### `GOOSE_PLANNER_PROVIDER`

**Usage locations:**
- `crates/goose-cli/src/session/mod.rs:1634` (potential_env_vars)

**Example context:**
```rust
// Try planner-specific provider first, fallback to default provider
    let provider = if let Ok(provider) = config.get_param::<String>("GOOSE_PLANNER_PROVIDER") {
        provider
    } else {
```

### `GOOSE_PROVIDER`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1125` (potential_env_vars)
- `crates/goose/src/agents/agent.rs:1370` (potential_env_vars)
- `crates/goose/src/config/signup_openrouter/mod.rs:169` (potential_env_vars)
- `crates/goose-bench/src/runners/model_runner.rs:79` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:297` (potential_env_vars)
- `crates/goose-cli/src/commands/web.rs:87` (potential_env_vars)
- `crates/goose-cli/src/session/mod.rs:1480` (potential_env_vars)
- `crates/goose-cli/src/session/builder.rs:187` (potential_env_vars)

**Example context:**
```rust
} else {
        let global_config = Config::global();
        let provider_name: String = match global_config.get_param("GOOSE_PROVIDER") {
            Ok(name) => name,
            Err(_) => return Err(JobExecutionError {
```

### `GOOSE_PROVIDER__API_KEY`

**Usage locations:**
- `crates/goose-server/src/error.rs:36` (potential_env_vars)

**Example context:**
```rust
fn test_env_var_conversion() {
        assert_eq!(to_env_var("type"), "GOOSE_PROVIDER__TYPE");
        assert_eq!(to_env_var("api_key"), "GOOSE_PROVIDER__API_KEY");
        assert_eq!(to_env_var("provider.host"), "GOOSE_PROVIDER__HOST");
        assert_eq!(to_env_var("provider.api_key"), "GOOSE_PROVIDER__API_KEY");
```

### `GOOSE_PROVIDER__HOST`

**Usage locations:**
- `crates/goose-server/src/error.rs:37` (potential_env_vars)

**Example context:**
```rust
assert_eq!(to_env_var("type"), "GOOSE_PROVIDER__TYPE");
        assert_eq!(to_env_var("api_key"), "GOOSE_PROVIDER__API_KEY");
        assert_eq!(to_env_var("provider.host"), "GOOSE_PROVIDER__HOST");
        assert_eq!(to_env_var("provider.api_key"), "GOOSE_PROVIDER__API_KEY");
    }
```

### `GOOSE_PROVIDER__TYPE`

**Usage locations:**
- `crates/goose-server/src/error.rs:35` (potential_env_vars)

**Example context:**
```rust
#[test]
    fn test_env_var_conversion() {
        assert_eq!(to_env_var("type"), "GOOSE_PROVIDER__TYPE");
        assert_eq!(to_env_var("api_key"), "GOOSE_PROVIDER__API_KEY");
        assert_eq!(to_env_var("provider.host"), "GOOSE_PROVIDER__HOST");
```

### `GOOSE_RECIPE_GITHUB_REPO`

**Usage locations:**
- `crates/goose-cli/src/recipes/github_recipe.rs:31` (potential_env_vars)

**Example context:**
```rust
}

pub const GOOSE_RECIPE_GITHUB_REPO_CONFIG_KEY: &str = "GOOSE_RECIPE_GITHUB_REPO";
pub fn retrieve_recipe_from_github(
    recipe_name: &str,
```

### `GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS`

**Usage locations:**
- `crates/goose/src/agents/retry.rs:35` (potential_env_vars)

**Example context:**
```rust
/// Environment variable for configuring on_failure timeout globally
const GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS: &str = "GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS";

/// Manages retry state and operations for agent execution
```

### `GOOSE_RECIPE_PATH`

**Usage locations:**
- `crates/goose-cli/src/recipes/search_recipe.rs:16` (potential_env_vars)

**Example context:**
```rust
};

const GOOSE_RECIPE_PATH_ENV_VAR: &str = "GOOSE_RECIPE_PATH";

pub fn retrieve_recipe_file(recipe_name: &str) -> Result<RecipeFile> {
```

### `GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS`

**Usage locations:**
- `crates/goose/src/agents/retry.rs:32` (potential_env_vars)

**Example context:**
```rust
/// Environment variable for configuring retry timeout globally
const GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS: &str = "GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS";

/// Environment variable for configuring on_failure timeout globally
```

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`

**Usage locations:**
- `crates/goose/src/agents/tool_route_manager.rs:78` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:1174` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:1174` (env_var_is_ok)
- `crates/goose-cli/src/commands/configure.rs:1174` (potential_env_vars)

**Example context:**
```rust
let config = Config::global();
        let router_tool_selection_strategy = config
            .get_param("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
            .unwrap_or_else(|_| "default".to_string());
```

### `GOOSE_SCHEDULER_TYPE`

**Usage locations:**
- `crates/goose/src/scheduler_factory.rs:22` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:1469` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:1469` (env_var_is_ok)
- `crates/goose-cli/src/commands/configure.rs:1469` (potential_env_vars)
- `crates/goose-cli/src/commands/schedule.rs:266` (env_var_std)
- `crates/goose-cli/src/commands/schedule.rs:266` (env_var_unwrap)
- `crates/goose-cli/src/commands/schedule.rs:266` (potential_env_vars)

**Example context:**
```rust
// Check scheduler type preference from GOOSE_SCHEDULER_TYPE
        match config.get_param::<String>("GOOSE_SCHEDULER_TYPE") {
            Ok(scheduler_type) => {
                tracing::debug!(
```

### `GOOSE_SERVER__SECRET_KEY`

**Usage locations:**
- `crates/goose-server/src/commands/agent.rs:31` (env_var_std)
- `crates/goose-server/src/commands/agent.rs:31` (env_var_unwrap)
- `crates/goose-server/src/commands/agent.rs:31` (potential_env_vars)

**Example context:**
```rust
let secret_key =
        std::env::var("GOOSE_SERVER__SECRET_KEY").unwrap_or_else(|_| "test".to_string());

    let new_agent = Agent::new();
```

### `GOOSE_SUBAGENT_MAX_TURNS`

**Usage locations:**
- `crates/goose/src/agents/subagent_task_config.rs:11` (potential_env_vars)

**Example context:**
```rust
/// Environment variable name for configuring max turns
pub const GOOSE_SUBAGENT_MAX_TURNS_ENV_VAR: &str = "GOOSE_SUBAGENT_MAX_TURNS";

/// Configuration for task execution with all necessary dependencies
```

### `GOOSE_SYSTEM_PROMPT_FILE_PATH`

**Usage locations:**
- `crates/goose-cli/src/session/builder.rs:563` (potential_env_vars)

**Example context:**
```rust
// Only override system prompt if a system override exists
    let system_prompt_file: Option<String> = config.get_param("GOOSE_SYSTEM_PROMPT_FILE_PATH").ok();
    if let Some(ref path) = system_prompt_file {
        let override_prompt =
```

### `GOOSE_TEMPERATURE`

**Usage locations:**
- `crates/goose/src/model.rs:141` (env_var_std)
- `crates/goose/src/model.rs:141` (potential_env_vars)

**Example context:**
```rust
fn parse_temperature() -> Result<Option<f32>, ConfigError> {
        if let Ok(val) = std::env::var("GOOSE_TEMPERATURE") {
            let temp = val.parse::<f32>().map_err(|_| {
                ConfigError::InvalidValue(
```

### `GOOSE_TEMPORAL_BIN`

**Usage locations:**
- `crates/goose/src/temporal_scheduler.rs:458` (env_var_std)
- `crates/goose/src/temporal_scheduler.rs:458` (potential_env_vars)

**Example context:**
```rust
// Check environment variable override
        if let Ok(binary_path) = std::env::var("GOOSE_TEMPORAL_BIN") {
            if std::path::Path::new(&binary_path).exists() {
                tracing::info!(
```

### `GOOSE_TEST_PROVIDER`

**Usage locations:**
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (env_var_std)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (potential_env_vars)

**Example context:**
```rust
F: Fn(&ScenarioResult) -> Result<()> + Send + Sync + 'static,
{
    if let Ok(only_provider) = std::env::var("GOOSE_TEST_PROVIDER") {
        let active_providers = get_provider_configs();
        let config = active_providers
```

### `GOOSE_TOOLSHIM`

**Usage locations:**
- `crates/goose/src/model.rs:162` (env_var_std)
- `crates/goose/src/model.rs:162` (potential_env_vars)
- `crates/goose-bench/src/runners/model_runner.rs:227` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:454` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:454` (potential_env_vars)

**Example context:**
```rust
fn parse_toolshim() -> Result<bool, ConfigError> {
        if let Ok(val) = std::env::var("GOOSE_TOOLSHIM") {
            match val.to_lowercase().as_str() {
                "1" | "true" | "yes" | "on" => Ok(true),
```

### `GOOSE_TOOLSHIM_OLLAMA_MODEL`

**Usage locations:**
- `crates/goose/src/model.rs:178` (env_var_std)
- `crates/goose/src/model.rs:178` (potential_env_vars)
- `crates/goose/src/providers/toolshim.rs:282` (env_var_std)
- `crates/goose/src/providers/toolshim.rs:282` (potential_env_vars)
- `crates/goose-bench/src/runners/model_runner.rs:230` (potential_env_vars)
- `crates/goose-cli/src/commands/configure.rs:461` (env_var_std)
- `crates/goose-cli/src/commands/configure.rs:461` (env_var_ok)
- `crates/goose-cli/src/commands/configure.rs:461` (potential_env_vars)

**Example context:**
```rust
fn parse_toolshim_model() -> Result<Option<String>, ConfigError> {
        match std::env::var("GOOSE_TOOLSHIM_OLLAMA_MODEL") {
            Ok(val) if val.trim().is_empty() => Err(ConfigError::InvalidValue(
                "GOOSE_TOOLSHIM_OLLAMA_MODEL".to_string(),
```

### `GOOSE_VECTOR_DB_PATH`

**Usage locations:**
- `crates/goose/src/agents/tool_vectordb.rs:62` (potential_env_vars)

**Example context:**
```rust
// Check for custom database path override
        if let Ok(custom_path) = config.get_param::<String>("GOOSE_VECTOR_DB_PATH") {
            let path = PathBuf::from(custom_path);
```

### `GOOSE_WORKER_CONTEXT_LIMIT`

**Usage locations:**
- `crates/goose/src/providers/factory.rs:397` (env_var_std)
- `crates/goose/src/providers/factory.rs:397` (env_var_ok)
- `crates/goose/src/providers/factory.rs:121` (potential_env_vars)

**Example context:**
```rust
(
                "GOOSE_WORKER_CONTEXT_LIMIT",
                env::var("GOOSE_WORKER_CONTEXT_LIMIT").ok(),
            ),
            ("GOOSE_CONTEXT_LIMIT", env::var("GOOSE_CONTEXT_LIMIT").ok()),
```

### `GOOSE_WORKING_DIR`

**Usage locations:**
- `crates/goose-mcp/src/memory/mod.rs:228` (env_var_std)
- `crates/goose-mcp/src/memory/mod.rs:228` (potential_env_vars)

**Example context:**
```rust
// Check for .goose/memory in current directory
        let local_memory_dir = std::env::var("GOOSE_WORKING_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap())
```

### `GROQ_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:47` (potential_env_vars)
- `crates/goose/tests/providers.rs:542` (potential_env_vars)
- `crates/goose/src/providers/groq.rs:38` (potential_env_vars)

**Example context:**
```rust
ProviderType::Databricks => &["DATABRICKS_HOST"],
            ProviderType::Google => &["GOOGLE_API_KEY"],
            ProviderType::Groq => &["GROQ_API_KEY"],
            ProviderType::Ollama => &[],
            ProviderType::OpenRouter => &["OPENROUTER_API_KEY"],
```

### `GROQ_HOST`

**Usage locations:**
- `crates/goose/src/providers/groq.rs:40` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("GROQ_API_KEY")?;
        let host: String = config
            .get_param("GROQ_HOST")
            .unwrap_or_else(|_| GROQ_API_HOST.to_string());
```

### `HOME`

**Usage locations:**
- `crates/goose/src/providers/gcpauth.rs:224` (potential_env_vars)
- `crates/goose/src/providers/claude_code.rs:53` (env_var_std)
- `crates/goose/src/providers/claude_code.rs:53` (env_var_ok)
- `crates/goose/src/providers/claude_code.rs:53` (potential_env_vars)
- `crates/goose/src/providers/gemini_cli.rs:52` (env_var_std)
- `crates/goose/src/providers/gemini_cli.rs:52` (env_var_ok)
- `crates/goose/src/providers/gemini_cli.rs:52` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:206` (potential_env_vars)
- `crates/goose-cli/src/session/output.rs:885` (env_var_std)
- `crates/goose-cli/src/session/output.rs:885` (env_var_ok)
- `crates/goose-cli/src/session/output.rs:885` (potential_env_vars)

**Example context:**
```rust
} else {
            (
                "HOME",
                ".config/gcloud/application_default_credentials.json",
            )
```

### `INVALID_ARGUMENT`

**Usage locations:**
- `crates/goose/src/providers/utils.rs:230` (potential_env_vars)

**Example context:**
```rust
error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
                    let error_status = error.get("status").and_then(|s| s.as_str()).unwrap_or("Unknown status");
                    if error_status == "INVALID_ARGUMENT" && error_msg.to_lowercase().contains("exceeds") {
                        return Err(ProviderError::ContextLengthExceeded(error_msg.to_string()));
                    }
```

### `LABEL_VIEW_BASIC`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:1106` (potential_env_vars)

**Example context:**
```rust
.labels()
                .list()
                .param("view", "LABEL_VIEW_BASIC");
            // .param("view", "LABEL_VIEW_FULL");
```

### `LABEL_VIEW_FULL`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:1107` (potential_env_vars)

**Example context:**
```rust
.list()
                .param("view", "LABEL_VIEW_BASIC");
            // .param("view", "LABEL_VIEW_FULL");

            let label_results = match label_builder.doit().await {
```

### `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:157` (env_var_std)
- `crates/goose/src/tracing/langfuse_layer.rs:157` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:466` (env_var_std)
- `crates/goose-cli/src/logging.rs:466` (env_var_ok)
- `crates/goose-cli/src/logging.rs:465` (potential_env_vars)

**Example context:**
```rust
pub fn create_langfuse_observer() -> Option<ObservationLayer> {
    let public_key = env::var("LANGFUSE_PUBLIC_KEY")
        .or_else(|_| env::var("LANGFUSE_INIT_PROJECT_PUBLIC_KEY"))
        .unwrap_or_default(); // Use empty string if not found
```

### `LANGFUSE_INIT_PROJECT_SECRET_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:161` (env_var_std)
- `crates/goose/src/tracing/langfuse_layer.rs:161` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:470` (env_var_std)
- `crates/goose-cli/src/logging.rs:470` (env_var_ok)
- `crates/goose-cli/src/logging.rs:469` (potential_env_vars)

**Example context:**
```rust
let secret_key = env::var("LANGFUSE_SECRET_KEY")
        .or_else(|_| env::var("LANGFUSE_INIT_PROJECT_SECRET_KEY"))
        .unwrap_or_default(); // Use empty string if not found
```

### `LANGFUSE_PUBLIC_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:156` (env_var_std)
- `crates/goose/src/tracing/langfuse_layer.rs:156` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:461` (env_var_std)
- `crates/goose-cli/src/logging.rs:461` (env_var_ok)
- `crates/goose-cli/src/logging.rs:461` (potential_env_vars)

**Example context:**
```rust
pub fn create_langfuse_observer() -> Option<ObservationLayer> {
    let public_key = env::var("LANGFUSE_PUBLIC_KEY")
        .or_else(|_| env::var("LANGFUSE_INIT_PROJECT_PUBLIC_KEY"))
        .unwrap_or_default(); // Use empty string if not found
```

### `LANGFUSE_SECRET_KEY`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:160` (env_var_std)
- `crates/goose/src/tracing/langfuse_layer.rs:160` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:462` (env_var_std)
- `crates/goose-cli/src/logging.rs:462` (env_var_ok)
- `crates/goose-cli/src/logging.rs:462` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_default(); // Use empty string if not found

    let secret_key = env::var("LANGFUSE_SECRET_KEY")
        .or_else(|_| env::var("LANGFUSE_INIT_PROJECT_SECRET_KEY"))
        .unwrap_or_default(); // Use empty string if not found
```

### `LANGFUSE_URL`

**Usage locations:**
- `crates/goose/src/tracing/langfuse_layer.rs:169` (env_var_std)
- `crates/goose/src/tracing/langfuse_layer.rs:169` (env_var_unwrap)
- `crates/goose/src/tracing/langfuse_layer.rs:169` (potential_env_vars)
- `crates/goose-cli/src/logging.rs:463` (env_var_std)
- `crates/goose-cli/src/logging.rs:463` (env_var_ok)
- `crates/goose-cli/src/logging.rs:463` (potential_env_vars)

**Example context:**
```rust
}

    let base_url = env::var("LANGFUSE_URL").unwrap_or_else(|_| DEFAULT_LANGFUSE_URL.to_string());

    let batch_manager = Arc::new(Mutex::new(LangfuseBatchManager::new(
```

### `LD_ASSUME_KERNEL`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:78` (potential_env_vars)

**Example context:**
```rust
"LD_DEBUG",         // Enables verbose linker logging (information disclosure risk)
        "LD_BIND_NOW",      // Forces immediate symbol resolution, affecting ASLR
        "LD_ASSUME_KERNEL", // Tricks linker into thinking it's running on an older kernel
        // 🍎 macOS dynamic linker variables
        "DYLD_LIBRARY_PATH",     // Same as LD_LIBRARY_PATH but for macOS
```

### `LD_AUDIT`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:75` (potential_env_vars)

**Example context:**
```rust
"LD_LIBRARY_PATH",  // Alters shared library resolution
        "LD_PRELOAD",       // Forces preloading of shared libraries — common attack vector
        "LD_AUDIT",         // Loads a monitoring library that can intercept execution
        "LD_DEBUG",         // Enables verbose linker logging (information disclosure risk)
        "LD_BIND_NOW",      // Forces immediate symbol resolution, affecting ASLR
```

### `LD_BIND_NOW`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:77` (potential_env_vars)

**Example context:**
```rust
"LD_AUDIT",         // Loads a monitoring library that can intercept execution
        "LD_DEBUG",         // Enables verbose linker logging (information disclosure risk)
        "LD_BIND_NOW",      // Forces immediate symbol resolution, affecting ASLR
        "LD_ASSUME_KERNEL", // Tricks linker into thinking it's running on an older kernel
        // 🍎 macOS dynamic linker variables
```

### `LD_DEBUG`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:76` (potential_env_vars)

**Example context:**
```rust
"LD_PRELOAD",       // Forces preloading of shared libraries — common attack vector
        "LD_AUDIT",         // Loads a monitoring library that can intercept execution
        "LD_DEBUG",         // Enables verbose linker logging (information disclosure risk)
        "LD_BIND_NOW",      // Forces immediate symbol resolution, affecting ASLR
        "LD_ASSUME_KERNEL", // Tricks linker into thinking it's running on an older kernel
```

### `LD_LIBRARY_PATH`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:73` (potential_env_vars)

**Example context:**
```rust
"windir",     // Windows: Alternative to SystemRoot (used in legacy apps)
        // 🧬 Dynamic linker hijacking (Linux/macOS)
        "LD_LIBRARY_PATH",  // Alters shared library resolution
        "LD_PRELOAD",       // Forces preloading of shared libraries — common attack vector
        "LD_AUDIT",         // Loads a monitoring library that can intercept execution
```

### `LD_PRELOAD`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:74` (potential_env_vars)

**Example context:**
```rust
// 🧬 Dynamic linker hijacking (Linux/macOS)
        "LD_LIBRARY_PATH",  // Alters shared library resolution
        "LD_PRELOAD",       // Forces preloading of shared libraries — common attack vector
        "LD_AUDIT",         // Loads a monitoring library that can intercept execution
        "LD_DEBUG",         // Enables verbose linker logging (information disclosure risk)
```

### `LITELLM_API_KEY`

**Usage locations:**
- `crates/goose/tests/providers.rs:610` (potential_env_vars)
- `crates/goose/src/providers/litellm.rs:34` (potential_env_vars)

**Example context:**
```rust
let env_mods = HashMap::from_iter([
        ("LITELLM_HOST", Some("http://localhost:4000".to_string())),
        ("LITELLM_API_KEY", Some("".to_string())),
    ]);
```

### `LITELLM_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:40` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| "https://api.litellm.ai".to_string());
        let base_path: String = config
            .get_param("LITELLM_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
```

### `LITELLM_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:43` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("LITELLM_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("LITELLM_CUSTOM_HEADERS"))
            .ok()
```

### `LITELLM_HOST`

**Usage locations:**
- `crates/goose/tests/providers.rs:602` (env_var_std)
- `crates/goose/tests/providers.rs:602` (potential_env_vars)
- `crates/goose/src/providers/litellm.rs:37` (potential_env_vars)

**Example context:**
```rust
#[tokio::test]
async fn test_litellm_provider() -> Result<()> {
    if std::env::var("LITELLM_HOST").is_err() {
        println!("LITELLM_HOST not set, skipping test");
        TEST_REPORT.record_skip("LiteLLM");
```

### `LITELLM_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:47` (potential_env_vars)

**Example context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("LITELLM_TIMEOUT").unwrap_or(600);

        let auth = if api_key.is_empty() {
```

### `NODE_OPTIONS`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:86` (potential_env_vars)

**Example context:**
```rust
"PYTHONPATH",   // Overrides Python module resolution
        "PYTHONHOME",   // Overrides Python root directory
        "NODE_OPTIONS", // Injects options/scripts into every Node.js process
        "RUBYOPT",      // Injects Ruby execution flags
        "GEM_PATH",     // Alters where RubyGems looks for installed packages
```

### `NO_COLOR`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:484` (env_var_os)
- `crates/goose-cli/src/session/output.rs:484` (potential_env_vars)

**Example context:**
```rust
pub fn env_no_color() -> bool {
    // if NO_COLOR is defined at all disable colors
    std::env::var_os("NO_COLOR").is_none()
}
```

### `OLLAMA_HOST`

**Usage locations:**
- `crates/goose/tests/providers.rs:533` (potential_env_vars)
- `crates/goose/src/providers/toolshim.rs:88` (potential_env_vars)
- `crates/goose/src/providers/ollama.rs:41` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Ollama",
        &["OLLAMA_HOST"],
        None,
        ollama::OllamaProvider::default,
```

### `OLLAMA_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/ollama.rs:45` (potential_env_vars)

**Example context:**
```rust
let timeout: Duration =
            Duration::from_secs(config.get_param("OLLAMA_TIMEOUT").unwrap_or(OLLAMA_TIMEOUT));

        // OLLAMA_HOST is sometimes just the 'host' or 'host:port' without a scheme
```

### `OPENAI_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:42` (potential_env_vars)
- `crates/goose/tests/providers.rs:454` (potential_env_vars)
- `crates/goose/src/config/base.rs:88` (potential_env_vars)
- `crates/goose/src/providers/factory.rs:256` (potential_env_vars)
- `crates/goose/src/providers/openai.rs:61` (potential_env_vars)
- `crates/goose-server/src/routes/audio.rs:100` (potential_env_vars)

**Example context:**
```rust
"AZURE_OPENAI_DEPLOYMENT_NAME",
            ],
            ProviderType::OpenAi => &["OPENAI_API_KEY"],
            ProviderType::Anthropic => &["ANTHROPIC_API_KEY"],
            ProviderType::Bedrock => &["AWS_PROFILE"],
```

### `OPENAI_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:66` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
            .get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
```

### `OPENAI_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:71` (potential_env_vars)

**Example context:**
```rust
let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("OPENAI_CUSTOM_HEADERS"))
            .ok()
```

### `OPENAI_HOST`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:63` (potential_env_vars)
- `crates/goose-server/src/routes/audio.rs:104` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("OPENAI_API_KEY")?;
        let host: String = config
            .get_param("OPENAI_HOST")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
```

### `OPENAI_ORGANIZATION`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:68` (potential_env_vars)

**Example context:**
```rust
.get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
```

### `OPENAI_PROJECT`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:69` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
```

### `OPENAI_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:75` (potential_env_vars)

**Example context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("OPENAI_TIMEOUT").unwrap_or(600);

        let auth = AuthMethod::BearerToken(api_key);
```

### `OPENROUTER_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:49` (potential_env_vars)
- `crates/goose/tests/providers.rs:560` (potential_env_vars)
- `crates/goose/src/providers/openrouter.rs:46` (potential_env_vars)
- `crates/goose/src/config/signup_openrouter/mod.rs:168` (potential_env_vars)

**Example context:**
```rust
ProviderType::Groq => &["GROQ_API_KEY"],
            ProviderType::Ollama => &[],
            ProviderType::OpenRouter => &["OPENROUTER_API_KEY"],
            ProviderType::GcpVertexAI => &["GCP_PROJECT_ID", "GCP_LOCATION"],
            ProviderType::Xai => &["XAI_API_KEY"],
```

### `OPENROUTER_HOST`

**Usage locations:**
- `crates/goose/src/providers/openrouter.rs:48` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("OPENROUTER_API_KEY")?;
        let host: String = config
            .get_param("OPENROUTER_HOST")
            .unwrap_or_else(|_| "https://openrouter.ai".to_string());
```

### `OTEL_EXPORTER_OTLP_ENDPOINT`

**Usage locations:**
- `crates/goose/src/tracing/otlp_layer.rs:35` (env_var_std)
- `crates/goose/src/tracing/otlp_layer.rs:249` (env_var_ok)
- `crates/goose/src/tracing/otlp_layer.rs:35` (potential_env_vars)
- `crates/goose-cli/src/main.rs:13` (env_var_std)
- `crates/goose-cli/src/main.rs:13` (env_var_is_ok)
- `crates/goose-cli/src/main.rs:13` (potential_env_vars)

**Example context:**
```rust
impl OtlpConfig {
    pub fn from_env() -> Option<Self> {
        if let Ok(endpoint) = env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
            let mut config = Self {
                endpoint,
```

### `OTEL_EXPORTER_OTLP_TIMEOUT`

**Usage locations:**
- `crates/goose/src/tracing/otlp_layer.rs:41` (env_var_std)
- `crates/goose/src/tracing/otlp_layer.rs:250` (env_var_ok)
- `crates/goose/src/tracing/otlp_layer.rs:41` (potential_env_vars)

**Example context:**
```rust
};

            if let Ok(timeout_str) = env::var("OTEL_EXPORTER_OTLP_TIMEOUT") {
                if let Ok(timeout_ms) = timeout_str.parse::<u64>() {
                    config.timeout = Duration::from_millis(timeout_ms);
```

### `PARENT_TOKEN`

**Usage locations:**
- `crates/goose-cli/src/recipes/secret_discovery.rs:293` (potential_env_vars)

**Example context:**
```rust
uri: "sse://parent.com".to_string(),
                envs: Envs::new(HashMap::new()),
                env_keys: vec!["PARENT_TOKEN".to_string()],
                description: None,
                timeout: None,
```

### `PATH`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:86` (env_var_std)
- `crates/goose/src/providers/claude_code.rs:86` (potential_env_vars)
- `crates/goose/src/providers/gemini_cli.rs:90` (env_var_std)
- `crates/goose/src/providers/gemini_cli.rs:90` (potential_env_vars)
- `crates/goose/src/agents/extension.rs:68` (potential_env_vars)
- `crates/goose-cli/src/cli.rs:54` (potential_env_vars)

**Example context:**
```rust
}

        if let Ok(path_var) = std::env::var("PATH") {
            #[cfg(unix)]
            let path_separator = ':';
```

### `PORT`

**Usage locations:**
- `crates/goose/src/temporal_scheduler.rs:127` (env_var_std)

**Example context:**
```rust
// Check PORT environment variable first
        if let Ok(port_str) = std::env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                if Self::is_temporal_service_running(http_client, port).await {
```

### `RANDOM_THINKING_MESSAGES`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:101` (potential_env_vars)

**Example context:**
```rust
let spinner = cliclack::spinner();
        if Config::global()
            .get_param("RANDOM_THINKING_MESSAGES")
            .unwrap_or(true)
        {
```

### `REQUEST_CHANGES`

**Usage locations:**
- `crates/goose/src/providers/formats/google.rs:557` (potential_env_vars)

**Example context:**
```rust
"event": {
                    "description": "Review action to perform",
                    "enum": ["APPROVE", "REQUEST_CHANGES", "COMMENT"],
                    "type": "string"
                },
```

### `SAGEMAKER_ENDPOINT_NAME`

**Usage locations:**
- `crates/goose/tests/providers.rs:593` (potential_env_vars)
- `crates/goose/src/providers/sagemaker_tgi.rs:40` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "SageMakerTgi",
        &["SAGEMAKER_ENDPOINT_NAME"],
        None,
        goose::providers::sagemaker_tgi::SageMakerTgiProvider::default,
```

### `SLACK_TOKEN`

**Usage locations:**
- `crates/goose-cli/src/recipes/secret_discovery.rs:152` (potential_env_vars)

**Example context:**
```rust
args: vec![],
                    envs: Envs::new(HashMap::new()),
                    env_keys: vec!["SLACK_TOKEN".to_string()],
                    timeout: None,
                    description: None,
```

### `SNOWFLAKE_HOST`

**Usage locations:**
- `crates/goose/tests/providers.rs:582` (potential_env_vars)
- `crates/goose/src/providers/snowflake.rs:48` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Snowflake",
        &["SNOWFLAKE_HOST", "SNOWFLAKE_TOKEN"],
        None,
        snowflake::SnowflakeProvider::default,
```

### `SNOWFLAKE_TOKEN`

**Usage locations:**
- `crates/goose/tests/providers.rs:582` (potential_env_vars)
- `crates/goose/src/providers/snowflake.rs:69` (potential_env_vars)

**Example context:**
```rust
test_provider(
        "Snowflake",
        &["SNOWFLAKE_HOST", "SNOWFLAKE_TOKEN"],
        None,
        snowflake::SnowflakeProvider::default,
```

### `TEMP`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:97` (potential_env_vars)
- `crates/goose-mcp/src/computercontroller/platform/windows.rs:24` (env_var_std)
- `crates/goose-mcp/src/computercontroller/platform/windows.rs:24` (potential_env_vars)

**Example context:**
```rust
"SESSIONNAME",  // Affects Windows session configuration
        "ComSpec",      // Determines default command interpreter (can replace `cmd.exe`)
        "TEMP",
        "TMP",          // Redirects temporary file storage (useful for injection attacks)
        "LOCALAPPDATA", // Controls application data paths (can be abused for persistence)
```

### `TEST_KEY`

**Usage locations:**
- `crates/goose/src/config/base.rs:814` (potential_env_vars)

**Example context:**
```rust
// Test with environment variable override
        std::env::set_var("TEST_KEY", "env_value");
        let value: String = config.get_param("test_key")?;
        assert_eq!(value, "env_value");
```

### `TEST_PRECEDENCE`

**Usage locations:**
- `crates/goose/src/config/base.rs:1455` (potential_env_vars)

**Example context:**
```rust
// Set environment variable
        std::env::set_var("TEST_PRECEDENCE", "env_value");

        // Environment variable should take precedence
```

### `TMP`

**Usage locations:**
- `crates/goose/src/agents/extension.rs:98` (potential_env_vars)

**Example context:**
```rust
"ComSpec",      // Determines default command interpreter (can replace `cmd.exe`)
        "TEMP",
        "TMP",          // Redirects temporary file storage (useful for injection attacks)
        "LOCALAPPDATA", // Controls application data paths (can be abused for persistence)
        "USERPROFILE",  // Windows user directory (can affect profile-based execution paths)
```

### `USER`

**Usage locations:**
- `crates/goose/src/agents/agent.rs:1360` (env_var_std)
- `crates/goose/src/agents/agent.rs:1360` (potential_env_vars)

**Example context:**
```rust
let author = Author {
            contact: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .ok(),
```

### `USERNAME`

**Usage locations:**
- `crates/goose/src/agents/agent.rs:1361` (env_var_std)

**Example context:**
```rust
let author = Author {
            contact: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .ok(),
            metadata: None,
```

### `USERPROFILE`

**Usage locations:**
- `crates/goose-mcp/src/developer/shell.rs:79` (env_var_std)
- `crates/goose-mcp/src/developer/shell.rs:79` (env_var_unwrap)

**Example context:**
```rust
let with_userprofile = path_str.replace(
            "%USERPROFILE%",
            &env::var("USERPROFILE").unwrap_or_default(),
        );
        // Add more Windows environment variables as needed
```

### `USER_ENTERED`

**Usage locations:**
- `crates/goose-mcp/src/google_drive/mod.rs:547` (potential_env_vars)

**Example context:**
```rust
"valueInputOption": {
                      "type": "string",
                      "enum": ["RAW", "USER_ENTERED"],
                      "description": "How input data should be interpreted (default: USER_ENTERED)",
                  }
```

### `VENICE_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:88` (potential_env_vars)

**Example context:**
```rust
pub fn from_env(mut model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
```

### `VENICE_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:93` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
            .get_param("VENICE_BASE_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
```

### `VENICE_HOST`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:90` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
            .unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
```

### `VENICE_MODELS_PATH`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:96` (potential_env_vars)

**Example context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
            .get_param("VENICE_MODELS_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_MODELS_PATH.to_string());
```

### `WAYLAND_DISPLAY`

**Usage locations:**
- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44` (env_var_std)
- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44` (potential_env_vars)

**Example context:**
```rust
fn detect_display_server() -> DisplayServer {
        if let Ok(wayland_display) = std::env::var("WAYLAND_DISPLAY") {
            if !wayland_display.is_empty() {
                return DisplayServer::Wayland;
```

### `XAI_API_KEY`

**Usage locations:**
- `crates/goose/tests/agent.rs:51` (potential_env_vars)
- `crates/goose/tests/providers.rs:624` (potential_env_vars)
- `crates/goose/src/providers/xai.rs:51` (potential_env_vars)

**Example context:**
```rust
ProviderType::OpenRouter => &["OPENROUTER_API_KEY"],
            ProviderType::GcpVertexAI => &["GCP_PROJECT_ID", "GCP_LOCATION"],
            ProviderType::Xai => &["XAI_API_KEY"],
        }
    }
```

### `XAI_HOST`

**Usage locations:**
- `crates/goose/src/providers/xai.rs:53` (potential_env_vars)

**Example context:**
```rust
let api_key: String = config.get_secret("XAI_API_KEY")?;
        let host: String = config
            .get_param("XAI_HOST")
            .unwrap_or_else(|_| XAI_API_HOST.to_string());
```

## Config File Parameters (99 items)

### `ANTHROPIC_HOST`

**Usage locations:**
- `crates/goose/src/providers/anthropic.rs:54` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get_param("ANTHROPIC_HOST")
            .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
```

### `AZURE_OPENAI_API_VERSION`

**Usage locations:**
- `crates/goose/src/providers/azure.rs:79` (config_param_get)

**Example context:**
```rust
let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| AZURE_DEFAULT_API_VERSION.to_string());
```

### `AZURE_OPENAI_DEPLOYMENT_NAME`

**Usage locations:**
- `crates/goose/src/providers/azure.rs:77` (config_param_get)

**Example context:**
```rust
let config = crate::config::Config::global();
        let endpoint: String = config.get_param("AZURE_OPENAI_ENDPOINT")?;
        let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
```

### `AZURE_OPENAI_ENDPOINT`

**Usage locations:**
- `crates/goose/src/providers/azure.rs:76` (config_param_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let endpoint: String = config.get_param("AZURE_OPENAI_ENDPOINT")?;
        let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
```

### `CLAUDE_CODE_COMMAND`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:36` (config_param_get)

**Example context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("CLAUDE_CODE_COMMAND")
            .unwrap_or_else(|_| "claude".to_string());
```

### `DATABRICKS_BACKOFF_MULTIPLIER`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:163` (config_param_get)

**Example context:**
```rust
let backoff_multiplier = config
            .get_param("DATABRICKS_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `DATABRICKS_HOST`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:113` (config_param_get)

**Example context:**
```rust
let config = crate::config::Config::global();

        let mut host: Result<String, ConfigError> = config.get_param("DATABRICKS_HOST");
        if host.is_err() {
            host = config.get_secret("DATABRICKS_HOST")
```

### `DATABRICKS_INITIAL_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:157` (config_param_get)

**Example context:**
```rust
let initial_interval_ms = config
            .get_param("DATABRICKS_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `DATABRICKS_MAX_RETRIES`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:151` (config_param_get)

**Example context:**
```rust
fn load_retry_config(config: &crate::config::Config) -> RetryConfig {
        let max_retries = config
            .get_param("DATABRICKS_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `DATABRICKS_MAX_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:169` (config_param_get)

**Example context:**
```rust
let max_interval_ms = config
            .get_param("DATABRICKS_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_BACKOFF_MULTIPLIER`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:148` (config_param_get)

**Example context:**
```rust
let backoff_multiplier = config
            .get_param("GCP_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `GCP_INITIAL_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:142` (config_param_get)

**Example context:**
```rust
let initial_interval_ms = config
            .get_param("GCP_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_LOCATION`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:174` (config_param_get)

**Example context:**
```rust
fn determine_location(config: &crate::config::Config) -> Result<String> {
        Ok(config
            .get_param("GCP_LOCATION")
            .ok()
            .filter(|location: &String| !location.trim().is_empty())
```

### `GCP_MAX_RETRIES`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:136` (config_param_get)

**Example context:**
```rust
// Load max retries for 429 rate limit errors
        let max_retries = config
            .get_param("GCP_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `GCP_MAX_RETRY_INTERVAL_MS`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:154` (config_param_get)

**Example context:**
```rust
let max_interval_ms = config
            .get_param("GCP_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_PROJECT_ID`

**Usage locations:**
- `crates/goose/src/providers/gcpvertexai.rs:108` (config_param_get)

**Example context:**
```rust
async fn new_async(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let project_id = config.get_param("GCP_PROJECT_ID")?;
        let location = Self::determine_location(config)?;
        let host = format!("https://{}-aiplatform.googleapis.com", location);
```

### `GEMINI_CLI_COMMAND`

**Usage locations:**
- `crates/goose/src/providers/gemini_cli.rs:35` (config_param_get)

**Example context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("GEMINI_CLI_COMMAND")
            .unwrap_or_else(|_| "gemini".to_string());
```

### `GOOGLE_HOST`

**Usage locations:**
- `crates/goose/src/providers/google.rs:61` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get_param("GOOGLE_HOST")
            .unwrap_or_else(|_| GOOGLE_API_HOST.to_string());
```

### `GOOSE_AUTO_COMPACT_THRESHOLD`

**Usage locations:**
- `crates/goose/src/context_mgmt/auto_compact.rs:546` (config_param_set)

**Example context:**
```rust
let config = Config::global();
        config
            .set_param("GOOSE_AUTO_COMPACT_THRESHOLD", serde_json::Value::from(0.1))
            .unwrap();
```

### `GOOSE_CLI_MIN_PRIORITY`

**Usage locations:**
- `crates/goose-cli/src/commands/configure.rs:1227` (config_param_set)

**Example context:**
```rust
match tool_log_level {
        "high" => {
            config.set_param("GOOSE_CLI_MIN_PRIORITY", Value::from(0.8))?;
            cliclack::outro("Showing tool output of high importance only.")?;
        }
```

### `GOOSE_CLI_THEME`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:82` (config_param_set)

**Example context:**
```rust
};

    if let Err(e) = config.set_param("GOOSE_CLI_THEME", Value::String(theme_str.to_string())) {
        eprintln!("Failed to save theme setting to config: {}", e);
    }
```

### `GOOSE_MAX_TURNS`

**Usage locations:**
- `crates/goose/src/agents/agent.rs:884` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:1522` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:1541` (config_param_set)

**Example context:**
```rust
.and_then(|s| s.max_turns)
                .unwrap_or_else(|| {
                    config.get_param("GOOSE_MAX_TURNS").unwrap_or(DEFAULT_MAX_TURNS)
                });
```

### `GOOSE_MODE`

**Usage locations:**
- `crates/goose/src/providers/claude_code.rs:538` (config_param_get)
- `crates/goose/src/providers/ollama.rs:134` (config_param_get)
- `crates/goose/src/agents/prompt_manager.rs:141` (config_param_get)
- `crates/goose/src/agents/agent.rs:1150` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:1150` (config_param_set)
- `crates/goose-cli/src/session/mod.rs:814` (config_param_get)
- `crates/goose-cli/src/session/mod.rs:621` (config_param_set)
- `crates/goose-server/src/routes/agent.rs:184` (config_param_get)

**Example context:**
```rust
let config = Config::global();
        let goose_mode: String = config.get_param("GOOSE_MODE").unwrap();
        assert_eq!(goose_mode, "auto");
```

### `GOOSE_MODEL`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1135` (config_param_get)
- `crates/goose/src/config/signup_openrouter/mod.rs:170` (config_param_set)
- `crates/goose-cli/src/commands/configure.rs:1578` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:503` (config_param_set)
- `crates/goose-cli/src/commands/web.rs:95` (config_param_get)
- `crates/goose-cli/src/session/builder.rs:198` (config_param_get)
- `crates/goose-server/src/routes/agent.rs:249` (config_param_get)

**Example context:**
```rust
};
        let model_name: String =
            match global_config.get_param("GOOSE_MODEL") {
                Ok(name) => name,
                Err(_) => return Err(JobExecutionError {
```

### `GOOSE_PROVIDER`

**Usage locations:**
- `crates/goose/src/scheduler.rs:1125` (config_param_get)
- `crates/goose/src/agents/agent.rs:1370` (config_param_get)
- `crates/goose/src/config/signup_openrouter/mod.rs:169` (config_param_set)
- `crates/goose-cli/src/commands/configure.rs:297` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:502` (config_param_set)
- `crates/goose-cli/src/commands/web.rs:87` (config_param_get)
- `crates/goose-cli/src/session/builder.rs:187` (config_param_get)

**Example context:**
```rust
} else {
        let global_config = Config::global();
        let provider_name: String = match global_config.get_param("GOOSE_PROVIDER") {
            Ok(name) => name,
            Err(_) => return Err(JobExecutionError {
```

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`

**Usage locations:**
- `crates/goose/src/agents/tool_route_manager.rs:78` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:1193` (config_param_set)

**Example context:**
```rust
let config = Config::global();
        let router_tool_selection_strategy = config
            .get_param("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
            .unwrap_or_else(|_| "default".to_string());
```

### `GOOSE_SCHEDULER_TYPE`

**Usage locations:**
- `crates/goose-cli/src/commands/configure.rs:1475` (config_param_get)
- `crates/goose-cli/src/commands/configure.rs:1492` (config_param_set)

**Example context:**
```rust
// Get current scheduler type from config for display
    let current_scheduler: String = config
        .get_param("GOOSE_SCHEDULER_TYPE")
        .unwrap_or_else(|_| "legacy".to_string());
```

### `GOOSE_SYSTEM_PROMPT_FILE_PATH`

**Usage locations:**
- `crates/goose-cli/src/session/builder.rs:563` (config_param_get)

**Example context:**
```rust
// Only override system prompt if a system override exists
    let system_prompt_file: Option<String> = config.get_param("GOOSE_SYSTEM_PROMPT_FILE_PATH").ok();
    if let Some(ref path) = system_prompt_file {
        let override_prompt =
```

### `GROQ_HOST`

**Usage locations:**
- `crates/goose/src/providers/groq.rs:40` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("GROQ_API_KEY")?;
        let host: String = config
            .get_param("GROQ_HOST")
            .unwrap_or_else(|_| GROQ_API_HOST.to_string());
```

### `LITELLM_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:40` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| "https://api.litellm.ai".to_string());
        let base_path: String = config
            .get_param("LITELLM_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
```

### `LITELLM_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:44` (config_param_get)

**Example context:**
```rust
let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("LITELLM_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("LITELLM_CUSTOM_HEADERS"))
            .ok()
            .map(parse_custom_headers);
```

### `LITELLM_HOST`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:37` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| String::new());
        let host: String = config
            .get_param("LITELLM_HOST")
            .unwrap_or_else(|_| "https://api.litellm.ai".to_string());
        let base_path: String = config
```

### `LITELLM_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:47` (config_param_get)

**Example context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("LITELLM_TIMEOUT").unwrap_or(600);

        let auth = if api_key.is_empty() {
```

### `OLLAMA_HOST`

**Usage locations:**
- `crates/goose/src/providers/toolshim.rs:88` (config_param_get)
- `crates/goose/src/providers/ollama.rs:41` (config_param_get)

**Example context:**
```rust
let config = crate::config::Config::global();
        let host: String = config
            .get_param("OLLAMA_HOST")
            .unwrap_or_else(|_| OLLAMA_HOST.to_string());
```

### `OLLAMA_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/ollama.rs:45` (config_param_get)

**Example context:**
```rust
let timeout: Duration =
            Duration::from_secs(config.get_param("OLLAMA_TIMEOUT").unwrap_or(OLLAMA_TIMEOUT));

        // OLLAMA_HOST is sometimes just the 'host' or 'host:port' without a scheme
```

### `OPENAI_API_KEY`

**Usage locations:**
- `crates/goose/src/config/base.rs:88` (config_param_get)

**Example context:**
```rust
/// // Get a string value
/// let config = Config::global();
/// let api_key: String = config.get_param("OPENAI_API_KEY").unwrap();
///
/// // Get a complex type
```

### `OPENAI_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:66` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
            .get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
```

### `OPENAI_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:72` (config_param_get)

**Example context:**
```rust
let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("OPENAI_CUSTOM_HEADERS"))
            .ok()
            .map(parse_custom_headers);
```

### `OPENAI_HOST`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:63` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("OPENAI_API_KEY")?;
        let host: String = config
            .get_param("OPENAI_HOST")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
```

### `OPENAI_ORGANIZATION`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:68` (config_param_get)

**Example context:**
```rust
.get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
```

### `OPENAI_PROJECT`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:69` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
```

### `OPENAI_TIMEOUT`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:75` (config_param_get)

**Example context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("OPENAI_TIMEOUT").unwrap_or(600);

        let auth = AuthMethod::BearerToken(api_key);
```

### `OPENROUTER_HOST`

**Usage locations:**
- `crates/goose/src/providers/openrouter.rs:48` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("OPENROUTER_API_KEY")?;
        let host: String = config
            .get_param("OPENROUTER_HOST")
            .unwrap_or_else(|_| "https://openrouter.ai".to_string());
```

### `RANDOM_THINKING_MESSAGES`

**Usage locations:**
- `crates/goose-cli/src/session/output.rs:101` (config_param_get)

**Example context:**
```rust
let spinner = cliclack::spinner();
        if Config::global()
            .get_param("RANDOM_THINKING_MESSAGES")
            .unwrap_or(true)
        {
```

### `SAGEMAKER_ENDPOINT_NAME`

**Usage locations:**
- `crates/goose/src/providers/sagemaker_tgi.rs:40` (config_param_get)

**Example context:**
```rust
// Get SageMaker endpoint name (just the name, not full URL)
        let endpoint_name: String = config.get_param("SAGEMAKER_ENDPOINT_NAME").map_err(|_| {
            anyhow::anyhow!("SAGEMAKER_ENDPOINT_NAME is required for SageMaker TGI provider")
        })?;
```

### `SNOWFLAKE_HOST`

**Usage locations:**
- `crates/goose/src/providers/snowflake.rs:48` (config_param_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let mut host: Result<String, ConfigError> = config.get_param("SNOWFLAKE_HOST");
        if host.is_err() {
            host = config.get_secret("SNOWFLAKE_HOST")
```

### `SNOWFLAKE_TOKEN`

**Usage locations:**
- `crates/goose/src/providers/snowflake.rs:69` (config_param_get)

**Example context:**
```rust
}

        let mut token: Result<String, ConfigError> = config.get_param("SNOWFLAKE_TOKEN");

        if token.is_err() {
```

### `VENICE_BASE_PATH`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:93` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
            .get_param("VENICE_BASE_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
```

### `VENICE_HOST`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:90` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
            .unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
```

### `VENICE_MODELS_PATH`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:96` (config_param_get)

**Example context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
            .get_param("VENICE_MODELS_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_MODELS_PATH.to_string());
```

### `XAI_HOST`

**Usage locations:**
- `crates/goose/src/providers/xai.rs:53` (config_param_get)

**Example context:**
```rust
let api_key: String = config.get_secret("XAI_API_KEY")?;
        let host: String = config
            .get_param("XAI_HOST")
            .unwrap_or_else(|_| XAI_API_HOST.to_string());
```

### `another_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:1149` (config_param_set)

**Example context:**
```rust
// First, create a config with some data
        config.set_param("test_key_backup", Value::String("backup_value".to_string()))?;
        config.set_param("another_key", Value::Number(42.into()))?;

        // Verify the backup was created
```

### `complex_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:841` (config_param_get)
- `crates/goose/src/config/base.rs:833` (config_param_set)

**Example context:**
```rust
)?;

        let value: TestStruct = config.get_param("complex_key")?;
        assert_eq!(value.field1, "hello");
        assert_eq!(value.field2, 42);
```

### `config`

**Usage locations:**
- `crates/goose/src/config/base.rs:1429` (config_param_get)

**Example context:**
```rust
level: i32,
        }
        let value: TestConfig = config.get_param("config")?;
        assert_eq!(value.debug, true);
        assert_eq!(value.level, 5);
```

### `enabled`

**Usage locations:**
- `crates/goose/src/config/base.rs:1419` (config_param_get)

**Example context:**
```rust
// Test boolean environment variable
        std::env::set_var("ENABLED", "true");
        let value: bool = config.get_param("enabled")?;
        assert_eq!(value, true);
```

### `experiments`

**Usage locations:**
- `crates/goose/src/config/experiments.rs:23` (config_param_get)
- `crates/goose/src/config/experiments.rs:38` (config_param_set)

**Example context:**
```rust
let config = Config::global();
        let mut experiments: HashMap<String, bool> =
            config.get_param("experiments").unwrap_or_default();
        Self::refresh_experiments(&mut experiments);
```

### `extensions`

**Usage locations:**
- `crates/goose/src/config/extensions.rs:36` (config_param_get)
- `crates/goose/src/config/extensions.rs:53` (config_param_set)

**Example context:**
```rust
// Try to get the extension entry
        let extensions: HashMap<String, ExtensionEntry> = match config.get_param("extensions") {
            Ok(exts) => exts,
            Err(super::ConfigError::NotFound(_)) => {
```

### `key`

**Usage locations:**
- `crates/goose/src/config/base.rs:880` (config_param_get)
- `crates/goose/src/config/base.rs:878` (config_param_set)

**Example context:**
```rust
config.set_param("key", Value::String("value".to_string()))?;

        let value: String = config.get_param("key")?;
        assert_eq!(value, "value");
```

### `key1`

**Usage locations:**
- `crates/goose/src/config/base.rs:862` (config_param_set)

**Example context:**
```rust
let config = Config::new(temp_file.path(), TEST_KEYRING_SERVICE)?;

        config.set_param("key1", Value::String("value1".to_string()))?;
        config.set_param("key2", Value::Number(42.into()))?;
```

### `key2`

**Usage locations:**
- `crates/goose/src/config/base.rs:863` (config_param_set)

**Example context:**
```rust
config.set_param("key1", Value::String("value1".to_string()))?;
        config.set_param("key2", Value::Number(42.into()))?;

        // Read the file directly to check YAML formatting
```

### `nonexistent_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:853` (config_param_get)

**Example context:**
```rust
let config = Config::new(temp_file.path(), TEST_KEYRING_SERVICE).unwrap();

        let result: Result<String, ConfigError> = config.get_param("nonexistent_key");
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }
```

### `port`

**Usage locations:**
- `crates/goose/src/config/base.rs:1414` (config_param_get)

**Example context:**
```rust
// Test number environment variable
        std::env::set_var("PORT", "8080");
        let value: i32 = config.get_param("port")?;
        assert_eq!(value, 8080);
```

### `provider`

**Usage locations:**
- `crates/goose/src/config/base.rs:1409` (config_param_get)

**Example context:**
```rust
// Test string environment variable (the original issue case)
        std::env::set_var("PROVIDER", "ANTHROPIC");
        let value: String = config.get_param("provider")?;
        assert_eq!(value, "ANTHROPIC");
```

### `server`

**Usage locations:**
- `crates/goose/src/config/base.rs:97` (config_param_get)

**Example context:**
```rust
/// }
///
/// let server_config: ServerConfig = config.get_param("server").unwrap();
/// ```
///
```

### `test_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:810` (config_param_get)
- `crates/goose/src/config/base.rs:807` (config_param_set)

**Example context:**
```rust
// Test simple string retrieval
        let value: String = config.get_param("test_key")?;
        assert_eq!(value, "test_value");
```

### `test_key_backup`

**Usage locations:**
- `crates/goose/src/config/base.rs:1148` (config_param_set)

**Example context:**
```rust
// First, create a config with some data
        config.set_param("test_key_backup", Value::String("backup_value".to_string()))?;
        config.set_param("another_key", Value::Number(42.into()))?;
```

### `test_precedence`

**Usage locations:**
- `crates/goose/src/config/base.rs:1451` (config_param_get)
- `crates/goose/src/config/base.rs:1448` (config_param_set)

**Example context:**
```rust
// Verify file value is returned when no env var
        let value: String = config.get_param("test_precedence")?;
        assert_eq!(value, "file_value");
```

### `third_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:1156` (config_param_set)

**Example context:**
```rust
// Make sure we have a backup by doing another write
        config.set_param("third_key", Value::Bool(true))?;
        assert!(primary_backup.exists(), "Backup should exist after writes");
```

### `version`

**Usage locations:**
- `crates/goose/src/config/base.rs:1213` (config_param_set)

**Example context:**
```rust
// Create multiple versions to test rotation
        for i in 1..=7 {
            config.set_param("version", Value::Number(i.into()))?;
        }
```

## Secret Storage (30 items)

### `ANTHROPIC_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/anthropic.rs:52` (secret_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get_param("ANTHROPIC_HOST")
```

### `AZURE_OPENAI_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/azure.rs:83` (secret_get)

**Example context:**
```rust
let api_key = config
            .get_secret("AZURE_OPENAI_API_KEY")
            .ok()
            .filter(|key: &String| !key.is_empty());
```

### `DATABRICKS_HOST`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:115` (secret_get)

**Example context:**
```rust
let mut host: Result<String, ConfigError> = config.get_param("DATABRICKS_HOST");
        if host.is_err() {
            host = config.get_secret("DATABRICKS_HOST")
        }
```

### `DATABRICKS_TOKEN`

**Usage locations:**
- `crates/goose/src/providers/databricks.rs:128` (secret_get)

**Example context:**
```rust
let retry_config = Self::load_retry_config(config);

        let auth = if let Ok(api_key) = config.get_secret("DATABRICKS_TOKEN") {
            DatabricksAuth::token(api_key)
        } else {
```

### `ELEVENLABS_API_KEY`

**Usage locations:**
- `crates/goose-server/src/routes/audio.rs:212` (secret_get)

**Example context:**
```rust
// First try to get it as a secret
    let api_key: String = match config.get_secret("ELEVENLABS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
```

### `GITHUB_COPILOT_TOKEN`

**Usage locations:**
- `crates/goose/src/providers/githubcopilot.rs:239` (secret_set)

**Example context:**
```rust
.await
                        .context("unable to login into github")?;
                    config.set_secret("GITHUB_COPILOT_TOKEN", Value::String(token.clone()))?;
                    token
                }
```

### `GOOGLE_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/google.rs:59` (secret_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get_param("GOOGLE_HOST")
```

### `GROQ_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/groq.rs:38` (secret_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("GROQ_API_KEY")?;
        let host: String = config
            .get_param("GROQ_HOST")
```

### `LITELLM_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:34` (secret_get)

**Example context:**
```rust
let config = crate::config::Config::global();
        let api_key: String = config
            .get_secret("LITELLM_API_KEY")
            .unwrap_or_else(|_| String::new());
        let host: String = config
```

### `LITELLM_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/litellm.rs:43` (secret_get)

**Example context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("LITELLM_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("LITELLM_CUSTOM_HEADERS"))
            .ok()
```

### `OPENAI_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:61` (secret_get)
- `crates/goose-server/src/routes/audio.rs:100` (secret_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("OPENAI_API_KEY")?;
        let host: String = config
            .get_param("OPENAI_HOST")
```

### `OPENAI_CUSTOM_HEADERS`

**Usage locations:**
- `crates/goose/src/providers/openai.rs:71` (secret_get)

**Example context:**
```rust
let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("OPENAI_CUSTOM_HEADERS"))
            .ok()
```

### `OPENROUTER_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/openrouter.rs:46` (secret_get)
- `crates/goose/src/config/signup_openrouter/mod.rs:168` (secret_set)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("OPENROUTER_API_KEY")?;
        let host: String = config
            .get_param("OPENROUTER_HOST")
```

### `SNOWFLAKE_HOST`

**Usage locations:**
- `crates/goose/src/providers/snowflake.rs:50` (secret_get)

**Example context:**
```rust
let mut host: Result<String, ConfigError> = config.get_param("SNOWFLAKE_HOST");
        if host.is_err() {
            host = config.get_secret("SNOWFLAKE_HOST")
        }
        if host.is_err() {
```

### `SNOWFLAKE_TOKEN`

**Usage locations:**
- `crates/goose/src/providers/snowflake.rs:72` (secret_get)

**Example context:**
```rust
if token.is_err() {
            token = config.get_secret("SNOWFLAKE_TOKEN")
        }
```

### `VENICE_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/venice.rs:88` (secret_get)

**Example context:**
```rust
pub fn from_env(mut model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
```

### `XAI_API_KEY`

**Usage locations:**
- `crates/goose/src/providers/xai.rs:51` (secret_get)

**Example context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("XAI_API_KEY")?;
        let host: String = config
            .get_param("XAI_HOST")
```

### `api_key`

**Usage locations:**
- `crates/goose/src/config/base.rs:919` (secret_get)
- `crates/goose/src/config/base.rs:918` (secret_set)
- `crates/goose/src/config/base.rs:929` (secret_delete)

**Example context:**
```rust
// Test setting and getting a simple secret
        config.set_secret("api_key", Value::String("secret123".to_string()))?;
        let value: String = config.get_secret("api_key")?;
        assert_eq!(value, "secret123");
```

### `key`

**Usage locations:**
- `crates/goose/src/config/base.rs:899` (secret_get)
- `crates/goose/src/config/base.rs:897` (secret_set)
- `crates/goose/src/config/base.rs:902` (secret_delete)

**Example context:**
```rust
config.set_secret("key", Value::String("value".to_string()))?;

        let value: String = config.get_secret("key")?;
        assert_eq!(value, "value");
```

### `key1`

**Usage locations:**
- `crates/goose/src/config/base.rs:949` (secret_get)
- `crates/goose/src/config/base.rs:945` (secret_set)
- `crates/goose/src/config/base.rs:955` (secret_delete)

**Example context:**
```rust
// Verify both exist
        let value1: String = config.get_secret("key1")?;
        let value2: String = config.get_secret("key2")?;
        assert_eq!(value1, "secret1");
```

### `key2`

**Usage locations:**
- `crates/goose/src/config/base.rs:950` (secret_get)
- `crates/goose/src/config/base.rs:946` (secret_set)

**Example context:**
```rust
// Verify both exist
        let value1: String = config.get_secret("key1")?;
        let value2: String = config.get_secret("key2")?;
        assert_eq!(value1, "secret1");
        assert_eq!(value2, "secret2");
```

## CLI Flags (38 items)

### `--explain`

**Description:** Show the recipe title, description, and parameters

**Usage locations:**
- `crates/goose-cli/src/cli.rs:469` (clap_long)

**Example context:**
```rust
no_session: bool,

        /// Show the recipe title, description, and parameters
        #[arg(
            long = "explain",
            help = "Show the recipe title, description, and parameters"
        )]
```

### `--interactive`

**Description:** Continue in interactive mode after processing initial input

**Usage locations:**
- `crates/goose-cli/src/cli.rs:452` (clap_long)

**Example context:**
```rust
params: Vec<(String, String)>,

        /// Continue in interactive mode after processing input
        #[arg(
            short = 's',
            long = "interactive",
            help = "Continue in interactive mode after processing initial input"
```

### `--max-tool-repetitions`

**Description:** Maximum number of consecutive identical tool calls allowed

**Usage locations:**
- `crates/goose-cli/src/cli.rs:327` (clap_long)

**Example context:**
```rust
debug: bool,

        /// Maximum number of consecutive identical tool calls allowed
        #[arg(
            long = "max-tool-repetitions",
            value_name = "NUMBER",
            help = "Maximum number of consecutive identical tool calls allowed",
```

### `--no-session`

**Description:** Run without storing a session file

**Usage locations:**
- `crates/goose-cli/src/cli.rs:460` (clap_long)

**Example context:**
```rust
interactive: bool,

        /// Run without storing a session file
        #[arg(
            long = "no-session",
            help = "Run without storing a session file",
            long_help = "Execute commands without creating or using a session file. Useful for automated runs.",
```

### `--quiet`

**Description:** Quiet mode. Suppress non-response output, printing only the model response to stdout

**Usage locations:**
- `crates/goose-cli/src/cli.rs:563` (clap_long)

**Example context:**
```rust
builtins: Vec<String>,

        /// Quiet mode - suppress non-response output
        #[arg(
            short = 'q',
            long = "quiet",
            help = "Quiet mode. Suppress non-response output, printing only the model response to stdout"
```

### `--render-recipe`

**Description:** Print the rendered recipe instead of running it.

**Usage locations:**
- `crates/goose-cli/src/cli.rs:476` (clap_long)

**Example context:**
```rust
explain: bool,

        /// Print the rendered recipe instead of running it
        #[arg(
            long = "render-recipe",
            help = "Print the rendered recipe instead of running it."
        )]
```

### `--system`

**Description:** Additional system prompt to customize agent behavior

**Usage locations:**
- `crates/goose-cli/src/cli.rs:420` (clap_long)

**Example context:**
```rust
input_text: Option<String>,

        /// Additional system prompt to customize agent behavior
        #[arg(
            long = "system",
            value_name = "TEXT",
            help = "Additional system prompt to customize agent behavior",
```

### `--text`

**Description:** Input text to provide to Goose directly

**Usage locations:**
- `crates/goose-cli/src/cli.rs:408` (clap_long)

**Example context:**
```rust
instructions: Option<String>,

        /// Input text containing commands
        #[arg(
            short = 't',
            long = "text",
            value_name = "TEXT",
```

### `-q`

**Usage locations:**
- `crates/goose-cli/src/cli.rs:563` (clap_short)

**Example context:**
```rust
builtins: Vec<String>,

        /// Quiet mode - suppress non-response output
        #[arg(
            short = 'q',
            long = "quiet",
            help = "Quiet mode. Suppress non-response output, printing only the model response to stdout"
```

### `-s`

**Usage locations:**
- `crates/goose-cli/src/cli.rs:452` (clap_short)

**Example context:**
```rust
params: Vec<(String, String)>,

        /// Continue in interactive mode after processing input
        #[arg(
            short = 's',
            long = "interactive",
            help = "Continue in interactive mode after processing initial input"
```

### `-t`

**Usage locations:**
- `crates/goose-cli/src/cli.rs:408` (clap_short)

**Example context:**
```rust
instructions: Option<String>,

        /// Input text containing commands
        #[arg(
            short = 't',
            long = "text",
            value_name = "TEXT",
```

### `Add`

**Description:** Add a new scheduled job

**Usage locations:**
- `crates/goose-cli/src/cli.rs:124` (clap_command)

**Example context:**
```rust
#[derive(Subcommand, Debug)]
enum SchedulerCommand {
    #[command(about = "Add a new scheduled job")]
    Add {
        #[arg(long, help = "Unique ID for the job")]
        id: String,
```

### `Bench`

**Description:** Evaluate system configuration across a range of practical tasks

**Usage locations:**
- `crates/goose-cli/src/cli.rs:641` (clap_command)

**Example context:**
```rust
},

    /// Evaluate system configuration across a range of practical tasks
    #[command(about = "Evaluate system configuration across a range of practical tasks")]
    Bench {
        #[command(subcommand)]
        cmd: BenchCommand,
```

### `Configure`

**Description:** Configure Goose settings

**Usage locations:**
- `crates/goose-cli/src/cli.rs:274` (clap_command)

**Example context:**
```rust
#[derive(Subcommand)]
enum Command {
    /// Configure Goose settings
    #[command(about = "Configure Goose settings")]
    Configure {},

    /// Display Goose configuration information
```

### `CronHelp`

**Description:** Show cron expression examples and help

**Usage locations:**
- `crates/goose-cli/src/cli.rs:171` (clap_command)

**Example context:**
```rust
#[command(about = "Stop Temporal services")]
    ServicesStop {},
    /// Show cron expression examples and help
    #[command(about = "Show cron expression examples and help")]
    CronHelp {},
}
```

### `Deeplink`

**Description:** Generate a deeplink for a recipe

**Usage locations:**
- `crates/goose-cli/src/cli.rs:240` (clap_command)

**Example context:**
```rust
},

    /// Generate a deeplink for a recipe file
    #[command(about = "Generate a deeplink for a recipe")]
    Deeplink {
        /// Recipe name to get recipe file to generate deeplink
        #[arg(
```

### `EvalModel`

**Description:** Run an eval of model

**Usage locations:**
- `crates/goose-cli/src/cli.rs:203` (clap_command)

**Example context:**
```rust
config: Option<PathBuf>,
    },

    #[command(name = "eval-model", about = "Run an eval of model")]
    EvalModel {
        #[arg(short, long, help = "A serialized config file for the model only.")]
        config: String,
```

### `ExecEval`

**Description:** run a single eval

**Usage locations:**
- `crates/goose-cli/src/cli.rs:209` (clap_command)

**Example context:**
```rust
config: String,
    },

    #[command(name = "exec-eval", about = "run a single eval")]
    ExecEval {
        #[arg(short, long, help = "A serialized config file for the eval only.")]
        config: String,
```

### `Export`

**Description:** Export a session to Markdown format

**Usage locations:**
- `crates/goose-cli/src/cli.rs:107` (clap_command)

**Example context:**
```rust
#[arg(short, long, help = "Regex for removing matched sessions (optional)")]
        regex: Option<String>,
    },
    #[command(about = "Export a session to Markdown format")]
    Export {
        #[command(flatten)]
        identifier: Option<Identifier>,
```

### `GenerateLeaderboard`

**Description:** Generate a leaderboard CSV from benchmark results

**Usage locations:**
- `crates/goose-cli/src/cli.rs:215` (clap_command)

**Example context:**
```rust
config: String,
    },

    #[command(
        name = "generate-leaderboard",
        about = "Generate a leaderboard CSV from benchmark results"
    )]
```

### `Info`

**Description:** Display Goose information

**Usage locations:**
- `crates/goose-cli/src/cli.rs:278` (clap_command)

**Example context:**
```rust
Configure {},

    /// Display Goose configuration information
    #[command(about = "Display Goose information")]
    Info {
        /// Show verbose information including current configuration
        #[arg(short, long, help = "Show verbose information including config.yaml")]
```

### `InitConfig`

**Description:** Create a new starter-config

**Usage locations:**
- `crates/goose-cli/src/cli.rs:177` (clap_command)

**Example context:**
```rust
#[derive(Subcommand)]
pub enum BenchCommand {
    #[command(name = "init-config", about = "Create a new starter-config")]
    InitConfig {
        #[arg(short, long, help = "filename with extension for generated config")]
        name: String,
```

### `List`

**Description:** List all available sessions

**Usage locations:**
- `crates/goose-cli/src/cli.rs:80` (clap_command)

**Example context:**
```rust
#[derive(Subcommand)]
enum SessionCommand {
    #[command(about = "List all available sessions")]
    List {
        #[arg(short, long, help = "List all available sessions")]
        verbose: bool,
```

### `Mcp`

**Description:** Run one of the mcp servers bundled with goose

**Usage locations:**
- `crates/goose-cli/src/cli.rs:286` (clap_command)

**Example context:**
```rust
},

    /// Manage system prompts and behaviors
    #[command(about = "Run one of the mcp servers bundled with goose")]
    Mcp { name: String },

    /// Start or resume interactive chat sessions
```

### `Project`

**Description:** Open the last project directory

**Usage locations:**
- `crates/goose-cli/src/cli.rs:386` (clap_command)

**Example context:**
```rust
},

    /// Open the last project directory
    #[command(about = "Open the last project directory", visible_alias = "p")]
    Project {},

    /// List recent project directories
```

### `Recipe`

**Description:** Recipe utilities for validation and deeplinking

**Usage locations:**
- `crates/goose-cli/src/cli.rs:610` (clap_command)

**Example context:**
```rust
},

    /// Recipe utilities for validation and deeplinking
    #[command(about = "Recipe utilities for validation and deeplinking")]
    Recipe {
        #[command(subcommand)]
        command: RecipeCommand,
```

### `Remove`

**Description:** Remove sessions. Runs interactively if no ID or regex is provided.

**Usage locations:**
- `crates/goose-cli/src/cli.rs:100` (clap_command)

**Example context:**
```rust
)]
        ascending: bool,
    },
    #[command(about = "Remove sessions. Runs interactively if no ID or regex is provided.")]
    Remove {
        #[arg(short, long, help = "Session ID to be removed (optional)")]
        id: Option<String>,
```

### `Run`

**Description:** Run all benchmarks from a config

**Usage locations:**
- `crates/goose-cli/src/cli.rs:183` (clap_command)

**Example context:**
```rust
name: String,
    },

    #[command(about = "Run all benchmarks from a config")]
    Run {
        #[arg(
            short,
```

### `RunNow`

**Description:** Run a scheduled job immediately

**Usage locations:**
- `crates/goose-cli/src/cli.rs:158` (clap_command)

**Example context:**
```rust
limit: Option<u32>,
    },
    /// Run a scheduled job immediately
    #[command(about = "Run a scheduled job immediately")]
    RunNow {
        /// ID of the schedule to run
        #[arg(long, help = "ID of the schedule to run")] // Explicitly make it --id
```

### `Schedule`

**Description:** Manage scheduled jobs

**Usage locations:**
- `crates/goose-cli/src/cli.rs:617` (clap_command)

**Example context:**
```rust
},

    /// Manage scheduled jobs
    #[command(about = "Manage scheduled jobs", visible_alias = "sched")]
    Schedule {
        #[command(subcommand)]
        command: SchedulerCommand,
```

### `Selectors`

**Description:** List all available selectors

**Usage locations:**
- `crates/goose-cli/src/cli.rs:193` (clap_command)

**Example context:**
```rust
config: PathBuf,
    },

    #[command(about = "List all available selectors")]
    Selectors {
        #[arg(
            short,
```

### `ServicesStatus`

**Description:** Check status of Temporal services

**Usage locations:**
- `crates/goose-cli/src/cli.rs:165` (clap_command)

**Example context:**
```rust
id: String,
    },
    /// Check status of Temporal services (temporal scheduler only)
    #[command(about = "Check status of Temporal services")]
    ServicesStatus {},
    /// Stop Temporal services (temporal scheduler only)
    #[command(about = "Stop Temporal services")]
```

### `ServicesStop`

**Description:** Stop Temporal services

**Usage locations:**
- `crates/goose-cli/src/cli.rs:168` (clap_command)

**Example context:**
```rust
#[command(about = "Check status of Temporal services")]
    ServicesStatus {},
    /// Stop Temporal services (temporal scheduler only)
    #[command(about = "Stop Temporal services")]
    ServicesStop {},
    /// Show cron expression examples and help
    #[command(about = "Show cron expression examples and help")]
```

### `Session`

**Description:** Start or resume interactive chat sessions

**Usage locations:**
- `crates/goose-cli/src/cli.rs:290` (clap_command)

**Example context:**
```rust
Mcp { name: String },

    /// Start or resume interactive chat sessions
    #[command(
        about = "Start or resume interactive chat sessions",
        visible_alias = "s"
    )]
```

### `Sessions`

**Description:** List sessions created by a specific schedule

**Usage locations:**
- `crates/goose-cli/src/cli.rs:148` (clap_command)

**Example context:**
```rust
id: String,
    },
    /// List sessions created by a specific schedule
    #[command(about = "List sessions created by a specific schedule")]
    Sessions {
        /// ID of the schedule
        #[arg(long, help = "ID of the schedule")] // Explicitly make it --id
```

### `Update`

**Description:** Update the goose CLI version

**Usage locations:**
- `crates/goose-cli/src/cli.rs:624` (clap_command)

**Example context:**
```rust
},

    /// Update the Goose CLI version
    #[command(about = "Update the goose CLI version")]
    Update {
        /// Update to canary version
        #[arg(
```

### `Validate`

**Description:** Validate a recipe

**Usage locations:**
- `crates/goose-cli/src/cli.rs:232` (clap_command)

**Example context:**
```rust
#[derive(Subcommand)]
enum RecipeCommand {
    /// Validate a recipe file
    #[command(about = "Validate a recipe")]
    Validate {
        /// Recipe name to get recipe file to validate
        #[arg(help = "recipe name to get recipe file or full path to the recipe file to validate")]
```

### `Web`

**Description:** Experimental: Start a web server with a chat interface

**Usage locations:**
- `crates/goose-cli/src/cli.rs:648` (clap_command)

**Example context:**
```rust
},

    /// Start a web server with a chat interface
    #[command(about = "Experimental: Start a web server with a chat interface")]
    Web {
        /// Port to run the web server on
        #[arg(
```

