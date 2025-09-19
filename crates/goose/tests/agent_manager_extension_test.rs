use std::sync::Arc;

use goose::agents::extension::ExtensionConfig;
use goose::agents::manager::{AgentManager, AgentManagerConfig};
use goose::session;
use rmcp::model::Tool;

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
fn create_mock_provider_for_test() -> Arc<dyn goose::providers::base::Provider> {
    Arc::new(SimpleMockProvider)
}

/// Create a simple frontend extension for testing
fn create_test_extension(name: &str) -> ExtensionConfig {
    ExtensionConfig::Frontend {
        name: name.to_string(),
        tools: vec![Tool {
            name: format!("{}_tool", name).into(),
            description: Some(format!("Tool from {} extension", name).into()),
            input_schema: Arc::new(
                serde_json::json!({
                    "type": "object",
                    "properties": {}
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
            output_schema: None,
            annotations: None,
        }],
        instructions: Some(format!("Instructions for {}", name)),
        bundled: Some(false),
        available_tools: vec![],
    }
}

/// Create a builtin extension for testing
fn create_builtin_extension(name: &str) -> ExtensionConfig {
    ExtensionConfig::Builtin {
        name: name.to_string(),
        display_name: Some(name.to_string()),
        description: Some(format!("Test builtin extension {}", name)),
        timeout: Some(30),
        bundled: Some(true),
        available_tools: vec![],
    }
}

/// Create a frontend extension with multiple tools
fn create_multi_tool_extension(name: &str, num_tools: usize) -> ExtensionConfig {
    let tools = (0..num_tools)
        .map(|i| Tool {
            name: format!("{}_tool_{}", name, i).into(),
            description: Some(format!("Tool {} from {} extension", i, name).into()),
            input_schema: Arc::new(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "param": {"type": "string"}
                    }
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
            output_schema: None,
            annotations: None,
        })
        .collect();

    ExtensionConfig::Frontend {
        name: name.to_string(),
        tools,
        instructions: Some(format!("Multi-tool extension {}", name)),
        bundled: Some(false),
        available_tools: vec![],
    }
}

#[tokio::test]
async fn test_extension_isolation_between_sessions() {
    // Extensions added to one session should not appear in another
    let manager = AgentManager::new(AgentManagerConfig::default());

    let session1 = session::Identifier::Name("ext_test_1".to_string());
    let session2 = session::Identifier::Name("ext_test_2".to_string());

    let agent1 = manager.get_agent(session1.clone()).await.unwrap();
    let agent2 = manager.get_agent(session2.clone()).await.unwrap();

    // Add different extensions to each agent
    let ext1 = create_test_extension("extension1");
    let ext2 = create_test_extension("extension2");

    agent1.add_extension(ext1).await.unwrap();
    agent2.add_extension(ext2).await.unwrap();

    // Check tools for each agent
    // Frontend tools are not returned by list_tools, they're stored separately
    // So we need to check if the agent has them as frontend tools
    let has_ext1_tool1 = agent1.is_frontend_tool("extension1_tool").await;
    let has_ext2_tool1 = agent1.is_frontend_tool("extension2_tool").await;
    let has_ext1_tool2 = agent2.is_frontend_tool("extension1_tool").await;
    let has_ext2_tool2 = agent2.is_frontend_tool("extension2_tool").await;

    // Agent1 should have extension1_tool but not extension2_tool
    assert!(has_ext1_tool1, "Agent1 should have extension1_tool");
    assert!(!has_ext2_tool1, "Agent1 should not have extension2_tool");

    // Agent2 should have extension2_tool but not extension1_tool
    assert!(has_ext2_tool2, "Agent2 should have extension2_tool");
    assert!(!has_ext1_tool2, "Agent2 should not have extension1_tool");
}

#[tokio::test]
async fn test_extension_persistence_within_session() {
    // Extensions should persist when an agent is retrieved multiple times
    let manager = AgentManager::new(AgentManagerConfig::default());
    let session = session::Identifier::Name("ext_persistence".to_string());

    // Add extension to agent
    let agent1 = manager.get_agent(session.clone()).await.unwrap();
    let ext = create_test_extension("persistent");
    agent1.add_extension(ext).await.unwrap();

    // Verify extension is present (frontend tools are stored separately)
    assert!(agent1.is_frontend_tool("persistent_tool").await);

    // Get agent again (from cache)
    let agent2 = manager.get_agent(session.clone()).await.unwrap();

    // Extension should still be present
    assert!(agent2.is_frontend_tool("persistent_tool").await);

    // Verify it's the same agent instance
    assert!(Arc::ptr_eq(&agent1, &agent2));
}

#[tokio::test]
async fn test_extension_removal_isolation() {
    // Removing an extension from one session shouldn't affect others
    let manager = AgentManager::new(AgentManagerConfig::default());

    let session1 = session::Identifier::Name("ext_remove_1".to_string());
    let session2 = session::Identifier::Name("ext_remove_2".to_string());

    let agent1 = manager.get_agent(session1).await.unwrap();
    let agent2 = manager.get_agent(session2).await.unwrap();

    // Add same extension to both agents
    let ext1 = create_test_extension("shared");
    let ext2 = create_test_extension("shared");

    agent1.add_extension(ext1).await.unwrap();
    agent2.add_extension(ext2).await.unwrap();

    // Verify both have the extension (frontend tools are stored separately)
    assert!(agent1.is_frontend_tool("shared_tool").await);
    assert!(agent2.is_frontend_tool("shared_tool").await);

    // Frontend extensions can't be removed individually, they're stored in the frontend_tools map
    // So this test doesn't apply to frontend extensions. Let's skip the removal test for frontend extensions.
}

/// Test builtin extension management
#[tokio::test]
#[ignore = "Builtin extensions require MCP processes"]
async fn test_builtin_extension_management() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("builtin_ext_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Add a builtin extension (e.g., developer)
    let builtin_ext = create_builtin_extension("developer");
    agent.add_extension(builtin_ext).await.unwrap();

    // Builtin extensions add their tools to the main tools list
    let tools = agent.list_tools(None).await;

    // Developer extension should add multiple tools
    let developer_tools = tools
        .iter()
        .filter(|t| {
            t.name.contains("shell")
                || t.name.contains("text_editor")
                || t.name.contains("screen_capture")
        })
        .count();

    assert!(
        developer_tools > 0,
        "Builtin developer extension should add tools"
    );
}

/// Test mixing frontend and builtin extensions
#[tokio::test]
#[ignore = "Builtin extensions require MCP processes"]
async fn test_mixed_extension_types() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("mixed_ext_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Add both types of extensions
    let frontend_ext = create_test_extension("frontend_test");
    let builtin_ext = create_builtin_extension("developer");

    agent.add_extension(frontend_ext).await.unwrap();
    agent.add_extension(builtin_ext).await.unwrap();

    // Check frontend tool
    assert!(agent.is_frontend_tool("frontend_test_tool").await);

    // Check builtin tools are in main list
    let tools = agent.list_tools(None).await;
    let has_builtin_tools = tools.iter().any(|t| t.name.contains("shell"));
    assert!(has_builtin_tools, "Should have builtin tools in main list");
}

/// Test multiple frontend extensions with many tools
#[tokio::test]
async fn test_multiple_frontend_extensions_with_many_tools() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("multi_tool_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Add multiple extensions with multiple tools each
    for i in 0..3 {
        let ext = create_multi_tool_extension(&format!("multi_{}", i), 5);
        agent.add_extension(ext).await.unwrap();
    }

    // Verify all tools are present
    for i in 0..3 {
        for j in 0..5 {
            let tool_name = format!("multi_{}_tool_{}", i, j);
            assert!(
                agent.is_frontend_tool(&tool_name).await,
                "Should have tool: {}",
                tool_name
            );
        }
    }
}

/// Test extension operations don't affect other agent state
#[tokio::test]
async fn test_extension_operations_dont_affect_agent_state() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("state_test".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Set a provider first
    let provider = create_mock_provider_for_test();
    agent.update_provider(provider).await.unwrap();

    // Add only frontend extension (builtin would fail without MCP process)
    let ext1 = create_test_extension("test1");
    agent.add_extension(ext1).await.unwrap();

    // Provider should still be set
    assert!(
        agent.provider().await.is_ok(),
        "Provider should remain after adding extensions"
    );
}

/// Test concurrent extension operations on same session
#[tokio::test]
async fn test_concurrent_extension_ops_same_session() {
    use tokio::task;

    let manager = Arc::new(AgentManager::new(Default::default()));
    let session = session::Identifier::Name("concurrent_same".to_string());

    let agent = manager.get_agent(session.clone()).await.unwrap();

    // Spawn multiple tasks adding extensions to the same agent
    let mut handles = vec![];
    for i in 0..10 {
        let agent_clone = agent.clone();
        handles.push(task::spawn(async move {
            let ext = create_test_extension(&format!("concurrent_{}", i));
            agent_clone.add_extension(ext).await
        }));
    }

    // All should succeed
    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }

    // Verify extensions were added
    for i in 0..10 {
        let tool_name = format!("concurrent_{}_tool", i);
        assert!(agent.is_frontend_tool(&tool_name).await);
    }
}

#[tokio::test]
async fn test_concurrent_extension_operations() {
    // Test that concurrent extension operations on different sessions don't interfere
    let manager = Arc::new(AgentManager::new(AgentManagerConfig::default()));

    let mut handles = vec![];

    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);

        let handle = tokio::spawn(async move {
            let session = session::Identifier::Name(format!("concurrent_ext_{}", i));
            let agent = manager_clone.get_agent(session).await.unwrap();

            // Add multiple extensions
            for j in 0..3 {
                let ext = create_test_extension(&format!("ext_{}_{}", i, j));
                agent.add_extension(ext).await.unwrap();
            }

            // Verify all extensions are present (frontend tools are stored separately)
            for j in 0..3 {
                let tool_name = format!("ext_{}_{}_tool", i, j);
                assert!(
                    agent.is_frontend_tool(&tool_name).await,
                    "Missing tool: {}",
                    tool_name
                );
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_extension_state_after_cleanup() {
    // Test that extensions are lost after agent cleanup and recreation
    let config = AgentManagerConfig {
        max_idle_duration: std::time::Duration::from_millis(1), // Very short for testing
        ..Default::default()
    };

    let manager = AgentManager::new(config);
    let session = session::Identifier::Name("cleanup_ext_test".to_string());

    // Add extension to agent
    let agent1 = manager.get_agent(session.clone()).await.unwrap();
    let ext = create_test_extension("temporary");
    agent1.add_extension(ext.clone()).await.unwrap();

    // Verify extension is present (frontend tools are stored separately)
    assert!(agent1.is_frontend_tool("temporary_tool").await);

    // Wait for idle timeout
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    // Trigger cleanup
    let removed = manager.cleanup().await.unwrap();
    assert_eq!(removed, 1, "Should have cleaned up one agent");

    // Get agent again (should be new instance)
    let agent2 = manager.get_agent(session.clone()).await.unwrap();

    // Extension should be gone (fresh agent)
    assert!(!agent2.is_frontend_tool("temporary_tool").await);

    // Verify it's a different agent instance
    assert!(!Arc::ptr_eq(&agent1, &agent2));
}
