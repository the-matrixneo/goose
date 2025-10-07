use crate::config::paths::Paths;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Returns the directory where log files should be stored for a specific component.
/// Creates the directory structure if it doesn't exist.
///
/// # Arguments
///
/// * `component` - The component name (e.g., "cli", "server", "debug")
/// * `use_date_subdir` - Whether to create a date-based subdirectory
pub fn get_log_directory(component: &str, use_date_subdir: bool) -> Result<PathBuf> {
    let base_log_dir = Paths::in_state_dir("logs");

    let component_dir = base_log_dir.join(component);

    let log_dir = if use_date_subdir {
        // Create date-based subdirectory
        let now = chrono::Local::now();
        component_dir.join(now.format("%Y-%m-%d").to_string())
    } else {
        component_dir
    };

    // Ensure log directory exists
    fs::create_dir_all(&log_dir).context("Failed to create log directory")?;

    Ok(log_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_log_directory_basic_functionality() {
        // Test basic directory creation without date subdirectory
        let result = get_log_directory("cli", false);
        assert!(result.is_ok());

        let log_dir = result.unwrap();

        // Verify the directory was created and has correct structure
        assert!(log_dir.exists());
        assert!(log_dir.is_dir());

        let path_str = log_dir.to_string_lossy();
        assert!(path_str.contains("cli"));
        assert!(path_str.contains("logs"));

        // Verify we can write to the directory
        let test_file = log_dir.join("test.log");
        assert!(fs::write(&test_file, "test log content").is_ok());
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_get_log_directory_with_date_subdir() {
        // Test date-based subdirectory creation
        let result = get_log_directory("server", true);
        assert!(result.is_ok());

        let log_dir = result.unwrap();

        // Verify the directory was created
        assert!(log_dir.exists());
        assert!(log_dir.is_dir());

        let path_str = log_dir.to_string_lossy();
        assert!(path_str.contains("server"));
        assert!(path_str.contains("logs"));

        // Verify date format (YYYY-MM-DD) is present
        let now = chrono::Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        assert!(path_str.contains(&date_str));

        // Verify path structure: logs -> component -> date
        let logs_pos = path_str.find("logs").unwrap();
        let component_pos = path_str.find("server").unwrap();
        let date_pos = path_str.find(&date_str).unwrap();
        assert!(logs_pos < component_pos);
        assert!(component_pos < date_pos);
    }

    #[test]
    fn test_get_log_directory_idempotent() {
        // Test that multiple calls return the same result and don't fail
        let component = "debug";

        let result1 = get_log_directory(component, false);
        assert!(result1.is_ok());
        let log_dir1 = result1.unwrap();

        let result2 = get_log_directory(component, false);
        assert!(result2.is_ok());
        let log_dir2 = result2.unwrap();

        // Both calls should return the same path and directory should exist
        assert_eq!(log_dir1, log_dir2);
        assert!(log_dir1.exists());
        assert!(log_dir2.exists());

        // Test same behavior with date subdirectories
        let result3 = get_log_directory(component, true);
        assert!(result3.is_ok());
        let log_dir3 = result3.unwrap();

        let result4 = get_log_directory(component, true);
        assert!(result4.is_ok());
        let log_dir4 = result4.unwrap();

        assert_eq!(log_dir3, log_dir4);
        assert!(log_dir3.exists());
    }

    #[test]
    fn test_get_log_directory_different_components() {
        // Test that different components create different directories
        let components = ["cli", "server", "debug"];
        let mut created_dirs = Vec::new();

        for component in &components {
            let result = get_log_directory(component, false);
            assert!(result.is_ok(), "Failed for component: {}", component);

            let log_dir = result.unwrap();
            assert!(log_dir.exists());
            assert!(log_dir.to_string_lossy().contains(component));

            created_dirs.push(log_dir);
        }

        // Verify all directories are different
        for i in 0..created_dirs.len() {
            for j in i + 1..created_dirs.len() {
                assert_ne!(created_dirs[i], created_dirs[j]);
            }
        }
    }
}
