use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_subscription_plan")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(string_len = 100)]
    pub name: String,
    #[sea_orm(string_len = 50)]
    pub code: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub original_price: Option<Decimal>,
    pub duration_days: i32,
    pub max_projects: i32,
    pub ai_credits: i32,
    #[sea_orm(column_type = "Json", nullable)]
    pub features: Option<Json>,
    pub is_active: i8,
    pub sort_order: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ppt_user_subscription::Entity")]
    UserSubscription,
    #[sea_orm(has_many = "super::ppt_payment_order::Entity")]
    PaymentOrder,
}

impl Related<super::ppt_user_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSubscription.def()
    }
}

impl Related<super::ppt_payment_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PaymentOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
