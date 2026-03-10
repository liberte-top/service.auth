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
