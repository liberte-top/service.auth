use axum::{
    extract::{Extension, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;

use crate::{
    service::{
        auth_actor::AuthScopeDefinition,
        auth_context::AuthContextResponse,
        email_auth::{EmailActionAccepted, EmailLoginResult, EmailVerifyResult},
    },
    state::AppState,
    telemetry::TraceContext,
};

fn normalize_language(value: Option<&str>) -> &'static str {
    match value.unwrap_or("en").trim().to_ascii_lowercase().as_str() {
        "zh" | "zh-cn" => "zh-CN",
        _ => "en",
    }
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| {
            raw.split(';').find_map(|item| {
                let mut parts = item.trim().splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) if key.trim() == name => Some(value.trim().to_owned()),
                    _ => None,
                }
            })
        })
}

fn request_language(headers: &HeaderMap) -> &'static str {
    normalize_language(cookie_value(headers, "liberte_language").as_deref())
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterEmailRequest {
    pub email: String,
    pub display_name: Option<String>,
    pub rewrite: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct EmailOnlyRequest {
    pub email: String,
    pub rewrite: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyQuery {
    pub token: String,
    pub rewrite: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CompleteLoginRequest {
    pub token: String,
}

fn session_cookie_value(state: &AppState, session_token: &str) -> String {
    let domain = state
        .config()
        .forwardauth_session_cookie_domain()
        .map(|value| format!("; Domain={value}"))
        .unwrap_or_default();
    format!(
        "{}={}; Path=/; HttpOnly; Secure; SameSite=Lax{}",
        state.config().forwardauth_session_cookie_name(),
        session_token,
        domain
    )
}

fn expired_session_cookie_value(state: &AppState) -> String {
    let domain = state
        .config()
        .forwardauth_session_cookie_domain()
        .map(|value| format!("; Domain={value}"))
        .unwrap_or_default();
    format!(
        "{}=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT{}",
        state.config().forwardauth_session_cookie_name(),
        domain
    )
}

fn profile_url(state: &AppState) -> String {
    Url::parse(state.config().forwardauth_login_url())
        .ok()
        .and_then(|base| base.join("profile").ok())
        .map(Into::into)
        .unwrap_or_else(|| "https://auth.liberte.top/profile".to_owned())
}

fn sanitized_rewrite(rewrite: Option<&str>) -> Option<String> {
    let value = rewrite?.trim();
    if value.is_empty() {
        return None;
    }
    if value.starts_with('/') {
        return Some(value.to_owned());
    }
    let url = Url::parse(value).ok()?;
    match url.scheme() {
        "http" | "https" => Some(url.into()),
        _ => None,
    }
}

fn resolve_post_auth_redirect(state: &AppState, rewrite: Option<&str>) -> String {
    sanitized_rewrite(rewrite).unwrap_or_else(|| profile_url(state))
}

fn login_page_url(
    state: &AppState,
    mode: &str,
    email: Option<&str>,
    verified: bool,
    rewrite: Option<&str>,
) -> String {
    let path = if mode == "register" {
        "register"
    } else {
        "login"
    };
    let mut url = Url::parse(state.config().forwardauth_login_url())
        .ok()
        .and_then(|base| base.join(path).ok())
        .unwrap_or_else(|| Url::parse(&format!("https://auth.liberte.top/{path}")).unwrap());
    {
        let mut query = url.query_pairs_mut();
        if verified {
            query.append_pair("verified", "1");
        }
        if let Some(email) = email.filter(|value| !value.is_empty()) {
            query.append_pair("email", email);
        }
        if let Some(rewrite) = sanitized_rewrite(rewrite) {
            query.append_pair("rewrite", &rewrite);
        }
    }
    url.into()
}

fn flow_page_url(
    state: &AppState,
    step: &str,
    email: Option<&str>,
    rewrite: Option<&str>,
    next: Option<&str>,
    trace_id: Option<&str>,
) -> String {
    let mut url = Url::parse(state.config().forwardauth_login_url())
        .ok()
        .and_then(|base| base.join("flow").ok())
        .unwrap_or_else(|| Url::parse("https://auth.liberte.top/flow").unwrap());
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("step", step);
        if let Some(email) = email.filter(|value| !value.is_empty()) {
            query.append_pair("email", email);
        }
        if let Some(rewrite) = sanitized_rewrite(rewrite) {
            query.append_pair("rewrite", &rewrite);
        }
        if let Some(next) = sanitized_rewrite(next) {
            query.append_pair("next", &next);
        }
        if let Some(trace_id) = trace_id.filter(|value| !value.is_empty()) {
            query.append_pair("trace_id", trace_id);
        }
    }
    url.into()
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/context",
    responses(
        (status = 200, description = "Current auth context", body = AuthContextResponse),
        (status = 304, description = "Auth context not modified")
    ),
    tag = "auth"
)]
pub async fn context(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> axum::response::Response {
    state.auth_context().context(&headers).await
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/scopes",
    responses((status = 200, description = "Canonical auth scope catalog", body = [AuthScopeDefinition])),
    tag = "auth"
)]
pub async fn scope_catalog(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.auth_actor().scope_catalog())
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register/email",
    request_body = RegisterEmailRequest,
    responses((status = 202, description = "Verification mail queued", body = EmailActionAccepted)),
    tag = "auth"
)]
pub async fn register_email(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<RegisterEmailRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .register_email(crate::service::email_auth::RegisterEmailInput {
            email: payload.email,
            display_name: payload.display_name,
            rewrite: payload.rewrite,
            language: request_language(&headers).to_owned(),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(result)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify/email/resend",
    request_body = EmailOnlyRequest,
    responses((status = 202, description = "Verification mail queued", body = EmailActionAccepted)),
    tag = "auth"
)]
pub async fn resend_verify_email(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<EmailOnlyRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .resend_verify(crate::service::email_auth::ResendVerifyInput {
            email: payload.email,
            rewrite: payload.rewrite,
            language: request_language(&headers).to_owned(),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(result)))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/verify/email",
    params(("token" = String, Query, description = "Verification token")),
    responses(
        (status = 200, description = "Email verified", body = EmailVerifyResult),
        (status = 404, description = "Token not found or expired")
    ),
    tag = "auth"
)]
pub async fn verify_email(
    State(state): State<Arc<AppState>>,
    Extension(trace): Extension<TraceContext>,
    Query(query): Query<VerifyQuery>,
) -> Result<Response, StatusCode> {
    let Some(result) = state
        .email_auth()
        .verify_email(&query.token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Ok(Redirect::to(&flow_page_url(
            &state,
            "verify-invalid",
            None,
            query.rewrite.as_deref(),
            Some(&login_page_url(
                &state,
                "register",
                None,
                false,
                query.rewrite.as_deref(),
            )),
            Some(&trace.trace_id),
        ))
        .into_response());
    };

    Ok(Redirect::to(&flow_page_url(
        &state,
        "verify-success",
        Some(&result.email),
        query.rewrite.as_deref(),
        Some(&login_page_url(
            &state,
            "login",
            Some(&result.email),
            true,
            query.rewrite.as_deref(),
        )),
        Some(&trace.trace_id),
    ))
    .into_response())
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login/email",
    request_body = EmailOnlyRequest,
    responses((status = 202, description = "Login mail queued", body = EmailActionAccepted)),
    tag = "auth"
)]
pub async fn request_email_login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<EmailOnlyRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .request_login(crate::service::email_auth::LoginEmailInput {
            email: payload.email,
            rewrite: payload.rewrite,
            language: request_language(&headers).to_owned(),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(result)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login/email/complete",
    request_body = CompleteLoginRequest,
    responses(
        (status = 200, description = "Login completed", body = EmailLoginResult),
        (status = 404, description = "Token not found or expired")
    ),
    tag = "auth"
)]
pub async fn complete_email_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CompleteLoginRequest>,
) -> Result<(HeaderMap, Json<EmailLoginResult>), StatusCode> {
    let Some(result) = state
        .email_auth()
        .complete_login(crate::service::email_auth::CompleteEmailLoginInput {
            token: payload.token,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&session_cookie_value(&state, &result.session_token))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((headers, Json(result)))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/login/email/complete",
    params(("token" = String, Query, description = "Login token")),
    responses(
        (status = 303, description = "Login completed and redirected"),
        (status = 404, description = "Token not found or expired")
    ),
    tag = "auth"
)]
pub async fn complete_email_login_link(
    State(state): State<Arc<AppState>>,
    Extension(trace): Extension<TraceContext>,
    Query(query): Query<VerifyQuery>,
) -> Result<Response, StatusCode> {
    let Some(result) = state
        .email_auth()
        .complete_login(crate::service::email_auth::CompleteEmailLoginInput { token: query.token })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Ok(Redirect::to(&flow_page_url(
            &state,
            "login-invalid",
            None,
            query.rewrite.as_deref(),
            Some(&login_page_url(
                &state,
                "login",
                None,
                false,
                query.rewrite.as_deref(),
            )),
            Some(&trace.trace_id),
        ))
        .into_response());
    };

    let next = resolve_post_auth_redirect(&state, query.rewrite.as_deref());
    let mut response = Redirect::to(&flow_page_url(
        &state,
        "login-success",
        None,
        query.rewrite.as_deref(),
        Some(&next),
        Some(&trace.trace_id),
    ))
    .into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&session_cookie_value(&state, &result.session_token))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses((status = 204, description = "Session cookie cleared")),
    tag = "auth"
)]
pub async fn logout(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&expired_session_cookie_value(&state))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((StatusCode::NO_CONTENT, headers))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/auth/context", get(context))
        .route("/api/v1/context", get(context))
        .route("/api/v1/auth/scopes", get(scope_catalog))
        .route("/api/v1/auth/register/email", post(register_email))
        .route(
            "/api/v1/auth/verify/email/resend",
            post(resend_verify_email),
        )
        .route("/api/v1/auth/verify/email", get(verify_email))
        .route("/api/v1/auth/login/email", post(request_email_login))
        .route("/api/v1/auth/logout", post(logout))
        .route(
            "/api/v1/auth/login/email/complete",
            get(complete_email_login_link).post(complete_email_login),
        )
        .with_state(state)
}
