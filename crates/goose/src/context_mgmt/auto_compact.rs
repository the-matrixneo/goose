use crate::conversation::message::Message;
use crate::conversation::{fix_conversation, Conversation};
use crate::{
    agents::Agent, config::Config, context_mgmt::get_messages_token_counts_async,
    token_counter::create_async_token_counter,
};
use anyhow::Result;
use tracing::{debug, info};

/// Result of auto-compaction check
#[derive(Debug)]
pub struct AutoCompactResult {
    /// Whether compaction was performed
    pub compacted: bool,
    /// The messages after potential compaction
    pub messages: Conversation,
    /// Provider usage from summarization (if compaction occurred)
    /// This contains the actual token counts after compaction
    pub summarization_usage: Option<crate::providers::base::ProviderUsage>,
}

/// Result of checking if compaction is needed
#[derive(Debug)]
pub struct CompactionCheckResult {
    /// Whether compaction is needed
    pub needs_compaction: bool,
    /// Current token count
    pub current_tokens: usize,
    /// Context limit being used
    pub context_limit: usize,
    /// Current usage ratio (0.0 to 1.0)
    pub usage_ratio: f64,
    /// Remaining tokens before compaction threshold
    pub remaining_tokens: usize,
    /// Percentage until compaction threshold (0.0 to 100.0)
    pub percentage_until_compaction: f64,
}

/// Check if messages need compaction without performing the compaction
///
/// This function analyzes the current token usage and returns detailed information
/// about whether compaction is needed and how close we are to the threshold.
/// It prioritizes actual token counts from session metadata when available,
/// falling back to estimated counts if needed.
///
/// # Arguments
/// * `agent` - The agent to use for context management
/// * `messages` - The current message history
/// * `threshold_override` - Optional threshold override (defaults to GOOSE_AUTO_COMPACT_THRESHOLD config)
/// * `session_metadata` - Optional session metadata containing actual token counts
///
/// # Returns
/// * `CompactionCheckResult` containing detailed information about compaction needs
pub async fn check_compaction_needed(
    agent: &Agent,
    messages: &[Message],
    threshold_override: Option<f64>,
    session_metadata: Option<&crate::session::Session>,
) -> Result<CompactionCheckResult> {
    // Get threshold from config or use override
    let config = Config::global();
    let threshold = threshold_override.unwrap_or_else(|| {
        config
            .get_param::<f64>("GOOSE_AUTO_COMPACT_THRESHOLD")
            .unwrap_or(0.8) // Default to 80%
    });

    let provider = agent.provider().await?;
    let context_limit = provider.get_model_config().context_limit();

    let (current_tokens, token_source) = match session_metadata.and_then(|m| m.total_tokens) {
        Some(tokens) => (tokens as usize, "session metadata"),
        None => {
            let token_counter = create_async_token_counter()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create token counter: {}", e))?;
            let token_counts = get_messages_token_counts_async(&token_counter, messages);
            (token_counts.iter().sum(), "estimated")
        }
    };

    // Calculate usage ratio
    let usage_ratio = current_tokens as f64 / context_limit as f64;

    // Calculate threshold token count and remaining tokens
    let threshold_tokens = (context_limit as f64 * threshold) as usize;
    let remaining_tokens = threshold_tokens.saturating_sub(current_tokens);

    // Calculate percentage until compaction (how much more we can use before hitting threshold)
    let percentage_until_compaction = if usage_ratio < threshold {
        (threshold - usage_ratio) * 100.0
    } else {
        0.0
    };

    // Check if compaction is needed (disabled if threshold is invalid)
    let needs_compaction = if threshold <= 0.0 || threshold >= 1.0 {
        false
    } else {
        usage_ratio > threshold
    };

    debug!(
        "Compaction check: {} / {} tokens ({:.1}%), threshold: {:.1}%, needs compaction: {}, source: {}",
        current_tokens,
        context_limit,
        usage_ratio * 100.0,
        threshold * 100.0,
        needs_compaction,
        token_source
    );

    Ok(CompactionCheckResult {
        needs_compaction,
        current_tokens,
        context_limit,
        usage_ratio,
        remaining_tokens,
        percentage_until_compaction,
    })
}

