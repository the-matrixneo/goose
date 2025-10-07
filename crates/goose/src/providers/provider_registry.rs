use super::base::{Provider, ProviderMetadata};
use crate::model::ModelConfig;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

type ProviderConstructor = Box<dyn Fn(ModelConfig) -> Result<Arc<dyn Provider>> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    Preferred,
    Builtin,
    Declarative,
    Custom,
}

struct ProviderEntry {
    metadata: ProviderMetadata,
    constructor: ProviderConstructor,
    provider_type: ProviderType,
}

#[derive(Default)]
pub struct ProviderRegistry {
    entries: HashMap<String, ProviderEntry>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register<P, F>(&mut self, constructor: F, preferred: bool)
    where
        P: Provider + 'static,
        F: Fn(ModelConfig) -> Result<P> + Send + Sync + 'static,
    {
        let metadata = P::metadata();
        let name = metadata.name.clone();

        self.entries.insert(
            name,
            ProviderEntry {
                metadata,
                constructor: Box::new(move |model| Ok(Arc::new(constructor(model)?))),
                provider_type: if preferred {
                    ProviderType::Preferred
                } else {
                    ProviderType::Builtin
                },
            },
        );
    }

    /// create provider with custom name
    pub fn register_with_name<P, F>(
        &mut self,
        custom_name: String,
        display_name: String,
        description: String,
        default_model: String,
        known_models: Vec<super::base::ModelInfo>,
        provider_type: ProviderType,
        constructor: F,
    ) where
        P: Provider + 'static,
        F: Fn(ModelConfig) -> Result<P> + Send + Sync + 'static,
    {
        let base_metadata = P::metadata();
        let custom_metadata = ProviderMetadata {
            name: custom_name.clone(),
            display_name,
            description,
            default_model,
            known_models,
            model_doc_link: base_metadata.model_doc_link,
            config_keys: base_metadata.config_keys,
        };

        self.entries.insert(
            custom_name,
            ProviderEntry {
                metadata: custom_metadata,
                constructor: Box::new(move |model| Ok(Arc::new(constructor(model)?))),
                provider_type,
            },
        );
    }

    pub fn with_providers<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        setup(&mut self);
        self
    }

    pub fn create(&self, name: &str, model: ModelConfig) -> Result<Arc<dyn Provider>> {
        let _available_providers: Vec<_> = self.entries.keys().collect();

        let entry = self
            .entries
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Unknown provider: {}", name))?;

        (entry.constructor)(model)
    }

    pub fn all_metadata_with_types(&self) -> Vec<(ProviderMetadata, ProviderType)> {
        self.entries
            .values()
            .map(|e| (e.metadata.clone(), e.provider_type))
            .collect()
    }

    pub fn remove_custom_providers(&mut self) {
        self.entries.retain(|name, _| !name.starts_with("custom_"));
    }
}
