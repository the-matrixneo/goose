use once_cell::sync::Lazy;
// -*- phase:3 unified-config registry expansion -*-

// -*- phase:3 unified-config registry expansion -*-

// -*- phase:2 unified-config registry expansion -*-

// Phase 2: expanding registry coverage per CONFIG_* docs

use serde_json::Value;
use url;

// --- Validators (Phase 2(b)): Enforced validators ---
pub type ValidatorFn = fn(&Value) -> Result<(), String>;

fn validate_port(v: &Value) -> Result<(), String> {
    match v {
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                if (1..=65535).contains(&u) {
                    Ok(())
                } else {
                    Err("must be between 1 and 65535".to_string())
                }
            } else {
                Err("must be an integer".to_string())
            }
        }
        _ => Err("must be an integer".to_string()),
    }
}

fn validate_nonzero_u32(v: &Value) -> Result<(), String> {
    match v {
        Value::Number(n) => match n.as_u64() {
            Some(u) if u >= 1 => Ok(()),
            Some(_) => Err("must be >= 1".to_string()),
            None => Err("must be an integer".to_string()),
        },
        _ => Err("must be an integer".to_string()),
    }
}

fn validate_url(v: &Value) -> Result<(), String> {
    match v {
        Value::String(s) => {
            if s.trim().is_empty() {
                return Err("must be a non-empty URL".to_string());
            }
            match url::Url::parse(s) {
                Ok(url) => {
                    // require http(s) scheme to reduce footguns
                    match url.scheme() {
                        "http" | "https" => Ok(()),
                        _ => Err("URL must start with http:// or https://".to_string()),
                    }
                }
                Err(e) => Err(format!("invalid URL: {}", e)),
            }
        }
        _ => Err("must be a string URL".to_string()),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ValueType {
    String,
    Bool,
    U32,
    U16,
    F64,
    Any,
}

fn validate_enum(v: &Value, allowed: &[&str]) -> Result<(), String> {
    match v {
        Value::String(s) => {
            if allowed.iter().any(|a| a.eq_ignore_ascii_case(s)) {
                Ok(())
            } else {
                Err(format!("must be one of: {}", allowed.join(", ")))
            }
        }
        _ => Err("must be a string".to_string()),
    }
}

fn validate_goose_mode(v: &Value) -> Result<(), String> {
    validate_enum(v, &["auto", "approve", "smart_approve", "chat"])
}

fn validate_router_strategy(v: &Value) -> Result<(), String> {
    validate_enum(v, &["default", "vector", "llm"])
}

fn validate_scheduler_type(v: &Value) -> Result<(), String> {
    validate_enum(v, &["legacy", "temporal"])
}

fn validate_temperature(v: &Value) -> Result<(), String> {
    match v {
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if (0.0..=2.0).contains(&f) {
                    Ok(())
                } else {
                    Err("must be between 0.0 and 2.0".to_string())
                }
            } else {
                Err("must be a number".to_string())
            }
        }
        _ => Err("must be a number".to_string()),
    }
}

fn validate_otlp_protocol(v: &Value) -> Result<(), String> {
    validate_enum(v, &["grpc", "http"])
}

#[derive(Clone, Debug)]
pub struct KeySpec {
    pub key: &'static str,
    pub ty: ValueType,
    pub default: Option<Value>,
    pub secret: bool,
    pub env_aliases: &'static [&'static str],
    pub validator: Option<ValidatorFn>,
}

macro_rules! key_spec {
    (
        key: $key:expr,
        ty: $ty:ident
        $(, default: $default:expr)?
        $(, secret: $secret:expr)?
        $(, env_aliases: [$($alias:expr),* $(,)?])?
        $(, validator: $validator:expr)?
        $(,)?
    ) => {
        KeySpec {
            key: $key,
            ty: ValueType::$ty,
            default: None $(.or(Some(serde_json::json!($default))))?,
            secret: false $(|| $secret)? ,
            env_aliases: &[$($($alias),* ,)?],
            validator: None $(.or(Some($validator)))?,
        }
    };
}

pub(crate) use key_spec;

pub fn canonical_to_env(key: &str) -> String {
    let upper = key.replace('.', "_").to_ascii_uppercase();
    format!("GOOSE_{}", upper)
}

