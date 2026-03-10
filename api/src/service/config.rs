use std::{env, sync::Arc};

use crate::config::Config;

pub trait ConfigService: Send + Sync {
    fn port(&self) -> u16;
    fn forwardauth_session_cookie_name(&self) -> &str;
    fn forwardauth_session_cookie_value(&self) -> &str;
    fn forwardauth_login_url(&self) -> &str;
}

pub struct ConfigServiceImpl {
    config: Arc<Config>,
}

impl ConfigServiceImpl {
    pub fn new() -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|value| value.trim().parse::<u16>().ok())
            .unwrap_or(3333);
        let forwardauth_session_cookie_name = env::var("FORWARDAUTH_SESSION_COOKIE_NAME")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "liberte_session".to_owned());
        let forwardauth_session_cookie_value = env::var("FORWARDAUTH_SESSION_COOKIE_VALUE")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "demo-smoke-session".to_owned());
        let forwardauth_login_url = env::var("FORWARDAUTH_LOGIN_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "https://auth.liberte.top/".to_owned());
        Self {
            config: Arc::new(Config {
                port,
                forwardauth_session_cookie_name,
                forwardauth_session_cookie_value,
                forwardauth_login_url,
            }),
        }
    }
}

impl ConfigService for ConfigServiceImpl {
    fn port(&self) -> u16 {
        self.config.port
    }

    fn forwardauth_session_cookie_name(&self) -> &str {
        &self.config.forwardauth_session_cookie_name
    }

    fn forwardauth_session_cookie_value(&self) -> &str {
        &self.config.forwardauth_session_cookie_value
    }

    fn forwardauth_login_url(&self) -> &str {
        &self.config.forwardauth_login_url
    }
}
