use std::time::{SystemTime, UNIX_EPOCH};
use axum::{extract::State, Json};
use bcrypt::verify;
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use crate::AppState;

use crate::{admin_model::AdminLoginRequest, models::ApiResponse};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize
}

const SECRET: &[u8] = b"secret";

impl Claims {
    pub fn new(username: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self {
            sub: username,
            iat: now,
            exp: now + (8 * 60 * 60),
        }
    }
}

pub fn create_jwt(username: String) -> Result<String, Error> {
    let claims = Claims::new(username);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub async fn login_admin(
    State(state): State<AppState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    
    let db = state.db.lock().await;

    match cleaner::database::admin::get_admin_by_username(&db.conn(), &payload.username) {
        Ok(admin) => {
            
            if verify(&payload.password, &admin.password).unwrap_or(false) {

                match create_jwt(admin.username) {
                    Ok(token) => Ok(Json(ApiResponse {
                        success: true,
                        data: Some(token),
                        message: Some("Login successful".to_string()),
                        error: None,
                    })),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                } 
            } else {
                Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Invalid credentials".to_string()),
                    error: Some("INVALID_CREDENTIALS".to_string()),
                }))
            }
        }

        Err(_) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid credentials".to_string()),
            error: Some("INVALID_CREDENTIALS".to_string()),
        }))
    }
}