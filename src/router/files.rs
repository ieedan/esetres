use crate::db::schema::{Access, Token};
use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Extension,
};
use std::{collections::HashMap, path::Path, sync::Arc};

use super::{is_authed, AppState};
use tokio::sync::Mutex;

pub async fn public_upload(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
    body: Bytes,
) -> impl IntoResponse {
    upload(headers, bucket_id, resource_path, state, token_cache, body, false).await
}

pub async fn private_upload(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
    body: Bytes,
) -> impl IntoResponse {
    upload(headers, bucket_id, resource_path, state, token_cache, body, true).await
}

pub async fn upload(
    headers: HeaderMap,
    bucket_id: String,
    resource_path: String,
    state: Arc<AppState>,
    token_cache: Arc<Mutex<HashMap<String, Token>>>,
    body: Bytes,
    private: bool
) -> impl IntoResponse {
    if !is_authed(headers, Some(&bucket_id), Access::WRITE, token_cache).await {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let scope = if private { "private" } else { "public" };

    let path = Path::new(&state.config.root_directory)
        .join(&bucket_id)
        .join(scope)
        .join(&resource_path);

    let bucket_path = Path::new(&state.config.root_directory).join(&bucket_id);

    // Check if bucket exists
    if let Err(_) = tokio::fs::read_dir(&bucket_path).await {
        return (
            StatusCode::NOT_FOUND,
            format!("Bucket: ({bucket_id}) not found."),
        )
            .into_response();
    }

    // Write to the file
    if let Err(err) = tokio::fs::write(&path, body).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    (
        StatusCode::CREATED,
        format!(
            "{}/buckets/{bucket_id}/{scope}/{resource_path}",
            &state.config.address()
        ),
    )
        .into_response()
}

pub async fn private_get(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
) -> impl IntoResponse {
    if !is_authed(headers, Some(&bucket_id), Access::READ, token_cache).await {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    get(bucket_id, resource_path, state, true).await.into_response()
}

pub async fn public_get(
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    get(bucket_id, resource_path, state, false).await
}

pub async fn get(
    bucket_id: String,
    resource_path: String,
    state: Arc<AppState>,
    private: bool
) -> impl IntoResponse {
    let start_extension = resource_path.rfind(".");

    let file_extension = if let Some(start) = start_extension {
        let (_, ext) = resource_path.split_at(start + 1);
        Some(ext)
    } else {
        None
    };

    let scope = if private { "private" } else { "public" };

    let resource_path = Path::new(&state.config.root_directory)
        .join(bucket_id)
        .join(scope)
        .join(&resource_path);

    let buffer = if let Ok(buffer) = tokio::fs::read(&resource_path).await {
        buffer
    } else {
        let mut response = Response::new(Body::from("Not Found".as_bytes().to_vec()));
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

        return response.into_response();
    };

    let content_type = if let Some(ext) = file_extension {
        if let Some(ct) = state.mime_types.get(ext) {
            ct.clone()
        } else {
            "text/plain".to_string()
        }
    } else {
        "text/plain".to_string()
    };

    let mut response = Response::new(Body::from(buffer));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, content_type.parse().unwrap());

    response.into_response()
}
