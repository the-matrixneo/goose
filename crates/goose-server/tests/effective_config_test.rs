use axum::http::{Request, StatusCode};
use axum::Router;
use goose_server::routes;
use goose_server::state::AppState;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_effective_config_endpoint() {
    // Create app state
    let state = Arc::new(AppState::new());

    // Create router with routes
    let app = Router::new().merge(routes::effective_config::routes(state.clone()));

    // Test without auth - should return 401
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config/effective")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test with auth header
    let secret_key =
        std::env::var("GOOSE_SERVER__SECRET_KEY").unwrap_or_else(|_| "test-secret-key".to_string());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config/effective?filter=llm&only_changed=false&include_sources=true")
                .header("X-Secret-Key", secret_key)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Parse response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let entries: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Verify we got some config entries
    assert!(!entries.is_empty(), "Should return config entries");

    // Check that entries have expected fields
    if let Some(entry) = entries.first() {
        assert!(entry.get("key").is_some());
        assert!(entry.get("value").is_some());
        assert!(entry.get("redacted").is_some());
        assert!(entry.get("is_secret").is_some());
        assert!(entry.get("source").is_some());
        assert!(entry.get("has_default").is_some());
    }
}

#[tokio::test]
async fn test_effective_config_secret_redaction() {
    // Set a test secret in environment
    std::env::set_var("OPENAI_API_KEY", "sk-test-secret-key");

    let state = Arc::new(AppState::new());
    let app = Router::new().merge(routes::effective_config::routes(state.clone()));

    let secret_key =
        std::env::var("GOOSE_SERVER__SECRET_KEY").unwrap_or_else(|_| "test-secret-key".to_string());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/config/effective?filter=providers.openai")
                .header("X-Secret-Key", secret_key)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let entries: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Find the API key entry
    let api_key_entry = entries
        .iter()
        .find(|e| e.get("key").and_then(|k| k.as_str()) == Some("providers.openai.api_key"));

    if let Some(entry) = api_key_entry {
        // Verify the secret is redacted
        assert_eq!(entry.get("redacted").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(entry.get("is_secret").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(entry.get("value").and_then(|v| v.as_str()), Some("***"));
    }

    // Clean up
    std::env::remove_var("OPENAI_API_KEY");
}
