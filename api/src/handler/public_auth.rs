use axum::{extract::State, http::HeaderMap, routing::get, Router};
use std::sync::Arc;

use crate::{service::auth_context::AuthContextResponse, state::AppState};

#[utoipa::path(
    get,
    path = "/api/v1/auth/context",
    responses(
        (status = 200, description = "Current auth context", body = AuthContextResponse),
        (status = 304, description = "Auth context not modified"),
        (status = 401, description = "Unauthenticated")
    ),
    tag = "auth"
)]
pub async fn context(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> axum::response::Response {
    state.auth_context().context(&headers)
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/auth/context", get(context))
        .route("/api/v1/context", get(context))
        .with_state(state)
}
