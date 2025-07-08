use crate::agents::sub_recipe_execution_tool::types::{Task, TaskInfo, TaskStatus};
use crate::agents::sub_recipe_execution_tool::utils::{
    count_by_status, format_task_display, format_task_error, format_task_output,
    format_task_timing, get_status_icon, get_task_name, process_output_lines, strip_ansi_codes,
    truncate_with_ellipsis,
};
use serde_json::json;
use std::collections::HashMap;
use tokio::time::Instant;

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

    mod get_status_icon {
        use super::*;

        #[test]
        fn returns_correct_icon_for_pending() {
            assert_eq!(get_status_icon(&TaskStatus::Pending), "⏳");
        }

        #[test]
        fn returns_correct_icon_for_running() {
            assert_eq!(get_status_icon(&TaskStatus::Running), "🏃");
        }

        #[test]
        fn returns_correct_icon_for_completed() {
            assert_eq!(get_status_icon(&TaskStatus::Completed), "✅");
        }

        #[test]
        fn returns_correct_icon_for_failed() {
            assert_eq!(get_status_icon(&TaskStatus::Failed), "❌");
        }
    }

    mod process_output_lines {
        use super::*;

        #[test]
        fn preserves_short_output() {
            let output = "line 1\nline 2";
            assert_eq!(process_output_lines(output), "line 1\nline 2");
        }

        #[test]
        fn keeps_only_recent_lines_when_too_many() {
            let output = "line 1\nline 2\nline 3\nline 4\nline 5";
            let result = process_output_lines(output);
            assert_eq!(result, "line 4\nline 5");
        }

        #[test]
        fn strips_ansi_codes_from_output() {
            let output = "\x1b[31mred line 1\x1b[0m\n\x1b[32mgreen line 2\x1b[0m";
            let result = process_output_lines(output);
            assert_eq!(result, "red line 1\ngreen line 2");
        }

        #[test]
        fn handles_empty_output() {
            assert_eq!(process_output_lines(""), "");
        }

        #[test]
        fn handles_single_line() {
            assert_eq!(process_output_lines("single line"), "single line");
        }

        #[test]
        fn combines_ansi_stripping_and_line_limiting() {
            let output = "\x1b[31mline 1\x1b[0m\n\x1b[32mline 2\x1b[0m\n\x1b[33mline 3\x1b[0m\n\x1b[34mline 4\x1b[0m";
            let result = process_output_lines(output);
            assert_eq!(result, "line 3\nline 4");
        }
    }

    mod format_task_timing {
        use super::*;
        use std::time::Duration;

        fn create_test_task_info_with_timing(
            start: Option<Instant>,
            end: Option<Instant>,
        ) -> TaskInfo {
            TaskInfo {
                task: Task {
                    id: "test_task".to_string(),
                    task_type: "test".to_string(),
                    payload: json!({}),
                },
                status: TaskStatus::Running,
                start_time: start,
                end_time: end,
                result: None,
                current_output: String::new(),
            }
        }

        #[test]
        fn returns_none_when_no_start_time() {
            let task_info = create_test_task_info_with_timing(None, None);
            let current_time = Instant::now();
            assert!(format_task_timing(&task_info, current_time).is_none());
        }

        #[test]
        fn formats_running_task_duration() {
            let start_time = Instant::now();
            let current_time = start_time + Duration::from_millis(1500);
            let task_info = create_test_task_info_with_timing(Some(start_time), None);
            
            let result = format_task_timing(&task_info, current_time).unwrap();
            assert!(result.contains("1.5s"));
            assert!(result.contains("⏱️"));
        }

        #[test]
        fn formats_completed_task_duration() {
            let start_time = Instant::now();
            let end_time = start_time + Duration::from_millis(2500);
            let current_time = Instant::now(); // This shouldn't matter for completed tasks
            let task_info = create_test_task_info_with_timing(Some(start_time), Some(end_time));
            
            let result = format_task_timing(&task_info, current_time).unwrap();
            assert!(result.contains("2.5s"));
            assert!(result.contains("⏱️"));
        }
    }

    mod format_task_output {
        use super::*;

        fn create_test_task_info_with_output(status: TaskStatus, output: &str) -> TaskInfo {
            TaskInfo {
                task: Task {
                    id: "test_task".to_string(),
                    task_type: "test".to_string(),
                    payload: json!({}),
                },
                status,
                start_time: None,
                end_time: None,
                result: None,
                current_output: output.to_string(),
            }
        }

        #[test]
        fn returns_none_for_non_running_tasks() {
            let task_info = create_test_task_info_with_output(TaskStatus::Pending, "some output");
            assert!(format_task_output(&task_info).is_none());

            let task_info = create_test_task_info_with_output(TaskStatus::Completed, "some output");
            assert!(format_task_output(&task_info).is_none());

            let task_info = create_test_task_info_with_output(TaskStatus::Failed, "some output");
            assert!(format_task_output(&task_info).is_none());
        }

        #[test]
        fn returns_none_for_running_task_with_empty_output() {
            let task_info = create_test_task_info_with_output(TaskStatus::Running, "");
            assert!(format_task_output(&task_info).is_none());
        }

        #[test]
        fn formats_running_task_with_output() {
            let task_info = create_test_task_info_with_output(TaskStatus::Running, "Building project...");
            let result = format_task_output(&task_info).unwrap();
            
            assert!(result.contains("💬"));
            assert!(result.contains("Building project..."));
        }

        #[test]
        fn replaces_newlines_with_pipes() {
            let task_info = create_test_task_info_with_output(TaskStatus::Running, "line 1\nline 2\nline 3");
            let result = format_task_output(&task_info).unwrap();
            
            assert!(result.contains("line 1 | line 2 | line 3"));
        }

        #[test]
        fn truncates_long_output() {
            let long_output = "a".repeat(150);
            let task_info = create_test_task_info_with_output(TaskStatus::Running, &long_output);
            let result = format_task_output(&task_info).unwrap();
            
            assert!(result.contains("..."));
            assert!(result.len() < long_output.len() + 20); // Account for formatting
        }
    }

    mod format_task_error {
        use super::*;
        use crate::agents::sub_recipe_execution_tool::types::{TaskResult, TaskStatus};

        fn create_test_task_info_with_error(error_msg: Option<&str>) -> TaskInfo {
            let result = error_msg.map(|msg| TaskResult {
                task_id: "test_task".to_string(),
                status: TaskStatus::Failed,
                data: None,
                error: Some(msg.to_string()),
            });

            TaskInfo {
                task: Task {
                    id: "test_task".to_string(),
                    task_type: "test".to_string(),
                    payload: json!({}),
                },
                status: TaskStatus::Failed,
                start_time: None,
                end_time: None,
                result,
                current_output: String::new(),
            }
        }

        #[test]
        fn returns_none_when_no_error() {
            let task_info = create_test_task_info_with_error(None);
            assert!(format_task_error(&task_info).is_none());
        }

        #[test]
        fn formats_error_message() {
            let task_info = create_test_task_info_with_error(Some("File not found"));
            let result = format_task_error(&task_info).unwrap();
            
            assert!(result.contains("⚠️"));
            assert!(result.contains("File not found"));
        }

        #[test]
        fn replaces_newlines_in_error() {
            let task_info = create_test_task_info_with_error(Some("Error on line 1\nError on line 2"));
            let result = format_task_error(&task_info).unwrap();
            
            assert!(result.contains("Error on line 1 Error on line 2"));
        }

        #[test]
        fn truncates_long_error() {
            let long_error = "error ".repeat(30);
            let task_info = create_test_task_info_with_error(Some(&long_error));
            let result = format_task_error(&task_info).unwrap();
            
            assert!(result.contains("..."));
            assert!(result.len() < long_error.len() + 20); // Account for formatting
        }
    }

    mod format_task_display {
        use super::*;
        use std::time::Duration;

        fn create_comprehensive_task_info(
            task_name: &str,
            status: TaskStatus,
            start_time: Option<Instant>,
            end_time: Option<Instant>,
            current_output: &str,
            error: Option<&str>,
        ) -> TaskInfo {
            let result = error.map(|msg| crate::agents::sub_recipe_execution_tool::types::TaskResult {
                task_id: task_name.to_string(),
                status: status.clone(),
                data: None,
                error: Some(msg.to_string()),
            });

            TaskInfo {
                task: Task {
                    id: task_name.to_string(),
                    task_type: "test".to_string(),
                    payload: json!({}),
                },
                status,
                start_time,
                end_time,
                result,
                current_output: current_output.to_string(),
            }
        }

        #[test]
        fn formats_pending_task() {
            let task_info = create_comprehensive_task_info(
                "pending_task",
                TaskStatus::Pending,
                None,
                None,
                "",
                None,
            );
            let current_time = Instant::now();
            let result = format_task_display(&task_info, current_time);
            
            assert!(result.contains("⏳"));
            assert!(result.contains("pending_task"));
            assert!(result.contains("(test)"));
        }

        #[test]
        fn formats_running_task_with_output() {
            let start_time = Instant::now();
            let current_time = start_time + Duration::from_secs(2);
            let task_info = create_comprehensive_task_info(
                "running_task",
                TaskStatus::Running,
                Some(start_time),
                None,
                "Compiling...",
                None,
            );
            let result = format_task_display(&task_info, current_time);
            
            assert!(result.contains("🏃"));
            assert!(result.contains("running_task"));
            assert!(result.contains("2.0s"));
            assert!(result.contains("💬"));
            assert!(result.contains("Compiling..."));
        }

        #[test]
        fn formats_failed_task_with_error() {
            let start_time = Instant::now();
            let end_time = start_time + Duration::from_millis(1500);
            let task_info = create_comprehensive_task_info(
                "failed_task",
                TaskStatus::Failed,
                Some(start_time),
                Some(end_time),
                "",
                Some("Compilation failed"),
            );
            let current_time = Instant::now();
            let result = format_task_display(&task_info, current_time);
            
            assert!(result.contains("❌"));
            assert!(result.contains("failed_task"));
            assert!(result.contains("1.5s"));
            assert!(result.contains("⚠️"));
            assert!(result.contains("Compilation failed"));
        }

        #[test]
        fn formats_completed_task() {
            let start_time = Instant::now();
            let end_time = start_time + Duration::from_secs(3);
            let task_info = create_comprehensive_task_info(
                "completed_task",
                TaskStatus::Completed,
                Some(start_time),
                Some(end_time),
                "",
                None,
            );
            let current_time = Instant::now();
            let result = format_task_display(&task_info, current_time);
            
            assert!(result.contains("✅"));
            assert!(result.contains("completed_task"));
            assert!(result.contains("3.0s"));
        }
    }
