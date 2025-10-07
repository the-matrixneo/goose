use crate::config::paths::Paths;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use utoipa::ToSchema;

/// Enum representing the possible permission levels for a tool.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    AlwaysAllow, // Tool can always be used without prompt
    AskBefore,   // Tool requires permission to be granted before use
    NeverAllow,  // Tool is never allowed to be used
}

/// Struct representing the configuration of permissions, categorized by level.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PermissionConfig {
    pub always_allow: Vec<String>, // List of tools that are always allowed
    pub ask_before: Vec<String>,   // List of tools that require user consent
    pub never_allow: Vec<String>,  // List of tools that are never allowed
}

/// PermissionManager manages permission configurations for various tools.
#[derive(Debug, Clone)]
pub struct PermissionManager {
    config_path: PathBuf, // Path to the permission configuration file
    permission_map: HashMap<String, PermissionConfig>, // Mapping of permission names to configurations
}

// Constants representing specific permission categories
const USER_PERMISSION: &str = "user";
const SMART_APPROVE_PERMISSION: &str = "smart_approve";

/// Implements the default constructor for `PermissionManager`.
impl Default for PermissionManager {
    fn default() -> Self {
        let config_path = Paths::config_dir().join("permission.yaml");

        // Load the existing configuration file or create an empty map if the file doesn't exist
        let permission_map = if config_path.exists() {
            // Load the configuration file
            let file_contents =
                fs::read_to_string(&config_path).expect("Failed to read permission.yaml");
            serde_yaml::from_str(&file_contents).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new() // No config file, create an empty map
        };

        PermissionManager {
            config_path,
            permission_map,
        }
    }
}

impl PermissionManager {
    /// Creates a new `PermissionManager` with a specified config path.
    pub fn new<P: AsRef<Path>>(config_path: P) -> Self {
        let config_path = config_path.as_ref().to_path_buf();

        // Load the existing configuration file or create an empty map if the file doesn't exist
        let permission_map = if config_path.exists() {
            // Load the configuration file
            let file_contents =
                fs::read_to_string(&config_path).expect("Failed to read permission.yaml");
            serde_yaml::from_str(&file_contents).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new() // No config file, create an empty map
        };

        PermissionManager {
            config_path,
            permission_map,
        }
    }

    /// Returns a list of all the names (keys) in the permission map.
    pub fn get_permission_names(&self) -> Vec<String> {
        self.permission_map.keys().cloned().collect()
    }

    /// Retrieves the user permission level for a specific tool.
    pub fn get_user_permission(&self, principal_name: &str) -> Option<PermissionLevel> {
        self.get_permission(USER_PERMISSION, principal_name)
    }

    /// Retrieves the smart approve permission level for a specific tool.
    pub fn get_smart_approve_permission(&self, principal_name: &str) -> Option<PermissionLevel> {
        self.get_permission(SMART_APPROVE_PERMISSION, principal_name)
    }

    /// Helper function to retrieve the permission level for a specific permission category and tool.
    fn get_permission(&self, name: &str, principal_name: &str) -> Option<PermissionLevel> {
        // Check if the permission category exists in the map
        if let Some(permission_config) = self.permission_map.get(name) {
            // Check the permission levels for the given tool
            if permission_config
                .always_allow
                .contains(&principal_name.to_string())
            {
                return Some(PermissionLevel::AlwaysAllow);
            } else if permission_config
                .ask_before
                .contains(&principal_name.to_string())
            {
                return Some(PermissionLevel::AskBefore);
            } else if permission_config
                .never_allow
                .contains(&principal_name.to_string())
            {
                return Some(PermissionLevel::NeverAllow);
            }
        }
        None // Return None if no matching permission level is found
    }

    /// Updates the user permission level for a specific tool.
    pub fn update_user_permission(&mut self, principal_name: &str, level: PermissionLevel) {
        self.update_permission(USER_PERMISSION, principal_name, level)
    }

    /// Updates the smart approve permission level for a specific tool.
    pub fn update_smart_approve_permission(
        &mut self,
        principal_name: &str,
        level: PermissionLevel,
    ) {
        self.update_permission(SMART_APPROVE_PERMISSION, principal_name, level)
    }

    /// Helper function to update a permission level for a specific tool in a given permission category.
    fn update_permission(&mut self, name: &str, principal_name: &str, level: PermissionLevel) {
        // Get or create a new PermissionConfig for the specified category
        let permission_config = self.permission_map.entry(name.to_string()).or_default();

        // Remove the principal from all existing lists to avoid duplicates
        permission_config
            .always_allow
            .retain(|p| p != principal_name);
        permission_config.ask_before.retain(|p| p != principal_name);
        permission_config
            .never_allow
            .retain(|p| p != principal_name);

        // Add the principal to the appropriate list
        match level {
            PermissionLevel::AlwaysAllow => permission_config
                .always_allow
                .push(principal_name.to_string()),
            PermissionLevel::AskBefore => permission_config
                .ask_before
                .push(principal_name.to_string()),
            PermissionLevel::NeverAllow => permission_config
                .never_allow
                .push(principal_name.to_string()),
        }

        // Serialize the updated permission map and write it back to the config file
        let yaml_content = serde_yaml::to_string(&self.permission_map)
            .expect("Failed to serialize permission config");
        fs::write(&self.config_path, yaml_content).expect("Failed to write to permission.yaml");
    }

