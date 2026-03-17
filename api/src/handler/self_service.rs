use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{service::profile::SelfProfileResponse, state::AppState};

#[derive(Deserialize, ToSchema)]
pub struct UpdateSelfProfileRequest {
    pub display_name: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateApiTokenRequest {
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
}

async fn require_actor(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<crate::service::auth_actor::ResolvedAuthActor, StatusCode> {
    state
        .auth_actor()
        .resolve(headers)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)
}

#[utoipa::path(
    get,
    path = "/api/v1/self/profile",
    responses(
        (status = 200, description = "Current self profile", body = SelfProfileResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn get_self_profile(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<SelfProfileResponse>, StatusCode> {
    let actor = require_actor(&state, &headers).await?;
    let profile = state
        .profile()
        .get_self_profile(actor.account_id, actor.subject, actor.principal_type, actor.scopes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(profile))
}

#[utoipa::path(
    patch,
    path = "/api/v1/self/profile",
    request_body = UpdateSelfProfileRequest,
    responses(
        (status = 200, description = "Updated self profile", body = SelfProfileResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn update_self_profile(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSelfProfileRequest>,
) -> Result<Json<SelfProfileResponse>, StatusCode> {
    let actor = require_actor(&state, &headers).await?;
    let profile = state
        .profile()
        .update_self_profile(crate::service::profile::UpdateSelfProfileInput {
            account_id: actor.account_id,
            subject: actor.subject,
            principal_type: actor.principal_type,
            scopes: actor.scopes,
            display_name: payload.display_name,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(profile))
}

#[utoipa::path(
    get,
    path = "/api/v1/self/tokens",
    responses(
        (status = 200, description = "Current personal API tokens", body = [crate::service::api_tokens::ApiTokenSummary]),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn list_self_tokens(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<crate::service::api_tokens::ApiTokenSummary>>, StatusCode> {
    let actor = require_actor(&state, &headers).await?;
    let tokens = state
        .api_tokens()
        .list_by_account_id(actor.account_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tokens))
}

#[utoipa::path(
    post,
    path = "/api/v1/self/tokens",
    request_body = CreateApiTokenRequest,
    responses(
        (status = 201, description = "Created personal API token", body = crate::service::api_tokens::ApiTokenSecret),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn create_self_token(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateApiTokenRequest>,
) -> Result<(StatusCode, Json<crate::service::api_tokens::ApiTokenSecret>), StatusCode> {
    let actor = require_actor(&state, &headers).await?;
    let token = state
        .api_tokens()
        .create(crate::service::api_tokens::CreateApiTokenInput {
            account_id: actor.account_id,
            name: payload.name,
            expires_at: payload.expires_at,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(token)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/self/tokens/{id}",
    params(("id" = i64, Path, description = "API token id")),
    responses(
        (status = 200, description = "Revoked personal API token", body = crate::service::api_tokens::ApiTokenSummary),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found")
    ),
    tag = "auth"
)]
pub async fn revoke_self_token(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<crate::service::api_tokens::ApiTokenSummary>, StatusCode> {
    let actor = require_actor(&state, &headers).await?;
    let Some(token) = state
        .api_tokens()
        .revoke(actor.account_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(token))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/self/profile", get(get_self_profile).patch(update_self_profile))
        .route("/api/v1/self/tokens", get(list_self_tokens).post(create_self_token))
        .route("/api/v1/self/tokens/:id", delete(revoke_self_token))
        .with_state(state)
}
