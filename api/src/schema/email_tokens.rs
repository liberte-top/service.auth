use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("email_tokens").await? {
        manager
            .create_table(
                Table::create()
                    .table(EmailTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailTokens::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailTokens::AccountEmailId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmailTokens::Purpose).string().not_null())
                    .col(ColumnDef::new(EmailTokens::TokenHash).string().not_null())
                    .col(
                        ColumnDef::new(EmailTokens::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmailTokens::ConsumedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(EmailTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_email_tokens_account_email_id")
                            .from(EmailTokens::Table, EmailTokens::AccountEmailId)
                            .to(
                                super::account_emails::AccountEmails::Table,
                                super::account_emails::AccountEmails::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS email_tokens_token_hash_unique \
                 ON email_tokens (token_hash)"
                .to_string(),
        ))
        .await?;
    }

    Ok(())
}

#[derive(Iden)]
pub enum EmailTokens {
    Table,
    Id,
    AccountEmailId,
    Purpose,
    TokenHash,
    ExpiresAt,
    ConsumedAt,
    CreatedAt,
}
