#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub forwardauth_session_cookie_name: String,
    pub forwardauth_session_cookie_value: String,
    pub forwardauth_login_url: String,
}
