use crate::recipes::extract_from_cli::extract_recipe_info_from_cli;
use crate::session::{build_session, SessionBuilderConfig};
use anyhow::{Context, Result};
use clap::Args;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Orchestrate parallel work on GitHub issues using swarm intelligence
#[derive(Args, Debug)]
pub struct SwarmArgs {
    /// GitHub repository in format owner/repo
    #[arg(help = "GitHub repository (e.g., owner/repo)")]
    pub repo: String,

    /// Optional worker ID for this swarm agent
    #[arg(
        long,
        help = "Worker ID for this swarm agent (auto-generated if not provided)"
    )]
    pub worker_id: Option<String>,

    /// Poll interval in seconds (default: 30)
    #[arg(long, default_value = "30", help = "Polling interval in seconds")]
    pub poll_interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubIssue {
    number: u32,
    title: String,
    body: Option<String>,
    labels: Vec<GitHubLabel>,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubLabel {
    name: String,
}

#[derive(Debug)]
enum WorkType {
    Planning(GitHubIssue), // Planning issue that needs breakdown
    Task(GitHubIssue),     // Task ready to execute
    None,                  // No work available
}

impl Clone for GitHubIssue {
    fn clone(&self) -> Self {
        GitHubIssue {
            number: self.number,
            title: self.title.clone(),
            body: self.body.clone(),
            labels: self.labels.clone(),
            state: self.state.clone(),
        }
    }
}

impl Clone for GitHubLabel {
    fn clone(&self) -> Self {
        GitHubLabel {
            name: self.name.clone(),
        }
    }
}

/// Generate a unique worker ID
fn generate_worker_id() -> String {
    let timestamp = chrono::Utc::now().format("%m%d");
    let random_suffix: String = (0..4)
        .map(|_| {
            let idx = rand::random::<usize>() % 26;
            (b'a' + idx as u8) as char
        })
        .collect();

    format!("thing-{}-{}", timestamp, random_suffix)
}

/// Register node in the goose swarm issue
fn register_node(repo: &str, worker_id: &str) -> Result<()> {
    // First, check if a goose swarm issue exists
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            "#gooseswarm",
            "--json",
            "number,title,body",
        ])
        .output()
        .context("Failed to search for goose swarm issue")?;

    let issues: Vec<serde_json::Value> = if output.status.success() {
        serde_json::from_slice(&output.stdout)?
    } else {
        Vec::new()
    };

    let node_entry = format!("goose:node:{}", worker_id);

    if let Some(issue) = issues.first() {
        // Issue exists, update it with our node ID
        let issue_number = issue["number"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Invalid issue number"))?;

        let current_body = issue["body"].as_str().unwrap_or("").to_string();

        // Check if we're already registered
        if current_body.contains(&node_entry) {
            println!("✅ Node already registered in swarm issue");
            return Ok(());
        }

        // Add our node entry
        let new_body = if current_body.is_empty() {
            node_entry
        } else {
            format!("{}\n{}", current_body, node_entry)
        };

        Command::new("gh")
            .args([
                "issue",
                "edit",
                &issue_number.to_string(),
                "--repo",
                repo,
                "--body",
                &new_body,
            ])
            .output()
            .context("Failed to update goose swarm issue")?;

        println!("📝 Registered node {} in existing swarm issue", worker_id);
    } else {
        // Create new goose swarm issue
        Command::new("gh")
            .args([
                "issue",
                "create",
                "--repo",
                repo,
                "--title",
                "#gooseswarm",
                "--body",
                &node_entry,
            ])
            .output()
            .context("Failed to create goose swarm issue")?;

        println!(
            "📝 Created goose swarm issue and registered node {}",
            worker_id
        );
    }

    Ok(())
}

/// Count available drone nodes from the goose swarm issue
fn count_available_nodes(repo: &str) -> Result<usize> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            "#gooseswarm",
            "--json",
            "body",
            "--jq",
            ".[0].body",
        ])
        .output()
        .context("Failed to get goose swarm issue")?;

    if output.status.success() {
        let body = String::from_utf8_lossy(&output.stdout);
        let node_count = body
            .lines()
            .filter(|line| line.trim().starts_with("goose:node:"))
            .count();

        // Subtract 1 for the current node (planner)
        Ok(node_count.saturating_sub(1))
    } else {
        Ok(0)
    }
}

