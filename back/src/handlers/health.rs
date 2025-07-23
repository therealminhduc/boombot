use axum::Json;
use serde_json::Value;

/// Health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}
