// Logging is handled by CLI entry point
use crate::session::{build_session, Session, SessionBuilderConfig};
use async_trait::async_trait;
use base64::Engine;
use goose::agents::extension::{Envs, ExtensionConfig};
use goose::message::Message;
use goose::session::Identifier;
use goose_bench::bench_session::{BenchAgent, BenchBaseSession};
use goose_bench::eval_suites::ExtensionRequirements;
use goose_bench::interaction_limited_agent::InteractionLimitedAgent;
use mcp_core::Tool;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use tokio::sync::Mutex;

// Global mutex to serialize environment variable operations
static ENV_MUTEX: OnceLock<StdMutex<()>> = OnceLock::new();


struct EnvGuard {
    key: String,
    original_value: Option<String>,
}

impl EnvGuard {
    fn new(key: &str, new_value: &str) -> Self {
        let mutex = ENV_MUTEX.get_or_init(|| StdMutex::new(()));
        let _lock = mutex.lock().unwrap();
        let original_value = std::env::var(key).ok();
        std::env::set_var(key, new_value);
        Self {
            key: key.to_string(),
            original_value,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        let mutex = ENV_MUTEX.get_or_init(|| StdMutex::new(()));
        let _lock = mutex.lock().unwrap();
        match &self.original_value {
            Some(value) => std::env::set_var(&self.key, value),
            None => std::env::remove_var(&self.key),
        }
    }
}

fn setup_clean_config_for_datasets() -> Result<EnvGuard, std::io::Error> {
    let temp_dir =
        std::env::temp_dir().join(format!("goose_bench_clean_config_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)?;
    let config_file = temp_dir.join("config.yaml");
    std::fs::write(&config_file, "extensions: {}\n")?;
    Ok(EnvGuard::new(
        "GOOSE_CONFIG_DIR",
        &temp_dir.to_string_lossy(),
    ))
}

fn get_tools_base64(tools: &[Tool]) -> String {
    let tools_json = serde_json::to_string(tools).expect("Failed to serialize tools");
    base64::engine::general_purpose::STANDARD.encode(tools_json)
}

fn get_mock_binary_path() -> String {
    std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory")
        .join("mock_mcp_server")
        .to_string_lossy()
        .to_string()
}

struct SessionWrapper {
    session: Session,
    _dummy_messages: Vec<Message>,
}

impl SessionWrapper {
    fn new(session: Session) -> Self {
        Self {
            session,
            _dummy_messages: Vec::new(),
        }
    }
}

// Wrapper for InteractionLimitedAgent to implement BenchBaseSession
struct InteractionLimitedAgentWrapper {
    agent: InteractionLimitedAgent,
    session_file: PathBuf,
    dummy_messages: Vec<Message>,
}

impl InteractionLimitedAgentWrapper {
    fn new(agent: InteractionLimitedAgent, session_file: PathBuf) -> Self {
        Self {
            agent,
            session_file,
            dummy_messages: Vec::new(),
        }
    }
}

// allow session obj to be used in benchmarking
#[async_trait]
impl BenchBaseSession for SessionWrapper {
    async fn headless(&mut self, message: String) -> anyhow::Result<()> {
        self.session.headless(message).await
    }
    fn session_file(&self) -> PathBuf {
        self.session.session_file()
    }
    fn message_history(&self) -> Vec<Message> {
        self.session.message_history()
    }
    async fn override_system_prompt(&self, override_prompt: String) {
        self.session.override_system_prompt(override_prompt).await
    }
    fn get_total_token_usage(&self) -> anyhow::Result<Option<i32>> {
        self.session.get_total_token_usage()
    }
    async fn cleanup_extensions(&self) -> anyhow::Result<()> {
        self.session.cleanup_extensions().await
    }
    fn get_agent(&self) -> &goose::agents::Agent {
        self.session.get_agent()
    }
    fn get_messages_mut(&mut self) -> &mut Vec<Message> {
        &mut self._dummy_messages
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[async_trait]
impl BenchBaseSession for InteractionLimitedAgentWrapper {
    async fn headless(&mut self, message: String) -> anyhow::Result<()> {
        self.agent.prompt(message).await?;
        Ok(())
    }

    fn session_file(&self) -> PathBuf {
        self.session_file.clone()
    }

    fn message_history(&self) -> Vec<Message> {
        self.agent.message_history()
    }

    async fn override_system_prompt(&self, override_prompt: String) {
        self.agent.override_system_prompt(override_prompt).await;
    }

    fn get_total_token_usage(&self) -> anyhow::Result<Option<i32>> {
        Ok(None)
    }

    async fn cleanup_extensions(&self) -> anyhow::Result<()> {
        // Call cleanup on the underlying agent through the InteractionLimitedAgent
        self.agent.cleanup_extensions().await
    }

    fn get_agent(&self) -> &goose::agents::Agent {
        self.agent.get_agent()
    }

    fn get_messages_mut(&mut self) -> &mut Vec<Message> {
        // InteractionLimitedAgent manages its own messages
        &mut self.dummy_messages
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        &mut self.agent
    }
}
pub async fn agent_generator(
    requirements: ExtensionRequirements,
    session_id: String,
    dataset: bool,
    available_extensions: Option<HashMap<String, Vec<Tool>>>,
) -> BenchAgent {
    if dataset {
        dataset_agent(requirements, session_id, available_extensions).await
    } else {
        standard_agent(requirements, session_id, available_extensions).await
    }
}

async fn standard_agent(
    requirements: ExtensionRequirements,
    session_id: String,
    _: Option<HashMap<String, Vec<Tool>>>,
) -> BenchAgent {
    let identifier = Some(Identifier::Name(session_id));

    let base_session = build_session(SessionBuilderConfig {
        identifier,
        resume: false,
        no_session: false,
        extensions: requirements.external,
        remote_extensions: requirements.remote,
        builtins: requirements.builtin,
        extensions_override: None,
        additional_system_prompt: None,
        settings: None,
        debug: false,
        max_tool_repetitions: None,
        interactive: false, // Benchmarking is non-interactive
        scheduled_job_id: None,
        quiet: false,
    })
    .await;

    // package session obj into benchmark-compatible struct
    let bench_agent = BenchAgent::new(Box::new(SessionWrapper::new(base_session)));

    // Logging is already set up by CLI - just register error capture if needed
    let errors = Arc::new(Mutex::new(bench_agent.get_errors().await));
    goose_bench::error_capture::ErrorCaptureLayer::register_error_vector(errors);

    bench_agent
}
async fn dataset_agent(
    _requirements: ExtensionRequirements,
    session_id: String,
    _available_extensions: Option<HashMap<String, Vec<Tool>>>,
) -> BenchAgent {
    tracing::info!(session_id = %session_id, "Starting dataset agent creation");

    // Set up clean config environment automatically for dataset benchmarking
    let _config_guard = setup_clean_config_for_datasets()
        .unwrap_or_else(|_| EnvGuard::new("GOOSE_DATASET_FALLBACK", "1"));
    tracing::debug!("Config setup completed");

    // Create Agent directly instead of going through session
    let agent = goose::agents::Agent::new();
    tracing::debug!("Base agent created");

    // Configure provider for the agent
    use goose::config::Config;
    use goose::model::ModelConfig;
    use goose::providers::create;

    // For dataset benchmarking, use a fresh config instance that respects GOOSE_CONFIG_DIR
    let config = Config::default();
    let provider_name = config
        .get_param::<String>("GOOSE_PROVIDER")
        .unwrap_or_else(|_| "anthropic".to_string());
    let model_name = config
        .get_param::<String>("GOOSE_MODEL")
        .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

    let model_config = ModelConfig::new(model_name.clone());
    let provider = create(&provider_name, model_config).unwrap_or_else(|_| std::process::exit(1));

    agent
        .update_provider(provider)
        .await
        .unwrap_or_else(|_| std::process::exit(1));
    tracing::debug!(provider = %provider_name, model = %model_name, "Provider configured");

    // Set up extensions for this specific agent if provided
    if let Some(extensions) = _available_extensions {
        let extension_count = extensions.len();
        tracing::debug!(
            extension_count = extension_count,
            "Setting up extensions for agent"
        );

        for (ext_name, tools) in extensions {
            let tools_base64 = get_tools_base64(&tools);
            let binary_path = get_mock_binary_path();

            // Fallback to cargo run if binary doesn't exist
            let (cmd, args) = if std::path::Path::new(&binary_path).exists() {
                (binary_path, vec![])
            } else {
                (
                    "cargo".to_string(),
                    vec![
                        "run".to_string(),
                        "-p".to_string(),
                        "goose-bench".to_string(),
                        "--bin".to_string(),
                        "mock_mcp_server".to_string(),
                    ],
                )
            };

            let extension_config = ExtensionConfig::Stdio {
                name: ext_name.clone(),
                cmd,
                args,
                envs: Envs::new(HashMap::from([
                    ("EXTENSION_NAME".to_string(), ext_name.clone()),
                    ("EXTENSION_TOOLS".to_string(), tools_base64),
                ])),
                env_keys: vec![],
                timeout: None,
                bundled: None,
                description: None,
            };

            // Add extension directly to agent
            let add_result = agent.add_extension(extension_config).await;

            match add_result {
                Ok(_) => tracing::debug!(
                    extension = %ext_name,
                    tools_count = tools.len(),
                    "Extension added successfully"
                ),
                Err(e) => tracing::warn!(
                    extension = %ext_name,
                    error = %e,
                    "Extension failed to add"
                ),
            }
        }

        tracing::debug!(
            extension_count = extension_count,
            "All extensions setup completed"
        );
    }

    // Create an InteractionLimitedAgent with the configured agent
    let interaction_limited = InteractionLimitedAgent::new(agent, None);

    // Create a dummy session file path for compatibility
    let session_file = std::env::temp_dir().join(format!("bench_session_{}.json", session_id));

    // Wrap in BenchBaseSession trait wrapper
    let wrapper = InteractionLimitedAgentWrapper::new(interaction_limited, session_file);

    // Wrap in BenchAgent for compatibility
    let bench_agent = BenchAgent::new(Box::new(wrapper));
    tracing::debug!("Agent wrappers created");

    // Environment variable is automatically restored when _config_guard is dropped

    // Logging is already set up by CLI - just register error capture if needed
    let errors = Arc::new(Mutex::new(bench_agent.get_errors().await));
    goose_bench::error_capture::ErrorCaptureLayer::register_error_vector(errors);
    tracing::debug!("Logging setup completed");

    tracing::info!(
        session_id = %session_id,
        "Dataset agent creation completed"
    );

    bench_agent
}