/// Get help wanted issues
fn get_help_wanted_issues(repo: &str) -> Result<Vec<u32>> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--label",
            "help wanted",
            "--state",
            "open",
            "--json",
            "number",
            "--jq",
            ".[].number",
        ])
        .output()
        .context("Failed to list help wanted issues")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let numbers_str = String::from_utf8_lossy(&output.stdout);
    let numbers: Vec<u32> = numbers_str
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    Ok(numbers)
}

/// Get all open issues
fn get_all_issues(repo: &str) -> Result<Vec<GitHubIssue>> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--state",
            "open",
            "--json",
            "number,title,body,labels",
            "--limit",
            "100",
        ])
        .output()
        .context("Failed to list issues")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to list issues: {}", error));
    }

    let issues: Vec<GitHubIssue> = serde_json::from_slice(&output.stdout)?;
    Ok(issues)
}

/// Check if issue is already claimed by checking for goose:swarm: in body
fn is_issue_claimed(repo: &str, issue_number: u32) -> Result<bool> {
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--json",
            "body",
        ])
        .output()
        .context("Failed to get issue body")?;

    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        return Ok(content.contains("goose:swarm:"));
    }

    Ok(false)
}

/// Detect available work
fn detect_work_type(repo: &str) -> Result<WorkType> {
    // Get all help wanted issues
    let help_wanted = get_help_wanted_issues(repo)?;

    if help_wanted.is_empty() {
        return Ok(WorkType::None);
    }

    // Get full details for all issues to classify them
    let all_issues = get_all_issues(repo)?;

    for issue_num in help_wanted {
        // Skip if already claimed
        if is_issue_claimed(repo, issue_num)? {
            continue;
        }

        // Find the full issue details
        if let Some(issue) = all_issues.iter().find(|i| i.number == issue_num) {
            // Check if it has help wanted label
            let has_help_wanted = issue.labels.iter().any(|l| l.name == "help wanted");

            if has_help_wanted {
                // Determine role based on title prefix
                if issue.title.starts_with("[task]") {
                    // Task issue = drone/worker role
                    return Ok(WorkType::Task(issue.clone()));
                } else {
                    // Non-task issue with help wanted = planner role
                    return Ok(WorkType::Planning(issue.clone()));
                }
            }
        }
    }

    Ok(WorkType::None)
}

/// Remove help wanted label from an issue
fn remove_help_wanted_label(repo: &str, issue_number: u32) -> Result<()> {
    let output = Command::new("gh")
        .args([
            "issue",
            "edit",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--remove-label",
            "help wanted",
        ])
        .output()
        .context("Failed to remove help wanted label")?;

    if !output.status.success() {
        // It's okay if the label wasn't there
        let error = String::from_utf8_lossy(&output.stderr);
        if !error.contains("does not have label") {
            return Err(anyhow::anyhow!("Failed to remove label: {}", error));
        }
    }

    Ok(())
}

/// Remove claim from issue body
fn unclaim_issue(repo: &str, issue_number: u32, worker_id: &str) -> Result<()> {
    // Get current issue body
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--json",
            "body",
            "--jq",
            ".body",
        ])
        .output()
        .context("Failed to get issue body")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to get issue body: {}", error));
    }

    let current_body = String::from_utf8_lossy(&output.stdout);

    // Remove the claim line for this worker
    let claim_line_drone = format!("Claimed by: {} (Goose drone)", worker_id);
    let claim_line_planner = format!("Claimed by: {} (Goose planner)", worker_id);

    let new_body = current_body
        .lines()
        .filter(|line| !line.contains(&claim_line_drone) && !line.contains(&claim_line_planner))
        .collect::<Vec<_>>()
        .join("\n");

    // Update issue body without the claim
    let output = Command::new("gh")
        .args([
            "issue",
            "edit",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--body",
            &new_body,
        ])
        .output()
        .context("Failed to update issue body")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to unclaim issue: {}", error));
    }

    Ok(())
}

