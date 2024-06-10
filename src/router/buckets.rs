use crate::db::schema::{Access, Token};

use super::{is_authed, AppState};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct CreateBucketRequest {
    pub name: String,
}

pub async fn create(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
    Json(request): Json<CreateBucketRequest>,
) -> impl IntoResponse {
    if !is_authed(headers, None, Access::WRITE, token_cache).await {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let path = Path::new(&state.config.root_directory).join(&request.name);

    if let Ok(_) = tokio::fs::read_dir(&path).await {
        return (
            StatusCode::BAD_REQUEST,
            format!("({}) already exists.", &request.name),
        )
            .into_response();
    }

    if let Err(err) = tokio::fs::create_dir(&path).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    if let Err(err) = tokio::fs::create_dir(&path.join("public")).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    if let Err(err) = tokio::fs::create_dir(&path.join("private")).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    (
        StatusCode::CREATED,
        format!("({}) was created!", &request.name),
    )
        .into_response()
}
