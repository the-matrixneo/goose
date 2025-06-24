use crate::bench_config::{BenchEval, BenchModel, BenchRunConfig};
use crate::bench_session::BenchAgent;
use crate::bench_work_dir::BenchmarkWorkDir;
use crate::eval_suites::{EvaluationSuite, ExtensionRequirements};
use crate::reporting::EvaluationResult;
use crate::utilities::await_process_exits;
use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use mcp_core::Tool;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use std::collections::VecDeque;

// Agent pool for reusing agents across dataset rows
#[derive(Clone)]
struct AgentPool {
    agents: Arc<Mutex<VecDeque<BenchAgent>>>,
    max_size: usize,
}

impl AgentPool {
    fn new(size: usize) -> Self {
        Self {
            agents: Arc::new(Mutex::new(VecDeque::new())),
            max_size: size,
        }
    }

    async fn get_agent(&self) -> Option<BenchAgent> {
        let mut agents = self.agents.lock().await;
        let agent = agents.pop_front();
        if agent.is_some() {
            tracing::debug!(pool_size = agents.len(), "Agent retrieved from pool");
        }
        agent
    }

    async fn return_agent(&self, mut agent: BenchAgent) {
        // Reset agent state before returning to pool
        match agent.reset_for_reuse().await {
            Ok(()) => {
                let mut agents = self.agents.lock().await;
                if agents.len() < self.max_size {
                    agents.push_back(agent);
                    tracing::debug!(pool_size = agents.len(), "Agent returned to pool");
                } else {
                    tracing::debug!("Pool full, discarding agent");
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to reset agent, discarding");
            }
        }
    }

}

// Configuration struct to reduce function parameter count
#[derive(Clone)]
struct RowProcessingConfig<'a> {
    dataset_config: &'a crate::bench_config::BenchDataset,
    selector: String,
    agent_pool: AgentPool,
    all_tools: Option<HashMap<String, Vec<Tool>>>,
    shared_table_name: Option<String>,
}

