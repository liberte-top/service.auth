use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    entities::{api_key_scopes, api_keys},
    repo::{api_key_scopes::ApiKeyScopesRepo, api_keys::ApiKeysRepo},
};

#[derive(Serialize, ToSchema)]
pub struct ApiTokenSummary {
    pub id: i64,
    pub name: String,
    pub prefix: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiTokenSecret {
    pub token: String,
    pub summary: ApiTokenSummary,
}

pub struct CreateApiTokenInput {
    pub account_id: i64,
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

#[async_trait]
pub trait ApiTokensService: Send + Sync {
    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<ApiTokenSummary>, sea_orm::DbErr>;
    async fn create(
        &self,
        input: CreateApiTokenInput,
    ) -> Result<ApiTokenSecret, sea_orm::DbErr>;
    async fn revoke(
        &self,
        account_id: i64,
        id: i64,
    ) -> Result<Option<ApiTokenSummary>, sea_orm::DbErr>;
}

pub struct ApiTokensServiceImpl {
    api_keys_repo: Arc<dyn ApiKeysRepo>,
    api_key_scopes_repo: Arc<dyn ApiKeyScopesRepo>,
}

impl ApiTokensServiceImpl {
    pub fn new(
        api_keys_repo: Arc<dyn ApiKeysRepo>,
        api_key_scopes_repo: Arc<dyn ApiKeyScopesRepo>,
    ) -> Self {
        Self {
            api_keys_repo,
            api_key_scopes_repo,
        }
    }

    async fn summary(&self, model: api_keys::Model) -> Result<ApiTokenSummary, sea_orm::DbErr> {
        let scopes = self
            .api_key_scopes_repo
            .list_by_api_key_id(model.id)
            .await?
            .into_iter()
            .map(|item| item.scope_name)
            .collect();

        Ok(ApiTokenSummary {
            id: model.id,
            name: model.name,
            prefix: model.key_prefix,
            created_at: model.created_at.with_timezone(&Utc),
            last_used_at: model.last_used_at.map(|value| value.with_timezone(&Utc)),
            expires_at: model.expires_at.map(|value| value.with_timezone(&Utc)),
            revoked_at: model.revoked_at.map(|value| value.with_timezone(&Utc)),
            scopes,
        })
    }

    fn generate_token() -> String {
        format!("ltpk_{}", Uuid::new_v4().simple())
    }

    fn prefix_for(token: &str) -> String {
        token.chars().take(12).collect()
    }

    pub fn hash_token(raw: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[async_trait]
impl ApiTokensService for ApiTokensServiceImpl {
    async fn list_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<ApiTokenSummary>, sea_orm::DbErr> {
        let models = self.api_keys_repo.list_by_account_id(account_id).await?;
        let mut tokens = Vec::with_capacity(models.len());
        for model in models {
            tokens.push(self.summary(model).await?);
        }
        Ok(tokens)
    }

    async fn create(
        &self,
        input: CreateApiTokenInput,
    ) -> Result<ApiTokenSecret, sea_orm::DbErr> {
        let token = Self::generate_token();
        let prefix = Self::prefix_for(&token);
        let inserted = self
            .api_keys_repo
            .insert(api_keys::ActiveModel {
                account_id: sea_orm::Set(input.account_id),
                name: sea_orm::Set(input.name),
                key_prefix: sea_orm::Set(prefix),
                key_hash: sea_orm::Set(Self::hash_token(&token)),
                expires_at: sea_orm::Set(input.expires_at.map(Into::into)),
                ..Default::default()
            })
            .await?;

        if !input.scopes.is_empty() {
            self.api_key_scopes_repo
                .insert_many(
                    input
                        .scopes
                        .into_iter()
                        .map(|scope_name| api_key_scopes::ActiveModel {
                            api_key_id: sea_orm::Set(inserted.id),
                            scope_name: sea_orm::Set(scope_name),
                            ..Default::default()
                        })
                        .collect(),
                )
                .await?;
        }

        Ok(ApiTokenSecret {
            token,
            summary: self.summary(inserted).await?,
        })
    }

    async fn revoke(
        &self,
        account_id: i64,
        id: i64,
    ) -> Result<Option<ApiTokenSummary>, sea_orm::DbErr> {
        let Some(model) = self
            .api_keys_repo
            .find_by_id_and_account_id(id, account_id)
            .await?
        else {
            return Ok(None);
        };

        let revoked = self.api_keys_repo.revoke(model).await?;
        Ok(Some(self.summary(revoked).await?))
    }

}
