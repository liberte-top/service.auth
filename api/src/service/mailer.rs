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
        html: Option<&str>,
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
        html: Option<&str>,
    ) -> Result<(), reqwest::Error> {
        let Some(api_key) = self.config.resend_api_key() else {
            return Ok(());
        };

        let mut payload = json!({
            "from": self.config.email_from(),
            "to": [to],
            "subject": subject,
            "text": text,
        });

        if let Some(html) = html {
            payload["html"] = json!(html);
        }

        self.client
            .post("https://api.resend.com/emails")
            .bearer_auth(api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
