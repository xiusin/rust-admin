use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_plugin_review")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub plugin_id: i64,
    pub user_id: i64,
    pub rating: i32,
    pub content: Option<String>,
    pub reply_content: Option<String>,
    pub reply_time: Option<DateTime>,
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
