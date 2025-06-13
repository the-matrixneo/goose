use crate::agents::a2a::agent_card::AgentCard;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use url::Url;

/// Agent discovery service for finding and managing A2A agents
pub struct AgentDiscovery {
    /// HTTP client for making discovery requests
    client: Client,
    /// Cache of discovered agents
    agent_cache: Arc<RwLock<HashMap<String, CachedAgent>>>,
    /// Configuration for discovery behavior
    config: DiscoveryConfig,
}

/// Configuration for agent discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Default timeout for discovery requests
    pub timeout: Duration,
    /// How long to cache agent cards before refreshing
    pub cache_ttl: Duration,
    /// Maximum number of agents to cache
    pub max_cache_size: usize,
    /// List of known discovery endpoints
    pub discovery_endpoints: Vec<Url>,
}

/// Cached agent information
#[derive(Debug, Clone)]
struct CachedAgent {
    card: AgentCard,
    cached_at: SystemTime,
    last_seen: SystemTime,
}

/// Discovery request for finding agents
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryRequest {
    /// Optional query to filter agents
    pub query: Option<String>,
    /// Capabilities to search for
    pub capabilities: Option<Vec<String>>,
    /// Tags to filter by
    pub tags: Option<Vec<String>>,
    /// Maximum number of results
    pub limit: Option<u32>,
}

/// Response from agent discovery
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    /// List of discovered agents
    pub agents: Vec<AgentCard>,
    /// Total number of agents available
    pub total: Option<u32>,
    /// Continuation token for pagination
    pub next_token: Option<String>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            cache_ttl: Duration::from_secs(300), // 5 minutes
            max_cache_size: 1000,
            discovery_endpoints: Vec::new(),
        }
    }
}

impl AgentDiscovery {
    /// Create a new agent discovery service
    pub fn new(config: DiscoveryConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            agent_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Discover agents by querying discovery endpoints
    pub async fn discover_agents(&self, request: DiscoveryRequest) -> Result<DiscoveryResponse> {
        let mut all_agents = Vec::new();
        
        // Query each discovery endpoint
        for endpoint in &self.config.discovery_endpoints {
            match self.query_discovery_endpoint(endpoint, &request).await {
                Ok(response) => {
                    all_agents.extend(response.agents);
                }
                Err(e) => {
                    tracing::warn!("Failed to query discovery endpoint {}: {}", endpoint, e);
                }
            }
        }

        // Cache discovered agents
        self.cache_agents(&all_agents).await;

        // Apply additional filtering if needed
        let filtered_agents = self.filter_agents(all_agents, &request);

        Ok(DiscoveryResponse {
            agents: filtered_agents,
            total: None,
            next_token: None,
        })
    }

    /// Get a specific agent by ID from cache or discovery
    pub async fn get_agent(&self, agent_id: &str) -> Result<Option<AgentCard>> {
        // Check cache first
        {
            let cache = self.agent_cache.read().await;
            if let Some(cached) = cache.get(agent_id) {
                if self.is_cache_valid(&cached.cached_at) {
                    return Ok(Some(cached.card.clone()));
                }
            }
        }

        // If not in cache or expired, try discovery
        let request = DiscoveryRequest {
            query: Some(agent_id.to_string()),
            capabilities: None,
            tags: None,
            limit: Some(1),
        };

        let response = self.discover_agents(request).await?;
        
        Ok(response.agents.into_iter()
            .find(|agent| agent.id == agent_id))
    }

    /// Register an agent with the discovery service
    pub async fn register_agent(&self, agent_card: &AgentCard) -> Result<()> {
        // Cache the agent locally
        self.cache_agent(agent_card).await;

        // Register with discovery endpoints that support registration
        for endpoint in &self.config.discovery_endpoints {
            if let Err(e) = self.register_with_endpoint(endpoint, agent_card).await {
                tracing::warn!("Failed to register with endpoint {}: {}", endpoint, e);
            }
        }

        Ok(())
    }

    /// Unregister an agent from discovery
    pub async fn unregister_agent(&self, agent_id: &str) -> Result<()> {
        // Remove from cache
        {
            let mut cache = self.agent_cache.write().await;
            cache.remove(agent_id);
        }

        // Unregister from endpoints
        for endpoint in &self.config.discovery_endpoints {
            if let Err(e) = self.unregister_from_endpoint(endpoint, agent_id).await {
                tracing::warn!("Failed to unregister from endpoint {}: {}", endpoint, e);
            }
        }

        Ok(())
    }

    /// Get all cached agents
    pub async fn get_cached_agents(&self) -> Vec<AgentCard> {
        let cache = self.agent_cache.read().await;
        cache.values()
            .filter(|cached| self.is_cache_valid(&cached.cached_at))
            .map(|cached| cached.card.clone())
            .collect()
    }

    /// Clear expired entries from cache
    pub async fn cleanup_cache(&self) {
        let mut cache = self.agent_cache.write().await;
        let now = SystemTime::now();
        
        cache.retain(|_id, cached| {
            now.duration_since(cached.cached_at)
                .map(|age| age < self.config.cache_ttl)
                .unwrap_or(false)
        });

        // If still over max size, remove oldest entries
        if cache.len() > self.config.max_cache_size {
            let entries: Vec<_> = cache.iter().map(|(id, cached)| (id.clone(), cached.cached_at)).collect();
            let mut sorted_entries = entries;
            sorted_entries.sort_by_key(|(_, cached_at)| *cached_at);
            
            let to_remove = cache.len() - self.config.max_cache_size;
            for (id, _) in sorted_entries.iter().take(to_remove) {
                cache.remove(id);
            }
        }
    }

    /// Query a specific discovery endpoint
    async fn query_discovery_endpoint(
        &self,
        endpoint: &Url,
        request: &DiscoveryRequest,
    ) -> Result<DiscoveryResponse> {
        let discovery_url = endpoint.join("v1/discover")?;
        
        let response = self.client
            .post(discovery_url)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Discovery request failed: {}", response.status()));
        }

        let discovery_response: DiscoveryResponse = response.json().await?;
        Ok(discovery_response)
    }

