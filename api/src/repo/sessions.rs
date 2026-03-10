use async_trait::async_trait;

use crate::entities::sessions;

#[async_trait]
pub trait SessionsRepo: Send + Sync {
    async fn find_active_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<sessions::Model>, sea_orm::DbErr>;
}
