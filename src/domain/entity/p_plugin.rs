use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_plugin")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub code: String,
    pub name: String,
    pub category_id: i64,
    pub developer_id: i64,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Json>,
    pub version: String,
    pub price_type: i32,
    pub verify_level: i32,
    pub download_count: i32,
    pub rating: Decimal,
    pub status: i32,
    pub sort: i32,
    pub tags: Option<Json>,
    pub is_official: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_plugin_category::Entity",
        from = "Column::CategoryId",
        to = "super::p_plugin_category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::p_developer::Entity",
        from = "Column::DeveloperId",
        to = "super::p_developer::Column::Id"
    )]
    Developer,
    #[sea_orm(has_many = "super::p_plugin_version::Entity")]
    PluginVersion,
    #[sea_orm(has_many = "super::p_plan::Entity")]
    Plan,
    #[sea_orm(has_many = "super::p_plugin_review::Entity")]
    PluginReview,
}

impl Related<super::p_plugin_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::p_developer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Developer.def()
    }
}

impl Related<super::p_plugin_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PluginVersion.def()
    }
}

impl Related<super::p_plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl Related<super::p_plugin_review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PluginReview.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
