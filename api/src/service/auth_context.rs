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
}

impl AuthContextServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        accounts_repo: Arc<dyn AccountsRepo>,
        account_scopes_repo: Arc<dyn AccountScopesRepo>,
    ) -> Self {
        Self {
            config,
            accounts_repo,
            account_scopes_repo,
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
}

#[async_trait]
impl AuthContextService for AuthContextServiceImpl {
    async fn context(&self, headers: &HeaderMap) -> Response {
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
            let scopes = self.resolve_scopes().await;
            Json(AuthContextResponse {
                subject: Some("demo-user".to_owned()),
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
    async fn resolve_scopes(&self) -> Vec<String> {
        let Some(account) = self
            .accounts_repo
            .find_by_username("demo-user")
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
