# Comprehensive Goose Configuration Analysis

## Summary

- **Total Configuration Usages:** 1045
- **Unique Configuration Keys:** 346
- **Files with Configuration:** 101
- **Test-related Usages:** 5

### By Category

- **Environment Variables:** 63 unique keys
- **Config File Parameters:** 269 unique keys
- **Secret Storage:** 21 unique keys
- **CLI Flags:** 11 unique keys

## Config File Parameters

### `$ref`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:72`

**Example Context:**
```rust
// Handle $ref
    if let Some(Value::String(reference)) = obj.get("$ref") {
        return RefOr::Ref(Ref::new(reference.clone()));
    }
```

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

**Method(s):** config_get, config_set

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

**Method(s):** config_get, config_set, env_set, env_remove

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

**Usage Locations (3):**

- `crates/goose-server/src/routes/audio.rs:104`
- `crates/goose-server/src/routes/audio.rs:104`
- `crates/goose/src/providers/openai.rs:63`

**Example Context:**
```rust
// Get the OpenAI host from config (with default)
    let openai_host = match config.get("OPENAI_HOST", false) {
        Ok(value) => value
            .as_str()
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

### `X-Secret-Key`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/routes/utils.rs:29`

**Example Context:**
```rust
// Verify secret key
    let secret_key = headers
        .get("X-Secret-Key")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
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

### `_errors`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:140`

**Example Context:**
```rust
// Specifically look for tool support issues
                    if let Some(tools) = details.get("tools") {
                        if let Some(errors) = tools.get("_errors") {
                            if errors.to_string().contains("not supported by this model") {
                                let model_name = self.model.model_name.clone();
```

### `access_token`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:180`

**Example Context:**
```rust
// Extract access token (required)
        let access_token = token_response
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("access_token not found in token response"))?
```

### `action`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/agent.rs:396`
- `crates/goose/src/agents/schedule_tool.rs:34`

**Example Context:**
```rust
let action = tool_call
                .arguments
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("")
```

### `active`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/export.rs:865`

**Example Context:**
```rust
def process_data(data: List[Dict]) -> List[Dict]:
    """Process a list of data dictionaries."""
    return [item for item in data if item.get('active', False)]"#;

        let text_content = TextContent {
```

### `activities`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/agent.rs:1306`

**Example Context:**
```rust
let activities = json_content
                    .get("activities")
                    .ok_or_else(|| anyhow!("Missing 'activities' in json response"))?
                    .as_array()
```

### `additionalProperties`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:155`

**Example Context:**
```rust
// Handle additional properties
            if let Some(additional) = obj.get("additionalProperties") {
                match additional {
                    Value::Bool(false) => {
```

### `alignment`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:47`

**Example Context:**
```rust
color: obj.get("color").and_then(|v| v.as_str()).map(String::from),
            alignment: obj
                .get("alignment")
                .and_then(|v| v.as_str())
                .and_then(|a| match a {
```

### `allOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:87`

**Example Context:**
```rust
}

    if let Some(Value::Array(all_of)) = obj.get("allOf") {
        let mut builder = AllOfBuilder::new();
        for item in all_of {
```

### `allowSharedDrives`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1991`
- `crates/goose-mcp/src/google_drive/mod.rs:2156`

**Example Context:**
```rust
let allow_shared_drives = params
            .get("allowSharedDrives")
            .and_then(|q| q.as_bool())
            .unwrap_or_default();
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

### `anthropic`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/pricing.rs:426`

**Example Context:**
```rust
// Print debug info
            let all_pricing = get_all_pricing().await;
            if let Some(anthropic_models) = all_pricing.get("anthropic") {
                println!("Available anthropic models in cache:");
                for model_name in anthropic_models.keys() {
```

### `anyOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:97`

**Example Context:**
```rust
}

    if let Some(Value::Array(any_of)) = obj.get("anyOf") {
        let mut builder = AnyOfBuilder::new();
        for item in any_of {
```

### `arg1`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/input.rs:393`
- `crates/goose-cli/src/session/input.rs:445`

**Example Context:**
```rust
assert!(!opts.info);
            assert_eq!(opts.arguments.len(), 2);
            assert_eq!(opts.arguments.get("arg1"), Some(&"val1".to_string()));
            assert_eq!(opts.arguments.get("arg2"), Some(&"val2".to_string()));
        } else {
```

### `arg2`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/input.rs:394`
- `crates/goose-cli/src/session/input.rs:449`

**Example Context:**
```rust
assert_eq!(opts.arguments.len(), 2);
            assert_eq!(opts.arguments.get("arg1"), Some(&"val1".to_string()));
            assert_eq!(opts.arguments.get("arg2"), Some(&"val2".to_string()));
        } else {
            panic!("Expected PromptCommand");
```

### `args`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:263`

**Example Context:**
```rust
content.push(MessageContent::tool_request(id, Err(error)));
            } else {
                let parameters = function_call.get("args");
                if let Some(params) = parameters {
                    content.push(MessageContent::tool_request(
```

### `arguments`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/providers/toolshim.rs:220`
- `crates/mcp-server/src/router.rs:182`
- `crates/mcp-server/src/router.rs:277`

**Example Context:**
```rust
if item.is_object()
                                && item.get("name").is_some()
                                && item.get("arguments").is_some()
                            {
                                // Create ToolCall directly from the JSON data
```

### `authorization_endpoint`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:114`

**Example Context:**
```rust
let authorization_endpoint = oidc_config
        .get("authorization_endpoint")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("authorization_endpoint not found in OIDC configuration"))?
```

### `body`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1987`
- `crates/goose-mcp/src/google_drive/mod.rs:2161`

**Example Context:**
```rust
let parent_id = params.get("parentId").and_then(|q| q.as_str());
        let target_id = params.get("targetId").and_then(|q| q.as_str());
        let body = params.get("body").and_then(|q| q.as_str());
        let path = params.get("path").and_then(|q| q.as_str());
```

### `bold`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:38`

**Example Context:**
```rust
let obj = value.as_object()?;
        Some(Self {
            bold: obj.get("bold").and_then(|v| v.as_bool()).unwrap_or(false),
            italic: obj.get("italic").and_then(|v| v.as_bool()).unwrap_or(false),
            underline: obj
```

### `cache_control`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:897`
- `crates/goose/src/providers/formats/anthropic.rs:910`

**Example Context:**
```rust
// Verify cache control is added to last tool
        assert!(spec[1].get("cache_control").is_some());
    }
```

### `cache_creation_input_tokens`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:277`
- `crates/goose/src/providers/formats/anthropic.rs:314`

**Example Context:**
```rust
let cache_creation_tokens = usage
            .get("cache_creation_input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
```

### `cache_read_input_tokens`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:282`
- `crates/goose/src/providers/formats/anthropic.rs:319`

**Example Context:**
```rust
let cache_read_tokens = usage
            .get("cache_read_input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
```

### `candidates`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:227`

**Example Context:**
```rust
let binding = vec![];
    let candidates: &Vec<Value> = response
        .get("candidates")
        .and_then(|v| v.as_array())
        .unwrap_or(&binding);
```

### `candidatesTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:284`

**Example Context:**
```rust
.map(|v| v as i32);
        let output_tokens = usage_meta_data
            .get("candidatesTokenCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as i32);
```

### `case_sensitive`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:894`

**Example Context:**
```rust
let case_sensitive = params
                    .get("case_sensitive")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
```

### `category`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:56`

**Example Context:**
```rust
if let Ok(args) = serde_json::from_value::<Value>(tool_call.arguments.clone()) {
                                // Check all required parameters match exactly
                                args.get("category").and_then(Value::as_str).is_some_and(|s| s.contains("fact")) &&
                                    args.get("data").and_then(Value::as_str) == Some("The capital of France is Paris.") &&
                                    args.get("is_global").and_then(Value::as_bool) == Some(true)
```

### `cell`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1654`

**Example Context:**
```rust
"update_cell" => {
                let cell = params
                    .get("cell")
                    .and_then(|q| q.as_str())
                    .ok_or(ToolError::InvalidParameters(
```

### `choiceValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2254`

**Example Context:**
```rust
);
                        } else if let Some(choice_value) =
                            op.get("choiceValue").and_then(|o| o.as_array())
                        {
                            field_mods.set_selection_values = Some(
```

