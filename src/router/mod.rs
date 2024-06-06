use crate::config;
use axum::Router;
use std::{collections::HashMap, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

pub mod buckets;
pub mod files;

pub struct AppState {
    pub config: config::Object,
    pub mime_types: HashMap<String, String>,
}

pub fn get(app_state: AppState) -> Router {
    let state = Arc::new(app_state);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    axum::Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/buckets", axum::routing::post(buckets::create))
        .route(
            "/buckets/:bucketId/blob/:resourcePath",
            axum::routing::get(files::get).put(files::upload),
        )
        .with_state(state)
        .layer(cors)
}

async fn health_check() -> String {
    "I'm Alive".to_string()
}
