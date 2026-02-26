use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::accounts, repo::accounts::AccountsRepo};

pub struct CreateAccountInput {
    pub account_type: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_by: Option<Uuid>,
}

pub struct UpdateAccountInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub updated_by: Option<Uuid>,
}

#[async_trait]
pub trait AccountsService: Send + Sync {
    async fn create(&self, input: CreateAccountInput) -> Result<accounts::Model, sea_orm::DbErr>;
    async fn get(&self, uid: Uuid) -> Result<Option<accounts::Model>, sea_orm::DbErr>;
    async fn update(
        &self,
        uid: Uuid,
        input: UpdateAccountInput,
    ) -> Result<Option<accounts::Model>, sea_orm::DbErr>;
    async fn delete(
        &self,
        uid: Uuid,
        deleted_by: Option<Uuid>,
    ) -> Result<Option<accounts::Model>, sea_orm::DbErr>;
}

pub struct AccountsServiceImpl {
    accounts_repo: std::sync::Arc<dyn AccountsRepo>,
}

impl AccountsServiceImpl {
    pub fn new(accounts_repo: std::sync::Arc<dyn AccountsRepo>) -> Self {
        Self { accounts_repo }
    }
}

#[async_trait]
impl AccountsService for AccountsServiceImpl {
    async fn create(&self, input: CreateAccountInput) -> Result<accounts::Model, sea_orm::DbErr> {
        let model = accounts::ActiveModel {
            uid: sea_orm::Set(Uuid::new_v4()),
            account_type: sea_orm::Set(input.account_type),
            username: sea_orm::Set(input.username),
            email: sea_orm::Set(input.email),
            phone: sea_orm::Set(input.phone),
            created_by: sea_orm::Set(input.created_by),
            updated_by: sea_orm::Set(input.created_by),
            ..Default::default()
        };

        self.accounts_repo.insert(model).await
    }

    async fn get(&self, uid: Uuid) -> Result<Option<accounts::Model>, sea_orm::DbErr> {
        self.accounts_repo.find_by_uid(uid).await
    }

    async fn update(
        &self,
        uid: Uuid,
        input: UpdateAccountInput,
    ) -> Result<Option<accounts::Model>, sea_orm::DbErr> {
        let Some(model) = self.accounts_repo.find_by_uid(uid).await? else {
            return Ok(None);
        };

        let mut active: accounts::ActiveModel = model.into();
        if let Some(username) = input.username {
            active.username = sea_orm::Set(Some(username));
        }
        if let Some(email) = input.email {
            active.email = sea_orm::Set(Some(email));
        }
        if let Some(phone) = input.phone {
            active.phone = sea_orm::Set(Some(phone));
        }
        active.updated_by = sea_orm::Set(input.updated_by);

        let updated = self.accounts_repo.update(active).await?;
        Ok(Some(updated))
    }

    async fn delete(
        &self,
        uid: Uuid,
        deleted_by: Option<Uuid>,
    ) -> Result<Option<accounts::Model>, sea_orm::DbErr> {
        let Some(model) = self.accounts_repo.find_by_uid(uid).await? else {
            return Ok(None);
        };

        let actor = deleted_by
            .or(model.updated_by)
            .or(model.created_by)
            .unwrap_or_else(Uuid::nil);
        let mut active: accounts::ActiveModel = model.into();
        active.deleted_at = sea_orm::Set(Some(chrono::Utc::now().into()));
        active.deleted_by = sea_orm::Set(Some(actor));
        active.updated_by = sea_orm::Set(Some(actor));

        let updated = self.accounts_repo.update(active).await?;
        Ok(Some(updated))
    }
}
