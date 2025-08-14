use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;

/// A generic key-value store for internal tools to persist state
/// This allows tools to pass data upstream to the agent loop and session
#[derive(Debug, Clone)]
pub struct InternalToolState {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl Default for InternalToolState {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalToolState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set a value in the store
    pub async fn set(&self, key: String, value: Value) {
        let mut store = self.store.write().await;
        store.insert(key, value);
    }

    /// Get a value from the store
    pub async fn get(&self, key: &str) -> Option<Value> {
        let store = self.store.read().await;
        store.get(key).cloned()
    }

    /// Remove a value from the store
    #[allow(dead_code)]
    pub async fn remove(&self, key: &str) -> Option<Value> {
        let mut store = self.store.write().await;
        store.remove(key)
    }

    /// Clear all values from the store
    #[allow(dead_code)]
    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        store.clear();
    }

    /// Get all key-value pairs
    #[allow(dead_code)]
    pub async fn get_all(&self) -> HashMap<String, Value> {
        let store = self.store.read().await;
        store.clone()
    }

    /// Load state from a HashMap (e.g., from session metadata)
    #[allow(dead_code)]
    pub async fn load_from(&self, data: HashMap<String, Value>) {
        let mut store = self.store.write().await;
        *store = data;
    }

    /// Check if a key exists
    #[allow(dead_code)]
    pub async fn contains_key(&self, key: &str) -> bool {
        let store = self.store.read().await;
        store.contains_key(key)
    }

    /// Get all keys
    #[allow(dead_code)]
    pub async fn keys(&self) -> Vec<String> {
        let store = self.store.read().await;
        store.keys().cloned().collect()
    }
}

/// Keys used by internal tools - centralized for consistency
pub mod tool_state_keys {
    /// Key for the TODO list tool
    pub const TODO_LIST: &str = "todo_list";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_internal_tool_state() {
        let state = InternalToolState::new();

        // Test set and get
        state.set("test_key".to_string(), json!("test_value")).await;
        assert_eq!(state.get("test_key").await, Some(json!("test_value")));

        // Test contains_key
        assert!(state.contains_key("test_key").await);
        assert!(!state.contains_key("nonexistent").await);

        // Test remove
        let removed = state.remove("test_key").await;
        assert_eq!(removed, Some(json!("test_value")));
        assert!(!state.contains_key("test_key").await);

        // Test get_all and load_from
        let mut data = HashMap::new();
        data.insert("key1".to_string(), json!("value1"));
        data.insert("key2".to_string(), json!(42));
        
        state.load_from(data.clone()).await;
        let all = state.get_all().await;
        assert_eq!(all, data);

        // Test clear
        state.clear().await;
        assert_eq!(state.get_all().await.len(), 0);
    }
}
