use utoipa::OpenApi;

use crate::{
    handler,
    handler::{
        accounts::{AccountResponse, CreateAccount, UpdateAccount},
        health::Health,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health::health,
        handler::accounts::create_account,
        handler::accounts::get_account,
        handler::accounts::update_account,
        handler::accounts::delete_account
    ),
    components(schemas(
        Health,
        CreateAccount,
        UpdateAccount,
        AccountResponse
    )),
    tags(
        (name = "health", description = "Health check"),
        (name = "accounts", description = "Accounts")
    )
)]
pub struct ApiDoc;
