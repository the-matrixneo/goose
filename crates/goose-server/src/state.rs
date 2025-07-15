use goose::agents::Agent;
use goose::scheduler_trait::SchedulerTrait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AgentRef = Arc<Agent>;

#[derive(Clone)]
pub struct AppState {
    agent: Option<AgentRef>,
    pub secret_key: String,
    pub scheduler: Arc<Mutex<Option<Arc<dyn SchedulerTrait>>>>,
    pub current_session_id: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub async fn new(agent: AgentRef, secret_key: String) -> Arc<AppState> {
        Arc::new(Self {
            agent: Some(agent.clone()),
            secret_key,
            scheduler: Arc::new(Mutex::new(None)),
            current_session_id: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn get_agent(&self) -> Result<Arc<Agent>, anyhow::Error> {
        self.agent
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Agent needs to be created first."))
    }

    pub async fn set_scheduler(&self, sched: Arc<dyn SchedulerTrait>) {
        let mut guard = self.scheduler.lock().await;
        *guard = Some(sched);
    }

    pub async fn scheduler(&self) -> Result<Arc<dyn SchedulerTrait>, anyhow::Error> {
        self.scheduler
            .lock()
            .await
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Scheduler not initialized"))
    }

    /// Check if this is a new session and reset agent state if needed
    /// Returns true if the session changed
    pub async fn handle_session_change(&self, new_session_id: &str) -> Result<bool, anyhow::Error> {
        let mut current_session = self.current_session_id.lock().await;
        
        // Check if this is a different session
        let session_changed = match current_session.as_ref() {
            Some(current_id) => current_id != new_session_id,
            None => true, // First session
        };

        if session_changed {
            tracing::info!("Session change detected: {:?} -> {}", current_session.as_ref(), new_session_id);
            
            // Reset agent state for session isolation
            if let Ok(agent) = self.get_agent().await {
                if let Err(e) = agent.reset_session_state().await {
                    tracing::error!("Failed to reset agent session state: {}", e);
                    return Err(e);
                }
            }

            // Update current session
            *current_session = Some(new_session_id.to_string());
        }

        Ok(session_changed)
    }
}
