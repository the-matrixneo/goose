use crate::agents::sub_recipe_execution_tool::tasks::process_task;
use crate::agents::sub_recipe_execution_tool::types::{SharedState, Task};
use std::sync::Arc;

async fn receive_task(state: &SharedState) -> Option<Task> {
    let mut receiver = state.task_receiver.lock().await;
    receiver.recv().await
}

pub fn spawn_worker(
    state: Arc<SharedState>,
    worker_id: usize,
    timeout_seconds: u64,
) -> tokio::task::JoinHandle<()> {
    state.increment_active_workers();

    tokio::spawn(async move {
        worker_loop(state, worker_id, timeout_seconds).await;
    })
}

async fn worker_loop(state: Arc<SharedState>, _worker_id: usize, timeout_seconds: u64) {
    while let Some(task) = receive_task(&state).await {
        state.dashboard.start_task(&task.id).await;
        let result = process_task(&task, timeout_seconds, state.dashboard.clone()).await;

        if let Err(e) = state.result_sender.send(result).await {
            eprintln!("Worker failed to send result: {}", e);
            break;
        }
    }

    state.decrement_active_workers();
}
