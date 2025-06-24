use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::{BitMortarConfig, LoadBalancingStrategy};
use crate::error::BitMortarError;
use crate::providers::BitMortarProvider;
use crate::types::{ChatCompletionRequest, EmbeddingRequest, EndpointInfo, ModelInfo};

pub struct BitMortarServer {
    config: BitMortarConfig,
    providers: HashMap<String, Arc<BitMortarProvider>>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
}

impl BitMortarServer {
    pub async fn new(config: BitMortarConfig) -> Result<Self, BitMortarError> {
        let mut providers: HashMap<String, Arc<BitMortarProvider>> = HashMap::new();

        // Initialize providers based on configuration
        for (name, provider_config) in &config.providers {
            if !provider_config.enabled {
                continue;
            }

            let provider = BitMortarProvider::new(&provider_config.provider_type, provider_config.config.clone())?;
            providers.insert(name.clone(), Arc::new(provider));
        }

        let load_balancer = Arc::new(RwLock::new(LoadBalancer::new(
            config.routing.load_balancing.clone(),
        )));

        Ok(Self {
            config,
            providers,
            load_balancer,
        })
    }

    pub async fn list_endpoints(&self) -> Result<Vec<EndpointInfo>, BitMortarError> {
        let mut endpoints = Vec::new();

        // Get all available models from all providers
        for (provider_name, provider) in &self.providers {
            match provider.list_models().await {
                Ok(models) => {
                    for model in models {
                        endpoints.push(EndpointInfo {
                            name: model.id.clone(),
                            config: crate::types::EndpointConfig {
                                served_models: vec![crate::types::ServedModel {
                                    name: model.id.clone(),
                                    model_name: model.id.clone(),
                                    model_version: "1".to_string(),
                                }],
                            },
                            state: crate::types::EndpointState {
                                ready: "READY".to_string(),
                                config_update: "NOT_UPDATING".to_string(),
                            },
                        });
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to list models for provider {}: {}", provider_name, e);
                }
            }
        }

        Ok(endpoints)
    }

    pub async fn chat_completions(
        &self,
        model: &str,
        payload: Value,
    ) -> Result<Value, BitMortarError> {
        // Parse the request
        let mut request: ChatCompletionRequest = serde_json::from_value(payload)
            .map_err(|e| BitMortarError::RequestError(format!("Invalid request format: {}", e)))?;

        // Ensure model is set in the request
        if request.extra.get("model").is_none() {
            if let Some(obj) = request.extra.as_object_mut() {
                obj.insert("model".to_string(), serde_json::json!(model));
            }
        }

        let provider = self.select_provider_for_model(model).await?;
        let response = provider.chat_completion(request).await?;
        Ok(serde_json::to_value(response)?)
    }

    pub async fn embeddings(&self, payload: Value) -> Result<Value, BitMortarError> {
        // Parse the request
        let request: EmbeddingRequest = serde_json::from_value(payload)
            .map_err(|e| BitMortarError::RequestError(format!("Invalid embedding request format: {}", e)))?;

        // For embeddings, we'll use the first available provider with embedding support
        let provider = self.select_provider_for_embeddings().await?;
        let response = provider.create_embeddings(request).await?;
        Ok(serde_json::to_value(response)?)
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, BitMortarError> {
        let mut all_models = Vec::new();

        for (provider_name, provider) in &self.providers {
            match provider.list_models().await {
                Ok(models) => {
                    all_models.extend(models);
                }
                Err(e) => {
                    tracing::warn!("Failed to list models for provider {}: {}", provider_name, e);
                }
            }
        }

        Ok(all_models)
    }

    async fn select_provider_for_model(&self, model: &str) -> Result<Arc<BitMortarProvider>, BitMortarError> {
        // Check if there's a specific route for this model
        if let Some(route) = self.config.routing.model_routes.get(model) {
            if let Some(provider) = self.providers.get(&route.provider) {
                return Ok(provider.clone());
            }

            // Try fallbacks
            for fallback in &route.fallbacks {
                if let Some(provider) = self.providers.get(fallback) {
                    return Ok(provider.clone());
                }
            }
        }

        // Use default provider
        if let Some(provider) = self.providers.get(&self.config.routing.default_provider) {
            return Ok(provider.clone());
        }

        // Use load balancer to select any available provider
        let mut lb = self.load_balancer.write().await;
        let provider_name = lb.select_provider(&self.providers).await?;
        
        self.providers
            .get(&provider_name)
            .cloned()
            .ok_or_else(|| BitMortarError::ProviderNotFound(provider_name))
    }

    async fn select_provider_for_embeddings(&self) -> Result<Arc<BitMortarProvider>, BitMortarError> {
        // Find first provider that supports embeddings
        for (_, provider) in &self.providers {
            if provider.supports_embeddings() {
                return Ok(provider.clone());
            }
        }

        Err(BitMortarError::ProviderError(
            "No provider supports embeddings".to_string(),
        ))
    }
}

struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    round_robin_index: usize,
}

impl LoadBalancer {
    fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            strategy,
            round_robin_index: 0,
        }
    }

    async fn select_provider(
        &mut self,
        providers: &HashMap<String, Arc<BitMortarProvider>>,
    ) -> Result<String, BitMortarError> {
        if providers.is_empty() {
            return Err(BitMortarError::ProviderError("No providers available".to_string()));
        }

        let provider_names: Vec<String> = providers.keys().cloned().collect();

        match self.strategy {
            LoadBalancingStrategy::FirstAvailable => {
                for name in &provider_names {
                    if let Some(provider) = providers.get(name) {
                        if provider.health_check().await {
                            return Ok(name.clone());
                        }
                    }
                }
                Err(BitMortarError::ProviderError("No healthy providers available".to_string()))
            }
            LoadBalancingStrategy::RoundRobin => {
                let index = self.round_robin_index % provider_names.len();
                self.round_robin_index += 1;
                Ok(provider_names[index].clone())
            }
            LoadBalancingStrategy::Priority => {
                // This would require priority information from config
                // For now, just return first available
                Ok(provider_names[0].clone())
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let index = rand::thread_rng().gen_range(0..provider_names.len());
                Ok(provider_names[index].clone())
            }
        }
    }
}
