use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_consumer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub phone: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub password_hash: String,
    pub wechat_openid: Option<String>,
    pub wechat_unionid: Option<String>,
    pub status: String,
    pub risk_score: i32,
    pub login_fail_count: i32,
    pub locked_until: Option<DateTime>,
    pub last_login_at: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub deactivated_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}