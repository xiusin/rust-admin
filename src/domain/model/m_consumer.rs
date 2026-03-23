use crate::model::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerStatus {
    Normal,
    Locked,
    Deactivated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoginType {
    Phone,
    Wechat,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerRegisterParams {
    pub phone: String,
    pub password: String,
    pub sms_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerLoginParams {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerWechatLoginParams {
    pub wechat_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerUpdateParams {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConsumerInfo {
    pub id: i64,
    pub phone: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: ConsumerStatus,
    pub risk_score: i32,
    pub wechat_openid: Option<String>,
    pub wechat_unionid: Option<String>,
    pub last_login_at: Option<DateTime>,
    pub last_login_ip: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConsumerListItem {
    pub id: i64,
    pub phone: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: String,
    pub risk_score: i32,
    pub wechat_openid: Option<String>,
    pub wechat_unionid: Option<String>,
    pub last_login_at: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_consumption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UserTagBrief>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserTagBrief {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConsumerResp {
    pub id: i64,
    pub phone: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: ConsumerStatus,
    pub risk_score: i32,
    pub wechat_openid: Option<String>,
    pub wechat_unionid: Option<String>,
    pub last_login_at: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LoginLogResp {
    pub id: i64,
    pub consumer_id: Option<i64>,
    pub phone: Option<String>,
    pub login_type: String,
    pub success: bool,
    pub fail_reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: Option<String>,
    pub login_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginLogAddParams {
    pub consumer_id: Option<i64>,
    pub phone: String,
    pub login_type: String,
    pub success: bool,
    pub fail_reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: Option<String>,
}