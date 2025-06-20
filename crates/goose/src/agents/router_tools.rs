use super::platform_tools::{
    PLATFORM_LIST_RESOURCES_TOOL_NAME, PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME,
    PLATFORM_READ_RESOURCE_TOOL_NAME, PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME,
};
use indoc::indoc;
use mcp_core::tool::{Tool, ToolAnnotations};
use serde_json::json;

pub const ROUTER_VECTOR_SEARCH_TOOL_NAME: &str = "router__vector_search";
pub const ROUTER_VECTOR_SEARCH_WITH_EXTENSION_TOOL_NAME: &str = "router__vector_search_with_extension";
pub const ROUTER_LLM_SEARCH_TOOL_NAME: &str = "router__llm_search";
pub const ROUTER_VECTOR_SEARCH_PASSTHROUGH_TOOL_NAME: &str = "router__vector_search_passthrough";
pub const ROUTER_VECTOR_SEARCH_WITH_EXTENSION_PASSTHROUGH_TOOL_NAME: &str = "router__vector_search_with_extension_passthrough";
pub const ROUTER_LLM_SEARCH_PASSTHROUGH_TOOL_NAME: &str = "router__llm_search_passthrough";

pub fn vector_search_tool_with_extension() -> Tool {
    Tool::new(
        ROUTER_VECTOR_SEARCH_WITH_EXTENSION_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages.
            Format a query to search for the most relevant tools based on the user's messages.
            Pay attention to the keywords in the user's messages, especially the last message and potential tools they are asking for.
            This tool should be invoked when the user's messages suggest they are asking for a tool to be run.
            You have the list of extension names available to you in your system prompt.
            Use the extension_name parameter to filter tools by the appropriate extension.
            For example, if the user is asking to list the files in the current directory, you filter for the "developer" extension.
            Example: {"User": "list the files in the current directory", "Query": "list files in current directory", "Extension Name": "developer", "k": 5}
            Extension name is not optional, it is required.
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["query", "extension_name"],
            "properties": {
                "query": {"type": "string", "description": "The query to search for the most relevant tools based on the user's messages"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5},
                "extension_name": {"type": "string", "description": "Name of the extension to filter tools by"}
            }
        }),
        Some(ToolAnnotations {
            title: Some("Vector search for relevant tools with extension".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn vector_search_tool() -> Tool {
    Tool::new(
        ROUTER_VECTOR_SEARCH_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages.
            Format a query to search for the most relevant tools based on the user's messages.
            Pay attention to the keywords in the user's messages, especially the last message and potential tools they are asking for.
            This tool should be invoked when the user's messages suggest they are asking for a tool to be run.
            This tool searches across all available extensions.
            Example: {"User": "list the files in the current directory", "Query": "list files in current directory", "k": 5}
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["query"],
            "properties": {
                "query": {"type": "string", "description": "The query to search for the most relevant tools based on the user's messages"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5}
            }
        }),
        Some(ToolAnnotations {
            title: Some("Vector search for relevant tools".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn vector_search_tool_prompt() -> String {
    format!(
        r#"# Tool Selection Instructions
    Important: the user has opted to dynamically enable tools, so although an extension could be enabled, \
    please invoke the vector search tool to actually retrieve the most relevant tools to use according to the user's messages.
    For example, if the user has 3 extensions enabled, but they are asking for a tool to read a pdf file, \
    you would invoke the vector_search tool to find the most relevant read pdf tool.
    By dynamically enabling tools, you (Goose) as the agent save context window space and allow the user to dynamically retrieve the most relevant tools.
    Be sure to format the query to search rather than pass in the user's messages directly.
    
    You have two vector search tools available:
    - router__vector_search: Searches across all available extensions without filtering
    - router__vector_search_with_extension: Searches within a specific extension (requires extension_name parameter)"#.to_string()
}

pub fn vector_search_tool_with_extension_prompt() -> String {
    r#"# Tool Selection Instructions (Extension-Specific)
    Important: the user has opted to dynamically enable tools, so although an extension could be enabled, \
    please invoke the vector search tool with extension to actually retrieve the most relevant tools to use according to the user's messages from a specific extension.
    You have the list of extension names available to you in your system prompt.
    Use the extension_name parameter to filter tools by the appropriate extension.
    For example, if the user is asking to list the files in the current directory, you filter for the "developer" extension.
    By dynamically enabling tools, you (Goose) as the agent save context window space and allow the user to dynamically retrieve the most relevant tools.
    Be sure to format the query to search rather than pass in the user's messages directly.
    In addition to the extension names available to you, you also have platform extension tools available to you.
    The platform extension contains the following tools:
    - {}
    - {}
    - {}
    - {}
    "#,
        PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME,
        PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME,
        PLATFORM_READ_RESOURCE_TOOL_NAME,
        PLATFORM_LIST_RESOURCES_TOOL_NAME
    )
}

pub fn llm_search_tool() -> Tool {
    Tool::new(
        ROUTER_LLM_SEARCH_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages.
            Format a query to search for the most relevant tools based on the user's messages.
            Pay attention to the keywords in the user's messages, especially the last message and potential tools they are asking for.
            This tool should be invoked when the user's messages suggest they are asking for a tool to be run.
            Use the extension_name parameter to filter tools by the appropriate extension.
            For example, if the user is asking to list the files in the current directory, you filter for the "developer" extension.
            Example: {"User": "list the files in the current directory", "Query": "list files in current directory", "Extension Name": "developer", "k": 5}
            Extension name is not optional, it is required.
            The returned result will be a list of tool names, descriptions, and schemas from which you, the agent can select the most relevant tool to invoke.
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["query", "extension_name"],
            "properties": {
                "extension_name": {"type": "string", "description": "The name of the extension to filter tools by"},
                "query": {"type": "string", "description": "The query to search for the most relevant tools based on the user's messages"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5}
            }
        }),
        Some(ToolAnnotations {
            title: Some("LLM search for relevant tools".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn llm_search_tool_prompt() -> String {
    format!(
        r#"# LLM Tool Selection Instructions
    Important: the user has opted to dynamically enable tools, so although an extension could be enabled, \
    please invoke the llm search tool to actually retrieve the most relevant tools to use according to the user's messages.
    For example, if the user has 3 extensions enabled, but they are asking for a tool to read a pdf file, \
    you would invoke the llm_search tool to find the most relevant read pdf tool.
    By dynamically enabling tools, you (Goose) as the agent save context window space and allow the user to dynamically retrieve the most relevant tools.
    Be sure to format a query packed with relevant keywords to search for the most relevant tools.
    In addition to the extension names available to you, you also have platform extension tools available to you.
    The platform extension contains the following tools:
    - {}
    - {}
    - {}
    - {}
    "#,
        PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME,
        PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME,
        PLATFORM_READ_RESOURCE_TOOL_NAME,
        PLATFORM_LIST_RESOURCES_TOOL_NAME
    )
}

pub fn vector_search_passthrough_tool() -> Tool {
    Tool::new(
        ROUTER_VECTOR_SEARCH_PASSTHROUGH_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages with passthrough capability.
            This tool takes the user's original query and AI-generated additional context.
            The user_query contains the user's actual message, and additional_context contains AI-generated context.
            This tool searches across all available extensions.
            Example: {"user_query": "list the files in the current directory", "additional_context": "file system navigation", "k": 5}
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["user_query", "additional_context"],
            "properties": {
                "user_query": {"type": "string", "description": "The user's original query/message (passthrough)"},
                "additional_context": {"type": "string", "description": "AI-generated additional context for the search"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5}
            }
        }),
        Some(ToolAnnotations {
            title: Some("Vector search with user message passthrough".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn vector_search_passthrough_tool_prompt() -> String {
    r#"# Tool Selection Instructions (User Message Passthrough)
    Important: the user has opted to dynamically enable tools with passthrough capability.
    This means the user's last message will be included in the search query for better context.
    Please invoke the vector search passthrough tool to retrieve the most relevant tools.
    Be sure to format the query and provide additional_query_context that complements the user's message.
    The system will automatically combine the user's last message with your additional_query_context."#.to_string()
}

pub fn vector_search_with_extension_passthrough_tool() -> Tool {
    Tool::new(
        ROUTER_VECTOR_SEARCH_WITH_EXTENSION_PASSTHROUGH_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages with passthrough capability and extension filtering.
            This tool takes the user's original query and AI-generated additional context.
            The user_query contains the user's actual message, and additional_context contains AI-generated context.
            You have the list of extension names available to you in your system prompt.
            Use the extension_name parameter to filter tools by the appropriate extension.
            Example: {"user_query": "list the files in the current directory", "additional_context": "file system navigation", "extension_name": "developer", "k": 5}
            Extension name is not optional, it is required.
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["user_query", "additional_context", "extension_name"],
            "properties": {
                "user_query": {"type": "string", "description": "The user's original query/message (passthrough)"},
                "additional_context": {"type": "string", "description": "AI-generated additional context for the search"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5},
                "extension_name": {"type": "string", "description": "Name of the extension to filter tools by"}
            }
        }),
        Some(ToolAnnotations {
            title: Some("Vector search with user message passthrough and extension filter".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn vector_search_with_extension_passthrough_tool_prompt() -> String {
    r#"# Tool Selection Instructions (User Message Passthrough with Extension)
    Important: the user has opted to dynamically enable tools with passthrough capability and extension filtering.
    This means the user's last message will be included in the search query for better context.
    You have the list of extension names available to you in your system prompt.
    Use the extension_name parameter to filter tools by the appropriate extension.
    Be sure to format the query and provide additional_query_context that complements the user's message.
    The system will automatically combine the user's last message with your additional_query_context."#.to_string()
}

pub fn llm_search_passthrough_tool() -> Tool {
    Tool::new(
        ROUTER_LLM_SEARCH_PASSTHROUGH_TOOL_NAME.to_string(),
        indoc! {r#"
            Searches for relevant tools based on the user's messages using LLM with passthrough capability.
            This tool takes the user's original query and AI-generated additional context.
            The user_query contains the user's actual message, and additional_context contains AI-generated context.
            Use the extension_name parameter to filter tools by the appropriate extension.
            Extension name is not optional, it is required.
            The returned result will be a list of tool names, descriptions, and schemas from which you, the agent can select the most relevant tool to invoke.
        "#}
        .to_string(),
        json!({
            "type": "object",
            "required": ["user_query", "additional_context", "extension_name"],
            "properties": {
                "extension_name": {"type": "string", "description": "The name of the extension to filter tools by"},
                "user_query": {"type": "string", "description": "The user's original query/message (passthrough)"},
                "additional_context": {"type": "string", "description": "AI-generated additional context for the search"},
                "k": {"type": "integer", "description": "The number of tools to retrieve (defaults to 5)", "default": 5}
            }
        }),
        Some(ToolAnnotations {
            title: Some("LLM search with user message passthrough".to_string()),
            read_only_hint: true,
            destructive_hint: false,
            idempotent_hint: false,
            open_world_hint: false,
        }),
    )
}

pub fn llm_search_passthrough_tool_prompt() -> String {
    format!(
        r#"# LLM Tool Selection Instructions (User Message Passthrough)
    Important: the user has opted to dynamically enable tools with passthrough capability using LLM search.
    This means the user's last message will be included in the search query for better context.
    Please invoke the llm search passthrough tool to retrieve the most relevant tools.
    Be sure to format a query packed with relevant keywords and provide additional_query_context.
    The system will automatically combine the user's last message with your additional_query_context.
    In addition to the extension names available to you, you also have platform extension tools available to you.
    The platform extension contains the following tools:
    - {}
    - {}
    - {}
    - {}
    "#,
        PLATFORM_SEARCH_AVAILABLE_EXTENSIONS_TOOL_NAME,
        PLATFORM_MANAGE_EXTENSIONS_TOOL_NAME,
        PLATFORM_READ_RESOURCE_TOOL_NAME,
        PLATFORM_LIST_RESOURCES_TOOL_NAME
    )
}