    /// Removes all entries where the principal name starts with the given extension name.
    pub fn remove_extension(&mut self, extension_name: &str) {
        for permission_config in self.permission_map.values_mut() {
            permission_config
                .always_allow
                .retain(|p| !p.starts_with(extension_name));
            permission_config
                .ask_before
                .retain(|p| !p.starts_with(extension_name));
            permission_config
                .never_allow
                .retain(|p| !p.starts_with(extension_name));
        }

        let yaml_content = serde_yaml::to_string(&self.permission_map)
            .expect("Failed to serialize permission config");
        fs::write(&self.config_path, yaml_content).expect("Failed to write to permission.yaml");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    // Helper function to create a test instance of PermissionManager with a temp dir
    fn create_test_permission_manager() -> PermissionManager {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path();
        PermissionManager::new(temp_path)
    }

    #[test]
    fn test_get_permission_names_empty() {
        let manager = create_test_permission_manager();

        assert!(manager.get_permission_names().is_empty());
    }

    #[test]
    fn test_update_user_permission() {
        let mut manager = create_test_permission_manager();
        manager.update_user_permission("tool1", PermissionLevel::AlwaysAllow);

        let permission = manager.get_user_permission("tool1");
        assert_eq!(permission, Some(PermissionLevel::AlwaysAllow));
    }

    #[test]
    fn test_update_smart_approve_permission() {
        let mut manager = create_test_permission_manager();
        manager.update_smart_approve_permission("tool2", PermissionLevel::AskBefore);

        let permission = manager.get_smart_approve_permission("tool2");
        assert_eq!(permission, Some(PermissionLevel::AskBefore));
    }

    #[test]
    fn test_get_permission_not_found() {
        let manager = create_test_permission_manager();

        let permission = manager.get_user_permission("non_existent_tool");
        assert_eq!(permission, None);
    }

    #[test]
    fn test_permission_levels() {
        let mut manager = create_test_permission_manager();

        manager.update_user_permission("tool4", PermissionLevel::AlwaysAllow);
        manager.update_user_permission("tool5", PermissionLevel::AskBefore);
        manager.update_user_permission("tool6", PermissionLevel::NeverAllow);

        // Check the permission levels
        assert_eq!(
            manager.get_user_permission("tool4"),
            Some(PermissionLevel::AlwaysAllow)
        );
        assert_eq!(
            manager.get_user_permission("tool5"),
            Some(PermissionLevel::AskBefore)
        );
        assert_eq!(
            manager.get_user_permission("tool6"),
            Some(PermissionLevel::NeverAllow)
        );
    }

    #[test]
    fn test_permission_update_replaces_existing_level() {
        let mut manager = create_test_permission_manager();

        // Initially AlwaysAllow
        manager.update_user_permission("tool7", PermissionLevel::AlwaysAllow);
        assert_eq!(
            manager.get_user_permission("tool7"),
            Some(PermissionLevel::AlwaysAllow)
        );

        // Now change to NeverAllow
        manager.update_user_permission("tool7", PermissionLevel::NeverAllow);
        assert_eq!(
            manager.get_user_permission("tool7"),
            Some(PermissionLevel::NeverAllow)
        );

        // Ensure it's removed from other levels
        let config = manager.permission_map.get(USER_PERMISSION).unwrap();
        assert!(!config.always_allow.contains(&"tool7".to_string()));
        assert!(!config.ask_before.contains(&"tool7".to_string()));
        assert!(config.never_allow.contains(&"tool7".to_string()));
    }

    #[test]
    fn test_remove_extension() {
        let mut manager = create_test_permission_manager();
        manager.update_user_permission("prefix__tool1", PermissionLevel::AlwaysAllow);
        manager.update_user_permission("nonprefix__tool2", PermissionLevel::AlwaysAllow);
        manager.update_user_permission("prefix__tool3", PermissionLevel::AskBefore);

        // Remove entries starting with "prefix"
        manager.remove_extension("prefix");

        let config = manager.permission_map.get(USER_PERMISSION).unwrap();

        // Verify entries with "prefix" are removed
        assert!(!config.always_allow.contains(&"prefix__tool1".to_string()));
        assert!(!config.ask_before.contains(&"prefix__tool3".to_string()));

        // Verify other entries remain
        assert!(config
            .always_allow
            .contains(&"nonprefix__tool2".to_string()));
    }
}
