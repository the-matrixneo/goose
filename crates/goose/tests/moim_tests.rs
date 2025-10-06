use goose::agents::extension::ExtensionConfig;
use goose::agents::Agent;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_moim_content_includes_timestamp() {
    let agent = Agent::new();

    // Collect MOIM content
    let moim_content = agent.extension_manager.collect_moim().await;

    assert!(moim_content.is_some());
    let content = moim_content.unwrap();

    // Should always include timestamp
    assert!(content.contains("Current date and time:"));
}

#[tokio::test]
#[serial]
async fn test_moim_with_todo_content() {
    let agent = Agent::new();

    // Add TODO extension with content
    let todo_config = ExtensionConfig::Platform {
        name: "todo".to_string(),
        description: "Task management".to_string(),
        bundled: None,
        available_tools: vec![],
    };

    agent.add_extension(todo_config).await.unwrap();

    // The TODO extension should now be available
    let moim_content = agent.extension_manager.collect_moim().await;
    assert!(moim_content.is_some());

    let content = moim_content.unwrap();
    // Should include timestamp
    assert!(content.contains("Current date and time:"));
}
