//! `SeaORM` Entity for page_element

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "page_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub page_id: i64,
    #[sea_orm(string_len = 20)]
    pub element_type: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub content: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub style: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub position: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub size: Option<Json>,
    pub rotation: Option<Decimal>,
    pub z_index: i32,
    pub locked: i8,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ppt_page::Entity",
        from = "Column::PageId",
        to = "super::ppt_page::Column::Id"
    )]
    PptPage,
}

impl Related<super::ppt_page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PptPage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