/// Mark issue as claimed by adding worker ID to body with role-specific summary
fn claim_issue(repo: &str, issue_number: u32, worker_id: &str, role: &str) -> Result<()> {
    // Get current issue body
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--json",
            "body",
            "--jq",
            ".body",
        ])
        .output()
        .context("Failed to get issue body")?;

    let current_body = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::new()
    };

    // Add worker ID marker with role-specific summary
    let new_body = format!(
        "{}\n<details><summary>Goose {}</summary>\n<p>\ngoose:swarm:{}</p>\n</details>",
        current_body.trim(),
        role,
        worker_id
    );

    // Update issue body
    let output = Command::new("gh")
        .args([
            "issue",
            "edit",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--body",
            &new_body,
        ])
        .output()
        .context("Failed to update issue body")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to claim issue: {}", error));
    }

    println!(
        "🏷️ Claimed issue #{} as {} with worker ID: {}",
        issue_number, role, worker_id
    );
    Ok(())
}

/// Get issue context including body and comments using GraphQL
fn get_issue_context(repo: &str, issue_number: u32) -> Result<String> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid repository format. Expected owner/repo"
        ));
    }

    let query = format!(
        r#"{{ repository(owner:"{}", name:"{}") {{ issue(number:{}) {{ title body comments(first:100) {{ nodes {{ body }} }} }} }} }}"#,
        parts[0], parts[1], issue_number
    );

    let output = Command::new("gh")
        .args([
            "api",
            "graphql",
            "-f",
            &format!("query={}", query),
            "--jq",
            r#".data.repository.issue | "Title: \(.title)\n\nBody:\n\(.body)\n\nComments:\n" + (.comments.nodes | map(.body) | join("\n---\n"))"#,
        ])
        .output()
        .context("Failed to get issue context")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to fetch issue details: {}", error));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get original issue ID if this task references another issue
fn get_original_issue_id(repo: &str, issue_number: u32) -> Result<Option<u32>> {
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--json",
            "body",
            "--jq",
            r##".body | scan("#([0-9]+)") | .[0]"##,
        ])
        .output()
        .context("Failed to check for original issue")?;

    if output.status.success() {
        let id_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !id_str.is_empty() {
            return Ok(id_str.parse().ok());
        }
    }

    Ok(None)
}

/// Clone or refresh repository to workspace
fn prepare_workspace(repo: &str) -> Result<String> {
    let repo_name = repo.split('/').next_back().unwrap_or(repo);

    // Use ~/.local/share/goose-swarm as per design spec
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Failed to get home directory")?;
    let workspace_dir = format!("{}/.local/share/goose-swarm/{}", home_dir, repo_name);

    // Create workspace directory
    std::fs::create_dir_all(&workspace_dir).context("Failed to create workspace")?;

    // Check if repo already cloned
    if !std::path::Path::new(&format!("{}/.git", workspace_dir)).exists() {
        println!("📦 Cloning repository...");
        let output = Command::new("gh")
            .args(["repo", "clone", repo, &workspace_dir])
            .output()
            .context("Failed to clone repository")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to clone: {}", error));
        }
    } else {
        // Pull latest changes
        println!("🔄 Updating repository...");
        let output = Command::new("git")
            .args(["pull", "origin", "main"])
            .current_dir(&workspace_dir)
            .output()
            .context("Failed to pull latest changes")?;

        if !output.status.success() {
            // Try pulling from master if main doesn't exist
            Command::new("git")
                .args(["pull", "origin", "master"])
                .current_dir(&workspace_dir)
                .output()
                .context("Failed to pull from master")?;
        }
    }

    Ok(workspace_dir)
}

