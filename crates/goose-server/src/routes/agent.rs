use super::utils::verify_secret_key;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use goose::config::PermissionManager;
use goose::model::ModelConfig;
use goose::providers::create;
use goose::recipe::Response;
use goose::{
    agents::{extension::ToolInfo, extension_manager::get_parameter_names},
    config::permission::PermissionLevel,
};
use goose::{config::Config, recipe::SubRecipe};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::PathBuf;
use goose::session;
use goose::session::storage::save_messages_with_metadata;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct ExtendPromptRequest {
    extension: String,
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ExtendPromptResponse {
    success: bool,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AddSubRecipesRequest {
    sub_recipes: Vec<SubRecipe>,
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct AddSubRecipesResponse {
    success: bool,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateProviderRequest {
    provider: String,
    model: Option<String>,
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct SessionConfigRequest {
    response: Option<Response>,
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct GetToolsQuery {
    extension_name: Option<String>,
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateRouterToolSelectorRequest {
    #[allow(dead_code)]
    session_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct StartAgentRequest {
    session_id: Option<String>,
    working_dir: Option<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct StartAgentResponse {
    session_id: String,
    working_dir: String,
    success: bool,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    error: String,
}

#[utoipa::path(
    post,
    path = "/agent/add_sub_recipes",
    request_body = AddSubRecipesRequest,
    responses(
        (status = 200, description = "Added sub recipes to agent successfully", body = AddSubRecipesResponse),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
    ),
)]
async fn add_sub_recipes(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<AddSubRecipesRequest>,
) -> Result<Json<AddSubRecipesResponse>, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let agent = state
        .get_agent()
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
    agent.add_sub_recipes(payload.sub_recipes.clone()).await;
    Ok(Json(AddSubRecipesResponse { success: true }))
}

#[utoipa::path(
    post,
    path = "/agent/prompt",
    request_body = ExtendPromptRequest,
    responses(
        (status = 200, description = "Extended system prompt successfully", body = ExtendPromptResponse),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
    ),
)]
async fn extend_prompt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ExtendPromptRequest>,
) -> Result<Json<ExtendPromptResponse>, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let agent = state
        .get_agent()
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
    agent.extend_system_prompt(payload.extension.clone()).await;
    Ok(Json(ExtendPromptResponse { success: true }))
}

#[utoipa::path(
    get,
    path = "/agent/tools",
    params(
        ("extension_name" = Option<String>, Query, description = "Optional extension name to filter tools"),
        ("session_id" = String, Query, description = "Required session ID to scope tools to a specific session")
    ),
    responses(
        (status = 200, description = "Tools retrieved successfully", body = Vec<ToolInfo>),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_tools(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<GetToolsQuery>,
) -> Result<Json<Vec<ToolInfo>>, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let config = Config::global();
    let goose_mode = config.get_param("GOOSE_MODE").unwrap_or("auto".to_string());
    let agent = state
        .get_agent()
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
    let permission_manager = PermissionManager::default();

    let mut tools: Vec<ToolInfo> = agent
        .list_tools(query.extension_name)
        .await
        .into_iter()
        .map(|tool| {
            let permission = permission_manager
                .get_user_permission(&tool.name)
                .or_else(|| {
                    if goose_mode == "smart_approve" {
                        permission_manager.get_smart_approve_permission(&tool.name)
                    } else if goose_mode == "approve" {
                        Some(PermissionLevel::AskBefore)
                    } else {
                        None
                    }
                });

            ToolInfo::new(
                &tool.name,
                tool.description
                    .as_ref()
                    .map(|d| d.as_ref())
                    .unwrap_or_default(),
                get_parameter_names(&tool),
                permission,
            )
        })
        .collect::<Vec<ToolInfo>>();
    tools.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Json(tools))
}

#[utoipa::path(
    post,
    path = "/agent/update_provider",
    request_body = UpdateProviderRequest,
    responses(
        (status = 200, description = "Provider updated successfully"),
        (status = 400, description = "Bad request - missing or invalid parameters"),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_agent_provider(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProviderRequest>,
) -> Result<StatusCode, StatusCode> {
    verify_secret_key(&headers, &state)?;

    let agent = state
        .get_agent()
        .await
        .map_err(|_e| StatusCode::PRECONDITION_FAILED)?;

    let config = Config::global();
    let model = match payload
        .model
        .or_else(|| config.get_param("GOOSE_MODEL").ok())
    {
        Some(m) => m,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let model_config = ModelConfig::new(&model).map_err(|_| StatusCode::BAD_REQUEST)?;

    let new_provider =
        create(&payload.provider, model_config).map_err(|_| StatusCode::BAD_REQUEST)?;
    agent
        .update_provider(new_provider)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/agent/update_router_tool_selector",
    request_body = UpdateRouterToolSelectorRequest,
    responses(
        (status = 200, description = "Tool selection strategy updated successfully", body = String),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_router_tool_selector(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(_payload): Json<UpdateRouterToolSelectorRequest>,
) -> Result<Json<String>, Json<ErrorResponse>> {
    verify_secret_key(&headers, &state).map_err(|_| {
        Json(ErrorResponse {
            error: "Unauthorized - Invalid or missing API key".to_string(),
        })
    })?;

    let agent = state.get_agent().await.map_err(|e| {
        tracing::error!("Failed to get agent: {}", e);
        Json(ErrorResponse {
            error: format!("Failed to get agent: {}", e),
        })
    })?;

    agent
        .update_router_tool_selector(None, Some(true))
        .await
        .map_err(|e| {
            tracing::error!("Failed to update tool selection strategy: {}", e);
            Json(ErrorResponse {
                error: format!("Failed to update tool selection strategy: {}", e),
            })
        })?;

    Ok(Json(
        "Tool selection strategy updated successfully".to_string(),
    ))
}

#[utoipa::path(
    post,
    path = "/agent/session_config",
    request_body = SessionConfigRequest,
    responses(
        (status = 200, description = "Session config updated successfully", body = String),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_session_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SessionConfigRequest>,
) -> Result<Json<String>, Json<ErrorResponse>> {
    verify_secret_key(&headers, &state).map_err(|_| {
        Json(ErrorResponse {
            error: "Unauthorized - Invalid or missing API key".to_string(),
        })
    })?;

    let agent = state.get_agent().await.map_err(|e| {
        tracing::error!("Failed to get agent: {}", e);
        Json(ErrorResponse {
            error: format!("Failed to get agent: {}", e),
        })
    })?;

    if let Some(response) = payload.response {
        agent.add_final_output_tool(response).await;

        tracing::info!("Added final output tool with response config");
        Ok(Json(
            "Session config updated with final output tool".to_string(),
        ))
    } else {
        Ok(Json("Nothing provided to update.".to_string()))
    }
}

#[utoipa::path(
    post,
    path = "/agent/start",
    request_body = StartAgentRequest,
    responses(
        (status = 200, description = "Agent started successfully", body = StartAgentResponse),
        (status = 400, description = "Bad request - invalid working directory"),
        (status = 401, description = "Unauthorized - invalid secret key"),
        (status = 424, description = "Agent not initialized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn start_agent(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<StartAgentRequest>,
) -> Result<Json<StartAgentResponse>, Json<ErrorResponse>> {
    verify_secret_key(&headers, &state).map_err(|_| {
        Json(ErrorResponse {
            error: "Unauthorized - Invalid or missing API key".to_string(),
        })
    })?;

    let (session_id, working_dir) = match payload.session_id {
        Some(existing_session_id) => {
            // Use existing session - get the working directory from session metadata
            let session_path = match session::get_path(session::Identifier::Name(existing_session_id.clone())) {
                Ok(path) => path,
                Err(e) => {
                    tracing::error!("Failed to get session path for {}: {}", existing_session_id, e);
                    return Err(Json(ErrorResponse {
                        error: format!("Failed to get session path: {}", e),
                    }));
                }
            };

            let working_dir = match session::read_metadata(&session_path) {
                Ok(metadata) => metadata.working_dir,
                Err(e) => {
                    tracing::error!("Failed to read session metadata for {}: {}", existing_session_id, e);
                    return Err(Json(ErrorResponse {
                        error: format!("Failed to read session metadata: {}", e),
                    }));
                }
            };

            (existing_session_id, working_dir)
        }
        None => {
            // Create new session - working_dir is required for new sessions
            let working_dir = match payload.working_dir {
                Some(dir) => PathBuf::from(dir),
                None => {
                    return Err(Json(ErrorResponse {
                        error: "working_dir is required when creating a new session".to_string(),
                    }));
                }
            };
            
            let new_session_id = session::generate_session_id();
            let session_path = match session::get_path(session::Identifier::Name(new_session_id.clone())) {
                Ok(path) => path,
                Err(e) => {
                    tracing::error!("Failed to get session path for new session {}: {}", new_session_id, e);
                    return Err(Json(ErrorResponse {
                        error: format!("Failed to create session path: {}", e),
                    }));
                }
            };

            // Initialize the session with empty messages and metadata
            let empty_conversation = goose::conversation::Conversation::new_unvalidated(vec![]);
            let initial_metadata = goose::session::SessionMetadata {
                working_dir: working_dir.clone(),
                description: "New session".to_string(),
                schedule_id: None,
                message_count: 0,
                total_tokens: Some(0),
                input_tokens: Some(0),
                output_tokens: Some(0),
                accumulated_total_tokens: Some(0),
                accumulated_input_tokens: Some(0),
                accumulated_output_tokens: Some(0),
            };
            
            if let Err(e) = save_messages_with_metadata(
                &session_path,
                &initial_metadata,
                &empty_conversation,
            ) {
                tracing::error!("Failed to initialize session {}: {}", new_session_id, e);
                return Err(Json(ErrorResponse {
                    error: format!("Failed to initialize session: {}", e),
                }));
            }
            
            (new_session_id, working_dir)
        }
    };

    // Convert working_dir back to string for response
    let working_dir_str = working_dir.to_string_lossy().to_string();

    tracing::info!(
        "Agent started with session_id: {}, working_dir: {}",
        session_id,
        working_dir_str
    );

    Ok(Json(StartAgentResponse {
        session_id,
        working_dir: working_dir_str,
        success: true,
    }))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/agent/start", post(start_agent))
        .route("/agent/prompt", post(extend_prompt))
        .route("/agent/tools", get(get_tools))
        .route("/agent/update_provider", post(update_agent_provider))
        .route(
            "/agent/update_router_tool_selector",
            post(update_router_tool_selector),
        )
        .route("/agent/session_config", post(update_session_config))
        .route("/agent/add_sub_recipes", post(add_sub_recipes))
        .with_state(state)
}
