use crate::agents::sub_recipe_execution_tool::dashboard::TaskDashboard;
use crate::agents::sub_recipe_execution_tool::tasks::{process_task, process_task_with_dashboard};
use crate::agents::sub_recipe_execution_tool::types::{SharedState, Task, TaskResult};
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use tokio::sync::mpsc;

    use super::*;
    use crate::agents::sub_recipe_execution_tool::types::Task;

    #[tokio::test]
    async fn test_spawn_worker_returns_handle() {
        // Create a simple shared state for testing
        let (task_tx, task_rx) = mpsc::channel::<Task>(1);
        let (result_tx, _result_rx) = mpsc::channel::<TaskResult>(1);

        let shared_state = Arc::new(SharedState {
            task_receiver: Arc::new(tokio::sync::Mutex::new(task_rx)),
            result_sender: result_tx,
            active_workers: Arc::new(AtomicUsize::new(0)),
            dashboard: None,
        });

        // Test that spawn_worker returns a JoinHandle
        let handle = spawn_worker(shared_state.clone(), 0, 5);

        // Verify it's a JoinHandle by checking we can abort it
        assert!(!handle.is_finished());

        // Signal stop and close the channel to let the worker exit
        drop(task_tx); // Close the channel

        // Wait for the worker to finish
        let result = handle.await;
        assert!(result.is_ok());
    }
}

async fn receive_task(state: &SharedState) -> Option<Task> {
    let mut receiver = state.task_receiver.lock().await;
    receiver.recv().await
}

async fn execute_task(
    task: Task,
    timeout: u64,
    dashboard: Option<Arc<TaskDashboard>>,
) -> TaskResult {
    if let Some(dashboard) = &dashboard {
        dashboard.start_task(&task.id).await;
    }

    if let Some(dashboard) = dashboard {
        process_task_with_dashboard(&task, timeout, Some(dashboard)).await
    } else {
        process_task(&task, timeout).await
    }
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
        let result = execute_task(task, timeout_seconds, state.dashboard.clone()).await;

        if let Err(e) = state.result_sender.send(result).await {
            eprintln!("Worker failed to send result: {}", e);
            break;
        }
    }

    state.decrement_active_workers();
}
