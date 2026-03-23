//! `SeaORM` Entity for p_verification_log

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_verification_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub verification_code_id: i64,
    pub code: String,
    pub order_no: String,
    pub product_name: String,
    pub store_id: Option<i64>,
    pub store_name: Option<String>,
    pub verified_by: Option<i64>,
    pub verified_by_name: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_verification_code::Entity",
        from = "Column::VerificationCodeId",
        to = "super::p_verification_code::Column::Id"
    )]
    VerificationCode,
    #[sea_orm(
        belongs_to = "super::p_store::Entity",
        from = "Column::StoreId",
        to = "super::p_store::Column::Id"
    )]
    Store,
}

impl Related<super::p_verification_code::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VerificationCode.def()
    }
}

impl Related<super::p_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