/// Get task issue numbers created from a planning issue
fn get_task_issue_numbers(repo: &str, parent_issue: u32) -> Result<Vec<u32>> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            &format!("\"for:#{}\" in:body \"[task]\" in:title", parent_issue),
            "--state",
            "all",
            "--json",
            "number",
        ])
        .output()
        .context("Failed to get task issue numbers")?;

    if output.status.success() {
        let issues: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        let numbers: Vec<u32> = issues
            .iter()
            .filter_map(|issue| issue["number"].as_u64().map(|n| n as u32))
            .collect();
        Ok(numbers)
    } else {
        Ok(Vec::new())
    }
}

/// Get PRs that reference task issues (either in title or body)
fn get_prs_for_tasks(repo: &str, task_issue_numbers: &[u32]) -> Result<Vec<serde_json::Value>> {
    let mut all_prs = Vec::new();

    // For each task issue, find PRs that reference it
    for task_num in task_issue_numbers {
        let output = Command::new("gh")
            .args([
                "pr",
                "list",
                "--repo",
                repo,
                "--search",
                &format!("#{} in:body,title", task_num),
                "--state",
                "all",
                "--json",
                "number,title,url,state,body,isDraft",
            ])
            .output()
            .context("Failed to get PRs for task")?;

        if output.status.success() {
            let prs: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;

            // Add task_issue field to each PR so we know which task it addresses
            for mut pr in prs {
                pr["task_issue"] = serde_json::json!(task_num);

                // Avoid duplicates if a PR mentions multiple task issues
                let pr_number = pr["number"].as_u64();
                if !all_prs
                    .iter()
                    .any(|p: &serde_json::Value| p["number"].as_u64() == pr_number)
                {
                    all_prs.push(pr);
                }
            }
        }
    }

    Ok(all_prs)
}

/// Get non-draft PRs that are ready (open or closed/merged) for task issues
fn get_ready_prs_for_tasks(
    repo: &str,
    task_issue_numbers: &[u32],
) -> Result<Vec<serde_json::Value>> {
    let all_prs = get_prs_for_tasks(repo, task_issue_numbers)?;

    // Filter out draft PRs - we only want non-draft PRs (open, closed, or merged)
    let ready_prs: Vec<serde_json::Value> = all_prs
        .into_iter()
        .filter(|pr| {
            // Check if PR is not a draft
            !pr.get("isDraft").and_then(|v| v.as_bool()).unwrap_or(false)
        })
        .collect();

    Ok(ready_prs)
}

/// Check which PRs are still open for the task issues
fn check_open_prs(repo: &str, task_issue_numbers: &[u32]) -> Result<Vec<serde_json::Value>> {
    let mut open_prs = Vec::new();

    // For each task issue, find open PRs that reference it
    for task_num in task_issue_numbers {
        let output = Command::new("gh")
            .args([
                "pr",
                "list",
                "--repo",
                repo,
                "--search",
                &format!("#{} in:body,title", task_num),
                "--state",
                "open",
                "--json",
                "number,title,url,state,body,isDraft",
            ])
            .output()
            .context("Failed to get open PRs for task")?;

        if output.status.success() {
            let prs: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;

            // Add task_issue field and filter out drafts
            for mut pr in prs {
                // Skip draft PRs
                if pr.get("isDraft").and_then(|v| v.as_bool()).unwrap_or(false) {
                    continue;
                }
                
                pr["task_issue"] = serde_json::json!(task_num);

                // Avoid duplicates if a PR mentions multiple task issues
                let pr_number = pr["number"].as_u64();
                if !open_prs
                    .iter()
                    .any(|p: &serde_json::Value| p["number"].as_u64() == pr_number)
                {
                    open_prs.push(pr);
                }
            }
        }
    }

    Ok(open_prs)
}

/// Count task issues created from a planning issue
fn count_task_issues(repo: &str, parent_issue: u32) -> Result<usize> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            &format!("\"for:#{}\" in:body \"[task]\" in:title", parent_issue),
            "--state",
            "all",
            "--json",
            "number",
        ])
        .output()
        .context("Failed to count task issues")?;

    if output.status.success() {
        let issues: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        Ok(issues.len())
    } else {
        Ok(0)
    }
}