### `choices`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:122`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:88`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:88`
- `crates/goose/src/providers/formats/snowflake.rs:151`
- `crates/goose/src/providers/snowflake.rs:144`

**Example Context:**
```rust
// Extract the content from the response
        let content = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
```

### `claude-3-5-sonnet-latest`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:525`

**Example Context:**
```rust
// claude-3-5-sonnet-latest should have 200k limit
        assert_eq!(
            *model_info.get("claude-3-5-sonnet-latest").unwrap(),
            200_000
        );
```

### `code`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/oauth.rs:306`
- `crates/goose/src/providers/openrouter.rs:91`
- `crates/goose/src/providers/snowflake.rs:110`
- `crates/goose/src/providers/snowflake.rs:112`
- `crates/goose/src/providers/utils.rs:188`

**Example Context:**
```rust
let state = state.clone();
                async move {
                    let code = params.get("code").cloned();
                    let received_state = params.get("state").cloned();
```

### `code_edit`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/export.rs:160`

**Example Context:**
```rust
md.push_str(&format!("*   **path**: `{}`\n", path));
                    }
                    if let Some(Value::String(code_edit)) = call.arguments.get("code_edit") {
                        md.push_str(&format!(
                            "*   **code_edit**:\n    ```\n{}\n    ```\n",
```

### `col`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:949`

**Example Context:**
```rust
})?;

                let col = params.get("col").and_then(|v| v.as_u64()).ok_or_else(|| {
                    ToolError::InvalidParameters("Missing 'col' parameter".into())
                })?;
```

### `color`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:45`

**Example Context:**
```rust
.unwrap_or(false),
            size: obj.get("size").and_then(|v| v.as_u64()).map(|s| s as usize),
            color: obj.get("color").and_then(|v| v.as_str()).map(String::from),
            alignment: obj
                .get("alignment")
```

### `command`

**Method(s):** config_get

**Usage Locations (12):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:56`
- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:87`
- `crates/goose-bench/src/eval_suites/core/developer/list_files.rs:49`
- `crates/goose-bench/src/eval_suites/vibes/flappy_bird.rs:65`
- `crates/goose-bench/src/eval_suites/vibes/goose_wiki.rs:75`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:86`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:116`
- `crates/goose-cli/src/session/export.rs:132`
- `crates/goose-cli/src/session/output.rs:416`
- `crates/goose-mcp/src/computercontroller/mod.rs:1012`
- ... and 2 more locations

**Example Context:**
```rust
if let Ok(args) = serde_json::from_value::<Value>(tool_call.arguments.clone()) {
                            // Check all required parameters match exactly
                            args.get("command").and_then(Value::as_str) == Some("write") &&
                            args.get("path").and_then(Value::as_str).is_some_and(|s| s.contains("test.txt")) &&
                            args.get("file_text").and_then(Value::as_str) == Some("Hello, World!")
```

### `command_parameters`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:34`

**Example Context:**
```rust
pub fn get_command_parameters(&self) -> Option<&Map<String, Value>> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("command_parameters"))
            .and_then(|cp| cp.as_object())
    }
```

### `commentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2520`

**Example Context:**
```rust
}
            "reply" => {
                let comment_id = params.get("commentId").and_then(|q| q.as_str()).ok_or(
                    ToolError::InvalidParameters(
                        "The commentId param is required for reply".to_string(),
```

### `completion_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:342`

**Example Context:**
```rust
let output_tokens = usage
        .get("completion_tokens")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
```

### `complex_key`

**Method(s):** config_get, config_set

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

