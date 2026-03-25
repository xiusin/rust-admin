use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_developer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub contact: Option<String>,
    pub plugins_count: i32,
    pub total_downloads: i64,
    pub rating: Decimal,
    pub status: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_plugin::Entity")]
    Plugin,
}

impl Related<super::p_plugin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}