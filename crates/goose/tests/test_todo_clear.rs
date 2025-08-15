use goose::agents::Agent;
use mcp_core::tool::ToolCall;
use serde_json::json;

#[tokio::test]
async fn test_clear_todo_list() {
    let agent = Agent::new();

    // Write to the todo list
    let write_call = ToolCall {
        name: "todo__write".to_string(),
        arguments: json!({
            "content": "Task 1\nTask 2\nTask 3"
        }),
    };

    let (_id, result) = agent
        .dispatch_tool_call(write_call, "test-1".to_string(), None)
        .await;
    assert!(result.is_ok());

    // Verify content is there
    let read_call = ToolCall {
        name: "todo__read".to_string(),
        arguments: json!({}),
    };

    let (_id, result) = agent
        .dispatch_tool_call(read_call.clone(), "test-2".to_string(), None)
        .await;

    if let Ok(tool_result) = result {
        let content = tool_result.result.await;
        if let Ok(contents) = content {
            assert_eq!(
                contents[0].as_text().unwrap().text.as_str(),
                "Task 1\nTask 2\nTask 3"
            );
        }
    }

    // Clear the todo list
    agent.clear_todo_list().await;

    // Verify it's empty now
    let (_id, result) = agent
        .dispatch_tool_call(read_call, "test-3".to_string(), None)
        .await;

    if let Ok(tool_result) = result {
        let content = tool_result.result.await;
        if let Ok(contents) = content {
            assert_eq!(contents[0].as_text().unwrap().text.as_str(), "");
        }
    }
}
