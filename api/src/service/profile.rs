use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    entities::account_profiles,
    repo::{
        account_emails::AccountEmailsRepo,
        account_profiles::AccountProfilesRepo,
        accounts::AccountsRepo,
    },
};

#[derive(Serialize, ToSchema)]
pub struct SelfProfileResponse {
    pub subject: String,
    pub principal_type: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub display_name: Option<String>,
    pub scopes: Vec<String>,
}

pub struct UpdateSelfProfileInput {
    pub account_id: i64,
    pub subject: String,
    pub principal_type: String,
    pub scopes: Vec<String>,
    pub display_name: Option<String>,
}

#[async_trait]
pub trait ProfileService: Send + Sync {
    async fn get_self_profile(
        &self,
        account_id: i64,
        subject: String,
        principal_type: String,
        scopes: Vec<String>,
    ) -> Result<SelfProfileResponse, sea_orm::DbErr>;
    async fn update_self_profile(
        &self,
        input: UpdateSelfProfileInput,
    ) -> Result<SelfProfileResponse, sea_orm::DbErr>;
}

pub struct ProfileServiceImpl {
    account_profiles_repo: Arc<dyn AccountProfilesRepo>,
    account_emails_repo: Arc<dyn AccountEmailsRepo>,
    accounts_repo: Arc<dyn AccountsRepo>,
}

impl ProfileServiceImpl {
    pub fn new(
        account_profiles_repo: Arc<dyn AccountProfilesRepo>,
        account_emails_repo: Arc<dyn AccountEmailsRepo>,
        accounts_repo: Arc<dyn AccountsRepo>,
    ) -> Self {
        Self {
            account_profiles_repo,
            account_emails_repo,
            accounts_repo,
        }
    }

    async fn response_for(
        &self,
        account_id: i64,
        subject: String,
        principal_type: String,
        scopes: Vec<String>,
    ) -> Result<SelfProfileResponse, sea_orm::DbErr> {
        let profile = self.account_profiles_repo.find_by_account_id(account_id).await?;
        let account = self.accounts_repo.find_by_id(account_id).await?;
        let email = self
            .account_emails_repo
            .find_primary_by_account_id(account_id)
            .await?;

        Ok(SelfProfileResponse {
            subject,
            principal_type,
            email: email.as_ref().map(|item| item.email_normalized.clone()),
            email_verified: email.and_then(|item| item.verified_at).is_some(),
            display_name: profile
                .and_then(|item| item.display_name)
                .or_else(|| account.and_then(|item| item.username)),
            scopes,
        })
    }
}

#[async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn get_self_profile(
        &self,
        account_id: i64,
        subject: String,
        principal_type: String,
        scopes: Vec<String>,
    ) -> Result<SelfProfileResponse, sea_orm::DbErr> {
        self.response_for(account_id, subject, principal_type, scopes)
            .await
    }

    async fn update_self_profile(
        &self,
        input: UpdateSelfProfileInput,
    ) -> Result<SelfProfileResponse, sea_orm::DbErr> {
        let display_name = input
            .display_name
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        if let Some(model) = self
            .account_profiles_repo
            .find_by_account_id(input.account_id)
            .await?
        {
            let mut active: account_profiles::ActiveModel = model.into();
            active.display_name = sea_orm::Set(display_name);
            self.account_profiles_repo.update(active).await?;
        } else {
            let active = account_profiles::ActiveModel {
                account_id: sea_orm::Set(input.account_id),
                display_name: sea_orm::Set(display_name),
                ..Default::default()
            };
            self.account_profiles_repo.insert(active).await?;
        }

        self.response_for(
            input.account_id,
            input.subject,
            input.principal_type,
            input.scopes,
        )
        .await
    }
}
