use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitMortarConfig {
    /// Server configuration
    pub server: ServerConfig,
    /// Provider configurations
    pub providers: HashMap<String, ProviderConfig>,
    /// Model routing configuration
    pub routing: RoutingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Request timeout in seconds
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type (openai, anthropic, databricks, etc.)
    pub provider_type: String,
    /// Provider-specific configuration
    pub config: HashMap<String, String>,
    /// Whether this provider is enabled
    pub enabled: bool,
    /// Priority for load balancing (higher = preferred)
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Default provider to use if no specific routing rule matches
    pub default_provider: String,
    /// Model-specific routing rules
    pub model_routes: HashMap<String, ModelRoute>,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRoute {
    /// Provider to route this model to
    pub provider: String,
    /// Optional model name mapping (if different on the target provider)
    pub target_model: Option<String>,
    /// Fallback providers in order of preference
    pub fallbacks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Use the first available provider
    FirstAvailable,
    /// Round-robin between available providers
    RoundRobin,
    /// Use provider with highest priority
    Priority,
    /// Random selection among available providers
    Random,
}

impl Default for BitMortarConfig {
    fn default() -> Self {
        let mut providers = HashMap::new();
        
        // Default OpenAI provider
        providers.insert(
            "openai".to_string(),
            ProviderConfig {
                provider_type: "openai".to_string(),
                config: HashMap::new(),
                enabled: true,
                priority: 100,
            },
        );

        // Default Anthropic provider
        providers.insert(
            "anthropic".to_string(),
            ProviderConfig {
                provider_type: "anthropic".to_string(),
                config: HashMap::new(),
                enabled: true,
                priority: 90,
            },
        );

        let mut model_routes = HashMap::new();
        
        // Route GPT models to OpenAI
        model_routes.insert(
            "gpt-4o".to_string(),
            ModelRoute {
                provider: "openai".to_string(),
                target_model: None,
                fallbacks: vec![],
            },
        );

        // Route Claude models to Anthropic
        model_routes.insert(
            "claude-3-5-sonnet-latest".to_string(),
            ModelRoute {
                provider: "anthropic".to_string(),
                target_model: None,
                fallbacks: vec![],
            },
        );

        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                timeout: 600,
            },
            providers,
            routing: RoutingConfig {
                default_provider: "openai".to_string(),
                model_routes,
                load_balancing: LoadBalancingStrategy::Priority,
            },
        }
    }
}

impl BitMortarConfig {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        if std::path::Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            let config: BitMortarConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            tracing::warn!("Config file {} not found, using default configuration", path);
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
