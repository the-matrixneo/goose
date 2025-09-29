use goose::conversation::message::ToolRequest;
use mcp_core::ToolCall;
use serde_json::json;

// This test targets: ToolRequest::to_readable_string
// It verifies that a successful tool call renders a readable string with
// pretty-printed JSON arguments and includes the tool name.
#[test]
fn test_tool_request_to_readable_string_pretty_json() {
    let tool_call = ToolCall::new(
        "fetch_user",
        json!({
            "id": 42,
            "active": true,
            "details": {"level": 3}
        }),
    );

    let req = ToolRequest {
        id: "req-1".to_string(),
        tool_call: Ok(tool_call),
    };

    let readable = req.to_readable_string();

    // Starts with the expected tool prefix
    assert!(readable.starts_with("Tool: fetch_user, Args: {"));

    // Pretty JSON should contain newlines and formatted key/value pairs
    assert!(readable.contains('\n'));
    assert!(readable.contains("\"id\": 42"));
    assert!(readable.contains("\"active\": true"));
    assert!(readable.contains("\"details\": {"));
}
