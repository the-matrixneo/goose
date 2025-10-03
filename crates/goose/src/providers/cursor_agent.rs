use anyhow::Result;
use async_trait::async_trait;
use rmcp::model::Role;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage, Usage};
use super::errors::ProviderError;
use super::utils::emit_debug_trace;
use crate::conversation::message::{Message, MessageContent};
use crate::impl_provider_default;
use crate::model::ModelConfig;
use rmcp::model::Tool;

pub const CURSOR_AGENT_DEFAULT_MODEL: &str = "gpt-5";
pub const CURSOR_AGENT_KNOWN_MODELS: &[&str] = &["gpt-5", "opus-4.1", "sonnet-4"];

pub const CURSOR_AGENT_DOC_URL: &str = "https://docs.cursor.com/en/cli/overview";

#[derive(Debug, serde::Serialize)]
pub struct CursorAgentProvider {
    command: String,
    model: ModelConfig,
}

impl_provider_default!(CursorAgentProvider);

impl CursorAgentProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let command: String = config
            .get_param("CURSOR_AGENT_COMMAND")
            .unwrap_or_else(|_| "cursor-agent".to_string());

        let resolved_command = if !command.contains('/') {
            Self::find_cursor_agent_executable(&command).unwrap_or(command)
        } else {
            command
        };

        Ok(Self {
            command: resolved_command,
            model,
        })
    }

    /// Search for cursor-agent executable in common installation locations
    fn find_cursor_agent_executable(command_name: &str) -> Option<String> {
        let home = std::env::var("HOME").ok()?;

        let search_paths = vec![
            format!("/opt/homebrew/bin/{}", command_name),
            format!("/usr/bin/{}", command_name),
            format!("/usr/local/bin/{}", command_name),
            format!("{}/.local/bin/{}", home, command_name),
            format!("{}/bin/{}", home, command_name),
        ];

        for path in search_paths {
            let path_buf = PathBuf::from(&path);
            if path_buf.exists() && path_buf.is_file() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&path_buf) {
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 != 0 {
                            tracing::info!("Found cursor-agent executable at: {}", path);
                            return Some(path);
                        }
                    }
                }
                #[cfg(not(unix))]
                {
                    tracing::info!("Found cursor-agent executable at: {}", path);
                    return Some(path);
                }
            }
        }

        if let Ok(path_var) = std::env::var("PATH") {
            #[cfg(unix)]
            let path_separator = ':';
            #[cfg(windows)]
            let path_separator = ';';

            for dir in path_var.split(path_separator) {
                let path_buf = PathBuf::from(dir).join(command_name);
                if path_buf.exists() && path_buf.is_file() {
                    let full_path = path_buf.to_string_lossy().to_string();
                    tracing::info!("Found cursor-agent executable in PATH at: {}", full_path);
                    return Some(full_path);
                }
            }
        }

        tracing::warn!("Could not find cursor-agent executable in common locations");
        None
    }

    /// Filter out the Extensions section from the system prompt
    fn filter_extensions_from_system_prompt(&self, system: &str) -> String {
        // Find the Extensions section and remove it
        if let Some(extensions_start) = system.find("# Extensions") {
            // Look for the next major section that starts with #
            let after_extensions = &system[extensions_start..];
            if let Some(next_section_pos) = after_extensions[1..].find("\n# ") {
                // Found next section, keep everything before Extensions and after the next section
                let before_extensions = &system[..extensions_start];
                let next_section_start = extensions_start + next_section_pos + 1;
                let after_next_section = &system[next_section_start..];
                format!("{}{}", before_extensions.trim_end(), after_next_section)
            } else {
                // No next section found, just remove everything from Extensions onward
                system[..extensions_start].trim_end().to_string()
            }
        } else {
            // No Extensions section found, return original
            system.to_string()
        }
    }

    /// Convert goose messages to a simple prompt format for cursor-agent CLI
    fn messages_to_cursor_agent_format(&self, system: &str, messages: &[Message]) -> String {
        let mut full_prompt = String::new();

        // Add system prompt
        let filtered_system = self.filter_extensions_from_system_prompt(system);
        full_prompt.push_str(&filtered_system);
        full_prompt.push_str("\n\n");

        // Add conversation history
        for message in messages.iter().filter(|m| m.is_agent_visible()) {
            let role_prefix = match message.role {
                Role::User => "Human: ",
                Role::Assistant => "Assistant: ",
            };
            full_prompt.push_str(role_prefix);

            for content in &message.content {
                match content {
                    MessageContent::Text(text_content) => {
                        full_prompt.push_str(&text_content.text);
                        full_prompt.push('\n');
                    }
                    MessageContent::ToolRequest(tool_request) => {
                        if let Ok(tool_call) = &tool_request.tool_call {
                            full_prompt.push_str(&format!(
                                "Tool Use: {} with args: {:?}\n",
                                tool_call.name, tool_call.arguments
                            ));
                        }
                    }
                    MessageContent::ToolResponse(tool_response) => {
                        if let Ok(tool_contents) = &tool_response.tool_result {
                            let content_text = tool_contents
                                .iter()
                                .filter_map(|content| match &content.raw {
                                    rmcp::model::RawContent::Text(text_content) => {
                                        Some(text_content.text.as_str())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<&str>>()
                                .join("\n");

                            full_prompt.push_str(&format!("Tool Result: {}\n", content_text));
                        }
                    }
                    _ => {
                        // Skip other content types for now
                    }
                }
            }
            full_prompt.push('\n');
        }

        full_prompt.push_str("Assistant: ");
        full_prompt
    }

    /// Parse the JSON response from cursor-agent CLI
    fn parse_cursor_agent_response(
        &self,
        lines: &[String],
    ) -> Result<(Message, Usage), ProviderError> {
        // Try parsing each line as a JSON object and find the one with type="result"
        for line in lines {
            if let Ok(json_value) = serde_json::from_str::<Value>(line) {
                if let Some(type_val) = json_value.get("type") {
                    if type_val == "result" {
                        let text_content = if let Some(result) = json_value.get("result") {
                            let result_str = result.as_str().unwrap_or("").to_string();

                            if result_str.is_empty() {
                                if json_value
                                    .get("is_error")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false)
                                {
                                    "Error: cursor-agent returned an error response".to_string()
                                } else {
                                    "cursor-agent completed successfully but returned no content"
                                        .to_string()
                                }
                            } else {
                                result_str
                            }
                        } else {
                            format!("Raw cursor-agent response: {}", line)
                        };

                        let message_content = vec![MessageContent::text(text_content)];
                        let response_message = Message::new(
                            Role::Assistant,
                            chrono::Utc::now().timestamp(),
                            message_content,
                        );

                        let usage = Usage::default();

                        return Ok((response_message, usage));
                    }
                }
            }
        }

        // If no valid result line found, fallback to joining all lines
        let response_text = lines.join("\n");

        let message_content = vec![MessageContent::text(response_text)];
        let response_message = Message::new(
            Role::Assistant,
            chrono::Utc::now().timestamp(),
            message_content,
        );
        let usage = Usage::default();

        Ok((response_message, usage))
    }

    async fn execute_command(
        &self,
        system: &str,
        messages: &[Message],
        _tools: &[Tool],
    ) -> Result<Vec<String>, ProviderError> {
        let prompt = self.messages_to_cursor_agent_format(system, messages);

        if std::env::var("GOOSE_CURSOR_AGENT_DEBUG").is_ok() {
            println!("=== CURSOR AGENT PROVIDER DEBUG ===");
            println!("Command: {}", self.command);
            println!("Original system prompt length: {} chars", system.len());
            println!(
                "Filtered system prompt length: {} chars",
                self.filter_extensions_from_system_prompt(system).len()
            );
            println!("Full prompt: {}", prompt);
            println!("Model: {}", self.model.model_name);
            println!("================================");
        }

        let mut cmd = Command::new(&self.command);

        // Only pass model parameter if it's in the known models list
        if CURSOR_AGENT_KNOWN_MODELS.contains(&self.model.model_name.as_str()) {
            cmd.arg("-m").arg(&self.model.model_name);
        }

        cmd.arg("-p")
            .arg(&prompt)
            .arg("--output-format")
            .arg("json")
            .arg("--force");

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd
                .spawn()
                .map_err(|e| ProviderError::RequestFailed(format!(
                    "Failed to spawn cursor-agent CLI command '{}': {}. \
                    Make sure the cursor-agent CLI is installed and in your PATH, or set CURSOR_AGENT_COMMAND in your config to the correct path.",
                    self.command, e
                )))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| ProviderError::RequestFailed("Failed to capture stdout".to_string()))?;

        let mut reader = BufReader::new(stdout);
        let mut lines = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        lines.push(trimmed.to_string());
                    }
                }
                Err(e) => {
                    return Err(ProviderError::RequestFailed(format!(
                        "Failed to read output: {}",
                        e
                    )));
                }
            }
        }

        let exit_status = child.wait().await.map_err(|e| {
            ProviderError::RequestFailed(format!("Failed to wait for command: {}", e))
        })?;

        if !exit_status.success() {
            return Err(ProviderError::RequestFailed(format!(
                "Command failed with exit code: {:?}",
                exit_status.code()
            )));
        }

        tracing::debug!("Command executed successfully, got {} lines", lines.len());
        for (i, line) in lines.iter().enumerate() {
            tracing::debug!("Line {}: {}", i, line);
        }

        Ok(lines)
    }

    /// Generate a simple session description without calling subprocess
    fn generate_simple_session_description(
        &self,
        messages: &[Message],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        // Extract the first user message text
        let description = messages
            .iter()
            .find(|m| m.role == Role::User)
            .and_then(|m| {
                m.content.iter().find_map(|c| match c {
                    MessageContent::Text(text_content) => Some(&text_content.text),
                    _ => None,
                })
            })
            .map(|text| {
                // Take first few words, limit to 4 words
                text.split_whitespace()
                    .take(4)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_else(|| "Simple task".to_string());

        if std::env::var("GOOSE_CURSOR_AGENT_DEBUG").is_ok() {
            println!("=== CURSOR AGENT PROVIDER DEBUG ===");
            println!("Generated simple session description: {}", description);
            println!("Skipped subprocess call for session description");
            println!("================================");
        }

        let message = Message::new(
            Role::Assistant,
            chrono::Utc::now().timestamp(),
            vec![MessageContent::text(description.clone())],
        );

        let usage = Usage::default();

        Ok((
            message,
            ProviderUsage::new(self.model.model_name.clone(), usage),
        ))
    }
}

#[async_trait]
impl Provider for CursorAgentProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "cursor-agent",
            "Cursor Agent",
            "Execute AI models via cursor-agent CLI tool",
            CURSOR_AGENT_DEFAULT_MODEL,
            CURSOR_AGENT_KNOWN_MODELS.to_vec(),
            CURSOR_AGENT_DOC_URL,
            vec![ConfigKey::new(
                "CURSOR_AGENT_COMMAND",
                false,
                false,
                Some("cursor-agent"),
            )],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        // Return the model config with appropriate context limit for Cursor models
        self.model.clone()
    }

    #[tracing::instrument(
        skip(self, model_config, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete_with_model(
        &self,
        model_config: &ModelConfig,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        // Check if this is a session description request (short system prompt asking for 4 words or less)
        if system.contains("four words or less") || system.contains("4 words or less") {
            return self.generate_simple_session_description(messages);
        }

        let lines = self.execute_command(system, messages, tools).await?;

        let (message, usage) = self.parse_cursor_agent_response(&lines)?;

        // Create a dummy payload for debug tracing
        let payload = json!({
            "command": self.command,
            "model": model_config.model_name,
            "system": system,
            "messages": messages.len()
        });

        let response = json!({
            "lines": lines.len(),
            "usage": usage
        });

        emit_debug_trace(model_config, &payload, &response, &usage);

        Ok((
            message,
            ProviderUsage::new(model_config.model_name.clone(), usage),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::ModelConfig;
    use super::*;

    #[test]
    fn test_cursor_agent_model_config() {
        let provider = CursorAgentProvider::default();
        let config = provider.get_model_config();

        assert_eq!(config.model_name, "gpt-5");
        // Context limit should be set by the ModelConfig
        assert!(config.context_limit() > 0);
    }

    #[test]
    fn test_cursor_agent_invalid_model_no_fallback() {
        // Test that an invalid model is kept as-is (no fallback)
        let invalid_model = ModelConfig::new_or_fail("invalid-model");
        let provider = CursorAgentProvider::from_env(invalid_model).unwrap();
        let config = provider.get_model_config();

        assert_eq!(config.model_name, "invalid-model");
    }

    #[test]
    fn test_cursor_agent_valid_model() {
        // Test that a valid model is preserved
        let valid_model = ModelConfig::new_or_fail("gpt-5");
        let provider = CursorAgentProvider::from_env(valid_model).unwrap();
        let config = provider.get_model_config();

        assert_eq!(config.model_name, "gpt-5");
    }

    #[test]
    fn test_filter_extensions_from_system_prompt() {
        let provider = CursorAgentProvider::default();

        let system_with_extensions = "Some system prompt\n\n# Extensions\nSome extension info\n\n# Next Section\nMore content";
        let filtered = provider.filter_extensions_from_system_prompt(system_with_extensions);
        assert_eq!(filtered, "Some system prompt\n# Next Section\nMore content");

        let system_without_extensions = "Some system prompt\n\n# Other Section\nContent";
        let filtered = provider.filter_extensions_from_system_prompt(system_without_extensions);
        assert_eq!(filtered, system_without_extensions);
    }
}
