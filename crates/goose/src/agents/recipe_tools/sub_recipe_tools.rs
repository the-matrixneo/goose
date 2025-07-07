use std::collections::HashSet;
use std::{collections::HashMap, fs};

use anyhow::Result;
use mcp_core::tool::{Tool, ToolAnnotations};
use serde_json::{json, Map, Value};

use crate::agents::sub_recipe_execution_tool::lib::Task;
use crate::recipe::{Recipe, RecipeParameter, RecipeParameterRequirement, SubRecipe};

pub const SUB_RECIPE_TASK_TOOL_NAME_PREFIX: &str = "subrecipe__create_task";

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn create_multiple_sub_recipe_task_tool(sub_recipe: &SubRecipe) -> Tool {
    let input_schema = get_input_schema_for_multiple_sub_recipe_task(sub_recipe).unwrap();
    Tool::new(
        format!("{}_{}", SUB_RECIPE_TASK_TOOL_NAME_PREFIX, sub_recipe.name),
        format!(
            "Create one or more tasks to run the '{}' sub recipe. \
            Provide an array of parameter sets in the 'task_parameters' field:\n\
            - For a single task: provide an array with one parameter set\n\
            - For multiple tasks: provide an array with multiple parameter sets, each with different values\n\n\
            Each task will run the same sub recipe but with different parameter values. \
            This is useful when you need to execute the same sub recipe multiple times with varying inputs. \
            After creating the task list, pass it to the task executor to run all tasks.",
            sub_recipe.name
        ),
        input_schema,
        Some(ToolAnnotations {
            title: Some(format!("create multiple sub recipe tasks for {}", sub_recipe.name)),
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_input_schema_for_multiple_sub_recipe_task(sub_recipe: &SubRecipe) -> Result<Value> {
    let mut sub_recipe_values = HashSet::<String>::new();
    if let Some(params_with_value) = &sub_recipe.values {
        for param_name in params_with_value.keys() {
            sub_recipe_values.insert(param_name.clone());
        }
    }

    if let Some(runs) = sub_recipe.executions.as_ref().and_then(|e| e.runs.as_ref()) {
        for run in runs {
            if let Some(params_with_value) = &run.values {
                for param_name in params_with_value.keys() {
                    sub_recipe_values.insert(param_name.clone());
                }
            }
        }
    }

    let parameter_definition = get_sub_recipe_parameter_definition(sub_recipe)?;

    let mut param_properties = Map::new();
    let mut param_required = Vec::new();

    if let Some(parameters) = parameter_definition {
        for param in parameters {
            if sub_recipe_values.contains(&param.key.clone()) {
                continue;
            }
            param_properties.insert(
                param.key.clone(),
                json!({
                    "type": param.input_type.to_string(),
                    "description": param.description.clone(),
                }),
            );
            if !matches!(param.requirement, RecipeParameterRequirement::Optional) {
                param_required.push(param.key);
            }
        }
    }

    // Create the schema using only task_parameters for both single and multiple tasks
    let mut properties = Map::new();
    if !param_properties.is_empty() {
        properties.insert(
            "task_parameters".to_string(),
            json!({
                "type": "array",
                "description": "Array of parameter sets for creating tasks. \
                    For a single task, provide an array with one element. \
                    For multiple tasks, provide an array with multiple elements, each with different parameter values. \
                    If there is no parameter set, provide an empty array.",
                "items": {
                    "type": "object",
                    "properties": param_properties,
                    "required": param_required
                },
            })
        );
    }
    Ok(json!({
        "type": "object",
        "properties": properties,
    }))
}

#[allow(dead_code)]
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

fn prepare_command_params_for_multiple_sub_recipe_task(
    sub_recipe: &SubRecipe,
    params_from_tool_call: Vec<Value>,
) -> Result<Vec<HashMap<String, String>>> {
    let mut sub_recipe_params = HashMap::<String, String>::new();
    if let Some(params_with_value) = &sub_recipe.values {
        for (param_name, param_value) in params_with_value {
            sub_recipe_params.insert(param_name.clone(), param_value.clone());
        }
    }
    let mut sub_recipe_run_params = Vec::<HashMap<String, String>>::new();
    if let Some(runs) = sub_recipe.executions.as_ref().and_then(|e| e.runs.as_ref()) {
        for run in runs {
            let mut sub_recipe_run_param = sub_recipe_params.clone();
            if let Some(params_with_value) = &run.values {
                sub_recipe_run_param.extend(params_with_value.clone());
            }
            sub_recipe_run_params.push(sub_recipe_run_param);
        }
    }
    println!("===== sub_recipe_run_params: {:?}", sub_recipe_run_params);
    println!("===== params_from_tool_call: {:?}", params_from_tool_call);
    if params_from_tool_call.is_empty() {
        return Ok(sub_recipe_run_params);
    }
    if sub_recipe_run_params.len() > 0 && sub_recipe_run_params.len() != params_from_tool_call.len()
    {
        return Err(anyhow::anyhow!(
            "The number of runs in the sub recipe does not match the number of task parameters"
        ));
    }
    let mut sub_recipe_run_params = vec![sub_recipe_params.clone(); params_from_tool_call.len()];
    for (index, sub_recipe_task_param) in params_from_tool_call.iter().enumerate() {
        if let Some(params_with_value) = sub_recipe_task_param.as_object() {
            for (key, value) in params_with_value {
                sub_recipe_run_params[index]
                    .entry(key.to_string())
                    .or_insert_with(|| value.as_str().unwrap_or(&value.to_string()).to_string());
            }
        }
    }
    Ok(sub_recipe_run_params)
}

#[allow(dead_code)]
pub async fn create_sub_recipe_task(sub_recipe: &SubRecipe, params: Value) -> Result<String> {
    let command_params = prepare_command_params(sub_recipe, params)?;
    let payload = json!({
        "sub_recipe": {
            "name": sub_recipe.name.clone(),
            "command_parameters": command_params,
            "recipe_path": sub_recipe.path.clone(),
        }
    });
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        task_type: "sub_recipe".to_string(),
        payload,
    };
    let task_json = serde_json::to_string(&task)
        .map_err(|e| anyhow::anyhow!("Failed to serialize Task: {}", e))?;
    Ok(task_json)
}

pub async fn create_multiple_sub_recipe_tasks(
    sub_recipe: &SubRecipe,
    params: Value,
) -> Result<String> {
    // Get the task_parameters array
    let empty_vec = vec![];
    let task_params_array = params
        .get("task_parameters")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_vec);
    let command_params =
        prepare_command_params_for_multiple_sub_recipe_task(sub_recipe, task_params_array.clone())?;
    let tasks = command_params
        .iter()
        .map(|task_command_param| {
            let payload = json!({
                "sub_recipe": {
                    "name": sub_recipe.name.clone(),
                    "command_parameters": task_command_param,
                    "recipe_path": sub_recipe.path.clone(),
                }
            });
            Task {
                id: uuid::Uuid::new_v4().to_string(),
                task_type: "sub_recipe".to_string(),
                payload,
            }
        })
        .collect::<Vec<Task>>();
    let is_parallel = sub_recipe
        .executions
        .as_ref()
        .map(|e| e.parallel)
        .unwrap_or(false);
    let task_execution_payload = json!({
        "tasks": tasks,
        "execution_mode": if is_parallel { "parallel" } else { "sequential" }
    });
    println!("===== task_execution_payload: {:?}", task_execution_payload);

    let tasks_json = serde_json::to_string(&task_execution_payload)
        .map_err(|e| anyhow::anyhow!("Failed to serialize task list: {}", e))?;
    Ok(tasks_json)
}

#[cfg(test)]
mod tests;
