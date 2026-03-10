use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("sessions").await? {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sessions::AccountId).big_integer().not_null())
                    .col(ColumnDef::new(Sessions::TokenHash).string().not_null())
                    .col(
                        ColumnDef::new(Sessions::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Sessions::RevokedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Sessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_account_id")
                            .from(Sessions::Table, Sessions::AccountId)
                            .to(super::accounts::Accounts::Table, super::accounts::Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS sessions_token_hash_unique \
                 ON sessions (token_hash)"
                .to_string(),
        ))
        .await?;
    }

    seed_demo_session(conn).await?;

    Ok(())
}

async fn seed_demo_session(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
INSERT INTO sessions (account_id, token_hash, expires_at)
SELECT a.id, 'demo-smoke-session', now() + interval '365 days'
FROM accounts a
WHERE lower(a.username) = 'demo-user'
ON CONFLICT DO NOTHING;
"#
        .to_string(),
    ))
    .await?;

    Ok(())
}

#[derive(Iden)]
pub enum Sessions {
    Table,
    Id,
    AccountId,
    TokenHash,
    ExpiresAt,
    RevokedAt,
    CreatedAt,
}
