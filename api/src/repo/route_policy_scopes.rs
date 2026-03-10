use async_trait::async_trait;

use crate::entities::route_policy_scopes;

#[async_trait]
pub trait RoutePolicyScopesRepo: Send + Sync {
    async fn list_by_route_policy_id(
        &self,
        route_policy_id: i64,
    ) -> Result<Vec<route_policy_scopes::Model>, sea_orm::DbErr>;
}
