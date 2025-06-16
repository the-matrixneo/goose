mod multi_task_plan;
mod multi_task_plan_description;
mod shell_command_job;
mod break_down_recipe_description;
mod break_down_recipe;

use anyhow::Result;
use mcp_core::{
    handler::{PromptError, ResourceError, ToolError},
    prompt::Prompt,
    protocol::{JsonRpcMessage, ServerCapabilities},
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
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::sub_recipe::{break_down_recipe::{RecipeTaskData, RecipeTasks}, break_down_recipe_description::{BREAK_DOWN_TASK_DESCRIPTION, BREAK_DOWN_TASK_SCHEMA}, shell_command_job::{
    start_dispatcher, submit_job, validate_shell_job_params, JobSender,
    SHELL_COMMAND_JOB_DESCRIPTION, SHELL_COMMAND_JOB_SCHEMA,
}};

use crate::sub_recipe::multi_task_plan::{format_task, validate_task_data, TasksState};
use crate::sub_recipe::multi_task_plan_description::{
    MULTI_TASK_PLAN_DESCRIPTION, MULTI_TASK_PLAN_SCHEMA,
};

pub struct SubRecipeRouter {
    tools: Vec<Tool>,
    state: Arc<Mutex<TasksState>>,
    job_map: Arc<tokio::sync::Mutex<HashMap<Uuid, shell_command_job::JobStatus>>>,
    job_sender: Option<JobSender>,
}

impl Default for SubRecipeRouter {
    fn default() -> Self {
        Self::new()
    }
}
// sub_recipe_run

impl SubRecipeRouter {
    pub fn new() -> Self {
        // let multi_task_plan_tool = Tool::new(
        //     "multi_task_plan".to_string(),
        //     MULTI_TASK_PLAN_DESCRIPTION.to_string(),
        //     serde_json::from_str(MULTI_TASK_PLAN_SCHEMA).unwrap(),
        //     Some(ToolAnnotations {
        //         title: Some("Multi-Task Planning".to_string()),
        //         read_only_hint: false,
        //         destructive_hint: false,
        //         idempotent_hint: true,
        //         open_world_hint: false,
        //     }),
        // );
        let break_down_task_tool = Tool::new(
            "break_down_task".to_string(),
            BREAK_DOWN_TASK_DESCRIPTION.to_string(),
            serde_json::from_str(BREAK_DOWN_TASK_SCHEMA).unwrap(),
            Some(ToolAnnotations {
                title: Some("break down recipe".to_string()),
                read_only_hint: false,
                destructive_hint: false,
                idempotent_hint: true,
                open_world_hint: false,
            }),
        );

        let shell_command_job_tool = Tool::new(
            "sub_recipe_run".to_string(),
            SHELL_COMMAND_JOB_DESCRIPTION.to_string(),
            serde_json::from_str(SHELL_COMMAND_JOB_SCHEMA).unwrap(),
            Some(ToolAnnotations {
                title: Some("Shell Command Job System".to_string()),
                read_only_hint: false,
                destructive_hint: true,
                idempotent_hint: false,
                open_world_hint: false,
            }),
        );

        let job_map = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
        let (job_sender, job_receiver) = mpsc::channel(100);

        // Start the job dispatcher
        let job_map_clone = job_map.clone();
        tokio::spawn(async move {
            start_dispatcher(job_receiver, job_map_clone, 5).await;
        });

        Self {
            tools: vec![shell_command_job_tool, break_down_task_tool],
            state: Arc::new(Mutex::new(TasksState {
                task_history: Vec::new(),
                branches: HashMap::new(),
            })),
            job_map,
            job_sender: Some(job_sender),
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

        if let (Some(_branch_from), Some(branch_id)) =
            (task_data.branch_from_task, task_data.branch_id.clone())
        {
            state
                .branches
                .entry(branch_id.clone())
                .or_default()
                .push(task_data.clone());
        }

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
        Ok(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )
        .with_audience(vec![Role::Assistant])])
    }

    async fn break_down_task(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let recipe_tasks: RecipeTasks = serde_json::from_value(params)
        .map_err(|e| {
            println!("error: {:?}", e);
            ToolError::InvalidParameters(format!("========Invalid task data: {}", e))
        })?;

        // println!("task_data: {:?}", &task_data);

        let response = json!({
            "tasks": recipe_tasks.tasks
        });

        // Return the response
        Ok(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )
        .with_audience(vec![Role::Assistant])])
    }

    async fn shell_command_job(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        let (action, command, job_id_str) =
            validate_shell_job_params(params).map_err(ToolError::InvalidParameters)?;

        match action.as_str() {
            "submit" => {
                let command = command.unwrap();
                let sender = self.job_sender.as_ref().ok_or_else(|| {
                    ToolError::ExecutionError("Job sender not initialized".to_string())
                })?;

                let job_id = submit_job(command.clone(), sender, &self.job_map).await;

                let response = json!({
                    "job_id": job_id.to_string(),
                    "command": command,
                    "status": "queued"
                });

                Ok(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )
                .with_audience(vec![Role::Assistant])])
            }
            "status" => {
                let job_id_str = job_id_str.unwrap(); // Safe because validate_shell_job_params ensures it's present
                let job_id = Uuid::parse_str(&job_id_str).map_err(|_| {
                    ToolError::InvalidParameters(format!("Invalid job ID: {}", job_id_str))
                })?;

                let status = {
                    let map = self.job_map.lock().await;
                    map.get(&job_id).cloned()
                };

                if let Some(status) = status {
                    let response = json!({
                        "job_id": job_id.to_string(),
                        "status": status
                    });

                    Ok(vec![Content::text(
                        serde_json::to_string_pretty(&response).unwrap(),
                    )
                    .with_audience(vec![Role::Assistant])])
                } else {
                    Err(ToolError::NotFound(format!(
                        "Job with ID {} not found",
                        job_id
                    )))
                }
            }
            "list" => {
                let jobs = {
                    let map = self.job_map.lock().await;
                    let mut jobs = HashMap::new();
                    for (id, status) in map.iter() {
                        jobs.insert(id.to_string(), status.clone());
                    }
                    jobs
                };

                let response = json!({
                    "jobs": jobs
                });

                Ok(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )
                .with_audience(vec![Role::Assistant])])
            }
            _ => Err(ToolError::InvalidParameters(format!(
                "Unknown action: {}",
                action
            ))),
        }
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
                // "multi_task_plan" => this.multi_task_planning(arguments).await,
                "sub_recipe_run" => this.shell_command_job(arguments).await,
                "break_down_task" => this.break_down_task(arguments).await,
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
            Err(ResourceError::NotFound(
                "No resources available".to_string(),
            ))
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
            Err(PromptError::NotFound(format!(
                "Prompt '{}' not found",
                prompt_name
            )))
        })
    }
}

impl Clone for SubRecipeRouter {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
            state: Arc::clone(&self.state),
            job_map: Arc::clone(&self.job_map),
            job_sender: self.job_sender.clone(),
        }
    }
}
