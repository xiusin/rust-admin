use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_verification_code")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub code: String,
    pub order_id: Option<i64>,
    pub order_no: Option<String>,
    pub product_id: Option<i64>,
    pub sku_id: Option<i64>,
    pub qr_code: Option<String>,
    pub total_count: i32,
    pub used_count: i32,
    pub expire_at: Option<DateTime>,
    pub store_id: Option<i64>,
    pub store_name: Option<String>,
    pub status: i32,
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
        belongs_to = "super::p_store::Entity",
        from = "Column::StoreId",
        to = "super::p_store::Column::Id"
    )]
    Store,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::p_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
