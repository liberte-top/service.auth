use async_trait::async_trait;
use axum::{
    http::{header, HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use super::auth_actor::AuthActorService;

#[derive(Serialize, ToSchema)]
pub struct AuthContextResponse {
    pub authenticated: bool,
    pub subject: Option<String>,
    pub principal_type: Option<String>,
    pub email: Option<String>,
    pub auth_type: Option<String>,
    pub scopes: Vec<String>,
}

#[async_trait]
pub trait AuthContextService: Send + Sync {
    async fn context(&self, headers: &HeaderMap) -> Response;
}

pub struct AuthContextServiceImpl {
    auth_actor: Arc<dyn AuthActorService>,
}

impl AuthContextServiceImpl {
    pub fn new(auth_actor: Arc<dyn AuthActorService>) -> Self {
        Self { auth_actor }
    }
}

#[async_trait]
impl AuthContextService for AuthContextServiceImpl {
    async fn context(&self, headers: &HeaderMap) -> Response {
        let actor = self.auth_actor.resolve(headers).await;

        let mut response = if let Some(actor) = actor {
            Json(AuthContextResponse {
                authenticated: true,
                subject: Some(actor.subject),
                principal_type: Some(actor.principal_type),
                email: actor.email,
                auth_type: Some(actor.auth_type),
                scopes: actor.scopes,
            })
            .into_response()
        } else {
            Json(AuthContextResponse {
                authenticated: false,
                subject: None,
                principal_type: None,
                email: None,
                auth_type: None,
                scopes: Vec::new(),
            })
            .into_response()
        };

        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("private, no-store, max-age=0"),
        );
        response
            .headers_mut()
            .insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
        response.headers_mut().insert(
            header::VARY,
            HeaderValue::from_static(
                "cookie, authorization, origin, access-control-request-method, access-control-request-headers",
            ),
        );
        response
    }
}
