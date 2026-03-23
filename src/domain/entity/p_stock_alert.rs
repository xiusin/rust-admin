//! `SeaORM` Entity for p_stock_alert

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_stock_alert")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub alert_stock: i32,
    pub is_alert: i32,
    pub last_alert_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
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
