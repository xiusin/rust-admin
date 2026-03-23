use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_consumer_statistics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub consumer_id: i64,
    pub total_consume: Decimal,
    pub month_consume: Decimal,
    pub year_consume: Decimal,
    pub order_count: i32,
    pub refund_count: i32,
    pub refund_amount: Decimal,
    pub last_order_at: Option<DateTime>,
    pub statistics_month: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_consumer::Entity",
        from = "Column::ConsumerId",
        to = "super::c_consumer::Column::Id"
    )]
    Consumer,
}

impl Related<super::c_consumer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Consumer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
