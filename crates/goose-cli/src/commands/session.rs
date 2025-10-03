use crate::session::message_to_markdown;
use anyhow::{Context, Result};

use cliclack::{confirm, multiselect, select};
use goose::session::{Session, SessionManager};
use goose::utils::safe_truncate;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

const TRUNCATED_DESC_LENGTH: usize = 60;

pub async fn remove_sessions(sessions: Vec<Session>) -> Result<()> {
    println!("The following sessions will be removed:");
    for session in &sessions {
        println!("- {} {}", session.id, session.description);
    }

    let should_delete = confirm("Are you sure you want to delete these sessions?")
        .initial_value(false)
        .interact()?;

    if should_delete {
        for session in sessions {
            SessionManager::delete_session(&session.id).await?;
            println!("Session `{}` removed.", session.id);
        }
    } else {
        println!("Skipping deletion of the sessions.");
    }

    Ok(())
}

fn prompt_interactive_session_removal(sessions: &[Session]) -> Result<Vec<Session>> {
    if sessions.is_empty() {
        println!("No sessions to delete.");
        return Ok(vec![]);
    }

    let mut selector = multiselect(
        "Select sessions to delete (use spacebar, Enter to confirm, Ctrl+C to cancel):",
    );

    let display_map: std::collections::HashMap<String, Session> = sessions
        .iter()
        .map(|s| {
            let desc = if s.description.is_empty() {
                "(no description)"
            } else {
                &s.description
            };
            let truncated_desc = safe_truncate(desc, TRUNCATED_DESC_LENGTH);
            let display_text = format!("{} - {} ({})", s.updated_at, truncated_desc, s.id);
            (display_text, s.clone())
        })
        .collect();

    for display_text in display_map.keys() {
        selector = selector.item(display_text.clone(), display_text.clone(), "");
    }

    let selected_display_texts: Vec<String> = selector.interact()?;

    let selected_sessions: Vec<Session> = selected_display_texts
        .into_iter()
        .filter_map(|text| display_map.get(&text).cloned())
        .collect();

    Ok(selected_sessions)
}

pub async fn handle_session_remove(id: Option<String>, regex_string: Option<String>) -> Result<()> {
    let all_sessions = match SessionManager::list_sessions().await {
        Ok(sessions) => sessions,
        Err(e) => {
            tracing::error!("Failed to retrieve sessions: {:?}", e);
            return Err(anyhow::anyhow!("Failed to retrieve sessions"));
        }
    };

    let matched_sessions: Vec<Session>;

    if let Some(id_val) = id {
        if let Some(session) = all_sessions.iter().find(|s| s.id == id_val) {
            matched_sessions = vec![session.clone()];
        } else {
            return Err(anyhow::anyhow!("Session '{}' not found.", id_val));
        }
    } else if let Some(regex_val) = regex_string {
        let session_regex = Regex::new(&regex_val)
            .with_context(|| format!("Invalid regex pattern '{}'", regex_val))?;

        matched_sessions = all_sessions
            .into_iter()
            .filter(|session| session_regex.is_match(&session.id))
            .collect();

        if matched_sessions.is_empty() {
            println!("Regex string '{}' does not match any sessions", regex_val);
            return Ok(());
        }
    } else {
        if all_sessions.is_empty() {
            return Err(anyhow::anyhow!("No sessions found."));
        }
        matched_sessions = prompt_interactive_session_removal(&all_sessions)?;
    }

    if matched_sessions.is_empty() {
        return Ok(());
    }

    remove_sessions(matched_sessions).await
}

pub async fn handle_session_list(
    format: String,
    ascending: bool,
    working_dir: Option<PathBuf>,
    limit: Option<usize>,
) -> Result<()> {
    let mut sessions = SessionManager::list_sessions().await?;

    if let Some(ref pat) = working_dir {
        let pat_lower = pat.to_string_lossy().to_lowercase();
        sessions.retain(|s| {
            s.working_dir
                .to_string_lossy()
                .to_lowercase()
                .contains(&pat_lower)
        });
    }

    if ascending {
        sessions.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
    } else {
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    }

    if let Some(n) = limit {
        sessions.truncate(n);
    }

    match format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string(&sessions)?);
        }
        _ => {
            if sessions.is_empty() {
                println!("No sessions found");
                return Ok(());
            }

            println!("Available sessions:");
            for session in sessions {
                let output = format!(
                    "{} - {} - {}",
                    session.id, session.description, session.updated_at
                );
                println!("{}", output);
            }
        }
    }
    Ok(())
}

