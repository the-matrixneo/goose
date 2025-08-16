use rmcp::model::Tool;
use rmcp::model::{Content, ErrorCode, ErrorData};

use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::router_tools::ROUTER_LLM_SEARCH_TOOL_NAME;
use crate::conversation::message::Message;
use crate::prompt_template::render_global_file;
use crate::providers::base::Provider;

#[derive(Serialize)]
struct ToolSelectorContext {
    tools: String,
    query: String,
}

#[async_trait]
pub trait RouterToolSelector: Send + Sync {
    async fn select_tools(&self, params: Value) -> Result<Vec<Content>, ErrorData>;
    async fn index_tools(&self, tools: &[Tool], extension_name: &str) -> Result<(), ErrorData>;
    async fn remove_tool(&self, tool_name: &str) -> Result<(), ErrorData>;
    async fn record_tool_call(&self, tool_name: &str) -> Result<(), ErrorData>;
    async fn get_recent_tool_calls(&self, limit: usize) -> Result<Vec<String>, ErrorData>;
    // NEW: Add method to get selected tools
    async fn get_selected_tools(&self, limit: usize) -> Result<Vec<String>, ErrorData>;
}

pub struct LLMToolSelector {
    llm_provider: Arc<dyn Provider>,
    tool_strings: Arc<RwLock<HashMap<String, String>>>, // extension_name -> tool_string
    recent_tool_calls: Arc<RwLock<VecDeque<String>>>,
    // NEW: Track selected tools from router searches
    selected_tools: Arc<RwLock<VecDeque<String>>>,
}

impl LLMToolSelector {
    pub async fn new(provider: Arc<dyn Provider>) -> Result<Self> {
        Ok(Self {
            llm_provider: provider.clone(),
            tool_strings: Arc::new(RwLock::new(HashMap::new())),
            recent_tool_calls: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            // NEW: Initialize selected tools tracking
            selected_tools: Arc::new(RwLock::new(VecDeque::with_capacity(50))),
        })
    }
}

#[async_trait]
impl RouterToolSelector for LLMToolSelector {
    async fn select_tools(&self, params: Value) -> Result<Vec<Content>, ErrorData> {
        let query = params
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ErrorData {
                code: ErrorCode::INVALID_PARAMS,
                message: Cow::from("Missing 'query' parameter"),
                data: None,
            })?;

        let extension_name = params
            .get("extension_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Get relevant tool strings based on extension_name
        let tool_strings = self.tool_strings.read().await;
        let relevant_tools = if let Some(ext) = &extension_name {
            tool_strings.get(ext).cloned()
        } else {
            // If no extension specified, use all tools
            Some(
                tool_strings
                    .values()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        };

        if let Some(tools) = relevant_tools {
            // Use template to generate the prompt
            let context = ToolSelectorContext {
                tools: tools.clone(),
                query: query.to_string(),
            };

            let user_prompt =
                render_global_file("router_tool_selector.md", &context).map_err(|e| ErrorData {
                    code: ErrorCode::INTERNAL_ERROR,
                    message: Cow::from(format!("Failed to render prompt template: {}", e)),
                    data: None,
                })?;

            let user_message = Message::user().with_text(&user_prompt);
            let response = self
                .llm_provider
                .complete("system", &[user_message], &[])
                .await
                .map_err(|e| ErrorData {
                    code: ErrorCode::INTERNAL_ERROR,
                    message: Cow::from(format!("Failed to search tools: {}", e)),
                    data: None,
                })?;

            // Extract just the message content from the response
            let (message, _usage) = response;
            let text = message.content[0].as_text().unwrap_or_default();

            // NEW: Extract and record tool names from the response
            let selected_tool_names = self.extract_tool_names_from_response(&text);
            self.record_selected_tools(&selected_tool_names).await?;

            // Split the response into individual tool entries
            let tool_entries: Vec<Content> = text
                .split("\n\n")
                .filter(|entry| entry.trim().starts_with("Tool:"))
                .map(|entry| Content::text(entry.trim().to_string()))
                .collect();

            Ok(tool_entries)
        } else {
            Ok(vec![])
        }
    }

    // NEW: Get selected tools (similar to get_recent_tool_calls)
    async fn get_selected_tools(&self, limit: usize) -> Result<Vec<String>, ErrorData> {
        let selected_tools = self.selected_tools.read().await;
        Ok(selected_tools.iter().take(limit).cloned().collect())
    }

    async fn index_tools(&self, tools: &[Tool], extension_name: &str) -> Result<(), ErrorData> {
        let mut tool_strings = self.tool_strings.write().await;

        for tool in tools {
            let tool_string = format!(
                "Tool: {}\nDescription: {}\nSchema: {}",
                tool.name,
                tool.description
                    .as_ref()
                    .map(|d| d.as_ref())
                    .unwrap_or_default(),
                serde_json::to_string_pretty(&tool.input_schema)
                    .unwrap_or_else(|_| "{}".to_string())
            );

            // Use the provided extension_name instead of parsing from tool name
            let entry = tool_strings.entry(extension_name.to_string()).or_default();

            // Check if this tool already exists in the entry
            if !entry.contains(&format!("Tool: {}", tool.name)) {
                if !entry.is_empty() {
                    entry.push_str("\n\n");
                }
                entry.push_str(&tool_string);
            }
        }

        Ok(())
    }
    async fn remove_tool(&self, tool_name: &str) -> Result<(), ErrorData> {
        let mut tool_strings = self.tool_strings.write().await;
        if let Some(extension_name) = tool_name.split("__").next() {
            tool_strings.remove(extension_name);
        }
        Ok(())
    }

    async fn record_tool_call(&self, tool_name: &str) -> Result<(), ErrorData> {
        let mut recent_calls = self.recent_tool_calls.write().await;
        if recent_calls.len() >= 100 {
            recent_calls.pop_front();
        }
        recent_calls.push_back(tool_name.to_string());
        Ok(())
    }

    async fn get_recent_tool_calls(&self, limit: usize) -> Result<Vec<String>, ErrorData> {
        let recent_calls = self.recent_tool_calls.read().await;
        Ok(recent_calls.iter().rev().take(limit).cloned().collect())
    }
}

impl LLMToolSelector {
    // NEW: Extract tool names from router response
    fn extract_tool_names_from_response(&self, response_text: &str) -> Vec<String> {
        response_text
            .split("\n\n")
            .filter(|entry| entry.trim().starts_with("Tool:"))
            .filter_map(|entry| {
                entry
                    .lines()
                    .next()
                    .and_then(|line| line.strip_prefix("Tool:"))
                    .map(|name| name.trim().to_string())
            })
            .filter(|tool_name| tool_name != ROUTER_LLM_SEARCH_TOOL_NAME) // Don't include the router tool itself
            .collect()
    }

