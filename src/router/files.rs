use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use std::{path::Path, sync::Arc};
use super::AppState;

pub async fn upload(
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    let path = Path::new(&state.config.root_directory)
        .join(&bucket_id)
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
            "http://{}:{}/buckets/{bucket_id}/blob/{}",
            &state.config.host_ip, &state.config.port, &resource_path
        ),
    )
        .into_response()
}

pub async fn get(
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let start_extension = resource_path.rfind(".");

    let file_extension = if let Some(start) = start_extension {
        let (_, ext) = resource_path.split_at(start + 1);
        Some(ext)
    } else {
        None
    };

    let resource_path = Path::new(&state.config.root_directory)
        .join(bucket_id)
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
