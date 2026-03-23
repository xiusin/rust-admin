//! `SeaORM` Entity for p_product_attribute

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_product_attribute")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub attribute_id: i64,
    pub attribute_name: String,
    pub attribute_value: String,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    Product,
    #[sea_orm(
        belongs_to = "super::p_attribute::Entity",
        from = "Column::AttributeId",
        to = "super::p_attribute::Column::Id"
    )]
    Attribute,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::p_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attribute.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
