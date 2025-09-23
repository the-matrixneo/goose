use crate::agents::recipe_agent_runner::RecipeAgentRunner;
use crate::{agents::subagent_task_config::TaskConfig, recipe::Recipe};
use crate::agents::Agent;
use crate::conversation::message::Message;
use anyhow::Result;
use rmcp::model::{ErrorCode, ErrorData};
use tokio::sync::mpsc::UnboundedSender;

pub async fn run_complete_subagent_task_with_options_stream(
    recipe: &Recipe,
    task_config: TaskConfig,
    event_sender: Option<UnboundedSender<Message>>,
) -> Result<String, anyhow::Error> {
    let recipe_agent_runner = RecipeAgentRunner::new(recipe.clone(), task_config);
    // Execute the task using a standalone agent instance
    let messages = recipe_agent_runner.run_recipe(event_sender)
        .await
        .map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to execute task: {}", e),
                None,
            )
        })?;

    // TODO: lifei: use recipe final output
    let return_last_only = true;
    // Extract text content based on return_last_only flag
    let response_text = if return_last_only {
        // Get only the last message's text content
        messages
            .messages()
            .last()
            .and_then(|message| {
                message.content.iter().find_map(|content| match content {
                    crate::conversation::message::MessageContent::Text(text_content) => {
                        Some(text_content.text.clone())
                    }
                    _ => None,
                })
            })
            .unwrap_or_else(|| String::from("No text content in last message"))
    } else {
        // Extract all text content from all messages (original behavior)
        let all_text_content: Vec<String> = messages
            .iter()
            .flat_map(|message| {
                message.content.iter().filter_map(|content| {
                    match content {
                        crate::conversation::message::MessageContent::Text(text_content) => {
                            Some(text_content.text.clone())
                        }
                        crate::conversation::message::MessageContent::ToolResponse(
                            tool_response,
                        ) => {
                            // Extract text from tool response
                            if let Ok(contents) = &tool_response.tool_result {
                                let texts: Vec<String> = contents
                                    .iter()
                                    .filter_map(|content| {
                                        if let rmcp::model::RawContent::Text(raw_text_content) =
                                            &content.raw
                                        {
                                            Some(raw_text_content.text.clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                if !texts.is_empty() {
                                    Some(format!("Tool result: {}", texts.join("\n")))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
            })
            .collect();

        all_text_content.join("\n")
    };

    // Return the result
    Ok(response_text)
}
