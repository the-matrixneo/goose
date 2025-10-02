use goose::permission::permission_inspector::PermissionInspector;
use goose::tool_inspection::{InspectionAction, InspectionResult};
use goose::conversation::message::ToolRequest;
use rmcp::model::CallToolRequestParam;
use rmcp::object;
use std::collections::HashSet;

// This test targets PermissionInspector::process_inspection_results
// It verifies that:
// - baseline permission inspector results are applied
// - non-permission inspectors (e.g., security) can override by denying
// - non-permission "Allow" does not override a "RequireApproval" baseline
#[test]
fn test_permission_inspector_process_results_with_overrides() {
    // Create a basic PermissionInspector (mode doesn't affect this method)
    let inspector = PermissionInspector::new(
        "smart".to_string(),
        HashSet::new(),
        HashSet::new(),
    );

    // Two tool requests remaining to be decided on
    let remaining_requests = vec![
        ToolRequest {
            id: "r1".to_string(),
            tool_call: Ok(CallToolRequestParam {
                name: "dangerous_tool".into(),
                arguments: Some(object!({"cmd": "rm -rf /"})),
            }),
        },
        ToolRequest {
            id: "r2".to_string(),
            tool_call: Ok(CallToolRequestParam {
                name: "safe_tool".into(),
                arguments: Some(object!({"value": 1})),
            }),
        },
    ];

    // Baseline decisions from the permission inspector
    let mut inspection_results = vec![
        InspectionResult {
            tool_request_id: "r1".to_string(),
            action: InspectionAction::Allow,
            reason: "pre-approved".to_string(),
            confidence: 1.0,
            inspector_name: "permission".to_string(),
            finding_id: None,
        },
        InspectionResult {
            tool_request_id: "r2".to_string(),
            action: InspectionAction::RequireApproval(None),
            reason: "needs approval".to_string(),
            confidence: 1.0,
            inspector_name: "permission".to_string(),
            finding_id: None,
        },
    ];

    // Security inspector flags r1 as dangerous (override to Deny)
    inspection_results.push(InspectionResult {
        tool_request_id: "r1".to_string(),
        action: InspectionAction::Deny,
        reason: "security detection".to_string(),
        confidence: 0.9,
        inspector_name: "security".to_string(),
        finding_id: Some("SEC-001".to_string()),
    });

    // Security inspector "Allow" for r2 should NOT override the RequireApproval baseline
    inspection_results.push(InspectionResult {
        tool_request_id: "r2".to_string(),
        action: InspectionAction::Allow,
        reason: "benign".to_string(),
        confidence: 0.8,
        inspector_name: "security".to_string(),
        finding_id: None,
    });

    // Execute the processing
    let result = inspector.process_inspection_results(&remaining_requests, &inspection_results);

    // r1 should be denied due to security override
    assert!(result.denied.iter().any(|r| r.id == "r1"));
    // r1 should not appear in approved or needs_approval
    assert!(!result.approved.iter().any(|r| r.id == "r1"));
    assert!(!result.needs_approval.iter().any(|r| r.id == "r1"));

    // r2 should still require approval (security Allow does not override baseline)
    assert!(result.needs_approval.iter().any(|r| r.id == "r2"));
    assert!(!result.approved.iter().any(|r| r.id == "r2"));
    assert!(!result.denied.iter().any(|r| r.id == "r2"));
}
