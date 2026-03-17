use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("account_profiles").await? {
        manager
            .create_table(
                Table::create()
                    .table(AccountProfiles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccountProfiles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AccountProfiles::AccountId).big_integer().not_null())
                    .col(ColumnDef::new(AccountProfiles::DisplayName).string())
                    .col(
                        ColumnDef::new(AccountProfiles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .col(
                        ColumnDef::new(AccountProfiles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_profiles_account_id")
                            .from(AccountProfiles::Table, AccountProfiles::AccountId)
                            .to(super::accounts::Accounts::Table, super::accounts::Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS account_profiles_account_id_unique ON account_profiles (account_id)".to_string(),
        ))
        .await?;
    }

    Ok(())
}

#[derive(Iden)]
pub enum AccountProfiles {
    Table,
    Id,
    AccountId,
    DisplayName,
    CreatedAt,
    UpdatedAt,
}
