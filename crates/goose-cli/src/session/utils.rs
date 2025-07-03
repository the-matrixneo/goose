use anyhow::{Context, Result};
use console::Color;
use goose::agents::Agent;
use goose::config::Config;
use goose::message::Message;
use goose::model::ModelConfig;
use goose::providers::base::Provider;
use goose::providers::create;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::session::output;
use crate::session::PlannerResponseType;

/// Generate a random alphanumeric string of specified length
pub fn generate_random_name(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Parse environment variables from command string
/// 
/// # Arguments
/// * `parts` - Mutable reference to command parts vector
/// 
/// # Returns
/// * HashMap of environment variables
pub fn parse_environment_variables(parts: &mut Vec<&str>) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    
    // Parse environment variables (format: KEY=value)
    while let Some(part) = parts.first() {
        if !part.contains('=') {
            break;
        }
        let env_part = parts.remove(0);
        let (key, value) = env_part.split_once('=').unwrap();
        envs.insert(key.to_string(), value.to_string());
    }
    
    envs
}

/// Extract session ID from a session file path
/// 
/// # Arguments
/// * `session_file` - Optional path to session file
/// 
/// # Returns
/// * Optional session ID string
pub fn extract_session_id_from_path(session_file: &Option<PathBuf>) -> Option<String> {
    session_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Resolve file path, handling relative paths
/// 
/// # Arguments
/// * `filepath_str` - The input file path string
/// 
/// # Returns
/// * Resolved PathBuf
pub fn resolve_file_path(filepath_str: &str) -> Result<PathBuf> {
    let path_buf = PathBuf::from(filepath_str);
    let mut path = path_buf.clone();

    // Update the final path if it's relative
    if path_buf.is_relative() {
        // If the path is relative, resolve it relative to the current working directory
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        path = cwd.join(&path_buf);
    }

    Ok(path)
}

/// Validate that parent directory exists for a given path
/// 
/// # Arguments
/// * `path` - The path to validate
/// 
/// # Returns
/// * Result indicating success or error
pub fn validate_parent_directory_exists(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(anyhow::anyhow!(
                "Directory '{}' does not exist",
                parent.display()
            ));
        }
    }
    Ok(())
}

/// Create a reasoner provider based on configuration
/// 
/// # Returns
/// * Arc<dyn Provider> - The configured reasoner provider
pub fn get_reasoner() -> Result<Arc<dyn Provider>, anyhow::Error> {
    let config = Config::global();

    // Try planner-specific provider first, fallback to default provider
    let provider = if let Ok(provider) = config.get_param::<String>("GOOSE_PLANNER_PROVIDER") {
        provider
    } else {
        println!("WARNING: GOOSE_PLANNER_PROVIDER not found. Using default provider...");
        config
            .get_param::<String>("GOOSE_PROVIDER")
            .expect("No provider configured. Run 'goose configure' first")
    };

    // Try planner-specific model first, fallback to default model
    let model = if let Ok(model) = config.get_param::<String>("GOOSE_PLANNER_MODEL") {
        model
    } else {
        println!("WARNING: GOOSE_PLANNER_MODEL not found. Using default model...");
        config
            .get_param::<String>("GOOSE_MODEL")
            .expect("No model configured. Run 'goose configure' first")
    };

    let model_config = ModelConfig::new(model);
    let reasoner = create(&provider, model_config)?;

    Ok(reasoner)
}

