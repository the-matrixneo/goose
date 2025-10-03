use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};
use std::borrow::Cow;
use std::sync::Arc;

use super::CompletionCache;

/// Completer for goose CLI commands
pub struct GooseCompleter {
    completion_cache: Arc<std::sync::RwLock<CompletionCache>>,
    filename_completer: FilenameCompleter,
}

impl GooseCompleter {
    /// Create a new GooseCompleter with a reference to the Session's completion cache
    pub fn new(completion_cache: Arc<std::sync::RwLock<CompletionCache>>) -> Self {
        Self {
            completion_cache,
            filename_completer: FilenameCompleter::new(),
        }
    }

    /// Complete prompt names for the /prompt command
    fn complete_prompt_names(&self, line: &str) -> Result<(usize, Vec<Pair>)> {
        // Get the prefix of the prompt name being typed
        let prefix = if line.len() > 8 { &line[8..] } else { "" };

        // Get available prompts from cache
        let cache = self.completion_cache.read().unwrap();

        // Create completion candidates that match the prefix
        let candidates: Vec<Pair> = cache
            .prompts
            .iter()
            .flat_map(|(_, names)| names)
            .filter(|name| name.starts_with(prefix.trim()))
            .map(|name| Pair {
                display: name.clone(),
                replacement: name.clone(),
            })
            .collect();

        Ok((8, candidates))
    }

    /// Complete flags for the /prompt command
    fn complete_prompt_flags(&self, line: &str) -> Result<(usize, Vec<Pair>)> {
        // Get the last part of the line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(last_part) = parts.last() {
            // If the last part starts with '-', it might be a partial flag
            if last_part.starts_with('-') {
                // Define available flags
                let flags = ["--info"];

                // Find flags that match the prefix
                let matching_flags: Vec<Pair> = flags
                    .iter()
                    .filter(|flag| flag.starts_with(last_part))
                    .map(|flag| Pair {
                        display: flag.to_string(),
                        replacement: flag.to_string(),
                    })
                    .collect();

                if !matching_flags.is_empty() {
                    // Return matches for the partial flag
                    // The position is the start of the last word
                    let pos = line.len() - last_part.len();
                    return Ok((pos, matching_flags));
                }
            }
        }