/// Process task files in tasks/ directory to create [task] issues
fn process_task_files(repo: &str, parent_issue: u32, tasks_dir: &str) -> Result<()> {
    let entries = std::fs::read_dir(tasks_dir).context("Failed to read tasks directory")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = std::fs::read_to_string(&path).context("Failed to read task file")?;

            // Extract title from first line (should be a markdown header)
            let title = content
                .lines()
                .next()
                .unwrap_or("Task")
                .trim_start_matches('#')
                .trim();

            // Add [task] prefix if not present
            let final_title = if title.starts_with("[task]") {
                title.to_string()
            } else {
                format!("[task] {}", title)
            };

            // Add "for:#<parent>" to the body
            let body_with_reference = format!("{}\n\nfor:#{}", content, parent_issue);

            // Create the task issue
            let output = Command::new("gh")
                .args([
                    "issue",
                    "create",
                    "--repo",
                    repo,
                    "--title",
                    &final_title,
                    "--body",
                    &body_with_reference,
                    "--label",
                    "help wanted",
                ])
                .output()
                .context("Failed to create task issue")?;

            if output.status.success() {
                println!("✅ Created task issue: {}", final_title);
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to create task issue: {}", error);
            }
        }
    }

    Ok(())
}

/// Process issue files in issues/ directory to create non-task issues
fn process_issue_files(repo: &str, parent_issue: u32, issues_dir: &str) -> Result<()> {
    let entries = std::fs::read_dir(issues_dir).context("Failed to read issues directory")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = std::fs::read_to_string(&path).context("Failed to read issue file")?;

            // Extract title from first line (should be a markdown header)
            let title = content
                .lines()
                .next()
                .unwrap_or("Issue")
                .trim_start_matches('#')
                .trim();

            // Make sure it doesn't have [task] prefix
            let final_title = if title.starts_with("[task]") {
                title.replace("[task]", "").trim().to_string()
            } else {
                title.to_string()
            };

            // Add "for:#<parent>" to the body
            let body_with_reference = format!("{}\n\nfor:#{}", content, parent_issue);

            // Create the issue (with help wanted for planning work)
            let output = Command::new("gh")
                .args([
                    "issue",
                    "create",
                    "--repo",
                    repo,
                    "--title",
                    &final_title,
                    "--body",
                    &body_with_reference,
                    "--label",
                    "help wanted",
                ])
                .output()
                .context("Failed to create issue")?;

            if output.status.success() {
                println!("✅ Created planning issue: {}", final_title);
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to create issue: {}", error);
            }
        }
    }

    Ok(())
}

/// Close all task issues created from a planning issue
fn close_task_issues(repo: &str, parent_issue: u32) -> Result<()> {
    // Find all task issues for this parent
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            &format!("\"for:#{}\" in:body \"[task]\" in:title", parent_issue),
            "--state",
            "open",
            "--json",
            "number",
        ])
        .output()
        .context("Failed to list task issues")?;

    if output.status.success() {
        let issues: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;

        for issue in issues {
            if let Some(issue_number) = issue["number"].as_u64() {
                println!("🔒 Closing task issue #{}...", issue_number);
                Command::new("gh")
                    .args([
                        "issue",
                        "close",
                        &issue_number.to_string(),
                        "--repo",
                        repo,
                        "--comment",
                        "Task completed. Closed by Goose Swarm after evaluation.",
                    ])
                    .output()
                    .context("Failed to close task issue")?;
            }
        }
    }

    Ok(())
}

/// Process new issues created by the evaluation recipe
fn process_evaluation_issues(repo: &str, parent_issue: u32, issues_dir: &str) -> Result<()> {
    let entries = match std::fs::read_dir(issues_dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()), // No issues directory or empty
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content =
                std::fs::read_to_string(&path).context("Failed to read evaluation issue file")?;

            // Extract title from first line (should be a markdown header)
            let title = content
                .lines()
                .next()
                .unwrap_or("Follow-up Issue")
                .trim_start_matches('#')
                .trim();

            let body = format!("{}\n\nOriginal issue: #{}", content, parent_issue);

            // Create the new issue
            let output = Command::new("gh")
                .args([
                    "issue", "create", "--repo", repo, "--title", title, "--body", &body,
                ])
                .output()
                .context("Failed to create evaluation issue")?;

            if output.status.success() {
                println!("✅ Created new issue from evaluation: {}", title);
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to create evaluation issue: {}", error);
            }
        }
    }

    Ok(())
}