pub static REGISTRY: Lazy<Vec<KeySpec>> = Lazy::new(|| {
    vec![
        // llm.provider
        key_spec! {
            key: "llm.provider",
            ty: String,
            default: "openai",
            env_aliases: ["GOOSE_PROVIDER", "PROVIDER"],
        },
        // llm.model
        key_spec! {
            key: "llm.model",
            ty: String,
            default: "gpt-4o",
            env_aliases: ["GOOSE_MODEL", "MODEL"],
        },
        // agent.mode (aka GOOSE_MODE)
        key_spec! {
            key: "agent.mode",
            ty: String,
            default: "auto",
            env_aliases: ["GOOSE_MODE"],
            validator: validate_goose_mode,
        },
        // router.tool_selection_strategy
        key_spec! {
            key: "router.tool_selection_strategy",
            ty: String,
            default: "default",
            env_aliases: ["GOOSE_ROUTER_TOOL_SELECTION_STRATEGY"],
            validator: validate_router_strategy,
        },
        // server.host
        key_spec! {
            key: "server.host",
            ty: String,
            default: "127.0.0.1",
            env_aliases: ["HOST"],
        },
        // server.port
        key_spec! {
            key: "server.port",
            ty: U16,
            default: 3000u16,
            env_aliases: ["PORT"],
            validator: validate_port,
        },
        // server.secret_key (used by Desktop <-> goosed auth)
        key_spec! {
            key: "server.secret_key",
            ty: String,
            secret: true,
            env_aliases: ["GOOSE_SERVER__SECRET_KEY"],
        },
        // security.allowlist.url
        key_spec! {
            key: "security.allowlist.url",
            ty: String,
            env_aliases: ["GOOSE_ALLOWLIST"],
            validator: validate_url,
        },
        // security.allowlist.bypass
        key_spec! {
            key: "security.allowlist.bypass",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_ALLOWLIST_BYPASS"],
        },
        // tracing.langfuse.url
        // Base URL for Langfuse API (defaults to http://localhost:3000)
        key_spec! {
            key: "tracing.langfuse.url",
            ty: String,
            env_aliases: ["LANGFUSE_URL"],
            validator: validate_url,
        },
        // tracing.langfuse.public_key (secret)
        // Langfuse authentication public key - supports both regular and init project keys
        key_spec! {
            key: "tracing.langfuse.public_key",
            ty: String,
            secret: true,
            env_aliases: ["LANGFUSE_PUBLIC_KEY", "LANGFUSE_INIT_PROJECT_PUBLIC_KEY"],
        },
        // tracing.langfuse.secret_key (secret)
        // Langfuse authentication secret key - supports both regular and init project keys
        key_spec! {
            key: "tracing.langfuse.secret_key",
            ty: String,
            secret: true,
            env_aliases: ["LANGFUSE_SECRET_KEY", "LANGFUSE_INIT_PROJECT_SECRET_KEY"],
        },
        // tracing.otlp.endpoint
        // OpenTelemetry Protocol exporter endpoint
        // Standard OTEL env vars for SDK compatibility
        key_spec! {
            key: "tracing.otlp.endpoint",
            ty: String,
            env_aliases: ["OTEL_EXPORTER_OTLP_ENDPOINT", "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT"],
        },
        // tracing.otlp.timeout_ms
        // Timeout for OTLP exports in milliseconds (defaults to 10000ms = 10s)
        // Standard OTEL env var for SDK compatibility
        key_spec! {
            key: "tracing.otlp.timeout_ms",
            ty: U32,
            default: 10000u32,
            env_aliases: ["OTEL_EXPORTER_OTLP_TIMEOUT"],
        },
        // model.context_limit (aka GOOSE_CONTEXT_LIMIT)
        key_spec! {
            key: "model.context_limit",
            ty: U32,
            env_aliases: ["GOOSE_CONTEXT_LIMIT"],
            validator: validate_nonzero_u32,
        },
        // lead.context_limit
        key_spec! {
            key: "lead.context_limit",
            ty: U32,
            env_aliases: ["GOOSE_LEAD_CONTEXT_LIMIT"],
            validator: validate_nonzero_u32,
        },
        // lead.model
        key_spec! {
            key: "lead.model",
            ty: String,
            env_aliases: ["GOOSE_LEAD_MODEL"],
        },
        // lead.provider
        key_spec! {
            key: "lead.provider",
            ty: String,
            env_aliases: ["GOOSE_LEAD_PROVIDER"],
        },
        // lead.turns
        key_spec! {
            key: "lead.turns",
            ty: U32,
            env_aliases: ["GOOSE_LEAD_TURNS"],
            validator: validate_nonzero_u32,
        },
        // lead.failure_threshold
        key_spec! {
            key: "lead.failure_threshold",
            ty: U32,
            env_aliases: ["GOOSE_LEAD_FAILURE_THRESHOLD"],
            validator: validate_nonzero_u32,
        },
        // lead.fallback_turns
        key_spec! {
            key: "lead.fallback_turns",
            ty: U32,
            env_aliases: ["GOOSE_LEAD_FALLBACK_TURNS"],
            validator: validate_nonzero_u32,
        },
        // worker.context_limit
        key_spec! {
            key: "worker.context_limit",
            ty: U32,
            env_aliases: ["GOOSE_WORKER_CONTEXT_LIMIT"],
            validator: validate_nonzero_u32,
        },
        // providers.openai.api_key (secret)
        key_spec! {
            key: "providers.openai.api_key",
            ty: String,
            secret: true,
            env_aliases: ["OPENAI_API_KEY"],
        },
        // providers.anthropic.api_key (secret)
        key_spec! {
            key: "providers.anthropic.api_key",
            ty: String,
            secret: true,
            env_aliases: ["ANTHROPIC_API_KEY"],
        },
        // providers.google.api_key (secret)
        key_spec! {
            key: "providers.google.api_key",
            ty: String,
            secret: true,
            env_aliases: ["GOOGLE_API_KEY"],
        },
        // providers.azure.api_key (secret)
        key_spec! {
            key: "providers.azure.api_key",
            ty: String,
            secret: true,
            env_aliases: ["AZURE_OPENAI_API_KEY"],
        },
        // providers.groq.api_key (secret)
        key_spec! {
            key: "providers.groq.api_key",
            ty: String,
            secret: true,
            env_aliases: ["GROQ_API_KEY"],
        },
        // providers.xai.api_key (secret)
        key_spec! {
            key: "providers.xai.api_key",
            ty: String,
            secret: true,
            env_aliases: ["XAI_API_KEY"],
        },
        // providers.venice.api_key (secret)
        key_spec! {
            key: "providers.venice.api_key",
            ty: String,
            secret: true,
            env_aliases: ["VENICE_API_KEY"],
        },
        // providers.litellm.api_key (secret)
        key_spec! {
            key: "providers.litellm.api_key",
            ty: String,
            secret: true,
            env_aliases: ["LITELLM_API_KEY"],
        },
        // providers.openrouter.api_key (secret)
        key_spec! {
            key: "providers.openrouter.api_key",
            ty: String,
            secret: true,
            env_aliases: ["OPENROUTER_API_KEY"],
        },
        // providers.databricks.token (secret)
        key_spec! {
            key: "providers.databricks.token",
            ty: String,
            secret: true,
            env_aliases: ["DATABRICKS_TOKEN"],
        },
        // providers.snowflake.token (secret)
        key_spec! {
            key: "providers.snowflake.token",
            ty: String,
            secret: true,
            env_aliases: ["SNOWFLAKE_TOKEN"],
        },
        // providers.anthropic.thinking_enabled
        key_spec! {
            key: "providers.anthropic.thinking_enabled",
            ty: Bool,
            default: false,
            env_aliases: ["CLAUDE_THINKING_ENABLED"],
        },
        // providers.anthropic.thinking_budget
        key_spec! {
            key: "providers.anthropic.thinking_budget",
            ty: U32,
            env_aliases: ["CLAUDE_THINKING_BUDGET"],
        },
        // planner.provider
        key_spec! {
            key: "planner.provider",
            ty: String,
            env_aliases: ["GOOSE_PLANNER_PROVIDER"],
        },
        // planner.model
        key_spec! {
            key: "planner.model",
            ty: String,
            env_aliases: ["GOOSE_PLANNER_MODEL"],
        },
        // planner.context_limit
        key_spec! {
            key: "planner.context_limit",
            ty: U32,
            env_aliases: ["GOOSE_PLANNER_CONTEXT_LIMIT"],
        },
        // embeddings.provider
        key_spec! {
            key: "embeddings.provider",
            ty: String,
            env_aliases: ["GOOSE_EMBEDDING_MODEL_PROVIDER"],
        },
        // embeddings.model
        key_spec! {
            key: "embeddings.model",
            ty: String,
            env_aliases: ["GOOSE_EMBEDDING_MODEL"],
        },
        // editor.api_key (secret)
        key_spec! {
            key: "editor.api_key",
            ty: String,
            secret: true,
            env_aliases: ["GOOSE_EDITOR_API_KEY"],
        },
        // editor.host
        key_spec! {
            key: "editor.host",
            ty: String,
            env_aliases: ["GOOSE_EDITOR_HOST"],
            validator: validate_url,
        },
        // editor.model
        key_spec! {
            key: "editor.model",
            ty: String,
            env_aliases: ["GOOSE_EDITOR_MODEL"],
        },
        // cli.theme
        key_spec! {
            key: "cli.theme",
            ty: String,
            default: "dark",
            env_aliases: ["GOOSE_CLI_THEME"],
        },
        // cli.show_cost
        key_spec! {
            key: "cli.show_cost",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_CLI_SHOW_COST"],
        },
        // cli.min_priority (float or string; accept Any)
        key_spec! {
            key: "cli.min_priority",
            ty: Any,
            env_aliases: ["GOOSE_CLI_MIN_PRIORITY"],
        },
        // cli.show_thinking
        key_spec! {
            key: "cli.show_thinking",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_CLI_SHOW_THINKING"],
        },
        // scheduler.type
        key_spec! {
            key: "scheduler.type",
            ty: String,
            default: "legacy",
            env_aliases: ["GOOSE_SCHEDULER_TYPE"],
            validator: validate_scheduler_type,
        },
        // scheduler.temporal.bin
        key_spec! {
            key: "scheduler.temporal.bin",
            ty: String,
            env_aliases: ["GOOSE_TEMPORAL_BIN"],
        },
        // session.max_turns
        key_spec! {
            key: "session.max_turns",
            ty: U32,
            default: 1000u32,
            env_aliases: ["GOOSE_MAX_TURNS", "GOOSE_SESSION_MAX_TURNS"],
            validator: validate_nonzero_u32,
        },
        // session.max_tool_repetitions
        key_spec! {
            key: "session.max_tool_repetitions",
            ty: U32,
            env_aliases: ["GOOSE_MAX_TOOL_REPETITIONS"],
        },
        // model.temperature
        key_spec! {
            key: "model.temperature",
            ty: F64,
            env_aliases: ["GOOSE_TEMPERATURE", "GOOSE_MODEL_TEMPERATURE"],
            validator: validate_temperature,
        },
        // toolshim.enabled
        key_spec! {
            key: "toolshim.enabled",
            ty: Bool,
            env_aliases: ["GOOSE_TOOLSHIM", "GOOSE_TOOLSHIM_ENABLED"],
        },
        // toolshim.model
        key_spec! {
            key: "toolshim.model",
            ty: String,
            env_aliases: ["GOOSE_TOOLSHIM_OLLAMA_MODEL", "GOOSE_TOOLSHIM_MODEL"],
        },
        // tracing.otlp.headers
        // Additional headers for OTLP exports (e.g., for authentication)
        // Format: "key1=value1,key2=value2"
        key_spec! {
            key: "tracing.otlp.headers",
            ty: String,
            env_aliases: ["GOOSE_OTLP_HEADERS"],
        },
        // tracing.otlp.protocol
        // Protocol for OTLP exports (http/protobuf or grpc)
        key_spec! {
            key: "tracing.otlp.protocol",
            ty: String,
            env_aliases: ["GOOSE_OTLP_PROTOCOL"],
            validator: validate_otlp_protocol,
        },
        // Provider host configurations
        // providers.openai.host
        key_spec! {
            key: "providers.openai.host",
            ty: String,
            default: "https://api.openai.com",
            env_aliases: ["OPENAI_HOST"],
            validator: validate_url,
        },
        // providers.anthropic.host
        key_spec! {
            key: "providers.anthropic.host",
            ty: String,
            default: "https://api.anthropic.com",
            env_aliases: ["ANTHROPIC_HOST"],
            validator: validate_url,
        },
        // providers.google.host
        key_spec! {
            key: "providers.google.host",
            ty: String,
            default: "https://generativelanguage.googleapis.com",
            env_aliases: ["GOOGLE_HOST"],
            validator: validate_url,
        },
        // providers.groq.host
        key_spec! {
            key: "providers.groq.host",
            ty: String,
            default: "https://api.groq.com",
            env_aliases: ["GROQ_HOST"],
            validator: validate_url,
        },
        // providers.xai.host
        key_spec! {
            key: "providers.xai.host",
            ty: String,
            default: "https://api.x.ai/v1",
            env_aliases: ["XAI_HOST"],
            validator: validate_url,
        },
        // providers.venice.host
        key_spec! {
            key: "providers.venice.host",
            ty: String,
            default: "https://api.venice.ai",
            env_aliases: ["VENICE_HOST"],
            validator: validate_url,
        },
        // providers.litellm.host
        key_spec! {
            key: "providers.litellm.host",
            ty: String,
            default: "http://localhost:4000",
            env_aliases: ["LITELLM_HOST"],
            validator: validate_url,
        },
        // providers.openrouter.host
        key_spec! {
            key: "providers.openrouter.host",
            ty: String,
            default: "https://openrouter.ai",
            env_aliases: ["OPENROUTER_HOST"],
            validator: validate_url,
        },
        // providers.openrouter.api_key (secret)
        key_spec! {
            key: "providers.openrouter.api_key",
            ty: String,
            secret: true,
            env_aliases: ["OPENROUTER_API_KEY"],
        },
        // providers.databricks.host
        key_spec! {
            key: "providers.databricks.host",
            ty: String,
            env_aliases: ["DATABRICKS_HOST"],
            validator: validate_url,
        },
        // providers.snowflake.host
        key_spec! {
            key: "providers.snowflake.host",
            ty: String,
            env_aliases: ["SNOWFLAKE_HOST"],
            validator: validate_url,
        },
        // providers.snowflake.token (secret)
        key_spec! {
            key: "providers.snowflake.token",
            ty: String,
            secret: true,
            env_aliases: ["SNOWFLAKE_TOKEN"],
        },
        // providers.azure.endpoint
        key_spec! {
            key: "providers.azure.endpoint",
            ty: String,
            env_aliases: ["AZURE_OPENAI_ENDPOINT"],
            validator: validate_url,
        },
        // providers.ollama.host (note: can be just host:port without scheme)
        key_spec! {
            key: "providers.ollama.host",
            ty: String,
            default: "localhost",
            env_aliases: ["OLLAMA_HOST"],
        },
        // Provider timeout configurations
        // providers.openai.timeout
        key_spec! {
            key: "providers.openai.timeout",
            ty: U32,
            default: 600u32,
            env_aliases: ["OPENAI_TIMEOUT"],
            validator: validate_nonzero_u32,
        },
        // providers.litellm.timeout
        key_spec! {
            key: "providers.litellm.timeout",
            ty: U32,
            default: 600u32,
            env_aliases: ["LITELLM_TIMEOUT"],
            validator: validate_nonzero_u32,
        },
        // providers.ollama.timeout
        key_spec! {
            key: "providers.ollama.timeout",
            ty: U32,
            default: 600u32,
            env_aliases: ["OLLAMA_TIMEOUT"],
            validator: validate_nonzero_u32,
        },
        // Additional provider organization/project settings
        // providers.openai.organization
        key_spec! {
            key: "providers.openai.organization",
            ty: String,
            env_aliases: ["OPENAI_ORGANIZATION"],
        },
        // providers.openai.project
        key_spec! {
            key: "providers.openai.project",
            ty: String,
            env_aliases: ["OPENAI_PROJECT"],
        },
        // providers.openai.base_path
        key_spec! {
            key: "providers.openai.base_path",
            ty: String,
            default: "v1/chat/completions",
            env_aliases: ["OPENAI_BASE_PATH"],
        },
        // providers.azure.deployment_name
        key_spec! {
            key: "providers.azure.deployment_name",
            ty: String,
            env_aliases: ["AZURE_OPENAI_DEPLOYMENT_NAME"],
        },
        // providers.azure.api_version
        key_spec! {
            key: "providers.azure.api_version",
            ty: String,
            default: "2024-10-21",
            env_aliases: ["AZURE_OPENAI_API_VERSION"],
        },
        // Provider retry settings
        // providers.databricks.max_retries
        key_spec! {
            key: "providers.databricks.max_retries",
            ty: U32,
            env_aliases: ["DATABRICKS_MAX_RETRIES"],
            validator: validate_nonzero_u32,
        },
        // providers.databricks.initial_retry_interval_ms
        key_spec! {
            key: "providers.databricks.initial_retry_interval_ms",
            ty: U32,
            env_aliases: ["DATABRICKS_INITIAL_RETRY_INTERVAL_MS"],
            validator: validate_nonzero_u32,
        },
        // providers.gcp.max_retries
        key_spec! {
            key: "providers.gcp.max_retries",
            ty: U32,
            env_aliases: ["GCP_MAX_RETRIES"],
            validator: validate_nonzero_u32,
        },
        // providers.gcp.project_id
        key_spec! {
            key: "providers.gcp.project_id",
            ty: String,
            env_aliases: ["GCP_PROJECT_ID"],
        },
        // providers.gcp.location
        key_spec! {
            key: "providers.gcp.location",
            ty: String,
            env_aliases: ["GCP_LOCATION"],
        },
        // system.prompt_file_path
        key_spec! {
            key: "system.prompt_file_path",
            ty: String,
            env_aliases: ["GOOSE_SYSTEM_PROMPT_FILE_PATH"],
        },
        // system.auto_compact_threshold
        key_spec! {
            key: "system.auto_compact_threshold",
            ty: F64,
            env_aliases: ["GOOSE_AUTO_COMPACT_THRESHOLD"],
        },
        // audio.elevenlabs.api_key (secret)
        key_spec! {
            key: "audio.elevenlabs.api_key",
            ty: String,
            secret: true,
            env_aliases: ["ELEVENLABS_API_KEY"],
        },
        // providers.githubcopilot.token (secret)
        key_spec! {
            key: "providers.githubcopilot.token",
            ty: String,
            secret: true,
            env_aliases: ["GITHUB_COPILOT_TOKEN"],
        },
        // Venice provider additional settings
        // providers.venice.base_path
        key_spec! {
            key: "providers.venice.base_path",
            ty: String,
            default: "api/v1/chat/completions",
            env_aliases: ["VENICE_BASE_PATH"],
        },
        // providers.venice.models_path
        key_spec! {
            key: "providers.venice.models_path",
            ty: String,
            default: "api/v1/models",
            env_aliases: ["VENICE_MODELS_PATH"],
        },
        // LiteLLM provider additional settings
        // providers.litellm.base_path
        key_spec! {
            key: "providers.litellm.base_path",
            ty: String,
            default: "v1/chat/completions",
            env_aliases: ["LITELLM_BASE_PATH"],
        },
        // providers.litellm.custom_headers (secret)
        key_spec! {
            key: "providers.litellm.custom_headers",
            ty: String,
            secret: true,
            env_aliases: ["LITELLM_CUSTOM_HEADERS"],
        },
        // OpenAI provider additional settings
        // providers.openai.custom_headers (secret)
        key_spec! {
            key: "providers.openai.custom_headers",
            ty: String,
            secret: true,
            env_aliases: ["OPENAI_CUSTOM_HEADERS"],
        },
        // Sagemaker provider settings
        // providers.sagemaker.endpoint_name
        key_spec! {
            key: "providers.sagemaker.endpoint_name",
            ty: String,
            env_aliases: ["SAGEMAKER_ENDPOINT_NAME"],
        },
        // GCP additional retry settings
        // providers.gcp.initial_retry_interval_ms
        key_spec! {
            key: "providers.gcp.initial_retry_interval_ms",
            ty: U32,
            default: 5000u32,
            env_aliases: ["GCP_INITIAL_RETRY_INTERVAL_MS"],
            validator: validate_nonzero_u32,
        },
        // providers.gcp.max_retry_interval_ms
        key_spec! {
            key: "providers.gcp.max_retry_interval_ms",
            ty: U32,
            default: 320000u32,
            env_aliases: ["GCP_MAX_RETRY_INTERVAL_MS"],
            validator: validate_nonzero_u32,
        },
        // providers.gcp.backoff_multiplier
        key_spec! {
            key: "providers.gcp.backoff_multiplier",
            ty: F64,
            default: 2.0,
            env_aliases: ["GCP_BACKOFF_MULTIPLIER"],
        },
        // Core system configuration (Phase 5)
        // cache.dir
        key_spec! {
            key: "cache.dir",
            ty: String,
            env_aliases: ["GOOSE_CACHE_DIR"],
        },
        // security.disable_keyring
        key_spec! {
            key: "security.disable_keyring",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_DISABLE_KEYRING"],
        },
        // vector_db.path
        key_spec! {
            key: "vector_db.path",
            ty: String,
            env_aliases: ["GOOSE_VECTOR_DB_PATH"],
        },
        // recipes.github_repo_config_key
        key_spec! {
            key: "recipes.github_repo_config_key",
            ty: String,
            env_aliases: ["GOOSE_RECIPE_GITHUB_REPO_CONFIG_KEY"],
        },
        // recipes.on_failure_timeout_seconds
        key_spec! {
            key: "recipes.on_failure_timeout_seconds",
            ty: U32,
            env_aliases: ["GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS"],
            validator: validate_nonzero_u32,
        },
        // recipes.retry_timeout_seconds
        key_spec! {
            key: "recipes.retry_timeout_seconds",
            ty: U32,
            env_aliases: ["GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS"],
            validator: validate_nonzero_u32,
        },
        // recipes.path
        key_spec! {
            key: "recipes.path",
            ty: String,
            env_aliases: ["GOOSE_RECIPE_PATH_ENV_VAR"],
        },
        // subagent.max_turns
        key_spec! {
            key: "subagent.max_turns",
            ty: U32,
            env_aliases: ["GOOSE_SUBAGENT_MAX_TURNS_ENV_VAR"],
            validator: validate_nonzero_u32,
        },
        // MCP configurations (Phase 5)
        // mcp.context_file_names
        key_spec! {
            key: "mcp.context_file_names",
            ty: Any,
            default: [".goosehints"],
            env_aliases: ["CONTEXT_FILE_NAMES"],
        },
        // mcp.google_drive.credentials_path
        key_spec! {
            key: "mcp.google_drive.credentials_path",
            ty: String,
            default: "./gdrive-server-credentials.json",
            env_aliases: ["GOOGLE_DRIVE_CREDENTIALS_PATH"],
        },
        // mcp.google_drive.oauth_config
        key_spec! {
            key: "mcp.google_drive.oauth_config",
            ty: String,
            env_aliases: ["GOOGLE_DRIVE_OAUTH_CONFIG"],
        },
        // mcp.google_drive.oauth_path
        key_spec! {
            key: "mcp.google_drive.oauth_path",
            ty: String,
            default: "./gcp-oauth.keys.json",
            env_aliases: ["GOOGLE_DRIVE_OAUTH_PATH"],
        },
        // mcp.working_dir
        key_spec! {
            key: "mcp.working_dir",
            ty: String,
            env_aliases: ["GOOSE_WORKING_DIR"],
        },
        // Debug configurations (Phase 5)
        // debug.claude_code
        key_spec! {
            key: "debug.claude_code",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_CLAUDE_CODE_DEBUG"],
        },
        // debug.gemini_cli
        key_spec! {
            key: "debug.gemini_cli",
            ty: Bool,
            default: false,
            env_aliases: ["GOOSE_GEMINI_CLI_DEBUG"],
        },
        // Test configurations (Phase 5)
        // test.provider
        key_spec! {
            key: "test.provider",
            ty: String,
            env_aliases: ["GOOSE_TEST_PROVIDER"],
        },
        // System environment variables (Phase 5)
        // system.home
        key_spec! {
            key: "system.home",
            ty: String,
            env_aliases: ["HOME"],
        },
        // system.user
        key_spec! {
            key: "system.user",
            ty: String,
            env_aliases: ["USER"],
        },
        // system.path
        key_spec! {
            key: "system.path",
            ty: String,
            env_aliases: ["PATH"],
        },
        // providers.claude_code.command
        key_spec! {
            key: "providers.claude_code.command",
            ty: String,
            default: "claude-code",
            env_aliases: ["CLAUDE_CODE_COMMAND"],
        },
        // providers.gemini_cli.command
        key_spec! {
            key: "providers.gemini_cli.command",
            ty: String,
            default: "gemini",
            env_aliases: ["GEMINI_CLI_COMMAND"],
        },
        // providers.databricks.backoff_multiplier
        key_spec! {
            key: "providers.databricks.backoff_multiplier",
            ty: F64,
            default: 2.0,
            env_aliases: ["DATABRICKS_BACKOFF_MULTIPLIER"],
        },
        // providers.databricks.max_retry_interval_ms
        key_spec! {
            key: "providers.databricks.max_retry_interval_ms",
            ty: U32,
            default: 320000u32,
            env_aliases: ["DATABRICKS_MAX_RETRY_INTERVAL_MS"],
            validator: validate_nonzero_u32,
        },
        // cli.random_thinking_messages
        key_spec! {
            key: "cli.random_thinking_messages",
            ty: Bool,
            default: false,
            env_aliases: ["RANDOM_THINKING_MESSAGES"],
        },
        // cli.tool_params_truncation_max_length
        key_spec! {
            key: "cli.tool_params_truncation_max_length",
            ty: U32,
            default: 40u32,
            env_aliases: ["GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH"],
            validator: validate_nonzero_u32,
        },
        // cli.edit_mode
        key_spec! {
            key: "cli.edit_mode",
            ty: String,
            default: "emacs",
            env_aliases: ["EDIT_MODE"],
        },
        // session.context_strategy
        key_spec! {
            key: "session.context_strategy",
            ty: String,
            env_aliases: ["GOOSE_CONTEXT_STRATEGY"],
        },
        // display.no_color
        key_spec! {
            key: "display.no_color",
            ty: Bool,
            default: false,
            env_aliases: ["NO_COLOR"],
        },
        // ci.github_actions
        key_spec! {
            key: "ci.github_actions",
            ty: Bool,
            default: false,
            env_aliases: ["GITHUB_ACTIONS"],
        },
        // experiments (Phase 6)
        // experiments - stores HashMap<String, bool> of experiment flags
        key_spec! {
            key: "experiments",
            ty: Any,
            env_aliases: ["GOOSE_EXPERIMENTS"],
        },
        // extensions (Phase 6)
        // extensions - stores HashMap<String, ExtensionEntry> of extension configurations
        key_spec! {
            key: "extensions",
            ty: Any,
            env_aliases: ["GOOSE_EXTENSIONS"],
        },
    ]
});