        // No flag completions available
        Ok((line.len(), vec![]))
    }

    /// Complete flags for the /mode command
    fn complete_mode_flags(&self, line: &str) -> Result<(usize, Vec<Pair>)> {
        let modes = ["auto", "approve", "smart_approve", "chat"];

        let parts: Vec<&str> = line.split_whitespace().collect();

        // If we're just after "/mode" with a space, show all options
        if line == "/mode " {
            return Ok((
                line.len(),
                modes
                    .iter()
                    .map(|mode| Pair {
                        display: mode.to_string(),
                        replacement: format!("{} ", mode),
                    })
                    .collect(),
            ));
        }

        // If we're typing a mode name, show the flags for that mode
        if parts.len() == 2 {
            let partial = parts[1].to_lowercase();
            return Ok((
                line.len() - partial.len(),
                modes
                    .iter()
                    .filter(|mode| mode.to_lowercase().starts_with(&partial.to_lowercase()))
                    .map(|mode| Pair {
                        display: mode.to_string(),
                        replacement: format!("{} ", mode),
                    })
                    .collect(),
            ));
        }

        // No completions available
        Ok((line.len(), vec![]))
    }

    /// Complete slash commands
    fn complete_slash_commands(&self, line: &str) -> Result<(usize, Vec<Pair>)> {
        // Define available slash commands
        let commands = [
            "/exit",
            "/quit",
            "/help",
            "/?",
            "/t",
            "/extension",
            "/builtin",
            "/prompts",
            "/prompt",
            "/mode",
            "/recipe",
        ];

        // Find commands that match the prefix
        let matching_commands: Vec<Pair> = commands
            .iter()
            .filter(|cmd| cmd.starts_with(line))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: format!("{} ", cmd), // Add a space after the command
            })
            .collect();

        if !matching_commands.is_empty() {
            return Ok((0, matching_commands));
        }

        // No command completions available
        Ok((line.len(), vec![]))
    }

    /// Complete argument keys for a specific prompt
    fn complete_argument_keys(&self, line: &str) -> Result<(usize, Vec<Pair>)> {
        let parts: Vec<&str> = line[8..].split_whitespace().collect();

        // We need at least the prompt name
        if parts.is_empty() {
            return Ok((line.len(), vec![]));
        }

        let prompt_name = parts[0];

        // Get prompt info from cache
        let cache = self.completion_cache.read().unwrap();
        let prompt_info = cache.prompt_info.get(prompt_name).cloned();

        if let Some(info) = prompt_info {
            if let Some(args) = info.arguments {
                // Find required arguments that haven't been provided yet
                let existing_args: Vec<&str> = parts
                    .iter()
                    .skip(1)
                    .filter_map(|part| {
                        if part.contains('=') {
                            Some(part.split('=').next().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();

                // Check if we're trying to complete a partial argument name
                if let Some(last_part) = parts.last() {
                    // ignore if last_part starts with = / \ for suggestions
                    if let Some(c) = last_part.chars().next() {
                        if matches!(c, '=' | '/' | '\\') {
                            return Ok((line.len(), vec![]));
                        }
                    }

                    // If the last part doesn't contain '=', it might be a partial argument name
                    if !last_part.contains('=') {
                        // Find arguments that match the prefix
                        let matching_args: Vec<Pair> = args
                            .iter()
                            .filter(|arg| {
                                arg.name.starts_with(last_part)
                                    && !existing_args.contains(&arg.name.as_str())
                            })
                            .map(|arg| Pair {
                                display: format!("{}=", arg.name),
                                replacement: format!("{}=", arg.name),
                            })
                            .collect();

                        if !matching_args.is_empty() {
                            // Return matches for the partial argument name
                            // The position is the start of the last word
                            let pos = line.len() - last_part.len();
                            return Ok((pos, matching_args));
                        }

                        // If we have a partial argument that doesn't match anything,
                        // return an empty list rather than suggesting unrelated arguments
                        if !last_part.is_empty() && *last_part != prompt_name {
                            return Ok((line.len(), vec![]));
                        }
                    }
                }

                // If no partial match or no last part, suggest all required arguments
                // Use a reference to avoid moving args
                let mut candidates: Vec<_> = Vec::new();
                for arg in &args {
                    if arg.required.unwrap_or(false) && !existing_args.contains(&arg.name.as_str())
                    {
                        candidates.push(Pair {
                            display: format!("{}=", arg.name),
                            replacement: format!("{}=", arg.name),
                        });
                    }
                }

                if !candidates.is_empty() {
                    return Ok((line.len(), candidates));
                }

                // If no required arguments left, suggest all optional ones
                // Use a reference to avoid moving args
                for arg in &args {
                    if !arg.required.unwrap_or(true) && !existing_args.contains(&arg.name.as_str())
                    {
                        candidates.push(Pair {
                            display: format!("{}=", arg.name),
                            replacement: format!("{}=", arg.name),
                        });
                    }
                }
                return Ok((line.len(), candidates));
            }
        }

        // No completions available
        Ok((line.len(), vec![]))
    }

    /// Complete file paths
    fn complete_file_path(&self, line: &str, ctx: &Context) -> Result<(usize, Vec<Pair>)> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let Some(last_part) = parts.last() {
            // Skip filename completion for words starting with special characters
            if last_part.starts_with('/') && last_part.len() == 1 {
                // Just a slash - no completion
                return Ok((line.len(), vec![]));
            }

            if last_part.starts_with('-') || last_part.contains('=') {
                // Skip flag or key-value pairs
                return Ok((line.len(), vec![]));
            }

            // Complete the partial path
            let pos = line.len() - last_part.len();
            let (start, candidates) =
                self.filename_completer
                    .complete(last_part, last_part.len(), ctx)?;

            // Return the completion results, with adjusted position
            return Ok((pos + start, candidates));
        }

        Ok((line.len(), vec![]))
    }
}

impl Completer for GooseCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        // If the cursor is not at the end of the line, don't try to complete
        if pos < line.len() {
            return Ok((pos, vec![]));
        }

        // If the line starts with '/', it might be a slash command
        if line.starts_with('/') {
            // If it's just a partial slash command (no space yet)
            if !line.contains(' ') {
                return self.complete_slash_commands(line);
            }

            // Handle /prompt command
            if line.starts_with("/prompt") {
                // If we're just after "/prompt" with or without a space
                if line == "/prompt" || line == "/prompt " {
                    return self.complete_prompt_names(line);
                }

                // Get the parts of the command
                let parts: Vec<&str> = line.split_whitespace().collect();

                // If we're typing a prompt name (only one part after /prompt)
                if parts.len() == 2 && !line.ends_with(' ') {
                    return self.complete_prompt_names(line);
                }

                // Check if we might be typing a flag
                if let Some(last_part) = parts.last() {
                    if last_part.starts_with('-') {
                        return self.complete_prompt_flags(line);
                    }
                }

                // If we have a prompt name and need argument completion
                if parts.len() >= 2 {
                    return self.complete_argument_keys(line);
                }
            }

            // Handle /prompts command
            if line.starts_with("/prompts") {
                // If we're just after "/prompts" with a space
                if line == "/prompts " {
                    // Suggest the --extension flag
                    return Ok((
                        line.len(),
                        vec![Pair {
                            display: "--extension".to_string(),
                            replacement: "--extension ".to_string(),
                        }],
                    ));
                }

                // Check if we might be typing the --extension flag
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2
                    && parts[1].starts_with('-')
                    && "--extension".starts_with(parts[1])
                {
                    return Ok((
                        line.len() - parts[1].len(),
                        vec![Pair {
                            display: "--extension".to_string(),
                            replacement: "--extension ".to_string(),
                        }],
                    ));
                }
            }

            if line.starts_with("/mode") {
                return self.complete_mode_flags(line);
            }

            return Ok((pos, vec![]));
        }

        // For normal text (not slash commands), try file path completion
        self.complete_file_path(line, ctx)
    }
}

