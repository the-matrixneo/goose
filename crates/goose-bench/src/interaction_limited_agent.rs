use anyhow::Result;
use futures::stream::StreamExt;
use goose::agents::{Agent, AgentEvent};
use goose::message::{Message, MessageContent};

pub struct InteractionLimitedAgent {
    agent: Agent,
    messages: Vec<Message>,
    max_interactions: Option<usize>,
}

impl InteractionLimitedAgent {
    pub fn new(agent: Agent, max_interactions: Option<usize>) -> Self {
        Self {
            agent,
            messages: Vec::new(),
            max_interactions,
        }
    }
    
    pub async fn prompt_with_limit(&mut self, prompt: String, max_interactions: usize) -> Result<Vec<Message>> {
        self.process_prompt(prompt, Some(max_interactions)).await
    }
    
    async fn process_prompt(&mut self, prompt: String, max_interactions: Option<usize>) -> Result<Vec<Message>> {
        let limit = max_interactions.or(self.max_interactions).unwrap_or(usize::MAX);
        
        if limit == 0 {
            return Ok(self.messages.clone());
        }
        
        let verbose_conversations = std::env::var("GOOSE_BENCH_VERBOSE_CONVERSATIONS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);
        
        self.messages.push(Message::user().with_text(&prompt));
        
        let mut reply_stream = self.agent.reply(&self.messages, None).await?;
        let mut interactions_count = 0;
        let mut collected_messages = Vec::new();
        
        while let Some(event) = reply_stream.next().await {
            if let AgentEvent::Message(msg) = event? {
                let has_tool_requests = msg.content.iter().any(|c| 
                    matches!(c, MessageContent::ToolRequest(_))
                );
                
                if has_tool_requests {
                    interactions_count += 1;
                    tracing::info!("LLM response {} with tool calls", interactions_count);
                    
                    // Log the message content if verbose
                    if verbose_conversations {
                        let content = msg.content.iter()
                            .map(|c| match c {
                                MessageContent::Text(text) => format!("Text: {}", text.text),
                                MessageContent::ToolRequest(tool_req) => {
                                    match &tool_req.tool_call {
                                        Ok(tool_call) => format!("Tool: {}", tool_call.name),
                                        Err(_) => "Tool: <invalid>".to_string(),
                                    }
                                },
                                MessageContent::ToolResponse(_) => "ToolResponse".to_string(),
                                _ => "Other".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        tracing::info!("  Content: {}", content);
                    }
                    
                    collected_messages.push(msg);
                    
                    if interactions_count >= limit {
                        tracing::info!("Reached interaction limit of {} interactions. Stopping before tool execution.", limit);
                        break;
                    }
                } else {
                    // Count non-tool assistant messages as interactions
                    let has_tool_responses = msg.content.iter().any(|c| 
                        matches!(c, MessageContent::ToolResponse(_))
                    );
                    
                    if !has_tool_responses && matches!(msg.role, mcp_core::role::Role::Assistant) {
                        interactions_count += 1;
                        tracing::info!("LLM response {} without tool calls", interactions_count);
                        
                        // Log the message content if verbose
                        if verbose_conversations {
                            let content = msg.content.iter()
                                .map(|c| match c {
                                    MessageContent::Text(text) => text.text.clone(),
                                    _ => "<non-text content>".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            tracing::info!("  Content: {}", content);
                        }
                        
                        collected_messages.push(msg);
                        
                        if interactions_count >= limit {
                            tracing::info!("Reached interaction limit of {} interactions.", limit);
                            break;
                        }
                    } else {
                        // Tool responses and other messages
                        if has_tool_responses {
                            tracing::debug!("Received tool response");
                        }
                        collected_messages.push(msg);
                    }
                }
            }
        }
        
        self.messages.extend(collected_messages);
        tracing::info!("Completed processing with {} LLM interactions", interactions_count);
        
        Ok(self.messages.clone())
    }
    
    pub async fn prompt(&mut self, prompt: String) -> Result<Vec<Message>> {
        // Use unlimited interactions for regular prompt
        self.process_prompt(prompt, None).await
    }
    
    pub fn message_history(&self) -> Vec<Message> {
        self.messages.clone()
    }
    
    pub async fn override_system_prompt(&self, override_prompt: String) {
        self.agent.override_system_prompt(override_prompt).await;
    }
    
    pub fn get_agent(&self) -> &Agent {
        &self.agent
    }
}