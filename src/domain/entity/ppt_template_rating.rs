//! `SeaORM` Entity for ppt_template_rating

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_template_rating")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub template_id: i64,
    pub user_id: i64,
    #[sea_orm(string_len = 50)]
    pub user_name: String,
    pub rating: i32,
    #[sea_orm(string_len = 500, nullable)]
    pub comment: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ppt_template_market::Entity",
        from = "Column::TemplateId",
        to = "super::ppt_template_market::Column::Id"
    )]
    Template,
}

impl Related<super::ppt_template_market::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Template.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
