use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CardItem {
    pub id: i64,
    pub card_no: String,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub batch_id: i64,
    pub face_value: f64,
    pub status: i32,
    pub status_name: String,
    pub used_user_id: Option<i64>,
    pub used_time: Option<DateTime>,
    pub expire_time: DateTime,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CardBatchItem {
    pub id: i64,
    pub batch_no: String,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plan_id: i64,
    pub plan_name: String,
    pub total_count: i32,
    pub used_count: i32,
    pub remaining_count: i32,
    pub price: f64,
    pub expire_time: DateTime,
    pub status: i32,
    pub status_name: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CardDetail {
    pub id: i64,
    pub card_no: String,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub batch_id: i64,
    pub face_value: f64,
    pub status: i32,
    pub status_name: String,
    pub used_user_id: Option<i64>,
    pub used_order_id: Option<i64>,
    pub used_time: Option<DateTime>,
    pub expire_time: DateTime,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateCardsParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub count: i32,
    pub price: f64,
    pub expire_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct RedeemCardParams {
    pub card_no: String,
    pub card_pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedeemResult {
    pub success: bool,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub plugin_name: String,
    pub plan_name: String,
    pub expire_time: Option<DateTime>,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CardExportData {
    pub card_no: String,
    pub card_pwd: String,
    pub plugin_name: String,
    pub plan_name: String,
    pub expire_time: String,
}

pub fn get_card_status_name(status: i32) -> String {
    match status {
        0 => "未使用".to_string(),
        1 => "已使用".to_string(),
        2 => "已过期".to_string(),
        3 => "已冻结".to_string(),
        _ => "未知".to_string(),
    }
}