use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_payment_order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(string_len = 64)]
    pub order_no: String,
    pub user_id: i64,
    pub plan_id: i64,
    pub subscription_id: Option<i64>,
    pub amount: Decimal,
    pub original_amount: Decimal,
    pub discount_amount: Decimal,
    #[sea_orm(string_len = 20)]
    pub payment_method: String,
    #[sea_orm(string_len = 50, nullable)]
    pub payment_channel: Option<String>,
    #[sea_orm(string_len = 128, nullable)]
    pub transaction_id: Option<String>,
    pub callback_data: Option<String>,
    pub paid_at: Option<DateTime>,
    pub refunded_at: Option<DateTime>,
    pub refund_amount: Option<Decimal>,
    #[sea_orm(string_len = 500, nullable)]
    pub refund_reason: Option<String>,
    #[sea_orm(string_len = 20)]
    pub status: String,
    pub expires_at: DateTime,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_user::Entity",
        from = "Column::UserId",
        to = "super::sys_user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::ppt_subscription_plan::Entity",
        from = "Column::PlanId",
        to = "super::ppt_subscription_plan::Column::Id"
    )]
    Plan,
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::ppt_subscription_plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