/// Execute planning work
async fn execute_planning_work(repo: &str, issue: &GitHubIssue, worker_id: &str) -> Result<()> {
    println!(
        "📋 Working on planning issue #{}: {}",
        issue.number, issue.title
    );

    // Claim the issue as planner
    claim_issue(repo, issue.number, worker_id, "planner")?;

    // Get full issue context
    let context = get_issue_context(repo, issue.number)?;

    // Count available drone nodes
    let available_nodes = count_available_nodes(repo)?;

    // Prepare repository workspace (shared location for read-only access)
    let repo_dir = prepare_workspace(repo)?;
    println!("📁 Repository available at: {}", repo_dir);

    // Create working directory with tasks/ and issues/ subdirectories
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Failed to get home directory")?;
    let work_dir = format!(
        "{}/.local/share/goose-swarm/planner-work-{}",
        home_dir, issue.number
    );
    let tasks_dir = format!("{}/tasks", work_dir);
    let issues_dir = format!("{}/issues", work_dir);

    std::fs::create_dir_all(&tasks_dir).context("Failed to create tasks directory")?;
    std::fs::create_dir_all(&issues_dir).context("Failed to create issues directory")?;

    // Build recipe parameters for planning
    let mut params = vec![
        ("repo".to_string(), repo.to_string()),
        ("issue_number".to_string(), issue.number.to_string()),
        ("worker_id".to_string(), worker_id.to_string()),
        ("context".to_string(), context.clone()),
        ("available_nodes".to_string(), available_nodes.to_string()),
        ("work_dir".to_string(), work_dir.clone()),
        ("repo_dir".to_string(), repo_dir), // Pass the cloned repo directory
    ];

    // Check for original issue reference
    if let Ok(Some(original_id)) = get_original_issue_id(repo, issue.number) {
        params.push(("original_issue".to_string(), original_id.to_string()));
    }

    // Run the planning recipe to create tasks
    let recipe_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/goose-swarm/plan_work.yaml");

    println!("🎯 Running planning recipe to break down the issue...");
    run_recipe(recipe_path, params).await?;

    // Process task files in tasks/ directory
    process_task_files(repo, issue.number, &tasks_dir)?;

    // Process issue files in issues/ directory
    process_issue_files(repo, issue.number, &issues_dir)?;

    // Count how many task issues were created
    let task_count = count_task_issues(repo, issue.number)?;
    if task_count == 0 {
        println!("⚠️ No task issues were created. Planning may have failed.");
        return Ok(());
    }

    println!(
        "📊 Created {} task issues. Now polling for PRs...",
        task_count
    );

    // Get task issue numbers for tracking PRs
    let task_issue_numbers = get_task_issue_numbers(repo, issue.number)?;
    println!("📝 Tracking PRs for task issues: {:?}", task_issue_numbers);

    // Poll for PRs to come in
    loop {
        let all_prs = get_prs_for_tasks(repo, &task_issue_numbers)?;
        let ready_prs = get_ready_prs_for_tasks(repo, &task_issue_numbers)?;

        println!("🔍 Found {} PR(s) addressing task issues", all_prs.len());
        println!(
            "   📋 {} non-draft PR(s) ready for evaluation",
            ready_prs.len()
        );

        // Show draft PRs for visibility
        let draft_count = all_prs.len() - ready_prs.len();
        if draft_count > 0 {
            println!("   📝 {} PR(s) still in draft", draft_count);
        }

        // Check if we have enough ready PRs (at least one per task, non-draft)
        if ready_prs.len() >= task_count {
            println!("✅ All tasks have ready PRs (non-draft). Running evaluation...");

            // Prepare context for evaluation (include all PRs for context, but evaluation focuses on ready ones)
            let pr_context = serde_json::to_string_pretty(&ready_prs)?;

            // Create evaluation working directory with issues/ subdirectory
            let eval_work_dir = format!(
                "{}/.local/share/goose-swarm/eval-work-{}",
                home_dir, issue.number
            );
            let eval_issues_dir = format!("{}/issues", eval_work_dir);
            std::fs::create_dir_all(&eval_issues_dir)
                .context("Failed to create evaluation issues directory")?;

            // Build parameters for evaluate recipe
            let eval_params = vec![
                ("repo".to_string(), repo.to_string()),
                ("issue_number".to_string(), issue.number.to_string()),
                ("worker_id".to_string(), worker_id.to_string()),
                ("issue_context".to_string(), context),
                ("pr_context".to_string(), pr_context),
                ("task_count".to_string(), task_count.to_string()),
                ("work_dir".to_string(), eval_work_dir.clone()),
            ];

            // Run the evaluate recipe
            let eval_recipe_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/commands/goose-swarm/evaluate.yaml");

            println!("🔮 Running evaluation recipe...");
            run_recipe(eval_recipe_path.clone(), eval_params.clone()).await?;

            // Process any new issues created by evaluate recipe
            process_evaluation_issues(repo, issue.number, &eval_issues_dir)?;

            // Check if all PRs are closed/merged
            let remaining_open_prs = check_open_prs(repo, &task_issue_numbers)?;
            
            if !remaining_open_prs.is_empty() {
                println!("⚠️ {} PR(s) still open after evaluation. Running evaluation once more...", remaining_open_prs.len());
                
                // Run evaluation one more time to give it a chance to address open PRs
                run_recipe(eval_recipe_path, eval_params).await?;
                
                // Process any new issues from second evaluation
                process_evaluation_issues(repo, issue.number, &eval_issues_dir)?;
                
                // Check PRs again
                let final_open_prs = check_open_prs(repo, &task_issue_numbers)?;
                
                if !final_open_prs.is_empty() {
                    println!("📝 {} PR(s) still open. Closing them now...", final_open_prs.len());
                    
                    // Close all remaining open PRs
                    for pr in final_open_prs {
                        if let Some(pr_num) = pr.get("number").and_then(|n| n.as_u64()) {
                            println!("   Closing PR #{}", pr_num);
                            Command::new("gh")
                                .args([
                                    "pr",
                                    "close",
                                    &pr_num.to_string(),
                                    "--repo",
                                    repo,
                                    "--comment",
                                    "Automatically closed by Goose Swarm after evaluation phase.",
                                ])
                                .output()
                                .context("Failed to close PR")?;
                        }
                    }
                }
            }

            // Close all task issues that were created
            close_task_issues(repo, issue.number)?;

            // Close the original planning issue
            println!("🔒 Closing planning issue #{}...", issue.number);
            Command::new("gh")
                .args([
                    "issue",
                    "close",
                    &issue.number.to_string(),
                    "--repo",
                    repo,
                    "--comment",
                    "All tasks completed and evaluated. Issue closed by Goose Swarm.",
                ])
                .output()
                .context("Failed to close issue")?;

            break;
        }

        // Wait before polling again
        println!(
            "⏳ Waiting for more ready PRs... ({}/{} ready)",
            ready_prs.len(),
            task_count
        );
        thread::sleep(Duration::from_secs(60)); // Poll every minute
    }

    Ok(())
}

