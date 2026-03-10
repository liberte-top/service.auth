use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "route_policies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub service_slug: String,
    pub route_key: String,
    pub host_pattern: String,
    pub path_pattern: String,
    pub method: String,
    pub enabled: bool,
    pub priority: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
