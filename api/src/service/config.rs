use std::{env, sync::Arc};
use url::Url;

use crate::config::Config;

pub trait ConfigService: Send + Sync {
    fn port(&self) -> u16;
    fn forwardauth_session_cookie_name(&self) -> &str;
    fn forwardauth_session_cookie_value(&self) -> &str;
    fn forwardauth_session_cookie_domain(&self) -> Option<&str>;
    fn forwardauth_login_url(&self) -> &str;
    fn resend_api_key(&self) -> Option<&str>;
    fn email_from(&self) -> &str;
    fn email_verify_base_url(&self) -> &str;
    fn email_login_base_url(&self) -> &str;
    fn email_token_ttl_secs(&self) -> i64;
    fn mail_grpc_addr(&self) -> Option<&str>;
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
        let forwardauth_session_cookie_domain = env::var("FORWARDAUTH_SESSION_COOKIE_DOMAIN")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| {
                Url::parse(&forwardauth_login_url)
                    .ok()
                    .and_then(|url| url.host_str().map(ToOwned::to_owned))
                    .and_then(|host| {
                        host.strip_suffix(".liberte.top")
                            .map(|_| ".liberte.top".to_owned())
                    })
            });
        let resend_api_key = env::var("RESEND_API_KEY")
            .ok()
            .filter(|value| !value.trim().is_empty());
        let email_from = env::var("EMAIL_FROM")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Auth <auth@mail.liberte.top>".to_owned());
        let email_verify_base_url = env::var("EMAIL_VERIFY_BASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "https://auth.liberte.top/api/v1/auth/verify/email".to_owned());
        let email_login_base_url = env::var("EMAIL_LOGIN_BASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| {
                "https://auth.liberte.top/api/v1/auth/login/email/complete".to_owned()
            });
        let email_token_ttl_secs = env::var("EMAIL_TOKEN_TTL_SECS")
            .ok()
            .and_then(|value| value.trim().parse::<i64>().ok())
            .unwrap_or(900);
        let mail_grpc_addr = env::var("MAIL_GRPC_ADDR")
            .ok()
            .filter(|value| !value.trim().is_empty());
        Self {
            config: Arc::new(Config {
                port,
                forwardauth_session_cookie_name,
                forwardauth_session_cookie_value,
                forwardauth_session_cookie_domain,
                forwardauth_login_url,
                resend_api_key,
                email_from,
                email_verify_base_url,
                email_login_base_url,
                email_token_ttl_secs,
                mail_grpc_addr,
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

    fn forwardauth_session_cookie_domain(&self) -> Option<&str> {
        self.config.forwardauth_session_cookie_domain.as_deref()
    }

    fn forwardauth_login_url(&self) -> &str {
        &self.config.forwardauth_login_url
    }

    fn resend_api_key(&self) -> Option<&str> {
        self.config.resend_api_key.as_deref()
    }

    fn email_from(&self) -> &str {
        &self.config.email_from
    }

    fn email_verify_base_url(&self) -> &str {
        &self.config.email_verify_base_url
    }

    fn email_login_base_url(&self) -> &str {
        &self.config.email_login_base_url
    }

    fn email_token_ttl_secs(&self) -> i64 {
        self.config.email_token_ttl_secs
    }

    fn mail_grpc_addr(&self) -> Option<&str> {
        self.config.mail_grpc_addr.as_deref()
    }
}
