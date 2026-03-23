//! `SeaORM` Entity for p_category

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: i32,
    pub level: i32,
    pub path: String,
    pub status: String,
    pub show_in_nav: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_product::Entity")]
    Product,
    #[sea_orm(has_many = "super::p_product_category::Entity")]
    ProductCategory,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        super::p_product_category::Relation::PProduct.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::p_product_category::Relation::PCategory.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
