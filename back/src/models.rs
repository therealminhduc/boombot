use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainRule {
    pub id: Option<i32>,
    pub domain: String,
    pub keys: Vec<String>,
    pub starts_with: Vec<String>,
    pub contributors: Vec<String>,
    pub status: String,
}

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