pub fn find_spec(key: &str) -> Option<&'static KeySpec> {
    REGISTRY.iter().find(|s| s.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validator_enforcement_catches_bad_values() {
        // server.port invalids
        let port = REGISTRY.iter().find(|s| s.key == "server.port").unwrap();
        assert!(port.validator.is_some());
        assert!(port.validator.unwrap()(&serde_json::json!(0)).is_err());
        assert!(port.validator.unwrap()(&serde_json::json!(65536)).is_err());
        assert!(port.validator.unwrap()(&serde_json::json!(3000)).is_ok());

        // session.max_turns invalid
        let max_turns = REGISTRY
            .iter()
            .find(|s| s.key == "session.max_turns")
            .unwrap();
        assert!(max_turns.validator.is_some());
        assert!(max_turns.validator.unwrap()(&serde_json::json!(0)).is_err());
        assert!(max_turns.validator.unwrap()(&serde_json::json!(1)).is_ok());

        // tracing.langfuse.url invalid
        let url = REGISTRY
            .iter()
            .find(|s| s.key == "tracing.langfuse.url")
            .unwrap();
        assert!(url.validator.is_some());
        assert!(url.validator.unwrap()(&serde_json::json!("not a url")).is_err());
        assert!(url.validator.unwrap()(&serde_json::json!("http://example.com")).is_ok());
        assert!(url.validator.unwrap()(&serde_json::json!("https://example.com")).is_ok());
    }
}
