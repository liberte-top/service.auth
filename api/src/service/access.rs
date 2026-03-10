use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use async_trait::async_trait;
use std::sync::Arc;

use crate::repo::route_policies::{RoutePoliciesRepo, RoutePolicyRecord};

use super::config::ConfigService;

const DEMO_SUBJECT: &str = "demo-user";
const DEMO_AUTH_TYPE: &str = "session";
const DEMO_EMAIL: &str = "demo@example.com";
const DEMO_SCOPES_HEADER: &str = "notes:read profile:read";

#[async_trait]
pub trait AccessService: Send + Sync {
    async fn check(&self, headers: &HeaderMap) -> Response;
}

pub struct AccessServiceImpl {
    config: Arc<dyn ConfigService>,
    route_policies_repo: Arc<dyn RoutePoliciesRepo>,
}

impl AccessServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        route_policies_repo: Arc<dyn RoutePoliciesRepo>,
    ) -> Self {
        Self {
            config,
            route_policies_repo,
        }
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

#[async_trait]
impl AccessService for AccessServiceImpl {
    async fn check(&self, headers: &HeaderMap) -> Response {
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

        if let Err(status) = self
            .authorize_request(host_from_headers(headers), method, path)
            .await
        {
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

impl AccessServiceImpl {
    async fn authorize_request(
        &self,
        host: &str,
        method: &str,
        path: &str,
    ) -> Result<(), StatusCode> {
        let policies = self.route_policies_repo.list_enabled().await.unwrap_or_default();

        let Some(policy) = match_policy(&policies, host, method, path) else {
            return Ok(());
        };

        let granted = demo_scopes();
        if policy
            .required_scopes
            .iter()
            .all(|scope| granted.iter().any(|item| item == scope))
        {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

fn host_from_headers(headers: &HeaderMap) -> &str {
    headers
        .get("x-forwarded-host")
        .and_then(|raw| raw.to_str().ok())
        .unwrap_or("smoke.liberte.top")
}

fn demo_scopes() -> Vec<&'static str> {
    vec!["notes:read", "profile:read"]
}

fn match_policy<'a>(
    policies: &'a [RoutePolicyRecord],
    host: &str,
    method: &str,
    path: &str,
) -> Option<&'a RoutePolicyRecord> {
    policies.iter().find(|policy| {
        policy.host_pattern == host
            && policy.method.eq_ignore_ascii_case(method)
            && path.starts_with(policy.path_pattern.as_str())
    })
}
