use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
    repo::account_emails::AccountEmailsRepo,
    repo::account_scopes::AccountScopesRepo,
    repo::accounts::AccountsRepo,
    repo::api_keys::ApiKeysRepo,
    repo::email_tokens::EmailTokensRepo,
    repo::route_policies::RoutePoliciesRepo,
    repo::sessions::SessionsRepo,
    service::{
        access::AccessService, accounts::AccountsService, auth_actor::AuthActorService,
        auth_context::AuthContextService, config::ConfigService, email_auth::EmailAuthService,
        mail_client::MailClientService, mailer::MailerService,
    },
};

pub trait DatabaseClient: Send + Sync {
    fn conn(&self) -> &DatabaseConnection;
}

pub struct SeaOrmDatabaseClient {
    conn: DatabaseConnection,
}

impl SeaOrmDatabaseClient {
    pub async fn new() -> Self {
        let conn = crate::db::connect()
            .await
            .expect("database connection failed");
        crate::schema::apply(&conn)
            .await
            .expect("schema apply failed");
        Self { conn }
    }
}

impl DatabaseClient for SeaOrmDatabaseClient {
    fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}

pub struct AppState {
    db: Arc<dyn DatabaseClient>,
    api_keys_repo: Arc<dyn ApiKeysRepo>,
    account_emails_repo: Arc<dyn AccountEmailsRepo>,
    email_tokens_repo: Arc<dyn EmailTokensRepo>,
    accounts_repo: Arc<dyn AccountsRepo>,
    account_scopes_repo: Arc<dyn AccountScopesRepo>,
    route_policies_repo: Arc<dyn RoutePoliciesRepo>,
    sessions_repo: Arc<dyn SessionsRepo>,
    auth_actor: Arc<dyn AuthActorService>,
    accounts: Arc<dyn AccountsService>,
    email_auth: Arc<dyn EmailAuthService>,
    access: Arc<dyn AccessService>,
    auth_context: Arc<dyn AuthContextService>,
    config: Arc<dyn ConfigService>,
}

impl AppState {
    pub async fn new() -> Arc<Self> {
        let db = Arc::new(SeaOrmDatabaseClient::new().await);
        let api_keys_repo = Arc::new(crate::repo::api_keys::SeaOrmApiKeysRepo::new(db.clone()));
        let account_emails_repo = Arc::new(
            crate::repo::account_emails::SeaOrmAccountEmailsRepo::new(db.clone()),
        );
        let email_tokens_repo = Arc::new(crate::repo::email_tokens::SeaOrmEmailTokensRepo::new(
            db.clone(),
        ));
        let accounts_repo = Arc::new(crate::repo::accounts::SeaOrmAccountsRepo::new(db.clone()));
        let account_scopes_repo = Arc::new(
            crate::repo::account_scopes::SeaOrmAccountScopesRepo::new(db.clone()),
        );
        let route_policies_repo = Arc::new(
            crate::repo::route_policies::SeaOrmRoutePoliciesRepo::new(db.clone()),
        );
        let sessions_repo = Arc::new(crate::repo::sessions::SeaOrmSessionsRepo::new(db.clone()));
        let accounts = Arc::new(crate::service::accounts::AccountsServiceImpl::new(
            accounts_repo.clone(),
        ));
        let config = Arc::new(crate::service::config::ConfigServiceImpl::new());
        let auth_actor = Arc::new(crate::service::auth_actor::AuthActorServiceImpl::new(
            config.clone(),
            api_keys_repo.clone(),
            accounts_repo.clone(),
            account_emails_repo.clone(),
            account_scopes_repo.clone(),
            sessions_repo.clone(),
        ));
        let mailer = Arc::new(crate::service::mailer::ResendMailerService::new(
            config.clone(),
        ));
        let mail_client = Arc::new(crate::service::mail_client::GrpcMailClientService::new(
            config.clone(),
        ));
        let access = Arc::new(crate::service::access::AccessServiceImpl::new(
            config.clone(),
            auth_actor.clone(),
            route_policies_repo.clone(),
        ));
        let auth_context = Arc::new(crate::service::auth_context::AuthContextServiceImpl::new(
            auth_actor.clone(),
        ));
        let email_auth = Arc::new(crate::service::email_auth::EmailAuthServiceImpl::new(
            accounts.clone(),
            accounts_repo.clone(),
            account_emails_repo.clone(),
            email_tokens_repo.clone(),
            sessions_repo.clone(),
            mail_client as Arc<dyn MailClientService>,
            mailer as Arc<dyn MailerService>,
            config.clone(),
        ));

        Arc::new(Self {
            db,
            api_keys_repo,
            account_emails_repo,
            email_tokens_repo,
            accounts_repo,
            account_scopes_repo,
            route_policies_repo,
            sessions_repo,
            auth_actor,
            accounts,
            email_auth,
            access,
            auth_context,
            config,
        })
    }

    pub fn db(&self) -> &dyn DatabaseClient {
        self.db.as_ref()
    }

    pub fn accounts(&self) -> &dyn AccountsService {
        self.accounts.as_ref()
    }

    pub fn access(&self) -> &dyn AccessService {
        self.access.as_ref()
    }

    pub fn auth_context(&self) -> &dyn AuthContextService {
        self.auth_context.as_ref()
    }

    pub fn auth_actor(&self) -> &dyn AuthActorService {
        self.auth_actor.as_ref()
    }

    pub fn email_auth(&self) -> &dyn EmailAuthService {
        self.email_auth.as_ref()
    }

    pub fn accounts_repo(&self) -> &dyn AccountsRepo {
        self.accounts_repo.as_ref()
    }

    pub fn api_keys_repo(&self) -> &dyn ApiKeysRepo {
        self.api_keys_repo.as_ref()
    }

    pub fn route_policies_repo(&self) -> &dyn RoutePoliciesRepo {
        self.route_policies_repo.as_ref()
    }

    pub fn account_scopes_repo(&self) -> &dyn AccountScopesRepo {
        self.account_scopes_repo.as_ref()
    }

    pub fn config(&self) -> &dyn ConfigService {
        self.config.as_ref()
    }
}
