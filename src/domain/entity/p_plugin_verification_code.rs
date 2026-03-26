use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_plugin_verification_code")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub code: String,
    pub license_id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub purpose: i32,
    pub device_hash: Option<String>,
    pub status: i32,
    pub attempts: i32,
    pub expire_time: DateTime,
    pub verified_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_license::Entity",
        from = "Column::LicenseId",
        to = "super::p_license::Column::Id"
    )]
    License,
    #[sea_orm(
        belongs_to = "super::p_plugin::Entity",
        from = "Column::PluginId",
        to = "super::p_plugin::Column::Id"
    )]
    Plugin,
}

impl Related<super::p_license::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::License.def()
    }
}

impl Related<super::p_plugin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
