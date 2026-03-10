use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::account_scopes, state::DatabaseClient};

#[async_trait]
pub trait AccountScopesRepo: Send + Sync {
    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<account_scopes::Model>, sea_orm::DbErr>;
}

pub struct SeaOrmAccountScopesRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmAccountScopesRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountScopesRepo for SeaOrmAccountScopesRepo {
    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<account_scopes::Model>, sea_orm::DbErr> {
        account_scopes::Entity::find()
            .filter(account_scopes::Column::AccountId.eq(account_id))
            .all(self.db.conn())
            .await
    }
}
