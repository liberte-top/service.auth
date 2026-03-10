use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/admin/accounts",
            post(crate::handler::accounts::create_account),
        )
        .route(
            "/api/v1/admin/accounts/:uid",
            get(crate::handler::accounts::get_account),
        )
        .route(
            "/api/v1/admin/accounts/:uid",
            patch(crate::handler::accounts::update_account),
        )
        .route(
            "/api/v1/admin/accounts/:uid",
            delete(crate::handler::accounts::delete_account),
        )
        .with_state(state)
}
