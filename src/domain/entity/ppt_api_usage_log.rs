use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_api_usage_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub api_key_id: i64,
    pub user_id: i64,
    #[sea_orm(string_len = 200)]
    pub endpoint: String,
    #[sea_orm(string_len = 10)]
    pub method: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub tokens_used: i32,
    pub error_message: Option<String>,
    #[sea_orm(string_len = 50, nullable)]
    pub ip_address: Option<String>,
    #[sea_orm(string_len = 500, nullable)]
    pub user_agent: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ppt_api_key::Entity",
        from = "Column::ApiKeyId",
        to = "super::ppt_api_key::Column::Id"
    )]
    ApiKey,
    #[sea_orm(
        belongs_to = "super::sys_user::Entity",
        from = "Column::UserId",
        to = "super::sys_user::Column::Id"
    )]
    User,
}

impl Related<super::ppt_api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKey.def()
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
