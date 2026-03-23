//! `SeaORM` Entity for p_product_category

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_product_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub category_id: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    PProduct,
    #[sea_orm(
        belongs_to = "super::p_category::Entity",
        from = "Column::CategoryId",
        to = "super::p_category::Column::Id"
    )]
    PCategory,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PProduct.def()
    }
}

impl Related<super::p_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