/// Execute task work
async fn execute_task_work(repo: &str, issue: &GitHubIssue, worker_id: &str) -> Result<()> {
    println!("🔧 Working on task #{}: {}", issue.number, issue.title);

    // Remove help wanted label
    remove_help_wanted_label(repo, issue.number)?;

    // Claim the issue as drone
    claim_issue(repo, issue.number, worker_id, "drone")?;

    // Get full issue context
    let context = get_issue_context(repo, issue.number)?;

    // Prepare workspace
    let workspace = prepare_workspace(repo)?;
    println!("📁 Workspace: {}", workspace);

    // Build recipe parameters
    let mut params = vec![
        ("repo".to_string(), repo.to_string()),
        ("issue_number".to_string(), issue.number.to_string()),
        ("worker_id".to_string(), worker_id.to_string()),
        ("context".to_string(), context),
        ("workspace".to_string(), workspace.clone()),
    ];

    // Check for original issue reference
    if let Ok(Some(original_id)) = get_original_issue_id(repo, issue.number) {
        params.push(("original_issue".to_string(), original_id.to_string()));
    }

    // Run the worker recipe
    let recipe_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/goose-swarm/swarm_drone.yaml");

    run_recipe(recipe_path, params).await?;

    // Check if still on main branch (indicates failure)
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&workspace)
        .output()
        .context("Failed to check current branch")?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch == "main" || branch == "master" {
            println!("⚠️ Drone failed to create PR (still on {} branch)", branch);

            // Remove claim from issue body
            unclaim_issue(repo, issue.number, worker_id)?;

            // Re-add help wanted label
            let output = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue.number.to_string(),
                    "--repo",
                    repo,
                    "--add-label",
                    "help wanted",
                ])
                .output()
                .context("Failed to re-add help wanted label")?;

            if output.status.success() {
                println!("🔄 Re-added 'help wanted' label for another worker to try");
            }

            return Err(anyhow::anyhow!("Failed to complete task - no PR created"));
        } else {
            println!("✅ Successfully created PR on branch: {}", branch);
        }
    }

    Ok(())
}

