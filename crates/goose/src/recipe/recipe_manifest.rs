use anyhow::Result;
use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::recipe::Recipe;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RecipeManifest {
    pub name: String,
    pub recipes: Recipe,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
}

impl RecipeManifest {
    pub fn from_yaml_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path.display(), e))?;
        let manifest = serde_yaml::from_str::<Self>(&content).map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;
        Ok(manifest)
    }
}
