use async_trait::async_trait;

use crate::entities::account_emails;

#[async_trait]
pub trait AccountEmailsRepo: Send + Sync {
    async fn find_primary_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Option<account_emails::Model>, sea_orm::DbErr>;
}
