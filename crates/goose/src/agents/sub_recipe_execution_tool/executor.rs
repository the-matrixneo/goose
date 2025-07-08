use mcp_core::protocol::JsonRpcMessage;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::agents::sub_recipe_execution_tool::dashboard::{DisplayMode, TaskDashboard};
use crate::agents::sub_recipe_execution_tool::lib::{
    Config, ExecutionResponse, ExecutionStats, SharedState, Task, TaskResult, TaskStatus,
};
use crate::agents::sub_recipe_execution_tool::tasks::process_task;
use crate::agents::sub_recipe_execution_tool::workers::spawn_worker;

const EXECUTION_STATUS_COMPLETED: &str = "completed";

pub async fn execute_single_task(
    task: &Task,
    config: Config,
    notifier: mpsc::Sender<JsonRpcMessage>,
) -> ExecutionResponse {
    let start_time = Instant::now();
    let dashboard = Arc::new(TaskDashboard::new(
        vec![task.clone()],
        DisplayMode::SingleTaskOutput,
        notifier,
    ));
    let result = process_task(task, config.timeout_seconds, dashboard).await;
    let execution_time = start_time.elapsed().as_millis();
    let stats = calculate_stats(&[result.clone()], execution_time);

    ExecutionResponse {
        status: EXECUTION_STATUS_COMPLETED.to_string(),
        results: vec![result],
        stats,
    }
}

pub async fn execute_tasks_in_parallel(
    tasks: Vec<Task>,
    config: Config,
    notifier: mpsc::Sender<JsonRpcMessage>,
) -> ExecutionResponse {
    let dashboard = Arc::new(TaskDashboard::new(
        tasks.clone(),
        DisplayMode::Dashboard,
        notifier,
    ));
    let start_time = Instant::now();
    let task_count = tasks.len();

    if task_count == 0 {
        return create_empty_response();
    }

    dashboard.refresh_display().await;

    let (task_tx, task_rx, result_tx, mut result_rx) = create_channels(task_count);

    if let Err(e) = send_tasks_to_channel(tasks, task_tx).await {
        eprintln!("Execution failed: {}", e);
        return create_error_response(e);
    }

    let shared_state = create_shared_state(task_rx, result_tx, dashboard.clone());

    // Simple static worker allocation - no dynamic scaling needed
    let worker_count = std::cmp::min(task_count, config.max_workers);
    let mut worker_handles = Vec::new();
    for i in 0..worker_count {
        let handle = spawn_worker(shared_state.clone(), i, config.timeout_seconds);
        worker_handles.push(handle);
    }

    let results = collect_results(&mut result_rx, dashboard.clone(), task_count).await;

    for handle in worker_handles {
        if let Err(e) = handle.await {
            eprintln!("Worker error: {}", e);
        }
    }

    dashboard.show_final_summary().await;

    let execution_time = start_time.elapsed().as_millis();
    let stats = calculate_stats(&results, execution_time);

    ExecutionResponse {
        status: EXECUTION_STATUS_COMPLETED.to_string(),
        results,
        stats,
    }
}

fn calculate_stats(results: &[TaskResult], execution_time_ms: u128) -> ExecutionStats {
    let completed = results
        .iter()
        .filter(|r| matches!(r.status, TaskStatus::Completed))
        .count();
    let failed = results
        .iter()
        .filter(|r| matches!(r.status, TaskStatus::Failed))
        .count();

    ExecutionStats {
        total_tasks: results.len(),
        completed,
        failed,
        execution_time_ms,
    }
}

fn create_channels(
    task_count: usize,
) -> (
    mpsc::Sender<Task>,
    mpsc::Receiver<Task>,
    mpsc::Sender<TaskResult>,
    mpsc::Receiver<TaskResult>,
) {
    let (task_tx, task_rx) = mpsc::channel::<Task>(task_count);
    let (result_tx, result_rx) = mpsc::channel::<TaskResult>(task_count);
    (task_tx, task_rx, result_tx, result_rx)
}

fn create_shared_state(
    task_rx: mpsc::Receiver<Task>,
    result_tx: mpsc::Sender<TaskResult>,
    dashboard: Arc<TaskDashboard>,
) -> Arc<SharedState> {
    Arc::new(SharedState {
        task_receiver: Arc::new(tokio::sync::Mutex::new(task_rx)),
        result_sender: result_tx,
        active_workers: Arc::new(AtomicUsize::new(0)),
        dashboard,
    })
}

async fn send_tasks_to_channel(
    tasks: Vec<Task>,
    task_tx: mpsc::Sender<Task>,
) -> Result<(), String> {
    for task in tasks {
        task_tx
            .send(task)
            .await
            .map_err(|e| format!("Failed to queue task: {}", e))?;
    }
    Ok(())
}

fn create_empty_response() -> ExecutionResponse {
    ExecutionResponse {
        status: EXECUTION_STATUS_COMPLETED.to_string(),
        results: vec![],
        stats: ExecutionStats {
            total_tasks: 0,
            completed: 0,
            failed: 0,
            execution_time_ms: 0,
        },
    }
}

async fn collect_results(
    result_rx: &mut mpsc::Receiver<TaskResult>,
    dashboard: Arc<TaskDashboard>,
    expected_count: usize,
) -> Vec<TaskResult> {
    let mut results = Vec::new();
    while let Some(result) = result_rx.recv().await {
        dashboard
            .complete_task(&result.task_id, result.clone())
            .await;
        results.push(result);
        if results.len() >= expected_count {
            break;
        }
    }
    results
}

fn create_error_response(_error: String) -> ExecutionResponse {
    ExecutionResponse {
        status: "failed".to_string(),
        results: vec![],
        stats: ExecutionStats {
            total_tasks: 0,
            completed: 0,
            failed: 1,
            execution_time_ms: 0,
        },
    }
}
