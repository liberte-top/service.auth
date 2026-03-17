use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::api_key_scopes, state::DatabaseClient};

#[async_trait]
pub trait ApiKeyScopesRepo: Send + Sync {
    async fn list_by_api_key_id(
        &self,
        api_key_id: i64,
    ) -> Result<Vec<api_key_scopes::Model>, sea_orm::DbErr>;
    async fn insert_many(
        &self,
        models: Vec<api_key_scopes::ActiveModel>,
    ) -> Result<(), sea_orm::DbErr>;
}

pub struct SeaOrmApiKeyScopesRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmApiKeyScopesRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ApiKeyScopesRepo for SeaOrmApiKeyScopesRepo {
    async fn list_by_api_key_id(
        &self,
        api_key_id: i64,
    ) -> Result<Vec<api_key_scopes::Model>, sea_orm::DbErr> {
        api_key_scopes::Entity::find()
            .filter(api_key_scopes::Column::ApiKeyId.eq(api_key_id))
            .all(self.db.conn())
            .await
    }

    async fn insert_many(
        &self,
        models: Vec<api_key_scopes::ActiveModel>,
    ) -> Result<(), sea_orm::DbErr> {
        for model in models {
            model.insert(self.db.conn()).await?;
        }
        Ok(())
    }
}
