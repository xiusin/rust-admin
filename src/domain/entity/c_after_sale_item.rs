use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_after_sale_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub after_sale_id: i64,
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_id: Option<i64>,
    pub sku_name: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub refund_amount: Decimal,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_after_sale::Entity",
        from = "Column::AfterSaleId",
        to = "super::c_after_sale::Column::Id"
    )]
    AfterSale,
}

impl Related<super::c_after_sale::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AfterSale.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
