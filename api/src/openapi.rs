use utoipa::OpenApi;

use crate::{
    handler,
    handler::{
        accounts::{AccountResponse, CreateAccount, UpdateAccount},
        health::Health,
        public_auth::{CompleteLoginRequest, EmailOnlyRequest, RegisterEmailRequest, VerifyQuery},
    },
    service::{
        auth_context::AuthContextResponse,
        email_auth::{EmailActionAccepted, EmailLoginResult, EmailVerifyResult},
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health::health,
        handler::public_auth::context,
        handler::public_auth::register_email,
        handler::public_auth::resend_verify_email,
        handler::public_auth::verify_email,
        handler::public_auth::request_email_login,
        handler::public_auth::complete_email_login,
        handler::accounts::create_account,
        handler::accounts::get_account,
        handler::accounts::update_account,
        handler::accounts::delete_account
    ),
    components(schemas(
        Health,
        AuthContextResponse,
        RegisterEmailRequest,
        EmailOnlyRequest,
        VerifyQuery,
        CompleteLoginRequest,
        EmailActionAccepted,
        EmailVerifyResult,
        EmailLoginResult,
        CreateAccount,
        UpdateAccount,
        AccountResponse
    )),
    tags(
        (name = "health", description = "Health check"),
        (name = "auth", description = "Auth context"),
        (name = "accounts", description = "Accounts")
    )
)]
pub struct ApiDoc;
