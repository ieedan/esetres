use crate::db::schema::{Access, Token};
use axum::{
    extract::{Multipart, State},
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Extension,
};
use std::{collections::HashMap, path::Path, sync::Arc};

use super::{is_authed, AppState};
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};
use tokio_util::io::ReaderStream;

pub async fn public_upload(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
    multipart: Multipart,
) -> impl IntoResponse {
    upload(
        headers,
        bucket_id,
        resource_path,
        state,
        token_cache,
        multipart,
        false,
    )
    .await
}

pub async fn private_upload(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
    multipart: Multipart,
) -> impl IntoResponse {
    upload(
        headers,
        bucket_id,
        resource_path,
        state,
        token_cache,
        multipart,
        true,
    )
    .await
}

pub async fn upload(
    headers: HeaderMap,
    bucket_id: String,
    resource_path: String,
    state: Arc<AppState>,
    token_cache: Arc<Mutex<HashMap<String, Token>>>,
    mut multipart: Multipart,
    private: bool,
) -> impl IntoResponse {
    if !is_authed(headers, Some(&bucket_id), Access::WRITE, token_cache).await {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let mut field = match multipart.next_field().await {
        Ok(Some(field)) => {
            field
        },
        Ok(None) => return (StatusCode::BAD_REQUEST, format!("You forgot to include a file!")).into_response(),
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Error processing multipart: {}", e)).into_response(),
    };

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

    let mut file = match tokio::fs::File::create(&path).await {
        Ok(file) => file,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create file: {}", e)).into_response(),
    };

    // streams the form data into the file
    loop {
        match field.chunk().await {
            Ok(Some(chunk)) => {
                if let Err(e) = file.write_all(&chunk).await {
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write to file: {}", e)).into_response();
                }
            }
            Ok(None) => break, // End of the stream
            Err(e) => {
                println!("Error reading chunk: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file data: {}", e)).into_response();
            }
        }
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

    get(bucket_id, resource_path, state, true)
        .await
        .into_response()
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
    private: bool,
) -> impl IntoResponse {
    let scope = if private { "private" } else { "public" };

    let path = Path::new(&state.config.root_directory)
        .join(bucket_id)
        .join(scope)
        .join(&resource_path);

    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = axum::body::Body::from_stream(stream);

            let mime_type = mime_guess::from_path(&path).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(body)
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap(),
    }
}

pub async fn public_delete(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
) -> impl IntoResponse {
    delete(headers, bucket_id, resource_path, state, token_cache, false).await
}

pub async fn private_delete(
    headers: HeaderMap,
    axum::extract::Path((bucket_id, resource_path)): axum::extract::Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
) -> impl IntoResponse {
    delete(headers, bucket_id, resource_path, state, token_cache, true).await
}

pub async fn delete(
    headers: HeaderMap,
    bucket_id: String,
    resource_path: String,
    state: Arc<AppState>,
    token_cache: Arc<Mutex<HashMap<String, Token>>>,
    private: bool,
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

    // delete the file
    if let Err(err) = tokio::fs::remove_file(&path).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    (
        StatusCode::OK,
        format!(
            "Deleted {}/buckets/{bucket_id}/{scope}/{resource_path}.",
            &state.config.address()
        ),
    )
        .into_response()
}
