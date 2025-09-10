use crate::commands::goose_swarm::repo::{
    add_help_wanted_label, check_open_prs, claim_issue, close_planning_issue, close_pr,
    close_task_issues, count_available_nodes, count_task_issues, create_evaluation_issue,
    create_planning_issue, create_task_issue, get_all_issues, get_help_wanted_issues,
    get_issue_context, get_original_issue_id, get_prs_for_tasks, get_ready_prs_for_tasks,
    get_task_issue_numbers, is_issue_claimed, register_node, remove_help_wanted_label,
    unclaim_issue, GitHubIssue,
};
use crate::session::{build_session, SessionBuilderConfig};
use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

// Embed recipe YAML files at compile time
const PLAN_WORK_RECIPE: &str = include_str!("goose_swarm/plan_work.yaml");
const SWARM_DRONE_RECIPE: &str = include_str!("goose_swarm/swarm_drone.yaml");
const EVALUATE_RECIPE: &str = include_str!("goose_swarm/evaluate.yaml");

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

#[derive(Debug)]
enum WorkType {
    Planning(GitHubIssue), // Planning issue that needs breakdown
    Task(GitHubIssue),     // Task ready to execute
    None,                  // No work available
}

/// Generate a unique worker ID with fun names
fn generate_worker_id() -> String {
    // Fun adjectives for the first word
    let adjectives = [
        "happy", "brave", "clever", "swift", "mighty", "gentle", "bright", "eager", "noble",
        "bold", "wise", "keen", "quick", "sharp", "calm", "cool", "smart", "lucky", "proud",
        "strong", "wild", "free", "grand", "true", "fair", "kind", "warm", "neat", "glad", "busy",
        "spry", "jolly",
    ];

    // Fun nouns for the last word
    let nouns = [
        "fox", "owl", "bee", "cat", "dog", "bat", "ant", "elk", "emu", "jay", "hawk", "wolf",
        "bear", "lion", "deer", "duck", "crow", "dove", "fish", "frog", "goat", "hare", "moth",
        "newt", "orca", "puma", "quail", "raven", "seal", "swan", "toad", "vole", "wasp", "yak",
        "lynx", "mole", "otter",
    ];

    let adj_idx = rand::random::<usize>() % adjectives.len();
    let noun_idx = rand::random::<usize>() % nouns.len();
    let timestamp = chrono::Utc::now().format("%m%d");

    format!("{}-{}-{}", adjectives[adj_idx], timestamp, nouns[noun_idx])
}

/// Load or generate worker ID, persisting it to .node-id file
fn get_or_create_worker_id(provided_id: Option<String>) -> Result<String> {
    // If explicitly provided, use that
    if let Some(id) = provided_id {
        return Ok(id);
    }

    // Check for .node-id file in current directory
    let node_id_path = Path::new(".node-id");

    if node_id_path.exists() {
        // Try to read existing ID
        match fs::read_to_string(node_id_path) {
            Ok(contents) => {
                let id = contents.trim().to_string();
                if !id.is_empty() {
                    println!("📎 Loaded worker ID from .node-id: {}", id);
                    return Ok(id);
                }
            }
            Err(e) => {
                println!("⚠️ Failed to read .node-id: {}", e);
            }
        }
    }

    // Generate new ID
    let new_id = generate_worker_id();

    // Try to save it
    if let Err(e) = fs::write(node_id_path, &new_id) {
        println!("⚠️ Failed to save .node-id: {}", e);
        println!("   Worker ID will be regenerated on next run");
    } else {
        println!("💾 Saved worker ID to .node-id for future runs");
    }

    Ok(new_id)
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

            create_task_issue(repo, parent_issue, title, &content)?;
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

            create_planning_issue(repo, parent_issue, title, &content)?;
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

            create_evaluation_issue(repo, parent_issue, title, &content)?;
        }
    }

    Ok(())
}

/// Execute planning work
#[allow(clippy::too_many_lines)]
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
    println!("🎯 Running planning recipe to break down the issue...");
    run_recipe(PLAN_WORK_RECIPE, params).await?;

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
            println!("🔮 Running evaluation recipe...");
            run_recipe(EVALUATE_RECIPE, eval_params.clone()).await?;

            // Process any new issues created by evaluate recipe
            process_evaluation_issues(repo, issue.number, &eval_issues_dir)?;

            // Check if all PRs are closed/merged
            let remaining_open_prs = check_open_prs(repo, &task_issue_numbers)?;

            if !remaining_open_prs.is_empty() {
                println!(
                    "⚠️ {} PR(s) still open after evaluation. Running evaluation once more...",
                    remaining_open_prs.len()
                );

                // Run evaluation one more time to give it a chance to address open PRs
                run_recipe(EVALUATE_RECIPE, eval_params).await?;

                // Process any new issues from second evaluation
                process_evaluation_issues(repo, issue.number, &eval_issues_dir)?;

                // Check PRs again
                let final_open_prs = check_open_prs(repo, &task_issue_numbers)?;

                if !final_open_prs.is_empty() {
                    println!(
                        "📝 {} PR(s) still open. Closing them now...",
                        final_open_prs.len()
                    );

                    // Close all remaining open PRs
                    for pr in final_open_prs {
                        if let Some(pr_num) = pr.get("number").and_then(|n| n.as_u64()) {
                            close_pr(
                                repo,
                                pr_num,
                                "Automatically closed by Goose Swarm after evaluation phase.",
                            )?;
                        }
                    }
                }
            }

            // Close all task issues that were created
            close_task_issues(repo, issue.number)?;

            // Close the original planning issue
            close_planning_issue(repo, issue.number)?;

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
    run_recipe(SWARM_DRONE_RECIPE, params).await?;

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
            add_help_wanted_label(repo, issue.number)?;

            return Err(anyhow::anyhow!("Failed to complete task - no PR created"));
        } else {
            println!("✅ Successfully created PR on branch: {}", branch);
        }
    }

    Ok(())
}

/// Run a recipe with given parameters
async fn run_recipe(recipe_content: &str, params: Vec<(String, String)>) -> Result<()> {
    // Parse the YAML content directly
    let recipe: goose::recipe::Recipe =
        serde_yaml::from_str(recipe_content).context("Failed to parse recipe YAML")?;

    // Build the prompt from recipe template and parameters
    let prompt = if let Some(mut template) = recipe.prompt.clone() {
        for (key, value) in &params {
            let placeholder = format!("{{{{ {} }}}}", key);
            template = template.replace(&placeholder, value);
        }
        template
    } else {
        return Err(anyhow::anyhow!("Recipe does not have a prompt template"));
    };

    // Convert recipe Settings to SessionSettings
    let session_settings = recipe
        .settings
        .clone()
        .map(|s| crate::session::SessionSettings {
            goose_provider: s.goose_provider,
            goose_model: s.goose_model,
            temperature: s.temperature,
        });

    // Create input config from the recipe
    let input_config = crate::cli::InputConfig {
        contents: Some(prompt),
        extensions_override: recipe.extensions.clone(),
        additional_system_prompt: None,
    };

    // Create recipe info from the recipe
    let recipe_info = crate::cli::RecipeInfo {
        session_settings,
        sub_recipes: recipe.sub_recipes.clone(),
        final_output_response: recipe.response.clone(),
        retry_config: recipe.retry.clone(),
    };

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
    let worker_id = get_or_create_worker_id(args.worker_id)?;

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
