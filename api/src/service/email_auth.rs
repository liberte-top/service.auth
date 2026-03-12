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

struct EmailTemplate {
    text: String,
    html: String,
}

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

    fn expiry_label(ttl_secs: i64) -> String {
        if ttl_secs % 3600 == 0 {
            let hours = ttl_secs / 3600;
            if hours == 1 {
                "1 hour".to_owned()
            } else {
                format!("{hours} hours")
            }
        } else if ttl_secs % 60 == 0 {
            let minutes = ttl_secs / 60;
            if minutes == 1 {
                "1 minute".to_owned()
            } else {
                format!("{minutes} minutes")
            }
        } else if ttl_secs == 1 {
            "1 second".to_owned()
        } else {
            format!("{ttl_secs} seconds")
        }
    }

    fn escape_html(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    fn build_email_template(
        purpose: &str,
        recipient_email: &str,
        action_link: &str,
        raw_token: &str,
        rewrite: Option<&str>,
        ttl_secs: i64,
    ) -> EmailTemplate {
        let (badge, heading, intro, action_label) = if purpose == VERIFY_PURPOSE {
            (
                "Verify your email",
                "Confirm your email address",
                "Finish creating your liberte.top account with one secure click.",
                "Verify email",
            )
        } else {
            (
                "Sign in",
                "Complete your sign-in",
                "Use the secure link below to continue into liberte.top without entering a password.",
                "Sign in now",
            )
        };

        let destination = rewrite
            .filter(|value| !value.is_empty())
            .map(|value| format!("After you continue, we will send you back to {value}."))
            .unwrap_or_else(|| {
                "If no destination was provided, we will send you to your auth profile page."
                    .to_owned()
            });
        let expires_in = Self::expiry_label(ttl_secs);
        let escaped_heading = Self::escape_html(heading);
        let escaped_intro = Self::escape_html(intro);
        let escaped_email = Self::escape_html(recipient_email);
        let escaped_destination = Self::escape_html(&destination);
        let escaped_link = Self::escape_html(action_link);
        let escaped_token = Self::escape_html(raw_token);
        let escaped_badge = Self::escape_html(badge);
        let escaped_action = Self::escape_html(action_label);
        let escaped_expiry = Self::escape_html(&expires_in);

        let text = format!(
            "{heading}\n\n{intro}\n\nContinue here: {action_link}\n\n{destination}\n\nThis link expires in {expires_in}.\n\nIf the button does not open in your mail client, you can copy this raw token instead:\n{raw_token}\n\nThis email was sent to {recipient_email}. If you did not request it, you can safely ignore this message.",
        );

        let html = format!(
            concat!(
                "<!doctype html><html><body style=\"margin:0;padding:0;background:#edf4f0;font-family:IBM Plex Sans,Segoe UI,sans-serif;color:#132027;\">",
                "<div style=\"padding:32px 16px;\">",
                "<div style=\"max-width:640px;margin:0 auto;background:#ffffff;border:1px solid #d4e4dd;border-radius:28px;overflow:hidden;box-shadow:0 18px 50px rgba(19,32,39,0.08);\">",
                "<div style=\"padding:40px 40px 24px;background:linear-gradient(135deg,#f7fbf8 0%,#edf6f2 55%,#f7f3eb 100%);border-bottom:1px solid #dce9e2;\">",
                "<div style=\"display:inline-block;padding:8px 12px;border-radius:999px;background:#dceee5;color:#0f5e46;font-size:12px;font-weight:700;letter-spacing:0.08em;text-transform:uppercase;\">{escaped_badge}</div>",
                "<h1 style=\"margin:18px 0 12px;font-size:34px;line-height:1.05;letter-spacing:-0.04em;color:#10231f;\">{escaped_heading}</h1>",
                "<p style=\"margin:0;font-size:16px;line-height:1.7;color:#567068;\">{escaped_intro}</p>",
                "</div>",
                "<div style=\"padding:32px 40px 40px;\">",
                "<div style=\"margin-bottom:24px;padding:18px 20px;border-radius:20px;background:#f5f9f7;border:1px solid #dbe7e1;\">",
                "<p style=\"margin:0 0 6px;font-size:12px;font-weight:700;letter-spacing:0.08em;text-transform:uppercase;color:#1d805f;\">Requested for</p>",
                "<p style=\"margin:0;font-size:16px;color:#132027;\">{escaped_email}</p>",
                "</div>",
                "<p style=\"margin:0 0 20px;font-size:15px;line-height:1.7;color:#567068;\">{escaped_destination}</p>",
                "<div style=\"margin:28px 0 24px;\"><a href=\"{escaped_link}\" style=\"display:inline-block;padding:14px 24px;border-radius:14px;background:linear-gradient(180deg,#1d805f,#0f5e46);color:#ffffff;text-decoration:none;font-weight:700;\">{escaped_action}</a></div>",
                "<p style=\"margin:0 0 14px;font-size:14px;line-height:1.7;color:#567068;\">This secure link expires in <strong style=\"color:#10231f;\">{escaped_expiry}</strong>.</p>",
                "<div style=\"padding:16px 18px;border-radius:18px;background:#fbfcfb;border:1px solid #e3ece7;\">",
                "<p style=\"margin:0 0 8px;font-size:12px;font-weight:700;letter-spacing:0.08em;text-transform:uppercase;color:#1d805f;\">Manual token</p>",
                "<p style=\"margin:0;font-family:IBM Plex Mono,ui-monospace,SFMono-Regular,monospace;font-size:13px;line-height:1.7;color:#29423b;word-break:break-all;\">{escaped_token}</p>",
                "</div>",
                "<p style=\"margin:24px 0 0;font-size:13px;line-height:1.7;color:#73857e;\">If you did not request this email, you can safely ignore it.</p>",
                "</div></div></div></body></html>"
            ),
            escaped_badge = escaped_badge,
            escaped_heading = escaped_heading,
            escaped_intro = escaped_intro,
            escaped_email = escaped_email,
            escaped_destination = escaped_destination,
            escaped_link = escaped_link,
            escaped_action = escaped_action,
            escaped_expiry = escaped_expiry,
            escaped_token = escaped_token,
        );

        EmailTemplate { text, html }
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
        let template = Self::build_email_template(
            purpose,
            &email.email_normalized,
            &action_link,
            &raw_token,
            rewrite,
            self.config.email_token_ttl_secs(),
        );
        let _ = self
            .mailer
            .send_email(
                &email.email_normalized,
                subject,
                &template.text,
                Some(&template.html),
            )
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
