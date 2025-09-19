use goose::agents::manager::{AgentManager, AgentManagerConfig};
use goose::agents::Agent;
use goose::scheduler_trait::SchedulerTrait;
use goose::session;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

type AgentRef = Arc<Agent>;

#[derive(Clone)]
pub struct AppState {
    agent_manager: Arc<AgentManager>,
    pub scheduler: Arc<RwLock<Option<Arc<dyn SchedulerTrait>>>>,
    pub recipe_file_hash_map: Arc<Mutex<HashMap<String, PathBuf>>>,
    pub session_counter: Arc<AtomicUsize>,
    /// Tracks sessions that have already emitted recipe telemetry to prevent double counting.
    recipe_session_tracker: Arc<Mutex<HashSet<String>>>,
}

impl AppState {
    pub async fn new() -> Arc<AppState> {
        let agent_manager = Arc::new(AgentManager::new(AgentManagerConfig::default()));

        // Spawn the cleanup task
        agent_manager.clone().spawn_cleanup_task().await;

        Arc::new(Self {
            agent_manager,
            scheduler: Arc::new(RwLock::new(None)),
            recipe_file_hash_map: Arc::new(Mutex::new(HashMap::new())),
            session_counter: Arc::new(AtomicUsize::new(0)),
            recipe_session_tracker: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    pub async fn get_agent(
        &self,
        session_id: session::Identifier,
    ) -> Result<AgentRef, anyhow::Error> {
        self.agent_manager
            .get_agent(session_id)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get agent: {}", e))
    }

    pub async fn set_scheduler(&self, sched: Arc<dyn SchedulerTrait>) {
        let mut guard = self.scheduler.write().await;
        *guard = Some(sched);
    }

    pub async fn scheduler(&self) -> Result<Arc<dyn SchedulerTrait>, anyhow::Error> {
        self.scheduler
            .read()
            .await
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Scheduler not initialized"))
    }

    pub async fn set_recipe_file_hash_map(&self, hash_map: HashMap<String, PathBuf>) {
        let mut map = self.recipe_file_hash_map.lock().await;
        *map = hash_map;
    }

    pub async fn cleanup_idle_agents(&self) -> Result<usize, anyhow::Error> {
        self.agent_manager
            .cleanup()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to cleanup agents: {}", e))
    }

    pub async fn get_agent_metrics(&self) -> goose::agents::manager::AgentMetrics {
        self.agent_manager.get_metrics().await
    }

    #[allow(dead_code)] // Will be used when server graceful shutdown is implemented
    pub async fn shutdown(&self) {
        self.agent_manager.shutdown().await;
    }

    pub async fn mark_recipe_run_if_absent(&self, session_id: &str) -> bool {
        let mut sessions = self.recipe_session_tracker.lock().await;
        if sessions.contains(session_id) {
            false
        } else {
            sessions.insert(session_id.to_string());
            true
        }
    }
}
