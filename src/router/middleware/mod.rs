use crate::jwt::validate;
use axum::{extract::Request, middleware::Next, response::{IntoResponse, Response}};
use reqwest::StatusCode;

/// Checks for token presence and validates it
pub async fn authorization(request: Request, next: Next) -> Response {
    let token = if let Some(header) = request.headers().get("Authorization") {
        // remove `Bearer ` prefix to get token
        let header = header.to_str().unwrap();
        &header["Bearer ".len()..]
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    if let Err(_) = validate(token.to_string()) {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let response = next.run(request).await;

    response
}
