use crate::config;
use axum::Router;
use std::{collections::HashMap, sync::Arc};

pub mod buckets;
pub mod files;

pub struct AppState {
    pub config: config::Object,
    pub mime_types: HashMap<String, String>,
}

pub fn get(app_state: AppState) -> Router {
    let state = Arc::new(app_state);

    axum::Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/buckets", axum::routing::post(buckets::create))
        .route(
            "/buckets/:bucketId/blob/:resourcePath",
            axum::routing::get(files::get).put(files::upload),
        )
        .with_state(state)
}

async fn health_check() -> String {
    "I'm Alive".to_string()
}
