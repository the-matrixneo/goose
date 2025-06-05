use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex, RwLock};
use uuid::Uuid;

use crate::message::Message;
use crate::recipe::Recipe;
use crate::providers::base::Provider;

/// Status of a subagent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubAgentStatus {
    Initializing,
    Ready,
    Processing,
    WaitingForInput,
    Completed,
    Failed(String),
    Terminated,
}

/// Progress update from a subagent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentProgress {
    pub subagent_id: String,
    pub status: SubAgentStatus,
    pub message: String,
    pub turn: usize,
    pub max_turns: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Configuration for creating a subagent
#[derive(Debug)]
pub struct SubAgentConfig {
    pub recipe: Recipe,
    pub max_turns: usize,
    pub timeout_seconds: Option<u64>,
    pub auto_terminate_phrases: Vec<String>,
}

impl SubAgentConfig {
    pub fn new(recipe: Recipe) -> Self {
        Self {
            recipe,
            max_turns: 5,
            timeout_seconds: Some(300), // 5 minutes
            auto_terminate_phrases: vec![
                "task complete".to_string(),
                "analysis complete".to_string(),
                "finished".to_string(),
                "done".to_string(),
            ],
        }
    }
}

/// SubAgent - Independent agent with its own context and lifecycle
pub struct SubAgent {
    pub id: String,
    pub config: SubAgentConfig,
    pub status: Arc<RwLock<SubAgentStatus>>,
    pub conversation: Arc<Mutex<Vec<Message>>>,
    pub current_turn: Arc<Mutex<usize>>,
    pub progress_tx: mpsc::Sender<SubAgentProgress>,
    
    // Internal agent for actual processing
    internal_agent: crate::agents::Agent,
}

impl SubAgent {
    /// Create a new subagent with the given configuration
    pub async fn new(
        config: SubAgentConfig,
        provider: Arc<dyn Provider>,
    ) -> Result<(Self, SubAgentHandle)> {
        let id = Uuid::new_v4().to_string();
        
        // Create communication channels
        let (command_tx, command_rx) = mpsc::channel(32);
        let (response_tx, response_rx) = mpsc::channel(32);
        let (progress_tx, progress_rx) = mpsc::channel(32);
        
        // Create internal agent and configure it with the recipe
        let internal_agent = crate::agents::Agent::new();
        internal_agent.update_provider(provider).await?;
        
        // Configure with recipe
        if let Some(instructions) = &config.recipe.instructions {
            internal_agent.extend_system_prompt(instructions.clone()).await;
        }
        
        // Add extensions from recipe
        if let Some(extensions) = &config.recipe.extensions {
            for extension in extensions {
                internal_agent.add_extension(extension.clone()).await?;
            }
        }
        
        let subagent = Self {
            id: id.clone(),
            config,
            status: Arc::new(RwLock::new(SubAgentStatus::Initializing)),
            conversation: Arc::new(Mutex::new(Vec::new())),
            current_turn: Arc::new(Mutex::new(0)),
            progress_tx: progress_tx.clone(),
            internal_agent,
        };
        
        let handle = SubAgentHandle {
            id,
            command_tx,
            response_rx: Arc::new(Mutex::new(response_rx)),
            progress_rx: Arc::new(Mutex::new(progress_rx)),
        };
        
        Ok((subagent, handle))
    }
    
    /// Process a single turn of conversation
    pub async fn process_message(&self, message: String) -> Result<Vec<Message>> {
        // Update status
        {
            let mut status_guard = self.status.write().await;
            *status_guard = SubAgentStatus::Processing;
        }
        
        // Send progress update
        let turn = {
            let mut turn_guard = self.current_turn.lock().await;
            *turn_guard += 1;
            *turn_guard
        };
        
        let progress = SubAgentProgress {
            subagent_id: self.id.clone(),
            status: SubAgentStatus::Processing,
            message: format!("Processing turn {}/{}", turn, self.config.max_turns),
            turn,
            max_turns: self.config.max_turns,
            timestamp: chrono::Utc::now(),
        };
        let _ = self.progress_tx.send(progress).await;
        
        // Add user message to conversation
        let user_message = Message::user().with_text(message);
        {
            let mut conv = self.conversation.lock().await;
            conv.push(user_message.clone());
        }
        
        // Get current conversation
        let current_conversation = {
            let conv = self.conversation.lock().await;
            conv.clone()
        };
        
        // Use internal agent to get response
        let provider = self.internal_agent.provider().await?;
        let (tools, _, system_prompt) = self.internal_agent.prepare_tools_and_prompt().await?;
        let (response, _usage) = provider.complete(&system_prompt, &current_conversation, &tools).await?;
        
        // Add response to conversation
        {
            let mut conv = self.conversation.lock().await;
            conv.push(response.clone());
        }
        
        // Check for termination
        let should_terminate = self.should_terminate(&response, turn);
        
        if should_terminate || turn >= self.config.max_turns {
            let mut status_guard = self.status.write().await;
            *status_guard = SubAgentStatus::Completed;
            
            let progress = SubAgentProgress {
                subagent_id: self.id.clone(),
                status: SubAgentStatus::Completed,
                message: "Conversation completed".to_string(),
                turn,
                max_turns: self.config.max_turns,
                timestamp: chrono::Utc::now(),
            };
            let _ = self.progress_tx.send(progress).await;
        } else {
            let mut status_guard = self.status.write().await;
            *status_guard = SubAgentStatus::WaitingForInput;
        }
        
        // Return the full conversation
        let final_conversation = self.conversation.lock().await.clone();
        Ok(final_conversation)
    }
    
    /// Get current conversation
    pub async fn get_conversation(&self) -> Vec<Message> {
        let conv = self.conversation.lock().await;
        conv.clone()
    }
    
    /// Get current status
    pub async fn get_status(&self) -> SubAgentStatus {
        let status = self.status.read().await;
        status.clone()
    }
    
    fn should_terminate(&self, message: &Message, turn: usize) -> bool {
        if turn >= self.config.max_turns {
            return true;
        }
        
        let content = message.as_concat_text().to_lowercase();
        for phrase in &self.config.auto_terminate_phrases {
            if content.contains(phrase) {
                return true;
            }
        }
        
        false
    }
}

/// Handle for communicating with a subagent (simplified)
pub struct SubAgentHandle {
    pub id: String,
    command_tx: mpsc::Sender<SubAgentCommand>,
    response_rx: Arc<Mutex<mpsc::Receiver<SubAgentResponse>>>,
    progress_rx: Arc<Mutex<mpsc::Receiver<SubAgentProgress>>>,
}

/// Command sent to a subagent (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubAgentCommand {
    Start { message: String },
    GetStatus,
    GetConversation,
    Terminate,
}

/// Response from a subagent (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubAgentResponse {
    Started,
    Status(SubAgentStatus),
    Conversation(Vec<Message>),
    Completed(Vec<Message>),
    Error(String),
}

impl SubAgentHandle {
    /// Receive progress update (non-blocking)
    pub async fn try_recv_progress(&self) -> Option<SubAgentProgress> {
        let mut rx = self.progress_rx.lock().await;
        rx.try_recv().ok()
    }
}