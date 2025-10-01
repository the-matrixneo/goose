use goose::conversation::message::ToolRequest;
use rmcp::model::{CallToolRequestParam, ErrorCode, ErrorData};
use rmcp::object;

// Tests the formatting logic in ToolRequest::to_readable_string for both
// success and error variants in a single comprehensive test.
#[test]
fn test_tool_request_to_readable_string_formats_ok_and_err() {
    // Success case
    let req_ok = ToolRequest {
        id: "abc".to_string(),
        tool_call: Ok(CallToolRequestParam {
            name: "my_tool".into(),
            arguments: Some(object!({
                "a": 1,
                "b": "x"
            })),
        }),
    };

    let s_ok = req_ok.to_readable_string();

    // Should start with the tool name
    assert!(s_ok.starts_with("Tool: my_tool, Args:"), "unexpected prefix: {}", s_ok);

    // The arguments are pretty-printed JSON of the Option<..> value
    // Ensure key/value pairs are present in pretty output
    assert!(s_ok.contains("\"a\": 1"), "missing 'a' key in: {}", s_ok);
    assert!(s_ok.contains("\"b\": \"x\""), "missing 'b' key in: {}", s_ok);

    // Should not contain the placeholder for invalid JSON
    assert!(!s_ok.contains("<<invalid json>>"));

    // Error case
    let req_err = ToolRequest {
        id: "err".to_string(),
        tool_call: Err(ErrorData {
            code: ErrorCode::INTERNAL_ERROR,
            message: std::borrow::Cow::from("Something went wrong".to_string()),
            data: None,
        }),
    };

    let s_err = req_err.to_readable_string();

    assert!(s_err.starts_with("Invalid tool call:"), "unexpected: {}", s_err);
    assert!(s_err.contains("Something went wrong"), "missing error message in: {}", s_err);
}
