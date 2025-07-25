use axum::{
    Json,
    extract::{Path, State},
};
use crate::models::{SubmissionRequest, ApiResponse};
use crate::validation::is_valid_domain;
use crate::AppState;

/// Get all domain rules
pub async fn get_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<cleaner::database::DomainRule>>> {
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
            error: Some(format!("Failed to fetch rules: {e}")),
        }),
    }
}

/// Submit a new domain rule for review
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

    if payload.contributor.trim().is_empty() {
        return Json(ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some("Contributor is required".to_string()),
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

    // Create new rule using the cleaner crate's DomainRule
    let new_rule = cleaner::database::DomainRule {
        id: None,
        domain: payload.domain.to_lowercase(),
        keys: payload.keys,
        starts_with,
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
            error: Some(format!("Failed to submit rule: {e}")),
        }),
    }
}

/// Approve a pending rule
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
            error: Some(format!("Failed to approve rule: {e}")),
        }),
    }
}

/// Reject a pending rule
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
            error: Some(format!("Failed to reject rule: {e}")),
        }),
    }
}

/// Get approved rules
pub async fn get_approved_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<cleaner::database::DomainRule>>> {
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
            error: Some(format!("Failed to fetch approved rules: {e}")),
        }),
    }
}

/// Get pending rules
pub async fn get_pending_rules(State(state): State<AppState>) -> Json<ApiResponse<Vec<cleaner::database::DomainRule>>> {
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
            error: Some(format!("Failed to fetch pending rules: {e}")),
        }),
    }
}
