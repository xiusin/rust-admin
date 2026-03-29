//! `SeaORM` Entity for ppt_template_tag

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_template_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(string_len = 50, unique)]
    pub name: String,
    #[sea_orm(string_len = 200, nullable)]
    pub description: Option<String>,
    pub usage_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ppt_template_tag_relation::Entity")]
    TemplateTags,
}

impl Related<super::ppt_template_tag_relation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TemplateTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
