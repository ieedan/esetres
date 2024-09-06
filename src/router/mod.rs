use crate::{
    config,
    db::schema::{Access, Token},
    jwt::validate,
};
use axum::{
    http::HeaderMap,
    routing::{get, post, put},
    Extension, Router,
};
use bcrypt::verify;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub mod buckets;

pub mod cache;

pub mod files;

pub mod health;

pub mod middleware;

pub struct AppState {
    pub config: config::Object
}

pub fn create(tokens: HashMap<String, Token>, app_state: AppState) -> Router {
    /* State initialization */

    let state = Arc::new(app_state);

    let token_cache = Arc::new(Mutex::new(tokens));

    /* CORS Config */

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    axum::Router::new()
        .route("/health", axum::routing::get(health::health_check))
        .route(
            "/buckets",
            post(buckets::create).layer(axum::middleware::from_fn(middleware::authorization)),
        )
        .route(
            "/cache/invalidate",
            post(cache::invalidate).layer(axum::middleware::from_fn(middleware::from_host)),
        )
        .route(
            "/buckets/:bucketId/public/:resourcePath",
            get(files::public_get),
        )
        .route(
            "/buckets/:bucketId/public/:resourcePath",
            put(files::public_upload)
                .delete(files::public_delete)
                .layer(axum::middleware::from_fn(middleware::authorization)),
        )
        .route(
            "/buckets/:bucketId/private/:resourcePath",
            get(files::private_get)
                .put(files::private_upload)
                .delete(files::private_delete)
                .layer(axum::middleware::from_fn(middleware::authorization)),
        )
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(Extension(token_cache)),
        )
}

pub async fn is_authed(
    headers: HeaderMap,
    bucket_id: Option<&str>,
    required_access: Access,
    token_cache: Arc<Mutex<HashMap<String, Token>>>,
) -> bool {
    let token = if let Some(header) = headers.get("Authorization") {
        // remove `Bearer ` prefix to get token
        let header = header.to_str().unwrap();
        
        let t = header.trim_start_matches("Bearer ");
        if t == "" {
            return false;
        }

        t
    } else {
        return false;
    };

    let token_data = if let Ok(data) = validate(token.to_string()) {
        data
    } else {
        return false;
    };

    let cache = token_cache.lock().await;

    let cached_token = if let Some(t) = cache.get(&token_data.claims.nm) {
        t
    } else {
        return false;
    };

    // controls access to specific buckets
    if let Some(identifier) = bucket_id {
        if cached_token.bucket_scope != "*" && cached_token.bucket_scope != identifier {
            return false;
        }
    }

    if cached_token.access != Access::FULL && required_access != cached_token.access {
        return false;
    }

    if let Ok(valid) = verify(&token, &cached_token.token) {
        if !valid {
            return false;
        }
    } else {
        return false;
    }

    true
}
