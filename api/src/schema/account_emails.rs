use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("account_emails").await? {
        manager
            .create_table(
                Table::create()
                    .table(AccountEmails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccountEmails::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AccountEmails::AccountId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AccountEmails::EmailNormalized)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccountEmails::VerifiedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(AccountEmails::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AccountEmails::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_emails_account_id")
                            .from(AccountEmails::Table, AccountEmails::AccountId)
                            .to(
                                super::accounts::Accounts::Table,
                                super::accounts::Accounts::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS account_emails_account_primary_unique \
                 ON account_emails (account_id) WHERE is_primary = true"
                .to_string(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS account_emails_email_unique \
                 ON account_emails (email_normalized)"
                .to_string(),
        ))
        .await?;
    }

    Ok(())
}

#[derive(Iden)]
pub enum AccountEmails {
    Table,
    Id,
    AccountId,
    EmailNormalized,
    VerifiedAt,
    IsPrimary,
    CreatedAt,
}
