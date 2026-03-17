use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("api_key_scopes").await? {
        manager
            .create_table(
                Table::create()
                    .table(ApiKeyScopes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKeyScopes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKeyScopes::ApiKeyId).big_integer().not_null())
                    .col(ColumnDef::new(ApiKeyScopes::ScopeName).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_api_key_scopes_api_key_id")
                            .from(ApiKeyScopes::Table, ApiKeyScopes::ApiKeyId)
                            .to(super::api_keys::ApiKeys::Table, super::api_keys::ApiKeys::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS api_key_scopes_unique ON api_key_scopes (api_key_id, scope_name)".to_string(),
        ))
        .await?;
    }

    Ok(())
}

#[derive(Iden)]
pub enum ApiKeyScopes {
    Table,
    Id,
    ApiKeyId,
    ScopeName,
}
