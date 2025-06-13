pub mod protocol;
pub mod discovery;
pub mod communication;
pub mod agent_card;

pub use protocol::{A2AProtocol, A2AConfig, A2ARequest, A2AResponse};
pub use discovery::{AgentDiscovery, DiscoveryConfig, DiscoveryRequest};
pub use communication::{A2ACommunication, CommunicationConfig};
pub use agent_card::{AgentCard, Capability};