use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_user_subscription")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub started_at: DateTime,
    pub expires_at: DateTime,
    pub ai_credits_total: i32,
    pub ai_credits_used: i32,
    pub ai_credits_remaining: i32,
    pub projects_count: i32,
    pub auto_renew: i8,
    #[sea_orm(string_len = 20)]
    pub status: String,
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
    #[sea_orm(has_many = "super::ppt_credit_record::Entity")]
    CreditRecord,
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

impl Related<super::ppt_credit_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreditRecord.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
