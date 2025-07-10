use std::sync::Arc;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde_json::json;
use tracing::{debug, error, info, warn};
use tokio::sync::RwLock;

use crate::agents::{
    SubagentTaskPayload,
    subagent_manager::SubAgentManager,
    subagent_types::SpawnSubAgentArgs,
    SubAgentStatus,
};
use crate::providers::base::Provider;
use crate::message::Content;

/// Error types specific to subagent task execution
#[derive(Debug, thiserror::Error)]
pub enum SubagentTaskError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Max turns exceeded: {0}")]
    MaxTurnsError(String),

    #[error("Task validation error: {0}")]
    ValidationError(String),
}

/// Represents the current state and progress of a subagent task
#[derive(Debug, Clone)]
pub struct SubagentTaskState {
    /// Unique identifier for the task
    pub id: String,
    
    /// Current status of the task
    pub status: String,
    
    /// Number of turns completed
    pub turn_count: usize,
    
    /// When the task started
    pub start_time: DateTime<Utc>,
    
    /// Last time the state was updated
    pub last_update: DateTime<Utc>,
    
    /// Description of the task being performed
    pub task_description: String,
    
    /// Optional recipe name if using a recipe
    pub recipe_name: Option<String>,
}

impl SubagentTaskState {
    /// Create a new task state
    pub fn new(id: String, task_description: String, recipe_name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            status: "initialized".to_string(),
            turn_count: 0,
            start_time: now,
            last_update: now,
            task_description,
            recipe_name,
        }
    }

    /// Update the task state
    pub fn update_status(&mut self, status: &str, turn_count: usize) {
        self.status = status.to_string();
        self.turn_count = turn_count;
        self.last_update = Utc::now();
        debug!(
            "Updated task {} status to {} (turn {})",
            self.id, status, turn_count
        );
    }

    /// Check if the task has exceeded its runtime
    pub fn check_timeout(&self, timeout_seconds: u64) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds();
        elapsed > timeout_seconds as i64
    }
}

/// Task execution context holding shared state
pub struct TaskExecutionContext {
    pub state: Arc<RwLock<SubagentTaskState>>,
    pub provider: Arc<dyn Provider>,
    pub subagent_manager: Arc<SubAgentManager>,
}

impl TaskExecutionContext {
    /// Create a new task execution context
    pub fn new(
        state: SubagentTaskState,
        provider: Arc<dyn Provider>,
        subagent_manager: Arc<SubAgentManager>,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(state)),
            provider,
            subagent_manager,
        }
    }
}

/// Execute a subagent task with the given payload
pub async fn execute_subagent_task(
    task_id: String,
    payload: SubagentTaskPayload,
    provider: Arc<dyn Provider>,
    subagent_manager: Arc<SubAgentManager>,
    namespace_manager: Arc<super::namespace::NamespaceManager>,
) -> Result<serde_json::Value, SubagentTaskError> {
    debug!("Starting execution of subagent task {}", task_id);

    // Validate the payload
    payload.validate().map_err(|e| SubagentTaskError::ValidationError(e.to_string()))?;

    // Create task state
    let state = SubagentTaskState::new(
        task_id.clone(),
        payload.task_description.clone(),
        payload.recipe_name.clone(),
    );

    // Create execution context
    let context = TaskExecutionContext::new(
        state,
        Arc::clone(&provider),
        Arc::clone(&subagent_manager),
    );

    // Execute the task
    execute_task_with_context(payload, context, namespace_manager).await
}

