//! Secrets management for Goose configuration
//! 
//! This module handles secure storage and retrieval of secrets with
//! multiple fallback mechanisms:
//! 1. Keyring (preferred)
//! 2. Environment variables
//! 3. Secrets file (when keyring is disabled)

use anyhow::{anyhow, Result};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use tokio::fs;

/// Secrets manager with multi-layer fallback
pub struct SecretsManager {
    /// Whether keyring is enabled
    keyring_enabled: bool,
    
    /// Keyring service name
    keyring_service: String,
    
    /// Path to secrets file (when keyring is disabled)
    secrets_file: Option<PathBuf>,
    
    /// Cached secrets from file
    cached_secrets: Option<HashMap<String, String>>,
}

impl SecretsManager {
    /// Create a new secrets manager
    pub fn new() -> Self {
        let keyring_enabled = env::var("GOOSE_DISABLE_KEYRING").is_err();
        
        let secrets_file = if !keyring_enabled {
            Some(Self::get_secrets_file_path())
        } else {
            None
        };
        
        Self {
            keyring_enabled,
            keyring_service: "goose".to_string(),
            secrets_file,
            cached_secrets: None,
        }
    }
    
    /// Get the path to the secrets file
    fn get_secrets_file_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("goose")
            .join("secrets.yaml")
    }
    
    /// Get a secret with automatic fallback
    pub async fn get_secret(&self, key: &str) -> Result<String> {
        // 1. Try environment variable first
        let env_key = self.key_to_env_var(key);
        if let Ok(value) = env::var(&env_key) {
            return Ok(value);
        }
        
        // 2. Try keyring (if enabled)
        if self.keyring_enabled {
            if let Ok(value) = self.get_from_keyring(key).await {
                return Ok(value);
            }
        }
        
        // 3. Try secrets file (if keyring disabled)
        if !self.keyring_enabled {
            if let Ok(value) = self.get_from_secrets_file(key).await {
                return Ok(value);
            }
        }
        
        Err(anyhow!("Secret '{}' not found", key))
    }
    
    /// Set a secret
    pub async fn set_secret(&mut self, key: &str, value: &str) -> Result<()> {
        if self.keyring_enabled {
            self.set_in_keyring(key, value).await
        } else {
            self.set_in_secrets_file(key, value).await
        }
    }
    
    /// Delete a secret
    pub async fn delete_secret(&mut self, key: &str) -> Result<()> {
        if self.keyring_enabled {
            self.delete_from_keyring(key).await
        } else {
            self.delete_from_secrets_file(key).await
        }
    }
    
    /// Convert a key to its environment variable name
    fn key_to_env_var(&self, key: &str) -> String {
        // Handle common patterns
        match key {
            k if k.ends_with("_api_key") => {
                // openai_api_key -> OPENAI_API_KEY
                k.to_uppercase()
            }
            k if k.ends_with("_token") => {
                // github_token -> GITHUB_TOKEN
                k.to_uppercase()
            }
            k if k.contains("_") => {
                // Already has underscores, just uppercase
                k.to_uppercase()
            }
            _ => {
                // Default: uppercase and replace non-alphanumeric with underscore
                key.chars()
                    .map(|c| if c.is_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
                    .collect()
            }
        }
    }
    
    /// Get a secret from the keyring
    async fn get_from_keyring(&self, key: &str) -> Result<String> {
        let entry = Entry::new(&self.keyring_service, key)?;
        match entry.get_password() {
            Ok(password) => Ok(password),
            Err(keyring::Error::NoEntry) => Err(anyhow!("Secret not found in keyring")),
            Err(e) => Err(anyhow!("Keyring error: {}", e)),
        }
    }
    
    /// Set a secret in the keyring
    async fn set_in_keyring(&self, key: &str, value: &str) -> Result<()> {
        let entry = Entry::new(&self.keyring_service, key)?;
        entry.set_password(value)?;
        Ok(())
    }
    
    /// Delete a secret from the keyring
    async fn delete_from_keyring(&self, key: &str) -> Result<()> {
        let entry = Entry::new(&self.keyring_service, key)?;
        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(anyhow!("Keyring error: {}", e)),
        }
    }
    
    /// Load secrets from file
    async fn load_secrets_file(&mut self) -> Result<()> {
        if let Some(ref path) = self.secrets_file {
            if path.exists() {
                let content = fs::read_to_string(path).await?;
                let secrets: HashMap<String, String> = serde_yaml::from_str(&content)?;
                self.cached_secrets = Some(secrets);
            } else {
                self.cached_secrets = Some(HashMap::new());
            }
        }
        Ok(())
    }
    
    /// Save secrets to file
    async fn save_secrets_file(&self) -> Result<()> {
        if let Some(ref path) = self.secrets_file {
            if let Some(ref secrets) = self.cached_secrets {
                // Ensure directory exists
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).await?;
                }
                
                let content = serde_yaml::to_string(secrets)?;
                fs::write(path, content).await?;
                
                // Set restrictive permissions on Unix
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let metadata = fs::metadata(path).await?;
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o600); // Read/write for owner only
                    fs::set_permissions(path, permissions).await?;
                }
            }
        }
        Ok(())
    }
    
    /// Get a secret from the secrets file
    async fn get_from_secrets_file(&mut self, key: &str) -> Result<String> {
        if self.cached_secrets.is_none() {
            self.load_secrets_file().await?;
        }
        
        self.cached_secrets
            .as_ref()
            .and_then(|secrets| secrets.get(key))
            .cloned()
            .ok_or_else(|| anyhow!("Secret not found in secrets file"))
    }
    
    /// Set a secret in the secrets file
    async fn set_in_secrets_file(&mut self, key: &str, value: &str) -> Result<()> {
        if self.cached_secrets.is_none() {
            self.load_secrets_file().await?;
        }
        
        if let Some(ref mut secrets) = self.cached_secrets {
            secrets.insert(key.to_string(), value.to_string());
            self.save_secrets_file().await?;
        }
        
        Ok(())
    }
    
    /// Delete a secret from the secrets file
    async fn delete_from_secrets_file(&mut self, key: &str) -> Result<()> {
        if self.cached_secrets.is_none() {
            self.load_secrets_file().await?;
        }
        
        if let Some(ref mut secrets) = self.cached_secrets {
            secrets.remove(key);
            self.save_secrets_file().await?;
        }
        
        Ok(())
    }
    
    /// List all available secrets (keys only, not values)
    pub async fn list_secrets(&mut self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        
        if self.keyring_enabled {
            // Note: Most keyring implementations don't support listing keys
            // This is a limitation we'll have to document
        } else {
            if self.cached_secrets.is_none() {
                self.load_secrets_file().await?;
            }
            
            if let Some(ref secrets) = self.cached_secrets {
                keys.extend(secrets.keys().cloned());
            }
        }
        
        Ok(keys)
    }
}

