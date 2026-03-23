//! `SeaORM` Entity for p_shipping_template

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_shipping_template")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub charge_type: i32,
    pub is_free: i32,
    pub free_condition_type: i32,
    pub free_condition_value: Decimal,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_shipping_template_region::Entity")]
    Region,
    #[sea_orm(has_many = "super::p_product::Entity")]
    Product,
}

impl Related<super::p_shipping_template_region::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Region.def()
    }
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
