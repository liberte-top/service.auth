use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("account_scopes").await? {
        manager
            .create_table(
                Table::create()
                    .table(AccountScopes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccountScopes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AccountScopes::AccountId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccountScopes::ScopeName).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_scopes_account_id")
                            .from(AccountScopes::Table, AccountScopes::AccountId)
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
            "CREATE UNIQUE INDEX IF NOT EXISTS account_scopes_unique \
                 ON account_scopes (account_id, scope_name)"
                .to_string(),
        ))
        .await?;
    }

    seed_demo_account_scopes(conn).await?;

    Ok(())
}

async fn seed_demo_account_scopes(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
INSERT INTO account_scopes (account_id, scope_name)
SELECT a.id, seed.scope_name
FROM (
  VALUES
    ('notes:read'),
    ('profile:read')
) AS seed(scope_name)
JOIN accounts a ON lower(a.username) = 'demo-user'
ON CONFLICT DO NOTHING;
"#
        .to_string(),
    ))
    .await?;

    Ok(())
}

#[derive(Iden)]
pub enum AccountScopes {
    Table,
    Id,
    AccountId,
    ScopeName,
}
