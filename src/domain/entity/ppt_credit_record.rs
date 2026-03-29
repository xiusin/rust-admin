use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_credit_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub subscription_id: Option<i64>,
    pub project_id: Option<i64>,
    #[sea_orm(string_len = 50)]
    pub task_type: String,
    pub amount: i32,
    pub balance_before: i32,
    pub balance_after: i32,
    #[sea_orm(string_len = 20)]
    pub source: String,
    #[sea_orm(string_len = 500, nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
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
        belongs_to = "super::ppt_user_subscription::Entity",
        from = "Column::SubscriptionId",
        to = "super::ppt_user_subscription::Column::Id"
    )]
    Subscription,
    #[sea_orm(
        belongs_to = "super::ppt_project::Entity",
        from = "Column::ProjectId",
        to = "super::ppt_project::Column::Id"
    )]
    Project,
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::ppt_user_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscription.def()
    }
}

impl Related<super::ppt_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
