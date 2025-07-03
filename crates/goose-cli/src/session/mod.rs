mod agent_response;
mod builder;
mod commands;
mod completion;
mod completion_cache;
mod export;
mod input;
mod interactive;
mod messages;
mod output;
mod prompt;
mod state;
mod thinking;
mod utils;

pub use self::export::message_to_markdown;
pub use builder::{build_session, SessionBuilderConfig, SessionSettings};
use console::Color;
use goose::agents::AgentEvent;
use goose::permission::permission_confirmation::PrincipalType;
use goose::permission::Permission;
use goose::permission::PermissionConfirmation;
pub use goose::session::Identifier;

use anyhow::Result;
use completion::GooseCompleter;
use completion_cache::CompletionCacheManager;
use etcetera::{choose_app_strategy, AppStrategy};
use goose::agents::{Agent, SessionConfig};
use goose::config::Config;
use goose::message::{Message, MessageContent};
use goose::session;
use input::InputResult;
use mcp_core::handler::ToolError;
use mcp_core::protocol::JsonRpcMessage;
use mcp_core::protocol::JsonRpcNotification;

use serde_json::Value;
use std::path::PathBuf;
use tokio;

pub enum RunMode {
    Normal,
    Plan,
}

pub struct Session {
    agent: Agent,
    messages: Vec<Message>,
    session_file: Option<PathBuf>,
    // Cache manager for completion data
    completion_cache_manager: CompletionCacheManager,
    debug: bool, // New field for debug mode
    run_mode: RunMode,
    scheduled_job_id: Option<String>, // ID of the scheduled job that triggered this session
    max_turns: Option<u32>,
}

pub use utils::{
    classify_planner_response, extract_session_id, get_reasoner, update_project_tracker,
    PlannerResponseType,
};

impl Session {
    pub fn new(
        agent: Agent,
        session_file: Option<PathBuf>,
        debug: bool,
        scheduled_job_id: Option<String>,
        max_turns: Option<u32>,
    ) -> Self {
        let messages = if let Some(session_file) = &session_file {
            match session::read_messages(session_file) {
                Ok(msgs) => msgs,
                Err(e) => {
                    eprintln!("Warning: Failed to load message history: {}", e);
                    Vec::new()
                }
            }
        } else {
            // Don't try to read messages if we're not saving sessions
            Vec::new()
        };

        Session {
            agent,
            messages,
            session_file,
            completion_cache_manager: CompletionCacheManager::new(),
            debug,
            run_mode: RunMode::Normal,
            scheduled_job_id,
            max_turns,
        }
    }

    /// Process a single message and get the response
    async fn process_message(&mut self, message: String) -> Result<()> {
        self.add_user_message_to_history(&message).await?;
        self.update_project_tracker(&message)?;
        self.process_agent_response(false).await?;
        Ok(())
    }

    /// Add a user message to the message history and persist it
    async fn add_user_message_to_history(&mut self, message: &str) -> Result<()> {
        self.messages.push(Message::user().with_text(message));

        // Get the provider from the agent for description generation
        let provider = self.agent.provider().await?;

        // Persist messages with provider for automatic description generation
        if let Some(session_file) = &self.session_file {
            session::persist_messages_with_schedule_id(
                session_file,
                &self.messages,
                Some(provider),
                self.scheduled_job_id.clone(),
            )
            .await?;
        }

        Ok(())
    }

    /// Update the project tracker with the current message
    fn update_project_tracker(&self, message: &str) -> Result<()> {
        let session_id = extract_session_id(&self.session_file);
        update_project_tracker(message, session_id.as_deref())
    }

    /// Set up the interactive editor with completer and configuration
    async fn setup_interactive_editor(
        &self,
    ) -> Result<rustyline::Editor<GooseCompleter, rustyline::history::DefaultHistory>> {
        let config = rustyline::Config::builder()
            .completion_type(rustyline::CompletionType::Circular)
            .build();
        let mut editor =
            rustyline::Editor::<GooseCompleter, rustyline::history::DefaultHistory>::with_config(
                config,
            )?;

        // Set up the completer with a reference to the completion cache
        let completer = GooseCompleter::new(self.completion_cache_manager.get_cache_ref());
        editor.set_helper(Some(completer));

        Ok(editor)
    }