/// Perform compaction on messages without checking thresholds
///
/// This function directly performs compaction on the provided messages.
/// If the most recent message is a user message, it will be preserved by removing it
/// before compaction and adding it back afterwards.
///
/// # Arguments
/// * `agent` - The agent to use for context management
/// * `messages` - The current message history
///
/// # Returns
/// * `AutoCompactResult` containing the compacted messages and metadata
pub async fn perform_compaction(agent: &Agent, messages: &[Message]) -> Result<AutoCompactResult> {
    info!("Performing message compaction");

    let mut messages_to_process = messages.to_vec();
    
    // Check if the last assistant message contains a tool request
    // If so, remove it to prevent orphaned tool responses after compaction
    if let Some(last_assistant_pos) = messages_to_process.iter().rposition(|m| matches!(m.role, rmcp::model::Role::Assistant)) {
        if messages_to_process[last_assistant_pos].is_tool_call() {
            info!("Removing last assistant message with pending tool request before compaction");
            messages_to_process.remove(last_assistant_pos);
        }
    }

    // Check if the most recent message is a user message
    let (messages_to_compact, preserved_user_message) = if let Some(last_message) = messages_to_process.last()
    {
        if matches!(last_message.role, rmcp::model::Role::User) {
            // Remove the last user message before compaction
            (&messages_to_process[..messages_to_process.len() - 1], Some(last_message.clone()))
        } else {
            (messages_to_process.as_slice(), None)
        }
    } else {
        (messages_to_process.as_slice(), None)
    };

    // Perform the compaction on messages excluding the preserved user message
    let (mut compacted_messages, _, summarization_usage) =
        agent.summarize_context(messages_to_compact).await?;

    // Add back the preserved user message if it exists
    if let Some(user_message) = preserved_user_message {
        compacted_messages.push(user_message);
    }

    // Apply fix_conversation as an additional safety net to catch any edge cases
    let (fixed_conversation, issues) = fix_conversation(compacted_messages.clone());
    if !issues.is_empty() {
        debug!("Fixed issues during compaction: {:?}", issues);
    }

    Ok(AutoCompactResult {
        compacted: true,
        messages: fixed_conversation,
        summarization_usage,
    })
}