/// Run a recipe with given parameters
async fn run_recipe(recipe_path: std::path::PathBuf, params: Vec<(String, String)>) -> Result<()> {
    // Extract recipe info and build input config
    let (input_config, recipe_info) = extract_recipe_info_from_cli(
        recipe_path.to_string_lossy().to_string(),
        params,
        Vec::new(),
    )?;

    // Build and run the session
    let mut session = build_session(SessionBuilderConfig {
        identifier: None,
        resume: false,
        no_session: true,
        extensions: Vec::new(),
        remote_extensions: Vec::new(),
        streamable_http_extensions: Vec::new(),
        builtins: vec!["developer".to_string()],
        extensions_override: input_config.extensions_override,
        additional_system_prompt: input_config.additional_system_prompt,
        settings: recipe_info.session_settings.clone(),
        provider: None,
        model: None,
        debug: false,
        max_tool_repetitions: None,
        max_turns: Some(100),
        scheduled_job_id: None,
        interactive: false,
        quiet: false,
        sub_recipes: recipe_info.sub_recipes.clone(),
        final_output_response: recipe_info.final_output_response.clone(),
        retry_config: recipe_info.retry_config.clone(),
    })
    .await;

    // Run the session
    if let Some(contents) = input_config.contents {
        session.headless(contents).await?;
    } else {
        return Err(anyhow::anyhow!("Recipe did not provide a prompt"));
    }

    Ok(())
}

pub async fn run(args: SwarmArgs) -> Result<()> {
    let worker_id = args.worker_id.unwrap_or_else(generate_worker_id);

    println!("🐝 Starting Goose Swarm");
    println!("📍 Repository: {}", args.repo);
    println!("🤖 Worker ID: {}", worker_id);
    println!("⏱️ Poll interval: {} seconds\n", args.poll_interval);

    // Register this node in the swarm issue
    register_node(&args.repo, &worker_id)?;

    // Main polling loop
    loop {
        println!("🔍 Polling for available work...");

        match detect_work_type(&args.repo)? {
            WorkType::Planning(issue) => {
                execute_planning_work(&args.repo, &issue, &worker_id).await?;
                println!("✅ Completed planning work");
                // Continue polling for more work
            }
            WorkType::Task(issue) => {
                execute_task_work(&args.repo, &issue, &worker_id).await?;
                println!("✅ Completed task work");
                // Continue polling for more work
            }
            WorkType::None => {
                println!(
                    "💤 No work available, waiting {} seconds...",
                    args.poll_interval
                );
                thread::sleep(Duration::from_secs(args.poll_interval));
            }
        }
    }
}
