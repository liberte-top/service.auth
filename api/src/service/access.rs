use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use super::config::ConfigService;

const DEMO_SUBJECT: &str = "demo-user";
const DEMO_AUTH_TYPE: &str = "session";
const DEMO_EMAIL: &str = "demo@example.com";
const DEMO_SCOPES_HEADER: &str = "notes:read profile:read";

pub trait AccessService: Send + Sync {
    fn check(&self, headers: &HeaderMap) -> Response;
}

pub struct AccessServiceImpl {
    config: Arc<dyn ConfigService>,
}

impl AccessServiceImpl {
    pub fn new(config: Arc<dyn ConfigService>) -> Self {
        Self { config }
    }

    fn is_authenticated(&self, headers: &HeaderMap) -> bool {
        let cookie_name = self.config.forwardauth_session_cookie_name();
        let cookie_value = self.config.forwardauth_session_cookie_value();
        headers
            .get(header::COOKIE)
            .and_then(|raw| raw.to_str().ok())
            .map(|cookie_header| {
                cookie_header
                    .split(';')
                    .any(|part| part.trim() == format!("{cookie_name}={cookie_value}"))
            })
            .unwrap_or(false)
    }

    fn unauthorized_response(&self, headers: &HeaderMap) -> Response {
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
        let login_url = format!(
            "{}?return_to={}",
            self.config.forwardauth_login_url(),
            redirect_to
        );

        let mut response = StatusCode::FOUND.into_response();
        response.headers_mut().insert(
            header::LOCATION,
            HeaderValue::from_str(&login_url)
                .unwrap_or_else(|_| HeaderValue::from_static("https://auth.liberte.top/")),
        );
        response
    }
}

impl AccessService for AccessServiceImpl {
    fn check(&self, headers: &HeaderMap) -> Response {
        if !self.is_authenticated(headers) {
            return self.unauthorized_response(headers);
        }

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
        response_headers.insert(
            "x-auth-scopes",
            HeaderValue::from_static(DEMO_SCOPES_HEADER),
        );
        response_headers.insert("x-auth-email", HeaderValue::from_static(DEMO_EMAIL));
        response
    }
}

fn authorize_request(method: &str, path: &str) -> Result<(), StatusCode> {
    match (method, path) {
        ("GET", p) if p.starts_with("/api/v1/viewer") => Ok(()),
        ("GET", p) if p.starts_with("/api/v1/notes") => Ok(()),
        ("POST", p) if p.starts_with("/api/v1/notes") => Err(StatusCode::FORBIDDEN),
        _ => Ok(()),
    }
}
