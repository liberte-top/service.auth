use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::sessions, state::DatabaseClient};

#[async_trait]
pub trait SessionsRepo: Send + Sync {
    async fn find_active_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<sessions::Model>, sea_orm::DbErr>;
}

pub struct SeaOrmSessionsRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmSessionsRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionsRepo for SeaOrmSessionsRepo {
    async fn find_active_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<sessions::Model>, sea_orm::DbErr> {
        sessions::Entity::find()
            .filter(sessions::Column::TokenHash.eq(token_hash))
            .filter(sessions::Column::RevokedAt.is_null())
            .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
            .one(self.db.conn())
            .await
    }
}
