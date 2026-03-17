use async_trait::async_trait;
use axum::http::{header, HeaderMap};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::repo::account_emails::AccountEmailsRepo;
use crate::repo::api_key_scopes::ApiKeyScopesRepo;
use crate::repo::api_keys::ApiKeysRepo;
use crate::repo::sessions::SessionsRepo;
use crate::repo::{account_scopes::AccountScopesRepo, accounts::AccountsRepo};

use super::config::ConfigService;

#[derive(Clone, Serialize, ToSchema)]
pub struct AuthScopeDefinition {
    pub name: String,
    pub label: String,
    pub description: String,
    pub granted_by_default: bool,
}

#[derive(Clone)]
pub struct ResolvedAuthActor {
    pub account_id: i64,
    pub subject: String,
    pub principal_type: String,
    pub email: Option<String>,
    pub auth_type: String,
    pub scopes: Vec<String>,
}

#[async_trait]
pub trait AuthActorService: Send + Sync {
    async fn resolve(&self, headers: &HeaderMap) -> Option<ResolvedAuthActor>;
    fn scope_catalog(&self) -> Vec<AuthScopeDefinition>;
}

pub struct AuthActorServiceImpl {
    config: Arc<dyn ConfigService>,
    api_keys_repo: Arc<dyn ApiKeysRepo>,
    api_key_scopes_repo: Arc<dyn ApiKeyScopesRepo>,
    accounts_repo: Arc<dyn AccountsRepo>,
    account_emails_repo: Arc<dyn AccountEmailsRepo>,
    account_scopes_repo: Arc<dyn AccountScopesRepo>,
    sessions_repo: Arc<dyn SessionsRepo>,
}

impl AuthActorServiceImpl {
    pub fn new(
        config: Arc<dyn ConfigService>,
        api_keys_repo: Arc<dyn ApiKeysRepo>,
        api_key_scopes_repo: Arc<dyn ApiKeyScopesRepo>,
        accounts_repo: Arc<dyn AccountsRepo>,
        account_emails_repo: Arc<dyn AccountEmailsRepo>,
        account_scopes_repo: Arc<dyn AccountScopesRepo>,
        sessions_repo: Arc<dyn SessionsRepo>,
    ) -> Self {
        Self {
            config,
            api_keys_repo,
            api_key_scopes_repo,
            accounts_repo,
            account_emails_repo,
            account_scopes_repo,
            sessions_repo,
        }
    }

    fn scope_catalog_static() -> &'static [(&'static str, &'static str, &'static str, bool)] {
        &[
            ("profile:read", "Profile read", "Read self profile and auth context identity data.", true),
            ("profile:write", "Profile write", "Update self profile fields owned by the auth service.", false),
            ("tokens:read", "Tokens read", "List and inspect personal API token summaries.", false),
            ("tokens:write", "Tokens write", "Create and revoke personal API tokens.", false),
            ("notes:read", "Notes read", "Read the protected notes demo flow.", true),
            ("notes:write", "Notes write", "Write through the protected notes demo flow.", false),
        ]
    }

    fn default_scopes() -> Vec<String> {
        Self::scope_catalog_static()
            .iter()
            .filter(|item| item.3)
            .map(|item| item.0.to_owned())
            .collect()
    }

    async fn resolve_scopes(&self, account_id: i64) -> Vec<String> {
        self.account_scopes_repo
            .list_by_account_id(account_id)
            .await
            .map(|items| {
                let scopes = items.into_iter().map(|item| item.scope_name).collect::<Vec<_>>();
                if scopes.is_empty() {
                    Self::default_scopes()
                } else {
                    scopes
                }
            })
            .unwrap_or_else(|_| Self::default_scopes())
    }

    async fn resolve_primary_email(&self, account_id: i64) -> Option<String> {
        self.account_emails_repo
            .find_primary_by_account_id(account_id)
            .await
            .ok()
            .flatten()
            .map(|item| item.email_normalized)
    }

    async fn resolve_session_actor(&self, headers: &HeaderMap) -> Option<ResolvedAuthActor> {
        let cookie_name = self.config.forwardauth_session_cookie_name();
        let token = headers
            .get(header::COOKIE)
            .and_then(|raw| raw.to_str().ok())
            .and_then(|cookie_header| {
                cookie_header.split(';').find_map(|part| {
                    let trimmed = part.trim();
                    let prefix = format!("{cookie_name}=");
                    trimmed.strip_prefix(prefix.as_str()).map(ToOwned::to_owned)
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
            .find_by_id(session.account_id)
            .await
            .ok()
            .flatten()?;

        Some(ResolvedAuthActor {
            account_id: account.id,
            subject: account.uid.to_string(),
            principal_type: account.account_type,
            email: self.resolve_primary_email(account.id).await,
            auth_type: "session".to_owned(),
            scopes: self.resolve_scopes(account.id).await,
        })
    }

    async fn resolve_api_key_actor(&self, headers: &HeaderMap) -> Option<ResolvedAuthActor> {
        let raw = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .or_else(|| {
                headers
                    .get("x-api-key")
                    .and_then(|value| value.to_str().ok())
            })?
            .trim();

        let key = self
            .api_keys_repo
            .find_active_by_key_hash(raw)
            .await
            .ok()
            .flatten()?;

        let account = self
            .accounts_repo
            .find_by_id(key.account_id)
            .await
            .ok()
            .flatten()?;

        let _ = self.api_keys_repo.touch_last_used(key.clone()).await;

        let token_scopes = self
            .api_key_scopes_repo
            .list_by_api_key_id(key.id)
            .await
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|item| item.scope_name)
            .collect::<Vec<_>>();
        let scopes = if token_scopes.is_empty() {
            self.resolve_scopes(account.id).await
        } else {
            token_scopes
        };

        Some(ResolvedAuthActor {
            account_id: account.id,
            subject: account.uid.to_string(),
            principal_type: account.account_type,
            email: self.resolve_primary_email(account.id).await,
            auth_type: "api_key".to_owned(),
            scopes,
        })
    }
}

#[async_trait]
impl AuthActorService for AuthActorServiceImpl {
    async fn resolve(&self, headers: &HeaderMap) -> Option<ResolvedAuthActor> {
        if let Some(actor) = self.resolve_session_actor(headers).await {
            return Some(actor);
        }

        self.resolve_api_key_actor(headers).await
    }

    fn scope_catalog(&self) -> Vec<AuthScopeDefinition> {
        Self::scope_catalog_static()
            .iter()
            .map(|(name, label, description, granted_by_default)| AuthScopeDefinition {
                name: (*name).to_owned(),
                label: (*label).to_owned(),
                description: (*description).to_owned(),
                granted_by_default: *granted_by_default,
            })
            .collect()
    }
}
