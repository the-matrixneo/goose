use std::collections::HashMap;
use crate::session::{Session, build_session, SessionBuilderConfig};
use crate::logging;
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use goose::message::Message;
use goose::session::Identifier;
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

    let mut base_session = build_session(SessionBuilderConfig {
        identifier,
        resume: false,
        no_session: true,
        extensions: vec![],
        remote_extensions: vec![],
        builtins: vec![],
        extensions_override: None,
        additional_system_prompt: None,
        debug: false,
        max_tool_repetitions: None,
        interactive: false, // Benchmarking is non-interactive
    }).await;

    if let Some(extensions) = available_extensions {
        // Add each extension's tools as stdio extensions using mock MCP server for evaluation
        for (ext_name, tools) in extensions {
            // Serialize the actual tools to pass to mock server
            let tools_json = serde_json::to_string(&tools).expect("Failed to serialize tools");
            let tools_base64 = BASE64_STANDARD.encode(tools_json);
            
            // Create command string for mock MCP server with actual tools
            let extension_command = format!(
                "EXTENSION_NAME={} EXTENSION_TOOLS={} cargo run -p goose-bench --bin mock_mcp_server",
                ext_name, tools_base64
            );
            
            base_session.add_extension(extension_command).await.expect("Failed to add mock stdio extension");
        }
    }
    
    // package session obj into benchmark-compatible struct
    let bench_agent = BenchAgent::new(Box::new(base_session));

    // Initialize logging with error capture
    let errors = Some(Arc::new(Mutex::new(bench_agent.get_errors().await)));
    logging::setup_logging(Some("bench"), errors).expect("Failed to initialize logging");

    bench_agent
}