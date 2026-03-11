use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::account_emails, state::DatabaseClient};

#[async_trait]
pub trait AccountEmailsRepo: Send + Sync {
    async fn insert(
        &self,
        model: account_emails::ActiveModel,
    ) -> Result<account_emails::Model, sea_orm::DbErr>;
    async fn find_primary_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Option<account_emails::Model>, sea_orm::DbErr>;
    async fn find_by_email(
        &self,
        email_normalized: &str,
    ) -> Result<Option<account_emails::Model>, sea_orm::DbErr>;
    async fn find_by_id(&self, id: i64) -> Result<Option<account_emails::Model>, sea_orm::DbErr>;
    async fn mark_verified(
        &self,
        model: account_emails::Model,
    ) -> Result<account_emails::Model, sea_orm::DbErr>;
}

pub struct SeaOrmAccountEmailsRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmAccountEmailsRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountEmailsRepo for SeaOrmAccountEmailsRepo {
    async fn insert(
        &self,
        model: account_emails::ActiveModel,
    ) -> Result<account_emails::Model, sea_orm::DbErr> {
        model.insert(self.db.conn()).await
    }

    async fn find_primary_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Option<account_emails::Model>, sea_orm::DbErr> {
        account_emails::Entity::find()
            .filter(account_emails::Column::AccountId.eq(account_id))
            .filter(account_emails::Column::IsPrimary.eq(true))
            .one(self.db.conn())
            .await
    }

    async fn find_by_email(
        &self,
        email_normalized: &str,
    ) -> Result<Option<account_emails::Model>, sea_orm::DbErr> {
        account_emails::Entity::find()
            .filter(account_emails::Column::EmailNormalized.eq(email_normalized))
            .one(self.db.conn())
            .await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<account_emails::Model>, sea_orm::DbErr> {
        account_emails::Entity::find_by_id(id)
            .one(self.db.conn())
            .await
    }

    async fn mark_verified(
        &self,
        model: account_emails::Model,
    ) -> Result<account_emails::Model, sea_orm::DbErr> {
        let mut active: account_emails::ActiveModel = model.into();
        active.verified_at = sea_orm::Set(Some(chrono::Utc::now().into()));
        active.update(self.db.conn()).await
    }
}
