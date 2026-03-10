use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use async_trait::async_trait;
use std::sync::Arc;

use crate::repo::{
    api_keys::ApiKeysRepo,
    account_scopes::AccountScopesRepo,
    accounts::AccountsRepo,
    route_policies::{RoutePoliciesRepo, RoutePolicyRecord},
    sessions::SessionsRepo,
};

use super::config::ConfigService;

const DEMO_SUBJECT: &str = "demo-user";
const DEMO_AUTH_TYPE: &str = "session";
const DEMO_EMAIL: &str = "demo@example.com";
const DEMO_API_KEY_PREFIX: &str = "demo";

#[async_trait]
pub trait AccessService: Send + Sync {
    async fn check(&self, headers: &HeaderMap) -> Response;
}

pub struct AccessServiceImpl {
    config: Arc<dyn ConfigService>,
    api_keys_repo: Arc<dyn ApiKeysRepo>,
    accounts_repo: Arc<dyn AccountsRepo>,
    account_scopes_repo: Arc<dyn AccountScopesRepo>,
    route_policies_repo: Arc<dyn RoutePoliciesRepo>,
    sessions_repo: Arc<dyn SessionsRepo>,
}

impl AccessServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        api_keys_repo: Arc<dyn ApiKeysRepo>,
        accounts_repo: Arc<dyn AccountsRepo>,
        account_scopes_repo: Arc<dyn AccountScopesRepo>,
        route_policies_repo: Arc<dyn RoutePoliciesRepo>,
        sessions_repo: Arc<dyn SessionsRepo>,
    ) -> Self {
        Self {
            config,
            api_keys_repo,
            accounts_repo,
            account_scopes_repo,
            route_policies_repo,
            sessions_repo,
        }
    }

    async fn session_subject(&self, headers: &HeaderMap) -> Option<String> {
        let cookie_name = self.config.forwardauth_session_cookie_name();
        let token = headers
            .get(header::COOKIE)
            .and_then(|raw| raw.to_str().ok())
            .and_then(|cookie_header| {
                cookie_header
                    .split(';')
                    .find_map(|part| {
                        let trimmed = part.trim();
                        let prefix = format!("{cookie_name}=");
                        trimmed
                            .strip_prefix(prefix.as_str())
                            .map(ToOwned::to_owned)
                    })
            })
            .filter(|value| !value.is_empty())?;

        let session = self
            .sessions_repo
            .find_active_by_token_hash(&token)
            .await
            .ok()
            .flatten()?;

        let account = self
            .accounts_repo
            .find_by_username(DEMO_SUBJECT)
            .await
            .ok()
            .flatten()?;

        if account.id == session.account_id {
            Some(DEMO_SUBJECT.to_owned())
        } else {
            None
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
}

#[async_trait]
impl AccessService for AccessServiceImpl {
    async fn check(&self, headers: &HeaderMap) -> Response {
        let Some(identity) = self.identity(headers).await else {
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
            .authorize_request(&identity.subject, host_from_headers(headers), method, path)
            .await
        {
            return status.into_response();
        }

        let mut response = StatusCode::OK.into_response();
        let response_headers = response.headers_mut();
        response_headers.insert(
            "x-auth-subject",
            HeaderValue::from_str(&identity.subject)
                .unwrap_or_else(|_| HeaderValue::from_static(DEMO_SUBJECT)),
        );
        response_headers.insert(
            "x-auth-type",
            HeaderValue::from_str(&identity.auth_type)
                .unwrap_or_else(|_| HeaderValue::from_static(DEMO_AUTH_TYPE)),
        );
        response_headers.insert(
            "x-auth-scopes",
            HeaderValue::from_str(&identity.scopes.join(" "))
                .unwrap_or_else(|_| HeaderValue::from_static("notes:read profile:read")),
        );
        response_headers.insert("x-auth-email", HeaderValue::from_static(DEMO_EMAIL));
        response
    }
}

struct ResolvedIdentity {
    subject: String,
    auth_type: String,
    scopes: Vec<String>,
}

impl AccessServiceImpl {
    async fn identity(&self, headers: &HeaderMap) -> Option<ResolvedIdentity> {
        if let Some(subject) = self.session_subject(headers).await {
            let scopes = self.resolve_scopes(&subject).await;
            return Some(ResolvedIdentity {
                subject,
                auth_type: DEMO_AUTH_TYPE.to_owned(),
                scopes,
            });
        }

        if let Some(subject) = self.api_key_subject(headers).await {
            let scopes = self.resolve_scopes(&subject).await;
            return Some(ResolvedIdentity {
                subject,
                auth_type: "api_key".to_owned(),
                scopes,
            });
        }

        None
    }

    async fn authorize_request(
        &self,
        subject: &str,
        host: &str,
        method: &str,
        path: &str,
    ) -> Result<(), StatusCode> {
        let policies = self.route_policies_repo.list_enabled().await.unwrap_or_default();

        let Some(policy) = match_policy(&policies, host, method, path) else {
            return Ok(());
        };

        let granted = self.resolve_scopes(subject).await;
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

    async fn resolve_scopes(&self, subject: &str) -> Vec<String> {
        let Some(account) = self
            .accounts_repo
            .find_by_username(subject)
            .await
            .ok()
            .flatten()
        else {
            return vec!["notes:read".to_owned(), "profile:read".to_owned()];
        };

        self.account_scopes_repo
            .list_by_account_id(account.id)
            .await
            .map(|items| items.into_iter().map(|item| item.scope_name).collect())
            .unwrap_or_else(|_| vec!["notes:read".to_owned(), "profile:read".to_owned()])
    }

    async fn api_key_subject(&self, headers: &HeaderMap) -> Option<String> {
        let raw = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .or_else(|| headers.get("x-api-key").and_then(|value| value.to_str().ok()))?
            .trim();

        let key = self
            .api_keys_repo
            .find_active_by_key_hash(raw)
            .await
            .ok()
            .flatten()?;

        if key.key_prefix == DEMO_API_KEY_PREFIX {
            Some(DEMO_SUBJECT.to_owned())
        } else {
            None
        }
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
