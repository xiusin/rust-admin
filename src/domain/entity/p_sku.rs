//! `SeaORM` Entity for p_sku

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_sku")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub sku_code: String,
    pub spec_value_ids: String,
    pub spec_text: String,
    pub image: Option<String>,
    pub sale_price: Decimal,
    pub line_price: Decimal,
    pub cost_price: Decimal,
    pub stock: i32,
    pub sales: i32,
    pub weight: Decimal,
    pub volume: Decimal,
    pub status: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    Product,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
