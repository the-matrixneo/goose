use anyhow::Result;
use goose::agents::Agent;
use goose::conversation::Conversation;
use goose::session::{persist_messages, read_messages, read_metadata, ensure_session_dir};
use goose::conversation::message::Message;
use goose::providers::testprovider::TestProvider;
use mcp_core::tool::ToolCall;
use rmcp::model::Content;

#[tokio::test] 
#[ignore] // This test demonstrates the TODO persistence feature - currently fails as feature not fully implemented
async fn test_todo_persistence_in_session() -> Result<()> {
    // Use the proper session directory
    let session_dir = ensure_session_dir()?;
    let session_file = session_dir.join("test_todo_session.jsonl");

    // Simulate a conversation where the user creates a TODO list
    let mut messages = Conversation::new_unvalidated(vec![
        Message::user().with_text("Please help me track these tasks using the todo tool:\n1. Write tests\n2. Update documentation\n3. Review PRs"),
    ]);

    // Mock the assistant's response with a tool call to write the TODO
    let assistant_response = Message::assistant()
        .with_tool_request(
            "todo_write_1",
            Ok(ToolCall::new(
                "platform__todo_write",
                serde_json::json!({
                    "content": "1. Write tests\n2. Update documentation\n3. Review PRs"
                })
            ))
        );
    
    messages.push(assistant_response);

    // Add the tool response
    let tool_response = Message::user()
        .with_tool_response(
            "todo_write_1",
            Ok(vec![Content::text("Updated (48 chars)")])
        );
    
    messages.push(tool_response);

    // Save the session with the TODO content
    persist_messages(&session_file, &messages, None, None).await?;

    // Read back the session metadata
    let metadata = read_metadata(&session_file)?;
    
    // Check if TODO content was persisted in metadata
    // Note: This will initially fail because we haven't implemented the persistence yet
    assert!(metadata.todo_list.is_some(), "TODO list should be saved in session metadata");
    assert_eq!(
        metadata.todo_list.unwrap(),
        "1. Write tests\n2. Update documentation\n3. Review PRs",
        "TODO content should match what was written"
    );

    // Clean up
    let _ = std::fs::remove_file(&session_file);

    Ok(())
}

#[tokio::test]
#[ignore] // This test demonstrates the TODO persistence feature - currently fails as feature not fully implemented  
async fn test_todo_updates_persist_across_session_saves() -> Result<()> {
    // Use the proper session directory
    let session_dir = ensure_session_dir()?;
    let session_file = session_dir.join("test_todo_updates.jsonl");

    // Initial TODO list
    let messages1 = Conversation::new_unvalidated(vec![
        Message::user().with_text("Create a todo: Buy groceries"),
        Message::assistant()
            .with_tool_request(
                "todo_1",
                Ok(ToolCall::new(
                    "platform__todo_write",
                    serde_json::json!({"content": "- Buy groceries"})
                ))
            ),
        Message::user()
            .with_tool_response(
                "todo_1",
                Ok(vec![Content::text("Updated (15 chars)")])
            ),
    ]);

    persist_messages(&session_file, &messages1, None, None).await?;

    // Check first save
    let metadata1 = read_metadata(&session_file)?;
    assert_eq!(metadata1.todo_list, Some("- Buy groceries".to_string()));

    // Update the TODO list
    let mut messages2 = messages1.clone();
    messages2.push(Message::user().with_text("Add to todo: Call dentist"));
    messages2.push(Message::assistant()
        .with_tool_request(
            "todo_2",
            Ok(ToolCall::new(
                "platform__todo_write",
                serde_json::json!({"content": "- Buy groceries\n- Call dentist"})
            ))
        ));
    messages2.push(Message::user()
        .with_tool_response(
            "todo_2",
            Ok(vec![Content::text("Updated (31 chars)")])
        ));

    persist_messages(&session_file, &messages2, None, None).await?;

    // Check second save has updated TODO
    let metadata2 = read_metadata(&session_file)?;
    assert_eq!(metadata2.todo_list, Some("- Buy groceries\n- Call dentist".to_string()));

    // Clean up
    let _ = std::fs::remove_file(&session_file);

    Ok(())
}
