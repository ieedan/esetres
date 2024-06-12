use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use crate::{config, jwt::validate};
use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
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

    next.run(request).await
}

pub async fn from_host(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let config = config::get();

    let valid_ip = IpAddr::from_str(&config.ip).unwrap();

    if addr.ip() != valid_ip {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    next.run(request).await
}
