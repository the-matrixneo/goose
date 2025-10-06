use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use fs2::FileExt;

use anyhow::Result;

use goose::recipe::local_recipes::list_local_recipes;
use goose::recipe::Recipe;

use std::path::Path;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct RecipeManifestWithPath {
    pub id: String,
    pub name: String,
    pub recipe: Recipe,
    pub file_path: PathBuf,
    pub last_modified: String,
}

fn short_id_from_path(path: &str) -> String {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let h = hasher.finish();
    format!("{:016x}", h)
}

pub fn get_all_recipes_manifests() -> Result<Vec<RecipeManifestWithPath>> {
    let recipes_with_path = list_local_recipes()?;
    let mut recipe_manifests_with_path = Vec::new();
    for (file_path, recipe) in recipes_with_path {
        let Ok(last_modified) = fs::metadata(file_path.clone())
            .map(|m| chrono::DateTime::<chrono::Utc>::from(m.modified().unwrap()).to_rfc3339())
        else {
            continue;
        };
        let recipe_metadata =
            RecipeManifestMetadata::from_yaml_file(&file_path).unwrap_or_else(|_| {
                RecipeManifestMetadata {
                    name: recipe.title.clone(),
                }
            });

        let manifest_with_path = RecipeManifestWithPath {
            id: short_id_from_path(file_path.to_string_lossy().as_ref()),
            name: recipe_metadata.name,
            recipe,
            file_path,
            last_modified,
        };
        recipe_manifests_with_path.push(manifest_with_path);
    }
    recipe_manifests_with_path.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

    Ok(recipe_manifests_with_path)
}

fn get_recipe_temp_file_path() -> std::path::PathBuf {
    std::env::temp_dir().join("goose_recipe_file_map.json")
}

pub fn save_recipe_file_hash_map(hash_map: &HashMap<String, std::path::PathBuf>) -> Result<()> {
    let temp_path = get_recipe_temp_file_path();
    let json_data = serde_json::to_string(hash_map)
        .map_err(|e| anyhow::anyhow!("Failed to serialize hash map: {}", e))?;

    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY_MS: u64 = 50;

    for attempt in 0..MAX_RETRIES {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)
            .map_err(|e| anyhow::anyhow!("Failed to open recipe file for writing: {}", e))?;

        match file.try_lock_exclusive() {
            Ok(_) => {
                fs::write(&temp_path, json_data)
                    .map_err(|e| anyhow::anyhow!("Failed to write recipe id to file: {}", e))?;

                return Ok(());
            }
            Err(_) if attempt < MAX_RETRIES - 1 => {
                std::thread::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS));
                continue;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to acquire lock on recipe file after {} attempts: {}",
                    MAX_RETRIES,
                    e
                ));
            }
        }
    }

    Err(anyhow::anyhow!("Failed to save recipe file hash map"))
}

fn load_recipe_file_hash_map() -> Result<HashMap<String, std::path::PathBuf>> {
    let temp_path = get_recipe_temp_file_path();
    if !temp_path.exists() {
        return Ok(HashMap::new());
    }
    let json_data = fs::read_to_string(temp_path)
        .map_err(|e| anyhow::anyhow!("Failed to read recipe id to file: {}", e))?;
    let hash_map = serde_json::from_str(&json_data)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize hash map: {}", e))?;
    Ok(hash_map)
}

pub fn find_recipe_file_path_by_id(recipe_id: &str) -> Result<PathBuf> {
    let recipe_file_hash_map = load_recipe_file_hash_map()?;
    recipe_file_hash_map
        .get(recipe_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Recipe not found with id: {}", recipe_id))
}

// this is a temporary struct to deserilize the UI recipe files. should not be used for other purposes.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct RecipeManifestMetadata {
    pub name: String,
}

impl RecipeManifestMetadata {
    pub fn from_yaml_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path.display(), e))?;
        let metadata = serde_yaml::from_str::<Self>(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_from_yaml_file_success() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_recipe.yaml");

        let yaml_content = r#"
name: "Test Recipe"
isGlobal: true
recipe: recipe_content
"#;

        fs::write(&file_path, yaml_content).unwrap();

        let result = RecipeManifestMetadata::from_yaml_file(&file_path).unwrap();

        assert_eq!(result.name, "Test Recipe");
    }
}
