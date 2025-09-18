use goose::permission::ToolPermissionStore;
use goose::conversation::message::ToolRequest;
use mcp_core::ToolCall;
use serde_json::json;
use std::thread::sleep;
use std::time::Duration;

// This test targets ToolPermissionStore::check_permission
// It validates that:
// - permissions are keyed by a hash of the tool arguments (context), not just tool name
// - an allowed permission is returned immediately after recording
// - expired permissions are ignored by check_permission
#[test]
fn test_check_permission_context_hash_and_expiry() {
    let mut store = ToolPermissionStore::new();

    // Build a tool request (success variant)
    let tool_call_ok = mcp_core::ToolResult::Ok(ToolCall {
        name: "write_file".to_string(),
        arguments: json!({
            "path": "/tmp/demo.txt",
            "content": "hello"
        }),
    });

    let request_id = "req-1".to_string();
    let tool_request = ToolRequest { id: request_id.clone(), tool_call: tool_call_ok };

    // Before recording any decision, there should be no stored permission
    assert_eq!(store.check_permission(&tool_request), None);

    // Record an allowed decision with a short expiry
    store
        .record_permission(&tool_request, true, Some(Duration::from_secs(1)))
        .expect("record_permission should succeed");

    // Immediately, the permission should be retrievable and allowed
    assert_eq!(store.check_permission(&tool_request), Some(true));

    // Create a similar request but with different arguments to ensure a different context hash
    let tool_call_different_args = mcp_core::ToolResult::Ok(ToolCall {
        name: "write_file".to_string(),
        arguments: json!({
            "path": "/tmp/demo.txt",
            "content": "world" // changed content -> different context hash
        }),
    });
    let different_request = ToolRequest { id: "req-2".to_string(), tool_call: tool_call_different_args };

    // Different arguments mean different context hash; no permission stored for this context
    assert_eq!(store.check_permission(&different_request), None);

    // Wait for the original record to expire (expiry is 1s; sleep slightly longer)
    sleep(Duration::from_millis(1200));

    // After expiry, the original permission should no longer be returned
    assert_eq!(store.check_permission(&tool_request), None);
}
