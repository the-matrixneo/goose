#[allow(dead_code)]
mod break_down_recipe;
#[allow(dead_code)]
mod break_down_recipe_description;
#[allow(dead_code)]
mod multi_task_plan;
#[allow(dead_code)]
mod multi_task_plan_description;
mod shell_command_job;

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
use std::{future::Future, pin::Pin};
use tokio::sync::mpsc;

use crate::sub_recipe::shell_command_job::{
    run_sub_recipe_command, SubRecipeAttributes, SUB_RECIPE_RUN_DESCRIPTION, SUB_RECIPE_RUN_SCHEMA,
};

pub struct SubRecipeRouter {
    tools: Vec<Tool>,
}

impl Clone for SubRecipeRouter {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
        }
    }
}

impl Default for SubRecipeRouter {
    fn default() -> Self {
        Self::new()
    }
}
impl SubRecipeRouter {
    pub fn new() -> Self {
        let sub_recipe_run = Tool::new(
            "sub_recipe_run".to_string(),
            SUB_RECIPE_RUN_DESCRIPTION.to_string(),
            serde_json::from_str(SUB_RECIPE_RUN_SCHEMA).unwrap(),
            Some(ToolAnnotations {
                title: Some("Run sub recipe".to_string()),
                read_only_hint: false,
                destructive_hint: true,
                idempotent_hint: false,
                open_world_hint: false,
            }),
        );

        Self {
            tools: vec![sub_recipe_run],
        }
    }

    async fn run_sub_recipe(params: Value) -> Result<Vec<Content>, ToolError> {
        let sub_recipe_attributes: SubRecipeAttributes =
            serde_json::from_value(params).map_err(|e| {
                ToolError::InvalidParameters(format!("Invalid sub-recipe attributes: {}", e))
            })?;

        let output = run_sub_recipe_command(&sub_recipe_attributes).map_err(|e| {
            ToolError::ExecutionError(format!("Sub-recipe execution failed: {}", e))
        })?;

        let response = json!({
            "recipe_name": sub_recipe_attributes.recipe_name,
            "recipe_path": sub_recipe_attributes.recipe_path,
            "parameters": sub_recipe_attributes.parameters,
            "output": output,
        });

        Ok(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )
        .with_audience(vec![Role::Assistant])])
    }
}

impl Router for SubRecipeRouter {
    fn name(&self) -> String {
        "sub-recipe-router".to_string()
    }

    fn instructions(&self) -> String {
        "Sub recipe MCP to run sub-recipe".to_string()
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
        let tool_name = tool_name.to_string();
        let arguments = arguments.clone();

        Box::pin(async move {
            match tool_name.as_str() {
                "sub_recipe_run" => SubRecipeRouter::run_sub_recipe(arguments).await,
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
