use anyhow::Result;
use goose::config::Config;
use serde_json::Value;

use crate::session::{output, Session};

impl Session {
    /// Handle theme toggle command
    pub fn handle_theme_toggle(&self) {
        let current = output::get_theme();
        let new_theme = match current {
            output::Theme::Light => {
                println!("Switching to Dark theme");
                output::Theme::Dark
            }
            output::Theme::Dark => {
                println!("Switching to Ansi theme");
                output::Theme::Ansi
            }
            output::Theme::Ansi => {
                println!("Switching to Light theme");
                output::Theme::Light
            }
        };
        output::set_theme(new_theme);
    }

    /// Handle goose mode setting command
    pub fn handle_goose_mode_setting(&self, mode: &str) -> Result<bool> {
        let config = Config::global();
        let mode = mode.to_lowercase();

        // Check if mode is valid
        if !["auto", "approve", "chat", "smart_approve"].contains(&mode.as_str()) {
            output::render_error(&format!(
                "Invalid mode '{}'. Mode must be one of: auto, approve, chat",
                mode
            ));
            return Ok(false);
        }

        config
            .set_param("GOOSE_MODE", Value::String(mode.to_string()))
            .unwrap();
        output::goose_mode_message(&format!("Goose mode set to '{}'", mode));

        Ok(true)
    }
}
