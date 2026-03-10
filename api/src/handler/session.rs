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

const DEMO_SESSION_ETAG: &str = "W/\"demo-smoke-session-v1\"";
const DEMO_SUBJECT: &str = "demo-user";
const DEMO_AUTH_TYPE: &str = "session";
const DEMO_EMAIL: &str = "demo@example.com";
const DEMO_SCOPES_HEADER: &str = "notes:read profile:read";

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
        let method = headers
            .get("x-forwarded-method")
            .and_then(|raw| raw.to_str().ok())
            .unwrap_or("GET");
        let path = headers
            .get("x-forwarded-uri")
            .and_then(|raw| raw.to_str().ok())
            .unwrap_or("/");

        if let Err(status) = authorize_request(method, path) {
            return status.into_response();
        }

        let mut response = StatusCode::OK.into_response();
        let response_headers = response.headers_mut();
        response_headers.insert("x-auth-subject", HeaderValue::from_static(DEMO_SUBJECT));
        response_headers.insert("x-auth-type", HeaderValue::from_static(DEMO_AUTH_TYPE));
        response_headers.insert("x-auth-scopes", HeaderValue::from_static(DEMO_SCOPES_HEADER));
        response_headers.insert("x-auth-email", HeaderValue::from_static(DEMO_EMAIL));
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
            .is_some_and(|value| value == DEMO_SESSION_ETAG)
    {
        let mut response = StatusCode::NOT_MODIFIED.into_response();
        response.headers_mut().insert(
            header::ETAG,
            HeaderValue::from_static(DEMO_SESSION_ETAG),
        );
        return response;
    }

    let mut response = if authenticated {
        Json(AuthContextResponse {
            subject: Some(DEMO_SUBJECT.to_owned()),
            auth_type: Some(DEMO_AUTH_TYPE.to_owned()),
            scopes: vec!["notes:read".to_owned(), "profile:read".to_owned()],
        })
        .into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    };

    response.headers_mut().insert(
        header::ETAG,
        HeaderValue::from_static(DEMO_SESSION_ETAG),
    );
    response
}

fn authorize_request(method: &str, path: &str) -> Result<(), StatusCode> {
    match (method, path) {
        ("GET", p) if p.starts_with("/api/v1/viewer") => Ok(()),
        ("GET", p) if p.starts_with("/api/v1/notes") => Ok(()),
        ("POST", p) if p.starts_with("/api/v1/notes") => Err(StatusCode::FORBIDDEN),
        _ => Ok(()),
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/internal/auth/session/check", get(forwardauth_session_check))
        .route("/api/v1/context", get(auth_context))
        .with_state(state)
}
