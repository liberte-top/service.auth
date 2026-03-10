use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn apply(manager: &SchemaManager<'_>, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if !manager.has_table("route_policies").await? {
        manager
            .create_table(
                Table::create()
                    .table(RoutePolicies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoutePolicies::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RoutePolicies::ServiceSlug)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RoutePolicies::RouteKey).string().not_null())
                    .col(
                        ColumnDef::new(RoutePolicies::HostPattern)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RoutePolicies::PathPattern)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RoutePolicies::Method).string().not_null())
                    .col(
                        ColumnDef::new(RoutePolicies::Enabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(RoutePolicies::Priority)
                            .integer()
                            .not_null()
                            .default(100),
                    )
                    .col(
                        ColumnDef::new(RoutePolicies::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .col(
                        ColumnDef::new(RoutePolicies::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .to_owned(),
            )
            .await?;

        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            "CREATE UNIQUE INDEX IF NOT EXISTS route_policies_service_route_method_unique \
                 ON route_policies (service_slug, route_key, method)"
                .to_string(),
        ))
        .await?;
    }

    seed_smoke_route_policies(conn).await?;

    Ok(())
}

async fn seed_smoke_route_policies(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
INSERT INTO route_policies (service_slug, route_key, host_pattern, path_pattern, method, enabled, priority)
VALUES
  ('smoke', 'smoke.viewer.read', 'smoke.liberte.top', '/api/v1/viewer', 'GET', true, 100),
  ('smoke', 'smoke.notes.read', 'smoke.liberte.top', '/api/v1/notes', 'GET', true, 100),
  ('smoke', 'smoke.notes.write', 'smoke.liberte.top', '/api/v1/notes', 'POST', true, 100)
ON CONFLICT (service_slug, route_key, method) DO UPDATE SET
  host_pattern = EXCLUDED.host_pattern,
  path_pattern = EXCLUDED.path_pattern,
  enabled = EXCLUDED.enabled,
  priority = EXCLUDED.priority;
"#
        .to_string(),
    ))
    .await?;

    Ok(())
}

#[derive(Iden)]
pub enum RoutePolicies {
    Table,
    Id,
    ServiceSlug,
    RouteKey,
    HostPattern,
    PathPattern,
    Method,
    Enabled,
    Priority,
    CreatedAt,
    UpdatedAt,
}
