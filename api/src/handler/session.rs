use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::state::AppState;

pub async fn forwardauth_session_check(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let config = state.config();
    let cookie_name = config.forwardauth_session_cookie_name();
    let cookie_value = config.forwardauth_session_cookie_value();

    let authenticated = headers
        .get(header::COOKIE)
        .and_then(|raw| raw.to_str().ok())
        .map(|cookie_header| cookie_header.split(';').any(|part| {
            let trimmed = part.trim();
            trimmed == format!("{cookie_name}={cookie_value}")
        }))
        .unwrap_or(false);

    if authenticated {
        let mut response = StatusCode::OK.into_response();
        let response_headers = response.headers_mut();
        response_headers.insert("x-auth-subject", HeaderValue::from_static("demo-user"));
        response_headers.insert("x-auth-type", HeaderValue::from_static("session"));
        response_headers.insert(
            "x-auth-scopes",
            HeaderValue::from_static("notes:read profile:read"),
        );
        response_headers.insert("x-auth-email", HeaderValue::from_static("demo@example.com"));
        return response;
    }

    let path = headers
        .get("x-forwarded-uri")
        .and_then(|raw| raw.to_str().ok())
        .unwrap_or("/");

    if path.starts_with("/api/") {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let host = headers
        .get("x-forwarded-host")
        .and_then(|raw| raw.to_str().ok())
        .unwrap_or("smoke.liberte.top");
    let proto = headers
        .get("x-forwarded-proto")
        .and_then(|raw| raw.to_str().ok())
        .unwrap_or("https");

    let redirect_to = format!("{proto}://{host}{path}");
    let login_url = format!("{}?return_to={}", config.forwardauth_login_url(), redirect_to);

    let mut response = StatusCode::FOUND.into_response();
    response.headers_mut().insert(
        header::LOCATION,
        HeaderValue::from_str(&login_url).unwrap_or_else(|_| HeaderValue::from_static("https://auth.liberte.top/")),
    );
    response
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/internal/auth/session/check", get(forwardauth_session_check))
        .with_state(state)
}
