use crate::model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionPlanListItem {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub original_price: Option<Decimal>,
    pub duration_days: i32,
    pub max_projects: i32,
    pub ai_credits: i32,
    pub features: Option<Json>,
    pub is_active: i8,
    pub sort_order: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionPlanDetail {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub original_price: Option<Decimal>,
    pub duration_days: i32,
    pub max_projects: i32,
    pub ai_credits: i32,
    pub features: Option<Json>,
    pub is_active: i8,
    pub sort_order: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserSubscriptionInfo {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub plan_name: String,
    pub plan_code: String,
    pub order_id: Option<i64>,
    pub started_at: DateTime,
    pub expires_at: DateTime,
    pub ai_credits_total: i32,
    pub ai_credits_used: i32,
    pub ai_credits_remaining: i32,
    pub projects_count: i32,
    pub max_projects: i32,
    pub auto_renew: i8,
    pub status: String,
    pub is_active: bool,
    pub days_remaining: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreditBalance {
    pub total: i32,
    pub used: i32,
    pub remaining: i32,
    pub reset_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreditUsageRecord {
    pub id: i64,
    pub user_id: i64,
    pub subscription_id: Option<i64>,
    pub project_id: Option<i64>,
    pub project_title: Option<String>,
    pub task_type: String,
    pub amount: i32,
    pub balance_before: i32,
    pub balance_after: i32,
    pub source: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PaymentOrderInfo {
    pub id: i64,
    pub order_no: String,
    pub user_id: i64,
    pub plan_id: i64,
    pub plan_name: String,
    pub subscription_id: Option<i64>,
    pub amount: Decimal,
    pub original_amount: Decimal,
    pub discount_amount: Decimal,
    pub payment_method: String,
    pub payment_channel: Option<String>,
    pub transaction_id: Option<String>,
    pub paid_at: Option<DateTime>,
    pub refunded_at: Option<DateTime>,
    pub refund_amount: Option<Decimal>,
    pub refund_reason: Option<String>,
    pub status: String,
    pub expires_at: DateTime,
    pub payment_url: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PaymentOrderListItem {
    pub id: i64,
    pub order_no: String,
    pub user_id: i64,
    pub plan_id: i64,
    pub plan_name: String,
    pub amount: Decimal,
    pub payment_method: String,
    pub status: String,
    pub paid_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionResult {
    pub success: bool,
    pub subscription: Option<UserSubscriptionInfo>,
    pub order: Option<PaymentOrderInfo>,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct APIKeyInfo {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub permissions: Option<Json>,
    pub rate_limit: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub total_requests: i64,
    pub success_requests: i64,
    pub failed_requests: i64,
    pub last_used_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub is_active: i8,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct APIKeyListItem {
    pub id: i64,
    pub name: String,
    pub api_key_prefix: String,
    pub permissions: Option<Json>,
    pub rate_limit: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub total_requests: i64,
    pub last_used_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub is_active: i8,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct APIKeyValidation {
    pub valid: bool,
    pub user_id: Option<i64>,
    pub key_id: Option<i64>,
    pub permissions: Option<Vec<String>>,
    pub rate_limit: Option<i32>,
    pub daily_limit: Option<i32>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct APIUsageStats {
    pub total_requests: u64,
    pub success_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time: f32,
    pub success_rate: f32,
    pub daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DailyUsage {
    pub date: String,
    pub requests: u64,
    pub tokens_used: u64,
    pub avg_response_time: f32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct APIUsageLogItem {
    pub id: i64,
    pub api_key_id: i64,
    pub api_key_name: String,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub tokens_used: i32,
    pub error_message: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FeatureAccess {
    pub feature: String,
    pub has_access: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SubscriptionStatistics {
    pub total_subscriptions: i64,
    pub active_subscriptions: i64,
    pub expired_subscriptions: i64,
    pub cancelled_subscriptions: i64,
    pub total_revenue: Decimal,
    pub monthly_revenue: Decimal,
    pub plan_distribution: HashMap<String, i64>,
}
