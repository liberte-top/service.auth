use axum::{extract::State, http::HeaderMap, routing::get, Router};
use std::sync::Arc;

use crate::state::AppState;

pub async fn access_check(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> axum::response::Response {
    state.access().check(&headers).await
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/internal/auth/v1/access/check", get(access_check))
        .route("/internal/auth/session/check", get(access_check))
        .with_state(state)
}
