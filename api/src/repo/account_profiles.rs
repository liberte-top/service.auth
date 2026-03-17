use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::account_profiles, state::DatabaseClient};

#[async_trait]
pub trait AccountProfilesRepo: Send + Sync {
    async fn find_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Option<account_profiles::Model>, sea_orm::DbErr>;
    async fn insert(
        &self,
        model: account_profiles::ActiveModel,
    ) -> Result<account_profiles::Model, sea_orm::DbErr>;
    async fn update(
        &self,
        model: account_profiles::ActiveModel,
    ) -> Result<account_profiles::Model, sea_orm::DbErr>;
}

pub struct SeaOrmAccountProfilesRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmAccountProfilesRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountProfilesRepo for SeaOrmAccountProfilesRepo {
    async fn find_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Option<account_profiles::Model>, sea_orm::DbErr> {
        account_profiles::Entity::find()
            .filter(account_profiles::Column::AccountId.eq(account_id))
            .one(self.db.conn())
            .await
    }

    async fn insert(
        &self,
        model: account_profiles::ActiveModel,
    ) -> Result<account_profiles::Model, sea_orm::DbErr> {
        model.insert(self.db.conn()).await
    }

    async fn update(
        &self,
        model: account_profiles::ActiveModel,
    ) -> Result<account_profiles::Model, sea_orm::DbErr> {
        model.update(self.db.conn()).await
    }
}
