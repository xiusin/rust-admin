use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_card")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub card_no: String,
    pub card_pwd: String,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub batch_id: i64,
    pub face_value: Decimal,
    pub status: i32,
    pub used_user_id: Option<i64>,
    pub used_order_id: Option<i64>,
    pub used_time: Option<DateTime>,
    pub expire_time: DateTime,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_card_batch::Entity",
        from = "Column::BatchId",
        to = "super::p_card_batch::Column::Id"
    )]
    CardBatch,
}

impl Related<super::p_card_batch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardBatch.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}