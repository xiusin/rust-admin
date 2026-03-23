//! `SeaORM` Entity for p_attribute_template

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_attribute_template")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub category_id: Option<i64>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_category::Entity",
        from = "Column::CategoryId",
        to = "super::p_category::Column::Id"
    )]
    Category,
    #[sea_orm(has_many = "super::p_attribute::Entity")]
    Attribute,
}

impl Related<super::p_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attribute.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
