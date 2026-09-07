use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::auth::AuthManager;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: String,
    pub requires_password_change: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

pub async fn login_handler(
    State(auth): State<Arc<AuthManager>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth.login(&payload.username, &payload.password) {
        Ok((token, requires_change)) => (
            StatusCode::OK,
            Json(json!(LoginResponse {
                token,
                user: payload.username,
                requires_password_change: requires_change,
            })),
        ),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": err })),
        ),
    }
}

pub async fn change_password_handler(
    State(auth): State<Arc<AuthManager>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    match auth.change_password(&payload.username, &payload.old_password, &payload.new_password) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "status": "Password changed successfully" })),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        ),
    }
}