/// Decide if the planner's response is a plan or a clarifying question
///
/// This function is called after the planner has generated a response
/// to the user's message. The response is either a plan or a clarifying
/// question.
/// 
/// # Arguments
/// * `message_text` - The text to classify
/// * `provider` - The provider to use for classification
/// 
/// # Returns
/// * PlannerResponseType indicating the classification result
pub async fn classify_planner_response(
    message_text: String,
    provider: Arc<dyn Provider>,
) -> Result<PlannerResponseType> {
    let prompt = format!("The text below is the output from an AI model which can either provide a plan or list of clarifying questions. Based on the text below, decide if the output is a \"plan\" or \"clarifying questions\".\n---\n{message_text}");

    // Generate the description
    let message = Message::user().with_text(&prompt);
    let (result, _usage) = provider
        .complete(
            "Reply only with the classification label: \"plan\" or \"clarifying questions\"",
            &[message],
            &[],
        )
        .await?;

    let predicted = result.as_concat_text();
    if predicted.to_lowercase().contains("plan") {
        Ok(PlannerResponseType::Plan)
    } else {
        Ok(PlannerResponseType::ClarifyingQuestions)
    }
}

/// Helper function to summarize context messages
/// 
/// # Arguments
/// * `messages` - Mutable reference to messages vector
/// * `agent` - Reference to the agent
/// * `message_suffix` - Suffix message to display
/// 
/// # Returns
/// * Result indicating success or error
pub async fn summarize_context_messages(
    messages: &mut Vec<Message>,
    agent: &Agent,
    message_suffix: &str,
) -> Result<()> {
    // Summarize messages to fit within context length
    let (summarized_messages, _) = agent.summarize_context(messages).await?;
    let msg = format!("Context maxed out\n{}\n{}", "-".repeat(50), message_suffix);
    output::render_text(&msg, Some(Color::Yellow), true);
    *messages = summarized_messages;

    Ok(())
}

/// Validate goose mode parameter
/// 
/// # Arguments
/// * `mode` - The mode string to validate
/// 
/// # Returns
/// * Boolean indicating if the mode is valid
pub fn is_valid_goose_mode(mode: &str) -> bool {
    ["auto", "approve", "chat", "smart_approve"].contains(&mode)
}

/// Convert arguments HashMap to JSON Value
/// 
/// # Arguments
/// * `arguments` - HashMap of string arguments
/// 
/// # Returns
/// * Result containing the JSON Value or error
pub fn arguments_to_json_value(arguments: HashMap<String, String>) -> Result<Value> {
    serde_json::to_value(arguments)
        .map_err(|e| anyhow::anyhow!("Failed to serialize arguments: {}", e))
}

/// Format context strategy message based on strategy and interactive mode
/// 
/// # Arguments
/// * `context_strategy` - The context strategy being used
/// * `interactive` - Whether in interactive mode
/// 
/// # Returns
/// * Formatted message string
pub fn format_context_strategy_message(context_strategy: &str, interactive: bool) -> String {
    match context_strategy {
        "clear" => format!("Context maxed out - automatically cleared session.\n{}", "-".repeat(50)),
        "truncate" => {
            if context_strategy == "truncate" {
                format!("Context maxed out - automatically truncated messages.\n{}\nGoose tried its best to truncate messages for you.", "-".repeat(50))
            } else {
                format!("Context maxed out\n{}\nGoose tried its best to truncate messages for you.", "-".repeat(50))
            }
        }
        "summarize" => {
            if context_strategy == "summarize" {
                "Goose automatically summarized messages for you.".to_string()
            } else if interactive {
                "Goose summarized messages for you.".to_string()
            } else {
                "Goose automatically summarized messages to continue processing.".to_string()
            }
        }
        _ => format!("Session cleared.\n{}", "-".repeat(50))
    }
}

/// Get default context strategy based on interactive mode
/// 
/// # Arguments
/// * `interactive` - Whether in interactive mode
/// 
/// # Returns
/// * Default context strategy string
pub fn get_default_context_strategy(interactive: bool) -> String {
    if interactive { 
        "prompt".to_string() 
    } else { 
        "summarize".to_string() 
    }
}

/// Format interruption notification message
/// 
/// # Arguments
/// * `interrupt` - Whether this was a user interruption
/// 
/// # Returns
/// * Formatted notification message
pub fn format_interruption_notification(interrupt: bool) -> String {
    if interrupt {
        "Interrupted by the user to make a correction".to_string()
    } else {
        "An uncaught error happened during tool use".to_string()
    }
}
