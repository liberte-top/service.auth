use utoipa::OpenApi;

use crate::{
    handler,
    handler::{
        accounts::{AccountResponse, CreateAccount, UpdateAccount},
        health::Health,
        preferences::{
            PreferenceOption, PreferenceOptionsResponse, PreferencesResponse,
            UpdatePreferencesRequest,
        },
        public_auth::{CompleteLoginRequest, EmailOnlyRequest, RegisterEmailRequest, VerifyQuery},
        self_service::{CreateApiTokenRequest, UpdateSelfProfileRequest},
    },
    service::{
        api_tokens::{ApiTokenSecret, ApiTokenSummary},
        auth_actor::AuthScopeDefinition,
        auth_context::AuthContextResponse,
        email_auth::{EmailActionAccepted, EmailLoginResult, EmailVerifyResult},
        profile::SelfProfileResponse,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health::health,
        handler::public_auth::context,
        handler::public_auth::scope_catalog,
        handler::public_auth::register_email,
        handler::public_auth::resend_verify_email,
        handler::public_auth::verify_email,
        handler::public_auth::request_email_login,
        handler::public_auth::complete_email_login,
        handler::self_service::get_self_profile,
        handler::self_service::update_self_profile,
        handler::self_service::list_self_tokens,
        handler::self_service::create_self_token,
        handler::self_service::revoke_self_token,
        handler::preferences::get_preference_options,
        handler::preferences::get_preferences,
        handler::preferences::update_preferences,
        handler::accounts::create_account,
        handler::accounts::get_account,
        handler::accounts::update_account,
        handler::accounts::delete_account
    ),
    components(schemas(
        Health,
        AuthScopeDefinition,
        AuthContextResponse,
        RegisterEmailRequest,
        EmailOnlyRequest,
        VerifyQuery,
        CompleteLoginRequest,
        UpdateSelfProfileRequest,
        CreateApiTokenRequest,
        PreferenceOption,
        PreferenceOptionsResponse,
        PreferencesResponse,
        UpdatePreferencesRequest,
        SelfProfileResponse,
        EmailActionAccepted,
        EmailVerifyResult,
        EmailLoginResult,
        ApiTokenSummary,
        ApiTokenSecret,
        CreateAccount,
        UpdateAccount,
        AccountResponse
    )),
    tags(
        (name = "health", description = "Health check"),
        (name = "auth", description = "Auth context"),
        (name = "preferences", description = "Cross-app user preferences"),
        (name = "accounts", description = "Accounts")
    )
)]
pub struct ApiDoc;
