use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::storage::BackupManager;

pub async fn backup_handler(
    State(backup_mgr): State<Arc<BackupManager>>,
) -> impl IntoResponse {
    match backup_mgr.create_backup() {
        Ok(path) => (
            StatusCode::OK,
            Json(json!({
                "status": "Backup created successfully",
                "path": path.to_string_lossy()
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("Backup failed: {}", e)
            })),
        ),
    }
}

