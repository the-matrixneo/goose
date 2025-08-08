# Comprehensive Goose Configuration Analysis

## Summary

- **Total Configuration Usages:** 584
- **Unique Configuration Keys:** 139
- **Files with Configuration:** 58
- **Test-related Usages:** 2

### By Category

- **Environment Variables:** 54 unique keys
- **Config File Parameters:** 71 unique keys
- **Secret Storage:** 21 unique keys
- **CLI Flags:** 11 unique keys

## Config File Parameters

### `1=1`

**Method(s):** config_delete

**Usage Locations (1):**

- `crates/goose/src/agents/tool_vectordb.rs:165`

**Example Context:**
```rust
// Delete all records instead of dropping the table
                table
                    .delete("1=1") // This will match all records
                    .await
                    .context("Failed to delete all records")?;
```

### `ANTHROPIC_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/anthropic.rs:54`

**Example Context:**
```rust
let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get_param("ANTHROPIC_HOST")
            .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
```

### `AZURE_OPENAI_API_VERSION`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/azure.rs:79`

**Example Context:**
```rust
let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| AZURE_DEFAULT_API_VERSION.to_string());
```

### `AZURE_OPENAI_DEPLOYMENT_NAME`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/azure.rs:77`
- `crates/goose/src/providers/azure.rs:77`

**Example Context:**
```rust
let config = crate::config::Config::global();
        let endpoint: String = config.get_param("AZURE_OPENAI_ENDPOINT")?;
        let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
```

### `AZURE_OPENAI_ENDPOINT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/azure.rs:76`
- `crates/goose/src/providers/azure.rs:76`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let endpoint: String = config.get_param("AZURE_OPENAI_ENDPOINT")?;
        let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
```

### `CLAUDE_CODE_COMMAND`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/claude_code.rs:36`

**Example Context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("CLAUDE_CODE_COMMAND")
            .unwrap_or_else(|_| "claude".to_string());
```

### `DATABRICKS_BACKOFF_MULTIPLIER`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:163`

**Example Context:**
```rust
let backoff_multiplier = config
            .get_param("DATABRICKS_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `DATABRICKS_HOST`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/databricks.rs:113`
- `crates/goose/src/providers/databricks.rs:113`
- `crates/goose/src/providers/databricks.rs:115`
- `crates/goose/src/providers/databricks.rs:115`

**Example Context:**
```rust
let config = crate::config::Config::global();

        let mut host: Result<String, ConfigError> = config.get_param("DATABRICKS_HOST");
        if host.is_err() {
            host = config.get_secret("DATABRICKS_HOST")
```

### `DATABRICKS_INITIAL_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:157`

