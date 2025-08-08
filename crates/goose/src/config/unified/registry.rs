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
        key_spec! {
            key: "tracing.langfuse.url",
            ty: String,
            env_aliases: ["LANGFUSE_URL"],
            validator: validate_url,
        },
        // tracing.otlp.endpoint
        key_spec! {
            key: "tracing.otlp.endpoint",
            ty: String,
            env_aliases: ["OTEL_EXPORTER_OTLP_ENDPOINT"],
        },
        // tracing.otlp.timeout_ms
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
            env_aliases: ["GOOSE_MAX_TURNS"],
            validator: validate_nonzero_u32,
        },
        // session.max_tool_repetitions
        key_spec! {
            key: "session.max_tool_repetitions",
            ty: U32,
            env_aliases: ["GOOSE_MAX_TOOL_REPETITIONS"],
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
