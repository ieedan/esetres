use std::{collections::HashMap, path::Path, sync::Arc};

use axum::{
    body::Body,
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use esetres::{config, mime};

struct AppState {
    config: config::Object,
    mime_types: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get()?;

    let mime_types = mime::get(&config).await?;

    // If not exists create
    if let Err(_) = std::fs::read_dir(&config.root_directory) {
        tokio::fs::create_dir(&config.root_directory).await?;
        println!("Created directory {}", &config.root_directory);
    }

    let address = format!("{}:{}", &config.host_ip, &config.port);

    let state = Arc::new(AppState { config, mime_types });

    let app = axum::Router::new()
        .route("/health", get(health))
        .fallback(router)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Listening at {address}...");

    axum::serve(listener, app).await?;

    Ok(())
}

/// # Health Check Route
/// Returns basic alive status
async fn health() -> String {
    "I'm Alive".to_string()
}

/// Routes the request to the correct file
async fn router(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<Body>,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    let start_extension = path.rfind(".");

    let file_extension = if let Some(start) = start_extension {
        let (_, ext) = path.split_at(start + 1);
        Some(ext)
    } else {
        None
    };

    let resource_path = Path::new(&state.config.root_directory).join(&path[1..]);

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