**Example Context:**
```rust
let initial_interval_ms = config
            .get_param("DATABRICKS_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `DATABRICKS_MAX_RETRIES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:151`

**Example Context:**
```rust
fn load_retry_config(config: &crate::config::Config) -> RetryConfig {
        let max_retries = config
            .get_param("DATABRICKS_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `DATABRICKS_MAX_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:169`

**Example Context:**
```rust
let max_interval_ms = config
            .get_param("DATABRICKS_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_BACKOFF_MULTIPLIER`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:148`

**Example Context:**
```rust
let backoff_multiplier = config
            .get_param("GCP_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
```

### `GCP_INITIAL_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:142`

**Example Context:**
```rust
let initial_interval_ms = config
            .get_param("GCP_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_LOCATION`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:174`

**Example Context:**
```rust
fn determine_location(config: &crate::config::Config) -> Result<String> {
        Ok(config
            .get_param("GCP_LOCATION")
            .ok()
            .filter(|location: &String| !location.trim().is_empty())
```

### `GCP_MAX_RETRIES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:136`

**Example Context:**
```rust
// Load max retries for 429 rate limit errors
        let max_retries = config
            .get_param("GCP_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
```

### `GCP_MAX_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:154`

**Example Context:**
```rust
let max_interval_ms = config
            .get_param("GCP_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
```

### `GCP_PROJECT_ID`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/gcpvertexai.rs:108`
- `crates/goose/src/providers/gcpvertexai.rs:108`

**Example Context:**
```rust
async fn new_async(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let project_id = config.get_param("GCP_PROJECT_ID")?;
        let location = Self::determine_location(config)?;
        let host = format!("https://{}-aiplatform.googleapis.com", location);
```

### `GEMINI_CLI_COMMAND`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gemini_cli.rs:35`

**Example Context:**
```rust
let config = crate::config::Config::global();
        let command: String = config
            .get_param("GEMINI_CLI_COMMAND")
            .unwrap_or_else(|_| "gemini".to_string());
```

### `GOOGLE_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/google.rs:61`

**Example Context:**
```rust
let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get_param("GOOGLE_HOST")
            .unwrap_or_else(|_| GOOGLE_API_HOST.to_string());
```

### `GOOSE_AUTO_COMPACT_THRESHOLD`

**Method(s):** config_set

**Usage Locations (4):**

- `crates/goose/src/context_mgmt/auto_compact.rs:546`
- `crates/goose/src/context_mgmt/auto_compact.rs:546`
- `crates/goose/src/context_mgmt/auto_compact.rs:576`
- `crates/goose/src/context_mgmt/auto_compact.rs:576`

**Example Context:**
```rust
let config = Config::global();
        config
            .set_param("GOOSE_AUTO_COMPACT_THRESHOLD", serde_json::Value::from(0.1))
            .unwrap();
```

### `GOOSE_MAX_TURNS`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose-cli/src/commands/configure.rs:1522`
- `crates/goose-cli/src/commands/configure.rs:1522`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose/src/agents/agent.rs:884`
- `crates/goose/src/agents/agent.rs:884`

**Example Context:**
```rust
let config = Config::global();

    let current_max_turns: u32 = config.get_param("GOOSE_MAX_TURNS").unwrap_or(1000);

    let max_turns_input: String =
```

### `GOOSE_PROVIDER`

**Method(s):** env_remove, config_set, env_set, config_get

**Usage Locations (18):**

- `crates/goose-cli/src/commands/configure.rs:297`
- `crates/goose-cli/src/commands/configure.rs:297`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:1310`
- `crates/goose-cli/src/commands/web.rs:87`
- `crates/goose-cli/src/commands/web.rs:87`
- `crates/goose-cli/src/session/builder.rs:187`
- `crates/goose-cli/src/session/builder.rs:187`
- ... and 8 more locations

**Example Context:**
```rust
// Get current default provider if it exists
    let current_provider: Option<String> = config.get_param("GOOSE_PROVIDER").ok();
    let default_provider = current_provider.unwrap_or_default();
```

### `GOOSE_SYSTEM_PROMPT_FILE_PATH`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/builder.rs:563`
- `crates/goose-cli/src/session/builder.rs:563`

**Example Context:**
```rust
// Only override system prompt if a system override exists
    let system_prompt_file: Option<String> = config.get_param("GOOSE_SYSTEM_PROMPT_FILE_PATH").ok();
    if let Some(ref path) = system_prompt_file {
        let override_prompt =
```

### `GROQ_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/groq.rs:40`

**Example Context:**
```rust
let api_key: String = config.get_secret("GROQ_API_KEY")?;
        let host: String = config
            .get_param("GROQ_HOST")
            .unwrap_or_else(|_| GROQ_API_HOST.to_string());
```

### `LITELLM_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:40`

**Example Context:**
```rust
.unwrap_or_else(|_| "https://api.litellm.ai".to_string());
        let base_path: String = config
            .get_param("LITELLM_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
```

### `LITELLM_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:37`

**Example Context:**
```rust
.unwrap_or_else(|_| String::new());
        let host: String = config
            .get_param("LITELLM_HOST")
            .unwrap_or_else(|_| "https://api.litellm.ai".to_string());
        let base_path: String = config
```

### `LITELLM_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/litellm.rs:47`
- `crates/goose/src/providers/litellm.rs:47`

**Example Context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("LITELLM_TIMEOUT").unwrap_or(600);

        let auth = if api_key.is_empty() {
```

### `OLLAMA_HOST`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/ollama.rs:41`
- `crates/goose/src/providers/toolshim.rs:88`

**Example Context:**
```rust
let config = crate::config::Config::global();
        let host: String = config
            .get_param("OLLAMA_HOST")
            .unwrap_or_else(|_| OLLAMA_HOST.to_string());
```

### `OLLAMA_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/ollama.rs:45`
- `crates/goose/src/providers/ollama.rs:45`

**Example Context:**
```rust
let timeout: Duration =
            Duration::from_secs(config.get_param("OLLAMA_TIMEOUT").unwrap_or(OLLAMA_TIMEOUT));

        // OLLAMA_HOST is sometimes just the 'host' or 'host:port' without a scheme
```

### `OPENAI_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openai.rs:66`

**Example Context:**
```rust
.unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
            .get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
```

### `OPENAI_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openai.rs:63`

**Example Context:**
```rust
let api_key: String = config.get_secret("OPENAI_API_KEY")?;
        let host: String = config
            .get_param("OPENAI_HOST")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());
        let base_path: String = config
```

### `OPENAI_ORGANIZATION`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:68`
- `crates/goose/src/providers/openai.rs:68`

**Example Context:**
```rust
.get_param("OPENAI_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
```

### `OPENAI_PROJECT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:69`
- `crates/goose/src/providers/openai.rs:69`

**Example Context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let organization: Option<String> = config.get_param("OPENAI_ORGANIZATION").ok();
        let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
```

### `OPENAI_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:75`
- `crates/goose/src/providers/openai.rs:75`

**Example Context:**
```rust
.ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("OPENAI_TIMEOUT").unwrap_or(600);

        let auth = AuthMethod::BearerToken(api_key);
```

### `OPENROUTER_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openrouter.rs:48`

**Example Context:**
```rust
let api_key: String = config.get_secret("OPENROUTER_API_KEY")?;
        let host: String = config
            .get_param("OPENROUTER_HOST")
            .unwrap_or_else(|_| "https://openrouter.ai".to_string());
```

### `RANDOM_THINKING_MESSAGES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/output.rs:101`

**Example Context:**
```rust
let spinner = cliclack::spinner();
        if Config::global()
            .get_param("RANDOM_THINKING_MESSAGES")
            .unwrap_or(true)
        {
```

### `SAGEMAKER_ENDPOINT_NAME`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/sagemaker_tgi.rs:40`
- `crates/goose/src/providers/sagemaker_tgi.rs:40`

**Example Context:**
```rust
// Get SageMaker endpoint name (just the name, not full URL)
        let endpoint_name: String = config.get_param("SAGEMAKER_ENDPOINT_NAME").map_err(|_| {
            anyhow::anyhow!("SAGEMAKER_ENDPOINT_NAME is required for SageMaker TGI provider")
        })?;
```

### `SNOWFLAKE_HOST`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/snowflake.rs:48`
- `crates/goose/src/providers/snowflake.rs:48`
- `crates/goose/src/providers/snowflake.rs:50`
- `crates/goose/src/providers/snowflake.rs:50`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let mut host: Result<String, ConfigError> = config.get_param("SNOWFLAKE_HOST");
        if host.is_err() {
            host = config.get_secret("SNOWFLAKE_HOST")
```

### `SNOWFLAKE_TOKEN`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/snowflake.rs:69`
- `crates/goose/src/providers/snowflake.rs:69`
- `crates/goose/src/providers/snowflake.rs:72`
- `crates/goose/src/providers/snowflake.rs:72`

**Example Context:**
```rust
}

        let mut token: Result<String, ConfigError> = config.get_param("SNOWFLAKE_TOKEN");

        if token.is_err() {
```

### `VENICE_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:93`

**Example Context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
            .get_param("VENICE_BASE_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
```

### `VENICE_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:90`

**Example Context:**
```rust
let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
            .unwrap_or_else(|_| VENICE_DEFAULT_HOST.to_string());
        let base_path: String = config
```

### `VENICE_MODELS_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:96`

**Example Context:**
```rust
.unwrap_or_else(|_| VENICE_DEFAULT_BASE_PATH.to_string());
        let models_path: String = config
            .get_param("VENICE_MODELS_PATH")
            .unwrap_or_else(|_| VENICE_DEFAULT_MODELS_PATH.to_string());
```

### `XAI_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/xai.rs:53`

**Example Context:**
```rust
let api_key: String = config.get_secret("XAI_API_KEY")?;
        let host: String = config
            .get_param("XAI_HOST")
            .unwrap_or_else(|_| XAI_API_HOST.to_string());
```

### `another_key`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1149`
- `crates/goose/src/config/base.rs:1149`
- `crates/goose/src/config/base.rs:1149`

**Example Context:**
```rust
// First, create a config with some data
        config.set_param("test_key_backup", Value::String("backup_value".to_string()))?;
        config.set_param("another_key", Value::Number(42.into()))?;

        // Verify the backup was created
```

### `complex_key`

**Method(s):** config_set, config_get

**Usage Locations (5):**

- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:841`
- `crates/goose/src/config/base.rs:841`

**Example Context:**
```rust
// Set a complex value
        config.set_param(
            "complex_key",
            serde_json::json!({
```

### `config`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1429`
- `crates/goose/src/config/base.rs:1429`

**Example Context:**
```rust
level: i32,
        }
        let value: TestConfig = config.get_param("config")?;
        assert_eq!(value.debug, true);
        assert_eq!(value.level, 5);
```

### `enabled`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1419`
- `crates/goose/src/config/base.rs:1419`

**Example Context:**
```rust
// Test boolean environment variable
        std::env::set_var("ENABLED", "true");
        let value: bool = config.get_param("enabled")?;
        assert_eq!(value, true);
```

### `experiments`

**Method(s):** config_set, config_get

**Usage Locations (6):**

- `crates/goose/src/config/experiments.rs:23`
- `crates/goose/src/config/experiments.rs:23`
- `crates/goose/src/config/experiments.rs:33`
- `crates/goose/src/config/experiments.rs:38`
- `crates/goose/src/config/experiments.rs:38`
- `crates/goose/src/config/experiments.rs:38`

**Example Context:**
```rust
let config = Config::global();
        let mut experiments: HashMap<String, bool> =
            config.get_param("experiments").unwrap_or_default();
        Self::refresh_experiments(&mut experiments);
```

### `extensions`

**Method(s):** config_set, config_get

**Usage Locations (23):**

- `crates/goose/src/config/extensions.rs:36`
- `crates/goose/src/config/extensions.rs:36`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:72`
- `crates/goose/src/config/extensions.rs:72`
- `crates/goose/src/config/extensions.rs:89`
- `crates/goose/src/config/extensions.rs:95`
- `crates/goose/src/config/extensions.rs:95`
- ... and 13 more locations

**Example Context:**
```rust
// Try to get the extension entry
        let extensions: HashMap<String, ExtensionEntry> = match config.get_param("extensions") {
            Ok(exts) => exts,
            Err(super::ConfigError::NotFound(_)) => {
```

### `key`

**Method(s):** config_delete, secret_delete, config_set, secret_get, secret_set, config_get

**Usage Locations (19):**

- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:880`
- `crates/goose/src/config/base.rs:880`
- `crates/goose/src/config/base.rs:883`
- `crates/goose/src/config/base.rs:883`
- `crates/goose/src/config/base.rs:885`
- `crates/goose/src/config/base.rs:885`
- `crates/goose/src/config/base.rs:897`
- ... and 9 more locations

**Example Context:**
```rust
let config = Config::new(temp_file.path(), TEST_KEYRING_SERVICE)?;

        config.set_param("key", Value::String("value".to_string()))?;

        let value: String = config.get_param("key")?;
```

### `key1`

**Method(s):** secret_delete, config_set, secret_get, secret_set

**Usage Locations (19):**

- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:949`
- `crates/goose/src/config/base.rs:949`
- `crates/goose/src/config/base.rs:955`
- `crates/goose/src/config/base.rs:955`
- ... and 9 more locations

**Example Context:**
```rust
let config = Config::new(temp_file.path(), TEST_KEYRING_SERVICE)?;

        config.set_param("key1", Value::String("value1".to_string()))?;
        config.set_param("key2", Value::Number(42.into()))?;
```

### `key2`

**Method(s):** config_set, secret_get, secret_set

**Usage Locations (13):**

- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:950`
- `crates/goose/src/config/base.rs:950`
- `crates/goose/src/config/base.rs:959`
- `crates/goose/src/config/base.rs:959`
- ... and 3 more locations

**Example Context:**
```rust
config.set_param("key1", Value::String("value1".to_string()))?;
        config.set_param("key2", Value::Number(42.into()))?;

        // Read the file directly to check YAML formatting
```

### `nonexistent_key`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:853`
- `crates/goose/src/config/base.rs:853`

**Example Context:**
```rust
let config = Config::new(temp_file.path(), TEST_KEYRING_SERVICE).unwrap();

        let result: Result<String, ConfigError> = config.get_param("nonexistent_key");
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }
```

### `port`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1414`
- `crates/goose/src/config/base.rs:1414`

**Example Context:**
```rust
// Test number environment variable
        std::env::set_var("PORT", "8080");
        let value: i32 = config.get_param("port")?;
        assert_eq!(value, 8080);
```

### `provider`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1409`
- `crates/goose/src/config/base.rs:1409`

**Example Context:**
```rust
// Test string environment variable (the original issue case)
        std::env::set_var("PROVIDER", "ANTHROPIC");
        let value: String = config.get_param("provider")?;
        assert_eq!(value, "ANTHROPIC");
```

### `server`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:97`
- `crates/goose/src/config/base.rs:97`

**Example Context:**
```rust
/// }
///
/// let server_config: ServerConfig = config.get_param("server").unwrap();
/// ```
///
```

### `test_key`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:810`
- `crates/goose/src/config/base.rs:810`
- `crates/goose/src/config/base.rs:815`
- `crates/goose/src/config/base.rs:815`

**Example Context:**
```rust
// Set a simple string value
        config.set_param("test_key", Value::String("test_value".to_string()))?;

        // Test simple string retrieval
```

### `test_key_backup`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1148`
- `crates/goose/src/config/base.rs:1148`
- `crates/goose/src/config/base.rs:1148`

**Example Context:**
```rust
// First, create a config with some data
        config.set_param("test_key_backup", Value::String("backup_value".to_string()))?;
        config.set_param("another_key", Value::Number(42.into()))?;
```

### `test_precedence`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1451`
- `crates/goose/src/config/base.rs:1451`
- `crates/goose/src/config/base.rs:1458`
- `crates/goose/src/config/base.rs:1458`

**Example Context:**
```rust
// Set value in config file
        config.set_param("test_precedence", Value::String("file_value".to_string()))?;

        // Verify file value is returned when no env var
```

### `third_key`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1156`
- `crates/goose/src/config/base.rs:1156`
- `crates/goose/src/config/base.rs:1156`

**Example Context:**
```rust
// Make sure we have a backup by doing another write
        config.set_param("third_key", Value::Bool(true))?;
        assert!(primary_backup.exists(), "Backup should exist after writes");
```

### `version`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1213`
- `crates/goose/src/config/base.rs:1213`
- `crates/goose/src/config/base.rs:1213`

**Example Context:**
```rust
// Create multiple versions to test rotation
        for i in 1..=7 {
            config.set_param("version", Value::Number(i.into()))?;
        }
```

## Secret Storage

### `ANTHROPIC_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/anthropic.rs:52`
- `crates/goose/src/providers/anthropic.rs:52`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get_param("ANTHROPIC_HOST")
```

### `AZURE_OPENAI_API_KEY`

**Method(s):** secret_get

**Usage Locations (1):**

- `crates/goose/src/providers/azure.rs:83`

**Example Context:**
```rust
let api_key = config
            .get_secret("AZURE_OPENAI_API_KEY")
            .ok()
            .filter(|key: &String| !key.is_empty());
```

### `ELEVENLABS_API_KEY`

**Method(s):** config_delete, secret_get

**Usage Locations (4):**

- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:231`
- `crates/goose-server/src/routes/audio.rs:231`

**Example Context:**
```rust
// First try to get it as a secret
    let api_key: String = match config.get_secret("ELEVENLABS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
```

### `GITHUB_COPILOT_TOKEN`

**Method(s):** secret_set

**Usage Locations (5):**

- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:498`
- `crates/goose/src/providers/githubcopilot.rs:498`

**Example Context:**
```rust
.await
                        .context("unable to login into github")?;
                    config.set_secret("GITHUB_COPILOT_TOKEN", Value::String(token.clone()))?;
                    token
                }
```

### `GOOGLE_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/google.rs:59`
- `crates/goose/src/providers/google.rs:59`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get_param("GOOGLE_HOST")
```

### `GROQ_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/groq.rs:38`
- `crates/goose/src/providers/groq.rs:38`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("GROQ_API_KEY")?;
        let host: String = config
            .get_param("GROQ_HOST")
```

### `LITELLM_API_KEY`

**Method(s):** secret_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:34`

**Example Context:**
```rust
let config = crate::config::Config::global();
        let api_key: String = config
            .get_secret("LITELLM_API_KEY")
            .unwrap_or_else(|_| String::new());
        let host: String = config
```

### `LITELLM_CUSTOM_HEADERS`

**Method(s):** secret_get, config_get

**Usage Locations (3):**

- `crates/goose/src/providers/litellm.rs:43`
- `crates/goose/src/providers/litellm.rs:44`
- `crates/goose/src/providers/litellm.rs:44`

**Example Context:**
```rust
.unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("LITELLM_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("LITELLM_CUSTOM_HEADERS"))
            .ok()
```

### `OPENAI_API_KEY`

**Method(s):** secret_get, config_get

**Usage Locations (5):**

- `crates/goose-server/src/routes/audio.rs:100`
- `crates/goose/src/config/base.rs:88`
- `crates/goose/src/config/base.rs:88`
- `crates/goose/src/providers/openai.rs:61`
- `crates/goose/src/providers/openai.rs:61`

**Example Context:**
```rust
let config = goose::config::Config::global();
    let api_key: String = config
        .get_secret("OPENAI_API_KEY")
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
```

### `OPENAI_CUSTOM_HEADERS`

**Method(s):** secret_get, config_get

**Usage Locations (3):**

- `crates/goose/src/providers/openai.rs:71`
- `crates/goose/src/providers/openai.rs:72`
- `crates/goose/src/providers/openai.rs:72`

**Example Context:**
```rust
let project: Option<String> = config.get_param("OPENAI_PROJECT").ok();
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("OPENAI_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("OPENAI_CUSTOM_HEADERS"))
            .ok()
```

### `OPENROUTER_API_KEY`

**Method(s):** secret_get, secret_set

**Usage Locations (5):**

- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/providers/openrouter.rs:46`
- `crates/goose/src/providers/openrouter.rs:46`

**Example Context:**
```rust
pub fn configure_openrouter(config: &Config, api_key: String) -> Result<()> {
    config.set_secret("OPENROUTER_API_KEY", Value::String(api_key))?;
    config.set_param("GOOSE_PROVIDER", Value::String("openrouter".to_string()))?;
    config.set_param(
```

### `VENICE_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/venice.rs:88`
- `crates/goose/src/providers/venice.rs:88`

**Example Context:**
```rust
pub fn from_env(mut model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("VENICE_API_KEY")?;
        let host: String = config
            .get_param("VENICE_HOST")
```

### `XAI_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/xai.rs:51`
- `crates/goose/src/providers/xai.rs:51`

**Example Context:**
```rust
pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("XAI_API_KEY")?;
        let host: String = config
            .get_param("XAI_HOST")
```

### `api_key`

**Method(s):** secret_delete, secret_get, secret_set

**Usage Locations (12):**

- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:919`
- `crates/goose/src/config/base.rs:919`
- `crates/goose/src/config/base.rs:924`
- `crates/goose/src/config/base.rs:924`
- `crates/goose/src/config/base.rs:929`
- `crates/goose/src/config/base.rs:929`
- `crates/goose/src/config/base.rs:929`
- ... and 2 more locations

**Example Context:**
```rust
// Test setting and getting a simple secret
        config.set_secret("api_key", Value::String("secret123".to_string()))?;
        let value: String = config.get_secret("api_key")?;
        assert_eq!(value, "secret123");
```

## Environment Variables

### `API_KEY`

**Method(s):** env_remove, env_set

**Usage Locations (4):**

- `crates/goose/src/config/base.rs:923`
- `crates/goose/src/config/base.rs:923`
- `crates/goose/src/config/base.rs:926`
- `crates/goose/src/config/base.rs:926`

**Example Context:**
```rust
// Test environment variable override
        std::env::set_var("API_KEY", "env_secret");
        let value: String = config.get_secret("api_key")?;
        assert_eq!(value, "env_secret");
```

### `CARGO_MANIFEST_DIR`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-server/src/bin/generate_schema.rs:9`

**Example Context:**
```rust
let schema = openapi::generate_schema();

    let package_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_path = PathBuf::from(package_dir)
        .join("..")
```

### `CLAUDE_THINKING_BUDGET`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/formats/anthropic.rs:419`
- `crates/goose/src/providers/formats/anthropic.rs:419`
- `crates/goose/src/providers/formats/databricks.rs:563`
- `crates/goose/src/providers/formats/databricks.rs:563`

**Example Context:**
```rust
if model_config.model_name.starts_with("claude-3-7-sonnet-") && is_thinking_enabled {
        // Minimum budget_tokens is 1024
        let budget_tokens = std::env::var("CLAUDE_THINKING_BUDGET")
            .unwrap_or_else(|_| "16000".to_string())
            .parse()
```

### `CLAUDE_THINKING_ENABLED`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (14):**

- `crates/goose/src/providers/anthropic.rs:71`
- `crates/goose/src/providers/anthropic.rs:71`
- `crates/goose/src/providers/formats/anthropic.rs:416`
- `crates/goose/src/providers/formats/anthropic.rs:416`
- `crates/goose/src/providers/formats/anthropic.rs:915`
- `crates/goose/src/providers/formats/anthropic.rs:915`
- `crates/goose/src/providers/formats/anthropic.rs:916`
- `crates/goose/src/providers/formats/anthropic.rs:916`
- `crates/goose/src/providers/formats/anthropic.rs:944`
- `crates/goose/src/providers/formats/anthropic.rs:944`
- ... and 4 more locations

**Example Context:**
```rust
let mut headers = Vec::new();

        let is_thinking_enabled = std::env::var("CLAUDE_THINKING_ENABLED").is_ok();
        if self.model.model_name.starts_with("claude-3-7-sonnet-") {
            if is_thinking_enabled {
```

### `CONTEXT_FILE_NAMES`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (10):**

- `crates/goose-mcp/src/developer/mod.rs:406`
- `crates/goose-mcp/src/developer/mod.rs:406`
- `crates/goose-mcp/src/developer/mod.rs:1714`
- `crates/goose-mcp/src/developer/mod.rs:1714`
- `crates/goose-mcp/src/developer/mod.rs:1723`
- `crates/goose-mcp/src/developer/mod.rs:1723`
- `crates/goose-mcp/src/developer/mod.rs:1731`
- `crates/goose-mcp/src/developer/mod.rs:1731`
- `crates/goose-mcp/src/developer/mod.rs:1739`
- `crates/goose-mcp/src/developer/mod.rs:1739`

**Example Context:**
```rust
};

        let hints_filenames: Vec<String> = std::env::var("CONTEXT_FILE_NAMES")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
```

### `DATABRICKS_TOKEN`

**Method(s):** env_remove, secret_get

**Usage Locations (4):**

- `crates/goose/examples/databricks_oauth.rs:16`
- `crates/goose/examples/databricks_oauth.rs:16`
- `crates/goose/src/providers/databricks.rs:128`
- `crates/goose/src/providers/databricks.rs:128`

**Example Context:**
```rust
// Clear any token to force OAuth
    std::env::remove_var("DATABRICKS_TOKEN");

    // Create the provider
```

### `GITHUB_ACTIONS`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (test)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (test)

### `GOOGLE_DRIVE_CREDENTIALS_PATH`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:104`

**Example Context:**
```rust
let keyfile_path_str = env::var("GOOGLE_DRIVE_OAUTH_PATH")
            .unwrap_or_else(|_| "./gcp-oauth.keys.json".to_string());
        let credentials_path_str = env::var("GOOGLE_DRIVE_CREDENTIALS_PATH")
            .unwrap_or_else(|_| "./gdrive-server-credentials.json".to_string());
```

### `GOOGLE_DRIVE_OAUTH_CONFIG`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:119`

**Example Context:**
```rust
);

        if let Ok(oauth_config) = env::var("GOOGLE_DRIVE_OAUTH_CONFIG") {
            // Ensure the parent directory exists (create_dir_all is idempotent)
            if let Some(parent) = keyfile_path.parent() {
```

### `GOOGLE_DRIVE_OAUTH_PATH`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:102`

**Example Context:**
```rust
Arc<CredentialsManager>,
    ) {
        let keyfile_path_str = env::var("GOOGLE_DRIVE_OAUTH_PATH")
            .unwrap_or_else(|_| "./gcp-oauth.keys.json".to_string());
        let credentials_path_str = env::var("GOOGLE_DRIVE_CREDENTIALS_PATH")
```

### `GOOSE_ALLOWLIST`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (3):**

- `crates/goose-server/src/routes/extension.rs:351`
- `crates/goose-server/src/routes/extension.rs:1057`
- `crates/goose-server/src/routes/extension.rs:1075`

**Example Context:**
```rust
#[allow(dead_code)]
fn fetch_allowed_extensions() -> Option<AllowedExtensions> {
    match env::var("GOOSE_ALLOWLIST") {
        Err(_) => {
            // Environment variable not set, no allowlist to enforce
```

### `GOOSE_ALLOWLIST_BYPASS`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (8):**

- `crates/goose-server/src/routes/extension.rs:392`
- `crates/goose-server/src/routes/extension.rs:1096`
- `crates/goose-server/src/routes/extension.rs:1105`
- `crates/goose-server/src/routes/extension.rs:1112`
- `crates/goose-server/src/routes/extension.rs:1117`
- `crates/goose-server/src/routes/extension.rs:1119`
- `crates/goose-server/src/routes/extension.rs:1123`
- `crates/goose-server/src/routes/extension.rs:1155`

**Example Context:**
```rust
fn is_command_allowed(cmd: &str, args: &[String]) -> bool {
    // Check if bypass is enabled
    if let Ok(bypass_value) = env::var("GOOSE_ALLOWLIST_BYPASS") {
        if bypass_value.to_lowercase() == "true" {
            // Bypass the allowlist check
```

### `GOOSE_CACHE_DIR`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/providers/pricing.rs:16`
- `crates/goose/src/providers/pricing.rs:16`

**Example Context:**
```rust
/// Get the cache directory path
fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = if let Ok(goose_dir) = std::env::var("GOOSE_CACHE_DIR") {
        PathBuf::from(goose_dir)
    } else {
```

### `GOOSE_CLAUDE_CODE_DEBUG`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/claude_code.rs:310`
- `crates/goose/src/providers/claude_code.rs:310`
- `crates/goose/src/providers/claude_code.rs:429`
- `crates/goose/src/providers/claude_code.rs:429`

**Example Context:**
```rust
let filtered_system = self.filter_extensions_from_system_prompt(system);

        if std::env::var("GOOSE_CLAUDE_CODE_DEBUG").is_ok() {
            println!("=== CLAUDE CODE PROVIDER DEBUG ===");
            println!("Command: {}", self.command);
```

### `GOOSE_CLI_MIN_PRIORITY`

**Method(s):** config_set, env_var

**Usage Locations (11):**

- `crates/goose-cli/src/commands/configure.rs:1216`
- `crates/goose-cli/src/commands/configure.rs:1216`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1235`
- `crates/goose-cli/src/commands/configure.rs:1235`
- ... and 1 more locations

**Example Context:**
```rust
let config = Config::global();
    // Check if GOOSE_CLI_MIN_PRIORITY is set as an environment variable
    if std::env::var("GOOSE_CLI_MIN_PRIORITY").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_CLI_MIN_PRIORITY environment variable is set and will override the configuration here.");
    }
```

### `GOOSE_CLI_SHOW_THINKING`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/session/output.rs:176`
- `crates/goose-cli/src/session/output.rs:176`

**Example Context:**
```rust
}
            MessageContent::Thinking(thinking) => {
                if std::env::var("GOOSE_CLI_SHOW_THINKING").is_ok()
                    && std::io::stdout().is_terminal()
                {
```

### `GOOSE_CLI_THEME`

**Method(s):** config_set, env_var

**Usage Locations (7):**

- `crates/goose-cli/src/session/output.rs:58`
- `crates/goose-cli/src/session/output.rs:58`
- `crates/goose-cli/src/session/output.rs:71`
- `crates/goose-cli/src/session/output.rs:71`
- `crates/goose-cli/src/session/output.rs:82`
- `crates/goose-cli/src/session/output.rs:82`
- `crates/goose-cli/src/session/output.rs:82`

**Example Context:**
```rust
thread_local! {
    static CURRENT_THEME: RefCell<Theme> = RefCell::new(
        std::env::var("GOOSE_CLI_THEME").ok()
            .map(|val| Theme::from_config_str(&val))
            .unwrap_or_else(||
```

### `GOOSE_CONTEXT_LIMIT`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (5):**

- `crates/goose/src/model.rs:115`
- `crates/goose/src/model.rs:115`
- `crates/goose/src/providers/factory.rs:399`
- `crates/goose/src/providers/factory.rs:423`
- `crates/goose/src/providers/factory.rs:425`

**Example Context:**
```rust
}
        }
        if let Ok(val) = std::env::var("GOOSE_CONTEXT_LIMIT") {
            return Self::validate_context_limit(&val, "GOOSE_CONTEXT_LIMIT").map(Some);
        }
```

### `GOOSE_DISABLE_KEYRING`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose/src/config/base.rs:132`

**Example Context:**
```rust
let config_path = config_dir.join("config.yaml");

        let secrets = match env::var("GOOSE_DISABLE_KEYRING") {
            Ok(_) => SecretStorage::File {
                path: config_dir.join("secrets.yaml"),
```

### `GOOSE_EDITOR_API_KEY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:78`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:78`

**Example Context:**
```rust
// Check if basic editor API variables are set
    let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;
```

### `GOOSE_EDITOR_HOST`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:79`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:79`

**Example Context:**
```rust
// Check if basic editor API variables are set
    let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;
```

### `GOOSE_EDITOR_MODEL`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:80`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:80`

**Example Context:**
```rust
let api_key = std::env::var("GOOSE_EDITOR_API_KEY").ok()?;
    let host = std::env::var("GOOSE_EDITOR_HOST").ok()?;
    let model = std::env::var("GOOSE_EDITOR_MODEL").ok()?;

    if api_key.is_empty() || host.is_empty() || model.is_empty() {
```

### `GOOSE_EMBEDDING_MODEL`

**Method(s):** env_var

**Usage Locations (5):**

- `crates/goose/src/agents/router_tool_selector.rs:48`
- `crates/goose/src/providers/litellm.rs:229`
- `crates/goose/src/providers/litellm.rs:229`
- `crates/goose/src/providers/openai.rs:268`
- `crates/goose/src/providers/openai.rs:268`

**Example Context:**
```rust
// If env var is set, create a new provider for embeddings
            // Get embedding model and provider from environment variables
            let embedding_model = env::var("GOOSE_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "text-embedding-3-small".to_string());
            let embedding_provider_name =
```

### `GOOSE_EMBEDDING_MODEL_PROVIDER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/agents/router_tool_selector.rs:45`
- `crates/goose/src/agents/router_tool_selector.rs:51`

**Example Context:**
```rust
let vector_db = ToolVectorDB::new(Some(table_name)).await?;

        let embedding_provider = if env::var("GOOSE_EMBEDDING_MODEL_PROVIDER").is_ok() {
            // If env var is set, create a new provider for embeddings
            // Get embedding model and provider from environment variables
```

### `GOOSE_GEMINI_CLI_DEBUG`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/gemini_cli.rs:161`
- `crates/goose/src/providers/gemini_cli.rs:161`
- `crates/goose/src/providers/gemini_cli.rs:280`
- `crates/goose/src/providers/gemini_cli.rs:280`

**Example Context:**
```rust
full_prompt.push_str("Assistant: ");

        if std::env::var("GOOSE_GEMINI_CLI_DEBUG").is_ok() {
            println!("=== GEMINI CLI PROVIDER DEBUG ===");
            println!("Command: {}", self.command);
```

### `GOOSE_LEAD_FAILURE_THRESHOLD`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (5):**

- `crates/goose/src/providers/factory.rs:291`
- `crates/goose/src/providers/factory.rs:324`
- `crates/goose/src/providers/factory.rs:345`
- `crates/goose/src/providers/factory.rs:352`
- `crates/goose/src/providers/factory.rs:381`

**Example Context:**
```rust
(
                "GOOSE_LEAD_FAILURE_THRESHOLD",
                env::var("GOOSE_LEAD_FAILURE_THRESHOLD").ok(),
            ),
            (
```

### `GOOSE_LEAD_FALLBACK_TURNS`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (5):**

- `crates/goose/src/providers/factory.rs:295`
- `crates/goose/src/providers/factory.rs:325`
- `crates/goose/src/providers/factory.rs:346`
- `crates/goose/src/providers/factory.rs:353`
- `crates/goose/src/providers/factory.rs:384`

**Example Context:**
```rust
(
                "GOOSE_LEAD_FALLBACK_TURNS",
                env::var("GOOSE_LEAD_FALLBACK_TURNS").ok(),
            ),
        ];
```

### `GOOSE_LEAD_MODEL`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (11):**

- `crates/goose/src/providers/factory.rs:236`
- `crates/goose/src/providers/factory.rs:241`
- `crates/goose/src/providers/factory.rs:269`
- `crates/goose/src/providers/factory.rs:270`
- `crates/goose/src/providers/factory.rs:286`
- `crates/goose/src/providers/factory.rs:305`
- `crates/goose/src/providers/factory.rs:342`
- `crates/goose/src/providers/factory.rs:349`
- `crates/goose/src/providers/factory.rs:372`
- `crates/goose/src/providers/factory.rs:394`
- ... and 1 more locations

**Example Context:**
```rust
fn test_create_lead_worker_provider() {
        // Save current env vars
        let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();
```

### `GOOSE_LEAD_PROVIDER`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (8):**

- `crates/goose/src/providers/factory.rs:237`
- `crates/goose/src/providers/factory.rs:261`
- `crates/goose/src/providers/factory.rs:273`
- `crates/goose/src/providers/factory.rs:274`
- `crates/goose/src/providers/factory.rs:287`
- `crates/goose/src/providers/factory.rs:343`
- `crates/goose/src/providers/factory.rs:350`
- `crates/goose/src/providers/factory.rs:375`

**Example Context:**
```rust
// Save current env vars
        let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();
```

### `GOOSE_LEAD_TURNS`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (9):**

- `crates/goose/src/providers/factory.rs:238`
- `crates/goose/src/providers/factory.rs:262`
- `crates/goose/src/providers/factory.rs:277`
- `crates/goose/src/providers/factory.rs:278`
- `crates/goose/src/providers/factory.rs:288`
- `crates/goose/src/providers/factory.rs:323`
- `crates/goose/src/providers/factory.rs:344`
- `crates/goose/src/providers/factory.rs:351`
- `crates/goose/src/providers/factory.rs:378`

**Example Context:**
```rust
let saved_lead = env::var("GOOSE_LEAD_MODEL").ok();
        let saved_provider = env::var("GOOSE_LEAD_PROVIDER").ok();
        let saved_turns = env::var("GOOSE_LEAD_TURNS").ok();

        // Test with basic lead model configuration
```

### `GOOSE_MODE`

**Method(s):** env_set, env_remove, env_var, config_set, config_get

**Usage Locations (35):**

- `crates/goose-cli/src/commands/configure.rs:1121`
- `crates/goose-cli/src/commands/configure.rs:1121`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1158`
- `crates/goose-cli/src/commands/configure.rs:1158`
- ... and 25 more locations

**Example Context:**
```rust
// Check if GOOSE_MODE is set as an environment variable
    if std::env::var("GOOSE_MODE").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_MODE environment variable is set and will override the configuration here.");
    }
```

### `GOOSE_MODEL`

**Method(s):** env_set, env_remove, env_var, config_set, config_get

**Usage Locations (21):**

- `crates/goose-cli/src/commands/configure.rs:442`
- `crates/goose-cli/src/commands/configure.rs:442`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:1314`
- `crates/goose-cli/src/commands/configure.rs:1578`
- `crates/goose-cli/src/commands/configure.rs:1578`
- `crates/goose-cli/src/commands/web.rs:95`
- `crates/goose-cli/src/commands/web.rs:95`
- ... and 11 more locations

**Example Context:**
```rust
Ok(None) => {
            let default_model =
                std::env::var("GOOSE_MODEL").unwrap_or(provider_meta.default_model.clone());
            cliclack::input("Enter a model from that provider:")
                .default_input(&default_model)
```

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`

**Method(s):** config_set, env_var, config_get

**Usage Locations (9):**

- `crates/goose-cli/src/commands/configure.rs:1174`
- `crates/goose-cli/src/commands/configure.rs:1174`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose/src/agents/tool_route_manager.rs:78`

**Example Context:**
```rust
// Check if GOOSE_ROUTER_STRATEGY is set as an environment variable
    if std::env::var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_ROUTER_TOOL_SELECTION_STRATEGY environment variable is set. Configuration will override this.");
    }
```

### `GOOSE_SCHEDULER_TYPE`

**Method(s):** config_set, env_var, config_get

**Usage Locations (13):**

- `crates/goose-cli/src/commands/configure.rs:1469`
- `crates/goose-cli/src/commands/configure.rs:1469`
- `crates/goose-cli/src/commands/configure.rs:1475`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/schedule.rs:266`
- ... and 3 more locations

**Example Context:**
```rust
// Check if GOOSE_SCHEDULER_TYPE is set as an environment variable
    if std::env::var("GOOSE_SCHEDULER_TYPE").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_SCHEDULER_TYPE environment variable is set and will override the configuration here.");
    }
```

### `GOOSE_SERVER__SECRET_KEY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-server/src/commands/agent.rs:31`
- `crates/goose-server/src/commands/agent.rs:31`

**Example Context:**
```rust
let secret_key =
        std::env::var("GOOSE_SERVER__SECRET_KEY").unwrap_or_else(|_| "test".to_string());

    let new_agent = Agent::new();
```

### `GOOSE_TOOLSHIM`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose-cli/src/commands/configure.rs:454`
- `crates/goose-cli/src/commands/configure.rs:454`
- `crates/goose/src/model.rs:162`
- `crates/goose/src/model.rs:162`

**Example Context:**
```rust
// Create model config with env var settings
    let toolshim_enabled = std::env::var("GOOSE_TOOLSHIM")
        .map(|val| val == "1" || val.to_lowercase() == "true")
        .unwrap_or(false);
```

### `GOOSE_TOOLSHIM_OLLAMA_MODEL`

**Method(s):** env_var

**Usage Locations (6):**

- `crates/goose-cli/src/commands/configure.rs:461`
- `crates/goose-cli/src/commands/configure.rs:461`
- `crates/goose/src/model.rs:178`
- `crates/goose/src/model.rs:178`
- `crates/goose/src/providers/toolshim.rs:282`
- `crates/goose/src/providers/toolshim.rs:282`

**Example Context:**
```rust
.with_max_tokens(Some(50))
        .with_toolshim(toolshim_enabled)
        .with_toolshim_model(std::env::var("GOOSE_TOOLSHIM_OLLAMA_MODEL").ok());

    let provider = create(provider_name, model_config)?;
```

### `GOOSE_VECTOR_DB_PATH`

**Method(s):** env_remove, env_set

**Usage Locations (5):**

- `crates/goose/src/agents/tool_vectordb.rs:554`
- `crates/goose/src/agents/tool_vectordb.rs:559`
- `crates/goose/src/agents/tool_vectordb.rs:568`
- `crates/goose/src/agents/tool_vectordb.rs:581`
- `crates/goose/src/agents/tool_vectordb.rs:589`

**Example Context:**
```rust
let custom_path = temp_dir.path().join("custom_vector_db");

        env::set_var("GOOSE_VECTOR_DB_PATH", custom_path.to_str().unwrap());

        let db_path = ToolVectorDB::get_db_path()?;
```

### `GOOSE_WORKER_CONTEXT_LIMIT`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (3):**

- `crates/goose/src/providers/factory.rs:397`
- `crates/goose/src/providers/factory.rs:418`
- `crates/goose/src/providers/factory.rs:420`

**Example Context:**
```rust
(
                "GOOSE_WORKER_CONTEXT_LIMIT",
                env::var("GOOSE_WORKER_CONTEXT_LIMIT").ok(),
            ),
            ("GOOSE_CONTEXT_LIMIT", env::var("GOOSE_CONTEXT_LIMIT").ok()),
```

### `GOOSE_WORKING_DIR`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/memory/mod.rs:228`
- `crates/goose-mcp/src/memory/mod.rs:228`

**Example Context:**
```rust
// Check for .goose/memory in current directory
        let local_memory_dir = std::env::var("GOOSE_WORKING_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap())
```

### `HOME`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (10):**

- `crates/goose-cli/src/logging.rs:206`
- `crates/goose-cli/src/logging.rs:258`
- `crates/goose-cli/src/session/output.rs:885`
- `crates/goose-cli/src/session/output.rs:888`
- `crates/goose-cli/src/session/output.rs:903`
- `crates/goose-cli/src/session/output.rs:905`
- `crates/goose/src/providers/claude_code.rs:53`
- `crates/goose/src/providers/claude_code.rs:53`
- `crates/goose/src/providers/gemini_cli.rs:52`
- `crates/goose/src/providers/gemini_cli.rs:52`

**Example Context:**
```rust
env::set_var("USERPROFILE", temp_dir.path());
        } else {
            env::set_var("HOME", temp_dir.path());
        }
        temp_dir
```

### `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (7):**

- `crates/goose-cli/src/logging.rs:466`
- `crates/goose-cli/src/logging.rs:490`
- `crates/goose-cli/src/logging.rs:495`
- `crates/goose/src/tracing/langfuse_layer.rs:157`
- `crates/goose/src/tracing/langfuse_layer.rs:431`
- `crates/goose/src/tracing/langfuse_layer.rs:437`
- `crates/goose/src/tracing/langfuse_layer.rs:463`

**Example Context:**
```rust
(
                "LANGFUSE_INIT_PROJECT_PUBLIC_KEY",
                env::var("LANGFUSE_INIT_PROJECT_PUBLIC_KEY").ok(),
            ),
            (
```

### `LANGFUSE_INIT_PROJECT_SECRET_KEY`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (6):**

- `crates/goose-cli/src/logging.rs:470`
- `crates/goose-cli/src/logging.rs:491`
- `crates/goose/src/tracing/langfuse_layer.rs:161`
- `crates/goose/src/tracing/langfuse_layer.rs:440`
- `crates/goose/src/tracing/langfuse_layer.rs:446`
- `crates/goose/src/tracing/langfuse_layer.rs:464`

**Example Context:**
```rust
(
                "LANGFUSE_INIT_PROJECT_SECRET_KEY",
                env::var("LANGFUSE_INIT_PROJECT_SECRET_KEY").ok(),
            ),
        ];
```

### `LANGFUSE_PUBLIC_KEY`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (8):**

- `crates/goose-cli/src/logging.rs:461`
- `crates/goose-cli/src/logging.rs:483`
- `crates/goose-cli/src/logging.rs:488`
- `crates/goose/src/tracing/langfuse_layer.rs:156`
- `crates/goose/src/tracing/langfuse_layer.rs:413`
- `crates/goose/src/tracing/langfuse_layer.rs:419`
- `crates/goose/src/tracing/langfuse_layer.rs:449`
- `crates/goose/src/tracing/langfuse_layer.rs:459`

**Example Context:**
```rust
// Store original environment variables (both sets)
        let original_vars = [
            ("LANGFUSE_PUBLIC_KEY", env::var("LANGFUSE_PUBLIC_KEY").ok()),
            ("LANGFUSE_SECRET_KEY", env::var("LANGFUSE_SECRET_KEY").ok()),
            ("LANGFUSE_URL", env::var("LANGFUSE_URL").ok()),
```

### `LANGFUSE_SECRET_KEY`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (8):**

- `crates/goose-cli/src/logging.rs:462`
- `crates/goose-cli/src/logging.rs:484`
- `crates/goose-cli/src/logging.rs:489`
- `crates/goose/src/tracing/langfuse_layer.rs:160`
- `crates/goose/src/tracing/langfuse_layer.rs:422`
- `crates/goose/src/tracing/langfuse_layer.rs:428`
- `crates/goose/src/tracing/langfuse_layer.rs:450`
- `crates/goose/src/tracing/langfuse_layer.rs:460`

**Example Context:**
```rust
let original_vars = [
            ("LANGFUSE_PUBLIC_KEY", env::var("LANGFUSE_PUBLIC_KEY").ok()),
            ("LANGFUSE_SECRET_KEY", env::var("LANGFUSE_SECRET_KEY").ok()),
            ("LANGFUSE_URL", env::var("LANGFUSE_URL").ok()),
            (
```

### `LANGFUSE_URL`

**Method(s):** env_var, env_set

**Usage Locations (3):**

- `crates/goose-cli/src/logging.rs:463`
- `crates/goose/src/tracing/langfuse_layer.rs:169`
- `crates/goose/src/tracing/langfuse_layer.rs:451`

**Example Context:**
```rust
("LANGFUSE_PUBLIC_KEY", env::var("LANGFUSE_PUBLIC_KEY").ok()),
            ("LANGFUSE_SECRET_KEY", env::var("LANGFUSE_SECRET_KEY").ok()),
            ("LANGFUSE_URL", env::var("LANGFUSE_URL").ok()),
            (
                "LANGFUSE_INIT_PROJECT_PUBLIC_KEY",
```

### `NO_COLOR`

**Method(s):** env_var_os

**Usage Locations (2):**

- `crates/goose-cli/src/session/output.rs:484`
- `crates/goose-cli/src/session/output.rs:484`

**Example Context:**
```rust
pub fn env_no_color() -> bool {
    // if NO_COLOR is defined at all disable colors
    std::env::var_os("NO_COLOR").is_none()
}
```

### `OTEL_EXPORTER_OTLP_ENDPOINT`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (8):**

- `crates/goose-cli/src/main.rs:13`
- `crates/goose-cli/src/main.rs:13`
- `crates/goose/src/tracing/otlp_layer.rs:35`
- `crates/goose/src/tracing/otlp_layer.rs:249`
- `crates/goose/src/tracing/otlp_layer.rs:252`
- `crates/goose/src/tracing/otlp_layer.rs:255`
- `crates/goose/src/tracing/otlp_layer.rs:263`
- `crates/goose/src/tracing/otlp_layer.rs:264`

**Example Context:**
```rust
// Only wait for telemetry flush if OTLP is configured
    if std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_ok() {
        // Use a shorter, dynamic wait with max timeout
        let max_wait = tokio::time::Duration::from_millis(500);
```

### `OTEL_EXPORTER_OTLP_TIMEOUT`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (5):**

- `crates/goose/src/tracing/otlp_layer.rs:41`
- `crates/goose/src/tracing/otlp_layer.rs:250`
- `crates/goose/src/tracing/otlp_layer.rs:256`
- `crates/goose/src/tracing/otlp_layer.rs:267`
- `crates/goose/src/tracing/otlp_layer.rs:268`

**Example Context:**
```rust
};

            if let Ok(timeout_str) = env::var("OTEL_EXPORTER_OTLP_TIMEOUT") {
                if let Ok(timeout_ms) = timeout_str.parse::<u64>() {
                    config.timeout = Duration::from_millis(timeout_ms);
```

### `PATH`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/claude_code.rs:86`
- `crates/goose/src/providers/claude_code.rs:86`
- `crates/goose/src/providers/gemini_cli.rs:90`
- `crates/goose/src/providers/gemini_cli.rs:90`

**Example Context:**
```rust
}

        if let Ok(path_var) = std::env::var("PATH") {
            #[cfg(unix)]
            let path_separator = ':';
```

### `PORT`

**Method(s):** env_remove, env_var, env_set

**Usage Locations (6):**

- `crates/goose/src/config/base.rs:1413`
- `crates/goose/src/config/base.rs:1413`
- `crates/goose/src/config/base.rs:1435`
- `crates/goose/src/config/base.rs:1435`
- `crates/goose/src/temporal_scheduler.rs:127`
- `crates/goose/src/temporal_scheduler.rs:127`

**Example Context:**
```rust
// Test number environment variable
        std::env::set_var("PORT", "8080");
        let value: i32 = config.get_param("port")?;
        assert_eq!(value, 8080);
```

### `USER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/agents/agent.rs:1360`
- `crates/goose/src/agents/agent.rs:1360`

**Example Context:**
```rust
let author = Author {
            contact: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .ok(),
```

### `WAYLAND_DISPLAY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44`
- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44`

**Example Context:**
```rust
fn detect_display_server() -> DisplayServer {
        if let Ok(wayland_display) = std::env::var("WAYLAND_DISPLAY") {
            if !wayland_display.is_empty() {
                return DisplayServer::Wayland;
```

## CLI Flags

### `--explain`

**Description:** Show the recipe title, description, and parameters

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:469`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:452`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (2):**

- `crates/goose-cli/src/cli.rs:327`
- `crates/goose-cli/src/cli.rs:483`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:460`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:563`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:476`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:420`

**Example Context:**
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

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:408`

**Example Context:**
```rust
instructions: Option<String>,

        /// Input text containing commands
        #[arg(
            short = 't',
            long = "text",
            value_name = "TEXT",
```

### `-q`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:563`

**Example Context:**
```rust
builtins: Vec<String>,

        /// Quiet mode - suppress non-response output
        #[arg(
            short = 'q',
            long = "quiet",
            help = "Quiet mode. Suppress non-response output, printing only the model response to stdout"
```

### `-s`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:452`

**Example Context:**
```rust
params: Vec<(String, String)>,

        /// Continue in interactive mode after processing input
        #[arg(
            short = 's',
            long = "interactive",
            help = "Continue in interactive mode after processing initial input"
```

### `-t`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:408`

**Example Context:**
```rust
instructions: Option<String>,

        /// Input text containing commands
        #[arg(
            short = 't',
            long = "text",
            value_name = "TEXT",
```

