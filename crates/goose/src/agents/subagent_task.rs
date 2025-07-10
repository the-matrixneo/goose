use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::debug;

/// Payload configuration for a subagent task.
/// 
/// This structure represents the configuration needed to run a subagent task,
/// including required instructions and task description, as well as optional
/// parameters like recipe name, maximum turns, and timeout.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubagentTaskPayload {
    /// Direct instructions for the subagent's behavior and role
    pub instructions: String,
    
    /// The specific task description that the subagent should execute
    pub task_description: String,
    
    /// Optional recipe name for specialized behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_name: Option<String>,
    
    /// Optional maximum number of conversation turns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<usize>,
    
    /// Optional timeout in seconds for the entire task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u64>,
}

impl SubagentTaskPayload {
    /// Creates a new SubagentTaskPayload with required fields.
    /// 
    /// # Arguments
    /// 
    /// * `instructions` - The behavioral instructions for the subagent
    /// * `task_description` - The specific task to be performed
    /// 
    /// # Example
    /// 
    /// ```
    /// use goose::agents::subagent_task::SubagentTaskPayload;
    /// 
    /// let payload = SubagentTaskPayload::new(
    ///     "You are a code review assistant".to_string(),
    ///     "Review PR #123 for security issues".to_string(),
    /// );
    /// ```
    pub fn new(instructions: String, task_description: String) -> Self {
        debug!("Creating new SubagentTaskPayload");
        Self {
            instructions,
            task_description,
            recipe_name: None,
            max_turns: None,
            timeout_seconds: None,
        }
    }

    /// Validates the payload configuration.
    /// 
    /// Checks:
    /// - Instructions are not empty
    /// - Task description is not empty
    /// - Max turns is greater than 0 if specified
    /// - Timeout is greater than 0 if specified
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if validation passes
    /// * `Err` with description if validation fails
    pub fn validate(&self) -> Result<()> {
        debug!("Validating SubagentTaskPayload");

        if self.instructions.trim().is_empty() {
            return Err(anyhow!("Instructions cannot be empty"));
        }

        if self.task_description.trim().is_empty() {
            return Err(anyhow!("Task description cannot be empty"));
        }

        if let Some(max_turns) = self.max_turns {
            if max_turns == 0 {
                return Err(anyhow!("Max turns must be greater than 0"));
            }
        }

        if let Some(timeout) = self.timeout_seconds {
            if timeout == 0 {
                return Err(anyhow!("Timeout must be greater than 0 seconds"));
            }
        }

        Ok(())
    }

    /// Adds a recipe name to the payload.
    /// 
    /// # Arguments
    /// 
    /// * `recipe_name` - The name of the recipe to use
    pub fn with_recipe_name(mut self, recipe_name: String) -> Self {
        debug!("Adding recipe name: {}", recipe_name);
        self.recipe_name = Some(recipe_name);
        self
    }

    /// Sets the maximum number of conversation turns.
    /// 
    /// # Arguments
    /// 
    /// * `max_turns` - Maximum number of turns allowed
    pub fn with_max_turns(mut self, max_turns: usize) -> Self {
        debug!("Setting max turns: {}", max_turns);
        self.max_turns = Some(max_turns);
        self
    }

    /// Sets the timeout duration in seconds.
    /// 
    /// # Arguments
    /// 
    /// * `timeout_seconds` - Timeout duration in seconds
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        debug!("Setting timeout: {} seconds", timeout_seconds);
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Returns true if a recipe name is specified.
    pub fn has_recipe(&self) -> bool {
        self.recipe_name.is_some()
    }

    /// Returns true if max turns is specified.
    pub fn has_max_turns(&self) -> bool {
        self.max_turns.is_some()
    }

    /// Returns true if timeout is specified.
    pub fn has_timeout(&self) -> bool {
        self.timeout_seconds.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_payload() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        );
        assert!(payload.validate().is_ok());
    }

    #[test]
    fn test_empty_instructions() {
        let payload = SubagentTaskPayload::new(
            "".to_string(),
            "Review this PR".to_string(),
        );
        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_empty_task_description() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "".to_string(),
        );
        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_whitespace_only_instructions() {
        let payload = SubagentTaskPayload::new(
            "   ".to_string(),
            "Review this PR".to_string(),
        );
        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_invalid_max_turns() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        ).with_max_turns(0);
        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_invalid_timeout() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        ).with_timeout(0);
        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_json_serialization() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        )
        .with_recipe_name("code_review".to_string())
        .with_max_turns(5)
        .with_timeout(300);

        let json_str = serde_json::to_string(&payload).unwrap();
        let deserialized: SubagentTaskPayload = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(deserialized.instructions, "You are a code reviewer");
        assert_eq!(deserialized.task_description, "Review this PR");
        assert_eq!(deserialized.recipe_name, Some("code_review".to_string()));
        assert_eq!(deserialized.max_turns, Some(5));
        assert_eq!(deserialized.timeout_seconds, Some(300));
    }

    #[test]
    fn test_json_deserialization() {
        let json_value = json!({
            "instructions": "You are a code reviewer",
            "task_description": "Review this PR",
            "recipe_name": "code_review",
            "max_turns": 5,
            "timeout_seconds": 300
        });

        let payload: SubagentTaskPayload = serde_json::from_value(json_value).unwrap();
        assert!(payload.validate().is_ok());
        assert_eq!(payload.recipe_name, Some("code_review".to_string()));
        assert_eq!(payload.max_turns, Some(5));
        assert_eq!(payload.timeout_seconds, Some(300));
    }

    #[test]
    fn test_builder_methods() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        )
        .with_recipe_name("code_review".to_string())
        .with_max_turns(5)
        .with_timeout(300);

        assert!(payload.has_recipe());
        assert!(payload.has_max_turns());
        assert!(payload.has_timeout());
        assert_eq!(payload.recipe_name, Some("code_review".to_string()));
        assert_eq!(payload.max_turns, Some(5));
        assert_eq!(payload.timeout_seconds, Some(300));
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let payload = SubagentTaskPayload::new(
            "You are a code reviewer".to_string(),
            "Review this PR".to_string(),
        );

        let json_str = serde_json::to_string(&payload).unwrap();
        assert!(!json_str.contains("recipe_name"));
        assert!(!json_str.contains("max_turns"));
        assert!(!json_str.contains("timeout_seconds"));
    }
}
