use std::{collections::HashMap, fs};

use anyhow::Result;
use mcp_core::tool::{Tool, ToolAnnotations};
use serde_json::{json, Map, Value};

use crate::agents::sub_recipe_execution_tool::lib::Task;
use crate::recipe::{Recipe, RecipeParameter, RecipeParameterRequirement, SubRecipe};

pub const SUB_RECIPE_TASK_TOOL_NAME_PREFIX: &str = "subrecipe__create_task";

#[derive(Debug, Clone)]
pub enum TaskType {
    SubRecipe(SubRecipe),
    TextInstruction(String),
}

impl TaskType {
    pub fn task_type_name(&self) -> &'static str {
        match self {
            TaskType::SubRecipe(_) => "sub_recipe",
            TaskType::TextInstruction(_) => "text_instruction",
        }
    }
}

pub fn create_task_tool(task_type: &TaskType) -> Tool {
    match task_type {
        TaskType::SubRecipe(sub_recipe) => create_sub_recipe_task_tool(sub_recipe),
        TaskType::TextInstruction(_) => create_text_instruction_task_tool(),
    }
}

pub fn create_text_instruction_task_tool() -> Tool {
    Tool::new(
        "text_instruction__create_task".to_string(),
        "Create a text instruction task that can be executed by the task executor".to_string(),
        json!({
            "type": "object",
            "properties": {
                "text_instruction": {
                    "type": "string",
                    "description": "The text instruction to execute"
                }
            },
            "required": ["text_instruction"]
        }),
        Some(ToolAnnotations {
            title: Some("Create text instruction task".to_string()),
            read_only_hint: false,
            destructive_hint: true,
            idempotent_hint: false,
            open_world_hint: true,
        }),
    )
}

pub fn create_sub_recipe_task_tool(sub_recipe: &SubRecipe) -> Tool {
    let input_schema = get_input_schema(sub_recipe).unwrap();
    Tool::new(
        format!("{}_{}", SUB_RECIPE_TASK_TOOL_NAME_PREFIX, sub_recipe.name),
        "Before running this sub recipe, you should first create a task with this tool and then pass the task to the task executor".to_string(),
        input_schema,
        Some(ToolAnnotations {
            title: Some(format!("create sub recipe task {}", sub_recipe.name)),
            read_only_hint: false,
            destructive_hint: true,
            idempotent_hint: false,
            open_world_hint: true,
        }),
    )
}

fn get_sub_recipe_parameter_definition(
    sub_recipe: &SubRecipe,
) -> Result<Option<Vec<RecipeParameter>>> {
    let content = fs::read_to_string(sub_recipe.path.clone())
        .map_err(|e| anyhow::anyhow!("Failed to read recipe file {}: {}", sub_recipe.path, e))?;
    let recipe = Recipe::from_content(&content)?;
    Ok(recipe.parameters)
}

fn get_input_schema(sub_recipe: &SubRecipe) -> Result<Value> {
    let mut sub_recipe_params_map = HashMap::<String, String>::new();
    if let Some(params_with_value) = &sub_recipe.values {
        for (param_name, param_value) in params_with_value {
            sub_recipe_params_map.insert(param_name.clone(), param_value.clone());
        }
    }
    let parameter_definition = get_sub_recipe_parameter_definition(sub_recipe)?;
    if let Some(parameters) = parameter_definition {
        let mut properties = Map::new();
        let mut required = Vec::new();
        for param in parameters {
            if sub_recipe_params_map.contains_key(&param.key) {
                continue;
            }
            properties.insert(
                param.key.clone(),
                json!({
                    "type": param.input_type.to_string(),
                    "description": param.description.clone(),
                }),
            );
            if !matches!(param.requirement, RecipeParameterRequirement::Optional) {
                required.push(param.key);
            }
        }
        Ok(json!({
            "type": "object",
            "properties": properties,
            "required": required
        }))
    } else {
        Ok(json!({
            "type": "object",
            "properties": {}
        }))
    }
}

fn prepare_command_params(
    sub_recipe: &SubRecipe,
    params_from_tool_call: Value,
) -> Result<HashMap<String, String>> {
    let mut sub_recipe_params = HashMap::<String, String>::new();
    if let Some(params_with_value) = &sub_recipe.values {
        for (param_name, param_value) in params_with_value {
            sub_recipe_params.insert(param_name.clone(), param_value.clone());
        }
    }
    if let Some(params_map) = params_from_tool_call.as_object() {
        for (key, value) in params_map {
            sub_recipe_params.insert(
                key.to_string(),
                value.as_str().unwrap_or(&value.to_string()).to_string(),
            );
        }
    }
    Ok(sub_recipe_params)
}

pub async fn create_sub_recipe_task(sub_recipe: &SubRecipe, params: Value) -> Result<String> {
    let command_params = prepare_command_params(sub_recipe, params)?;
    let payload = json!({
        "sub_recipe": {
            "name": sub_recipe.name.clone(),
            "command_parameters": command_params,
            "recipe_path": sub_recipe.path.clone(),
        }
    });
    create_task(TaskType::SubRecipe(sub_recipe.clone()), payload).await
}

pub async fn create_text_instruction_task(text_instruction: String) -> Result<String> {
    let payload = json!({
        "text_instruction": text_instruction
    });
    create_task(TaskType::TextInstruction(text_instruction), payload).await
}

pub async fn create_text_instruction_task_from_args(args: Value) -> Result<String> {
    let text_instruction = args
        .get("text_instruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing text_instruction parameter"))?
        .to_string();
    create_text_instruction_task(text_instruction).await
}

/// Generalized task creation function that can handle both sub-recipe and text instruction tasks
async fn create_task(task_type: TaskType, payload: Value) -> Result<String> {
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        task_type: task_type.task_type_name().to_string(),
        payload,
    };
    let task_json = serde_json::to_string(&task)
        .map_err(|e| anyhow::anyhow!("Failed to serialize Task: {}", e))?;
    Ok(task_json)
}
