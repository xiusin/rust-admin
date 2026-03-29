//! `SeaORM` Entity for style_template

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "style_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(string_len = 100)]
    pub name: String,
    #[sea_orm(string_len = 50, nullable)]
    pub industry: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub color_scheme: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub font_scheme: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub layout_rules: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub component_styles: Option<Json>,
    #[sea_orm(string_len = 500, nullable)]
    pub preview_url: Option<String>,
    pub usage_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ppt_project::Entity")]
    PptProject,
}

impl Related<super::ppt_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PptProject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