// Implement the Helper trait which is required by rustyline
impl Helper for GooseCompleter {}

// Implement required traits with default implementations
impl Hinter for GooseCompleter {
    type Hint = String;

    fn hint(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        // Only show hint when line is empty
        if line.is_empty() {
            Some("Press Enter to send, Ctrl-J for new line".to_string())
        } else {
            None
        }
    }
}

impl Highlighter for GooseCompleter {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // Style the hint text with a dim color
        let styled = console::Style::new().dim().apply_to(hint).to_string();
        Cow::Owned(styled)
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _cmd_kind: CmdKind) -> bool {
        false
    }
}

impl Validator for GooseCompleter {
    fn validate(
        &self,
        _ctx: &mut rustyline::validate::ValidationContext,
    ) -> Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}

#[cfg(test)]
mod tests {
    use rmcp::model::PromptArgument;

    use super::*;
    use crate::session::output;
    use std::sync::{Arc, RwLock};

    // Helper function to create a test completion cache
    fn create_test_cache() -> Arc<RwLock<CompletionCache>> {
        let mut cache = CompletionCache::new();

        // Add some test prompts
        cache.prompts.insert(
            "extension1".to_string(),
            vec!["test_prompt1".to_string(), "test_prompt2".to_string()],
        );

        cache
            .prompts
            .insert("extension2".to_string(), vec!["other_prompt".to_string()]);

        // Add prompt info with arguments
        let test_prompt1_args = vec![
            PromptArgument {
                name: "required_arg".to_string(),
                description: Some("A required argument".to_string()),
                required: Some(true),
                title: None,
            },
            PromptArgument {
                name: "optional_arg".to_string(),
                description: Some("An optional argument".to_string()),
                required: Some(false),
                title: None,
            },
        ];

        let test_prompt1_info = output::PromptInfo {
            name: "test_prompt1".to_string(),
            description: Some("Test prompt 1 description".to_string()),
            arguments: Some(test_prompt1_args),
            extension: Some("extension1".to_string()),
        };
        cache
            .prompt_info
            .insert("test_prompt1".to_string(), test_prompt1_info);

        let test_prompt2_info = output::PromptInfo {
            name: "test_prompt2".to_string(),
            description: Some("Test prompt 2 description".to_string()),
            arguments: None,
            extension: Some("extension1".to_string()),
        };
        cache
            .prompt_info
            .insert("test_prompt2".to_string(), test_prompt2_info);

        let other_prompt_info = output::PromptInfo {
            name: "other_prompt".to_string(),
            description: Some("Other prompt description".to_string()),
            arguments: None,
            extension: Some("extension2".to_string()),
        };
        cache
            .prompt_info
            .insert("other_prompt".to_string(), other_prompt_info);

        Arc::new(RwLock::new(cache))
    }

