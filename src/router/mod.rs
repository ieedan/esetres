use crate::{config, jwt::validate};
use axum::{
    extract::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Extension, Router,
};
use reqwest::{Method, StatusCode};
use std::{collections::HashMap, net::Ipv4Addr, sync::Arc};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
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
        .route(
            "/buckets",
            axum::routing::post(buckets::create).layer(middleware::from_fn(authorization)),
        )
        .route(
            "/cache/invalidate",
            axum::routing::post(cache::invalidate).layer(private),
        )
        .route(
            "/buckets/:bucketId/blob/:resourcePath",
            axum::routing::get(files::get),
        )
        .route(
            "/buckets/:bucketId/blob/:resourcePath",
            axum::routing::put(files::upload).layer(middleware::from_fn(authorization)),
        )
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(token_cache))
                .layer(cors),
        )
}

async fn health_check() -> String {
    "I'm Alive".to_string()
}

async fn authorization(
    Extension(token_cache): Extension<HashMap<String, Token>>,
    request: Request,
    next: Next,
) -> Response {
    let token = if let Some(header) = request.headers().get("Authorization") {
        // remove `Bearer ` prefix to get token
        let header = header.to_str().unwrap();
        &header["Bearer ".len()..]
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let token_data = if let Ok(validated) = validate(token.to_string()) {
        validated
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let response = next.run(request).await;

    response
}
