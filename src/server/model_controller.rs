use std::collections::HashMap;
use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

use crate::auth::AuthManager;
use crate::engines::EngineRegistry;
use crate::storage::IdMode;

pub struct AppState {
    pub auth: Arc<AuthManager>,
    pub engines: Arc<EngineRegistry>,
}

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

pub async fn post_model_handler(
    State(state): State<Arc<AppState>>,
    Path((model, namespace, id)): Path<(String, String, String)>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let model_type = model.to_uppercase();
    match model_type.as_str() {
        "VECTOR" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            let vec_data = parsed
                .get("vector")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_f64().map(|f| f as f32)).collect())
                .unwrap_or_else(|| vec![0.0]);
            let meta = parsed.get("metadata").cloned();
            state.engines.vector.insert_vector(&namespace, &id, vec_data, meta);
            StatusCode::CREATED.into_response()
        }
        "GRAPH" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            state.engines.graph.add_node(&namespace, &id, parsed);
            StatusCode::CREATED.into_response()
        }
        "TIMESERIES" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            let ts = id.parse::<i64>().unwrap_or_else(|_| chrono::Utc::now().timestamp_millis());
            state.engines.timeseries.insert(&namespace, ts, parsed);
            StatusCode::CREATED.into_response()
        }
        "COLUMN" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            state.engines.column.insert_row(&namespace, &id, parsed);
            StatusCode::CREATED.into_response()
        }
        "KEYVALUE" => {
            state.engines.keyvalue.put(&namespace, &id, &body);
            StatusCode::CREATED.into_response()
        }
        "GEOSPATIAL" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            let lat = parsed.get("lat").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let lon = parsed.get("lon").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let meta = parsed.get("metadata").cloned();
            state.engines.geospatial.insert_location(&namespace, &id, lat, lon, meta);
            StatusCode::CREATED.into_response()
        }
        "OBJECT" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            let class_name = parsed.get("_class").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let state_val = parsed.get("state").cloned().unwrap_or(parsed);
            state.engines.object.save_object(&namespace, &id, &class_name, state_val);
            StatusCode::CREATED.into_response()
        }
        "RECORDS" => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            let record_class = parsed
                .get("_recordClass")
                .or_else(|| parsed.get("_class"))
                .and_then(|v| v.as_str())
                .unwrap_or("Record");
            let components = parsed.get("components").cloned().unwrap_or_else(|| parsed.clone());
            let schema = parsed.get("_schema").cloned();
            state.engines.records.save_record(&namespace, &id, record_class, components, schema);
            StatusCode::CREATED.into_response()
        }
        "DOCUMENT" | _ => {
            let parsed: Value = serde_json::from_str(&body).unwrap_or(json!({}));
            state.engines.document.insert(&namespace, Some(&id), parsed, IdMode::Manual);
            StatusCode::CREATED.into_response()
        }
    }
}

pub async fn get_model_handler(
    State(state): State<Arc<AppState>>,
    Path((model, namespace, id)): Path<(String, String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let model_type = model.to_uppercase();
    match model_type.as_str() {
        "VECTOR" => match state.engines.vector.get_vector(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "GRAPH" => match state.engines.graph.get_node(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "TIMESERIES" => {
            let ts = id.parse::<i64>().unwrap_or(0);
            match state.engines.timeseries.get(&namespace, ts) {
                Some(v) => (StatusCode::OK, Json(v)).into_response(),
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
        "COLUMN" => match state.engines.column.get_row(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "KEYVALUE" => match state.engines.keyvalue.get(&namespace, &id) {
            Some(val) => (StatusCode::OK, val).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "GEOSPATIAL" => match state.engines.geospatial.get_location(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "OBJECT" => match state.engines.object.get_object(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "RECORDS" => {
            if let Some(fields_str) = params.get("fields") {
                let fields: Vec<String> = fields_str.split(',').map(|s| s.trim().to_string()).collect();
                match state.engines.records.project_fields(&namespace, &id, &fields) {
                    Some(v) => (StatusCode::OK, Json(v)).into_response(),
                    None => StatusCode::NOT_FOUND.into_response(),
                }
            } else {
                match state.engines.records.get_record(&namespace, &id) {
                    Some(v) => (StatusCode::OK, Json(v)).into_response(),
                    None => StatusCode::NOT_FOUND.into_response(),
                }
            }
        }
        "DOCUMENT" | _ => match state.engines.document.get(&namespace, &id) {
            Some(v) => (StatusCode::OK, Json(v)).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
    }
}

pub async fn delete_model_handler(
    State(state): State<Arc<AppState>>,
    Path((model, namespace, id)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let model_type = model.to_uppercase();
    match model_type.as_str() {
        "RECORDS" => state.engines.records.delete_record(&namespace, &id),
        "OBJECT" => state.engines.object.delete_object(&namespace, &id),
        "KEYVALUE" => state.engines.keyvalue.delete(&namespace, &id),
        "GEOSPATIAL" => state.engines.geospatial.delete_location(&namespace, &id),
        "COLUMN" => state.engines.column.delete_row(&namespace, &id),
        "TIMESERIES" => {
            let ts = id.parse::<i64>().unwrap_or(0);
            state.engines.timeseries.delete(&namespace, ts);
        }
        "GRAPH" => state.engines.graph.delete_node(&namespace, &id),
        "VECTOR" => state.engines.vector.delete_vector(&namespace, &id),
        "DOCUMENT" | _ => state.engines.document.delete(&namespace, &id),
    }

    StatusCode::NO_CONTENT.into_response()
}

pub async fn list_model_handler(
    State(state): State<Arc<AppState>>,
    Path((model, namespace)): Path<(String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !validate_auth(&headers, &state.auth) {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response();
    }

    let model_type = model.to_uppercase();
    match model_type.as_str() {
        "DOCUMENT" => (StatusCode::OK, Json(state.engines.document.list(&namespace))).into_response(),
        "GRAPH" => (StatusCode::OK, Json(state.engines.graph.list_nodes(&namespace))).into_response(),
        "TIMESERIES" => (StatusCode::OK, Json(state.engines.timeseries.list(&namespace))).into_response(),
        "COLUMN" => (StatusCode::OK, Json(state.engines.column.list(&namespace))).into_response(),
        "KEYVALUE" => (StatusCode::OK, Json(state.engines.keyvalue.list(&namespace))).into_response(),
        "GEOSPATIAL" => (StatusCode::OK, Json(state.engines.geospatial.list(&namespace))).into_response(),
        "OBJECT" => (StatusCode::OK, Json(state.engines.object.list(&namespace))).into_response(),
        "RECORDS" => (StatusCode::OK, Json(state.engines.records.list(&namespace))).into_response(),
        _ => (StatusCode::OK, Json(json!({}))).into_response(),
    }
}

pub async fn put_model_handler(
    state: State<Arc<AppState>>,
    path: Path<(String, String, String)>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    post_model_handler(state, path, headers, body).await
}

