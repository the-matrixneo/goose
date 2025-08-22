use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::agents::subagent_execution_tool::task_execution_tracker::TaskExecutionTracker;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    #[default]
    Sequential,
    Parallel,
}

/// Filter for selecting which extensions to load in a subagent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum ExtensionFilter {
    /// Load only specified extensions (by normalized name)
    Include { extensions: Vec<String> },
    /// Load all except specified extensions
    Exclude { extensions: Vec<String> },
    /// Load no extensions
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: String,
    pub payload: Value,
    /// Optional filter for which extensions to load in the subagent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_filter: Option<ExtensionFilter>,
}

impl Task {
    pub fn get_sub_recipe(&self) -> Option<&Map<String, Value>> {
        (self.task_type == "sub_recipe")
            .then(|| self.payload.get("sub_recipe")?.as_object())
            .flatten()
    }

    pub fn get_command_parameters(&self) -> Option<&Map<String, Value>> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("command_parameters"))
            .and_then(|cp| cp.as_object())
    }

    pub fn get_sequential_when_repeated(&self) -> bool {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("sequential_when_repeated").and_then(|v| v.as_bool()))
            .unwrap_or_default()
    }

    pub fn get_sub_recipe_name(&self) -> Option<&str> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("name"))
            .and_then(|name| name.as_str())
    }

    pub fn get_sub_recipe_path(&self) -> Option<&str> {
        self.get_sub_recipe()
            .and_then(|sr| sr.get("recipe_path"))
            .and_then(|path| path.as_str())
    }

    pub fn get_text_instruction(&self) -> Option<&str> {
        if self.task_type != "sub_recipe" {
            self.payload
                .get("text_instruction")
                .and_then(|text| text.as_str())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub task: Task,
    pub status: TaskStatus,
    pub start_time: Option<tokio::time::Instant>,
    pub end_time: Option<tokio::time::Instant>,
    pub result: Option<TaskResult>,
    pub current_output: String,
}

impl TaskInfo {
    pub fn error(&self) -> Option<&String> {
        self.result.as_ref().and_then(|r| r.error.as_ref())
    }

    pub fn data(&self) -> Option<&Value> {
        self.result.as_ref().and_then(|r| r.data.as_ref())
    }
}

pub struct SharedState {
    pub task_receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<Task>>>,
    pub result_sender: mpsc::Sender<TaskResult>,
    pub active_workers: Arc<AtomicUsize>,
    pub task_execution_tracker: Arc<TaskExecutionTracker>,
    pub cancellation_token: CancellationToken,
}

impl SharedState {
    pub fn increment_active_workers(&self) {
        self.active_workers.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_active_workers(&self) {
        self.active_workers.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug, Serialize)]
pub struct ExecutionStats {
    pub total_tasks: usize,
    pub completed: usize,
    pub failed: usize,
    pub execution_time_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResponse {
    pub status: String,
    pub results: Vec<TaskResult>,
    pub stats: ExecutionStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extension_filter_serialization() {
        // Test Include mode
        let filter = ExtensionFilter::Include {
            extensions: vec!["slack".to_string(), "github".to_string()],
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["mode"], "include");
        assert_eq!(json["extensions"], json!(["slack", "github"]));

        // Test Exclude mode
        let filter = ExtensionFilter::Exclude {
            extensions: vec!["jira".to_string()],
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["mode"], "exclude");
        assert_eq!(json["extensions"], json!(["jira"]));

        // Test None mode
        let filter = ExtensionFilter::None;
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["mode"], "none");
    }

    #[test]
    fn test_extension_filter_deserialization() {
        // Test Include mode
        let json = json!({
            "mode": "include",
            "extensions": ["developer", "slack"]
        });
        let filter: ExtensionFilter = serde_json::from_value(json).unwrap();
        match filter {
            ExtensionFilter::Include { extensions } => {
                assert_eq!(extensions, vec!["developer", "slack"]);
            }
            _ => panic!("Expected Include variant"),
        }

        // Test None mode
        let json = json!({"mode": "none"});
        let filter: ExtensionFilter = serde_json::from_value(json).unwrap();
        assert!(matches!(filter, ExtensionFilter::None));
    }

    #[test]
    fn test_task_with_extension_filter() {
        let task = Task {
            id: "test-123".to_string(),
            task_type: "text_instruction".to_string(),
            payload: json!({"text_instruction": "test"}),
            extension_filter: Some(ExtensionFilter::Include {
                extensions: vec!["developer".to_string()],
            }),
        };

        let json = serde_json::to_value(&task).unwrap();
        assert_eq!(json["extension_filter"]["mode"], "include");
        assert_eq!(json["extension_filter"]["extensions"], json!(["developer"]));
    }

    #[test]
    fn test_task_without_extension_filter() {
        let task = Task {
            id: "test-456".to_string(),
            task_type: "text_instruction".to_string(),
            payload: json!({"text_instruction": "test"}),
            extension_filter: None,
        };

        let json = serde_json::to_value(&task).unwrap();
        assert!(!json.as_object().unwrap().contains_key("extension_filter"));
    }
}
