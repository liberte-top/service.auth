#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub forwardauth_session_cookie_name: String,
    pub forwardauth_session_cookie_value: String,
    pub forwardauth_session_cookie_domain: Option<String>,
    pub forwardauth_login_url: String,
    pub resend_api_key: Option<String>,
    pub email_from: String,
    pub email_verify_base_url: String,
    pub email_login_base_url: String,
    pub email_token_ttl_secs: i64,
}
