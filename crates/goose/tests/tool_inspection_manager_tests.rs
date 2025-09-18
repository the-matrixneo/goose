use goose::tool_inspection::{InspectionAction, InspectionResult, ToolInspectionManager, ToolInspector};
use goose::conversation::message::{Message, ToolRequest};
use anyhow::Result;
use async_trait::async_trait;

// A mock inspector that can be configured to be enabled/disabled and succeed/fail
struct MockInspector {
    name: &'static str,
    enabled: bool,
    results: Option<Vec<InspectionResult>>, // None => return Err
}

#[async_trait]
impl ToolInspector for MockInspector {
    fn name(&self) -> &'static str { self.name }
    fn is_enabled(&self) -> bool { self.enabled }
    fn as_any(&self) -> &dyn std::any::Any { self }

    async fn inspect(&self, _tool_requests: &[ToolRequest], _messages: &[Message]) -> Result<Vec<InspectionResult>> {
        match &self.results {
            Some(v) => Ok(v.clone()),
            None => Err(anyhow::anyhow!("forced failure")),
        }
    }
}

#[tokio::test]
async fn inspect_tools_aggregates_results_and_skips_disabled_and_errors() {
    // Prepare a couple of fake inspection results
    let res1 = InspectionResult {
        tool_request_id: "req-1".to_string(),
        action: InspectionAction::Allow,
        reason: "looks safe".to_string(),
        confidence: 0.9,
        inspector_name: "ok1".to_string(),
        finding_id: None,
    };
    let res2 = InspectionResult {
        tool_request_id: "req-2".to_string(),
        action: InspectionAction::RequireApproval(Some("double check".to_string())),
        reason: "needs review".to_string(),
        confidence: 0.7,
        inspector_name: "ok2".to_string(),
        finding_id: Some("F-42".to_string()),
    };

    // Build manager with 3 inspectors:
    // - one enabled and successful (returns res1)
    // - one enabled but failing (returns error) -> should be skipped and not panic
    // - one disabled but successful (returns res2) -> should be ignored due to is_enabled=false
    let mut mgr = ToolInspectionManager::new();
    mgr.add_inspector(Box::new(MockInspector { name: "ok1", enabled: true, results: Some(vec![res1.clone()]) }));
    mgr.add_inspector(Box::new(MockInspector { name: "fail", enabled: true, results: None }));
    mgr.add_inspector(Box::new(MockInspector { name: "ok2", enabled: false, results: Some(vec![res2.clone()]) }));

    // Run with empty inputs (mock ignores them)
    let out = mgr.inspect_tools(&[], &[]).await.expect("manager should succeed");

    // Only the enabled+successful inspector contributes results
    assert_eq!(out.len(), 1, "only one inspector should contribute results");
    assert_eq!(out[0].tool_request_id, res1.tool_request_id);
    assert_eq!(out[0].inspector_name, res1.inspector_name);
    assert_eq!(out[0].action, res1.action);
}
