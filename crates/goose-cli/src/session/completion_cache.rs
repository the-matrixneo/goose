use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::session::output;

/// Cache structure for completion data
/// Uses std::sync::RwLock for thread safety without async
pub struct CompletionCache {
    prompts: HashMap<String, Vec<String>>,
    prompt_info: HashMap<String, output::PromptInfo>,
    last_updated: Instant,
}

impl CompletionCache {
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            prompt_info: HashMap::new(),
            last_updated: Instant::now(),
        }
    }

    /// Clear all cached data
    pub fn clear(&mut self) {
        self.prompts.clear();
        self.prompt_info.clear();
        self.last_updated = Instant::now();
    }

    /// Update the cache with fresh prompt data
    pub fn update_prompts(&mut self, extension: String, prompt_names: Vec<String>) {
        self.prompts.insert(extension, prompt_names);
        self.last_updated = Instant::now();
    }

    /// Update the cache with prompt info
    pub fn update_prompt_info(&mut self, name: String, info: output::PromptInfo) {
        self.prompt_info.insert(name, info);
    }

    /// Get cached prompts for an extension
    pub fn get_prompts(&self, extension: &str) -> Option<&Vec<String>> {
        self.prompts.get(extension)
    }

    /// Get all cached prompts
    pub fn get_all_prompts(&self) -> &HashMap<String, Vec<String>> {
        &self.prompts
    }

    /// Get cached prompt info
    pub fn get_prompt_info(&self, name: &str) -> Option<&output::PromptInfo> {
        self.prompt_info.get(name)
    }

    /// Get the last update time
    pub fn last_updated(&self) -> Instant {
        self.last_updated
    }
}

/// Thread-safe completion cache manager
pub struct CompletionCacheManager {
    cache: Arc<std::sync::RwLock<CompletionCache>>,
}

impl CompletionCacheManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(std::sync::RwLock::new(CompletionCache::new())),
        }
    }

    /// Get a clone of the cache Arc for sharing with other components
    pub fn get_cache_ref(&self) -> Arc<std::sync::RwLock<CompletionCache>> {
        self.cache.clone()
    }

    /// Update the completion cache with fresh data from the agent
    pub async fn update_cache(&self, agent: &goose::agents::Agent) -> Result<()> {
        // Get fresh data
        let prompts = agent.list_extension_prompts().await;

        // Update the cache with write lock
        let mut cache = self.cache.write().unwrap();
        cache.clear();

        for (extension, prompt_list) in prompts {
            let names: Vec<String> = prompt_list.iter().map(|p| p.name.clone()).collect();
            cache.update_prompts(extension.clone(), names);

            for prompt in prompt_list {
                cache.update_prompt_info(
                    prompt.name.clone(),
                    output::PromptInfo {
                        name: prompt.name.clone(),
                        description: prompt.description.clone(),
                        arguments: prompt.arguments.clone(),
                        extension: Some(extension.clone()),
                    },
                );
            }
        }

        Ok(())
    }

    /// Invalidate the completion cache
    /// This should be called when extensions are added or removed
    pub fn invalidate_cache(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }

    /// Get a read lock on the cache for safe access
    pub fn with_cache<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&CompletionCache) -> R,
    {
        let cache = self.cache.read().unwrap();
        f(&*cache)
    }
}

impl Default for CompletionCacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_cache_new() {
        let cache = CompletionCache::new();
        assert!(cache.get_all_prompts().is_empty());
        assert!(cache.get_prompt_info("test").is_none());
    }

    #[test]
    fn test_completion_cache_update() {
        let mut cache = CompletionCache::new();
        let extension = "test_ext".to_string();
        let prompts = vec!["prompt1".to_string(), "prompt2".to_string()];

        cache.update_prompts(extension.clone(), prompts.clone());

        assert_eq!(cache.get_prompts(&extension), Some(&prompts));
    }

    #[test]
    fn test_completion_cache_clear() {
        let mut cache = CompletionCache::new();
        cache.update_prompts("test".to_string(), vec!["prompt".to_string()]);

        assert!(!cache.get_all_prompts().is_empty());

        cache.clear();

        assert!(cache.get_all_prompts().is_empty());
    }

    #[test]
    fn test_completion_cache_manager() {
        let manager = CompletionCacheManager::new();

        manager.with_cache(|cache| {
            assert!(cache.get_all_prompts().is_empty());
        });

        manager.invalidate_cache();

        manager.with_cache(|cache| {
            assert!(cache.get_all_prompts().is_empty());
        });
    }
}
