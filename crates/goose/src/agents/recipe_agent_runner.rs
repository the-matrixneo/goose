use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use chrono::Utc;
use futures::future::BoxFuture;
use mcp_core::Tool;
use serde_json::Value;

use crate::{agents::{Agent, ExtensionConfig, SessionConfig, TaskConfig}, config::ExtensionConfigManager, conversation::{message::Message, Conversation}, prompt_template::render_global_file, recipe::Recipe, session::Identifier};

/// Default maximum number of turns for task execution
pub const DEFAULT_RECIPE_AGENT_MAX_TURNS: u32 = 25;

pub struct RecipeAgentRunner {
    recipe: Recipe,
    task_config: TaskConfig,
}

impl RecipeAgentRunner {
    // TODO: lifei: rename task_config to default settings
    pub fn new(recipe: Recipe, task_config: TaskConfig) -> Self {
        Self { recipe, task_config }
    }

    pub fn run_recipe(
        &self, event_sender: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    ) -> BoxFuture<'static, Result<Conversation>> {
        
        Box::pin(async move {
            let session_config = SessionConfig {
                id: Identifier::Name("recipe_agent_runner".to_string()),
                working_dir: std::env::current_dir()?,
                schedule_id: None,
                execution_mode: None,
                max_turns: Some(DEFAULT_RECIPE_AGENT_MAX_TURNS),
                retry_config: self.recipe.retry.clone(),
            };
            // self.initialize_for_task(&task_config).await?;
            self.execute_task_sequence(text_instruction, task_config, event_sender)
                .await
        })
    }

    async fn initialize_agent(&self) -> Result<()> {
        let agent = Agent::new();
        let provider = self.task_config
            .provider()
            .ok_or_else(|| anyhow!("No provider configured for standalone task"))?;
        agent.update_provider(Arc::clone(provider)).await?;

        // TODO: lifei: use recipe settings to update provider

        agent.disable_router_for_recipe().await;
        let extensions_to_add = if let Some(ref extensions) = self.recipe.extensions {
            extensions.clone()
        } else {
            ExtensionConfigManager::get_all()
                .unwrap_or_default()
                .into_iter()
                .filter(|ext| ext.enabled)
                .map(|ext| ext.config)
                .collect::<Vec<ExtensionConfig>>()
        };

        for extension in extensions_to_add {
            agent.extension_manager.add_extension(extension).await 
                .map_err(|e| anyhow!("Failed to add extension to standalone agent: {}", e))?;
        } 
        let system_prompt = build_subagent_system_prompt(&self.task_config, &extensions_to_add)?;   
        Ok(())
    }

    fn execute_task_sequence(
        &self,
        task_config: TaskConfig,
        event_sender: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    ) -> BoxFuture<'_, Result<Conversation>> {
        Box::pin(async move {
            let user_message = Message::user().with_text(self.recipe.instructions.clone().unwrap_or_default());
            let conversation = Conversation::new_unvalidated(vec![user_message]);

            let tools = self
                .extension_manager
                .get_prefixed_tools(None)
                .await
                .unwrap_or_default();
            let system_prompt = build_subagent_system_prompt(&task_config, &tools)?;
            let previous_override = {
                let prompt_manager = self.prompt_manager.lock().await;
                prompt_manager.system_prompt_override()
            };
            {
                let mut prompt_manager = self.prompt_manager.lock().await;
                prompt_manager.set_system_prompt_override(system_prompt);
            }

            let mut stream = self
                .reply_internal(conversation.clone(), None, None)
                .await?;
            let mut final_conversation = conversation;
            let mut assistant_turns = 0u32;
            let max_turns = task_config
                .max_turns
                .map(|turns| turns as u32)
                .unwrap_or(DEFAULT_SUBAGENT_MAX_TURNS as u32);
            let mut stop_after_message = false;

            while let Some(event) = stream.next().await {
                match event? {
                    AgentEvent::Message(message) => {
                        if let Some(sender) = &event_sender {
                            let _ = sender.send(message.clone());
                        }
                        if message.role == Role::Assistant {
                            assistant_turns = assistant_turns.saturating_add(1);
                            if assistant_turns >= max_turns {
                                stop_after_message = true;
                            }
                        }
                        final_conversation.push(message);
                    }
                    AgentEvent::HistoryReplaced(history) => {
                        final_conversation = Conversation::new_unvalidated(history);
                    }
                    AgentEvent::McpNotification(_) | AgentEvent::ModelChange { .. } => {}
                }

                if stop_after_message {
                    break;
                }
            }

            {
                let mut prompt_manager = self.prompt_manager.lock().await;
                if let Some(override_text) = previous_override {
                    prompt_manager.set_system_prompt_override(override_text);
                } else {
                    prompt_manager.clear_system_prompt_override();
                }
            }

            Ok(final_conversation)
        })
    }

    fn build_subagent_system_prompt(
        task_config: &TaskConfig,
        max_turns: Option<u32>,
        available_tools: &[Tool],
    ) -> Result<String> {
        let mut context = HashMap::new();
    
        context.insert(
            "current_date_time",
            Value::String(Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()),
        );
        context.insert("subagent_id", Value::String(task_config.id.clone()));
    
        if let Some(max_turns) = max_turns {
            context.insert(
                "max_turns",
                Value::Number(serde_json::Number::from(max_turns as u64)),
            );
        }
    
        let tools_with_descriptions: Vec<String> = available_tools
            .iter()
            .map(|tool| {
                if let Some(description) = &tool.description {
                    format!("{}: {}", tool.name, description)
                } else {
                    tool.name.to_string()
                }
            })
            .collect();
    
        context.insert(
            "available_tools",
            Value::String(if tools_with_descriptions.is_empty() {
                "None".to_string()
            } else {
                tools_with_descriptions.join(", ")
            }),
        );
    
        context.insert(
            "tool_count",
            Value::Number(serde_json::Number::from(available_tools.len() as u64)),
        );
    
        render_global_file("subagent_system.md", &context)
            .map_err(|e| anyhow!("Failed to render subagent system prompt: {}", e))
    }
}