use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionListItem {
    pub id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plugin_cover: Option<String>,
    pub plan_id: i64,
    pub plan_name: String,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub auto_renew: i32,
    pub status: i32,
    pub status_name: String,
    pub is_expired: bool,
    pub days_remaining: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionDetail {
    pub id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub auto_renew: i32,
    pub status: i32,
    pub status_name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionStatistics {
    pub total: i64,
    pub active: i64,
    pub expired: i64,
    pub expiring_soon: i64,
}

#[derive(Debug, Deserialize)]
pub struct RenewParams {
    pub subscription_id: i64,
    pub plan_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeParams {
    pub subscription_id: i64,
    pub new_plan_id: i64,
}

pub fn get_subscription_status_name(status: i32) -> String {
    match status {
        0 => "待生效".to_string(),
        1 => "生效中".to_string(),
        2 => "已过期".to_string(),
        3 => "已取消".to_string(),
        _ => "未知".to_string(),
    }
}