use std::collections::HashMap;
use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

use super::model_controller::AppState;
use crate::auth::AuthManager;
use crate::storage::IdMode;

fn validate_auth(headers: &HeaderMap, auth: &AuthManager) -> bool {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(val) = auth_header.to_str() {
            if let Some(token) = val.strip_prefix("Bearer ") {
                return auth.validate_token(token.trim());
            }
        }
    }
    false
}

pub async fn post_document_auto_id_handler(
    State(state): State<Arc<AppState>>,
    Path(collection): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let id = state.engines.document.insert(&collection, None, payload, IdMode::Uuid);
    (StatusCode::CREATED, Json(json!({ "id": id, "collection": collection }))).into_response()
}

pub async fn post_document_id_handler(
    State(state): State<Arc<AppState>>,
    Path((collection, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let mode_str = params.get("id_mode").map(|s| s.as_str()).unwrap_or("manual");
    let id_mode = IdMode::from_str(mode_str);

    let resolved_id = state.engines.document.insert(&collection, Some(&id), payload, id_mode);
    (StatusCode::CREATED, Json(json!({ "id": resolved_id, "collection": collection }))).into_response()
}

pub async fn get_document_handler(
    State(state): State<Arc<AppState>>,
    Path((collection, id)): Path<(String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    match state.engines.document.get(&collection, &id) {
        Some(doc) => (StatusCode::OK, Json(doc)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_document_handler(
    State(state): State<Arc<AppState>>,
    Path((collection, id)): Path<(String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    state.engines.document.delete(&collection, &id);
    StatusCode::NO_CONTENT.into_response()
}

pub async fn get_document_history_handler(
    State(state): State<Arc<AppState>>,
    Path((collection, id)): Path<(String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let history = state.engines.document.get_history(&collection, &id);
    (StatusCode::OK, Json(history)).into_response()
}

pub async fn post_document_restore_handler(
    State(state): State<Arc<AppState>>,
    Path((collection, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let ts = params.get("timestamp").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    if state.engines.document.restore_version(&collection, &id, ts) {
        (StatusCode::OK, Json(json!({ "status": "Restored successfully", "id": id, "timestamp": ts }))).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "Version not found" }))).into_response()
    }
}

