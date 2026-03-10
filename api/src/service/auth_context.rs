use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::repo::{account_scopes::AccountScopesRepo, accounts::AccountsRepo};
use crate::repo::sessions::SessionsRepo;

use super::config::ConfigService;

const DEMO_SESSION_ETAG: &str = "W/\"demo-smoke-session-v1\"";

#[derive(Serialize, ToSchema)]
pub struct AuthContextResponse {
    pub subject: Option<String>,
    pub auth_type: Option<String>,
    pub scopes: Vec<String>,
}

#[async_trait]
pub trait AuthContextService: Send + Sync {
    async fn context(&self, headers: &HeaderMap) -> Response;
}

pub struct AuthContextServiceImpl {
    config: Arc<dyn ConfigService>,
    accounts_repo: Arc<dyn AccountsRepo>,
    account_scopes_repo: Arc<dyn AccountScopesRepo>,
    sessions_repo: Arc<dyn SessionsRepo>,
}

impl AuthContextServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        accounts_repo: Arc<dyn AccountsRepo>,
        account_scopes_repo: Arc<dyn AccountScopesRepo>,
        sessions_repo: Arc<dyn SessionsRepo>,
    ) -> Self {
        Self {
            config,
            accounts_repo,
            account_scopes_repo,
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
            .find_by_username("demo-user")
            .await
            .ok()
            .flatten()?;

        if account.id == session.account_id {
            Some("demo-user".to_owned())
        } else {
            None
        }
    }
}

#[async_trait]
impl AuthContextService for AuthContextServiceImpl {
    async fn context(&self, headers: &HeaderMap) -> Response {
        let subject = self.session_subject(headers).await;
        let authenticated = subject.is_some();

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
            let subject = subject.unwrap_or_else(|| "demo-user".to_owned());
            let scopes = self.resolve_scopes(&subject).await;
            Json(AuthContextResponse {
                subject: Some(subject),
                auth_type: Some("session".to_owned()),
                scopes,
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

impl AuthContextServiceImpl {
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
}