    /// Get the path to the global history file
    fn get_history_file_path(&self) -> Result<PathBuf> {
        let strategy =
            choose_app_strategy(crate::APP_STRATEGY.clone()).expect("goose requires a home dir");
        let config_dir = strategy.config_dir();
        let history_file = config_dir.join("history.txt");

        // Ensure config directory exists
        if let Some(parent) = history_file.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(history_file)
    }

    /// Load command history from the global history file
    fn load_command_history(
        &self,
        editor: &mut rustyline::Editor<GooseCompleter, rustyline::history::DefaultHistory>,
        history_file: &PathBuf,
    ) {
        if history_file.exists() {
            if let Err(err) = editor.load_history(history_file) {
                eprintln!("Warning: Failed to load command history: {}", err);
            }
        }
    }

    /// Start an interactive session, optionally with an initial message
    pub async fn interactive(&mut self, message: Option<String>) -> Result<()> {
        // Process initial message if provided
        if let Some(msg) = message {
            self.process_message(msg).await?;
        }

        // Initialize the completion cache
        self.update_completion_cache().await?;

        let mut editor = self.setup_interactive_editor().await?;
        let history_file = self.get_history_file_path()?;
        self.load_command_history(&mut editor, &history_file);

        // Helper function to save history after commands
        let save_history =
            |editor: &mut rustyline::Editor<GooseCompleter, rustyline::history::DefaultHistory>| {
                if let Err(err) = editor.save_history(&history_file) {
                    eprintln!("Warning: Failed to save command history: {}", err);
                }
            };

        output::display_greeting();
        loop {
            // Display context usage before each prompt
            self.display_context_usage().await?;

            match input::get_input(&mut editor)? {
                input::InputResult::Message(content) => {
                    save_history(&mut editor);

                    match self.run_mode {
                        RunMode::Normal => {
                            self.handle_normal_mode_message(&content).await?;
                        }
                        RunMode::Plan => {
                            self.handle_plan_mode_message(&content).await?;
                        }
                    }
                }
                input::InputResult::Exit => break,
                input::InputResult::AddExtension(cmd) => {
                    save_history(&mut editor);

                    match self.add_extension(cmd.clone()).await {
                        Ok(_) => output::render_extension_success(&cmd),
                        Err(e) => output::render_extension_error(&cmd, &e.to_string()),
                    }
                }
                input::InputResult::AddBuiltin(names) => {
                    save_history(&mut editor);

                    match self.add_builtin(names.clone()).await {
                        Ok(_) => output::render_builtin_success(&names),
                        Err(e) => output::render_builtin_error(&names, &e.to_string()),
                    }
                }
                input::InputResult::ToggleTheme => {
                    save_history(&mut editor);
                    self.handle_theme_toggle();
                    continue;
                }
                input::InputResult::Retry => continue,
                input::InputResult::ListPrompts(extension) => {
                    save_history(&mut editor);

                    match self.list_prompts(extension).await {
                        Ok(prompts) => output::render_prompts(&prompts),
                        Err(e) => output::render_error(&e.to_string()),
                    }
                }
                input::InputResult::GooseMode(mode) => {
                    save_history(&mut editor);

                    if self.handle_goose_mode_setting(&mode)? {
                        continue;
                    }
                }
                input::InputResult::Plan(options) => {
                    save_history(&mut editor);
                    self.handle_plan_command(options).await?;
                }
                input::InputResult::EndPlan => {
                    self.run_mode = RunMode::Normal;
                    output::render_exit_plan_mode();
                    continue;
                }
                input::InputResult::Clear => {
                    save_history(&mut editor);
                    self.handle_clear_command();
                    continue;
                }
                input::InputResult::PromptCommand(opts) => {
                    save_history(&mut editor);
                    self.handle_prompt_command(opts).await?;
                }
                InputResult::Recipe(filepath_opt) => {
                    save_history(&mut editor);
                    self.handle_recipe_command(filepath_opt).await?;
                    continue;
                }
                InputResult::Summarize => {
                    save_history(&mut editor);
                    self.handle_summarize_command().await?;
                    continue;
                }
            }
        }

        println!(
            "\nClosing session.{}",
            self.session_file
                .as_ref()
                .map(|p| format!(" Recorded to {}", p.display()))
                .unwrap_or_default()
        );
        Ok(())
    }

