use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct OrderListItem {
    pub id: i64,
    pub order_no: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plugin_cover: Option<String>,
    pub plan_id: i64,
    pub plan_name: String,
    pub amount: f64,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub payment_method: i32,
    pub payment_method_name: String,
    pub payment_time: Option<DateTime>,
    pub status: i32,
    pub status_name: String,
    pub expire_time: DateTime,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct OrderDetail {
    pub id: i64,
    pub order_no: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub plugin_id: i64,
    pub plugin_info: PluginSimple,
    pub plan_id: i64,
    pub plan_info: PlanSimple,
    pub amount: f64,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub coupon_id: Option<i64>,
    pub payment_method: i32,
    pub payment_method_name: String,
    pub payment_time: Option<DateTime>,
    pub status: i32,
    pub status_name: String,
    pub expire_time: DateTime,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PluginSimple {
    pub id: i64,
    pub name: String,
    pub cover_image: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PlanSimple {
    pub id: i64,
    pub name: String,
    pub period_type_name: String,
    pub period_days: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct OrderStatistics {
    pub total: i64,
    pub paid: i64,
    pub pending: i64,
    pub cancelled: i64,
    pub refunded: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub coupon_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PayParams {
    pub order_id: i64,
    pub payment_method: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PayResult {
    pub order_no: String,
    pub amount: f64,
    pub qr_code: Option<String>,
    pub pay_url: Option<String>,
}

pub fn get_payment_method_name(method: i32) -> String {
    match method {
        0 => "微信支付".to_string(),
        1 => "支付宝".to_string(),
        2 => "卡密支付".to_string(),
        3 => "余额支付".to_string(),
        _ => "未知".to_string(),
    }
}

pub fn get_order_status_name(status: i32) -> String {
    match status {
        0 => "待支付".to_string(),
        1 => "已支付".to_string(),
        2 => "已取消".to_string(),
        3 => "已退款".to_string(),
        4 => "已过期".to_string(),
        _ => "未知".to_string(),
    }
}