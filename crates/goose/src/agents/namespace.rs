use dashmap::DashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NamespaceError {
    #[error("Extension {0} does not have access to namespace {1}")]
    AccessDenied(String, String),
    #[error("Namespace {0} does not exist")]
    NamespaceNotFound(String),
    #[error("Extension {0} is not registered")]
    ExtensionNotFound(String),
}

pub type NamespaceResult<T> = Result<T, NamespaceError>;

/// Manages namespace access control for extensions
#[derive(Clone, Debug)]
pub struct NamespaceManager {
    // Key: namespace name, Value: Set of extension names that have access
    namespaces: Arc<DashMap<String, dashmap::DashSet<String>>>,
}

impl Default for NamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(DashMap::new()),
        }
    }

    /// Grant access to a namespace for multiple extensions
    pub fn grant_access(&self, namespace: &str, extension_names: Vec<String>) {
        let extensions = self
            .namespaces
            .entry(namespace.to_string())
            .or_insert_with(dashmap::DashSet::new);
        
        for name in extension_names {
            extensions.insert(name);
        }
    }

    /// Revoke access from a namespace for an extension
    pub fn revoke_access(&self, namespace: &str, extension_name: &str) -> NamespaceResult<()> {
        match self.namespaces.get(namespace) {
            Some(extensions) => {
                extensions.remove(extension_name);
                Ok(())
            }
            None => Err(NamespaceError::NamespaceNotFound(namespace.to_string())),
        }
    }

    /// Check if an extension has access to a namespace
    pub fn has_access(&self, namespace: &str, extension_name: &str) -> bool {
        self.namespaces
            .get(namespace)
            .map(|extensions| extensions.contains(extension_name))
            .unwrap_or(false)
    }

    /// List all namespaces an extension has access to
    pub fn list_namespaces(&self, extension_name: &str) -> Vec<String> {
        self.namespaces
            .iter()
            .filter(|entry| entry.value().contains(extension_name))
            .map(|entry| entry.key().clone())
            .collect()
    }

    /// Remove a namespace entirely
    pub fn remove_namespace(&self, namespace: &str) -> NamespaceResult<()> {
        match self.namespaces.remove(namespace) {
            Some(_) => Ok(()),
            None => Err(NamespaceError::NamespaceNotFound(namespace.to_string())),
        }
    }

    /// Clear all namespace data
    pub fn clear(&self) {
        self.namespaces.clear();
    }

    /// Get all extensions with access to a namespace
    pub fn list_extensions(&self, namespace: &str) -> NamespaceResult<Vec<String>> {
        match self.namespaces.get(namespace) {
            Some(extensions) => Ok(extensions.iter().map(|ext| ext.clone()).collect()),
            None => Err(NamespaceError::NamespaceNotFound(namespace.to_string())),
        }
    }

    /// Check if a namespace exists
    pub fn namespace_exists(&self, namespace: &str) -> bool {
        self.namespaces.contains_key(namespace)
    }

    /// Create a new namespace without granting any access
    pub fn create_namespace(&self, namespace: &str) {
        self.namespaces
            .entry(namespace.to_string())
            .or_insert_with(dashmap::DashSet::new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_access_control() {
        let nm = NamespaceManager::new();
        
        // Grant access
        nm.grant_access("test_ns", vec!["ext1".to_string()]);
        assert!(nm.has_access("test_ns", "ext1"));
        assert!(!nm.has_access("test_ns", "ext2"));

        // Revoke access
        nm.revoke_access("test_ns", "ext1").unwrap();
        assert!(!nm.has_access("test_ns", "ext1"));
    }

    #[test]
    fn test_multiple_extensions() {
        let nm = NamespaceManager::new();
        
        // Grant access to multiple extensions
        nm.grant_access(
            "shared_ns",
            vec!["ext1".to_string(), "ext2".to_string(), "ext3".to_string()],
        );

        assert!(nm.has_access("shared_ns", "ext1"));
        assert!(nm.has_access("shared_ns", "ext2"));
        assert!(nm.has_access("shared_ns", "ext3"));
        assert!(!nm.has_access("shared_ns", "ext4"));
    }

    #[test]
    fn test_list_namespaces() {
        let nm = NamespaceManager::new();
        
        nm.grant_access("ns1", vec!["ext1".to_string()]);
        nm.grant_access("ns2", vec!["ext1".to_string()]);
        nm.grant_access("ns3", vec!["ext2".to_string()]);

        let ext1_namespaces = nm.list_namespaces("ext1");
        assert_eq!(ext1_namespaces.len(), 2);
        assert!(ext1_namespaces.contains(&"ns1".to_string()));
        assert!(ext1_namespaces.contains(&"ns2".to_string()));

        let ext2_namespaces = nm.list_namespaces("ext2");
        assert_eq!(ext2_namespaces.len(), 1);
        assert!(ext2_namespaces.contains(&"ns3".to_string()));
    }

    #[test]
    fn test_namespace_management() {
        let nm = NamespaceManager::new();
        
        // Create empty namespace
        nm.create_namespace("empty_ns");
        assert!(nm.namespace_exists("empty_ns"));
        
        // List extensions in empty namespace
        let extensions = nm.list_extensions("empty_ns").unwrap();
        assert!(extensions.is_empty());

        // Remove namespace
        nm.remove_namespace("empty_ns").unwrap();
        assert!(!nm.namespace_exists("empty_ns"));

        // Try to remove non-existent namespace
        assert!(nm.remove_namespace("nonexistent").is_err());
    }

    #[test]
    fn test_clear_all() {
        let nm = NamespaceManager::new();
        
        nm.grant_access("ns1", vec!["ext1".to_string()]);
        nm.grant_access("ns2", vec!["ext2".to_string()]);
        
        assert!(nm.namespace_exists("ns1"));
        assert!(nm.namespace_exists("ns2"));

        nm.clear();

        assert!(!nm.namespace_exists("ns1"));
        assert!(!nm.namespace_exists("ns2"));
    }

    #[test]
    fn test_error_handling() {
        let nm = NamespaceManager::new();
        
        // Try to revoke access from non-existent namespace
        let result = nm.revoke_access("nonexistent", "ext1");
        assert!(matches!(result, Err(NamespaceError::NamespaceNotFound(_))));

        // Try to list extensions for non-existent namespace
        let result = nm.list_extensions("nonexistent");
        assert!(matches!(result, Err(NamespaceError::NamespaceNotFound(_))));
    }

    #[test]
    fn test_namespace_integration_with_agent() {
        let nm = NamespaceManager::new();
        
        // Set up namespace access like the agent would
        nm.grant_access("main", vec!["developer".to_string(), "memory".to_string()]);
        nm.grant_access("subagent_id1", vec!["developer".to_string(), "slack".to_string()]);
        
        // Test main namespace access
        assert!(nm.has_access("main", "developer"));
        assert!(nm.has_access("main", "memory"));
        assert!(!nm.has_access("main", "slack"));
        
        // Test subagent namespace access
        assert!(nm.has_access("subagent_id1", "developer"));
        assert!(nm.has_access("subagent_id1", "slack"));
        assert!(!nm.has_access("subagent_id1", "memory"));
        
        // Test listing namespaces for extensions
        let developer_namespaces = nm.list_namespaces("developer");
        assert_eq!(developer_namespaces.len(), 2);
        assert!(developer_namespaces.contains(&"main".to_string()));
        assert!(developer_namespaces.contains(&"subagent_id1".to_string()));
        
        let slack_namespaces = nm.list_namespaces("slack");
        assert_eq!(slack_namespaces.len(), 1);
        assert!(slack_namespaces.contains(&"subagent_id1".to_string()));
    }
}