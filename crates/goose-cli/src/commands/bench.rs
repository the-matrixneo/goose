use std::collections::HashMap;
use crate::session::{Session, build_session, SessionBuilderConfig};
use crate::logging;
use async_trait::async_trait;
use base64::Engine;
use goose::message::Message;
use goose::session::Identifier;
use goose::agents::extension::{ExtensionConfig, Envs};
use goose_bench::bench_session::{BenchAgent, BenchBaseSession};
use goose_bench::eval_suites::ExtensionRequirements;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use mcp_core::Tool;

// allow session obj to be used in benchmarking
#[async_trait]
impl BenchBaseSession for Session {
    async fn headless(&mut self, message: String) -> anyhow::Result<()> {
        self.headless(message).await
    }
    fn session_file(&self) -> PathBuf {
        self.session_file()
    }
    fn message_history(&self) -> Vec<Message> {
        self.message_history()
    }
    async fn override_system_prompt(&self, override_prompt: String)  { self.override_system_prompt(override_prompt).await }
    fn get_total_token_usage(&self) -> anyhow::Result<Option<i32>> {
        self.get_total_token_usage()
    }
}
pub async fn agent_generator(
    requirements: ExtensionRequirements,
    session_id: String,
    dataset: bool,
    available_extensions: Option<HashMap<String, Vec<Tool>>>
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
    _: Option<HashMap<String, Vec<Tool>>>
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
    }).await;

    // package session obj into benchmark-compatible struct
    let bench_agent = BenchAgent::new(Box::new(base_session));

    // Initialize logging with error capture
    let errors = Some(Arc::new(Mutex::new(bench_agent.get_errors().await)));
    logging::setup_logging(Some("bench"), errors).expect("Failed to initialize logging");

    bench_agent
}
async fn dataset_agent(
    _requirements: ExtensionRequirements,
    session_id: String,
    available_extensions: Option<HashMap<String, Vec<Tool>>>
) -> BenchAgent {
    let identifier = Some(Identifier::Name(session_id));


    // Prepare mock extensions as Stdio MCP extensions
    let mock_extensions: Vec<ExtensionConfig> = if let Some(extensions) = available_extensions {
        extensions.into_iter().map(|(ext_name, tools)| {
            // Serialize the actual tools to pass to mock server
            let tools_json = serde_json::to_string(&tools).expect("Failed to serialize tools");
            let tools_base64 = base64::engine::general_purpose::STANDARD.encode(tools_json);
            
            // Use pre-compiled binary path for better performance
            let binary_path = std::env::current_exe()
                .expect("Failed to get current executable path")
                .parent()
                .expect("Failed to get parent directory")
                .join("mock_mcp_server")
                .to_string_lossy()
                .to_string();
            
            // Fallback to cargo run if binary doesn't exist
            let (cmd, args) = if std::path::Path::new(&binary_path).exists() {
                (binary_path, vec![])
            } else {
                ("cargo".to_string(), vec![
                    "run".to_string(),
                    "-p".to_string(),
                    "goose-bench".to_string(),
                    "--bin".to_string(),
                    "mock_mcp_server".to_string(),
                ])
            };
            
            ExtensionConfig::Stdio {
                name: ext_name.clone(),
                cmd,
                args,
                envs: Envs::new(HashMap::from([
                    ("EXTENSION_NAME".to_string(), ext_name),
                    ("EXTENSION_TOOLS".to_string(), tools_base64),
                ])),
                env_keys: vec![],
                timeout: None,
                bundled: None,
                description: None,
            }
        }).collect()
    } else {
        vec![]
    };

    let base_session = build_session(SessionBuilderConfig {
        identifier,
        resume: false,
        no_session: true,
        extensions: vec![], // Not used when extensions_override is set
        remote_extensions: vec![],
        builtins: vec![],
        extensions_override: Some(mock_extensions), // Use mock extensions and prevent loading real ones
        additional_system_prompt: None,
        settings: None,
        debug: false,
        max_tool_repetitions: None,
        interactive: false, // Benchmarking is non-interactive
    }).await;
    
    // package session obj into benchmark-compatible struct
    let bench_agent = BenchAgent::new(Box::new(base_session));

    // Initialize logging with error capture
    let errors = Some(Arc::new(Mutex::new(bench_agent.get_errors().await)));
    logging::setup_logging(Some("bench"), errors).expect("Failed to initialize logging");

    bench_agent
}