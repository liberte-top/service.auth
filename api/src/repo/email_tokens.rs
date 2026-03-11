use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::email_tokens, state::DatabaseClient};

#[async_trait]
pub trait EmailTokensRepo: Send + Sync {
    async fn insert(
        &self,
        model: email_tokens::ActiveModel,
    ) -> Result<email_tokens::Model, sea_orm::DbErr>;
    async fn find_active_by_token_hash(
        &self,
        token_hash: &str,
        purpose: &str,
    ) -> Result<Option<email_tokens::Model>, sea_orm::DbErr>;
    async fn mark_consumed(
        &self,
        model: email_tokens::Model,
    ) -> Result<email_tokens::Model, sea_orm::DbErr>;
}

pub struct SeaOrmEmailTokensRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmEmailTokensRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EmailTokensRepo for SeaOrmEmailTokensRepo {
    async fn insert(
        &self,
        model: email_tokens::ActiveModel,
    ) -> Result<email_tokens::Model, sea_orm::DbErr> {
        model.insert(self.db.conn()).await
    }

    async fn find_active_by_token_hash(
        &self,
        token_hash: &str,
        purpose: &str,
    ) -> Result<Option<email_tokens::Model>, sea_orm::DbErr> {
        email_tokens::Entity::find()
            .filter(email_tokens::Column::TokenHash.eq(token_hash))
            .filter(email_tokens::Column::Purpose.eq(purpose))
            .filter(email_tokens::Column::ConsumedAt.is_null())
            .filter(email_tokens::Column::ExpiresAt.gt(Utc::now()))
            .one(self.db.conn())
            .await
    }

    async fn mark_consumed(
        &self,
        model: email_tokens::Model,
    ) -> Result<email_tokens::Model, sea_orm::DbErr> {
        let mut active: email_tokens::ActiveModel = model.into();
        active.consumed_at = sea_orm::Set(Some(Utc::now().into()));
        active.update(self.db.conn()).await
    }
}
