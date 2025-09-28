use goose::agents::platform_tools::{
    manage_schedule_tool, PLATFORM_MANAGE_SCHEDULE_TOOL_NAME,
};

#[test]
fn test_manage_schedule_tool_schema_and_annotations() {
    let tool = manage_schedule_tool();

    // Basic identity checks
    assert_eq!(tool.name, PLATFORM_MANAGE_SCHEDULE_TOOL_NAME);
    assert!(tool
        .description
        .as_ref()
        .map(|d| d.contains("Manage scheduled recipe execution"))
        .unwrap_or(false));

    // Schema shape
    let schema = &tool.input_schema;
    assert_eq!(schema["type"].as_str(), Some("object"));

    // Required fields
    let required = schema["required"].as_array().expect("required must be array");
    assert!(required.iter().any(|v| v == "action"));

    // Properties
    let props = &schema["properties"];
    // action property type and enum values
    assert_eq!(props["action"]["type"].as_str(), Some("string"));
    let action_enum = props["action"]["enum"].as_array().expect("enum must be array");
    let expected_variants = vec![
        "list",
        "create",
        "run_now",
        "pause",
        "unpause",
        "delete",
        "kill",
        "inspect",
        "sessions",
        "session_content",
    ];
    for variant in expected_variants {
        assert!(action_enum.iter().any(|v| v.as_str() == Some(variant)),
            "missing enum variant: {}", variant);
    }

    // execution_mode property defaults and enum
    assert_eq!(
        props["execution_mode"]["type"].as_str(),
        Some("string")
    );
    assert_eq!(
        props["execution_mode"]["default"].as_str(),
        Some("background")
    );
    let exec_enum = props["execution_mode"]["enum"].as_array().unwrap();
    assert!(exec_enum.iter().any(|v| v.as_str() == Some("foreground")));
    assert!(exec_enum.iter().any(|v| v.as_str() == Some("background")));

    // Annotations
    let ann = tool.annotations.as_ref().expect("annotations should be set");
    assert_eq!(ann.title.as_deref(), Some("Manage scheduled recipes"));
    assert_eq!(ann.read_only_hint, Some(false));
    assert_eq!(ann.destructive_hint, Some(true));
    assert_eq!(ann.idempotent_hint, Some(false));
    assert_eq!(ann.open_world_hint, Some(false));
}
