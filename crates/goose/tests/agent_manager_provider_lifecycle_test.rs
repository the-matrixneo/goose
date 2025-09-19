use goose::agents::manager::{AgentManager, AgentManagerConfig};
use goose::session;
use std::sync::Arc;
use std::time::Duration;

// Create a simple mock provider for testing
#[derive(Clone)]
struct SimpleMockProvider;

#[async_trait::async_trait]
impl goose::providers::base::Provider for SimpleMockProvider {
    fn metadata() -> goose::providers::base::ProviderMetadata {
        goose::providers::base::ProviderMetadata::empty()
    }

    async fn complete_with_model(
        &self,
        _model_config: &goose::model::ModelConfig,
        _system: &str,
        _messages: &[goose::conversation::message::Message],
        _tools: &[rmcp::model::Tool],
    ) -> Result<
        (
            goose::conversation::message::Message,
            goose::providers::base::ProviderUsage,
        ),
        goose::providers::errors::ProviderError,
    > {
        use goose::conversation::message::{Message, MessageContent};
        use goose::providers::base::{ProviderUsage, Usage};
        use rmcp::model::{RawTextContent, Role, TextContent};

        Ok((
            Message::new(
                Role::Assistant,
                chrono::Utc::now().timestamp(),
                vec![MessageContent::Text(TextContent {
                    raw: RawTextContent {
                        text: "Mock response".to_string(),
                    },
                    annotations: None,
                })],
            ),
            ProviderUsage::new("mock".to_string(), Usage::default()),
        ))
    }

    fn get_model_config(&self) -> goose::model::ModelConfig {
        goose::model::ModelConfig::new_or_fail("mock-model")
    }
}

// Helper function to create a mock provider for testing
fn create_mock_provider() -> Arc<dyn goose::providers::base::Provider> {
    Arc::new(SimpleMockProvider)
}

/// Test that agents start without providers and can have them set later
#[tokio::test]
async fn test_agent_starts_without_provider() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("no_provider_test".to_string());

    // Get agent without provider
    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Verify agent exists but has no provider initially
    // Agents should start without providers per the new design
    let provider_result = agent.provider().await;
    assert!(
        provider_result.is_err(),
        "Agent should not have a provider initially"
    );
}

/// Test that provider can be set after agent creation
#[tokio::test]
async fn test_provider_can_be_set_after_creation() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("set_provider_test".to_string());

    // Get agent without provider
    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Verify no provider initially
    assert!(agent.provider().await.is_err());

    // Create and set a provider
    let provider = create_mock_provider();
    agent.update_provider(provider.clone()).await.unwrap();

    // Verify provider is now set
    let provider_after = agent.provider().await;
    assert!(
        provider_after.is_ok(),
        "Agent should have a provider after setting it"
    );
}

/// Test that provider persists across agent retrievals from cache
#[tokio::test]
async fn test_provider_persists_across_retrievals() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("persist_provider_test".to_string());

    // Get agent and set provider
    let agent1 = manager.get_agent(session.clone()).await.unwrap();
    let provider = create_mock_provider();
    agent1.update_provider(provider.clone()).await.unwrap();

    // Verify provider is set
    assert!(agent1.provider().await.is_ok());

    // Get the same agent again (should be from cache)
    let agent2 = manager.get_agent(session.clone()).await.unwrap();

    // Verify it's the same agent instance
    assert!(
        Arc::ptr_eq(&agent1, &agent2),
        "Should get same agent from cache"
    );

    // Verify provider is still set
    assert!(
        agent2.provider().await.is_ok(),
        "Provider should persist when agent is retrieved from cache"
    );
}

/// Test that provider can be updated multiple times
#[tokio::test]
async fn test_provider_can_be_updated() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("update_provider_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Set first provider
    let provider1 = create_mock_provider();
    agent.update_provider(provider1.clone()).await.unwrap();
    assert!(agent.provider().await.is_ok());

    // Update to a different provider
    let provider2 = create_mock_provider();
    agent.update_provider(provider2.clone()).await.unwrap();

    // Verify provider was updated (we can't easily check it's different, but it shouldn't error)
    assert!(agent.provider().await.is_ok());
}

/// Test that provider is NOT shared between different sessions
#[tokio::test]
async fn test_provider_isolation_between_sessions() {
    let manager = AgentManager::new(Default::default());
    let session1 = session::Identifier::Name("provider_session1".to_string());
    let session2 = session::Identifier::Name("provider_session2".to_string());

    // Get two different agents
    let agent1 = manager.get_agent(session1.clone()).await.unwrap();
    let agent2 = manager.get_agent(session2.clone()).await.unwrap();

    // Verify they are different agents
    assert!(
        !Arc::ptr_eq(&agent1, &agent2),
        "Different sessions should have different agents"
    );

    // Set provider only on agent1
    let provider = create_mock_provider();
    agent1.update_provider(provider.clone()).await.unwrap();

    // Verify agent1 has provider
    assert!(agent1.provider().await.is_ok());

    // Verify agent2 still has no provider
    assert!(
        agent2.provider().await.is_err(),
        "Setting provider on one agent should not affect another"
    );
}

