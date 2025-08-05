use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    Wip,
    Done,
}

impl TaskStatus {
    fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "to do",
            TaskStatus::Wip => "wip",
            TaskStatus::Done => "done",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskTracker {
    tasks: Arc<Mutex<HashMap<String, TaskStatus>>>,
}

impl TaskTracker {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn list_tasks(&self) -> Vec<String> {
        let tasks = self.tasks.lock().unwrap();
        let mut task_list = Vec::new();
        for (task, status) in tasks.iter() {
            task_list.push(format!("{} [{}]", task, status.as_str()));
        }
        task_list
    }

    pub fn add_task(&self, task: String) -> String {
        if task.len() > 200 {
            return "Task description is too long (max 200 chars)".to_string();
        }
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task.clone(), TaskStatus::Todo);
        format!("Added task: {}", task)
    }

    pub fn mark_task_wip(&self, task: String) -> String {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(&task) {
            tasks.insert(task.clone(), TaskStatus::Wip);
            format!("Marked as WIP: {}", task)
        } else {
            format!("Task not found: {}", task)
        }
    }

    pub fn mark_task_done(&self, task: String) -> String {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(&task) {
            tasks.insert(task.clone(), TaskStatus::Done);
            format!("Marked as done: {}", task)
        } else {
            format!("Task not found: {}", task)
        }
    }

    pub fn clear_tasks(&self) -> String {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.clear();
        "All tasks cleared".to_string()
    }
}

impl Default for TaskTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_task_list() {
        let tracker = TaskTracker::new();
        let tasks = tracker.list_tasks();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn test_add_task() {
        let tracker = TaskTracker::new();
        tracker.add_task("Write unit tests".to_string());

        let tasks = tracker.list_tasks();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].contains("Write unit tests [to do]"));
    }

    #[tokio::test]
    async fn test_mark_task_wip() {
        let tracker = TaskTracker::new();

        // Add task first
        tracker.add_task("Fix bug".to_string());

        // Mark as WIP
        tracker.mark_task_wip("Fix bug".to_string());

        let tasks = tracker.list_tasks();
        assert!(tasks[0].contains("Fix bug [wip]"));
    }

    #[tokio::test]
    async fn test_mark_task_done() {
        let tracker = TaskTracker::new();

        // Add task first
        tracker.add_task("Review PR".to_string());

        // Mark as done
        tracker.mark_task_done("Review PR".to_string());

        let tasks = tracker.list_tasks();
        assert!(tasks[0].contains("Review PR [done]"));
    }

    #[tokio::test]
    async fn test_clear_tasks() {
        let tracker = TaskTracker::new();

        // Add some tasks
        tracker.add_task("Task 1".to_string());
        tracker.add_task("Task 2".to_string());

        // Clear all
        tracker.clear_tasks();

        let tasks = tracker.list_tasks();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn test_long_task_not_added() {
        let tracker = TaskTracker::new();
        let long_task = "a".repeat(201);

        tracker.add_task(long_task);

        let tasks = tracker.list_tasks();
        assert!(tasks.is_empty()); // Task should not be added because it's too long
    }

    #[tokio::test]
    async fn test_mark_nonexistent_task() {
        let tracker = TaskTracker::new();

        // Try to mark non-existent task as WIP
        tracker.mark_task_wip("Non-existent task".to_string());

        let tasks = tracker.list_tasks();
        assert!(tasks.is_empty()); // No task should exist
    }
}
