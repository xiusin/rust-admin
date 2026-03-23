use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthType {
    Wechat,
    Github,
    QQ,
    Weibo,
    Apple,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserOauthModel {
    pub id: i64,
    pub consumer_id: i64,
    pub oauth_type: String,
    pub oauth_id: String,
    pub oauth_name: Option<String>,
    pub oauth_avatar: Option<String>,
    pub bind_at: DateTime,
    pub unbind_at: Option<DateTime>,
    pub is_primary: bool,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BindOauthParams {
    pub consumer_id: i64,
    pub oauth_type: OAuthType,
    pub oauth_id: String,
    pub oauth_name: Option<String>,
    pub oauth_avatar: Option<String>,
    pub oauth_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnbindOauthParams {
    pub consumer_id: i64,
    pub oauth_type: OAuthType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OauthListParams {
    pub consumer_id: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserLevelModel {
    pub id: i64,
    pub consumer_id: i64,
    pub level: i32,
    pub level_name: String,
    pub exp: i32,
    pub total_exp: i64,
    pub next_level_exp: Option<i32>,
    pub discount_rate: String,
    pub level_up_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LevelConfigModel {
    pub id: i64,
    pub level: i32,
    pub level_name: String,
    pub min_exp: i32,
    pub max_exp: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub discount_rate: String,
    pub privileges: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddExpParams {
    pub consumer_id: i64,
    pub exp: i32,
    pub source: String,
    pub source_id: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LevelRecordModel {
    pub id: i64,
    pub consumer_id: i64,
    pub old_level: Option<i32>,
    pub new_level: i32,
    pub old_exp: Option<i32>,
    pub new_exp: i32,
    pub exp_change: i32,
    pub change_type: String,
    pub source: Option<String>,
    pub source_id: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LevelRecordListParams {
    pub consumer_id: i64,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserTagModel {
    pub id: i64,
    pub name: String,
    pub tag_type: String,
    pub category: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTagParams {
    pub name: String,
    pub tag_type: String,
    pub category: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddUserTagParams {
    pub consumer_id: i64,
    pub tag_ids: Vec<i64>,
    pub source: Option<String>,
    pub source_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoveUserTagParams {
    pub consumer_id: i64,
    pub tag_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserTagListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub tag_type: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserBanModel {
    pub id: i64,
    pub consumer_id: i64,
    pub ban_type: String,
    pub reason: String,
    pub start_at: DateTime,
    pub end_at: Option<DateTime>,
    pub banned_by: Option<i64>,
    pub unban_at: Option<DateTime>,
    pub unban_by: Option<i64>,
    pub unban_reason: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BanUserParams {
    pub consumer_id: i64,
    pub ban_type: String,
    pub reason: String,
    pub end_at: Option<DateTime>,
    pub banned_by: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnbanUserParams {
    pub consumer_id: i64,
    pub unban_reason: Option<String>,
    pub unban_by: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BanListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConsumerStatisticsModel {
    pub consumer_id: i64,
    pub total_consume: String,
    pub month_consume: String,
    pub year_consume: String,
    pub order_count: i32,
    pub refund_count: i32,
    pub refund_amount: String,
    pub last_order_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStatisticsParams {
    pub consumer_id: i64,
    pub order_amount: Option<String>,
    pub is_refund: bool,
    pub refund_amount: Option<String>,
}
