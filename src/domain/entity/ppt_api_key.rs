use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_api_key")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    #[sea_orm(string_len = 100)]
    pub name: String,
    #[sea_orm(string_len = 64)]
    pub api_key: String,
    #[sea_orm(string_len = 128)]
    pub api_secret: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub permissions: Option<Json>,
    pub rate_limit: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub total_requests: i64,
    pub success_requests: i64,
    pub failed_requests: i64,
    pub last_used_at: Option<DateTime>,
    pub last_reset_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub is_active: i8,
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
    #[sea_orm(has_many = "super::ppt_api_usage_log::Entity")]
    UsageLog,
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::ppt_api_usage_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsageLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