/// Test provider behavior after agent cleanup and recreation
#[tokio::test]
async fn test_provider_after_cleanup_and_recreation() {
    let manager = AgentManager::new(AgentManagerConfig {
        cleanup_interval: Duration::from_secs(300),
        max_idle_duration: Duration::from_millis(100), // Very short for testing
        ..Default::default()
    });

    let session = session::Identifier::Name("cleanup_provider_test".to_string());

    // Create agent and set provider
    let agent1 = manager.get_agent(session.clone()).await.unwrap();
    let provider = create_mock_provider();
    agent1.update_provider(provider.clone()).await.unwrap();
    assert!(agent1.provider().await.is_ok());

    // Wait for idle timeout
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Cleanup idle agents
    let cleaned = manager
        .cleanup_idle(Duration::from_millis(100))
        .await
        .unwrap();
    assert_eq!(cleaned, 1, "Should have cleaned up 1 idle agent");

    // Get agent again - should be a new instance
    let agent2 = manager.get_agent(session.clone()).await.unwrap();

    // Verify it's a different agent instance
    assert!(
        !Arc::ptr_eq(&agent1, &agent2),
        "Should be a new agent after cleanup"
    );

    // Verify new agent has no provider (providers are not persisted)
    assert!(
        agent2.provider().await.is_err(),
        "Newly created agent after cleanup should not have a provider"
    );
}

/// Test concurrent provider operations on the same agent
#[tokio::test]
async fn test_concurrent_provider_operations() {
    use tokio::task;

    let manager = Arc::new(AgentManager::new(Default::default()));
    let session = session::Identifier::Name("concurrent_provider".to_string());

    // Get the agent
    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Spawn multiple tasks trying to set provider concurrently
    let mut handles = vec![];
    for _i in 0..10 {
        let agent_clone = agent.clone();
        handles.push(task::spawn(async move {
            let provider = create_mock_provider();
            // This should not panic or cause race conditions
            agent_clone.update_provider(provider).await
        }));
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(
            result.is_ok(),
            "Concurrent provider updates should not fail"
        );
    }

    // Verify provider is set after all operations
    assert!(agent.provider().await.is_ok());
}

/// Test that multiple agents can have providers set independently
#[tokio::test]
async fn test_multiple_agents_with_providers() {
    let manager = Arc::new(AgentManager::new(Default::default()));

    let mut agents = vec![];

    // Create multiple agents and set providers
    for i in 0..5 {
        let session = session::Identifier::Name(format!("multi_provider_{}", i));
        let agent = manager.get_agent(session).await.unwrap();

        let provider = create_mock_provider();
        agent.update_provider(provider).await.unwrap();

        agents.push(agent);
    }

    // Verify all agents have providers
    for (i, agent) in agents.iter().enumerate() {
        assert!(
            agent.provider().await.is_ok(),
            "Agent {} should have a provider",
            i
        );
    }
}

/// Test provider behavior with touch_session to keep agent alive
#[tokio::test]
async fn test_provider_with_touch_session() {
    let manager = AgentManager::new(AgentManagerConfig {
        cleanup_interval: Duration::from_millis(100),
        max_idle_duration: Duration::from_millis(200),
        ..Default::default()
    });

    let session = session::Identifier::Name("touch_provider_test".to_string());

    // Create agent and set provider
    let agent = manager.get_agent(session.clone()).await.unwrap();
    let provider = create_mock_provider();
    agent.update_provider(provider).await.unwrap();

    // Keep touching the session to prevent cleanup
    for _ in 0..3 {
        tokio::time::sleep(Duration::from_millis(150)).await;
        manager.touch_session(&session).await.unwrap();
    }

    // Wait a bit more
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Agent should still exist (not cleaned up due to touches)
    assert!(manager.has_agent(&session).await);

    // Get agent again - should be same instance
    let agent2 = manager.get_agent(session.clone()).await.unwrap();
    assert!(
        Arc::ptr_eq(&agent, &agent2),
        "Should be same agent after touches"
    );

    // Provider should still be set
    assert!(
        agent2.provider().await.is_ok(),
        "Provider should persist when agent is kept alive with touches"
    );
}

/// Test error handling when setting provider fails
#[tokio::test]
async fn test_provider_error_handling() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("provider_error_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Initially no provider
    assert!(agent.provider().await.is_err());

    // Set a provider successfully
    let provider = create_mock_provider();
    let result = agent.update_provider(provider).await;
    assert!(result.is_ok(), "Setting provider should succeed");

    // Verify provider is set
    assert!(agent.provider().await.is_ok());

    // Note: We can't easily test provider setting failures without modifying
    // the Agent implementation to inject failures, but the structure is here
    // for when such testing is needed
}
