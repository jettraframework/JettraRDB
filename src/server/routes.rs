use std::sync::Arc;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use super::auth_controller::{change_password_handler, login_handler};
use super::backup_controller::backup_handler;
use super::document_controller::{
    delete_document_handler, get_document_handler, get_document_history_handler,
    post_document_auto_id_handler, post_document_id_handler, post_document_restore_handler,
};
use super::model_controller::{
    delete_model_handler, get_model_handler, list_model_handler, post_model_handler,
    put_model_handler, AppState,
};
use crate::cluster::RaftClusterManager;
use crate::storage::{BackupManager, LsmBTreeHybrid};
use crate::web::WebTemplates;

#[derive(Deserialize)]
pub struct CreateDbRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct RenameDbRequest {
    pub new_name: String,
}

#[derive(Deserialize)]
pub struct ExplorerQuery {
    pub db: Option<String>,
}

#[derive(Clone)]
pub struct ServerContext {
    pub storage: Arc<LsmBTreeHybrid>,
    pub app_state: Arc<AppState>,
    pub backup_mgr: Arc<BackupManager>,
    pub cluster: Arc<RaftClusterManager>,
    pub node_id: String,
    pub rest_port: u16,
    pub gui_port: u16,
    pub grpc_port: u16,
    pub peers: String,
}

pub fn create_router(ctx: ServerContext) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let auth_routes = Router::new()
        .route("/login", post(login_handler))
        .route("/change-password", post(change_password_handler))
        .with_state(ctx.app_state.auth.clone());

    let document_routes = Router::new()
        .route("/{collection}", post(post_document_auto_id_handler))
        .route("/{collection}/{id}", post(post_document_id_handler).get(get_document_handler).delete(delete_document_handler))
        .route("/{collection}/{id}/history", get(get_document_history_handler))
        .route("/{collection}/{id}/restore", post(post_document_restore_handler))
        .with_state(ctx.app_state.clone());

    let model_routes = Router::new()
        .route(
            "/{model}/{namespace}/{id}",
            post(post_model_handler)
                .put(put_model_handler)
                .get(get_model_handler)
                .delete(delete_model_handler),
        )
        .route("/{model}/{namespace}", get(list_model_handler))
        .with_state(ctx.app_state.clone());

    let backup_routes = Router::new()
        .route("/", post(backup_handler))
        .with_state(ctx.backup_mgr.clone());

    let ctx_for_web = ctx.clone();
    let ctx_for_db = ctx.clone();

    Router::new()
        // Web Console HTML routes
        .route("/", get({
            let ctx = ctx_for_web.clone();
            move || async move {
                let dbs = ctx.storage.get_database_names();
                let engines = ctx.app_state.engines.list_engines();
                Html(WebTemplates::dashboard_page(dbs.len(), engines.len(), &ctx.node_id))
            }
        }))
        .route("/dashboard", get({
            let ctx = ctx_for_web.clone();
            move || async move {
                let dbs = ctx.storage.get_database_names();
                let engines = ctx.app_state.engines.list_engines();
                Html(WebTemplates::dashboard_page(dbs.len(), engines.len(), &ctx.node_id))
            }
        }))
        .route("/wui", get({
            let ctx = ctx_for_web.clone();
            move || async move {
                let dbs = ctx.storage.get_database_names();
                let engines = ctx.app_state.engines.list_engines();
                Html(WebTemplates::dashboard_page(dbs.len(), engines.len(), &ctx.node_id))
            }
        }))
        .route("/databases", get({
            let ctx = ctx_for_web.clone();
            move || async move {
                let dbs = ctx.storage.get_database_names();
                Html(WebTemplates::databases_page(&dbs))
            }
        }))
        .route("/explorer", get({
            let ctx = ctx_for_web.clone();
            move |Query(query): Query<ExplorerQuery>| async move {
                let dbs = ctx.storage.get_database_names();
                let active_db = query.db.unwrap_or_else(|| {
                    dbs.first().cloned().unwrap_or_else(|| "default".to_string())
                });
                let records = ctx.storage.get_database_records(&active_db);
                Html(WebTemplates::explorer_page(&dbs, &active_db, &records))
            }
        }))
        .route("/engines", get(|| async { Html(WebTemplates::engines_page()) }))
        .route("/users", get(|| async { Html(WebTemplates::users_page()) }))
        .route("/components", get({
            let ctx = ctx_for_web.clone();
            move || async move {
                Html(WebTemplates::components_page(
                    &ctx.node_id,
                    ctx.grpc_port,
                    ctx.rest_port,
                    &ctx.peers,
                ))
            }
        }))
        .route("/information", get(|| async { Html(WebTemplates::information_page()) }))
        .route("/swagger-ui", get({
            let port = ctx_for_web.rest_port;
            move || async move { Html(WebTemplates::swagger_ui_page(port)) }
        }))
        // Database Management API
        .route("/api/databases", get({
            let ctx = ctx_for_db.clone();
            move || async move {
                let dbs = ctx.storage.get_database_names();
                Json(json!({ "databases": dbs }))
            }
        }).post({
            let ctx = ctx_for_db.clone();
            move |Json(payload): Json<CreateDbRequest>| async move {
                match ctx.storage.create_database(&payload.name) {
                    Ok(()) => (StatusCode::CREATED, Json(json!({ "status": "success", "database": payload.name }))),
                    Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": e }))),
                }
            }
        }))
        .route("/api/databases/{name}/rename", put({
            let ctx = ctx_for_db.clone();
            move |Path(name): Path<String>, Json(payload): Json<RenameDbRequest>| async move {
                match ctx.storage.rename_database(&name, &payload.new_name) {
                    Ok(()) => (StatusCode::OK, Json(json!({ "status": "success", "old_name": name, "new_name": payload.new_name }))),
                    Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": e }))),
                }
            }
        }))
        .route("/api/databases/{name}/records", get({
            let ctx = ctx_for_db.clone();
            move |Path(name): Path<String>| async move {
                let records = ctx.storage.get_database_records(&name);
                Json(json!({ "database": name, "count": records.len(), "records": records }))
            }
        }))
        .route("/api/databases/{name}", delete({
            let ctx = ctx_for_db.clone();
            move |Path(name): Path<String>| async move {
                ctx.storage.drop_database(&name);
                StatusCode::NO_CONTENT
            }
        }))
        // System API
        .route("/api/system/health", get(|| async {
            Json(json!({ "status": "UP", "engine": "JettraRDB (Rust)", "version": "1.0.0" }))
        }))
        .route("/api/system/info", get({
            let ctx = ctx_for_db.clone();
            move || async move {
                Json(json!({
                    "node_id": ctx.node_id,
                    "rest_port": ctx.rest_port,
                    "gui_port": ctx.gui_port,
                    "grpc_port": ctx.grpc_port,
                    "runtime": "Rust / Tokio",
                    "models": 9
                }))
            }
        }))
        // Nested REST APIs
        .nest("/api/auth", auth_routes)
        .nest("/api/document", document_routes)
        .nest("/api/model", model_routes)
        .nest("/api/backup", backup_routes)
        .layer(cors)
}

