mod agent_response_handler;
mod builder;
mod commands;
mod completion;
mod completion_cache;
mod export;
mod input;
mod message_handler;
mod output;
mod prompt;
mod thinking;
mod utils;

pub use self::export::message_to_markdown;
pub use builder::{build_session, SessionBuilderConfig, SessionSettings};
pub use goose::session::Identifier;

use anyhow::Result;
use completion::GooseCompleter;
use completion_cache::CompletionCacheManager;
use etcetera::{choose_app_strategy, AppStrategy};
use goose::agents::Agent;
use goose::message::Message;
use goose::session;
use input::InputResult;
use std::path::PathBuf;

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
