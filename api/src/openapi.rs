use utoipa::OpenApi;

use crate::{
    handler,
    handler::{
        accounts::{AccountResponse, CreateAccount, UpdateAccount},
        health::Health,
    },
    service::auth_context::AuthContextResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health::health,
        handler::public_auth::context,
        handler::accounts::create_account,
        handler::accounts::get_account,
        handler::accounts::update_account,
        handler::accounts::delete_account
    ),
    components(schemas(
        Health,
        AuthContextResponse,
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
