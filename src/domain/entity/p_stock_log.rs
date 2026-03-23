//! `SeaORM` Entity for p_stock_log

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_stock_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub change_type: i32,
    pub change_num: i32,
    pub before_stock: i32,
    pub after_stock: i32,
    pub order_no: Option<String>,
    pub remark: Option<String>,
    pub operator_id: Option<i64>,
    pub operator_name: Option<String>,
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
        belongs_to = "super::p_sku::Entity",
        from = "Column::SkuId",
        to = "super::p_sku::Column::Id"
    )]
    Sku,
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