    /// Register an agent with a discovery endpoint
    async fn register_with_endpoint(&self, endpoint: &Url, agent_card: &AgentCard) -> Result<()> {
        let register_url = endpoint.join("v1/register")?;
        
        let response = self.client
            .post(register_url)
            .json(agent_card)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Registration failed: {}", response.status()));
        }

        Ok(())
    }

    /// Unregister an agent from a discovery endpoint
    async fn unregister_from_endpoint(&self, endpoint: &Url, agent_id: &str) -> Result<()> {
        let unregister_url = endpoint.join(&format!("v1/unregister/{}", agent_id))?;
        
        let response = self.client
            .delete(unregister_url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Unregistration failed: {}", response.status()));
        }

        Ok(())
    }

    /// Cache multiple agents
    async fn cache_agents(&self, agents: &[AgentCard]) {
        let mut cache = self.agent_cache.write().await;
        let now = SystemTime::now();

        for agent in agents {
            cache.insert(
                agent.id.clone(),
                CachedAgent {
                    card: agent.clone(),
                    cached_at: now,
                    last_seen: now,
                },
            );
        }
    }

    /// Cache a single agent
    async fn cache_agent(&self, agent: &AgentCard) {
        let mut cache = self.agent_cache.write().await;
        let now = SystemTime::now();

        cache.insert(
            agent.id.clone(),
            CachedAgent {
                card: agent.clone(),
                cached_at: now,
                last_seen: now,
            },
        );
    }

    /// Check if cache entry is still valid
    fn is_cache_valid(&self, cached_at: &SystemTime) -> bool {
        SystemTime::now()
            .duration_since(*cached_at)
            .map(|age| age < self.config.cache_ttl)
            .unwrap_or(false)
    }

    /// Filter agents based on request criteria
    fn filter_agents(&self, agents: Vec<AgentCard>, request: &DiscoveryRequest) -> Vec<AgentCard> {
        let mut filtered = agents;

        // Filter by capabilities
        if let Some(required_caps) = &request.capabilities {
            filtered.retain(|agent| {
                required_caps.iter().all(|cap| agent.has_capability(cap))
            });
        }

        // Filter by tags
        if let Some(required_tags) = &request.tags {
            filtered.retain(|agent| {
                required_tags.iter().any(|tag| {
                    agent.capabilities.iter().any(|cap| cap.tags.contains(tag))
                })
            });
        }

        // Apply limit
        if let Some(limit) = request.limit {
            filtered.truncate(limit as usize);
        }

        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::a2a::agent_card::Capability;

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.cache_ttl, Duration::from_secs(300));
        assert_eq!(config.max_cache_size, 1000);
    }

    #[tokio::test]
    async fn test_agent_caching() {
        let config = DiscoveryConfig::default();
        let discovery = AgentDiscovery::new(config);

        let agent_card = AgentCard::new(
            "test_agent".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            Url::parse("https://example.com").unwrap(),
        );

        // Cache the agent
        discovery.cache_agent(&agent_card).await;

        // Retrieve from cache
        let cached = discovery.get_agent("test_agent").await.unwrap();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().id, "test_agent");
    }

    #[test]
    fn test_filter_agents() {
        let config = DiscoveryConfig::default();
        let discovery = AgentDiscovery::new(config);

        let cap1 = Capability::new(
            "search".to_string(),
            "Search".to_string(),
            "Search capability".to_string(),
        ).with_tag("search".to_string());

        let cap2 = Capability::new(
            "analyze".to_string(),
            "Analyze".to_string(),
            "Analysis capability".to_string(),
        ).with_tag("analysis".to_string());

        let agent1 = AgentCard::new(
            "agent1".to_string(),
            "Agent 1".to_string(),
            "1.0.0".to_string(),
            Url::parse("https://example.com").unwrap(),
        ).with_capability(cap1);

        let agent2 = AgentCard::new(
            "agent2".to_string(),
            "Agent 2".to_string(),
            "1.0.0".to_string(),
            Url::parse("https://example.com").unwrap(),
        ).with_capability(cap2);

        let agents = vec![agent1, agent2];

        // Filter by capability
        let request = DiscoveryRequest {
            query: None,
            capabilities: Some(vec!["search".to_string()]),
            tags: None,
            limit: None,
        };

        let filtered = discovery.filter_agents(agents.clone(), &request);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "agent1");

        // Filter by tag
        let request = DiscoveryRequest {
            query: None,
            capabilities: None,
            tags: Some(vec!["analysis".to_string()]),
            limit: None,
        };

        let filtered = discovery.filter_agents(agents, &request);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "agent2");
    }
}