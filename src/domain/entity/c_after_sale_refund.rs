use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_after_sale_refund")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub after_sale_id: i64,
    pub refund_no: String,
    pub transaction_id: Option<String>,
    pub refund_channel: String,
    pub refund_amount: Decimal,
    pub status: String,
    pub refund_at: Option<DateTime>,
    pub fail_reason: Option<String>,
    pub retry_count: i32,
    pub next_retry_at: Option<DateTime>,
    pub callback_data: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
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