    /// Process a single message and exit
    pub async fn headless(&mut self, message: String) -> Result<()> {
        self.process_message(message).await
    }

    async fn process_agent_response(&mut self, interactive: bool) -> Result<()> {
        let session_config = self.session_file.as_ref().map(|s| {
            let session_id = session::Identifier::Path(s.clone());
            SessionConfig {
                id: session_id.clone(),
                working_dir: std::env::current_dir()
                    .expect("failed to get current session working directory"),
                schedule_id: self.scheduled_job_id.clone(),
                execution_mode: None,
                max_turns: self.max_turns,
            }
        });
        let mut stream = self
            .agent
            .reply(&self.messages, session_config.clone())
            .await?;

        let mut progress_bars = output::McpSpinners::new();

        use futures::StreamExt;
        loop {
            tokio::select! {
                result = stream.next() => {
                    match result {
                        Some(Ok(AgentEvent::Message(message))) => {
                            // If it's a confirmation request, get approval but otherwise do not render/persist
                            if let Some(MessageContent::ToolConfirmationRequest(confirmation)) = message.content.first() {
                                output::hide_thinking();

                                // Format the confirmation prompt
                                let prompt = "Goose would like to call the above tool, do you allow?".to_string();

                                // Get confirmation from user
                                let permission_result = cliclack::select(prompt)
                                    .item(Permission::AllowOnce, "Allow", "Allow the tool call once")
                                    .item(Permission::AlwaysAllow, "Always Allow", "Always allow the tool call")
                                    .item(Permission::DenyOnce, "Deny", "Deny the tool call")
                                    .item(Permission::Cancel, "Cancel", "Cancel the AI response and tool call")
                                    .interact();

                                let permission = match permission_result {
                                    Ok(p) => p, // If Ok, use the selected permission
                                    Err(e) => {
                                        // Check if the error is an interruption (Ctrl+C/Cmd+C, Escape)
                                        if e.kind() == std::io::ErrorKind::Interrupted {
                                            Permission::Cancel // If interrupted, set permission to Cancel
                                        } else {
                                            return Err(e.into()); // Otherwise, convert and propagate the original error
                                        }
                                    }
                                };

                                if permission == Permission::Cancel {
                                    output::render_text("Tool call cancelled. Returning to chat...", Some(Color::Yellow), true);

                                    let mut response_message = Message::user();
                                    response_message.content.push(MessageContent::tool_response(
                                        confirmation.id.clone(),
                                        Err(ToolError::ExecutionError("Tool call cancelled by user".to_string()))
                                    ));
                                    self.messages.push(response_message);
                                    if let Some(session_file) = &self.session_file {
                                        session::persist_messages_with_schedule_id(
                                            session_file,
                                            &self.messages,
                                            None,
                                            self.scheduled_job_id.clone(),
                                        )
                                        .await?;
                                    }

                                    drop(stream);
                                    break;
                                } else {
                                    self.agent.handle_confirmation(confirmation.id.clone(), PermissionConfirmation {
                                        principal_type: PrincipalType::Tool,
                                        permission,
                                    },).await;
                                }
                            } else if let Some(MessageContent::ContextLengthExceeded(_)) = message.content.first() {
                                output::hide_thinking();

                                // Check for user-configured default context strategy
                                let config = Config::global();
                                let context_strategy = config.get_param::<String>("GOOSE_CONTEXT_STRATEGY")
                                    .unwrap_or_else(|_| if interactive { "prompt".to_string() } else { "summarize".to_string() });

                                let selected = match context_strategy.as_str() {
                                    "clear" => "clear",
                                    "truncate" => "truncate",
                                    "summarize" => "summarize",
                                    _ => {
                                        if interactive {
                                            // In interactive mode with no default, ask the user what to do
                                            let prompt = "The model's context length is maxed out. You will need to reduce the # msgs. Do you want to?".to_string();
                                            cliclack::select(prompt)
                                                .item("clear", "Clear Session", "Removes all messages from Goose's memory")
                                                .item("truncate", "Truncate Messages", "Removes old messages till context is within limits")
                                                .item("summarize", "Summarize Session", "Summarize the session to reduce context length")
                                                .interact()?
                                        } else {
                                            // In headless mode, default to summarize
                                            "summarize"
                                        }
                                    }
                                };

                                match selected {
                                    "clear" => {
                                        self.messages.clear();
                                        let msg = if context_strategy == "clear" {
                                            format!("Context maxed out - automatically cleared session.\n{}", "-".repeat(50))
                                        } else {
                                            format!("Session cleared.\n{}", "-".repeat(50))
                                        };
                                        output::render_text(&msg, Some(Color::Yellow), true);
                                        break;  // exit the loop to hand back control to the user
                                    }
                                    "truncate" => {
                                        // Truncate messages to fit within context length
                                        let (truncated_messages, _) = self.agent.truncate_context(&self.messages).await?;
                                        let msg = if context_strategy == "truncate" {
                                            format!("Context maxed out - automatically truncated messages.\n{}\nGoose tried its best to truncate messages for you.", "-".repeat(50))
                                        } else {
                                            format!("Context maxed out\n{}\nGoose tried its best to truncate messages for you.", "-".repeat(50))
                                        };
                                        output::render_text("", Some(Color::Yellow), true);
                                        output::render_text(&msg, Some(Color::Yellow), true);
                                        self.messages = truncated_messages;
                                    }
                                    "summarize" => {
                                        // Use the helper function to summarize context
                                        let message_suffix = if context_strategy == "summarize" {
                                            "Goose automatically summarized messages for you."
                                        } else if interactive {
                                            "Goose summarized messages for you."
                                        } else {
                                            "Goose automatically summarized messages to continue processing."
                                        };
                                        crate::session::commands::context::summarize_context_messages(&mut self.messages, &self.agent, message_suffix).await?;
                                    }
                                    _ => {
                                        unreachable!()
                                    }
                                }

                                // Restart the stream after handling ContextLengthExceeded
                                stream = self
                                    .agent
                                    .reply(
                                        &self.messages,
                                        session_config.clone(),
                                    )
                                    .await?;
                            }
                            // otherwise we have a model/tool to render
                            else {
                                self.messages.push(message.clone());

                                // No need to update description on assistant messages
                                if let Some(session_file) = &self.session_file {
                                    session::persist_messages_with_schedule_id(
                                        session_file,
                                        &self.messages,
                                        None,
                                        self.scheduled_job_id.clone(),
                                    )
                                    .await?;
                                }

                                if interactive {output::hide_thinking()};
                                let _ = progress_bars.hide();
                                output::render_message(&message, self.debug);
                                if interactive {output::show_thinking()};
                            }
                        }
                        Some(Ok(AgentEvent::McpNotification((_id, message)))) => {
                                if let JsonRpcMessage::Notification(JsonRpcNotification{
                                    method,
                                    params: Some(Value::Object(o)),
                                    ..
                                }) = message {
                                match method.as_str() {
                                    "notifications/message" => {
                                        let data = o.get("data").unwrap_or(&Value::Null);
                                        let (formatted_message, subagent_id, _notification_type) = match data {
                                            Value::String(s) => (s.clone(), None, None),
                                            Value::Object(o) => {
                                                // Check for subagent notification structure first
                                                if let Some(Value::String(msg)) = o.get("message") {
                                                    // Extract subagent info for better display
                                                    let subagent_id = o.get("subagent_id")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("unknown");
                                                    let notification_type = o.get("type")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("");

                                                    let formatted = match notification_type {
                                                        "subagent_created" | "completed" | "terminated" => {
                                                            format!("🤖 {}", msg)
                                                        }
                                                        "tool_usage" | "tool_completed" | "tool_error" => {
                                                            format!("🔧 {}", msg)
                                                        }
                                                        "message_processing" | "turn_progress" => {
                                                            format!("💭 {}", msg)
                                                        }
                                                        "response_generated" => {
                                                            // Check verbosity setting for subagent response content
                                                            let config = Config::global();
                                                            let min_priority = config
                                                                .get_param::<f32>("GOOSE_CLI_MIN_PRIORITY")
                                                                .ok()
                                                                .unwrap_or(0.5);

                                                            if min_priority > 0.1 && !self.debug {
                                                                // High/Medium verbosity: show truncated response
                                                                if let Some(response_content) = msg.strip_prefix("Responded: ") {
                                                                    if response_content.len() > 100 {
                                                                        format!("🤖 Responded: {}...", &response_content[..100])
                                                                    } else {
                                                                        format!("🤖 {}", msg)
                                                                    }
                                                                } else {
                                                                    format!("🤖 {}", msg)
                                                                }
                                                            } else {
                                                                // All verbosity or debug: show full response
                                                                format!("🤖 {}", msg)
                                                            }
                                                        }
                                                        _ => {
                                                            msg.to_string()
                                                        }
                                                    };
                                                    (formatted, Some(subagent_id.to_string()), Some(notification_type.to_string()))
                                                } else if let Some(Value::String(output)) = o.get("output") {
                                                    // Fallback for other MCP notification types
                                                    (output.to_owned(), None, None)
                                                } else {
                                                    (data.to_string(), None, None)
                                                }
                                            },
                                            v => {
                                                (v.to_string(), None, None)
                                            },
                                        };

                                        // Handle subagent notifications - show immediately
                                        if let Some(_id) = subagent_id {
                                            // Show subagent notifications immediately (no buffering) with compact spacing
                                            if interactive {
                                                let _ = progress_bars.hide();
                                                println!("{}", console::style(&formatted_message).green().dim());
                                            } else {
                                                progress_bars.log(&formatted_message);
                                            }
                                        } else {
                                            // Non-subagent notification, display immediately with compact spacing
                                            if interactive {
                                                let _ = progress_bars.hide();
                                                println!("{}", console::style(&formatted_message).green().dim());
                                            } else {
                                                progress_bars.log(&formatted_message);
                                            }
                                        }
                                    },
                                    "notifications/progress" => {
                                        let progress = o.get("progress").and_then(|v| v.as_f64());
                                        let token = o.get("progressToken").map(|v| v.to_string());
                                        let message = o.get("message").and_then(|v| v.as_str());
                                        let total = o
                                            .get("total")
                                            .and_then(|v| v.as_f64());
                                        if let (Some(progress), Some(token)) = (progress, token) {
                                            progress_bars.update(
                                                token.as_str(),
                                                progress,
                                                total,
                                                message,
                                            );
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        }
                        Some(Ok(AgentEvent::ModelChange { model, mode })) => {
                            // Log model change if in debug mode
                            if self.debug {
                                eprintln!("Model changed to {} in {} mode", model, mode);
                            }
                        }

                        Some(Err(e)) => {
                            eprintln!("Error: {}", e);
                            drop(stream);
                            if let Err(e) = self.handle_interrupted_messages(false).await {
                                eprintln!("Error handling interruption: {}", e);
                            }
                            output::render_error(
                                "The error above was an exception we were not able to handle.\n\
                                These errors are often related to connection or authentication\n\
                                We've removed the conversation up to the most recent user message\n\
                                - depending on the error you may be able to continue",
                            );
                            break;
                        }
                        None => break,
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    drop(stream);
                    if let Err(e) = self.handle_interrupted_messages(true).await {
                        eprintln!("Error handling interruption: {}", e);
                    }
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_interrupted_messages(&mut self, interrupt: bool) -> Result<()> {
        // First, get any tool requests from the last message if it exists
        let tool_requests = self
            .messages
            .last()
            .filter(|msg| msg.role == mcp_core::role::Role::Assistant)
            .map_or(Vec::new(), |msg| {
                msg.content
                    .iter()
                    .filter_map(|content| {
                        if let MessageContent::ToolRequest(req) = content {
                            Some((req.id.clone(), req.tool_call.clone()))
                        } else {
                            None
                        }
                    })
                    .collect()
            });

        if !tool_requests.is_empty() {
            // Interrupted during a tool request
            // Create tool responses for all interrupted tool requests
            let mut response_message = Message::user();
            let last_tool_name = tool_requests
                .last()
                .and_then(|(_, tool_call)| tool_call.as_ref().ok().map(|tool| tool.name.clone()))
                .unwrap_or_else(|| "tool".to_string());

            let notification = if interrupt {
                "Interrupted by the user to make a correction".to_string()
            } else {
                "An uncaught error happened during tool use".to_string()
            };
            for (req_id, _) in &tool_requests {
                response_message.content.push(MessageContent::tool_response(
                    req_id.clone(),
                    Err(ToolError::ExecutionError(notification.clone())),
                ));
            }
            self.messages.push(response_message);

            // No need for description update here
            if let Some(session_file) = &self.session_file {
                session::persist_messages_with_schedule_id(
                    session_file,
                    &self.messages,
                    None,
                    self.scheduled_job_id.clone(),
                )
                .await?;
            }

            let prompt = format!(
                "The existing call to {} was interrupted. How would you like to proceed?",
                last_tool_name
            );
            self.messages.push(Message::assistant().with_text(&prompt));

            // No need for description update here
            if let Some(session_file) = &self.session_file {
                session::persist_messages_with_schedule_id(
                    session_file,
                    &self.messages,
                    None,
                    self.scheduled_job_id.clone(),
                )
                .await?;
            }

            output::render_message(&Message::assistant().with_text(&prompt), self.debug);
        } else {
            // An interruption occurred outside of a tool request-response.
            if let Some(last_msg) = self.messages.last() {
                if last_msg.role == mcp_core::role::Role::User {
                    match last_msg.content.first() {
                        Some(MessageContent::ToolResponse(_)) => {
                            // Interruption occurred after a tool had completed but not assistant reply
                            let prompt = "The tool calling loop was interrupted. How would you like to proceed?";
                            self.messages.push(Message::assistant().with_text(prompt));

                            // No need for description update here
                            if let Some(session_file) = &self.session_file {
                                session::persist_messages_with_schedule_id(
                                    session_file,
                                    &self.messages,
                                    None,
                                    self.scheduled_job_id.clone(),
                                )
                                .await?;
                            }

                            output::render_message(
                                &Message::assistant().with_text(prompt),
                                self.debug,
                            );
                        }
                        Some(_) => {
                            // A real users message
                            self.messages.pop();
                            let prompt = "Interrupted before the model replied and removed the last message.";
                            output::render_message(
                                &Message::assistant().with_text(prompt),
                                self.debug,
                            );
                        }
                        None => panic!("No content in last message"),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn session_file(&self) -> Option<PathBuf> {
        self.session_file.clone()
    }

    /// Update the completion cache with fresh data
    /// This should be called before the interactive session starts
    pub async fn update_completion_cache(&mut self) -> Result<()> {
        self.completion_cache_manager
            .update_cache(&self.agent)
            .await
    }

    /// Invalidate the completion cache
    /// This should be called when extensions are added or removed
    pub async fn invalidate_completion_cache(&self) {
        self.completion_cache_manager.invalidate_cache();
    }

    pub fn message_history(&self) -> Vec<Message> {
        self.messages.clone()
    }

    /// Render all past messages from the session history
    pub fn render_message_history(&self) {
        if self.messages.is_empty() {
            return;
        }

        // Print session restored message
        println!(
            "\n{} {} messages loaded into context.",
            console::style("Session restored:").green().bold(),
            console::style(self.messages.len()).green()
        );

        // Render each message
        for message in &self.messages {
            output::render_message(message, self.debug);
        }

        // Add a visual separator after restored messages
        println!(
            "\n{}\n",
            console::style("──────── New Messages ────────").dim()
        );
    }

    pub fn get_metadata(&self) -> Result<session::SessionMetadata> {
        if !self.session_file.as_ref().is_some_and(|f| f.exists()) {
            return Err(anyhow::anyhow!("Session file does not exist"));
        }

        session::read_metadata(self.session_file.as_ref().unwrap())
    }

    // Get the session's total token usage
    pub fn get_total_token_usage(&self) -> Result<Option<i32>> {
        let metadata = self.get_metadata()?;
        Ok(metadata.total_tokens)
    }
}