/// Check if messages need compaction and compact them if necessary
///
/// This is a convenience wrapper function that combines checking and compaction.
/// If the most recent message is a user message, it will be preserved by removing it
/// before compaction and adding it back afterwards.
/// If the last assistant message contains a tool request, it will be removed to
/// prevent orphaned tool responses.
///
/// # Arguments
/// * `agent` - The agent to use for context management
/// * `messages` - The current message history
/// * `threshold_override` - Optional threshold override (defaults to GOOSE_AUTO_COMPACT_THRESHOLD config)
/// * `session_metadata` - Optional session metadata containing actual token counts
///
/// # Returns
/// * `AutoCompactResult` containing the potentially compacted messages and metadata
pub async fn check_and_compact_messages(
    agent: &Agent,
    messages: &[Message],
    threshold_override: Option<f64>,
    session_metadata: Option<&crate::session::Session>,
) -> Result<AutoCompactResult> {
    // First check if compaction is needed
    let check_result =
        check_compaction_needed(agent, messages, threshold_override, session_metadata).await?;

    // If no compaction is needed, return early
    if !check_result.needs_compaction {
        debug!(
            "No compaction needed (usage: {:.1}% <= {:.1}% threshold)",
            check_result.usage_ratio * 100.0,
            check_result.percentage_until_compaction
        );
        return Ok(AutoCompactResult {
            compacted: false,
            messages: Conversation::new_unvalidated(messages.to_vec()),
            summarization_usage: None,
        });
    }

    info!(
        "Auto-compacting messages (usage: {:.1}%)",
        check_result.usage_ratio * 100.0
    );

    let mut messages_to_process = messages.to_vec();
    
    // Check if the last assistant message contains a tool request
    // If so, remove it to prevent orphaned tool responses after compaction
    if let Some(last_assistant_pos) = messages_to_process.iter().rposition(|m| matches!(m.role, rmcp::model::Role::Assistant)) {
        if messages_to_process[last_assistant_pos].is_tool_call() {
            info!("Removing last assistant message with pending tool request before auto-compaction");
            messages_to_process.remove(last_assistant_pos);
        }
    }

    // Check if the most recent message is a user message
    let (messages_to_compact, preserved_user_message) = if let Some(last_message) = messages_to_process.last()
    {
        if matches!(last_message.role, rmcp::model::Role::User) {
            // Remove the last user message before auto-compaction
            (&messages_to_process[..messages_to_process.len() - 1], Some(last_message.clone()))
        } else {
            (messages_to_process.as_slice(), None)
        }
    } else {
        (messages_to_process.as_slice(), None)
    };

    // Perform the compaction on messages excluding the preserved user message
    // The summarize_context method already handles the visibility properly
    let (mut summary_messages, _, summarization_usage) =
        agent.summarize_context(messages_to_compact).await?;

    // Add back the preserved user message if it exists
    // (keeps default visibility: both true)
    if let Some(user_message) = preserved_user_message {
        summary_messages.push(user_message);
    }

    // Apply fix_conversation as an additional safety net to catch any edge cases
    let (fixed_conversation, issues) = fix_conversation(summary_messages.clone());
    if !issues.is_empty() {
        debug!("Fixed issues during auto-compaction: {:?}", issues);
    }

    Ok(AutoCompactResult {
        compacted: true,
        messages: fixed_conversation,
        summarization_usage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::message::{Message, MessageContent};
    use crate::session::extension_data;
    use crate::{
        agents::Agent,
        model::ModelConfig,
        providers::base::{Provider, ProviderMetadata, ProviderUsage, Usage},
        providers::errors::ProviderError,
    };
    use chrono::Utc;
    use rmcp::model::{AnnotateAble, RawTextContent, Role, Tool};
    use std::sync::Arc;

    #[derive(Clone)]
    struct MockProvider {
        model_config: ModelConfig,
    }

    #[async_trait::async_trait]
    impl Provider for MockProvider {
        fn metadata() -> ProviderMetadata {
            ProviderMetadata::empty()
        }

        fn get_model_config(&self) -> ModelConfig {
            self.model_config.clone()
        }

        async fn complete_with_model(
            &self,
            _model_config: &ModelConfig,
            _system: &str,
            _messages: &[Message],
            _tools: &[Tool],
        ) -> Result<(Message, ProviderUsage), ProviderError> {
            // Return a short summary message
            Ok((
                Message::new(
                    Role::Assistant,
                    Utc::now().timestamp(),
                    vec![MessageContent::Text(
                        RawTextContent {
                            text: "Summary of conversation".to_string(),
                            meta: None,
                        }
                        .no_annotation(),
                    )],
                ),
                ProviderUsage::new("mock".to_string(), Usage::default()),
            ))
        }
    }

    fn create_test_message(text: &str) -> Message {
        Message::new(
            Role::User,
            Utc::now().timestamp(),
            vec![MessageContent::text(text.to_string())],
        )
    }

    fn create_test_session_metadata(
        message_count: usize,
        working_dir: &str,
    ) -> crate::session::Session {
        use crate::conversation::Conversation;
        use std::path::PathBuf;

        let mut conversation = Conversation::default();
        for i in 0..message_count {
            conversation.push(create_test_message(format!("message {}", i).as_str()));
        }

        crate::session::Session {
            id: "test_session".to_string(),
            working_dir: PathBuf::from(working_dir),
            description: "Test session".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
            schedule_id: Some("test_job".to_string()),
            recipe: None,
            total_tokens: Some(100),
            input_tokens: Some(50),
            output_tokens: Some(50),
            accumulated_total_tokens: Some(100),
            accumulated_input_tokens: Some(50),
            accumulated_output_tokens: Some(50),
            extension_data: extension_data::ExtensionData::new(),
            conversation: Some(conversation),
            message_count,
        }
    }

    #[tokio::test]
    async fn test_check_compaction_needed() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(Some(100_000)),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create small messages that won't trigger compaction
        let messages = vec![create_test_message("Hello"), create_test_message("World")];

        let result = check_compaction_needed(&agent, &messages, Some(0.3), None)
            .await
            .unwrap();

        assert!(!result.needs_compaction);
        assert!(result.current_tokens > 0);
        assert!(result.context_limit > 0);
        assert!(result.usage_ratio < 0.3);
        assert!(result.remaining_tokens > 0);
        assert!(result.percentage_until_compaction > 0.0);
    }

    #[tokio::test]
    async fn test_check_compaction_needed_disabled() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(Some(100_000)),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        let messages = vec![create_test_message("Hello")];

        // Test with threshold 0 (disabled)
        let result = check_compaction_needed(&agent, &messages, Some(0.0), None)
            .await
            .unwrap();

        assert!(!result.needs_compaction);

        // Test with threshold 1.0 (disabled)
        let result = check_compaction_needed(&agent, &messages, Some(1.0), None)
            .await
            .unwrap();

        assert!(!result.needs_compaction);
    }

    #[tokio::test]
    async fn test_auto_compact_disabled() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(Some(10_000)),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        let messages = vec![create_test_message("Hello"), create_test_message("World")];

        // Test with threshold 0 (disabled)
        let result = check_and_compact_messages(&agent, &messages, Some(0.0), None)
            .await
            .unwrap();

        assert!(!result.compacted);
        assert_eq!(result.messages.len(), messages.len());
        assert!(result.summarization_usage.is_none());

        // Test with threshold 1.0 (disabled)
        let result = check_and_compact_messages(&agent, &messages, Some(1.0), None)
            .await
            .unwrap();

        assert!(!result.compacted);
    }

    #[tokio::test]
    async fn test_auto_compact_below_threshold() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(Some(100_000)), // Increased to ensure overhead doesn't dominate
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create small messages that won't trigger compaction
        let messages = vec![create_test_message("Hello"), create_test_message("World")];

        let result = check_and_compact_messages(&agent, &messages, Some(0.3), None)
            .await
            .unwrap();

        assert!(!result.compacted);
        assert_eq!(result.messages.len(), messages.len());
    }

    #[tokio::test]
    async fn test_auto_compact_above_threshold() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(30_000.into()), // Smaller context limit to make threshold easier to hit
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create messages that will exceed 30% of the context limit
        // With 30k context limit, 30% is 9k tokens
        let mut messages = Vec::new();

        // Create much longer messages with more content to reach the threshold
        for i in 0..300 {
            messages.push(create_test_message(&format!(
                "This is message number {} with significantly more content to increase token count substantially. \
                 We need to ensure that our total token usage exceeds 30% of the available context \
                 limit after accounting for system prompt and tools overhead. This message contains \
                 multiple sentences to increase the token count substantially. Adding even more text here \
                 to make sure we have enough tokens. Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                 sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
                 quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute \
                 irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
                 Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit \
                 anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
                 doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi \
                 architecto beatae vitae dicta sunt explicabo.",
                i
            )));
        }

        let result = check_and_compact_messages(&agent, &messages, Some(0.3), None)
            .await
            .unwrap();

        if !result.compacted {
            eprintln!("Test failed - compaction not triggered");
        }

        assert!(result.compacted);
        assert!(result.summarization_usage.is_some());

        // Verify that summarization usage contains token counts
        if let Some(usage) = &result.summarization_usage {
            assert!(usage.usage.total_tokens.is_some());
            let after = usage.usage.total_tokens.unwrap_or(0) as usize;
            assert!(
                after > 0,
                "Token count after compaction should be greater than 0"
            );
        }

        // After compaction and fix_conversation, we should have some messages
        // Note: fix_conversation may remove messages (e.g., trailing assistant messages)
        assert!(!result.messages.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compact_uses_session_metadata() {
        use crate::session::Session;

        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(10_000.into()),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create some test messages
        let messages = vec![
            create_test_message("First message"),
            create_test_message("Second message"),
        ];

        // Create session with specific token counts
        #[allow(clippy::field_reassign_with_default)]
        let mut session = Session::default();
        {
            session.total_tokens = Some(8000); // High token count to trigger compaction
            session.accumulated_total_tokens = Some(15000); // Even higher accumulated count
            session.input_tokens = Some(5000);
            session.output_tokens = Some(3000);
        }

        // Test with session - should use total_tokens for compaction (not accumulated)
        let result_with_metadata = check_compaction_needed(
            &agent,
            &messages,
            Some(0.3), // 30% threshold
            Some(&session),
        )
        .await
        .unwrap();

        // With 8000 tokens and context limit around 10000, should trigger compaction
        assert!(result_with_metadata.needs_compaction);
        assert_eq!(result_with_metadata.current_tokens, 8000);

        // Test without session metadata - should use estimated tokens
        let result_without_metadata = check_compaction_needed(
            &agent,
            &messages,
            Some(0.3), // 30% threshold
            None,
        )
        .await
        .unwrap();

        // Without metadata, should use much lower estimated token count
        assert!(!result_without_metadata.needs_compaction);
        assert!(result_without_metadata.current_tokens < 8000);

        // Test with session that has only accumulated tokens (no total_tokens)
        let mut session_metadata_no_total = Session::default();
        #[allow(clippy::field_reassign_with_default)]
        {
            session_metadata_no_total.accumulated_total_tokens = Some(7500);
        }

        let result_with_no_total = check_compaction_needed(
            &agent,
            &messages,
            Some(0.3), // 30% threshold
            Some(&session_metadata_no_total),
        )
        .await
        .unwrap();

        // Should fall back to estimation since total_tokens is None
        assert!(!result_with_no_total.needs_compaction);
        assert!(result_with_no_total.current_tokens < 7500);

        // Test with metadata that has no token counts - should fall back to estimation
        let empty_metadata = Session::default();

        let result_with_empty_metadata = check_compaction_needed(
            &agent,
            &messages,
            Some(0.3), // 30% threshold
            Some(&empty_metadata),
        )
        .await
        .unwrap();

        // Should fall back to estimation
        assert!(!result_with_empty_metadata.needs_compaction);
        assert!(result_with_empty_metadata.current_tokens < 7500);
    }

    #[tokio::test]
    async fn test_auto_compact_end_to_end_with_metadata() {
        use crate::session::Session;

        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(10_000.into()),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create some test messages
        let messages = vec![
            create_test_message("First message"),
            create_test_message("Second message"),
            create_test_message("Third message"),
            create_test_message("Fourth message"),
            create_test_message("Fifth message"),
        ];

        // Create session metadata with high token count to trigger compaction
        let mut session = Session::default();
        #[allow(clippy::field_reassign_with_default)]
        {
            session.total_tokens = Some(9000); // High enough to trigger compaction
        }

        // Test full compaction flow with session metadata
        let result = check_and_compact_messages(
            &agent,
            &messages,
            Some(0.3), // 30% threshold
            Some(&session),
        )
        .await
        .unwrap();

        // Should have triggered compaction
        assert!(result.compacted);
        assert!(result.summarization_usage.is_some());

        // Verify the compacted messages are returned
        assert!(!result.messages.is_empty());

        // After compaction and fix_conversation, we should have some messages
        // Note: fix_conversation may remove messages (e.g., trailing assistant messages)
    }

    #[tokio::test]
    async fn test_auto_compact_removes_pending_tool_request() {
        use mcp_core::tool::ToolCall;
        
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(10_000.into()),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        // Create messages including an assistant message with a tool request
        let mut messages = vec![
            create_test_message("First message"),
            create_test_message("Second message"),
        ];
        
        // Add an assistant message with a tool request
        let tool_call = ToolCall::new("test_tool", serde_json::json!({}));
        
        let assistant_msg = Message::assistant()
            .with_tool_request("test_tool_id".to_string(), Ok(tool_call))
            .with_text("I'll help you with that");
        messages.push(assistant_msg);
        
        // Create session metadata with high token count to trigger compaction
        let mut session_metadata = crate::session::storage::SessionMetadata::default();
        session_metadata.total_tokens = Some(9000); // High enough to trigger compaction

        // Perform compaction
        let result = perform_compaction(&agent, &messages).await.unwrap();
        
        // The compaction should have removed the last assistant message with tool request
        assert!(result.compacted);
        
        // Check that the last assistant message with tool request is not in the compacted messages
        let has_tool_request = result.messages.messages().iter().any(|m| {
            matches!(m.role, rmcp::model::Role::Assistant) && m.is_tool_call()
        });
        assert!(!has_tool_request, "Compacted messages should not contain assistant message with tool request");
    }

    #[tokio::test]
    async fn test_auto_compact_with_comprehensive_session_metadata() {
        let mock_provider = Arc::new(MockProvider {
            model_config: ModelConfig::new("test-model")
                .unwrap()
                .with_context_limit(8_000.into()),
        });

        let agent = Agent::new();
        let _ = agent.update_provider(mock_provider).await;

        let messages = vec![
            create_test_message("Test message 1"),
            create_test_message("Test message 2"),
            create_test_message("Test message 3"),
        ];

        // Use the helper function to create comprehensive non-null session metadata
        let comprehensive_metadata = create_test_session_metadata(3, "/test/working/dir");

        // Verify the helper created non-null metadata
        assert_eq!(
            comprehensive_metadata
                .clone()
                .conversation
                .unwrap_or_default()
                .len(),
            3
        );
        assert_eq!(
            comprehensive_metadata.working_dir.to_str().unwrap(),
            "/test/working/dir"
        );
        assert_eq!(comprehensive_metadata.description, "Test session");
        assert_eq!(
            comprehensive_metadata.schedule_id,
            Some("test_job".to_string())
        );
        assert_eq!(comprehensive_metadata.total_tokens, Some(100));
        assert_eq!(comprehensive_metadata.input_tokens, Some(50));
        assert_eq!(comprehensive_metadata.output_tokens, Some(50));
        assert_eq!(comprehensive_metadata.accumulated_total_tokens, Some(100));
        assert_eq!(comprehensive_metadata.accumulated_input_tokens, Some(50));
        assert_eq!(comprehensive_metadata.accumulated_output_tokens, Some(50));

        // Test compaction with the comprehensive metadata (low token count, shouldn't compact)
        let result_low_tokens = check_compaction_needed(
            &agent,
            &messages,
            Some(0.7), // 70% threshold
            Some(&comprehensive_metadata),
        )
        .await
        .unwrap();

        assert!(!result_low_tokens.needs_compaction);
        assert_eq!(result_low_tokens.current_tokens, 100); // Should use total_tokens from metadata

        // Create a modified version with high token count to trigger compaction
        let mut high_token_metadata = create_test_session_metadata(5, "/test/working/dir");
        high_token_metadata.total_tokens = Some(6_000); // High enough to trigger compaction
        high_token_metadata.input_tokens = Some(4_000);
        high_token_metadata.output_tokens = Some(2_000);
        high_token_metadata.accumulated_total_tokens = Some(12_000);

        let result_high_tokens = check_compaction_needed(
            &agent,
            &messages,
            Some(0.7), // 70% threshold
            Some(&high_token_metadata),
        )
        .await
        .unwrap();

        assert!(result_high_tokens.needs_compaction);
        assert_eq!(result_high_tokens.current_tokens, 6_000); // Should use total_tokens, not accumulated

        // Test that metadata fields are preserved correctly in edge cases
        let mut edge_case_metadata = create_test_session_metadata(10, "/edge/case/dir");
        edge_case_metadata.total_tokens = None; // No total tokens
        edge_case_metadata.accumulated_total_tokens = Some(7_000); // Has accumulated

        let result_edge_case = check_compaction_needed(
            &agent,
            &messages,
            Some(0.5), // 50% threshold
            Some(&edge_case_metadata),
        )
        .await
        .unwrap();

        // Should fall back to estimation since total_tokens is None
        assert!(result_edge_case.current_tokens < 7_000);
        // With estimation, likely won't trigger compaction
        assert!(!result_edge_case.needs_compaction);
    }
}
