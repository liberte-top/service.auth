use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct AuthContextResponse {
    pub subject: Option<String>,
    pub auth_type: Option<String>,
    pub scopes: Vec<String>,
}

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

#[utoipa::path(
    get,
    path = "/api/v1/context",
    responses(
        (status = 200, description = "Current auth context", body = AuthContextResponse),
        (status = 304, description = "Auth context not modified"),
        (status = 401, description = "Unauthenticated")
    )
)]

pub async fn auth_context(
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

    if authenticated
        && headers
            .get(header::IF_NONE_MATCH)
            .and_then(|raw| raw.to_str().ok())
            .is_some_and(|value| value == "W/\"demo-smoke-session-v1\"")
    {
        let mut response = StatusCode::NOT_MODIFIED.into_response();
        response.headers_mut().insert(
            header::ETAG,
            HeaderValue::from_static("W/\"demo-smoke-session-v1\""),
        );
        return response;
    }

    let mut response = if authenticated {
        Json(AuthContextResponse {
            subject: Some("demo-user".to_owned()),
            auth_type: Some("session".to_owned()),
            scopes: vec!["notes:read".to_owned(), "profile:read".to_owned()],
        })
        .into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    };

    response.headers_mut().insert(
        header::ETAG,
        HeaderValue::from_static("W/\"demo-smoke-session-v1\""),
    );
    response
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/internal/auth/session/check", get(forwardauth_session_check))
        .route("/api/v1/context", get(auth_context))
        .with_state(state)
}
