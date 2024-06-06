use crate::config;
use axum::{Extension, Router};
use reqwest::Method;
use std::{collections::HashMap, net::Ipv4Addr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

pub mod buckets;

pub mod cache;

pub mod files;

pub struct AppState {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub config: config::Object,
    pub mime_types: HashMap<String, String>,
}

pub struct Token {
    pub name: String,
    pub token: String,
}

pub fn get(app_state: AppState) -> Router {
    let state = Arc::new(app_state);

    let tokens: HashMap<String, Token> = HashMap::new();

    let token_cache = Arc::new(Mutex::new(tokens));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let private = CorsLayer::new()
        .allow_private_network(true)
        .allow_methods([Method::POST])
        .allow_headers(Any);

    axum::Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/buckets", axum::routing::post(buckets::create))
        .route(
            "/cache/invalidate",
            axum::routing::post(cache::invalidate).layer(private),
        )
        .route(
            "/buckets/:bucketId/blob/:resourcePath",
            axum::routing::get(files::get).put(files::upload),
        )
        .with_state(state)
        .layer(Extension(token_cache))
        .layer(cors)
}

async fn health_check() -> String {
    "I'm Alive".to_string()
}
