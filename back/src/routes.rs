use axum::{
    routing::{get, post, put},
    Router,
};
use crate::handlers::*;

pub fn create_router() -> Router<crate::AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/rules", get(get_rules))
        .route("/api/submit", post(submit_rule))
        .route("/api/rules/{id}/approve", put(approve_rule))
        .route("/api/rules/{id}/reject", put(reject_rule))
        .route("/api/rules/approved", get(get_approved_rules))
        .route("/api/rules/pending", get(get_pending_rules))
        .route("/api/admin/create", post(create_admin))
}