    #[test]
    fn test_complete_slash_commands() {
        let cache = create_test_cache();
        let completer = GooseCompleter::new(cache);

        // Test complete match
        let (pos, candidates) = completer.complete_slash_commands("/exit").unwrap();
        assert_eq!(pos, 0);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "/exit");
        assert_eq!(candidates[0].replacement, "/exit ");

        // Test partial match
        let (pos, candidates) = completer.complete_slash_commands("/e").unwrap();
        assert_eq!(pos, 0);
        // There might be multiple commands starting with "e" like "/exit" and "/extension"
        assert!(!candidates.is_empty());

        // Test multiple matches
        let (pos, candidates) = completer.complete_slash_commands("/").unwrap();
        assert_eq!(pos, 0);
        assert!(candidates.len() > 1);

        // Test no match
        let (_pos, candidates) = completer.complete_slash_commands("/nonexistent").unwrap();
        assert_eq!(candidates.len(), 0);
    }

    #[test]
    fn test_complete_prompt_names() {
        let cache = create_test_cache();
        let completer = GooseCompleter::new(cache);

        // Test with just "/prompt "
        let (pos, candidates) = completer.complete_prompt_names("/prompt ").unwrap();
        assert_eq!(pos, 8);
        assert_eq!(candidates.len(), 3); // All prompts

        // Test with partial prompt name
        let (pos, candidates) = completer.complete_prompt_names("/prompt test").unwrap();
        assert_eq!(pos, 8);
        assert_eq!(candidates.len(), 2); // test_prompt1 and test_prompt2

        // Test with specific prompt name
        let (pos, candidates) = completer
            .complete_prompt_names("/prompt test_prompt1")
            .unwrap();
        assert_eq!(pos, 8);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "test_prompt1");

        // Test with no match
        let (pos, candidates) = completer
            .complete_prompt_names("/prompt nonexistent")
            .unwrap();
        assert_eq!(pos, 8);
        assert_eq!(candidates.len(), 0);
    }

    #[test]
    fn test_complete_prompt_flags() {
        let cache = create_test_cache();
        let completer = GooseCompleter::new(cache);

        // Test with partial flag
        let (_pos, candidates) = completer
            .complete_prompt_flags("/prompt test_prompt1 --")
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "--info");

        // Test with exact flag
        let (_pos, candidates) = completer
            .complete_prompt_flags("/prompt test_prompt1 --info")
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "--info");

        // Test with no match
        let (_pos, candidates) = completer
            .complete_prompt_flags("/prompt test_prompt1 --nonexistent")
            .unwrap();
        assert_eq!(candidates.len(), 0);

        // Test with no flag
        let (_pos, candidates) = completer
            .complete_prompt_flags("/prompt test_prompt1")
            .unwrap();
        assert_eq!(candidates.len(), 0);
    }

    #[test]
    fn test_complete_argument_keys() {
        let cache = create_test_cache();
        let completer = GooseCompleter::new(cache);

        // Test with just a prompt name (no space after)
        // This case doesn't return any candidates in the current implementation
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt test_prompt1")
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "required_arg=");

        // Test with partial argument
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt test_prompt1 req")
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "required_arg=");

        // Test with one argument already provided
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt test_prompt1 required_arg=value")
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].display, "optional_arg=");

        // Test with all arguments provided
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt test_prompt1 required_arg=value optional_arg=value")
            .unwrap();
        assert_eq!(candidates.len(), 0);

        // Test with prompt that has no arguments
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt test_prompt2")
            .unwrap();
        assert_eq!(candidates.len(), 0);

        // Test with nonexistent prompt
        let (_pos, candidates) = completer
            .complete_argument_keys("/prompt nonexistent")
            .unwrap();
        assert_eq!(candidates.len(), 0);
    }
}
