use std::env;

pub struct Config {
    pub port: u16,
    pub resend_api_key: Option<String>,
    pub email_from: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(50051),
            resend_api_key: env::var("RESEND_API_KEY")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            email_from: env::var("EMAIL_FROM")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "Auth <auth@mail.liberte.top>".to_owned()),
        }
    }
}
