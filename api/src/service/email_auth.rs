use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::Serialize;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    entities::{account_emails, email_tokens, sessions},
    repo::{
        account_emails::AccountEmailsRepo, accounts::AccountsRepo, email_tokens::EmailTokensRepo,
        sessions::SessionsRepo,
    },
};

use super::{accounts::AccountsService, config::ConfigService, mailer::MailerService};

const VERIFY_PURPOSE: &str = "verify_email";
const LOGIN_PURPOSE: &str = "login_email";

pub struct RegisterEmailInput {
    pub email: String,
    pub display_name: Option<String>,
    pub rewrite: Option<String>,
}

pub struct ResendVerifyInput {
    pub email: String,
    pub rewrite: Option<String>,
}

pub struct LoginEmailInput {
    pub email: String,
    pub rewrite: Option<String>,
}

pub struct CompleteEmailLoginInput {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct EmailActionAccepted {
    pub accepted: bool,
}

#[derive(Serialize, ToSchema)]
pub struct EmailVerifyResult {
    pub verified: bool,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct EmailLoginResult {
    pub authenticated: bool,
    pub subject: String,
    pub auth_type: &'static str,
    pub session_token: String,
}

#[async_trait]
pub trait EmailAuthService: Send + Sync {
    async fn register_email(
        &self,
        input: RegisterEmailInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr>;
    async fn resend_verify(
        &self,
        input: ResendVerifyInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr>;
    async fn verify_email(&self, token: &str) -> Result<Option<EmailVerifyResult>, sea_orm::DbErr>;
    async fn request_login(
        &self,
        input: LoginEmailInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr>;
    async fn complete_login(
        &self,
        input: CompleteEmailLoginInput,
    ) -> Result<Option<EmailLoginResult>, sea_orm::DbErr>;
}

pub struct EmailAuthServiceImpl {
    accounts: Arc<dyn AccountsService>,
    accounts_repo: Arc<dyn AccountsRepo>,
    account_emails_repo: Arc<dyn AccountEmailsRepo>,
    email_tokens_repo: Arc<dyn EmailTokensRepo>,
    sessions_repo: Arc<dyn SessionsRepo>,
    mailer: Arc<dyn MailerService>,
    config: Arc<dyn ConfigService>,
}

impl EmailAuthServiceImpl {
    pub fn new(
        accounts: Arc<dyn AccountsService>,
        accounts_repo: Arc<dyn AccountsRepo>,
        account_emails_repo: Arc<dyn AccountEmailsRepo>,
        email_tokens_repo: Arc<dyn EmailTokensRepo>,
        sessions_repo: Arc<dyn SessionsRepo>,
        mailer: Arc<dyn MailerService>,
        config: Arc<dyn ConfigService>,
    ) -> Self {
        Self {
            accounts,
            accounts_repo,
            account_emails_repo,
            email_tokens_repo,
            sessions_repo,
            mailer,
            config,
        }
    }

    fn normalize_email(email: &str) -> String {
        email.trim().to_lowercase()
    }

    fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now() + Duration::seconds(self.config.email_token_ttl_secs())
    }

    fn build_action_link(base_url: &str, raw_token: &str, rewrite: Option<&str>) -> String {
        let mut url = Url::parse(base_url)
            .unwrap_or_else(|_| Url::parse("https://auth.liberte.top/").unwrap());
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("token", raw_token);
            if let Some(rewrite) = rewrite.filter(|value| !value.is_empty()) {
                query.append_pair("rewrite", rewrite);
            }
        }
        url.into()
    }

    async fn issue_token(
        &self,
        email: &account_emails::Model,
        purpose: &str,
        base_url: &str,
        subject: &str,
        rewrite: Option<&str>,
    ) -> Result<(), sea_orm::DbErr> {
        let raw_token = Uuid::new_v4().to_string();
        let token = email_tokens::ActiveModel {
            account_email_id: sea_orm::Set(email.id),
            purpose: sea_orm::Set(purpose.to_owned()),
            token_hash: sea_orm::Set(raw_token.clone()),
            expires_at: sea_orm::Set(self.expiry().into()),
            ..Default::default()
        };
        self.email_tokens_repo.insert(token).await?;

        let action_link = Self::build_action_link(base_url, &raw_token, rewrite);
        let body = format!("Use this link to continue: {action_link}\n\nRaw token: {raw_token}");
        let _ = self
            .mailer
            .send_email(&email.email_normalized, subject, &body)
            .await;

        Ok(())
    }
}

#[async_trait]
impl EmailAuthService for EmailAuthServiceImpl {
    async fn register_email(
        &self,
        input: RegisterEmailInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr> {
        let email = Self::normalize_email(&input.email);
        if let Some(existing) = self.account_emails_repo.find_by_email(&email).await? {
            if existing.verified_at.is_none() {
                self.issue_token(
                    &existing,
                    VERIFY_PURPOSE,
                    self.config.email_verify_base_url(),
                    "Verify your email",
                    input.rewrite.as_deref(),
                )
                .await?;
            }
            return Ok(EmailActionAccepted { accepted: true });
        }

        let account = self
            .accounts
            .create(super::accounts::CreateAccountInput {
                account_type: "user".to_owned(),
                username: input.display_name,
                email: Some(email.clone()),
                phone: None,
                created_by: None,
            })
            .await?;

        let account_email = self
            .account_emails_repo
            .insert(account_emails::ActiveModel {
                account_id: sea_orm::Set(account.id),
                email_normalized: sea_orm::Set(email),
                is_primary: sea_orm::Set(true),
                ..Default::default()
            })
            .await?;

        self.issue_token(
            &account_email,
            VERIFY_PURPOSE,
            self.config.email_verify_base_url(),
            "Verify your email",
            input.rewrite.as_deref(),
        )
        .await?;

        Ok(EmailActionAccepted { accepted: true })
    }

    async fn resend_verify(
        &self,
        input: ResendVerifyInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr> {
        let email = Self::normalize_email(&input.email);
        if let Some(existing) = self.account_emails_repo.find_by_email(&email).await? {
            if existing.verified_at.is_none() {
                self.issue_token(
                    &existing,
                    VERIFY_PURPOSE,
                    self.config.email_verify_base_url(),
                    "Verify your email",
                    input.rewrite.as_deref(),
                )
                .await?;
            }
        }
        Ok(EmailActionAccepted { accepted: true })
    }

    async fn verify_email(&self, token: &str) -> Result<Option<EmailVerifyResult>, sea_orm::DbErr> {
        let Some(record) = self
            .email_tokens_repo
            .find_active_by_token_hash(token, VERIFY_PURPOSE)
            .await?
        else {
            return Ok(None);
        };

        let Some(email) = self
            .account_emails_repo
            .find_by_id(record.account_email_id)
            .await?
        else {
            return Ok(None);
        };

        self.email_tokens_repo.mark_consumed(record).await?;
        let verified = self.account_emails_repo.mark_verified(email).await?;

        Ok(Some(EmailVerifyResult {
            verified: true,
            email: verified.email_normalized,
        }))
    }

    async fn request_login(
        &self,
        input: LoginEmailInput,
    ) -> Result<EmailActionAccepted, sea_orm::DbErr> {
        let email = Self::normalize_email(&input.email);
        if let Some(existing) = self.account_emails_repo.find_by_email(&email).await? {
            if existing.verified_at.is_some() {
                self.issue_token(
                    &existing,
                    LOGIN_PURPOSE,
                    self.config.email_login_base_url(),
                    "Complete your login",
                    input.rewrite.as_deref(),
                )
                .await?;
            }
        }
        Ok(EmailActionAccepted { accepted: true })
    }

    async fn complete_login(
        &self,
        input: CompleteEmailLoginInput,
    ) -> Result<Option<EmailLoginResult>, sea_orm::DbErr> {
        let Some(record) = self
            .email_tokens_repo
            .find_active_by_token_hash(&input.token, LOGIN_PURPOSE)
            .await?
        else {
            return Ok(None);
        };

        let Some(email) = self
            .account_emails_repo
            .find_by_id(record.account_email_id)
            .await?
        else {
            return Ok(None);
        };

        if email.verified_at.is_none() {
            return Ok(None);
        }

        self.email_tokens_repo.mark_consumed(record).await?;

        let raw_session_token = Uuid::new_v4().to_string();
        let session = sessions::ActiveModel {
            account_id: sea_orm::Set(email.account_id),
            token_hash: sea_orm::Set(raw_session_token.clone()),
            expires_at: sea_orm::Set((Utc::now() + Duration::days(30)).into()),
            ..Default::default()
        };
        self.sessions_repo.insert(session).await?;

        let subject = self
            .accounts_repo
            .find_by_id(email.account_id)
            .await?
            .map(|account| account.uid.to_string())
            .unwrap_or_else(|| email.account_id.to_string());

        Ok(Some(EmailLoginResult {
            authenticated: true,
            subject,
            auth_type: "session",
            session_token: raw_session_token,
        }))
    }
}
