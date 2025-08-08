use super::utils::verify_secret_key;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct EffectiveConfigQuery {
    pub filter: Option<String>,
    pub only_changed: Option<bool>,
    pub include_sources: Option<bool>,
}

#[derive(Serialize)]
pub struct Output {
    pub key: String,
    pub value: serde_json::Value,
    pub redacted: bool,
    pub is_secret: bool,
    pub source: goose::config::unified::Source,
    pub env_name: Option<String>,
    pub alias_used: Option<bool>,
    pub has_default: bool,
}

pub async fn get_effective_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<EffectiveConfigQuery>,
) -> Result<Json<Vec<Output>>, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let entries = goose::config::unified::effective_config(
        q.filter.as_deref(),
        q.only_changed.unwrap_or(false),
        q.include_sources.unwrap_or(false),
    );

    let out: Vec<Output> = entries
        .into_iter()
        .map(|e| Output {
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
