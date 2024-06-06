use super::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use std::{path::Path, sync::Arc};

#[derive(Deserialize)]
pub struct CreateBucketRequest {
    pub name: String,
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBucketRequest>,
) -> impl IntoResponse {
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

    (
        StatusCode::CREATED,
        format!("({}) was created!", &request.name),
    )
        .into_response()
}
