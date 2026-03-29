//! `SeaORM` Entity for ppt_page

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_page")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub project_id: i64,
    pub page_order: i32,
    #[sea_orm(string_len = 20)]
    pub page_type: String,
    #[sea_orm(string_len = 200, nullable)]
    pub title: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub layout_config: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub style_config: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ppt_project::Entity",
        from = "Column::ProjectId",
        to = "super::ppt_project::Column::Id"
    )]
    PptProject,
    #[sea_orm(has_many = "super::page_element::Entity")]
    PageElement,
}

impl Related<super::ppt_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PptProject.def()
    }
}

impl Related<super::page_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
