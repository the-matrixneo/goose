use std::path::PathBuf;

use anyhow::Result;
use etcetera::{choose_app_strategy, AppStrategy};

use crate::config::APP_STRATEGY;

pub fn list_recipes() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let local_recipe_path = current_dir.join(".goose/recipes");
    let global_recipe_path = choose_app_strategy(APP_STRATEGY.clone())
            .map(|strategy| strategy.in_config_dir("recipes"))
            .unwrap_or_else(|_| PathBuf::from("~/.config/goose/recipes"));
    
    Ok(())
}