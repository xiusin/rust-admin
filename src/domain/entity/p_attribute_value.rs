//! `SeaORM` Entity for p_attribute_value

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_attribute_value")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub attribute_id: i64,
    pub value: String,
    pub sort: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_attribute::Entity",
        from = "Column::AttributeId",
        to = "super::p_attribute::Column::Id"
    )]
    Attribute,
}

impl Related<super::p_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attribute.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
