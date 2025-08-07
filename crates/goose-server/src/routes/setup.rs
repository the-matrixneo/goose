use crate::state::AppState;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use goose::config::compat;
use goose::config::signup_openrouter::OpenRouterAuth;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct SetupResponse {
    pub success: bool,
    pub message: String,
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/handle_openrouter", post(start_openrouter_setup))
        .with_state(state)
}

async fn start_openrouter_setup(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<SetupResponse>, StatusCode> {
    tracing::info!("Starting OpenRouter setup flow");

    let mut auth_flow = OpenRouterAuth::new().map_err(|e| {
        tracing::error!("Failed to initialize auth flow: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!("Auth flow initialized, starting complete_flow");

    match auth_flow.complete_flow().await {
        Ok(api_key) => {
            tracing::info!("Got API key, configuring OpenRouter...");

            // Configure OpenRouter using compat functions
            if let Err(e) = compat::set_secret("OPENROUTER_API_KEY", &api_key) {
                tracing::error!("Failed to set OpenRouter API key: {}", e);
                return Ok(Json(SetupResponse {
                    success: false,
                    message: format!("Failed to set OpenRouter API key: {}", e),
                }));
            }

            if let Err(e) = compat::set("GOOSE_PROVIDER", "openrouter") {
                tracing::error!("Failed to set provider: {}", e);
                return Ok(Json(SetupResponse {
                    success: false,
                    message: format!("Failed to set provider: {}", e),
                }));
            }

            // Use the default model from the original function
            const OPENROUTER_DEFAULT_MODEL: &str = "anthropic/claude-3.5-sonnet";
            if let Err(e) = compat::set("GOOSE_MODEL", OPENROUTER_DEFAULT_MODEL) {
                tracing::error!("Failed to set model: {}", e);
                return Ok(Json(SetupResponse {
                    success: false,
                    message: format!("Failed to set model: {}", e),
                }));
            }

            tracing::info!("OpenRouter setup completed successfully");
            Ok(Json(SetupResponse {
                success: true,
                message: "OpenRouter setup completed successfully".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("OpenRouter setup failed: {}", e);
            Ok(Json(SetupResponse {
                success: false,
                message: format!("Setup failed: {}", e),
            }))
        }
    }
}
