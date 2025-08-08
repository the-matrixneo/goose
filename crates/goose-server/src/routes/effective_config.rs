use super::utils::verify_secret_key;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use goose::config::unified::Source;
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};

/// Query parameters for the effective config endpoint
#[derive(Deserialize, IntoParams)]
pub struct EffectiveConfigQuery {
    /// Optional filter to match against configuration keys
    pub filter: Option<String>,
    /// Only return values that differ from defaults
    pub only_changed: Option<bool>,
    /// Include source information for each configuration value
    pub include_sources: Option<bool>,
}

/// Configuration entry with source information
#[derive(Serialize, ToSchema)]
pub struct EffectiveConfigEntry {
    /// The configuration key
    pub key: String,
    /// The configuration value (may be redacted if secret)
    pub value: serde_json::Value,
    /// Whether this value has been redacted for security
    pub redacted: bool,
    /// Whether this is a secret configuration value
    pub is_secret: bool,
    /// The source of this configuration value
    pub source: Source,
    /// The environment variable name if sourced from environment
    pub env_name: Option<String>,
    /// Whether an alias was used for this configuration
    pub alias_used: Option<bool>,
    /// Whether this configuration has a default value
    pub has_default: bool,
}

/// Get the effective configuration with optional filtering and source information
#[utoipa::path(
    get,
    path = "/config/effective",
    params(EffectiveConfigQuery),
    responses(
        (status = 200, description = "Effective configuration retrieved successfully", body = Vec<EffectiveConfigEntry>),
        (status = 401, description = "Unauthorized - invalid secret key")
    ),
    tag = "Configuration Management"
)]
pub async fn get_effective_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<EffectiveConfigQuery>,
) -> Result<Json<Vec<EffectiveConfigEntry>>, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let entries = goose::config::unified::effective_config(
        q.filter.as_deref(),
        q.only_changed.unwrap_or(false),
        q.include_sources.unwrap_or(false),
    );

    let out: Vec<EffectiveConfigEntry> = entries
        .into_iter()
        .map(|e| EffectiveConfigEntry {
            key: e.key,
            value: e.value,
            redacted: e.redacted,
            is_secret: e.is_secret,
            source: e.source,
            env_name: e.env_name,
            alias_used: e.alias_used,
            has_default: e.has_default,
        })
        .collect();

    Ok(Json(out))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/config/effective", get(get_effective_config))
        .with_state(state)
}
