use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::agents::sub_recipe_execution_tool::lib::{
    Config, ExecutionResponse, ExecutionStats, Task, TaskResult,
};
use crate::agents::sub_recipe_execution_tool::tasks::process_task;
use crate::agents::sub_recipe_execution_tool::workers::{run_scaler, spawn_worker, SharedState};

pub async fn execute_single_task(task: &Task, global_config: Config) -> ExecutionResponse {
    let start_time = Instant::now();

    // Extract config from task payload if present and merge with global config
    let effective_config = if let Some(task_config_value) = task.payload.get("config") {
        let mut merged_config = global_config.clone();

        if let Some(timeout) = task_config_value
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
        {
            merged_config.timeout_seconds = timeout;
        }
        if let Some(max_workers) = task_config_value
            .get("max_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.max_workers = max_workers as usize;
        }
        if let Some(initial_workers) = task_config_value
            .get("initial_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.initial_workers = initial_workers as usize;
        }

        merged_config
    } else {
        global_config
    };

    let result = process_task(task, effective_config.timeout_seconds).await;

    let execution_time = start_time.elapsed().as_millis();
    let completed = if result.status == "success" { 1 } else { 0 };
    let failed = if result.status == "failed" { 1 } else { 0 };

    ExecutionResponse {
        status: "completed".to_string(),
        results: vec![result],
        stats: ExecutionStats {
            total_tasks: 1,
            completed,
            failed,
            execution_time_ms: execution_time,
        },
    }
}

