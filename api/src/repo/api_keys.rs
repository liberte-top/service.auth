use async_trait::async_trait;

use crate::entities::api_keys;

#[async_trait]
pub trait ApiKeysRepo: Send + Sync {
    async fn find_active_by_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<Option<api_keys::Model>, sea_orm::DbErr>;
}