    // NEW: Record selected tools
    async fn record_selected_tools(&self, tool_names: &[String]) -> Result<(), ErrorData> {
        let mut selected_tools = self.selected_tools.write().await;

        // Add new selected tools to the front (most recent first)
        for tool_name in tool_names.iter().rev() {
            // Remove if already exists (to avoid duplicates)
            selected_tools.retain(|name| name != tool_name);
            // Add to front
            selected_tools.push_front(tool_name.clone());
        }

        // Keep only the most recent 50 selected tools
        while selected_tools.len() > 50 {
            selected_tools.pop_back();
        }

        Ok(())
    }
}

// Helper function to create a boxed tool selector
pub async fn create_tool_selector(
    provider: Arc<dyn Provider>,
) -> Result<Box<dyn RouterToolSelector>> {
    let selector = LLMToolSelector::new(provider).await?;
    Ok(Box::new(selector))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::mock::MockProvider;

    #[tokio::test]
    async fn test_extract_tool_names_from_response() {
        let provider = Arc::new(MockProvider::new());
        let selector = LLMToolSelector::new(provider).await.unwrap();

        let response_text = r#"
Tool: developer__list_files
Description: List files in a directory
Schema: {"type": "object"}

Tool: developer__read_file
Description: Read file contents
Schema: {"type": "object"}

Tool: router__llm_search
Description: Search for tools
Schema: {"type": "object"}
"#;

        let tool_names = selector.extract_tool_names_from_response(response_text);

        // Should extract tool names but exclude the router tool itself
        assert_eq!(tool_names.len(), 2);
        assert!(tool_names.contains(&"developer__list_files".to_string()));
        assert!(tool_names.contains(&"developer__read_file".to_string()));
        assert!(!tool_names.contains(&"router__llm_search".to_string()));
    }

    #[tokio::test]
    async fn test_record_and_get_selected_tools() {
        let provider = Arc::new(MockProvider::new());
        let selector = LLMToolSelector::new(provider).await.unwrap();

        // Record some selected tools
        let tool_names = vec![
            "developer__list_files".to_string(),
            "developer__read_file".to_string(),
        ];

        selector.record_selected_tools(&tool_names).await.unwrap();

        // Get selected tools
        let selected = selector.get_selected_tools(10).await.unwrap();

        // Should return tools in reverse order (most recent first)
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0], "developer__read_file");
        assert_eq!(selected[1], "developer__list_files");
    }

    #[tokio::test]
    async fn test_selected_tools_deduplication() {
        let provider = Arc::new(MockProvider::new());
        let selector = LLMToolSelector::new(provider).await.unwrap();

        // Record tools first time
        let tool_names1 = vec![
            "developer__list_files".to_string(),
            "developer__read_file".to_string(),
        ];
        selector.record_selected_tools(&tool_names1).await.unwrap();

        // Record same tools again (should deduplicate)
        let tool_names2 = vec![
            "developer__list_files".to_string(),
            "developer__write_file".to_string(),
        ];
        selector.record_selected_tools(&tool_names2).await.unwrap();

        // Get selected tools
        let selected = selector.get_selected_tools(10).await.unwrap();

        // Should have 3 unique tools, with most recent first
        assert_eq!(selected.len(), 3);
        assert_eq!(selected[0], "developer__write_file");
        assert_eq!(selected[1], "developer__list_files");
        assert_eq!(selected[2], "developer__read_file");
    }
}
