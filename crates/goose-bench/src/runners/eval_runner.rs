use crate::bench_config::{BenchEval, BenchModel, BenchRunConfig};
use crate::bench_session::BenchAgent;
use crate::bench_work_dir::BenchmarkWorkDir;
use crate::eval_suites::{EvaluationSuite, ExtensionRequirements};
use crate::reporting::EvaluationResult;
use crate::utilities::await_process_exits;
use anyhow::{bail, Context, Result};
use futures::future::join_all;
use mcp_core::Tool;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing;

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

    pub async fn run<F, Fut>(&mut self, agent_generator: F) -> Result<()>
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut + Clone,
        Fut: Future<Output=BenchAgent> + Send,
    {
        let bench_eval = self
            .config
            .evals
            .first()
            .context("No evaluations specified in configuration")?;

        if bench_eval.dataset.is_some() {
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
        tracing::info!("Set evaluation directory for {}", bench_eval.selector);

        if let Some(eval) = EvaluationSuite::from(&bench_eval.selector) {
            let now_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Failed to get current timestamp")?
                .as_nanos();

            let session_id = format!("{}-{}", bench_eval.selector.clone(), now_stamp);
            let mut agent = agent_generator(eval.required_extensions(),
                                            session_id,
                                            false,
                                            None).await;
            tracing::info!("Agent created for {}", eval.name());

            let mut result = EvaluationResult::new(eval.name().to_string());

            match eval.run(&mut agent, &mut work_dir).await {
                Ok(metrics) => {
                    tracing::info!("Evaluation run successful with {} metrics", metrics.len());
                    for (name, metric) in metrics {
                        result.add_metric(name, metric);
                    }
                }
                Err(e) => {
                    tracing::error!("Evaluation run failed: {}", e);
                }
            }

            // Add any errors that occurred
            let errors = agent.get_errors().await;
            tracing::info!("Agent reported {} errors", errors.len());
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

            tracing::info!(
                "Wrote evaluation results to {}",
                eval_results_file.display()
            );

            self.config.save("config.cfg".to_string());
            work_dir.save();

            // handle running post-process cmd if configured
            if let Some(cmd) = &bench_eval.post_process_cmd {
                tracing::info!("Running post-process command: {:?}", cmd);

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

            tracing::info!("Evaluation completed successfully");
        } else {
            tracing::error!("No evaluation found for selector: {}", bench_eval.selector);
            bail!("No evaluation found for selector: {}", bench_eval.selector);
        }

        Ok(())
    }
    pub async fn run_dataset<F, Fut>(&mut self, agent_generator: F) -> Result<()>
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut + Clone,
        Fut: Future<Output=BenchAgent> + Send,
    {
        let bench_eval = self.config.evals.first().context("No evaluations specified")?;
        let dataset_config = bench_eval.dataset.as_ref().context("No dataset specified")?;

        if dataset_config.prompt_column.is_empty() {
            bail!("Prompt column cannot be empty");
        }

        let dataset: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&dataset_config.path)?)?;
        let conversations = dataset.as_array().context("Dataset must be an array")?;

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
        tracing::info!("Set evaluation directory for {}", bench_eval.selector);


        let mut all_results = Vec::new();
        for conv in conversations {
            let rows = conv.as_array().context("Conversation must be an array")?;
            let processed = self.run_dataset_rows(rows, &agent_generator, dataset_config).await;
            all_results.push(processed);
        }

        self.write_dataset_results(&bench_eval.selector, all_results).await
    }

    async fn run_dataset_rows<F, Fut>(
        &self,
        rows: &[serde_json::Value],
        agent_generator: &F,
        dataset_config: &crate::bench_config::BenchDataset,
    ) -> Vec<serde_json::Value>
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut + Clone,
        Fut: Future<Output=BenchAgent> + Send,
    {
        let futures: Vec<_> = rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                self.process_single_dataset_row(
                    idx,
                    row.clone(),
                    agent_generator.clone(),
                    dataset_config,
                )
            })
            .collect();

        join_all(futures).await
    }

    async fn process_single_dataset_row<F, Fut>(
        &self,
        idx: usize,
        row: serde_json::Value,
        agent_generator: F,
        dataset_config: &crate::bench_config::BenchDataset,
    ) -> serde_json::Value
    where
        F: Fn(ExtensionRequirements, String, bool, Option<HashMap<String, Vec<Tool>>>) -> Fut,
        Fut: Future<Output=BenchAgent> + Send,
    {
        let result = async {
            let now_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Failed to get current timestamp")?
                .as_nanos();

            let bench_eval = self.config.evals.first().context("No evaluations specified")?;
            let session_id = format!("{}-{}", bench_eval.selector.clone(), now_stamp);

            let extensions = self.parse_extensions(&row, &dataset_config.tools_column)?;
            let mut agent = agent_generator(ExtensionRequirements::default(), session_id, true, Some(extensions)).await;

            if let Some(system_prompt) = self.get_dataset_system_prompt(&row, &dataset_config.system_prompt_column) {
                agent.session.override_system_prompt(system_prompt).await;
            }

            let prompt = row.get(&dataset_config.prompt_column).and_then(|v| v.as_str()).context("Missing prompt field")?;
            agent.prompt(prompt.to_string()).await
        }.await;

        self.create_dataset_result_row(row, result, idx)
    }

    fn parse_extensions(&self, row: &serde_json::Value, tools_column: &Option<String>) -> Result<HashMap<String, Vec<Tool>>> {
        let mut extensions = HashMap::new();
        if let Some(tools_col) = tools_column {
            if let Some(tools_str) = row.get(tools_col).and_then(|v| v.as_str()) {
                extensions = serde_json::from_str(tools_str)?;
            }
        }
        Ok(extensions)
    }

    fn get_dataset_system_prompt(&self, row: &serde_json::Value, system_prompt_column: &Option<String>) -> Option<String> {
        system_prompt_column.as_ref()
            .and_then(|col| row.get(col))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn create_dataset_result_row(&self, mut row: serde_json::Value, result: Result<Vec<goose::message::Message>>, idx: usize) -> serde_json::Value {
        let obj = row.as_object_mut().unwrap();

        match result {
            Ok(messages) => {
                obj.insert("output".to_string(), serde_json::to_value(messages).unwrap());
                obj.insert("error".to_string(), serde_json::Value::Null);
                obj.insert("success".to_string(), serde_json::Value::Bool(true));
            }
            Err(e) => {
                obj.insert("output".to_string(), serde_json::Value::Null);
                obj.insert("error".to_string(), serde_json::Value::String(e.to_string()));
                obj.insert("success".to_string(), serde_json::Value::Bool(false));
            }
        }

        obj.insert("row_index".to_string(), serde_json::Value::Number(serde_json::Number::from(idx)));
        row
    }

    async fn write_dataset_results(&self, selector: &str, results: Vec<Vec<serde_json::Value>>) -> Result<()> {
        let results_path = format!("{}_results.json", selector);
        std::fs::write(&results_path, serde_json::to_string_pretty(&results)?)?;
        tracing::info!("Wrote dataset evaluation results to {}", results_path);
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