/// Execute a task with the given context
async fn execute_task_with_context(
    payload: SubagentTaskPayload,
    context: TaskExecutionContext,
    namespace_manager: Arc<super::namespace::NamespaceManager>,
) -> Result<serde_json::Value, SubagentTaskError> {
    let task_id = {
        let state = context.state.read().await;
        state.id.clone()
    };

    info!("Executing task {} with context", task_id);

    // Update state to running
    {
        let mut state = context.state.write().await;
        state.update_status("running", 0);
    }

    // Create subagent args
    let args = SpawnSubAgentArgs {
        instructions: Some(payload.instructions),
        recipe_name: payload.recipe_name,
        task: payload.task_description,
        max_turns: payload.max_turns,
        timeout_seconds: payload.timeout_seconds,
    };

    // Start timeout monitoring if specified
    let timeout_monitor = if let Some(timeout) = payload.timeout_seconds {
        let state_clone = Arc::clone(&context.state);
        let task_id_clone = task_id.clone();
        
        Some(tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let state = state_clone.read().await;
                if state.check_timeout(timeout) {
                    error!("Task {} timed out after {} seconds", task_id_clone, timeout);
                    return Some(SubagentTaskError::TimeoutError(format!(
                        "Task timed out after {} seconds",
                        timeout
                    )));
                }
                if state.status == "completed" || state.status == "failed" {
                    return None;
                }
            }
        }))
    } else {
        None
    };

    // Run the subagent task
    let result = match context
        .subagent_manager
        .run_complete_subagent_task(args, context.provider, get_extension_manager()?, namespace_manager)
        .await
    {
        Ok(output) => {
            info!("Task {} completed successfully", task_id);
            {
                let mut state = context.state.write().await;
                state.update_status("completed", state.turn_count);
            }
            Ok(json!({
                "status": "completed",
                "task_id": task_id,
                "output": output,
            }))
        }
        Err(e) => {
            error!("Task {} failed: {}", task_id, e);
            {
                let mut state = context.state.write().await;
                state.update_status("failed", state.turn_count);
            }
            Err(SubagentTaskError::ExecutionError(e.to_string()))
        }
    };

    // Cancel timeout monitor if it exists
    if let Some(handle) = timeout_monitor {
        handle.abort();
    }

    result
}

/// Helper function to get extension manager - implementation depends on your setup
fn get_extension_manager() -> Result<Arc<tokio::sync::RwLockReadGuard<'static, ExtensionManager>>, SubagentTaskError> {
    // Implementation needed - this is just a placeholder
    Err(SubagentTaskError::ConfigurationError(
        "Extension manager not implemented".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;
    use std::time::Duration;

    // Helper function to create a test provider
    fn create_test_provider() -> Arc<dyn Provider> {
        // Implementation needed - return a mock provider
        unimplemented!()
    }

    // Helper function to create a test subagent manager
    fn create_test_subagent_manager() -> Arc<SubAgentManager> {
        // Implementation needed - return a mock manager
        unimplemented!()
    }

    #[tokio::test]
    async fn test_task_state_timeout() {
        let state = SubagentTaskState::new(
            "test-1".to_string(),
            "Test task".to_string(),
            None,
        );
        
        assert!(!state.check_timeout(10)); // Should not timeout immediately
        
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(state.check_timeout(1)); // Should timeout after waiting
    }

    #[tokio::test]
    async fn test_task_state_updates() {
        let state = SubagentTaskState::new(
            "test-2".to_string(),
            "Test task".to_string(),
            Some("test-recipe".to_string()),
        );
        
        assert_eq!(state.status, "initialized");
        assert_eq!(state.turn_count, 0);
        
        let mut state = state;
        state.update_status("running", 1);
        
        assert_eq!(state.status, "running");
        assert_eq!(state.turn_count, 1);
    }

    #[tokio::test]
    async fn test_execute_subagent_task_validation() {
        let task_id = "test-3".to_string();
        let payload = SubagentTaskPayload::new(
            "".to_string(), // Empty instructions should fail validation
            "Test task".to_string(),
        );
        
        let result = execute_subagent_task(
            task_id,
            payload,
            create_test_provider(),
            create_test_subagent_manager(),
            Arc::new(super::namespace::NamespaceManager::new()),
        )
        .await;
        
        assert!(matches!(result, Err(SubagentTaskError::ValidationError(_))));
    }

    // Add more tests as needed
}
