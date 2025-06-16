use anyhow::Result;
use async_trait::async_trait;
use mcp_core::{
    content::Content,
    handler::{PromptError, ResourceError, ToolError},
    prompt::Prompt,
    protocol::{JsonRpcMessage, ServerCapabilities},
    resource::Resource,
    tool::{Tool, ToolAnnotations, ToolCall},
};
use mcp_server::router::CapabilitiesBuilder;
use mcp_server::Router;
use serde_json::{json, Value};
use std::{
    future::Future, io, pin::Pin
};
use tokio::sync::mpsc;

// Define a custom router
#[derive(Clone)]
pub struct SubRecipeRouter {
    tools: Vec<Tool>,
    instructions: String,
}

impl Default for SubRecipeRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl SubRecipeRouter {
    pub fn new() -> Self {
        // Define a simple tool for demonstration
        let greet_tool = Tool::new(
            "greet",
            "Greets a person with a customized message",
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "greeting": {"type": "string", "default": "Hello"}
                },
                "required": ["name"]
            }),
            Some(ToolAnnotations {
                title: Some("Greeting Tool".to_string()),
                read_only_hint: true,
                destructive_hint: false,
                idempotent_hint: true,
                open_world_hint: false,
            }),
        );

        let echo_tool = Tool::new(
            "echo",
            "Echoes back the input message",
            json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                },
                "required": ["message"]
            }),
            Some(ToolAnnotations {
                title: Some("Echo Tool".to_string()),
                read_only_hint: true,
                destructive_hint: false,
                idempotent_hint: true,
                open_world_hint: false,
            }),
        );

        let instructions = "This is a custom MCP router that provides basic greeting and echo functionality.".to_string();

        Self {
            tools: vec![greet_tool, echo_tool],
            instructions,
        }
    }

    async fn execute_tool_call(&self, tool_call: ToolCall) -> Result<String, ToolError> {
        match tool_call.name.as_str() {
            "greet" => {
                let name = tool_call.arguments["name"]
                    .as_str()
                    .ok_or_else(|| ToolError::ExecutionError("Name must be a string".to_string()))?;
                
                let greeting = tool_call.arguments["greeting"]
                    .as_str()
                    .unwrap_or("Hello");
                
                Ok(format!("{} {}!", greeting, name))
            }
            "echo" => {
                let message = tool_call.arguments["message"]
                    .as_str()
                    .ok_or_else(|| ToolError::ExecutionError("Message must be a string".to_string()))?;
                
                Ok(format!("Echo: {}", message))
            }
            _ => Err(ToolError::ExecutionError("Unknown tool".to_string())),
        }
    }
}

#[async_trait]
impl Router for SubRecipeRouter {
    fn name(&self) -> String {
        "sub-recipe-router".to_string()
    }

    fn instructions(&self) -> String {
        self.instructions.clone()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new().with_tools(false).build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.tools.clone()
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
        _notifier: mpsc::Sender<JsonRpcMessage>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let this = self.clone();
        let tool_name = tool_name.to_string();

        Box::pin(async move {
            let tool_call = ToolCall {
                name: tool_name,
                arguments,
            };
            
            match this.execute_tool_call(tool_call).await {
                Ok(result) => Ok(vec![Content::text(result)]),
                Err(err) => Err(err),
            }
        })
    }

    fn list_resources(&self) -> Vec<Resource> {
        Vec::new()
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async move {
            Err(ResourceError::NotFound("No resources available".to_string()))
        })
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        Vec::new()
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        let prompt_name = prompt_name.to_string();
        Box::pin(async move {
            Err(PromptError::NotFound(format!(
                "Prompt {} not found",
                prompt_name
            )))
        })
    }
}
