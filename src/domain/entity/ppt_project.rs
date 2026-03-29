//! `SeaORM` Entity for ppt_project

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    #[sea_orm(string_len = 200)]
    pub title: String,
    #[sea_orm(string_len = 500, nullable)]
    pub description: Option<String>,
    #[sea_orm(string_len = 20)]
    pub source_type: String,
    #[sea_orm(string_len = 500, nullable)]
    pub source_file: Option<String>,
    pub style_template_id: Option<i64>,
    #[sea_orm(string_len = 50, nullable)]
    pub industry: Option<String>,
    pub industry_confidence: Option<Decimal>,
    #[sea_orm(string_len = 20)]
    pub status: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ppt_page::Entity")]
    PptPage,
    #[sea_orm(
        belongs_to = "super::style_template::Entity",
        from = "Column::StyleTemplateId",
        to = "super::style_template::Column::Id"
    )]
    StyleTemplate,
}

impl Related<super::ppt_page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PptPage.def()
    }
}

impl Related<super::style_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StyleTemplate.def()
    }
}

impl Related<super::ai_generation_log::Entity> for Entity {
    fn to() -> RelationDef {
        super::ai_generation_log::Relation::PptProject.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::ai_generation_log::Relation::PptProject.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
