use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;
use crate::models::{SubmissionRequest, ApiResponse, DomainRule};
use crate::validation::is_valid_domain;
use crate::AppState;

pub async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Get all rules
pub async fn get_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<DomainRule>>> {
    let db = state.db.lock().await;
    match db.get_all_rules() {
        Ok(rules) => Json(ApiResponse {
            success: true,
            data: Some(rules),
            message: None,
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to fetch rules: {}", e)),
        }),
    }
}

/// Submit a new rule
pub async fn submit_rule(
    State(state): State<AppState>,
    Json(payload): Json<SubmissionRequest>,
) -> Json<ApiResponse<()>> {

    // Validate domain format
    if !is_valid_domain(&payload.domain) {
        return Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some("Invalid domain format".to_string()),
        });
    }

    let db = state.db.lock().await;

    // Always include utm_ and add any additional prefixes
    let mut starts_with = vec!["utm_".to_string()];
    if let Some(additional) = payload.starts_with {
        for prefix in additional {
            if prefix != "utm_" {
                starts_with.push(prefix);
            }
        }
    }

    // Create new rule
    let new_rule = DomainRule {
        id: None,
        domain: payload.domain.to_lowercase(),
        keys: payload.keys,
        starts_with, // Empty for user submissions
        contributors: vec![payload.contributor],
        status: "pending".to_string(),
    };

    match db.upsert_rule(&new_rule) {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Rule submitted successfully and is pending review".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to submit rule: {}", e)),
        }),
    }
}

/// Approve a rule
pub async fn approve_rule(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<()>> {
    let db = state.db.lock().await;
    match db.update_rule_status(id, "approved") {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Rule approved".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to approve rule: {}", e)),
        }),
    }
}

/// Reject a rule
pub async fn reject_rule(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<()>> {
    let db = state.db.lock().await;
    match db.update_rule_status(id, "rejected") {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Rule rejected".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to reject rule: {}", e)),
        }),
    }
}

/// Get approved rules
pub async fn get_approved_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<DomainRule>>> {
    let db = state.db.lock().await;

    match db.get_approved_rules_for_api() {
        Ok(rules) => Json(ApiResponse {
            success: true,
            data: Some(rules),
            message: None,
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to fetch approved rules: {}", e)),
        }),
    }
}

/// Get pending rules
pub async fn get_pending_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<DomainRule>>> {
    let db = state.db.lock().await;

    match db.get_pending_rules_for_api() {
        Ok(rules) => Json(ApiResponse {
            success: true,
            data: Some(rules),
            message: None,
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(format!("Failed to fetch pending rules: {}", e)),
        }),
    }
}
