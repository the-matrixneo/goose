use anyhow::Result;
use mcp_core::{
    handler::{PromptError, ResourceError, ToolError},
    protocol::{JsonRpcMessage, ServerCapabilities},
    prompt::Prompt,
    resource::Resource,
    role::Role,
    tool::{Tool, ToolAnnotations},
    Content,
};
use mcp_server::{router::CapabilitiesBuilder, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::{sync::mpsc};

// Define the ThoughtData structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThoughtData {
    thought: String,
    thought_number: u32,
    total_thoughts: u32,
    next_thought_needed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revises_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_from_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    needs_more_thoughts: Option<bool>,
}

// Server state
struct SequentialThinkingState {
    thought_history: Vec<ThoughtData>,
    branches: HashMap<String, Vec<ThoughtData>>,
}

// Sequential thinking router
pub struct MultiTaskRouter {
    tools: Vec<Tool>,
    state: Arc<Mutex<SequentialThinkingState>>,
}

impl Default for MultiTaskRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiTaskRouter {
    pub fn new() -> Self {
        // Create the sequential thinking tool
        let sequential_thinking_tool = Tool::new(
            "multitask".to_string(),
            r#"A detailed tool for dynamic and reflective problem-solving through thoughts.
This tool helps analyze problems through a flexible thinking process that can adapt and evolve.
Each thought can build on, question, or revise previous insights as understanding deepens.

When to use this tool:
- Breaking down complex problems into steps
- Planning and design with room for revision
- Analysis that might need course correction
- Problems where the full scope might not be clear initially
- Problems that require a multi-step solution
- Tasks that need to maintain context over multiple steps
- Situations where irrelevant information needs to be filtered out

Key features:
- You can adjust total_thoughts up or down as you progress
- You can question or revise previous thoughts
- You can add more thoughts even after reaching what seemed like the end
- You can express uncertainty and explore alternative approaches
- Not every thought needs to build linearly - you can branch or backtrack
- Generates a solution hypothesis
- Verifies the hypothesis based on the Chain of Thought steps
- Repeats the process until satisfied
- Provides a correct answer"#.to_string(),
            json!({
                "type": "object",
                "properties": {
                    "thought": {
                        "type": "string",
                        "description": "Your current thinking step"
                    },
                    "next_thought_needed": {
                        "type": "boolean",
                        "description": "Whether another thought step is needed"
                    },
                    "thought_number": {
                        "type": "integer",
                        "description": "Current thought number",
                        "minimum": 1
                    },
                    "total_thoughts": {
                        "type": "integer",
                        "description": "Estimated total thoughts needed",
                        "minimum": 1
                    },
                    "is_revision": {
                        "type": "boolean",
                        "description": "Whether this revises previous thinking"
                    },
                    "revises_thought": {
                        "type": "integer",
                        "description": "Which thought is being reconsidered",
                        "minimum": 1
                    },
                    "branch_from_thought": {
                        "type": "integer",
                        "description": "Branching point thought number",
                        "minimum": 1
                    },
                    "branch_id": {
                        "type": "string",
                        "description": "Branch identifier"
                    },
                    "needs_more_thoughts": {
                        "type": "boolean",
                        "description": "If more thoughts are needed"
                    }
                },
                "required": ["thought", "next_thought_needed", "thought_number", "total_thoughts"]
            }),
            Some(ToolAnnotations {
                title: Some("Sequential Thinking".to_string()),
                read_only_hint: false,
                destructive_hint: false,
                idempotent_hint: false,
                open_world_hint: false,
            }),
        );

        Self {
            tools: vec![sequential_thinking_tool],
            state: Arc::new(Mutex::new(SequentialThinkingState {
                thought_history: Vec::new(),
                branches: HashMap::new(),
            })),
        }
    }

    // Helper function to validate thought data
    fn validate_thought_data(&self, params: Value) -> Result<ThoughtData, ToolError> {
        // Parse the JSON data into ThoughtData
        let thought_data: ThoughtData = serde_json::from_value(params)
            .map_err(|e| ToolError::InvalidParameters(format!("Invalid thought data: {}", e)))?;

        // Validate required fields
        if thought_data.thought.is_empty() {
            return Err(ToolError::InvalidParameters(
                "Invalid thought: must be a non-empty string".into(),
            ));
        }

        if thought_data.thought_number == 0 {
            return Err(ToolError::InvalidParameters(
                "Invalid thoughtNumber: must be a positive number".into(),
            ));
        }

        if thought_data.total_thoughts == 0 {
            return Err(ToolError::InvalidParameters(
                "Invalid totalThoughts: must be a positive number".into(),
            ));
        }

        Ok(thought_data)
    }

