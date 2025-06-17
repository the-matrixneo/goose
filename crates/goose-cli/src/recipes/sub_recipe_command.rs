use goose::{config::ExtensionConfig, recipe::{Recipe, SubRecipe}};
use serde_json::json;

pub fn create_sub_recipe_extensions(recipe: &Recipe) -> Vec<ExtensionConfig> {
    let mut extensions: Vec<ExtensionConfig> = Vec::new();
    if let Some(sub_recipes) = &recipe.sub_recipes {
        for sub_recipe in sub_recipes {
            // Convert the SubRecipe to SubRecipeInitialData format
            let params = match &sub_recipe.params {
                Some(params) => params
                    .iter()
                    .map(|p| json!({
                        "name": p.name,
                        "value": p.value
                    }))
                    .collect::<Vec<_>>(),
                None => Vec::new(),
            };

            // Create initial data JSON
            let initial_data = json!({
                "name": sub_recipe.name,
                "path": sub_recipe.path,
                "params": params,
                "metadata": {
                    "source": "recipe",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });

            // Create environment variables map with the initial data
            let mut envs = std::collections::HashMap::new();
            envs.insert(
                "SUB_RECIPE_INITIAL_DATA".to_string(),
                initial_data.to_string(),
            );

            let extension = ExtensionConfig::Builtin {
                name: "sub-recipe".to_string(),
                timeout: Some(300),
                bundled: Some(true),
                display_name: Some(format!("sub recipe {}", sub_recipe.name)),
                args: Some(vec![sub_recipe.name.clone()]),
            };
            extensions.push(extension);
        }
    }
    extensions
}

pub fn create_sub_recipe_instructions(recipe: &Recipe) -> String {
    let mut instructions = String::new();
    if let Some(sub_recipes) = &recipe.sub_recipes {
        for sub_recipe in sub_recipes {
            instructions.push_str(&format!(
                "if {} is required to run, then use sub_recipe_run tool to run the recipe: \n
                 recipe_path: {} \n
                 params: {} \n
                 if the sub-recipe required parameters are not provided, use the context to fill in the parameters.
                 You can also use get_initial_data tool to retrieve the initial data passed to the sub-recipe server.
                ", sub_recipe.name, sub_recipe.path, serde_json::to_string(&sub_recipe.params).unwrap_or_default()));
        }
    }
    instructions
}

fn create_sub_recipe_command(sub_recipe: &SubRecipe) -> String {
    let mut command = String::new();
    command.push_str(&format!("goose run --recipe {} ", sub_recipe.path));
    if let Some(params) = &sub_recipe.params {
        for param in params {
            command.push_str(&format!(" --params {}={}", param.name, param.value));
        }
    }
    return command;
}