// Main parallel execution function
pub async fn parallel_execute(tasks: Vec<Task>, config: Config) -> ExecutionResponse {
    let start_time = Instant::now();
    let task_count = tasks.len();

    // Create channels
    let (task_tx, task_rx) = mpsc::channel::<Task>(task_count);
    let (result_tx, mut result_rx) = mpsc::channel::<TaskResult>(task_count);

    // Initialize shared state
    let shared_state = Arc::new(SharedState {
        task_receiver: Arc::new(tokio::sync::Mutex::new(task_rx)),
        result_sender: result_tx,
        active_workers: Arc::new(AtomicUsize::new(0)),
        should_stop: Arc::new(AtomicBool::new(false)),
        completed_tasks: Arc::new(AtomicUsize::new(0)),
    });

    // Send all tasks to the queue
    for task in tasks.clone() {
        let _ = task_tx.send(task).await;
    }
    // Close sender so workers know when queue is empty
    drop(task_tx);

    // Start initial workers
    let mut worker_handles = Vec::new();
    for i in 0..config.initial_workers {
        let handle = spawn_worker(shared_state.clone(), i, config.timeout_seconds);
        worker_handles.push(handle);
    }

    // Start the scaler
    let scaler_state = shared_state.clone();
    let scaler_handle = tokio::spawn(async move {
        run_scaler(
            scaler_state,
            task_count,
            config.max_workers,
            config.timeout_seconds,
        )
        .await;
    });

    // Collect results
    let mut results = Vec::new();
    while let Some(result) = result_rx.recv().await {
        results.push(result);
        if results.len() >= task_count {
            break;
        }
    }

    // Wait for scaler to finish
    let _ = scaler_handle.await;

    // Calculate stats
    let execution_time = start_time.elapsed().as_millis();
    let completed = results.iter().filter(|r| r.status == "success").count();
    let failed = results.iter().filter(|r| r.status == "failed").count();

    ExecutionResponse {
        status: "completed".to_string(),
        results,
        stats: ExecutionStats {
            total_tasks: task_count,
            completed,
            failed,
            execution_time_ms: execution_time,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_execute_single_task_with_config_override() {
        // Create a task with config override
        let task = Task {
            id: "test_task".to_string(),
            task_type: "text_instruction".to_string(),
            payload: json!({
                "text_instruction": "echo 'test'",
                "config": {
                    "timeout_seconds": 600,
                    "max_workers": 8,
                    "initial_workers": 4
                }
            }),
        };

        // Global config with different values
        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        // Execute the task
        let response = execute_single_task(&task, global_config).await;

        // Verify the response structure
        assert_eq!(response.status, "completed");
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.stats.total_tasks, 1);
        
        // Note: We can't directly verify the timeout was applied since process_task
        // is called internally, but we can verify the function completes without error
        // and that the config merging logic doesn't panic
    }

    #[tokio::test]
    async fn test_execute_single_task_without_config_override() {
        // Create a task without config override
        let task = Task {
            id: "test_task".to_string(),
            task_type: "text_instruction".to_string(),
            payload: json!({
                "text_instruction": "echo 'test'"
            }),
        };

        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        // Execute the task
        let response = execute_single_task(&task, global_config).await;

        // Verify the response structure
        assert_eq!(response.status, "completed");
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.stats.total_tasks, 1);
    }

    #[tokio::test]
    async fn test_execute_single_task_partial_config_override() {
        // Create a task with partial config override (only timeout)
        let task = Task {
            id: "test_task".to_string(),
            task_type: "text_instruction".to_string(),
            payload: json!({
                "text_instruction": "echo 'test'",
                "config": {
                    "timeout_seconds": 900
                    // max_workers and initial_workers not specified
                }
            }),
        };

        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        // Execute the task
        let response = execute_single_task(&task, global_config).await;

        // Verify the response structure
        assert_eq!(response.status, "completed");
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.stats.total_tasks, 1);
    }

    #[test]
    fn test_config_merging_logic() {
        // Test the config merging logic directly
        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        let task_config_value = json!({
            "timeout_seconds": 600,
            "max_workers": 8,
            "initial_workers": 4
        });

        // Simulate the config merging logic from execute_single_task
        let mut merged_config = global_config.clone();

        if let Some(timeout) = task_config_value
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
        {
            merged_config.timeout_seconds = timeout;
        }
        if let Some(max_workers) = task_config_value
            .get("max_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.max_workers = max_workers as usize;
        }
        if let Some(initial_workers) = task_config_value
            .get("initial_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.initial_workers = initial_workers as usize;
        }

        // Verify the merged config has the overridden values
        assert_eq!(merged_config.timeout_seconds, 600);
        assert_eq!(merged_config.max_workers, 8);
        assert_eq!(merged_config.initial_workers, 4);
    }

    #[test]
    fn test_config_merging_with_partial_override() {
        // Test partial config override
        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        let task_config_value = json!({
            "timeout_seconds": 600
            // Only timeout is overridden
        });

        // Simulate the config merging logic
        let mut merged_config = global_config.clone();

        if let Some(timeout) = task_config_value
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
        {
            merged_config.timeout_seconds = timeout;
        }
        if let Some(max_workers) = task_config_value
            .get("max_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.max_workers = max_workers as usize;
        }
        if let Some(initial_workers) = task_config_value
            .get("initial_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.initial_workers = initial_workers as usize;
        }

        // Verify only timeout was overridden, others remain from global config
        assert_eq!(merged_config.timeout_seconds, 600); // Overridden
        assert_eq!(merged_config.max_workers, 5);        // From global config
        assert_eq!(merged_config.initial_workers, 2);    // From global config
    }

    #[test]
    fn test_config_merging_with_invalid_types() {
        // Test that invalid types are ignored gracefully
        let global_config = Config {
            timeout_seconds: 300,
            max_workers: 5,
            initial_workers: 2,
        };

        let task_config_value = json!({
            "timeout_seconds": "invalid_string", // Invalid type
            "max_workers": 8,
            "initial_workers": null // Invalid type
        });

        // Simulate the config merging logic
        let mut merged_config = global_config.clone();

        if let Some(timeout) = task_config_value
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
        {
            merged_config.timeout_seconds = timeout;
        }
        if let Some(max_workers) = task_config_value
            .get("max_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.max_workers = max_workers as usize;
        }
        if let Some(initial_workers) = task_config_value
            .get("initial_workers")
            .and_then(|v| v.as_u64())
        {
            merged_config.initial_workers = initial_workers as usize;
        }

        // Verify invalid types are ignored, valid ones are applied
        assert_eq!(merged_config.timeout_seconds, 300); // Invalid, so global value retained
        assert_eq!(merged_config.max_workers, 8);        // Valid, so overridden
        assert_eq!(merged_config.initial_workers, 2);    // Invalid, so global value retained
    }
}
