use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_plan")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub plugin_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub period_type: i32,
    pub period_days: i32,
    pub price: Decimal,
    pub original_price: Decimal,
    pub features: Option<Json>,
    pub max_devices: i32,
    pub max_users: i32,
    pub storage_limit: i64,
    pub api_calls_limit: i64,
    pub support_level: i32,
    pub sort: i32,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_plugin::Entity",
        from = "Column::PluginId",
        to = "super::p_plugin::Column::Id"
    )]
    Plugin,
}

impl Related<super::p_plugin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}