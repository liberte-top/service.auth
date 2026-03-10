use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("route_policy_scopes").await? {
        manager
            .create_table(
                Table::create()
                    .table(RoutePolicyScopes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoutePolicyScopes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RoutePolicyScopes::RoutePolicyId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RoutePolicyScopes::ScopeName)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_route_policy_scopes_route_policy_id")
                            .from(RoutePolicyScopes::Table, RoutePolicyScopes::RoutePolicyId)
                            .to(
                                super::route_policies::RoutePolicies::Table,
                                super::route_policies::RoutePolicies::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS route_policy_scopes_unique \
                 ON route_policy_scopes (route_policy_id, scope_name)"
                .to_string(),
        ))
        .await?;
    }

    seed_smoke_route_policy_scopes(conn).await?;

    Ok(())
}

async fn seed_smoke_route_policy_scopes(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
INSERT INTO route_policy_scopes (route_policy_id, scope_name)
SELECT rp.id, seed.scope_name
FROM (
  VALUES
    ('smoke.viewer.read', 'notes:read'),
    ('smoke.notes.read', 'notes:read'),
    ('smoke.notes.write', 'notes:write')
) AS seed(route_key, scope_name)
JOIN route_policies rp ON rp.route_key = seed.route_key
ON CONFLICT DO NOTHING;
"#
        .to_string(),
    ))
    .await?;

    Ok(())
}

#[derive(Iden)]
pub enum RoutePolicyScopes {
    Table,
    Id,
    RoutePolicyId,
    ScopeName,
}
