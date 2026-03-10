use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
    repo::route_policies::RoutePoliciesRepo,
    repo::accounts::AccountsRepo,
    service::{
        access::AccessService, accounts::AccountsService, auth_context::AuthContextService,
        config::ConfigService,
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
    accounts_repo: Arc<dyn AccountsRepo>,
    route_policies_repo: Arc<dyn RoutePoliciesRepo>,
    accounts: Arc<dyn AccountsService>,
    access: Arc<dyn AccessService>,
    auth_context: Arc<dyn AuthContextService>,
    config: Arc<dyn ConfigService>,
}

impl AppState {
    pub async fn new() -> Arc<Self> {
        let db = Arc::new(SeaOrmDatabaseClient::new().await);
        let accounts_repo = Arc::new(crate::repo::accounts::SeaOrmAccountsRepo::new(db.clone()));
        let route_policies_repo =
            Arc::new(crate::repo::route_policies::SeaOrmRoutePoliciesRepo::new(db.clone()));
        let accounts = Arc::new(crate::service::accounts::AccountsServiceImpl::new(
            accounts_repo.clone(),
        ));
        let config = Arc::new(crate::service::config::ConfigServiceImpl::new());
        let access = Arc::new(crate::service::access::AccessServiceImpl::new(
            config.clone(),
            route_policies_repo.clone(),
        ));
        let auth_context = Arc::new(crate::service::auth_context::AuthContextServiceImpl::new(
            config.clone(),
        ));

        Arc::new(Self {
            db,
            accounts_repo,
            route_policies_repo,
            accounts,
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

    pub fn accounts_repo(&self) -> &dyn AccountsRepo {
        self.accounts_repo.as_ref()
    }

    pub fn route_policies_repo(&self) -> &dyn RoutePoliciesRepo {
        self.route_policies_repo.as_ref()
    }

    pub fn config(&self) -> &dyn ConfigService {
        self.config.as_ref()
    }
}