pub async fn handle_session_export(
    session_id: String,
    output_path: Option<PathBuf>,
    format: String,
) -> Result<()> {
    let session = match SessionManager::get_session(&session_id, true).await {
        Ok(session) => session,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Session '{}' not found or failed to read: {}",
                session_id,
                e
            ));
        }
    };

    let output = match format.as_str() {
        "json" => serde_json::to_string_pretty(&session)?,
        "yaml" => serde_yaml::to_string(&session)?,
        "markdown" => {
            let conversation = session
                .conversation
                .ok_or_else(|| anyhow::anyhow!("Session has no messages"))?;
            export_session_to_markdown(conversation.messages().to_vec(), &session.description)
        }
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    };

    if let Some(output_path) = output_path {
        fs::write(&output_path, output).with_context(|| {
            format!("Failed to write to output file: {}", output_path.display())
        })?;
        println!("Session exported to {}", output_path.display());
    } else {
        println!("{}", output);
    }

    Ok(())
}
/// Convert a list of messages to markdown format for session export
///
/// This function handles the formatting of a complete session including headers,
/// message organization, and proper tool request/response pairing.
fn export_session_to_markdown(
    messages: Vec<goose::conversation::message::Message>,
    session_name: &String,
) -> String {
    let mut markdown_output = String::new();

    markdown_output.push_str(&format!("# Session Export: {}\n\n", session_name));

    if messages.is_empty() {
        markdown_output.push_str("*(This session has no messages)*\n");
        return markdown_output;
    }

    markdown_output.push_str(&format!("*Total messages: {}*\n\n---\n\n", messages.len()));

    // Track if the last message had tool requests to properly handle tool responses
    let mut skip_next_if_tool_response = false;

    for message in &messages {
        // Check if this is a User message containing only ToolResponses
        let is_only_tool_response = message.role == rmcp::model::Role::User
            && message.content.iter().all(|content| {
                matches!(
                    content,
                    goose::conversation::message::MessageContent::ToolResponse(_)
                )
            });

        // If the previous message had tool requests and this one is just tool responses,
        // don't create a new User section - we'll attach the responses to the tool calls
        if skip_next_if_tool_response && is_only_tool_response {
            // Export the tool responses without a User heading
            markdown_output.push_str(&message_to_markdown(message, false));
            markdown_output.push_str("\n\n---\n\n");
            skip_next_if_tool_response = false;
            continue;
        }

        // Reset the skip flag - we'll update it below if needed
        skip_next_if_tool_response = false;

        // Output the role prefix except for tool response-only messages
        if !is_only_tool_response {
            let role_prefix = match message.role {
                rmcp::model::Role::User => "### User:\n",
                rmcp::model::Role::Assistant => "### Assistant:\n",
            };
            markdown_output.push_str(role_prefix);
        }

        // Add the message content
        markdown_output.push_str(&message_to_markdown(message, false));
        markdown_output.push_str("\n\n---\n\n");

        // Check if this message has any tool requests, to handle the next message differently
        if message.content.iter().any(|content| {
            matches!(
                content,
                goose::conversation::message::MessageContent::ToolRequest(_)
            )
        }) {
            skip_next_if_tool_response = true;
        }
    }

    markdown_output
}

/// Prompt the user to interactively select a session
///
/// Shows a list of available sessions and lets the user select one
pub async fn prompt_interactive_session_selection() -> Result<String> {
    let sessions = SessionManager::list_sessions().await?;

    if sessions.is_empty() {
        return Err(anyhow::anyhow!("No sessions found"));
    }

    // Build the selection prompt
    let mut selector = select("Select a session to export:");

    // Map to display text
    let display_map: std::collections::HashMap<String, Session> = sessions
        .iter()
        .map(|s| {
            let desc = if s.description.is_empty() {
                "(no description)"
            } else {
                &s.description
            };
            let truncated_desc = safe_truncate(desc, TRUNCATED_DESC_LENGTH);

            let display_text = format!("{} - {} ({})", s.updated_at, truncated_desc, s.id);
            (display_text, s.clone())
        })
        .collect();

    // Add each session as an option
    for display_text in display_map.keys() {
        selector = selector.item(display_text.clone(), display_text.clone(), "");
    }

    // Add a cancel option
    let cancel_value = String::from("cancel");
    selector = selector.item(cancel_value, "Cancel", "Cancel export");

    // Get user selection
    let selected_display_text: String = selector.interact()?;

    if selected_display_text == "cancel" {
        return Err(anyhow::anyhow!("Export canceled"));
    }

    // Retrieve the selected session
    if let Some(session) = display_map.get(&selected_display_text) {
        Ok(session.id.clone())
    } else {
        Err(anyhow::anyhow!("Invalid selection"))
    }
}
