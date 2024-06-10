use std::{collections::HashMap, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    Extension,
};
use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::db::{self, schema::Token};

pub async fn invalidate(
    Extension(token_cache): Extension<Arc<Mutex<HashMap<String, Token>>>>,
) -> Response {
    let mut cache = token_cache.lock().await;

    cache.clear();

    if let Ok(tokens) = db::schema::Token::get_all().await {
        for token in tokens {
            cache.insert(token.name.clone(), token);
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "There was an error getting the tokens").into_response();
    }

    (StatusCode::OK).into_response()
}
