use goose::agents::Agent;
use goose::scheduler_trait::SchedulerTrait;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

type AgentRef = Arc<Agent>;

#[derive(Clone)]
pub struct AppState {
    agent: Arc<Mutex<AgentRef>>,
    pub secret_key: String,
    pub scheduler: Arc<Mutex<Option<Arc<dyn SchedulerTrait>>>>,
    pub session_counter: Arc<AtomicUsize>,
}

impl AppState {
    pub async fn new(agent: AgentRef, secret_key: String) -> Arc<AppState> {
        Arc::new(Self {
            agent: Arc::new(Mutex::new(agent)),
            secret_key,
            scheduler: Arc::new(Mutex::new(None)),
            session_counter: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub async fn get_agent(&self) -> AgentRef {
        self.agent.lock().await.clone()
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

    /// Reset the app state
    /// after we make goosed multi-agent, we shouldn't need this anymore
    pub async fn reset(&self) {
        let mut agent = self.agent.lock().await;
        *agent = Arc::new(Agent::new());

        {
            let mut scheduler = self.scheduler.lock().await;
            *scheduler = None;
        }

        // Reset session counter
        self.session_counter.store(0, Ordering::SeqCst);
    }
}
