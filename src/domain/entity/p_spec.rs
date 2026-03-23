//! `SeaORM` Entity for p_spec

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_spec")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub sort: i32,
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
    #[sea_orm(has_many = "super::p_spec_value::Entity")]
    SpecValue,
}

impl Related<super::p_spec_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SpecValue.def()
    }
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