### `content`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-cli/src/recipes/github_recipe.rs:323`
- `crates/goose-mcp/src/computercontroller/mod.rs:990`
- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:125`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:91`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:91`
- `crates/goose-mcp/src/google_drive/mod.rs:2482`
- `crates/goose/src/providers/claude_code.rs:213`
- `crates/goose/src/providers/formats/databricks.rs:291`
- `crates/goose/src/providers/formats/databricks.rs:330`
- `crates/goose/src/providers/formats/google.rs:238`
- ... and 6 more locations

**Example Context:**
```rust
.map_err(|e| anyhow!("Failed to parse file info: {}", e))?;

    if let Some(content_b64) = file_info.get("content").and_then(|c| c.as_str()) {
        // Decode base64 content
        use base64::{engine::general_purpose, Engine as _};
```

### `content_block`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:522`

**Example Context:**
```rust
"content_block_start" => {
                    // A new content block started
                    if let Some(content_block) = event.data.get("content_block") {
                        if content_block.get("type") == Some(&json!("tool_use")) {
                            if let Some(id) = content_block.get("id").and_then(|v| v.as_str()) {
```

### `content_list`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/snowflake.rs:204`
- `crates/goose/src/providers/snowflake.rs:169`

**Example Context:**
```rust
let mut message = Message::assistant();

    let content_list = response.get("content_list").and_then(|cl| cl.as_array());

    // Handle case where content_list is missing or empty
```

### `context`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:152`

**Example Context:**
```rust
// Check for specific error message in context.issues
                if let Some(context) = json.get("context") {
                    if let Some(issues) = context.get("issues") {
                        if let Some(issues_array) = issues.as_array() {
```

### `corpora`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1014`

**Example Context:**
```rust
// extract corpora query parameter, validate options, or default to "user"
        let corpus = params
            .get("corpora")
            .and_then(|c| c.as_str())
            .map(|s| {
```

### `cron_expression`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:92`

**Example Context:**
```rust
let cron_expression = arguments
            .get("cron_expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
```

### `currentFolderId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2111`

**Example Context:**
```rust
))?;
        let current_folder_id = params
            .get("currentFolderId")
            .and_then(|q| q.as_str())
            .ok_or(ToolError::InvalidParameters(
```

### `cursor`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1848`

**Example Context:**
```rust
async fn list_google_resources(&self, params: Value) -> Vec<Resource> {
        let next_page_token = params.get("cursor").and_then(|q| q.as_str());

        let mut query = self
```

### `data`

**Method(s):** config_get

**Usage Locations (8):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:57`
- `crates/goose-mcp/src/memory/mod.rs:592`
- `crates/goose/src/providers/formats/databricks.rs:317`
- `crates/goose/src/providers/formats/snowflake.rs:264`
- `crates/goose/src/providers/githubcopilot.rs:455`
- `crates/goose/src/providers/groq.rs:120`
- `crates/goose/src/providers/openai.rs:187`
- `crates/goose/src/providers/openrouter.rs:297`

**Example Context:**
```rust
// Check all required parameters match exactly
                                args.get("category").and_then(Value::as_str).is_some_and(|s| s.contains("fact")) &&
                                    args.get("data").and_then(Value::as_str) == Some("The capital of France is Paris.") &&
                                    args.get("is_global").and_then(Value::as_bool) == Some(true)
                            } else {
```

### `dateValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2229`

**Example Context:**
```rust
}

                        if let Some(date_value) = op.get("dateValue").and_then(|o| o.as_array()) {
                            let parsed_dates: Result<Vec<NaiveDate>, ToolError> = date_value
                                .iter()
```

### `delta`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/providers/formats/anthropic.rs:535`
- `crates/goose/src/providers/formats/snowflake.rs:153`
- `crates/goose/src/providers/snowflake.rs:158`

**Example Context:**
```rust
}
                "content_block_delta" => {
                    if let Some(delta) = event.data.get("delta") {
                        if delta.get("type") == Some(&json!("text_delta")) {
                            // Text content delta
```

### `details`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:137`

**Example Context:**
```rust
// Check for tool support errors
                if let Some(details) = json.get("details") {
                    // Specifically look for tool support issues
                    if let Some(tools) = details.get("tools") {
```

### `display`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/developer_image/image.rs:53`
- `crates/goose-mcp/src/developer/mod.rs:1459`

**Example Context:**
```rust
{
                                if tool_call.name == "developer__screen_capture"
                                    && (args.get("display").and_then(Value::as_i64) == Some(0))
                                {
                                    valid_tool_call = true;
```

### `documentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2571`

**Example Context:**
```rust
async fn docs_tool(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let document_id = params.get("documentId").and_then(|q| q.as_str()).ok_or(
            ToolError::InvalidParameters("The documentId is required".to_string()),
        )?;
```

### `driveId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1009`

**Example Context:**
```rust
let name = params.get("name").and_then(|q| q.as_str());
        let mime_type = params.get("mimeType").and_then(|q| q.as_str());
        let drive_id = params.get("driveId").and_then(|q| q.as_str());
        let parent = params.get("parent").and_then(|q| q.as_str());
```

### `driveType`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:993`

**Example Context:**
```rust
// To minimize tool growth, we search/list for a number of different
        // objects in Gdrive with sub-funcs.
        let drive_type = params.get("driveType").and_then(|q| q.as_str()).ok_or(
            ToolError::InvalidParameters("The type is required".to_string()),
        )?;
```

### `emailMessage`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3096`

**Example Context:**
```rust
).transpose()?;
        let target = params.get("target").and_then(|s| s.as_str());
        let email_message = params.get("emailMessage").and_then(|s| s.as_str());

        match operation {
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

### `endPosition`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2894`

**Example Context:**
```rust
)?;

                let end_position = params.get("endPosition").and_then(|q| q.as_i64()).ok_or(
                    ToolError::InvalidParameters("The endPosition parameter is required for delete_content operation".to_string()),
                )?;
```

### `endpoints`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:367`

**Example Context:**
```rust
};

        let endpoints = match json.get("endpoints").and_then(|v| v.as_array()) {
            Some(endpoints) => endpoints,
            None => {
```

### `error`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/providers/anthropic.rs:102`
- `crates/goose/src/providers/openai.rs:179`
- `crates/goose/src/providers/openrouter.rs:84`
- `crates/goose/src/providers/openrouter.rs:288`
- `crates/goose/src/providers/utils.rs:88`
- `crates/goose/src/providers/utils.rs:187`
- `crates/goose/src/providers/utils.rs:227`

**Example Context:**
```rust
.payload
                        .as_ref()
                        .and_then(|p| p.get("error"))
                        .and_then(|e| e.get("message"))
                        .and_then(|m| m.as_str())
```

### `exclusiveMaximum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:258`
- `crates/goose-server/src/openapi.rs:289`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(exclusive_maximum)) = obj.get("exclusiveMaximum") {
                if let Some(max) = exclusive_maximum.as_f64() {
                    object_builder = object_builder.exclusive_maximum(Some(max));
```

### `exclusiveMinimum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:253`
- `crates/goose-server/src/openapi.rs:284`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(exclusive_minimum)) = obj.get("exclusiveMinimum") {
                if let Some(min) = exclusive_minimum.as_f64() {
                    object_builder = object_builder.exclusive_minimum(Some(min));
```

### `execution_mode`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/schedule_tool.rs:100`
- `crates/goose/src/agents/subagent_execution_tool/subagent_execute_task_tool.rs:75`

**Example Context:**
```rust
// Get the execution_mode parameter, defaulting to "background" if not provided
        let execution_mode = arguments
            .get("execution_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("background");
```

### `experiments`

**Method(s):** config_get, config_set

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

### `expires_in`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:194`

**Example Context:**
```rust
// Handle token expiration
        let expires_at =
            if let Some(expires_in) = token_response.get("expires_in").and_then(|v| v.as_u64()) {
                // Traditional OAuth flow with expires_in seconds
                Some(Utc::now() + chrono::Duration::seconds(expires_in as i64))
```

### `extension`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/extension_manager.rs:672`

**Example Context:**
```rust
cancellation_token: CancellationToken,
    ) -> Result<Vec<Content>, ToolError> {
        let extension = params.get("extension").and_then(|v| v.as_str());

        match extension {
```

### `extension_name`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/agents/agent.rs:390`
- `crates/goose/src/agents/extension_manager.rs:551`
- `crates/goose/src/agents/router_tool_selector.rs:84`
- `crates/goose/src/agents/router_tool_selector.rs:265`

**Example Context:**
```rust
let extension_name = tool_call
                .arguments
                .get("extension_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
```

### `extensions`

**Method(s):** config_get, config_set

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

### `fieldId`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:2205`
- `crates/goose-mcp/src/google_drive/mod.rs:2225`

**Example Context:**
```rust
}
                    Some("unsetField") => {
                        let field_id = op.get("fieldId").and_then(|o| o.as_str()).ok_or(
                            ToolError::InvalidParameters(
                                "The fieldId param is required for unsetting a field.".to_string(),
```

### `fileId`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose-mcp/src/google_drive/mod.rs:2105`
- `crates/goose-mcp/src/google_drive/mod.rs:2149`
- `crates/goose-mcp/src/google_drive/mod.rs:2407`
- `crates/goose-mcp/src/google_drive/mod.rs:2472`
- `crates/goose-mcp/src/google_drive/mod.rs:3017`
- `crates/goose-mcp/src/google_drive/mod.rs:3068`

**Example Context:**
```rust
let file_id =
            params
                .get("fileId")
                .and_then(|q| q.as_str())
                .ok_or(ToolError::InvalidParameters(
```

### `file_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:58`

**Example Context:**
```rust
args.get("command").and_then(Value::as_str) == Some("write") &&
                            args.get("path").and_then(Value::as_str).is_some_and(|s| s.contains("test.txt")) &&
                            args.get("file_text").and_then(Value::as_str) == Some("Hello, World!")
                        } else {
                            false
```

### `format`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:234`

**Example Context:**
```rust
object_builder = object_builder.pattern(Some(pattern.clone()));
            }
            if let Some(Value::String(format)) = obj.get("format") {
                object_builder = object_builder.format(Some(SchemaFormat::Custom(format.clone())));
            }
```

### `functionCall`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:246`

**Example Context:**
```rust
if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
            content.push(MessageContent::text(text.to_string()));
        } else if let Some(function_call) = part.get("functionCall") {
            let id: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
```

### `generated_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/sagemaker_tgi.rs:197`

**Example Context:**
```rust
let first_result = &response_array[0];
        let generated_text = first_result
            .get("generated_text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
```

### `gpt-4o`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:521`

**Example Context:**
```rust
// gpt-4o should have 128k limit
        assert_eq!(*model_info.get("gpt-4o").unwrap(), 128_000);

        // claude-3-5-sonnet-latest should have 200k limit
```

### `height`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:229`

**Example Context:**
```rust
let height = params
                            .get("height")
                            .and_then(|v| v.as_u64())
                            .map(|h| h as u32);
```

### `host`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/config/base.rs:1329`

**Example Context:**
```rust
if let Value::Object(obj) = value {
            assert_eq!(
                obj.get("host"),
                Some(&Value::String("localhost".to_string()))
            );
```

### `https://openrouter.ai/api/v1/models`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/pricing.rs:279`

**Example Context:**
```rust
let client = create_http_client();
    let response = client
        .get("https://openrouter.ai/api/v1/models")
        .send()
        .await?;
```

### `id`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/providers/anthropic.rs:209`
- `crates/goose/src/providers/formats/anthropic.rs:501`
- `crates/goose/src/providers/formats/anthropic.rs:524`
- `crates/goose/src/providers/githubcopilot.rs:465`
- `crates/goose/src/providers/groq.rs:128`
- `crates/goose/src/providers/openai.rs:192`
- `crates/goose/src/providers/openrouter.rs:305`

**Example Context:**
```rust
Some(s.to_string())
                } else if let Some(obj) = m.as_object() {
                    obj.get("id").and_then(|v| v.as_str()).map(str::to_string)
                } else {
                    None
```

### `image_path`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:214`

**Example Context:**
```rust
"add_image" => {
                        let image_path = params
                            .get("image_path")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| {
```

### `includeImages`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1419`

**Example Context:**
```rust
let include_images = params
            .get("includeImages")
            .and_then(|i| i.as_bool())
            .unwrap_or(false);
```

### `includeLabels`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1049`

**Example Context:**
```rust
let include_labels = params
            .get("includeLabels")
            .and_then(|b| b.as_bool())
            .unwrap_or(false);
```

### `input`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose/src/providers/databricks.rs:207`
- `crates/goose/src/providers/formats/snowflake.rs:167`
- `crates/goose/src/providers/formats/snowflake.rs:244`
- `crates/goose/src/providers/snowflake.rs:192`
- `crates/goose/src/providers/snowflake.rs:210`
- `crates/goose/src/tracing/observation_layer.rs:202`

**Example Context:**
```rust
async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let is_embedding = payload.get("input").is_some() && payload.get("messages").is_none();
        let path = self.get_endpoint_path(is_embedding);
```

### `input_tokens`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/claude_code.rs:234`
- `crates/goose/src/providers/claude_code.rs:258`
- `crates/goose/src/providers/formats/anthropic.rs:272`
- `crates/goose/src/providers/formats/anthropic.rs:309`
- `crates/goose/src/providers/formats/snowflake.rs:283`

**Example Context:**
```rust
if let Some(usage_info) = message.get("usage") {
                                usage.input_tokens = usage_info
                                    .get("input_tokens")
                                    .and_then(|v| v.as_i64())
                                    .map(|v| v as i32);
```

### `insert_line`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:820`

**Example Context:**
```rust
"insert" => {
                let insert_line = params
                    .get("insert_line")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| {
```

### `instructions`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/agent.rs:1299`

**Example Context:**
```rust
if let Ok(json_content) = serde_json::from_str::<Value>(&clean_content) {
                let instructions = json_content
                    .get("instructions")
                    .ok_or_else(|| anyhow!("Missing 'instructions' in json response"))?
                    .as_str()
```

### `integerValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2263`

**Example Context:**
```rust
);
                        } else if let Some(int_value) =
                            op.get("integerValue").and_then(|o| o.as_array())
                        {
                            field_mods.set_integer_values = Some(
```

### `is_global`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:58`
- `crates/goose-mcp/src/memory/mod.rs:600`

**Example Context:**
```rust
args.get("category").and_then(Value::as_str).is_some_and(|s| s.contains("fact")) &&
                                    args.get("data").and_then(Value::as_str) == Some("The capital of France is Paris.") &&
                                    args.get("is_global").and_then(Value::as_bool) == Some(true)
                            } else {
                                false
```

### `issues`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:153`

**Example Context:**
```rust
// Check for specific error message in context.issues
                if let Some(context) = json.get("context") {
                    if let Some(issues) = context.get("issues") {
                        if let Some(issues_array) = issues.as_array() {
                            for issue in issues_array {
```

### `italic`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:39`

**Example Context:**
```rust
Some(Self {
            bold: obj.get("bold").and_then(|v| v.as_bool()).unwrap_or(false),
            italic: obj.get("italic").and_then(|v| v.as_bool()).unwrap_or(false),
            underline: obj
                .get("underline")
```

### `items`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:181`

**Example Context:**
```rust
// Add items schema
            if let Some(items) = obj.get("items") {
                match items {
                    Value::Object(_) | Value::Bool(_) => {
```

### `job_id`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/agents/schedule_tool.rs:175`
- `crates/goose/src/agents/schedule_tool.rs:198`
- `crates/goose/src/agents/schedule_tool.rs:221`
- `crates/goose/src/agents/schedule_tool.rs:244`
- `crates/goose/src/agents/schedule_tool.rs:267`
- `crates/goose/src/agents/schedule_tool.rs:290`
- `crates/goose/src/agents/schedule_tool.rs:320`

**Example Context:**
```rust
) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;
```

### `key`

**Method(s):** config_set, config_get, secret_delete, config_delete, secret_set, secret_get

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

**Method(s):** secret_get, secret_delete, secret_set, config_set

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

**Method(s):** secret_get, secret_set, config_set

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

### `labelId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2190`

**Example Context:**
```rust
for op in label_ops {
            if let Some(op) = op.as_object() {
                let label_id = op.get("labelId").and_then(|o| o.as_str()).ok_or(
                    ToolError::InvalidParameters(
                        "The labelId param is required for label changes".to_string(),
```

### `language`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:670`

**Example Context:**
```rust
async fn quick_script(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let language = params
            .get("language")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidParameters("Missing 'language' parameter".into()))?;
```

### `level`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:204`

**Example Context:**
```rust
"structured" => {
                        let level = params
                            .get("level")
                            .and_then(|v| v.as_str())
                            .map(String::from);
```

### `limit`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:325`

**Example Context:**
```rust
let limit = arguments
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;
```

### `location`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/scenario_tests/mock_client.rs:155` (test)

### `maxItems`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:210`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(max_items)) = obj.get("maxItems") {
                if let Some(max) = max_items.as_u64() {
                    array_builder = array_builder.max_items(Some(max as usize));
```

### `maxLength`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:226`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(max_length)) = obj.get("maxLength") {
                if let Some(max) = max_length.as_u64() {
                    object_builder = object_builder.max_length(Some(max as usize));
```

### `maximum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:248`
- `crates/goose-server/src/openapi.rs:279`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(maximum)) = obj.get("maximum") {
                if let Some(max) = maximum.as_f64() {
                    object_builder = object_builder.maximum(Some(max));
```

### `message`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-bench/src/error_capture.rs:47`
- `crates/goose-cli/src/session/mod.rs:1104`
- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:124`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:90`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:90`
- `crates/goose/src/providers/anthropic.rs:103`
- `crates/goose/src/providers/claude_code.rs:211`
- `crates/goose/src/providers/formats/anthropic.rs:499`
- `crates/goose/src/providers/openai.rs:181`
- `crates/goose/src/providers/openrouter.rs:87`
- ... and 6 more locations

**Example Context:**
```rust
event.record(&mut visitor);

            if let Some(message) = visitor.recorded_fields.get("message") {
                let error = BenchAgentError {
                    message: message.to_string(),
```

### `messages`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:207`

**Example Context:**
```rust
async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let is_embedding = payload.get("input").is_some() && payload.get("messages").is_none();
        let path = self.get_endpoint_path(is_embedding);
```

### `metadata`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/tracing/observation_layer.rs:223`

**Example Context:**
```rust
if !remaining_metadata.is_empty() {
                let flattened = flatten_metadata(remaining_metadata);
                if update.get("metadata").is_some() {
                    // If metadata exists (from model_config), merge with it
                    if let Some(obj) = update["metadata"].as_object_mut() {
```

### `mimeType`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-mcp/src/google_drive/mod.rs:1008`
- `crates/goose-mcp/src/google_drive/mod.rs:1979`
- `crates/goose-mcp/src/google_drive/mod.rs:2160`

**Example Context:**
```rust
async fn search_files(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let name = params.get("name").and_then(|q| q.as_str());
        let mime_type = params.get("mimeType").and_then(|q| q.as_str());
        let drive_id = params.get("driveId").and_then(|q| q.as_str());
        let parent = params.get("parent").and_then(|q| q.as_str());
```

### `minItems`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:205`

**Example Context:**
```rust
// Add constraints
            if let Some(Value::Number(min_items)) = obj.get("minItems") {
                if let Some(min) = min_items.as_u64() {
                    array_builder = array_builder.min_items(Some(min as usize));
```

### `minLength`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:221`

**Example Context:**
```rust
let mut object_builder = ObjectBuilder::new().schema_type(SchemaType::String);

            if let Some(Value::Number(min_length)) = obj.get("minLength") {
                if let Some(min) = min_length.as_u64() {
                    object_builder = object_builder.min_length(Some(min as usize));
```

### `minimum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:243`
- `crates/goose-server/src/openapi.rs:274`

**Example Context:**
```rust
let mut object_builder = ObjectBuilder::new().schema_type(SchemaType::Number);

            if let Some(Value::Number(minimum)) = obj.get("minimum") {
                if let Some(min) = minimum.as_f64() {
                    object_builder = object_builder.minimum(Some(min));
```

### `mode`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:181`

**Example Context:**
```rust
let (mode, style) = if let Some(params) = params {
                let mode = params
                    .get("mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("append");
```

### `model`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose/src/providers/formats/anthropic.rs:509`
- `crates/goose/src/providers/formats/anthropic.rs:629`
- `crates/goose/src/providers/formats/anthropic.rs:648`
- `crates/goose/src/providers/githubcopilot.rs:141`
- `crates/goose/src/providers/utils.rs:172`
- `crates/goose/src/providers/utils.rs:267`

**Example Context:**
```rust
tracing::debug!("🔍 Anthropic message_start parsed usage: input_tokens={:?}, output_tokens={:?}, total_tokens={:?}",
                                    usage.input_tokens, usage.output_tokens, usage.total_tokens);
                            let model = message_data.get("model")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
```

### `modelVersion`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/google.rs:126`

**Example Context:**
```rust
let message = response_to_message(unescape_json_values(&response))?;
        let usage = get_usage(&response)?;
        let model = match response.get("modelVersion") {
            Some(model_version) => model_version.as_str().unwrap_or_default().to_string(),
            None => self.model.model_name.clone(),
```

### `model_config`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/tracing/observation_layer.rs:210`

**Example Context:**
```rust
}

            if let Some(val) = metadata.get("model_config") {
                update["metadata"] = json!({ "model_config": val });
            }
```

### `models`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/anthropic.rs:198`
- `crates/goose/src/providers/google.rs:139`

**Example Context:**
```rust
let json = response.payload.unwrap_or_default();
        let arr = match json.get("models").and_then(|v| v.as_array()) {
            Some(arr) => arr,
            None => return Ok(None),
```

### `multipleOf`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:263`
- `crates/goose-server/src/openapi.rs:294`

**Example Context:**
```rust
}
            }
            if let Some(Value::Number(multiple_of)) = obj.get("multipleOf") {
                if let Some(mult) = multiple_of.as_f64() {
                    object_builder = object_builder.multiple_of(Some(mult));
```

### `name`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-cli/src/recipes/github_recipe.rs:249`
- `crates/goose-cli/src/recipes/github_recipe.rs:284`
- `crates/goose-mcp/src/google_drive/mod.rs:1007`
- `crates/goose-mcp/src/google_drive/mod.rs:1971`
- `crates/goose-mcp/src/tutorial/mod.rs:137`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:46`
- `crates/goose/src/providers/databricks.rs:381`
- `crates/goose/src/providers/formats/anthropic.rs:526`
- `crates/goose/src/providers/formats/snowflake.rs:164`
- `crates/goose/src/providers/formats/snowflake.rs:239`
- ... and 6 more locations

**Example Context:**
```rust
for item in items {
            if let (Some(name), Some(item_type)) = (
                item.get("name").and_then(|n| n.as_str()),
                item.get("type").and_then(|t| t.as_str()),
            ) {
```

### `name_contains`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2947`

**Example Context:**
```rust
async fn list_drives(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let query = params.get("name_contains").and_then(|q| q.as_str());

        let mut results: Vec<String> = Vec::new();
```

### `newFolderId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2116`

**Example Context:**
```rust
"The currentFolderId param is required".to_string(),
            ))?;
        let new_folder_id = params.get("newFolderId").and_then(|q| q.as_str()).ok_or(
            ToolError::InvalidParameters("The newFolderId param is required".to_string()),
        )?;
```

### `new_str`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/mod.rs:810`
- `crates/goose-mcp/src/developer/mod.rs:826`

**Example Context:**
```rust
})?;
                let new_str = params
                    .get("new_str")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
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

### `old_str`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:804`

**Example Context:**
```rust
"str_replace" | "edit_file" => {
                let old_str = params
                    .get("old_str")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
```

### `old_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:191`

**Example Context:**
```rust
let old_text =
                            params
                                .get("old_text")
                                .and_then(|v| v.as_str())
                                .ok_or_else(|| {
```

### `oneOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:77`

**Example Context:**
```rust
// Handle oneOf, allOf, anyOf
    if let Some(Value::Array(one_of)) = obj.get("oneOf") {
        let mut builder = OneOfBuilder::new();
        for item in one_of {
```

### `operation`

**Method(s):** config_get

**Usage Locations (9):**

- `crates/goose-mcp/src/computercontroller/mod.rs:833`
- `crates/goose-mcp/src/computercontroller/mod.rs:983`
- `crates/goose-mcp/src/computercontroller/mod.rs:1003`
- `crates/goose-mcp/src/google_drive/mod.rs:1447`
- `crates/goose-mcp/src/google_drive/mod.rs:2195`
- `crates/goose-mcp/src/google_drive/mod.rs:2292`
- `crates/goose-mcp/src/google_drive/mod.rs:2477`
- `crates/goose-mcp/src/google_drive/mod.rs:2575`
- `crates/goose-mcp/src/google_drive/mod.rs:3073`

**Example Context:**
```rust
let operation = params
            .get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidParameters("Missing 'operation' parameter".into()))?;
```

### `output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/mod.rs:1146`
- `crates/goose/src/tracing/observation_layer.rs:206`

**Example Context:**
```rust
};
                                                (formatted, subagent_id.map(str::to_string), notification_type.map(str::to_string))
                                            } else if let Some(Value::String(output)) = o.get("output") {
                                                // Fallback for other MCP notification types
                                                (output.to_owned(), None, None)
```

### `output_tokens`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/claude_code.rs:238`
- `crates/goose/src/providers/claude_code.rs:264`
- `crates/goose/src/providers/formats/anthropic.rs:287`
- `crates/goose/src/providers/formats/anthropic.rs:324`
- `crates/goose/src/providers/formats/snowflake.rs:288`

**Example Context:**
```rust
.map(|v| v as i32);
                                usage.output_tokens = usage_info
                                    .get("output_tokens")
                                    .and_then(|v| v.as_i64())
                                    .map(|v| v as i32);
```

### `pageSize`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1030`

**Example Context:**
```rust
// extract pageSize, and convert it to an i32, default to 10
        let page_size: i32 = params
            .get("pageSize")
            .map(|s| {
                s.as_i64()
```

### `params`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:991`

**Example Context:**
```rust
operation,
            params.get("content").and_then(|v| v.as_str()),
            params.get("params"),
        )
        .await
```

### `parent`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1010`

**Example Context:**
```rust
let mime_type = params.get("mimeType").and_then(|q| q.as_str());
        let drive_id = params.get("driveId").and_then(|q| q.as_str());
        let parent = params.get("parent").and_then(|q| q.as_str());

        // extract corpora query parameter, validate options, or default to "user"
```

### `parentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1985`

**Example Context:**
```rust
))?;

        let parent_id = params.get("parentId").and_then(|q| q.as_str());
        let target_id = params.get("targetId").and_then(|q| q.as_str());
        let body = params.get("body").and_then(|q| q.as_str());
```

### `partial_json`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:553`

**Example Context:**
```rust
// Tool input delta
                            if let Some(tool_id) = &current_tool_id {
                                if let Some(partial_json) = delta.get("partial_json").and_then(|v| v.as_str()) {
                                    if let Some((_name, args)) = accumulated_tool_calls.get_mut(tool_id) {
                                        args.push_str(partial_json);
```

### `partial_output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/task_execution_display/mod.rs:24`
- `crates/goose/src/agents/subagent_execution_tool/lib/mod.rs:79`

**Example Context:**
```rust
Value::String(s) => s.to_string(),
        Value::Object(obj) => {
            if let Some(partial_output) = obj.get("partial_output").and_then(|v| v.as_str()) {
                format!("Partial output: {}", partial_output)
            } else {
```

### `parts`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:239`

**Example Context:**
```rust
let parts = candidate
        .get("content")
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.as_array())
        .unwrap_or(&binding);
```

### `path`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:57`
- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:88`
- `crates/goose-bench/src/eval_suites/vibes/flappy_bird.rs:67`
- `crates/goose-bench/src/eval_suites/vibes/goose_wiki.rs:77`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:88`
- `crates/goose-cli/src/session/export.rs:157`
- `crates/goose-cli/src/session/output.rs:392`
- `crates/goose-mcp/src/computercontroller/mod.rs:828`
- `crates/goose-mcp/src/computercontroller/mod.rs:978`
- `crates/goose-mcp/src/computercontroller/mod.rs:998`
- ... and 6 more locations

**Example Context:**
```rust
// Check all required parameters match exactly
                            args.get("command").and_then(Value::as_str) == Some("write") &&
                            args.get("path").and_then(Value::as_str).is_some_and(|s| s.contains("test.txt")) &&
                            args.get("file_text").and_then(Value::as_str) == Some("Hello, World!")
                        } else {
```

### `pattern`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:231`

**Example Context:**
```rust
}
            }
            if let Some(Value::String(pattern)) = obj.get("pattern") {
                object_builder = object_builder.pattern(Some(pattern.clone()));
            }
```

### `permissionId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3076`

**Example Context:**
```rust
ToolError::InvalidParameters("The operation is required".to_string()),
        )?;
        let permission_id = params.get("permissionId").and_then(|q| q.as_str());
        let role = params.get("role").and_then(|s| {
            s.as_str().map(|s| {
```

### `port`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1332`
- `crates/goose/src/config/base.rs:1414`
- `crates/goose/src/config/base.rs:1414`

**Example Context:**
```rust
Some(&Value::String("localhost".to_string()))
            );
            assert_eq!(obj.get("port"), Some(&Value::Number(8080.into())));
        }
```

### `position`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2631`

**Example Context:**
```rust
)?;

                let position = params.get("position").and_then(|q| q.as_i64()).ok_or(
                    ToolError::InvalidParameters("The position parameter is required for insert_text operation".to_string()),
                )?;
```

### `promptTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:280`

**Example Context:**
```rust
if let Some(usage_meta_data) = data.get("usageMetadata") {
        let input_tokens = usage_meta_data
            .get("promptTokenCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as i32);
```

### `prompt_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:337`

**Example Context:**
```rust
pub fn get_usage(usage: &Value) -> Usage {
    let input_tokens = usage
        .get("prompt_tokens")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
```

### `properties`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose-server/src/openapi.rs:136`
- `crates/goose/src/agents/extension_manager.rs:96`
- `crates/goose/src/providers/formats/google.rs:139`
- `crates/goose/src/providers/formats/google.rs:694`

**Example Context:**
```rust
// Add properties
            if let Some(Value::Object(properties)) = obj.get("properties") {
                for (name, prop_value) in properties {
                    if let Ok(prop_schema) = rmcp::schemars::Schema::try_from(prop_value.clone()) {
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

### `query`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/router_tool_selector.rs:77`
- `crates/goose/src/agents/router_tool_selector.rs:260`

**Example Context:**
```rust
async fn select_tools(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let query = params
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidParameters("Missing 'query' parameter".to_string()))?;
```

### `quoted`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:464`

**Example Context:**
```rust
assert_eq!(opts.arguments.get("simple"), Some(&"value".to_string()));
            assert_eq!(
                opts.arguments.get("quoted"),
                Some(&r#"value with "nested" quotes"#.to_string())
            );
```

### `range`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose-mcp/src/computercontroller/mod.rs:864`
- `crates/goose-mcp/src/google_drive/mod.rs:1537`
- `crates/goose-mcp/src/google_drive/mod.rs:1582`
- `crates/goose-mcp/src/google_drive/mod.rs:1787`

**Example Context:**
```rust
"get_range" => {
                let range = params
                    .get("range")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
```

### `read_only_tools`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/permission/permission_judge.rs:108`

**Example Context:**
```rust
if let Value::Object(arguments) = &tool_call.arguments {
                        if let Some(Value::Array(read_only_tools)) =
                            arguments.get("read_only_tools")
                        {
                            return Some(
```

### `recipe`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/recipe/mod.rs:291`
- `crates/goose/src/recipe/mod.rs:297`

**Example Context:**
```rust
let recipe: Recipe =
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(content) {
                if let Some(nested_recipe) = json_value.get("recipe") {
                    serde_json::from_value(nested_recipe.clone())?
                } else {
```

### `recipe_path`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/schedule_tool.rs:85`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:52`

**Example Context:**
```rust
) -> ToolResult<Vec<Content>> {
        let recipe_path = arguments
            .get("recipe_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
```

### `refresh_token`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:187`

**Example Context:**
```rust
// Extract refresh token if available
        let refresh_token = token_response
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
```

### `replaceText`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2757`

**Example Context:**
```rust
)?;

                let replace_text = params.get("replaceText").and_then(|q| q.as_str()).ok_or(
                    ToolError::InvalidParameters("The replaceText parameter is required for replace_text operation".to_string()),
                )?;
```

### `required`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:146`

**Example Context:**
```rust
// Add required fields
            if let Some(Value::Array(required)) = obj.get("required") {
                for req in required {
                    if let Value::String(field_name) = req {
```

### `resolveComment`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2527`

**Example Context:**
```rust
let resolve_comment = params
                    .get("resolveComment")
                    .and_then(|q| q.as_bool())
                    .unwrap_or(false);
```

### `role`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/google_drive/mod.rs:3077`
- `crates/goose/src/providers/litellm.rs:276`
- `crates/goose/src/providers/litellm.rs:295`
- `crates/goose/src/providers/openrouter.rs:131`
- `crates/goose/src/providers/openrouter.rs:151`

**Example Context:**
```rust
)?;
        let permission_id = params.get("permissionId").and_then(|q| q.as_str());
        let role = params.get("role").and_then(|s| {
            s.as_str().map(|s| {
                if ROLES.contains(&s) {
```

### `row`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:945`

**Example Context:**
```rust
}
            "get_cell" => {
                let row = params.get("row").and_then(|v| v.as_u64()).ok_or_else(|| {
                    ToolError::InvalidParameters("Missing 'row' parameter".into())
                })?;
```

### `save_as`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:602`

**Example Context:**
```rust
let url = require_str_parameter(&params, "url")?;
        let save_as = params
            .get("save_as")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
```

### `save_output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/computercontroller/mod.rs:680`
- `crates/goose-mcp/src/computercontroller/mod.rs:800`

**Example Context:**
```rust
let save_output = params
            .get("save_output")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
```

### `script`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-bench/src/eval_suites/core/computercontroller/script.rs:53`
- `crates/goose-mcp/src/computercontroller/mod.rs:675`
- `crates/goose-mcp/src/computercontroller/mod.rs:795`

**Example Context:**
```rust
if let Ok(args) = serde_json::from_value::<Value>(tool_call.arguments.clone()) {
                            // Check all required parameters match exactly
                            args.get("script").and_then(Value::as_str).is_some_and(|s| s.contains("beep"))
                        } else {
                            false
```

### `search_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:887`

**Example Context:**
```rust
"find_text" => {
                let search_text = params
                    .get("search_text")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
```

### `sequential_when_repeated`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:40`

**Example Context:**
```rust
pub fn get_sequential_when_repeated(&self) -> bool {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("sequential_when_repeated").and_then(|v| v.as_bool()))
            .unwrap_or_default()
    }
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

### `session_id`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:369`

**Example Context:**
```rust
) -> ToolResult<Vec<Content>> {
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
```

### `sheetName`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1496`

**Example Context:**
```rust
// Get the sheet name if provided, otherwise we'll use the first sheet
                let sheet_name = params
                    .get("sheetName")
                    .and_then(|q| q.as_str())
                    .map(|s| format!("{}!1:1", s))
```

### `signature`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/databricks.rs:311`
- `crates/goose/src/providers/formats/snowflake.rs:257`

**Example Context:**
```rust
.unwrap_or_default();
                                    let signature = summary
                                        .get("signature")
                                        .and_then(|s| s.as_str())
                                        .unwrap_or_default();
```

### `simple`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:462`

**Example Context:**
```rust
assert_eq!(opts.name, "test-prompt");
            assert_eq!(opts.arguments.len(), 2);
            assert_eq!(opts.arguments.get("simple"), Some(&"value".to_string()));
            assert_eq!(
                opts.arguments.get("quoted"),
```

### `size`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:44`

**Example Context:**
```rust
.and_then(|v| v.as_bool())
                .unwrap_or(false),
            size: obj.get("size").and_then(|v| v.as_u64()).map(|s| s as usize),
            color: obj.get("color").and_then(|v| v.as_str()).map(String::from),
            alignment: obj
```

### `spreadsheetId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1443`

**Example Context:**
```rust
// Implement sheets_tool functionality
    async fn sheets_tool(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let spreadsheet_id = params.get("spreadsheetId").and_then(|q| q.as_str()).ok_or(
            ToolError::InvalidParameters("The spreadsheetId is required".to_string()),
        )?;
```

### `startPosition`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2890`

**Example Context:**
```rust
},
            "delete_content" => {
                let start_position = params.get("startPosition").and_then(|q| q.as_i64()).ok_or(
                    ToolError::InvalidParameters("The startPosition parameter is required for delete_content operation".to_string()),
                )?;
```

### `state`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:307`

**Example Context:**
```rust
async move {
                    let code = params.get("code").cloned();
                    let received_state = params.get("state").cloned();

                    if let (Some(code), Some(received_state)) = (code, received_state) {
```

### `status`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/utils.rs:229`

**Example Context:**
```rust
if let Some(error) = payload.get("error") {
                    error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
                    let error_status = error.get("status").and_then(|s| s.as_str()).unwrap_or("Unknown status");
                    if error_status == "INVALID_ARGUMENT" && error_msg.to_lowercase().contains("exceeds") {
                        return Err(ProviderError::ContextLengthExceeded(error_msg.to_string()));
```

### `style`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:184`

**Example Context:**
```rust
.and_then(|v| v.as_str())
                    .unwrap_or("append");
                let style = params.get("style").and_then(DocxStyle::from_json);

                let mode = match mode {
```

### `sub_recipe`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:28`

**Example Context:**
```rust
pub fn get_sub_recipe(&self) -> Option<&Map<String, Value>> {
        (self.task_type == "sub_recipe")
            .then(|| self.payload.get("sub_recipe")?.as_object())
            .flatten()
    }
```

### `subagent_id`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/mod.rs:1106`

**Example Context:**
```rust
if let Some(Value::String(msg)) = o.get("message") {
                                                // Extract subagent info for better display
                                                let subagent_id = o.get("subagent_id")
                                                    .and_then(|v| v.as_str());
                                                let notification_type = o.get("type")
```

### `summary`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/databricks.rs:301`

**Example Context:**
```rust
Some("reasoning") => {
                    if let Some(summary_array) =
                        content_item.get("summary").and_then(|s| s.as_array())
                    {
                        for summary in summary_array {
```

### `supported_parameters`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openrouter.rs:309`

**Example Context:**
```rust
// Check if the model supports tools
                let supported_params =
                    match model.get("supported_parameters").and_then(|v| v.as_array()) {
                        Some(params) => params,
                        None => {
```

### `target`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3095`

**Example Context:**
```rust
})
        ).transpose()?;
        let target = params.get("target").and_then(|s| s.as_str());
        let email_message = params.get("emailMessage").and_then(|s| s.as_str());
```

### `targetId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1986`

**Example Context:**
```rust
let parent_id = params.get("parentId").and_then(|q| q.as_str());
        let target_id = params.get("targetId").and_then(|q| q.as_str());
        let body = params.get("body").and_then(|q| q.as_str());
        let path = params.get("path").and_then(|q| q.as_str());
```

### `task_ids`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/lib/mod.rs:24`

**Example Context:**
```rust
let task_ids: Vec<String> = serde_json::from_value(
        input
            .get("task_ids")
            .ok_or("Missing task_ids field")?
            .clone(),
```

### `task_parameters`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-cli/src/session/output.rs:428`
- `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs:72`
- `crates/goose/src/agents/recipe_tools/sub_recipe_tools.rs:47`

**Example Context:**
```rust
// Print task_parameters array
    if let Some(Value::Array(task_parameters)) = call.arguments.get("task_parameters") {
        println!("{}:", style("task_parameters").dim());
```

### `temperature`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:937`

**Example Context:**
```rust
// Temperature should not be present for 3.7 models with thinking
            assert!(payload.get("temperature").is_none());

            Ok(())
```

### `test_key`

**Method(s):** config_get, config_set

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

**Method(s):** config_get, config_set

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

### `text`

**Method(s):** config_get

**Usage Locations (11):**

- `crates/goose-mcp/src/google_drive/mod.rs:2627`
- `crates/goose-mcp/src/google_drive/mod.rs:2678`
- `crates/goose-mcp/src/google_drive/mod.rs:2753`
- `crates/goose-mcp/src/google_drive/mod.rs:2815`
- `crates/goose/src/providers/claude_code.rs:221`
- `crates/goose/src/providers/formats/anthropic.rs:538`
- `crates/goose/src/providers/formats/databricks.rs:295`
- `crates/goose/src/providers/formats/databricks.rs:307`
- `crates/goose/src/providers/formats/google.rs:244`
- `crates/goose/src/providers/formats/snowflake.rs:227`
- ... and 1 more locations

**Example Context:**
```rust
},
            "insert_text" => {
                let text = params.get("text").and_then(|q| q.as_str()).ok_or(
                    ToolError::InvalidParameters("The text parameter is required for insert_text operation".to_string()),
                )?;
```

### `textValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2245`

**Example Context:**
```rust
field_mods.set_date_values = Some(parsed_dates?);
                        } else if let Some(text_value) =
                            op.get("textValue").and_then(|o| o.as_array())
                        {
                            field_mods.set_text_values = Some(
```

### `text_instruction`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs:83`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:59`

**Example Context:**
```rust
.map(|task_param| {
            let text_instruction = task_param
                .get("text_instruction")
                .and_then(|v| v.as_str())
                .unwrap_or("")
```

### `thinking`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:932`
- `crates/goose/src/providers/formats/snowflake.rs:253`

**Example Context:**
```rust
// Verify thinking parameters
            assert!(payload.get("thinking").is_some());
            assert_eq!(payload["thinking"]["type"], "enabled");
            assert!(payload["thinking"]["budget_tokens"].as_i64().unwrap() >= 1024);
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

### `title`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1709`

**Example Context:**
```rust
"add_sheet" => {
                let title = params
                    .get("title")
                    .and_then(|q| q.as_str())
                    .ok_or(ToolError::InvalidParameters(
```

### `token_endpoint`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:120`

**Example Context:**
```rust
let token_endpoint = oidc_config
        .get("token_endpoint")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("token_endpoint not found in OIDC configuration"))?
```

### `tool_calls`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/providers/formats/databricks.rs:336`
- `crates/goose/src/providers/formats/openai.rs:235`
- `crates/goose/src/providers/formats/openai.rs:279`
- `crates/goose/src/providers/toolshim.rs:214`

**Example Context:**
```rust
// Handle tool calls
    if let Some(tool_calls) = original.get("tool_calls") {
        if let Some(tool_calls_array) = tool_calls.as_array() {
            for tool_call in tool_calls_array {
```

### `tool_use_id`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/providers/formats/snowflake.rs:161`
- `crates/goose/src/providers/formats/snowflake.rs:235`
- `crates/goose/src/providers/snowflake.rs:182`
- `crates/goose/src/providers/snowflake.rs:205`

**Example Context:**
```rust
}
                        Some("tool_use") => {
                            if let Some(id) = delta.get("tool_use_id").and_then(|i| i.as_str()) {
                                tool_use_id = Some(id.to_string());
                            }
```

### `tools`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/snowflake.rs:674`
- `crates/goose/src/providers/venice.rs:139`

**Example Context:**
```rust
// Should not include tools for description requests
        assert!(request.get("tools").is_none());

        Ok(())
```

### `totalTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:288`

**Example Context:**
```rust
.map(|v| v as i32);
        let total_tokens = usage_meta_data
            .get("totalTokenCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as i32);
```

### `total_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:347`

**Example Context:**
```rust
let total_tokens = usage
        .get("total_tokens")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
```

### `type`

**Method(s):** config_get

**Usage Locations (19):**

- `crates/goose-cli/src/recipes/github_recipe.rs:250`
- `crates/goose-cli/src/session/mod.rs:1108`
- `crates/goose-mcp/src/google_drive/mod.rs:3086`
- `crates/goose-server/src/openapi.rs:108`
- `crates/goose/src/agents/extension_manager.rs:163`
- `crates/goose/src/providers/claude_code.rs:208`
- `crates/goose/src/providers/claude_code.rs:217`
- `crates/goose/src/providers/formats/anthropic.rs:523`
- `crates/goose/src/providers/formats/anthropic.rs:536`
- `crates/goose/src/providers/formats/anthropic.rs:550`
- ... and 9 more locations

**Example Context:**
```rust
if let (Some(name), Some(item_type)) = (
                item.get("name").and_then(|n| n.as_str()),
                item.get("type").and_then(|t| t.as_str()),
            ) {
                if item_type == "dir" {
```

### `underline`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:41`

**Example Context:**
```rust
italic: obj.get("italic").and_then(|v| v.as_bool()).unwrap_or(false),
            underline: obj
                .get("underline")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
```

### `unknown-model`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:530`

**Example Context:**
```rust
// unknown model should have default limit (128k)
        assert_eq!(*model_info.get("unknown-model").unwrap(), 128_000);
    }
```

### `updateLabels`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2173`

**Example Context:**
```rust
};

        if let Some(label_ops) = params.get("updateLabels").and_then(|q| q.as_array()) {
            let label_result = self.update_label(file_id, label_ops).await?;
            final_result.extend(label_result);
```

### `uri`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1378`
- `crates/mcp-server/src/router.rs:226`

**Example Context:**
```rust
async fn read(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let (maybe_uri, maybe_url) = (
            params.get("uri").and_then(|q| q.as_str()),
            params.get("url").and_then(|q| q.as_str()),
        );
```

### `url`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/computercontroller/web_scrape.rs:56`
- `crates/goose-mcp/src/google_drive/mod.rs:1379`

**Example Context:**
```rust
if let Ok(args) = serde_json::from_value::<Value>(tool_call.arguments.clone()) {
                            // Check all required parameters match exactly                                                        
                            args.get("url").and_then(Value::as_str).map(|s| s.trim_end_matches('/')) == Some("https://news.ycombinator.com")
                        } else {
                            false
```

### `usage`

**Method(s):** config_get

**Usage Locations (15):**

- `crates/goose/src/providers/azure.rs:156`
- `crates/goose/src/providers/claude_code.rs:232`
- `crates/goose/src/providers/claude_code.rs:255`
- `crates/goose/src/providers/databricks.rs:259`
- `crates/goose/src/providers/formats/anthropic.rs:269`
- `crates/goose/src/providers/formats/anthropic.rs:505`
- `crates/goose/src/providers/formats/anthropic.rs:605`
- `crates/goose/src/providers/formats/anthropic.rs:643`
- `crates/goose/src/providers/formats/snowflake.rs:281`
- `crates/goose/src/providers/githubcopilot.rs:425`
- ... and 5 more locations

**Example Context:**
```rust
let message = response_to_message(&response)?;
        let usage = response.get("usage").map(get_usage).unwrap_or_else(|| {
            tracing::debug!("Failed to get usage data");
            Usage::default()
```

### `usageMetadata`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:278`

**Example Context:**
```rust
/// Extract usage information from Google's API response
pub fn get_usage(data: &Value) -> Result<Usage> {
    if let Some(usage_meta_data) = data.get("usageMetadata") {
        let input_tokens = usage_meta_data
            .get("promptTokenCount")
```

### `userValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2272`

**Example Context:**
```rust
);
                        } else if let Some(user_value) =
                            op.get("userValue").and_then(|o| o.as_array())
                        {
                            field_mods.set_user_values = Some(
```

### `valid`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:481`

**Example Context:**
```rust
assert_eq!(opts.name, "test-prompt");
            assert_eq!(opts.arguments.len(), 1);
            assert_eq!(opts.arguments.get("valid"), Some(&"value".to_string()));
            // Invalid arguments are ignored but logged
        } else {
```

### `value`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1661`

**Example Context:**
```rust
let value = params
                    .get("value")
                    .and_then(|q| q.as_str())
                    .ok_or(ToolError::InvalidParameters(
```

### `valueInputOption`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1609`
- `crates/goose-mcp/src/google_drive/mod.rs:1669`

**Example Context:**
```rust
// Determine the input option (default to USER_ENTERED)
                let value_input_option = params
                    .get("valueInputOption")
                    .and_then(|q| q.as_str())
                    .unwrap_or("USER_ENTERED");
```

### `values`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1589`

**Example Context:**
```rust
let values_csv = params
                    .get("values")
                    .and_then(|q| q.as_str())
                    .ok_or(ToolError::InvalidParameters(
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

### `view_range`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:784`

**Example Context:**
```rust
"view" => {
                let view_range = params
                    .get("view_range")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| {
```

### `width`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:224`

**Example Context:**
```rust
let width = params
                            .get("width")
                            .and_then(|v| v.as_u64())
                            .map(|w| w as u32);
```

### `window_title`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:1435`

**Example Context:**
```rust
async fn screen_capture(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let mut image = if let Some(window_title) =
            params.get("window_title").and_then(|v| v.as_str())
        {
            // Try to find and capture the specified window
```

### `worksheet`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/computercontroller/mod.rs:849`
- `crates/goose-mcp/src/computercontroller/mod.rs:872`
- `crates/goose-mcp/src/computercontroller/mod.rs:900`
- `crates/goose-mcp/src/computercontroller/mod.rs:922`
- `crates/goose-mcp/src/computercontroller/mod.rs:955`

**Example Context:**
```rust
let xlsx = xlsx_tool::XlsxTool::new(path)
                    .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                let worksheet = if let Some(name) = params.get("worksheet").and_then(|v| v.as_str())
                {
                    xlsx.get_worksheet_by_name(name)
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

**Method(s):** secret_get, config_get, config_delete, config_set

**Usage Locations (9):**

- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:216`
- `crates/goose-server/src/routes/audio.rs:216`
- `crates/goose-server/src/routes/audio.rs:223`
- `crates/goose-server/src/routes/audio.rs:223`
- `crates/goose-server/src/routes/audio.rs:231`
- `crates/goose-server/src/routes/audio.rs:231`
- `crates/goose-server/src/routes/audio.rs:339`

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

**Method(s):** secret_get, secret_delete, secret_set

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

**Method(s):** env_set, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** secret_get, env_remove

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

### `GOOGLE_DRIVE_DISK_FALLBACK`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:47`

**Example Context:**
```rust
pub const KEYCHAIN_SERVICE: &str = "mcp_google_drive";
pub const KEYCHAIN_USERNAME: &str = "oauth_credentials";
pub const KEYCHAIN_DISK_FALLBACK_ENV: &str = "GOOGLE_DRIVE_DISK_FALLBACK";

const GOOGLE_DRIVE_SCOPES: Scope = Scope::Full;
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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** config_set, env_set, config_get, env_var, env_remove

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

**Method(s):** config_set, env_set, config_get, env_var, env_remove

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

### `GOOSE_RECIPE_GITHUB_REPO`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-cli/src/recipes/github_recipe.rs:31`

**Example Context:**
```rust
}

pub const GOOSE_RECIPE_GITHUB_REPO_CONFIG_KEY: &str = "GOOSE_RECIPE_GITHUB_REPO";
pub fn retrieve_recipe_from_github(
    recipe_name: &str,
```

### `GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/retry.rs:35`

**Example Context:**
```rust
/// Environment variable for configuring on_failure timeout globally
const GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS: &str = "GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS";

/// Manages retry state and operations for agent execution
```

### `GOOSE_RECIPE_PATH`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-cli/src/recipes/search_recipe.rs:16`

**Example Context:**
```rust
};

const GOOSE_RECIPE_PATH_ENV_VAR: &str = "GOOSE_RECIPE_PATH";

pub fn retrieve_recipe_file(recipe_name: &str) -> Result<RecipeFile> {
```

### `GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/retry.rs:32`

**Example Context:**
```rust
/// Environment variable for configuring retry timeout globally
const GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS: &str = "GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS";

/// Environment variable for configuring on_failure timeout globally
```

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`

**Method(s):** config_get, config_set, env_var

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

**Method(s):** config_get, config_set, env_var

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

### `GOOSE_SUBAGENT_MAX_TURNS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_task_config.rs:11`

**Example Context:**
```rust
/// Environment variable name for configuring max turns
pub const GOOSE_SUBAGENT_MAX_TURNS_ENV_VAR: &str = "GOOSE_SUBAGENT_MAX_TURNS";

/// Configuration for task execution with all necessary dependencies
```

### `GOOSE_TEMPERATURE`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/model.rs:141`
- `crates/goose/src/model.rs:141`

**Example Context:**
```rust
fn parse_temperature() -> Result<Option<f32>, ConfigError> {
        if let Ok(val) = std::env::var("GOOSE_TEMPERATURE") {
            let temp = val.parse::<f32>().map_err(|_| {
                ConfigError::InvalidValue(
```

### `GOOSE_TEMPORAL_BIN`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/temporal_scheduler.rs:458`
- `crates/goose/src/temporal_scheduler.rs:458`

**Example Context:**
```rust
// Check environment variable override
        if let Ok(binary_path) = std::env::var("GOOSE_TEMPORAL_BIN") {
            if std::path::Path::new(&binary_path).exists() {
                tracing::info!(
```

### `GOOSE_TEST_PROVIDER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (test)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (test)

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

**Method(s):** env_set, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_var, env_set, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_set, env_var, env_remove

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

**Method(s):** env_var, env_set, env_remove

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

