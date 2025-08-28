use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use etcetera::{choose_app_strategy, AppStrategy};

use crate::config::APP_STRATEGY;
use crate::recipe::recipe_manifest::RecipeManifest;

fn load_recipes_from_path(path: &PathBuf) -> Result<Vec<RecipeManifest>> {
    let mut recipe_manifests = Vec::new();
    if path.exists() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.extension() == Some("yaml".as_ref()) {
                if let Ok(recipe_manifest) = RecipeManifest::from_yaml_file(&path) {
                    recipe_manifests.push(recipe_manifest);
                }
            }
        }
    }
    Ok(recipe_manifests)
}

fn get_all_recipes_manifests() -> Result<Vec<RecipeManifest>> {
    let current_dir = std::env::current_dir()?;
    let local_recipe_path = current_dir.join(".goose/recipes");
    
    let global_recipe_path = choose_app_strategy(APP_STRATEGY.clone())
            .map(|strategy| strategy.in_config_dir("recipes"))
            .unwrap_or_else(|_| PathBuf::from("~/.config/goose/recipes"));
    
    let mut recipe_manifests = Vec::new();
    
    recipe_manifests.extend(load_recipes_from_path(&local_recipe_path)?);
    recipe_manifests.extend(load_recipes_from_path(&global_recipe_path)?);
    
    Ok(recipe_manifests)
}

pub fn list_sorted_recipe_manifests(include_archived: bool) -> Result<Vec<RecipeManifest>> {
    let mut recipe_manifests = get_all_recipes_manifests()?;
    if !include_archived {
        recipe_manifests.retain(|manifest| !manifest.is_archived);
    }
    recipe_manifests.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(recipe_manifests)
}