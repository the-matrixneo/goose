use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeTaskData {
    pub task: String,
    pub task_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeTasks {
    pub tasks: Vec<RecipeTaskData>,
    // pub current_task_id: String,
}