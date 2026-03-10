use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    entities::{route_policies, route_policy_scopes},
    state::DatabaseClient,
};

pub struct RoutePolicyRecord {
    pub route_key: String,
    pub host_pattern: String,
    pub path_pattern: String,
    pub method: String,
    pub required_scopes: Vec<String>,
}

#[async_trait]
pub trait RoutePoliciesRepo: Send + Sync {
    async fn list_enabled(&self) -> Result<Vec<RoutePolicyRecord>, sea_orm::DbErr>;
}

pub struct SeaOrmRoutePoliciesRepo {
    db: std::sync::Arc<dyn DatabaseClient>,
}

impl SeaOrmRoutePoliciesRepo {
    pub fn new(db: std::sync::Arc<dyn DatabaseClient>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoutePoliciesRepo for SeaOrmRoutePoliciesRepo {
    async fn list_enabled(&self) -> Result<Vec<RoutePolicyRecord>, sea_orm::DbErr> {
        let policies = route_policies::Entity::find()
            .filter(route_policies::Column::Enabled.eq(true))
            .order_by_asc(route_policies::Column::Priority)
            .all(self.db.conn())
            .await?;

        let mut results = Vec::with_capacity(policies.len());
        for policy in policies {
            let scopes = route_policy_scopes::Entity::find()
                .filter(route_policy_scopes::Column::RoutePolicyId.eq(policy.id))
                .all(self.db.conn())
                .await?
                .into_iter()
                .map(|item| item.scope_name)
                .collect();

            results.push(RoutePolicyRecord {
                route_key: policy.route_key,
                host_pattern: policy.host_pattern,
                path_pattern: policy.path_pattern,
                method: policy.method,
                required_scopes: scopes,
            });
        }

        Ok(results)
    }
}
