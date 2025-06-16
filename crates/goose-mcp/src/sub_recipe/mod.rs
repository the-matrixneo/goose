mod multi_task_plan;
mod multi_task_plan_description;

use anyhow::Result;
use mcp_core::{
    handler::{PromptError, ResourceError, ToolError},
    protocol::{JsonRpcMessage, ServerCapabilities},
    prompt::Prompt,
    resource::Resource,
    role::Role,
    tool::{Tool, ToolAnnotations},
    Content,
};
use mcp_server::{router::CapabilitiesBuilder, Router};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::{sync::mpsc};

use crate::sub_recipe::multi_task_plan::{format_task, validate_task_data, TasksState};
use crate::sub_recipe::multi_task_plan_description::MULTI_TASK_PLAN_DESCRIPTION;

pub struct SubRecipeRouter {
    tools: Vec<Tool>,
    state: Arc<Mutex<TasksState>>,
}

impl Default for SubRecipeRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl SubRecipeRouter {
    pub fn new() -> Self {
        let multi_task_plan_tool = Tool::new(
            "multi_task_plan".to_string(),
            MULTI_TASK_PLAN_DESCRIPTION.to_string(),
            json!({
                "type": "object",
                "properties": {
                    "task": {
                        "type": "string",
                        "description": "Your current thinking step"
                    },
                    "task_id": {
                        "type": "string",
                        "description": "A unique ID for the task"
                    },
                    "next_task_needed": {
                        "type": "boolean",
                        "description": "Whether another task step is needed"
                    },
                    "task_number": {
                        "type": "integer",
                        "description": "Current task number",
                        "minimum": 1
                    },
                    "total_tasks": {
                        "type": "integer",
                        "description": "Estimated total tasks needed",
                        "minimum": 1
                    },
                    "is_revision": {
                        "type": "boolean",
                        "description": "Whether this revises previous thinking"
                    },
                    "revises_task": {
                        "type": "integer",
                        "description": "Which task is being reconsidered",
                        "minimum": 1
                    },
                    "branch_from_task": {
                        "type": "integer",
                        "description": "Branching point task number",
                        "minimum": 1
                    },
                    "branch_id": {
                        "type": "string",
                        "description": "Branch identifier"
                    },
                    "needs_more_tasks": {
                        "type": "boolean",
                        "description": "If more tasks are needed"
                    },
                    "depends_on": {
                        "type": "array",
                        "description": "Task IDs this task depends on. If not provided, the task is independent.",
                        "items": { "type": "string" }
                    },
                    "execution_id": {
                        "type": "string",
                        "description": "A unique ID for the execution attempt for the task"
                    },
                    "execution_status": {
                        "type": "string",
                        "enum": ["pending", "running", "completed", "failed"],
                        "description": "Task execution status"
                    },
                },
                "required": ["task", "next_task_needed", "task_number", "total_tasks"]
            }),
            Some(ToolAnnotations {
                title: Some("Multi-Task Planning".to_string()),
                read_only_hint: false,
                destructive_hint: false,
                idempotent_hint: true,
                open_world_hint: false,
            }),
        );

        Self {
            tools: vec![multi_task_plan_tool],
            state: Arc::new(Mutex::new(TasksState {
                task_history: Vec::new(),
                branches: HashMap::new(),
            })),
        }
    }

    async fn multi_task_planning(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let mut task_data = validate_task_data(params)?;
        
        if task_data.task_number > task_data.total_tasks {
            task_data.total_tasks = task_data.task_number;
        }
        
        let formatted_task = format_task(&task_data);
        eprintln!("{}", formatted_task);
        
        let mut state = self.state.lock().unwrap();

        
        state.task_history.push(task_data.clone());
        println!("task_history: {:?}", state.task_history);
        
        // Handle branch storage
        if let (Some(_branch_from), Some(branch_id)) = (task_data.branch_from_task, task_data.branch_id.clone()) {
            state.branches.entry(branch_id.clone())
                .or_insert_with(Vec::new)
                .push(task_data.clone());
        }
        
        // Prepare response
        let response = json!({
            "task_number": task_data.task_number,
            "total_tasks": task_data.total_tasks,
            "next_task_needed": task_data.next_task_needed,
            "branches": state.branches.keys().collect::<Vec<_>>(),
            "task_history_length": state.task_history.len(),
            "depends_on": task_data.depends_on,
            "execution_id": task_data.execution_id,
            "execution_status": task_data.execution_status,
        });
        
        // Return the response
        Ok(vec![Content::text(serde_json::to_string_pretty(&response).unwrap())
            .with_audience(vec![Role::Assistant])])
    }
}

impl Router for SubRecipeRouter {
    fn name(&self) -> String {
        "sub-recipe-router".to_string()
    }

    fn instructions(&self) -> String {
        "Sub recipe MCP for step-by-step problem solving".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new()
            .with_tools(true)
            .with_prompts(false)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.tools.clone()
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
        _notifier: mpsc::Sender<JsonRpcMessage>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let this = self.clone();
        let tool_name = tool_name.to_string();
        
        Box::pin(async move {
            match tool_name.as_str() {
                "multi_task_plan" => this.multi_task_planning(arguments).await,
                _ => Err(ToolError::NotFound(format!("Tool {} not found", tool_name))),
            }
        })
    }
    
    // Implement the required resource-related methods
    fn list_resources(&self) -> Vec<Resource> {
        Vec::new() // No resources for this MCP
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async move {
            Err(ResourceError::NotFound("No resources available".to_string()))
        })
    }

    // Implement the required prompt-related methods
    fn list_prompts(&self) -> Vec<Prompt> {
        Vec::new() // No prompts for this MCP
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        let prompt_name = prompt_name.to_string(); // Clone the string to own it
        Box::pin(async move {
            Err(PromptError::NotFound(format!("Prompt '{}' not found", prompt_name)))
        })
    }
}

impl Clone for SubRecipeRouter {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
            state: Arc::clone(&self.state),
        }
    }
}