use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRecipeParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRecipeAttributes {
    pub recipe_path: String,
    pub parameters: Vec<SubRecipeParameter>,
    pub recipe_name: String,
}

pub fn run_sub_recipe_command(
    sub_recipe_attributes: &SubRecipeAttributes,
) -> Result<String, String> {
    println!("Running sub-recipe");
    println!("========== Params: {:?}", sub_recipe_attributes);
    let mut command = Command::new("goose");
    command
        .arg("run")
        .arg("--recipe")
        .arg(&sub_recipe_attributes.recipe_path);

    // Add each parameter individually
    for param in &sub_recipe_attributes.parameters {
        command.arg(format!("--params={}={}", param.name, param.value));
    }

    let output = command
        .output()
        .map_err(|e| format!("Failed to execute: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub const SUB_RECIPE_RUN_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "recipe_path": {
            "type": "string",
            "description": "Path to the sub-recipe file (required)"
        },
        "recipe_name": {
            "type": "string",
            "description": "Name of the sub-recipe to run (required)"
        },
        "parameters": {
            "type": "array",
            "description": "Parameters to fill in the sub-recipe",
            "items": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "value": { "type": "string" }
                }
            }
        }
    },
    "required": ["recipe_path", "recipe_name"]
}"#;

pub const SUB_RECIPE_RUN_DESCRIPTION: &str = r#"
A tool for running a sub-recipe.
When you are given a sub-recipe, you should first read the sub-recipe file and understand the parameters that are required to run the sub-recipe.
Using params section of the sub-recipe in the main recipe as parameters to run the sub-recipe. If the required parameters of the sub-recipe are not provided, use the context to fill in the parameters.

Example usage:
Run a sub-recipe: {"recipe_name": "joke-of-the-day", "recipe_path": "path/to/sub-recipe.yaml", "parameters": [{"name": "date", "value": "2025-06-17"}]}
"#;
