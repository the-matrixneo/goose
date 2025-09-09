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
    // First check for help wanted tasks (highest priority)
    let help_wanted = get_help_wanted_issues(repo)?;

    for issue_num in help_wanted {
        // Verify it's not already claimed
        if !is_issue_claimed(repo, issue_num)? {
            // Get full issue details
            let issues = get_all_issues(repo)?;
            if let Some(issue) = issues.iter().find(|i| i.number == issue_num) {
                return Ok(WorkType::Task(issue.clone()));
            }
        }
    }

    // Then check for planning work (non-task issues)
    let all_issues = get_all_issues(repo)?;

    for issue in all_issues {
        // Skip if it's a task
        if !issue.title.starts_with("[task]") {
            // Check if not already claimed
            if !is_issue_claimed(repo, issue.number)? {
                return Ok(WorkType::Planning(issue));
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

/// Mark issue as claimed by adding worker ID to body
fn claim_issue(repo: &str, issue_number: u32, worker_id: &str) -> Result<()> {
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

    // Add worker ID marker
    let new_body = format!(
        "{}\n<details><summary>Goose planner</summary>\n<p>\ngoose:swarm:{}</p>\n</details>",
        current_body.trim(),
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
        "🏷️ Claimed issue #{} with worker ID: {}",
        issue_number, worker_id
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

/// Clone repository to workspace
fn prepare_workspace(repo: &str) -> Result<String> {
    let repo_name = repo.split('/').next_back().unwrap_or(repo);
    let workspace_dir = format!("/tmp/goose-swarm/{}", repo_name);

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

/// Get PRs related to an issue
fn get_related_prs(repo: &str, issue_number: u32) -> Result<Vec<serde_json::Value>> {
    let output = Command::new("gh")
        .args([
            "pr",
            "list",
            "--repo",
            repo,
            "--search",
            &format!("{} in:body,title", issue_number),
            "--state",
            "all",
            "--json",
            "number,title,url,state,body",
        ])
        .output()
        .context("Failed to get related PRs")?;

    if output.status.success() {
        let prs: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        Ok(prs)
    } else {
        Ok(Vec::new())
    }
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
            &format!("\"#{}\" in:body \"[task]\" in:title", parent_issue),
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

/// Execute planning work
async fn execute_planning_work(repo: &str, issue: &GitHubIssue, worker_id: &str) -> Result<()> {
    println!(
        "📋 Working on planning issue #{}: {}",
        issue.number, issue.title
    );

    // Claim the issue
    claim_issue(repo, issue.number, worker_id)?;

    // Get full issue context
    let context = get_issue_context(repo, issue.number)?;

    // Build recipe parameters for planning
    let mut params = vec![
        ("repo".to_string(), repo.to_string()),
        ("issue_number".to_string(), issue.number.to_string()),
        ("worker_id".to_string(), worker_id.to_string()),
        ("context".to_string(), context.clone()),
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

    // Wait a bit for task issues to be created
    thread::sleep(Duration::from_secs(10));

    // Count how many task issues were created
    let task_count = count_task_issues(repo, issue.number)?;
    if task_count == 0 {
        println!("⚠️ No task issues were created. Planning may have failed.");
        return Ok(());
    }

    println!("📊 Created {} task issues. Now polling for PRs...", task_count);

    // Poll for PRs to come in
    loop {
        let prs = get_related_prs(repo, issue.number)?;
        println!("🔍 Found {} PR(s) related to issue #{}", prs.len(), issue.number);

        // Check if we have enough PRs (at least one per task)
        if prs.len() >= task_count {
            println!("✅ All tasks have associated PRs. Running evaluation...");
            
            // Prepare context for evaluation
            let pr_context = serde_json::to_string_pretty(&prs)?;
            
            // Build parameters for evaluate recipe
            let eval_params = vec![
                ("repo".to_string(), repo.to_string()),
                ("issue_number".to_string(), issue.number.to_string()),
                ("worker_id".to_string(), worker_id.to_string()),
                ("issue_context".to_string(), context),
                ("pr_context".to_string(), pr_context),
                ("task_count".to_string(), task_count.to_string()),
            ];

            // Run the evaluate recipe
            let eval_recipe_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/commands/goose-swarm/evaluate.yaml");

            println!("🔮 Running evaluation recipe...");
            run_recipe(eval_recipe_path, eval_params).await?;

            // Check if FOLLOW_ON.md was created
            if std::path::Path::new("FOLLOW_ON.md").exists() {
                println!("📝 Follow-on work identified. Creating new issue...");
                
                // Read follow-on content
                let follow_on_content = std::fs::read_to_string("FOLLOW_ON.md")?;
                
                // Create a new help wanted issue
                let output = Command::new("gh")
                    .args([
                        "issue",
                        "create",
                        "--repo",
                        repo,
                        "--title",
                        &format!("Follow-on work for #{}", issue.number),
                        "--body",
                        &format!("{}\n\nOriginal issue: #{}", follow_on_content, issue.number),
                        "--label",
                        "help wanted",
                    ])
                    .output()
                    .context("Failed to create follow-on issue")?;

                if output.status.success() {
                    println!("✅ Created follow-on issue");
                }

                // Clean up
                std::fs::remove_file("FOLLOW_ON.md").ok();
            }

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
        println!("⏳ Waiting for more PRs... ({}/{} received)", prs.len(), task_count);
        thread::sleep(Duration::from_secs(60)); // Poll every minute
    }

    Ok(())
}

/// Execute task work
async fn execute_task_work(repo: &str, issue: &GitHubIssue, worker_id: &str) -> Result<()> {
    println!("🔧 Working on task #{}: {}", issue.number, issue.title);

    // Remove help wanted label
    remove_help_wanted_label(repo, issue.number)?;

    // Claim the issue
    claim_issue(repo, issue.number, worker_id)?;

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
        ("workspace".to_string(), workspace),
    ];

    // Check for original issue reference
    if let Ok(Some(original_id)) = get_original_issue_id(repo, issue.number) {
        params.push(("original_issue".to_string(), original_id.to_string()));
    }

    // Run the worker recipe
    let recipe_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/goose-swarm/swarm_worker.yaml");

    run_recipe(recipe_path, params).await
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
