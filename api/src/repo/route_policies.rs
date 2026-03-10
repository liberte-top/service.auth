use async_trait::async_trait;

use crate::entities::route_policies;

#[async_trait]
pub trait RoutePoliciesRepo: Send + Sync {
    async fn list_enabled(&self) -> Result<Vec<route_policies::Model>, sea_orm::DbErr>;
}
