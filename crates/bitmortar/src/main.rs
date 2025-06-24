use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use clap::Parser;
use serde_json::Value;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod config;
mod error;
mod providers;
mod server;
mod types;

use config::BitMortarConfig;
use error::BitMortarError;
use server::BitMortarServer;

#[derive(Parser)]
#[command(name = "bitmortar")]
#[command(about = "A unified API server that provides Databricks-compatible endpoints")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "bitmortar.toml")]
    config: String,

    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = BitMortarConfig::load(&cli.config)?;
    info!("Loaded configuration from {}", cli.config);

    // Create server instance
    let server = Arc::new(BitMortarServer::new(config).await?);

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/serving-endpoints", get(list_endpoints))
        .route(
            "/serving-endpoints/:model/invocations",
            post(chat_completions),
        )
        .route("/v1/embeddings", post(embeddings))
        .route("/v1/models", get(list_models))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(server);

    let addr = format!("{}:{}", cli.host, cli.port);
    info!("Starting BitMortar server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "bitmortar",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn list_endpoints(
    State(server): State<Arc<BitMortarServer>>,
) -> Result<Json<Value>, BitMortarError> {
    let endpoints = server.list_endpoints().await?;
    Ok(Json(serde_json::json!({
        "endpoints": endpoints
    })))
}

async fn chat_completions(
    State(server): State<Arc<BitMortarServer>>,
    Path(model): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, BitMortarError> {
    let response = server.chat_completions(&model, payload).await?;
    Ok(Json(response))
}

async fn embeddings(
    State(server): State<Arc<BitMortarServer>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, BitMortarError> {
    let response = server.embeddings(payload).await?;
    Ok(Json(response))
}

async fn list_models(
    State(server): State<Arc<BitMortarServer>>,
) -> Result<Json<Value>, BitMortarError> {
    let models = server.list_models().await?;
    Ok(Json(serde_json::json!({
        "data": models
    })))
}
