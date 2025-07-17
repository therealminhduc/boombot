use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use crate::admin_model::CreateAdminRequest;
use crate::models::ApiResponse;
use crate::AppState;

/// Create admin user
pub async fn create_admin(
    State(state): State<AppState>,
    Json(payload): Json<CreateAdminRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let db = state.db.lock().await;
    
    // Check if any admin already exists
    match cleaner::database::admin::has_admin(db.conn()) {
        Ok(true) => {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Admin user already exists".to_string()),
                error: Some("ADMIN_EXISTS".to_string()),
            }));
        }
        Ok(false) => {},
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    // Hash the password
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Create the admin user
    match cleaner::database::admin::insert_admin(db.conn(), &payload.username, &password_hash) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Admin user created successfully".to_string()),
            message: Some("Admin user created".to_string()),
            error: None,
        })),
        Err(cleaner::database::admin::AdminError::UsernameExists) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Username already exists".to_string()),
            error: Some("USERNAME_EXISTS".to_string()),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
