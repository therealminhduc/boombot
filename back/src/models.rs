use serde::{Deserialize, Serialize};

// Request/Response types for the API
#[derive(Debug, Deserialize)]
pub struct SubmissionRequest {
    pub domain: String,
    pub keys: Vec<String>,
    pub starts_with: Option<Vec<String>>,
    pub contributor: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

// Re-export the DomainRule from cleaner crate for convenience
pub use cleaner::database::DomainRule;