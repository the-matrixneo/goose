use crate::agents::extension_manager::ExtensionManager;
use crate::agents::SessionConfig;
use crate::conversation::message::{Message, MessageContent};
use uuid::Uuid;

/// Inject MOIM (Minus One Info Message) into conversation.
///
/// MOIM provides ephemeral context that's included in LLM calls
/// but never persisted to conversation history.
pub async fn inject_moim(
    messages: &[Message],
    extension_manager: &ExtensionManager,
    _session: &Option<SessionConfig>,
) -> Vec<Message> {
    let moim_content = match extension_manager.collect_moim().await {
        Some(content) if !content.trim().is_empty() => content,
        _ => {
            tracing::debug!("No MOIM content available");
            return messages.to_vec();
        }
    };

    tracing::debug!("Injecting MOIM: {} chars", moim_content.len());

    let moim_message = Message::user()
        .with_text(moim_content)
        .with_id(format!("moim_{}", Uuid::new_v4()));

    let mut messages_with_moim = messages.to_vec();

    if messages_with_moim.is_empty() {
        messages_with_moim.push(moim_message);
    } else {
        let insert_pos = find_moim_insertion_point(&messages_with_moim);
        messages_with_moim.insert(insert_pos, moim_message);
    }

    messages_with_moim
}

/// Find a safe insertion point for MOIM that won't break tool call/response pairs.
fn find_moim_insertion_point(messages: &[Message]) -> usize {
    if messages.is_empty() {
        return 0;
    }

    let last_pos = messages.len() - 1;

    // Don't break tool call/response pairs
    if last_pos > 0 {
        let prev_msg = &messages[last_pos - 1];
        let curr_msg = &messages[last_pos];

        let prev_has_tool_calls = prev_msg
            .content
            .iter()
            .any(|c| matches!(c, MessageContent::ToolRequest(_)));

        let curr_has_tool_responses = curr_msg
            .content
            .iter()
            .any(|c| matches!(c, MessageContent::ToolResponse(_)));

        if prev_has_tool_calls && curr_has_tool_responses {
            tracing::debug!("MOIM: Adjusting position to avoid breaking tool pair");
            return last_pos.saturating_sub(1);
        }
    }

    last_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_moim_insertion_empty_conversation() {
        let messages = vec![];
        let extension_manager = ExtensionManager::new();

        let result = inject_moim(&messages, &extension_manager, &None).await;

        // MOIM always includes timestamp when enabled, so should have one message
        assert_eq!(result.len(), 1);
        assert!(result[0].id.as_ref().unwrap().starts_with("moim_"));

        // Verify the message contains timestamp
        let content = result[0].content.first().and_then(|c| c.as_text()).unwrap();
        assert!(content.contains("Current date and time:"));
    }

    #[tokio::test]
    async fn test_moim_insertion_with_messages() {
        let messages = vec![
            Message::user().with_text("Hello"),
            Message::assistant().with_text("Hi there"),
        ];
        let extension_manager = ExtensionManager::new();

        let result = inject_moim(&messages, &extension_manager, &None).await;

        // MOIM always includes timestamp, should have 3 messages with MOIM inserted before last
        assert_eq!(result.len(), 3);
        // MOIM should be inserted at position 1 (before the last message)
        assert!(result[1].id.as_ref().unwrap().starts_with("moim_"));

        // Verify the message contains timestamp
        let content = result[1].content.first().and_then(|c| c.as_text()).unwrap();
        assert!(content.contains("Current date and time:"));
    }

    #[test]
    fn test_find_insertion_point_empty() {
        let messages = vec![];
        assert_eq!(find_moim_insertion_point(&messages), 0);
    }

    #[test]
    fn test_find_insertion_point_single_message() {
        let messages = vec![Message::user().with_text("Hello")];
        assert_eq!(find_moim_insertion_point(&messages), 0);
    }

    #[test]
    fn test_find_insertion_point_multiple_messages() {
        let messages = vec![
            Message::user().with_text("Hello"),
            Message::assistant().with_text("Hi"),
            Message::user().with_text("How are you?"),
        ];
        assert_eq!(find_moim_insertion_point(&messages), 2);
    }
}
