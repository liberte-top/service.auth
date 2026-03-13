use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::Config;

pub struct ProviderMessage {
    pub to_email: String,
    pub to_name: Option<String>,
    pub subject: String,
    pub text_body: String,
    pub html_body: String,
}

pub struct ProviderResult {
    pub message_id: String,
    pub provider: &'static str,
}

#[derive(Clone)]
pub struct ResendProvider {
    config: Arc<Config>,
    client: Client,
}

#[derive(Deserialize)]
struct ResendResponse {
    id: Option<String>,
}

impl ResendProvider {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub async fn send(&self, message: ProviderMessage) -> Result<ProviderResult, String> {
        let Some(api_key) = self.config.resend_api_key.as_deref() else {
            return Ok(ProviderResult {
                message_id: format!("dry-run-{}", Uuid::new_v4()),
                provider: "dry-run",
            });
        };

        let to = match message.to_name {
            Some(display_name) if !display_name.trim().is_empty() => {
                format!("{display_name} <{}>", message.to_email)
            }
            _ => message.to_email,
        };

        let response = self
            .client
            .post("https://api.resend.com/emails")
            .bearer_auth(api_key)
            .json(&json!({
                "from": self.config.email_from,
                "to": [to],
                "subject": message.subject,
                "text": message.text_body,
                "html": message.html_body,
            }))
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?;

        let payload = response
            .json::<ResendResponse>()
            .await
            .map_err(|error| error.to_string())?;

        Ok(ProviderResult {
            message_id: payload.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            provider: "resend",
        })
    }
}
