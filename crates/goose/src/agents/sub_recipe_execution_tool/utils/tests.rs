#[cfg(test)]
mod tests {
    use crate::agents::sub_recipe_execution_tool::types::{Task, TaskInfo, TaskStatus};
    use crate::agents::sub_recipe_execution_tool::utils::{
        count_by_status, get_task_name, strip_ansi_codes, truncate_with_ellipsis,
    };
    use serde_json::json;
    use std::collections::HashMap;

    mod truncate_with_ellipsis {
        use super::*;

        #[test]
        fn returns_original_when_under_limit() {
            assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
            assert_eq!(truncate_with_ellipsis("hi", 5), "hi");
        }

        #[test]
        fn truncates_when_over_limit() {
            assert_eq!(truncate_with_ellipsis("hello world", 5), "he...");
            assert_eq!(
                truncate_with_ellipsis("very long text here", 10),
                "very lo..."
            );
        }

        #[test]
        fn handles_empty_string() {
            assert_eq!(truncate_with_ellipsis("", 5), "");
        }

        #[test]
        fn handles_exact_limit() {
            assert_eq!(truncate_with_ellipsis("hello", 5), "hello");
        }

        #[test]
        fn handles_very_short_limit() {
            assert_eq!(truncate_with_ellipsis("hello", 3), "...");
            assert_eq!(truncate_with_ellipsis("hi", 2), "hi"); // Under limit, return as-is
        }
    }

    mod strip_ansi_codes {
        use super::*;

        #[test]
        fn preserves_plain_text() {
            assert_eq!(strip_ansi_codes("hello world"), "hello world");
            assert_eq!(strip_ansi_codes("no ansi codes"), "no ansi codes");
        }

        #[test]
        fn removes_color_codes() {
            assert_eq!(strip_ansi_codes("\x1b[31mred text\x1b[0m"), "red text");
            assert_eq!(strip_ansi_codes("\x1b[32mgreen\x1b[0m"), "green");
        }

        #[test]
        fn removes_complex_formatting() {
            assert_eq!(
                strip_ansi_codes("\x1b[1;32mbold green\x1b[0m"),
                "bold green"
            );
            assert_eq!(
                strip_ansi_codes("\x1b[4;31munderline red\x1b[0m"),
                "underline red"
            );
        }

        #[test]
        fn handles_multiple_sequences() {
            let input = "\x1b[31mred\x1b[0m normal \x1b[32mgreen\x1b[0m";
            assert_eq!(strip_ansi_codes(input), "red normal green");
        }

        #[test]
        fn handles_empty_string() {
            assert_eq!(strip_ansi_codes(""), "");
        }

        #[test]
        fn handles_malformed_sequences() {
            // Incomplete escape sequence - our function strips the \x1b char
            assert_eq!(strip_ansi_codes("\x1b hello"), "hello");
        }
    }

    mod get_task_name {
        use super::*;

        #[test]
        fn extracts_sub_recipe_name() {
            let sub_recipe_task = Task {
                id: "task_1".to_string(),
                task_type: "sub_recipe".to_string(),
                payload: json!({
                    "sub_recipe": {
                        "name": "my_recipe",
                        "recipe_path": "/path/to/recipe"
                    }
                }),
            };

            let task_info = TaskInfo {
                task: sub_recipe_task,
                status: TaskStatus::Pending,
                start_time: None,
                end_time: None,
                result: None,
                current_output: String::new(),
            };

            assert_eq!(get_task_name(&task_info), "my_recipe");
        }

        #[test]
        fn falls_back_to_task_id_for_text_instruction() {
            let text_task = Task {
                id: "task_2".to_string(),
                task_type: "text_instruction".to_string(),
                payload: json!({"text_instruction": "do something"}),
            };

            let task_info = TaskInfo {
                task: text_task,
                status: TaskStatus::Pending,
                start_time: None,
                end_time: None,
                result: None,
                current_output: String::new(),
            };

            assert_eq!(get_task_name(&task_info), "task_2");
        }

        #[test]
        fn falls_back_to_task_id_when_sub_recipe_name_missing() {
            let malformed_task = Task {
                id: "task_3".to_string(),
                task_type: "sub_recipe".to_string(),
                payload: json!({
                    "sub_recipe": {
                        "recipe_path": "/path/to/recipe"
                        // missing "name" field
                    }
                }),
            };

            let task_info = TaskInfo {
                task: malformed_task,
                status: TaskStatus::Pending,
                start_time: None,
                end_time: None,
                result: None,
                current_output: String::new(),
            };

            assert_eq!(get_task_name(&task_info), "task_3");
        }

        #[test]
        fn falls_back_to_task_id_when_sub_recipe_missing() {
            let malformed_task = Task {
                id: "task_4".to_string(),
                task_type: "sub_recipe".to_string(),
                payload: json!({}), // missing "sub_recipe" field
            };

            let task_info = TaskInfo {
                task: malformed_task,
                status: TaskStatus::Pending,
                start_time: None,
                end_time: None,
                result: None,
                current_output: String::new(),
            };

            assert_eq!(get_task_name(&task_info), "task_4");
        }
    }

    mod count_by_status {
        use super::*;

        fn create_test_task(id: &str, status: TaskStatus) -> TaskInfo {
            TaskInfo {
                task: Task {
                    id: id.to_string(),
                    task_type: "test".to_string(),
                    payload: json!({}),
                },
                status,
                start_time: None,
                end_time: None,
                result: None,
                current_output: String::new(),
            }
        }

        #[test]
        fn counts_empty_map() {
            let tasks = HashMap::new();
            let (total, pending, running, completed, failed) = count_by_status(&tasks);
            assert_eq!(
                (total, pending, running, completed, failed),
                (0, 0, 0, 0, 0)
            );
        }

        #[test]
        fn counts_single_status() {
            let mut tasks = HashMap::new();
            tasks.insert(
                "task1".to_string(),
                create_test_task("task1", TaskStatus::Pending),
            );
            tasks.insert(
                "task2".to_string(),
                create_test_task("task2", TaskStatus::Pending),
            );

            let (total, pending, running, completed, failed) = count_by_status(&tasks);
            assert_eq!(
                (total, pending, running, completed, failed),
                (2, 2, 0, 0, 0)
            );
        }

        #[test]
        fn counts_mixed_statuses() {
            let mut tasks = HashMap::new();
            tasks.insert(
                "task1".to_string(),
                create_test_task("task1", TaskStatus::Pending),
            );
            tasks.insert(
                "task2".to_string(),
                create_test_task("task2", TaskStatus::Running),
            );
            tasks.insert(
                "task3".to_string(),
                create_test_task("task3", TaskStatus::Completed),
            );
            tasks.insert(
                "task4".to_string(),
                create_test_task("task4", TaskStatus::Failed),
            );
            tasks.insert(
                "task5".to_string(),
                create_test_task("task5", TaskStatus::Completed),
            );

            let (total, pending, running, completed, failed) = count_by_status(&tasks);
            assert_eq!(
                (total, pending, running, completed, failed),
                (5, 1, 1, 2, 1)
            );
        }
    }
}
