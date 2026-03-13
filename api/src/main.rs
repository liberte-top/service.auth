use axum::middleware;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod db;
mod entities;
mod handler;
mod mail_proto;
mod openapi;
mod repo;
mod schema;
mod service;
mod state;
mod telemetry;

use openapi::ApiDoc;
use state::AppState;

#[tokio::main]
async fn main() {
    telemetry::init_tracing("service-auth-api");
    let state = AppState::new().await;
    let _ = state.db().conn();
    let _ = state.accounts_repo();
    let _ = state.config().port();

    let app = Router::new()
        .merge(handler::health::routes())
        .merge(handler::public_auth::routes(state.clone()))
        .merge(handler::internal_auth::routes(state.clone()))
        .merge(handler::admin_accounts::routes(state.clone()))
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn(telemetry::trace_http_request))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config().port()));

    eprintln!("starting server on {}", addr);
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("bind failed: {}", err);
            std::process::exit(1);
        }
    };
    eprintln!("bound on {}", addr);

    axum::serve(listener, app).await.expect("serve error");
}
