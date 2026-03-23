//! `SeaORM` Entity for p_store_stock

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_store_stock")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub store_id: i64,
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub stock: i32,
    pub alert_stock: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_store::Entity",
        from = "Column::StoreId",
        to = "super::p_store::Column::Id"
    )]
    Store,
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    Product,
    #[sea_orm(
        belongs_to = "super::p_sku::Entity",
        from = "Column::SkuId",
        to = "super::p_sku::Column::Id"
    )]
    Sku,
}

impl Related<super::p_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::p_sku::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sku.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
