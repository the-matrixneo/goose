use crate::config::{Config, ConfigError};
use crate::providers::custom::{AnthropicCompatibleConfig, OpenAICompatibleConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// custom providers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CustomProviderConfig {
    #[serde(rename = "openai_compatible")]
    OpenAICompatible(OpenAICompatibleConfig),
    #[serde(rename = "anthropic_compatible")]
    AnthropicCompatible(AnthropicCompatibleConfig),
}

impl CustomProviderConfig {
    pub fn id(&self) -> &str {
        match self {
            CustomProviderConfig::OpenAICompatible(config) => &config.id,
            CustomProviderConfig::AnthropicCompatible(config) => &config.id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            CustomProviderConfig::OpenAICompatible(config) => &config.display_name,
            CustomProviderConfig::AnthropicCompatible(config) => &config.display_name,
        }
    }

    pub fn models(&self) -> &[String] {
        match self {
            CustomProviderConfig::OpenAICompatible(config) => &config.models,
            CustomProviderConfig::AnthropicCompatible(config) => &config.models,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            CustomProviderConfig::OpenAICompatible(config) => config.enabled,
            CustomProviderConfig::AnthropicCompatible(config) => config.enabled,
        }
    }
}

pub struct CustomProviderManager;

impl CustomProviderManager {
    /// get all added custom providers
    pub fn get_all() -> Result<Vec<CustomProviderConfig>> {
        let config = Config::global();
        match config.get("custom_providers", false) {
            Ok(value) => {
                let custom_providers: HashMap<String, CustomProviderConfig> =
                    serde_json::from_value(value).unwrap_or_default();
                Ok(custom_providers.into_values().collect())
            }
            Err(ConfigError::NotFound(_)) => Ok(vec![]),
            Err(e) => Err(e.into()),
        }
    }

    /// get a specific custom provider by its id
    pub fn get(id: &str) -> Result<Option<CustomProviderConfig>> {
        let config = Config::global();
        match config.get("custom_providers", false) {
            Ok(value) => {
                let custom_providers: HashMap<String, CustomProviderConfig> =
                    serde_json::from_value(value).unwrap_or_default();
                Ok(custom_providers.get(id).cloned())
            }
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // TODO: add/update a custom provider

    pub fn set(provider_config: CustomProviderConfig) -> Result<()> {
        let config = Config::global();
        let mut custom_providers: HashMap<String, CustomProviderConfig> =
            match config.get("custom_providers", false) {
                Ok(value) => serde_json::from_value(value).unwrap_or_default(),
                Err(ConfigError::NotFound(_)) => HashMap::new(),
                Err(e) => return Err(e.into()),
            };

        custom_providers.insert(provider_config.id().to_string(), provider_config);
        config.set(
            "custom_providers",
            serde_json::to_value(custom_providers)?,
            false,
        )?;
        Ok(())
    }

    // TODO: remove a custom provider

    pub fn remove(id: &str) -> Result<()> {
        let config = Config::global();
        let mut custom_providers: HashMap<String, CustomProviderConfig> =
            match config.get("custom_providers", false) {
                Ok(value) => serde_json::from_value(value).unwrap_or_default(),
                Err(ConfigError::NotFound(_)) => return Ok(()),
                Err(e) => return Err(e.into()),
            };

        custom_providers.remove(id);
        config.set(
            "custom_providers",
            serde_json::to_value(custom_providers)?,
            false,
        )?;
        Ok(())
    }

    pub fn exists(id: &str) -> Result<bool> {
        let config = Config::global();
        match config.get("custom_providers", false) {
            Ok(value) => {
                let custom_providers: HashMap<String, CustomProviderConfig> =
                    serde_json::from_value(value).unwrap_or_default();
                Ok(custom_providers.contains_key(id))
            }
            Err(ConfigError::NotFound(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}
