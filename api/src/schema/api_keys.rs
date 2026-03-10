use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("api_keys").await? {
        manager
            .create_table(
                Table::create()
                    .table(ApiKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKeys::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKeys::AccountId).big_integer().not_null())
                    .col(ColumnDef::new(ApiKeys::Name).string().not_null())
                    .col(ColumnDef::new(ApiKeys::KeyPrefix).string().not_null())
                    .col(ColumnDef::new(ApiKeys::KeyHash).string().not_null())
                    .col(ColumnDef::new(ApiKeys::LastUsedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ApiKeys::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ApiKeys::RevokedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ApiKeys::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_api_keys_account_id")
                            .from(ApiKeys::Table, ApiKeys::AccountId)
                            .to(super::accounts::Accounts::Table, super::accounts::Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS api_keys_key_hash_unique \
                 ON api_keys (key_hash)"
                .to_string(),
        ))
        .await?;
    }

    Ok(())
}

#[derive(Iden)]
pub enum ApiKeys {
    Table,
    Id,
    AccountId,
    Name,
    KeyPrefix,
    KeyHash,
    LastUsedAt,
    ExpiresAt,
    RevokedAt,
    CreatedAt,
}
