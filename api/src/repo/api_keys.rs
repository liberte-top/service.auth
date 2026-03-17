use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{entities::api_keys, state::DatabaseClient};

#[async_trait]
pub trait ApiKeysRepo: Send + Sync {
    async fn find_active_by_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<Option<api_keys::Model>, sea_orm::DbErr>;
    async fn touch_last_used(
        &self,
        model: api_keys::Model,
    ) -> Result<api_keys::Model, sea_orm::DbErr>;
    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<api_keys::Model>, sea_orm::DbErr>;
    async fn insert(
        &self,
        model: api_keys::ActiveModel,
    ) -> Result<api_keys::Model, sea_orm::DbErr>;
    async fn find_by_id_and_account_id(
        &self,
        id: i64,
        account_id: i64,
    ) -> Result<Option<api_keys::Model>, sea_orm::DbErr>;
    async fn revoke(
        &self,
        model: api_keys::Model,
    ) -> Result<api_keys::Model, sea_orm::DbErr>;
}

pub struct SeaOrmApiKeysRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmApiKeysRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ApiKeysRepo for SeaOrmApiKeysRepo {
    async fn find_active_by_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<Option<api_keys::Model>, sea_orm::DbErr> {
        api_keys::Entity::find()
            .filter(
                Condition::any()
                    .add(api_keys::Column::KeyHash.eq(key_hash))
                    .add(api_keys::Column::KeyHash.eq(crate::service::api_tokens::ApiTokensServiceImpl::hash_token(key_hash))),
            )
            .filter(api_keys::Column::RevokedAt.is_null())
            .filter(
                api_keys::Column::ExpiresAt
                    .is_null()
                    .or(api_keys::Column::ExpiresAt.gt(Utc::now())),
            )
            .one(self.db.conn())
            .await
    }

    async fn touch_last_used(
        &self,
        model: api_keys::Model,
    ) -> Result<api_keys::Model, sea_orm::DbErr> {
        let mut active: api_keys::ActiveModel = model.into();
        active.last_used_at = Set(Some(Utc::now().into()));
        active.update(self.db.conn()).await
    }

    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<api_keys::Model>, sea_orm::DbErr> {
        api_keys::Entity::find()
            .filter(api_keys::Column::AccountId.eq(account_id))
            .all(self.db.conn())
            .await
    }

    async fn insert(
        &self,
        model: api_keys::ActiveModel,
    ) -> Result<api_keys::Model, sea_orm::DbErr> {
        model.insert(self.db.conn()).await
    }

    async fn find_by_id_and_account_id(
        &self,
        id: i64,
        account_id: i64,
    ) -> Result<Option<api_keys::Model>, sea_orm::DbErr> {
        api_keys::Entity::find_by_id(id)
            .filter(api_keys::Column::AccountId.eq(account_id))
            .one(self.db.conn())
            .await
    }

    async fn revoke(
        &self,
        model: api_keys::Model,
    ) -> Result<api_keys::Model, sea_orm::DbErr> {
        let mut active: api_keys::ActiveModel = model.into();
        active.revoked_at = Set(Some(Utc::now().into()));
        active.update(self.db.conn()).await
    }
}
