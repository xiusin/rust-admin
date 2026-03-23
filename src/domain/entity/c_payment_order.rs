use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_payment_order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub payment_method: String,
    pub payment_type: String,
    pub amount: Decimal,
    pub status: String,
    pub transaction_id: Option<String>,
    pub callback_data: Option<String>,
    pub paid_at: Option<DateTime>,
    pub closed_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_consumer::Entity",
        from = "Column::ConsumerId",
        to = "super::c_consumer::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    CConsumer,
}

impl Related<super::c_consumer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CConsumer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}