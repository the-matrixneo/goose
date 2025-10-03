pub mod patterns;
pub mod scanner;
pub mod security_inspector;

use crate::conversation::message::{Message, ToolRequest};
use crate::permission::permission_judge::PermissionCheckResult;
use anyhow::Result;
use scanner::PromptInjectionScanner;
use std::sync::OnceLock;
use uuid::Uuid;

pub struct SecurityManager {
    scanner: OnceLock<PromptInjectionScanner>,
}

#[derive(Debug, Clone)]
pub struct SecurityResult {
    pub is_malicious: bool,
    pub confidence: f32,
    pub explanation: String,
    pub should_ask_user: bool,
    pub finding_id: String,
    pub tool_request_id: String,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            scanner: OnceLock::new(),
        }
    }

    /// Check if prompt injection security is enabled
    pub fn is_prompt_injection_detection_enabled(&self) -> bool {
        use crate::config::Config;
        let config = Config::global();

        config
            .get_param::<bool>("security_prompt_enabled")
            .unwrap_or(false)
    }

    /// New method for tool inspection framework - works directly with tool requests
    pub async fn analyze_tool_requests(
        &self,
        tool_requests: &[ToolRequest],
        messages: &[Message],
    ) -> Result<Vec<SecurityResult>> {
        if !self.is_prompt_injection_detection_enabled() {
            tracing::debug!("🔓 Security scanning disabled - returning empty results");
            return Ok(vec![]);
        }

        let scanner = self.scanner.get_or_init(|| {
            tracing::info!("Security scanner initialized and enabled");
            PromptInjectionScanner::new()
        });

        let mut results = Vec::new();

        tracing::info!(
            "🔍 Starting security analysis - {} tool requests, {} messages",
            tool_requests.len(),
            messages.len()
        );

        // Analyze each tool request
        for (i, tool_request) in tool_requests.iter().enumerate() {
            if let Ok(tool_call) = &tool_request.tool_call {
                tracing::info!(
                    tool_name = %tool_call.name,
                    tool_index = i,
                    tool_request_id = %tool_request.id,
                    tool_args = ?tool_call.arguments,
                    "🔍 Starting security analysis for current tool call"
                );

                let analysis_result = scanner
                    .analyze_tool_call_with_context(tool_call, messages)
                    .await?;

                // Get threshold from config - only flag things above threshold
                let config_threshold = scanner.get_threshold_from_config();

                if analysis_result.is_malicious && analysis_result.confidence > config_threshold {
                    // Generate a globally unique finding ID for each security finding
                    let finding_id = format!("SEC-{}", Uuid::new_v4().simple());

                    tracing::warn!(
                        tool_name = %tool_call.name,
                        tool_request_id = %tool_request.id,
                        confidence = analysis_result.confidence,
                        explanation = %analysis_result.explanation,
                        finding_id = %finding_id,
                        threshold = config_threshold,
                        "🔒 Current tool call flagged as malicious after security analysis (above threshold)"
                    );

                    results.push(SecurityResult {
                        is_malicious: analysis_result.is_malicious,
                        confidence: analysis_result.confidence,
                        explanation: analysis_result.explanation,
                        should_ask_user: true, // Always ask user for threats above threshold
                        finding_id,
                        tool_request_id: tool_request.id.clone(),
                    });
                } else if analysis_result.is_malicious {
                    tracing::warn!(
                        tool_name = %tool_call.name,
                        tool_request_id = %tool_request.id,
                        confidence = analysis_result.confidence,
                        explanation = %analysis_result.explanation,
                        threshold = config_threshold,
                        "🔒 Security finding below threshold - logged but not blocking execution"
                    );
                } else {
                    tracing::debug!(
                        tool_name = %tool_call.name,
                        tool_request_id = %tool_request.id,
                        confidence = analysis_result.confidence,
                        explanation = %analysis_result.explanation,
                        "✅ Current tool call passed security analysis"
                    );
                }
            }
        }

        tracing::info!(
            "🔍 Security analysis complete - found {} security issues in current tool requests",
            results.len()
        );
        Ok(results)
    }

    /// Main security check function - called from reply_internal
    /// Uses the proper two-step security analysis process
    /// Scans ALL tools (approved + needs_approval) for security threats
    pub async fn filter_malicious_tool_calls(
        &self,
        messages: &[Message],
        permission_check_result: &PermissionCheckResult,
        _system_prompt: Option<&str>,
    ) -> Result<Vec<SecurityResult>> {
        // Extract tool requests from permission result and delegate to new method
        let tool_requests: Vec<_> = permission_check_result
            .approved
            .iter()
            .chain(permission_check_result.needs_approval.iter())
            .cloned()
            .collect();

        self.analyze_tool_requests(&tool_requests, messages).await
    }

    /// Check if models need to be downloaded and return appropriate user message
    pub async fn check_model_download_status(&self) -> Option<String> {
        // Phase 1: No ML models needed, pattern matching is instant
        None
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}
