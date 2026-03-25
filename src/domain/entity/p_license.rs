use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_license")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub license_key: String,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub ip_address: Option<String>,
    pub bind_count: i32,
    pub max_devices: i32,
    pub verify_count: i32,
    pub last_verify_time: Option<DateTime>,
    pub status: i32,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_plugin::Entity",
        from = "Column::PluginId",
        to = "super::p_plugin::Column::Id"
    )]
    Plugin,
    #[sea_orm(
        belongs_to = "super::p_plan::Entity",
        from = "Column::PlanId",
        to = "super::p_plan::Column::Id"
    )]
    Plan,
}

impl Related<super::p_plugin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugin.def()
    }
}

impl Related<super::p_plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