/// Secret reference that can be resolved at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretRef {
    /// Direct reference to a secret by key
    Direct(String),
    
    /// Reference with explicit source
    Explicit {
        source: SecretSource,
        key: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecretSource {
    Keyring,
    Env,
    File,
}

impl SecretRef {
    /// Resolve the secret value
    pub async fn resolve(&self, manager: &SecretsManager) -> Result<String> {
        match self {
            SecretRef::Direct(key) => manager.get_secret(key).await,
            SecretRef::Explicit { source, key } => {
                match source {
                    SecretSource::Keyring => {
                        if manager.keyring_enabled {
                            manager.get_from_keyring(key).await
                        } else {
                            Err(anyhow!("Keyring is disabled"))
                        }
                    }
                    SecretSource::Env => {
                        let env_key = manager.key_to_env_var(key);
                        env::var(&env_key).map_err(|_| anyhow!("Environment variable '{}' not set", env_key))
                    }
                    SecretSource::File => {
                        let mut manager_mut = manager.clone();
                        manager_mut.get_from_secrets_file(key).await
                    }
                }
            }
        }
    }
}

// Make SecretsManager cloneable for the resolve method
impl Clone for SecretsManager {
    fn clone(&self) -> Self {
        Self {
            keyring_enabled: self.keyring_enabled,
            keyring_service: self.keyring_service.clone(),
            secrets_file: self.secrets_file.clone(),
            cached_secrets: None, // Don't clone the cache
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_to_env_var() {
        let manager = SecretsManager::new();
        
        assert_eq!(manager.key_to_env_var("openai_api_key"), "OPENAI_API_KEY");
        assert_eq!(manager.key_to_env_var("github_token"), "GITHUB_TOKEN");
        assert_eq!(manager.key_to_env_var("some_other_key"), "SOME_OTHER_KEY");
        assert_eq!(manager.key_to_env_var("key-with-dashes"), "KEY_WITH_DASHES");
    }
    
    #[tokio::test]
    async fn test_env_var_fallback() {
        env::set_var("TEST_SECRET_KEY", "test_value");
        
        let manager = SecretsManager::new();
        let result = manager.get_secret("test_secret_key").await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
        
        env::remove_var("TEST_SECRET_KEY");
    }
}
