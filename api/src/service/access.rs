use async_trait::async_trait;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::repo::route_policies::{RoutePoliciesRepo, RoutePolicyRecord};

use super::{auth_actor::AuthActorService, config::ConfigService};

#[async_trait]
pub trait AccessService: Send + Sync {
    async fn check(&self, headers: &HeaderMap) -> Response;
}

pub struct AccessServiceImpl {
    config: Arc<dyn ConfigService>,
    auth_actor: Arc<dyn AuthActorService>,
    route_policies_repo: Arc<dyn RoutePoliciesRepo>,
}

impl AccessServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        auth_actor: Arc<dyn AuthActorService>,
        route_policies_repo: Arc<dyn RoutePoliciesRepo>,
    ) -> Self {
        Self {
            config,
            auth_actor,
            route_policies_repo,
        }
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

    async fn authorize_request(
        &self,
        scopes: &[String],
        host: &str,
        method: &str,
        path: &str,
    ) -> Result<(), StatusCode> {
        let policies = self.route_policies_repo.list_enabled().await.unwrap_or_default();

        let Some(policy) = match_policy(&policies, host, method, path) else {
            return Ok(());
        };

        if policy
            .required_scopes
            .iter()
            .all(|scope| scopes.iter().any(|item| item == scope))
        {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

#[async_trait]
impl AccessService for AccessServiceImpl {
    async fn check(&self, headers: &HeaderMap) -> Response {
        let Some(actor) = self.auth_actor.resolve(headers).await else {
            return self.unauthorized_response(headers);
        };

        let method = headers
            .get("x-forwarded-method")
            .and_then(|raw| raw.to_str().ok())
            .unwrap_or("GET");
        let path = headers
            .get("x-forwarded-uri")
            .and_then(|raw| raw.to_str().ok())
            .unwrap_or("/");

        if let Err(status) = self
            .authorize_request(&actor.scopes, host_from_headers(headers), method, path)
            .await
        {
            return status.into_response();
        }

        let mut response = StatusCode::OK.into_response();
        let response_headers = response.headers_mut();
        response_headers.insert(
            "x-auth-subject",
            HeaderValue::from_str(&actor.subject).unwrap_or_else(|_| {
                HeaderValue::from_static("00000000-0000-0000-0000-000000000000")
            }),
        );
        response_headers.insert(
            "x-auth-type",
            HeaderValue::from_str(&actor.auth_type)
                .unwrap_or_else(|_| HeaderValue::from_static("session")),
        );
        response_headers.insert(
            "x-auth-scopes",
            HeaderValue::from_str(&actor.scopes.join(" "))
                .unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        response_headers.insert(
            "x-auth-principal-type",
            HeaderValue::from_str(&actor.principal_type)
                .unwrap_or_else(|_| HeaderValue::from_static("user")),
        );
        if let Some(email) = actor.email {
            if let Ok(value) = HeaderValue::from_str(&email) {
                response_headers.insert("x-auth-email", value);
            }
        }
        response
    }
}

fn host_from_headers(headers: &HeaderMap) -> &str {
    headers
        .get("x-forwarded-host")
        .and_then(|raw| raw.to_str().ok())
        .unwrap_or("smoke.liberte.top")
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