// Stateless functions for parallel processing
async fn process_single_dataset_row_stateless<F, Fut>(
    idx: usize,
    row: serde_json::Value,
    agent_generator: F,
    config: RowProcessingConfig<'_>,
) -> serde_json::Value
where
    F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut,
    Fut: Future<Output = BenchAgent> + Send,
{
    let start_time = std::time::Instant::now();
    let (result, timed_out) = async {
        let now_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get current timestamp")?
            .as_nanos();

        let session_id = format!("{}-{}-row{}", config.selector, now_stamp, idx);

        // Use pre-scanned tools if available, otherwise parse from row
        let extensions = if let Some(ref all_tools) = config.all_tools {
            all_tools.clone()
        } else {
            parse_extensions_stateless(&row, &config.dataset_config.tools_column)?
        };

        // Try to get an agent from the pool, otherwise create a new one
        let mut agent = if let Some(pooled_agent) = config.agent_pool.get_agent().await {
            tracing::debug!(
                row_idx = idx,
                "Retrieved agent from pool"
            );
            pooled_agent
        } else {
            tracing::debug!(row_idx = idx, "No agent available in pool, creating new one");
            // Set environment variables for shared vector database if available
            let _env_guards = if let Some(table_name) = &config.shared_table_name {
                let router_guard = env::var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
                    .map(|_| None)
                    .unwrap_or_else(|_| {
                        env::set_var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY", "vector");
                        Some("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
                    });
                
                // Set table name as a temporary environment variable for the agent
                env::set_var("GOOSE_BENCH_SHARED_TABLE", table_name);
                vec![router_guard, Some("GOOSE_BENCH_SHARED_TABLE")]
            } else {
                vec![]
            };
            
            let agent = agent_generator(
                ExtensionRequirements::default(),
                session_id,
                true,
                Some(extensions),
            )
            .await;
            
            // Clean up environment variables
            for guard in _env_guards.into_iter().flatten() {
                env::remove_var(guard);
            }
            
            agent
        };

        if let Some(system_prompt) = get_dataset_system_prompt_stateless(&row, &config.dataset_config.system_prompt_column) {
            agent.session.override_system_prompt(system_prompt).await;
        }
        
        let prompts = match row.get(&config.dataset_config.prompt_column) {
            Some(value) => {
                if let Some(array) = value.as_array() {
                    // Handle array case - new format
                    let prompt_strs: Result<Vec<&str>> = array
                        .iter()
                        .map(|v| v.as_str().ok_or_else(|| anyhow::anyhow!("Prompt array must contain strings")))
                        .collect();
                    prompt_strs?.into_iter().map(|s| s.to_string()).collect()
                } else if let Some(single_str) = value.as_str() {
                    // Handle single string case - backward compatibility
                    vec![
                        single_str.to_string(),
                        "Please immediately do whatever you think is most sensible.".to_string(),
                    ]
                } else {
                    bail!("Prompt field must be either a string or an array of strings");
                }
            }
            None => bail!("Missing prompt field"),
        };
        
        if prompts.is_empty() {
            bail!("Prompt array must contain at least one string");
        }

        // Row-level rate limiting is now handled by the global rate limiter

        // Add timeout to prevent hanging on LLM clarifying questions
        let prompt_timeout = tokio::time::Duration::from_secs(300);
        let result = match tokio::time::timeout(prompt_timeout, async {
            agent.prompt_multi_turn(prompts).await
        }).await {
            Ok(result) => {
                // Log conversation results
                result
            }
            Err(_timeout) => {
                Err(anyhow::anyhow!("Prompt timeout - likely LLM asking for clarification"))
            }
        };

        // Return agent to pool instead of shutting it down
        config.agent_pool.return_agent(agent).await;
        tracing::debug!(
            row_idx = idx,
            "Returned agent to pool"
        );

        Ok::<(Result<Vec<goose::message::Message>>, bool), anyhow::Error>((result, false))
    }
    .await
    .unwrap_or_else(|e: anyhow::Error| {
        (Err(e), false)
    });

    let duration_ms = start_time.elapsed().as_millis() as u64;
    create_dataset_result_row_stateless(row, result, idx, timed_out, duration_ms)
}

fn parse_extensions_stateless(
    row: &serde_json::Value,
    tools_column: &Option<String>,
) -> Result<HashMap<String, Vec<Tool>>> {
    let mut extensions = HashMap::new();
    if let Some(tools_col) = tools_column {
        if let Some(tools_str) = row.get(tools_col).and_then(|v| v.as_str()) {
            extensions = serde_json::from_str(tools_str)?;
        }
    }
    Ok(extensions)
}

fn get_dataset_system_prompt_stateless(
    row: &serde_json::Value,
    system_prompt_column: &Option<String>,
) -> Option<String> {
    system_prompt_column
        .as_ref()
        .and_then(|col| row.get(col))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}


fn create_dataset_result_row_stateless(
    mut row: serde_json::Value,
    result: Result<Vec<goose::message::Message>>,
    idx: usize,
    timed_out: bool,
    duration_ms: u64,
) -> serde_json::Value {
    let obj = row.as_object_mut().unwrap();

    match result {
        Ok(messages) => {
            obj.insert(
                "output".to_string(),
                serde_json::to_value(messages).unwrap(),
            );
            obj.insert("error".to_string(), serde_json::Value::Null);
            obj.insert("success".to_string(), serde_json::Value::Bool(!timed_out));
            // False if timed out
        }
        Err(e) => {
            obj.insert("output".to_string(), serde_json::Value::Null);
            obj.insert(
                "error".to_string(),
                serde_json::Value::String(e.to_string()),
            );
            obj.insert("success".to_string(), serde_json::Value::Bool(false));
        }
    }

    obj.insert(
        "row_index".to_string(),
        serde_json::Value::Number(serde_json::Number::from(idx)),
    );
    obj.insert("timed_out".to_string(), serde_json::Value::Bool(timed_out));
    obj.insert(
        "duration_ms".to_string(),
        serde_json::Value::Number(serde_json::Number::from(duration_ms)),
    );

    if timed_out {
        obj.insert(
            "completion_status".to_string(),
            serde_json::Value::String("partial".to_string()),
        );
    } else {
        obj.insert(
            "completion_status".to_string(),
            serde_json::Value::String("complete".to_string()),
        );
    }

    row
}

#[derive(Clone)]
pub struct EvalRunner {
    config: BenchRunConfig,
}

impl EvalRunner {
    pub fn from(config: String) -> Result<EvalRunner> {
        let config = BenchRunConfig::from_string(config)
            .context("Failed to parse evaluation configuration")?;
        Ok(EvalRunner { config })
    }

    fn create_work_dir(&self, config: &BenchRunConfig) -> Result<BenchmarkWorkDir> {
        let goose_model = config
            .models
            .first()
            .context("No model specified in configuration")?;
        let model_name = goose_model.name.clone();
        let provider_name = goose_model.provider.clone();

        // construct work-dir name to have a shim component only if shim configured to be used
        let work_dir_name_shim = {
            let mut shim_name = "".to_string();
            if let Some(shim_opt) = &goose_model.tool_shim {
                if shim_opt.use_tool_shim {
                    let shim_model = if let Some(shim_model) = &shim_opt.tool_shim_model {
                        shim_model.clone()
                    } else {
                        "default".to_string()
                    };
                    shim_name = format!("-{}-shim-model", shim_model);
                }
            }
            shim_name
        };

        let include_dir = config.include_dirs.clone();
        let work_dir_name = format!("{}-{}{}", provider_name, model_name, work_dir_name_shim);
        let work_dir = BenchmarkWorkDir::new(work_dir_name, include_dir);
        Ok(work_dir)
    }

    /// Helper method to get the dataset config from the global dataset
    fn get_dataset_config(&self) -> Option<&crate::bench_config::BenchDatasetConfig> {
        self.config.dataset.as_ref().and_then(|d| d.config.as_ref())
    }

    pub async fn run<F, Fut>(&mut self, agent_generator: F) -> Result<()>
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut
            + Clone,
        Fut: Future<Output = BenchAgent> + Send,
    {
        let bench_eval = self
            .config
            .evals
            .first()
            .context("No evaluations specified in configuration")?;

        if self.config.dataset.is_some() {
            return self.run_dataset(agent_generator).await;
        }

        let mut work_dir = self
            .create_work_dir(&self.config)
            .context("Failed to create evaluation work directory")?;

        let run_id = &self
            .config
            .run_id
            .clone()
            .unwrap_or_else(|| "run-0".to_string());
        let run_id = format!("run-{}", run_id.clone());

        // create entire dir subtree for eval and cd into dir for running eval
        work_dir.set_eval(&bench_eval.selector, run_id);

        if let Some(eval) = EvaluationSuite::from(&bench_eval.selector) {
            let now_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Failed to get current timestamp")?
                .as_nanos();

            let session_id = format!("{}-{}", bench_eval.selector.clone(), now_stamp);
            let mut agent =
                agent_generator(eval.required_extensions(), session_id, false, None).await;

            let mut result = EvaluationResult::new(eval.name().to_string());

            if let Ok(metrics) = eval.run(&mut agent, &mut work_dir).await {
                for (name, metric) in metrics {
                    result.add_metric(name, metric);
                }
            }

            // Add any errors that occurred
            let errors = agent.get_errors().await;
            for error in errors {
                result.add_error(error);
            }

            // Write results to file
            let eval_results = serde_json::to_string_pretty(&result)
                .context("Failed to serialize evaluation results to JSON")?;

            let eval_results_file = env::current_dir()
                .context("Failed to get current directory")?
                .join(&self.config.eval_result_filename);

            fs::write(&eval_results_file, &eval_results).with_context(|| {
                format!(
                    "Failed to write evaluation results to {}",
                    eval_results_file.display()
                )
            })?;


            self.config.save("config.cfg".to_string());
            work_dir.save();

            // handle running post-process cmd if configured
            if let Some(cmd) = &bench_eval.post_process_cmd {

                let handle = Command::new(cmd)
                    .arg(&eval_results_file)
                    .spawn()
                    .with_context(|| {
                        format!("Failed to execute post-process command: {:?}", cmd)
                    })?;

                await_process_exits(&mut [handle], Vec::new());
            }

            // copy session file into eval-dir
            let here = env::current_dir()
                .context("Failed to get current directory")?
                .canonicalize()
                .context("Failed to canonicalize current directory path")?;

            BenchmarkWorkDir::deep_copy(agent.session_file().as_path(), here.as_path(), false)
                .context("Failed to copy session file to evaluation directory")?;


        } else {
            bail!("No evaluation found for selector: {}", bench_eval.selector);
        }

        Ok(())
    }
    pub async fn run_dataset<F, Fut>(&mut self, agent_generator: F) -> Result<()>
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut
            + Clone,
        Fut: Future<Output = BenchAgent> + Send,
    {
        let bench_eval = self
            .config
            .evals
            .first()
            .context("No evaluations specified")?;
        let dataset_config = self
            .config
            .dataset
            .as_ref()
            .context("No dataset specified")?;

        if dataset_config.prompt_column.is_empty() {
            bail!("Prompt column cannot be empty");
        }

        tracing::info!(path = %dataset_config.path.display(), "Loading dataset file");
        
        let file_content = std::fs::read_to_string(&dataset_config.path)?;
        tracing::info!(
            file_size_mb = file_content.len() as f64 / 1024.0 / 1024.0,
            "Dataset file loaded"
        );
        
        let dataset: serde_json::Value = serde_json::from_str(&file_content)?;
        tracing::info!("Dataset JSON parsed");
        
        let mut rows = dataset
            .as_array()
            .context("Dataset must be an array")?
            .clone();

        // Apply debug size limit BEFORE scanning tools to avoid processing huge datasets
        if let Some(dataset_cfg) = self.get_dataset_config() {
            if let Some(debug_size) = dataset_cfg.debug_size {
                if debug_size > 0 {
                    let limit = debug_size as usize;
                    rows.truncate(limit);
                    tracing::info!(
                        original_size = dataset.as_array().unwrap().len(),
                        debug_size = limit,
                        "Applied debug size limit to dataset"
                    );
                }
            }
        }
        
        tracing::info!(
            final_row_count = rows.len(),
            "Dataset loading completed"
        );

        let mut work_dir = self
            .create_work_dir(&self.config)
            .context("Failed to create evaluation work directory")?;

        let run_id = &self
            .config
            .run_id
            .clone()
            .unwrap_or_else(|| "run-0".to_string());
        let run_id = format!("run-{}", run_id.clone());

        work_dir.set_eval(&bench_eval.selector, run_id);

        // Now scan the TRUNCATED rows to collect all unique tools/extensions
        let mut all_tools: HashMap<String, Vec<Tool>> = HashMap::new();
        if dataset_config.tools_column.is_some() {
            tracing::info!(row_count = rows.len(), "Starting tool scanning");
            for row in &rows {
                if let Ok(tools) = parse_extensions_stateless(&row, &dataset_config.tools_column) {
                    for (ext_name, tools_vec) in tools {
                        let entry = all_tools.entry(ext_name).or_insert_with(Vec::new);
                        // Union tools, avoiding duplicates based on tool name
                        for tool in tools_vec {
                            if !entry.iter().any(|t| t.name == tool.name) {
                                entry.push(tool);
                            }
                        }
                    }
                }
            }
            tracing::info!(
                tools_found = all_tools.len(),
                "Tool scanning completed"
            );
        }

        // Skip shared vector database creation since tool_vectordb module is private
        let shared_table_name: Option<String> = None;

        // Process all rows with maximum parallelism using chunked approach
        let max_concurrent = self
            .get_dataset_config()
            .map(|dc| dc.max_concurrent)
            .unwrap_or(10);


        // Process rows in parallel with rate limiting
        let bench_eval = self
            .config
            .evals
            .first()
            .context("No evaluations specified")?;
        let selector = bench_eval.selector.clone();

        // Initialize global rate limiter if configured
        if let Some(dataset_cfg) = self.get_dataset_config() {
            if let Some(rate_limit_rps) = dataset_cfg.rate_limit_rps {
                // Use a reasonable burst capacity (2x the rate or minimum 10)
                let burst_capacity = ((rate_limit_rps * 2.0).ceil() as u32).max(10);
                crate::rate_limiter::initialize_global_rate_limiter(rate_limit_rps, Some(burst_capacity));
            } else {
                crate::rate_limiter::disable_global_rate_limiter();
            }
        } else {
            crate::rate_limiter::disable_global_rate_limiter();
        }

        // Create and pre-warm agent pool with size equal to max_concurrent
        let agent_pool = AgentPool::new(max_concurrent);
        
        // Pre-warm the pool by creating all agents upfront
        tracing::info!(pool_size = max_concurrent, "Pre-warming agent pool");
        
        // Create progress bar for agent pool warming
        let warmup_progress = ProgressBar::new(max_concurrent as u64);
        warmup_progress.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.green/blue} {pos}/{len} agents ({percent}%) [{elapsed_precise}] {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        warmup_progress.set_message("Initializing agents with extensions");
        
        for i in 0..max_concurrent {
            let warmup_session_id = format!("{}-warmup-{}", selector, i);
            
            // Set environment variables for shared vector database if available
            let _env_guards = if let Some(table_name) = &shared_table_name {
                let router_guard = env::var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
                    .map(|_| None)
                    .unwrap_or_else(|_| {
                        env::set_var("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY", "vector");
                        Some("GOOSE_ROUTER_TOOL_SELECTION_STRATEGY")
                    });
                
                env::set_var("GOOSE_BENCH_SHARED_TABLE", table_name);
                vec![router_guard, Some("GOOSE_BENCH_SHARED_TABLE")]
            } else {
                vec![]
            };
            
            let agent = agent_generator(
                ExtensionRequirements::default(),
                warmup_session_id,
                true,
                if all_tools.is_empty() { None } else { Some(all_tools.clone()) },
            )
            .await;
            
            // Clean up environment variables
            for guard in _env_guards.into_iter().flatten() {
                env::remove_var(guard);
            }
            
            // Add the pre-warmed agent to the pool
            agent_pool.return_agent(agent).await;
            
            // Update progress
            warmup_progress.inc(1);
            let progress_msg = format!("Agent {} ready", i + 1);
            warmup_progress.set_message(progress_msg);
            
            tracing::debug!(agent_index = i, "Agent pre-warmed and added to pool");
        }
        
        // Finish progress bar
        warmup_progress.finish_with_message("Agent pool ready for processing");
        
        tracing::info!(
            pool_size = max_concurrent,
            "Agent pool pre-warming completed"
        );

        let mut all_results = Vec::new();
        let chunk_size = max_concurrent;


        // Create progress bar
        let progress = ProgressBar::new(rows.len() as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos}/{len} rows ({percent}%) [{elapsed_precise}]")
                .unwrap()
                .progress_chars("##-"),
        );
        progress.set_message("Processing dataset");

        tracing::info!(total_rows = rows.len(), "Starting row processing");

        for (chunk_num, chunk) in rows.chunks(chunk_size).enumerate() {
            tracing::debug!(chunk_num = chunk_num, chunk_size = chunk.len(), "Processing chunk");

            let futures: Vec<_> = chunk
                .iter()
                .enumerate()
                .map(|(chunk_idx, row)| {
                    let global_idx = chunk_num * chunk_size + chunk_idx;
                    let pool_clone = agent_pool.clone();
                    process_single_dataset_row_stateless(
                        global_idx,
                        row.clone(),
                        agent_generator.clone(),
                        RowProcessingConfig {
                            dataset_config,
                            selector: selector.clone(),
                            agent_pool: pool_clone,
                            all_tools: if all_tools.is_empty() { None } else { Some(all_tools.clone()) },
                            shared_table_name: shared_table_name.clone(),
                        },
                    )
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;
            all_results.extend(chunk_results);
            
            tracing::info!(
                chunk_num = chunk_num,
                rows_processed = chunk.len(),
                "Chunk processing completed"
            );

            // Update progress bar by the number of rows completed in this chunk
            progress.inc(chunk.len() as u64);

            // Chunk delays are now handled by the global rate limiter
        }
        
        tracing::info!(
            total_rows_processed = rows.len(),
            "All row processing completed"
        );

        // Finish progress bar
        progress.finish_with_message("Dataset processing complete");

        self.write_dataset_results(&bench_eval.selector, vec![all_results])
            .await
    }

    async fn write_dataset_results(
        &self,
        selector: &str,
        results: Vec<Vec<serde_json::Value>>,
    ) -> Result<()> {
        let results_path = format!("{}_results.json", selector);
        std::fs::write(&results_path, serde_json::to_string_pretty(&results)?)?;
        Ok(())
    }

    pub fn path_for_eval(model: &BenchModel, eval: &BenchEval, run_id: String) -> PathBuf {
        let provider = model.provider.clone();
        let model = model.name.clone();
        let eval_path = &eval.selector.replace(":", std::path::MAIN_SEPARATOR_STR);
        let eval_results_location = format!(
            "{}-{}/run-{}{}{}",
            &provider,
            model,
            run_id,
            std::path::MAIN_SEPARATOR_STR,
            eval_path
        );
        PathBuf::from(eval_results_location.clone())
    }

}
