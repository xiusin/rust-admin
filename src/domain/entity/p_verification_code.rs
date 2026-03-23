//! `SeaORM` Entity for p_verification_code

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_verification_code")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub order_id: i64,
    pub order_no: String,
    pub order_item_id: i64,
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub code: String,
    pub qr_code: Option<String>,
    pub total_count: i32,
    pub used_count: i32,
    pub status: i32,
    pub expire_at: Option<DateTime>,
    pub verified_at: Option<DateTime>,
    pub verified_by: Option<i64>,
    pub verified_by_name: Option<String>,
    pub store_id: Option<i64>,
    pub store_name: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_verification_log::Entity")]
    VerificationLog,
}

impl Related<super::p_verification_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VerificationLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
