use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_after_sale")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub after_sale_no: String,
    pub order_id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub r#type: String,
    pub reason: String,
    pub description: Option<String>,
    pub evidence_urls: Option<Json>,
    pub refund_amount: Decimal,
    pub status: String,
    pub apply_at: DateTime,
    pub process_at: Option<DateTime>,
    pub complete_at: Option<DateTime>,
    pub close_at: Option<DateTime>,
    pub reject_reason: Option<String>,
    pub processor_id: Option<i64>,
    pub processor_name: Option<String>,
    pub timeout_at: Option<DateTime>,
    pub is_timeout: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::c_after_sale_item::Entity")]
    Items,
    #[sea_orm(has_many = "super::c_after_sale_refund::Entity")]
    Refunds,
    #[sea_orm(has_many = "super::c_after_sale_logistics::Entity")]
    Logistics,
    #[sea_orm(has_many = "super::c_after_sale_status_log::Entity")]
    StatusLogs,
}

impl Related<super::c_after_sale_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl Related<super::c_after_sale_refund::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Refunds.def()
    }
}

impl Related<super::c_after_sale_logistics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logistics.def()
    }
}

impl Related<super::c_after_sale_status_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatusLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
