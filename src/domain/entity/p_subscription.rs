use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_subscription")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub auto_renew: i32,
    pub status: i32,
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
