use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_card_batch")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub batch_no: String,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub total_count: i32,
    pub used_count: i32,
    pub price: Decimal,
    pub expire_time: DateTime,
    pub creator_id: i64,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_card::Entity")]
    Card,
}

impl Related<super::p_card::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Card.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}