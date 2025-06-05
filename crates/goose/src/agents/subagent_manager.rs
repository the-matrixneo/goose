use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use serde_json::Value;

use super::subagent::{SubAgent, SubAgentHandle, SubAgentConfig, SubAgentStatus};
use crate::recipe::Recipe;
use crate::providers::base::Provider;
use mcp_core::Content;

/// Manager for all subagents associated with an agent
pub struct SubAgentManager {
    subagents: Arc<RwLock<HashMap<String, SubAgentHandle>>>,
    provider: Arc<dyn Provider>,
}

impl SubAgentManager {
    pub fn new(provider: Arc<dyn Provider>) -> Self {
        Self {
            subagents: Arc::new(RwLock::new(HashMap::new())),
            provider,
        }
    }
    
    /// Spawn a new interactive subagent
    pub async fn spawn_interactive_subagent(&self, args: Value) -> Result<Vec<Content>> {
        let recipe_name = args
            .get("recipe_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let _max_turns = args
            .get("max_turns")
            .and_then(|v| v.as_u64())
            .unwrap_or(5)
            .min(10) as usize;
        let parameters = args
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if recipe_name.is_empty() || message.is_empty() {
            return Err(anyhow::anyhow!("Both recipe_name and message are required"));
        }

        // Load recipe
        let recipe = self.load_recipe(&recipe_name, parameters).await?;
        
        // Create subagent config
        let config = SubAgentConfig::new(recipe);
        
        // Create subagent
        let (subagent, handle) = SubAgent::new(config, Arc::clone(&self.provider)).await?;
        let subagent_id = handle.id.clone();
        
        // Process the conversation
        let conversation = subagent.process_message(message).await?;
        
        // Store the handle for potential future use
        {
            let mut subagents = self.subagents.write().await;
            subagents.insert(subagent_id.clone(), handle);
        }
        
        // Format response
        let response = self.format_subagent_conversation(&subagent_id, &recipe_name, &conversation);
        
        Ok(vec![Content::text(response)])
    }
    
    /// Format the subagent conversation for display
    fn format_subagent_conversation(
        &self,
        subagent_id: &str,
        recipe_name: &str,
        conversation: &[crate::message::Message],
    ) -> String {
        let mut response_parts = Vec::new();
        response_parts.push(format!(
            "=== Interactive Subagent Conversation ===\n\
            Recipe: {}\n\
            Subagent ID: {}\n\
            Turns: {}\n\
            ===", 
            recipe_name, 
            subagent_id, 
            conversation.len()
        ));
        
        for (i, msg) in conversation.iter().enumerate() {
            let role = match msg.role {
                mcp_core::role::Role::User => "User",
                mcp_core::role::Role::Assistant => "Subagent",
            };
            response_parts.push(format!("Turn {}: {}: {}", i + 1, role, msg.as_concat_text()));
        }
        
        response_parts.push("=== End of Subagent Conversation ===".to_string());
        response_parts.join("\n\n")
    }
    
    /// Load a recipe from file
    async fn load_recipe(&self, recipe_name: &str, _parameters: Vec<(String, String)>) -> Result<Recipe> {
        // Try to load from file first
        let recipe_path = std::path::Path::new(recipe_name);
        
        if recipe_path.exists() {
            let content = std::fs::read_to_string(recipe_path)
                .map_err(|e| anyhow::anyhow!("Failed to read recipe file '{}': {}", recipe_name, e))?;
            
            // Try YAML first, then JSON
            if let Ok(recipe) = serde_yaml::from_str::<Recipe>(&content) {
                return Ok(recipe);
            } else if let Ok(recipe) = serde_json::from_str::<Recipe>(&content) {
                return Ok(recipe);
            } else {
                return Err(anyhow::anyhow!("Failed to parse recipe file '{}' as YAML or JSON", recipe_name));
            }
        }

        // If not a file path, try to find it as a recipe name
        for ext in &["yaml", "yml", "json"] {
            let filename = format!("{}.{}", recipe_name, ext);
            let path = std::path::Path::new(&filename);
            
            if path.exists() {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| anyhow::anyhow!("Failed to read recipe file '{}': {}", filename, e))?;
                
                if *ext == "json" {
                    if let Ok(recipe) = serde_json::from_str::<Recipe>(&content) {
                        return Ok(recipe);
                    }
                } else {
                    if let Ok(recipe) = serde_yaml::from_str::<Recipe>(&content) {
                        return Ok(recipe);
                    }
                }
            }
        }

        Err(anyhow::anyhow!(
            "Recipe '{}' not found. Please provide a valid recipe file path or ensure the recipe file exists in the current directory.",
            recipe_name
        ))
    }
    
    /// Get status of all subagents
    pub async fn get_all_subagent_status(&self) -> HashMap<String, SubAgentStatus> {
        let mut status_map = HashMap::new();
        let subagent_ids: Vec<String> = {
            let subagents = self.subagents.read().await;
            subagents.keys().cloned().collect()
        };
        
        // For the simplified version, we'll just return Ready status
        // In a full implementation, you'd query each subagent
        for id in subagent_ids {
            status_map.insert(id, SubAgentStatus::Ready);
        }
        
        status_map
    }
    
    /// Get list of active subagent IDs
    pub async fn list_subagents(&self) -> Vec<String> {
        let subagents = self.subagents.read().await;
        subagents.keys().cloned().collect()
    }
    
    /// Terminate a specific subagent
    pub async fn terminate_subagent(&self, subagent_id: &str) -> Result<()> {
        let mut subagents = self.subagents.write().await;
        subagents.remove(subagent_id);
        Ok(())
    }
    
    /// Terminate all subagents
    pub async fn terminate_all_subagents(&self) -> Result<()> {
        let mut subagents = self.subagents.write().await;
        subagents.clear();
        Ok(())
    }
    
    /// Get conversation from a specific subagent
    pub async fn get_subagent_conversation(&self, _subagent_id: &str) -> Result<Vec<crate::message::Message>> {
        // For the simplified version, return empty conversation
        // In a full implementation, you'd query the subagent
        Ok(Vec::new())
    }
    
    /// Update provider for all future subagents
    pub async fn update_provider(&mut self, provider: Arc<dyn Provider>) {
        self.provider = provider;
    }
}