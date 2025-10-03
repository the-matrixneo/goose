use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
    Registry,
};

use goose::tracing::{langfuse_layer, otlp_layer};

/// Returns the directory where log files should be stored.
/// Creates the directory structure if it doesn't exist.
fn get_log_directory() -> Result<PathBuf> {
    goose::logging::get_log_directory("server", true)
}

/// Sets up the logging infrastructure for the application.
/// This includes:
/// - File-based logging with JSON formatting (DEBUG level)
/// - Console output for development (INFO level)
/// - Optional Langfuse integration (DEBUG level)
pub fn setup_logging(name: Option<&str>) -> Result<()> {
    // Set up file appender for goose module logs
    let log_dir = get_log_directory()?;
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();

    // Create log file name by prefixing with timestamp
    let log_filename = if name.is_some() {
        format!("{}-{}.log", timestamp, name.unwrap())
    } else {
        format!("{}.log", timestamp)
    };

    // Create non-rolling file appender for detailed logs
    let file_appender =
        tracing_appender::rolling::RollingFileAppender::new(Rotation::NEVER, log_dir, log_filename);

    // Create JSON file logging layer
    let file_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_writer(file_appender)
        .with_ansi(false)
        .with_file(true);

    // Create console logging layer for development - INFO and above only
    let console_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    // Base filter for all logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Set default levels for different modules
        EnvFilter::new("")
            // Set mcp-client to DEBUG
            .add_directive("mcp_client=debug".parse().unwrap())
            // Set goose module to DEBUG
            .add_directive("goose=debug".parse().unwrap())
            // Set goose-server to INFO
            .add_directive("goose_server=info".parse().unwrap())
            // Set tower-http to INFO for request logging
            .add_directive("tower_http=info".parse().unwrap())
            // Set everything else to WARN
            .add_directive(LevelFilter::WARN.into())
    });

    let mut layers = vec![
        file_layer.with_filter(env_filter).boxed(),
        console_layer.with_filter(LevelFilter::INFO).boxed(),
    ];

    if let Ok((otlp_tracing_layer, otlp_metrics_layer)) = otlp_layer::init_otlp() {
        layers.push(
            otlp_tracing_layer
                .with_filter(otlp_layer::create_otlp_tracing_filter())
                .boxed(),
        );
        layers.push(
            otlp_metrics_layer
                .with_filter(otlp_layer::create_otlp_metrics_filter())
                .boxed(),
        );
    }

    if let Some(langfuse) = langfuse_layer::create_langfuse_observer() {
        layers.push(langfuse.with_filter(LevelFilter::DEBUG).boxed());
    }

    let subscriber = Registry::default().with(layers);

    subscriber
        .try_init()
        .context("Failed to set global subscriber")?;

    Ok(())
}
