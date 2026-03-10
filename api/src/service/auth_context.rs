use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use super::config::ConfigService;

const DEMO_SESSION_ETAG: &str = "W/\"demo-smoke-session-v1\"";

#[derive(Serialize, ToSchema)]
pub struct AuthContextResponse {
    pub subject: Option<String>,
    pub auth_type: Option<String>,
    pub scopes: Vec<String>,
}

pub trait AuthContextService: Send + Sync {
    fn context(&self, headers: &HeaderMap) -> Response;
}

pub struct AuthContextServiceImpl {
    config: Arc<dyn ConfigService>,
}

impl AuthContextServiceImpl {
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
}

impl AuthContextService for AuthContextServiceImpl {
    fn context(&self, headers: &HeaderMap) -> Response {
        let authenticated = self.is_authenticated(headers);

        if authenticated
            && headers
                .get(header::IF_NONE_MATCH)
                .and_then(|raw| raw.to_str().ok())
                .is_some_and(|value| value == DEMO_SESSION_ETAG)
        {
            let mut response = StatusCode::NOT_MODIFIED.into_response();
            response
                .headers_mut()
                .insert(header::ETAG, HeaderValue::from_static(DEMO_SESSION_ETAG));
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

        response
            .headers_mut()
            .insert(header::ETAG, HeaderValue::from_static(DEMO_SESSION_ETAG));
        response
    }
}
