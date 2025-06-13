use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// Agent Card represents an agent's capabilities and connection information
/// as specified by the A2A protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentCard {
    /// Unique identifier for the agent
    pub id: String,
    /// Human-readable name of the agent
    pub name: String,
    /// Description of the agent's purpose and capabilities
    pub description: Option<String>,
    /// Version of the agent implementation
    pub version: String,
    /// Connection information for reaching this agent
    pub connection: ConnectionInfo,
    /// Capabilities that this agent supports
    pub capabilities: Vec<Capability>,
    /// Metadata for additional agent information
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Connection information for establishing communication with an agent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionInfo {
    /// Base URL for the agent's A2A endpoint
    pub base_url: Url,
    /// Supported communication protocols
    pub protocols: Vec<String>,
    /// Authentication information if required
    pub auth: Option<AuthInfo>,
    /// Connection timeout in seconds
    pub timeout: Option<u32>,
}

/// Authentication information for secure agent communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    /// Type of authentication (e.g., "bearer", "api_key", "oauth2")
    pub auth_type: String,
    /// Authentication credentials or metadata
    pub credentials: HashMap<String, String>,
}

/// Represents a specific capability that an agent can perform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Capability {
    /// Unique identifier for the capability
    pub id: String,
    /// Human-readable name of the capability
    pub name: String,
    /// Description of what this capability does
    pub description: String,
    /// Input schema for invoking this capability
    pub input_schema: Option<serde_json::Value>,
    /// Output schema for the capability's response
    pub output_schema: Option<serde_json::Value>,
    /// Whether this capability supports streaming responses
    pub supports_streaming: bool,
    /// Tags for categorizing the capability
    pub tags: Vec<String>,
}

impl AgentCard {
    /// Create a new AgentCard with required fields
    pub fn new(
        id: String,
        name: String,
        version: String,
        base_url: Url,
    ) -> Self {
        Self {
            id,
            name,
            description: None,
            version,
            connection: ConnectionInfo {
                base_url,
                protocols: vec!["http".to_string(), "https".to_string()],
                auth: None,
                timeout: Some(30),
            },
            capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a capability to this agent
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Set the description for this agent
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add metadata to this agent
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set authentication information
    pub fn with_auth(mut self, auth: AuthInfo) -> Self {
        self.connection.auth = Some(auth);
        self
    }

    /// Check if this agent has a specific capability
    pub fn has_capability(&self, capability_id: &str) -> bool {
        self.capabilities.iter().any(|c| c.id == capability_id)
    }

    /// Get a capability by ID
    pub fn get_capability(&self, capability_id: &str) -> Option<&Capability> {
        self.capabilities.iter().find(|c| c.id == capability_id)
    }

    /// Get all capabilities with a specific tag
    pub fn get_capabilities_by_tag(&self, tag: &str) -> Vec<&Capability> {
        self.capabilities
            .iter()
            .filter(|c| c.tags.contains(&tag.to_string()))
            .collect()
    }
}

impl Capability {
    /// Create a new capability with required fields
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            input_schema: None,
            output_schema: None,
            supports_streaming: false,
            tags: Vec::new(),
        }
    }

    /// Add a tag to this capability
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Set input schema for this capability
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Set output schema for this capability
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Enable streaming support for this capability
    pub fn with_streaming(mut self) -> Self {
        self.supports_streaming = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_agent_card_creation() {
        let url = Url::parse("https://agent.example.com").unwrap();
        let card = AgentCard::new(
            "agent1".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            url.clone(),
        );

        assert_eq!(card.id, "agent1");
        assert_eq!(card.name, "Test Agent");
        assert_eq!(card.version, "1.0.0");
        assert_eq!(card.connection.base_url, url);
        assert!(card.capabilities.is_empty());
    }

    #[test]
    fn test_capability_creation() {
        let cap = Capability::new(
            "search".to_string(),
            "Search Documents".to_string(),
            "Search through document database".to_string(),
        )
        .with_tag("search".to_string())
        .with_streaming();

        assert_eq!(cap.id, "search");
        assert_eq!(cap.name, "Search Documents");
        assert!(cap.supports_streaming);
        assert!(cap.tags.contains(&"search".to_string()));
    }

    #[test]
    fn test_agent_card_with_capability() {
        let url = Url::parse("https://agent.example.com").unwrap();
        let capability = Capability::new(
            "analyze".to_string(),
            "Analyze Data".to_string(),
            "Perform data analysis".to_string(),
        );

        let card = AgentCard::new(
            "agent1".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            url,
        )
        .with_capability(capability.clone());

        assert!(card.has_capability("analyze"));
        assert!(!card.has_capability("nonexistent"));
        assert_eq!(card.get_capability("analyze").unwrap(), &capability);
    }

    #[test]
    fn test_serialization() {
        let url = Url::parse("https://agent.example.com").unwrap();
        let capability = Capability::new(
            "test_cap".to_string(),
            "Test Capability".to_string(),
            "A test capability".to_string(),
        )
        .with_input_schema(json!({"type": "object"}));

        let card = AgentCard::new(
            "agent1".to_string(),
            "Test Agent".to_string(),
            "1.0.0".to_string(),
            url,
        )
        .with_capability(capability)
        .with_description("A test agent".to_string())
        .with_metadata("environment".to_string(), json!("test"));

        // Test serialization
        let json = serde_json::to_string(&card).unwrap();
        let deserialized: AgentCard = serde_json::from_str(&json).unwrap();

        assert_eq!(card, deserialized);
    }
}