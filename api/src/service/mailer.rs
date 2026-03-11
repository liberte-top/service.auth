use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

use super::config::ConfigService;

#[async_trait]
pub trait MailerService: Send + Sync {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        text: &str,
    ) -> Result<(), reqwest::Error>;
}

pub struct ResendMailerService {
    config: Arc<dyn ConfigService>,
    client: reqwest::Client,
}

impl ResendMailerService {
    pub fn new(config: Arc<dyn ConfigService>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl MailerService for ResendMailerService {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        text: &str,
    ) -> Result<(), reqwest::Error> {
        let Some(api_key) = self.config.resend_api_key() else {
            return Ok(());
        };

        self.client
            .post("https://api.resend.com/emails")
            .bearer_auth(api_key)
            .json(&json!({
                "from": self.config.email_from(),
                "to": [to],
                "subject": subject,
                "text": text,
            }))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