    // Format thought for display
    fn format_thought(&self, thought_data: &ThoughtData) -> String {
        let (prefix, context) = if thought_data.is_revision.unwrap_or(false) {
            (
                "🔄 Revision",
                format!(
                    " (revising thought {})",
                    thought_data.revises_thought.unwrap_or(0)
                ),
            )
        } else if thought_data.branch_from_thought.is_some() {
            (
                "🌿 Branch",
                format!(
                    " (from thought {}, ID: {})",
                    thought_data.branch_from_thought.unwrap_or(0),
                    thought_data.branch_id.as_deref().unwrap_or("unknown")
                ),
            )
        } else {
            ("💭 Thought", String::new())
        };

        let header = format!(
            "{} {}/{}{}",
            prefix, thought_data.thought_number, thought_data.total_thoughts, context
        );
        
        let thought_len = thought_data.thought.len();
        let border_len = std::cmp::max(100, thought_len) + 4;
        let border = "─".repeat(border_len);

        format!(
            "\n┌{}┐\n│ {} │\n├{}┤\n│ {} │\n└{}┘",
            border,
            header,
            border,
            format!("{:<width$}", thought_data.thought, width = border_len - 2),
            border
        )
    }

    async fn sequential_thinking(&self, params: Value) -> Result<Vec<Content>, ToolError> {
        // Validate and parse the thought data
        let mut thought_data = self.validate_thought_data(params)?;
        
        // Adjust total thoughts if needed
        if thought_data.thought_number > thought_data.total_thoughts {
            thought_data.total_thoughts = thought_data.thought_number;
        }
        
        // Format and print the thought
        let formatted_thought = self.format_thought(&thought_data);
        eprintln!("{}", formatted_thought);
        
        // Store the thought in history
        let mut state = self.state.lock().unwrap();
        
        state.thought_history.push(thought_data.clone());
        
        // Handle branch storage
        if let (Some(_branch_from), Some(branch_id)) = (thought_data.branch_from_thought, thought_data.branch_id.clone()) {
            state.branches.entry(branch_id.clone())
                .or_insert_with(Vec::new)
                .push(thought_data.clone());
        }
        
        // Prepare response
        let response = json!({
            "thought_number": thought_data.thought_number,
            "total_thoughts": thought_data.total_thoughts,
            "next_thought_needed": thought_data.next_thought_needed,
            "branches": state.branches.keys().collect::<Vec<_>>(),
            "thought_history_length": state.thought_history.len()
        });
        
        // Return the response
        Ok(vec![Content::text(serde_json::to_string_pretty(&response).unwrap())
            .with_audience(vec![Role::Assistant])])
    }
}

impl Router for MultiTaskRouter {
    fn name(&self) -> String {
        "multi-task-router".to_string()
    }

    fn instructions(&self) -> String {
        "Multi-task MCP for step-by-step problem solving".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new()
            .with_tools(true)
            .with_prompts(false)
            .build()
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
            match tool_name.as_str() {
                "multitask" => this.sequential_thinking(arguments).await,
                _ => Err(ToolError::NotFound(format!("Tool {} not found", tool_name))),
            }
        })
    }
    
    // Implement the required resource-related methods
    fn list_resources(&self) -> Vec<Resource> {
        Vec::new() // No resources for this MCP
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async move {
            Err(ResourceError::NotFound("No resources available".to_string()))
        })
    }

    // Implement the required prompt-related methods
    fn list_prompts(&self) -> Vec<Prompt> {
        Vec::new() // No prompts for this MCP
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        let prompt_name = prompt_name.to_string(); // Clone the string to own it
        Box::pin(async move {
            Err(PromptError::NotFound(format!("Prompt '{}' not found", prompt_name)))
        })
    }
}

impl Clone for MultiTaskRouter {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
            state: Arc::clone(&self.state),
        }
    }
}