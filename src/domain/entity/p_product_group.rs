//! `SeaORM` Entity for p_product_group

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_product_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_product_group_relation::Entity")]
    ProductGroupRelation,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        super::p_product_group_relation::Relation::PProduct.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::p_product_group_relation::Relation::PProductGroup.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
