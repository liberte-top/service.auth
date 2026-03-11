use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    service::{
        auth_context::AuthContextResponse,
        email_auth::{EmailActionAccepted, EmailLoginResult, EmailVerifyResult},
    },
    state::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct RegisterEmailRequest {
    pub email: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct EmailOnlyRequest {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyQuery {
    pub token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CompleteLoginRequest {
    pub token: String,
}

fn session_cookie_value(state: &AppState, session_token: &str) -> String {
    format!(
        "{}={}; Path=/; HttpOnly; Secure; SameSite=Lax",
        state.config().forwardauth_session_cookie_name(),
        session_token
    )
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
    post,
    path = "/api/v1/auth/register/email",
    request_body = RegisterEmailRequest,
    responses((status = 202, description = "Verification mail queued", body = EmailActionAccepted)),
    tag = "auth"
)]
pub async fn register_email(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterEmailRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .register_email(crate::service::email_auth::RegisterEmailInput {
            email: payload.email,
            display_name: payload.display_name,
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
    Json(payload): Json<EmailOnlyRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .resend_verify(crate::service::email_auth::ResendVerifyInput { email: payload.email })
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
    Query(query): Query<VerifyQuery>,
) -> Result<Json<EmailVerifyResult>, StatusCode> {
    state
        .email_auth()
        .verify_email(&query.token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
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
    Json(payload): Json<EmailOnlyRequest>,
) -> Result<(StatusCode, Json<EmailActionAccepted>), StatusCode> {
    let result = state
        .email_auth()
        .request_login(crate::service::email_auth::LoginEmailInput { email: payload.email })
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
        .complete_login(crate::service::email_auth::CompleteEmailLoginInput { token: payload.token })
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
    Query(query): Query<VerifyQuery>,
) -> Result<Response, StatusCode> {
    let Some(result) = state
        .email_auth()
        .complete_login(crate::service::email_auth::CompleteEmailLoginInput { token: query.token })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut response = Redirect::to(state.config().forwardauth_login_url()).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&session_cookie_value(&state, &result.session_token))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/auth/context", get(context))
        .route("/api/v1/context", get(context))
        .route("/api/v1/auth/register/email", post(register_email))
        .route("/api/v1/auth/verify/email/resend", post(resend_verify_email))
        .route("/api/v1/auth/verify/email", get(verify_email))
        .route("/api/v1/auth/login/email", post(request_email_login))
        .route(
            "/api/v1/auth/login/email/complete",
            get(complete_email_login_link).post(complete_email_login),
        )
        .with_state(state)
}
