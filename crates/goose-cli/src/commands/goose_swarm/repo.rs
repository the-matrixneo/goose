// GitHub repository interaction utilities for Goose Swarm

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLabel {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<GitHubLabel>,
}

/// Register a node in the goose swarm issue
pub fn register_node(repo: &str, worker_id: &str) -> Result<()> {
    // Check if goose swarm issue exists
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            "\"goose swarm\" in:title",
            "--state",
            "open",
            "--json",
            "number",
        ])
        .output()
        .context("Failed to check for goose swarm issue")?;

    let issue_number = if output.status.success() {
        let issues: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        if issues.is_empty() {
            // Create new goose swarm issue
            let output = Command::new("gh")
                .args([
                    "issue",
                    "create",
                    "--repo",
                    repo,
                    "--title",
                    "goose swarm",
                    "--body",
                    &format!("Active nodes:\ngoose:node:{}", worker_id),
                ])
                .output()
                .context("Failed to create goose swarm issue")?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Failed to create swarm issue: {}", error));
            }

            // Get the issue number we just created
            let url = String::from_utf8_lossy(&output.stdout);
            url.split('/')
                .next_back()
                .and_then(|s| s.trim().parse::<u32>().ok())
                .context("Failed to parse issue number from URL")?
        } else {
            issues[0]["number"]
                .as_u64()
                .context("Failed to get issue number")? as u32
        }
    } else {
        return Err(anyhow::anyhow!("Failed to list issues"));
    };

    // Add worker ID to the issue body
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
        let body_json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let current_body = body_json["body"].as_str().unwrap_or("");

        // Check if already registered
        if !current_body.contains(&format!("goose:node:{}", worker_id)) {
            let new_body = format!("{}\ngoose:node:{}", current_body, worker_id);

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
                .context("Failed to update issue body")?;

            println!("✅ Registered in swarm issue #{}", issue_number);
        } else {
            println!("ℹ️ Already registered in swarm issue #{}", issue_number);
        }
    }

    Ok(())
}

/// Count available drone nodes from the swarm issue
pub fn count_available_nodes(repo: &str) -> Result<usize> {
    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            repo,
            "--search",
            "\"goose swarm\" in:title",
            "--state",
            "open",
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
pub fn get_help_wanted_issues(repo: &str) -> Result<Vec<u32>> {
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
pub fn get_all_issues(repo: &str) -> Result<Vec<GitHubIssue>> {
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
pub fn is_issue_claimed(repo: &str, issue_number: u32) -> Result<bool> {
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

/// Remove help wanted label from an issue
pub fn remove_help_wanted_label(repo: &str, issue_number: u32) -> Result<()> {
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
pub fn unclaim_issue(repo: &str, issue_number: u32, worker_id: &str) -> Result<()> {
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
pub fn claim_issue(repo: &str, issue_number: u32, worker_id: &str, role: &str) -> Result<()> {
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

    println!("✅ Claimed issue #{} as {}", issue_number, role);
    Ok(())
}

/// Get the original issue ID from a task issue
pub fn get_original_issue_id(repo: &str, task_issue: u32) -> Result<Option<u32>> {
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &task_issue.to_string(),
            "--repo",
            repo,
            "--json",
            "body",
        ])
        .output()
        .context("Failed to get issue body")?;

    if output.status.success() {
        let body_json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        if let Some(body) = body_json["body"].as_str() {
            // Look for "for:#<number>" pattern
            for line in body.lines() {
                if let Some(pos) = line.find("for:#") {
                    let number_str = &line[pos + 5..];
                    if let Some(end) = number_str.find(|c: char| !c.is_ascii_digit()) {
                        return Ok(number_str[..end].parse().ok());
                    } else {
                        return Ok(number_str.parse().ok());
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Get full issue context (title, body, comments)
pub fn get_issue_context(repo: &str, issue_number: u32) -> Result<String> {
    let output = Command::new("gh")
        .args([
            "issue",
            "view",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--json",
            "title,body,comments",
        ])
        .output()
        .context("Failed to get issue details")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to get issue details: {}", error));
    }

    let issue_data: serde_json::Value = serde_json::from_slice(&output.stdout)?;

    let title = issue_data["title"].as_str().unwrap_or("");
    let body = issue_data["body"].as_str().unwrap_or("");

    let mut context = format!("Issue #{}: {}\n\n{}", issue_number, title, body);

    // Add comments if any
    if let Some(comments) = issue_data["comments"].as_array() {
        if !comments.is_empty() {
            context.push_str("\n\n--- Comments ---\n");
            for comment in comments {
                if let Some(author) = comment["author"]["login"].as_str() {
                    if let Some(body) = comment["body"].as_str() {
                        context.push_str(&format!("\n@{}: {}\n", author, body));
                    }
                }
            }
        }
    }

    Ok(context)
}

/// Get task issue numbers for a parent planning issue
pub fn get_task_issue_numbers(repo: &str, parent_issue: u32) -> Result<Vec<u32>> {
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
        .context("Failed to list task issues")?;

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
pub fn get_prs_for_tasks(repo: &str, task_issue_numbers: &[u32]) -> Result<Vec<serde_json::Value>> {
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
pub fn get_ready_prs_for_tasks(
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
pub fn check_open_prs(repo: &str, task_issue_numbers: &[u32]) -> Result<Vec<serde_json::Value>> {
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

/// Close a PR with a comment
pub fn close_pr(repo: &str, pr_number: u64, comment: &str) -> Result<()> {
    println!("   Closing PR #{}", pr_number);
    Command::new("gh")
        .args([
            "pr",
            "close",
            &pr_number.to_string(),
            "--repo",
            repo,
            "--comment",
            comment,
        ])
        .output()
        .context("Failed to close PR")?;
    Ok(())
}

/// Count task issues created from a planning issue
pub fn count_task_issues(repo: &str, parent_issue: u32) -> Result<usize> {
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

/// Create a task issue from a file
pub fn create_task_issue(repo: &str, parent_issue: u32, title: &str, content: &str) -> Result<()> {
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

    Ok(())
}

/// Create a planning issue from a file
pub fn create_planning_issue(
    repo: &str,
    parent_issue: u32,
    title: &str,
    content: &str,
) -> Result<()> {
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

    Ok(())
}

/// Close all task issues created from a planning issue
pub fn close_task_issues(repo: &str, parent_issue: u32) -> Result<()> {
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

/// Close a planning issue
pub fn close_planning_issue(repo: &str, issue_number: u32) -> Result<()> {
    println!("🔒 Closing planning issue #{}...", issue_number);
    Command::new("gh")
        .args([
            "issue",
            "close",
            &issue_number.to_string(),
            "--repo",
            repo,
            "--comment",
            "All tasks completed and evaluated. Issue closed by Goose Swarm.",
        ])
        .output()
        .context("Failed to close issue")?;
    Ok(())
}

/// Create an evaluation issue
pub fn create_evaluation_issue(
    repo: &str,
    parent_issue: u32,
    title: &str,
    content: &str,
) -> Result<()> {
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

    Ok(())
}

/// Re-add help wanted label to an issue
pub fn add_help_wanted_label(repo: &str, issue_number: u32) -> Result<()> {
    let output = Command::new("gh")
        .args([
            "issue",
            "edit",
            &issue_number.to_string(),
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

    Ok(())
}
