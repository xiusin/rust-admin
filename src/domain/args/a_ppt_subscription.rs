use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SubscriptionPlanAddArgs {
    #[validate(length(min = 1, max = 100, message = "套餐名称长度1-100"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "套餐编码长度1-50"))]
    pub code: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub original_price: Option<rust_decimal::Decimal>,
    pub duration_days: i32,
    pub max_projects: Option<i32>,
    pub ai_credits: i32,
    pub features: Option<serde_json::Value>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SubscriptionPlanEditArgs {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub original_price: Option<rust_decimal::Decimal>,
    pub duration_days: Option<i32>,
    pub max_projects: Option<i32>,
    pub ai_credits: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub is_active: Option<i8>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubscriptionPlanListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub is_active: Option<i8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubscriptionPlanDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSubscriptionListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SubscribeArgs {
    pub user_id: i64,
    pub plan_id: i64,
    #[validate(length(min = 1, max = 20, message = "支付方式长度1-20"))]
    pub payment_method: String,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CancelSubscriptionArgs {
    pub user_id: i64,
    pub subscription_id: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheckFeatureAccessArgs {
    pub user_id: i64,
    pub feature: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumeCreditArgs {
    pub user_id: i64,
    pub task_type: String,
    pub amount: i32,
    pub project_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RechargeCreditArgs {
    pub user_id: i64,
    pub amount: i32,
    pub source: String,
    pub subscription_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditRecordListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: i64,
    pub task_type: Option<String>,
    pub source: Option<String>,
    pub project_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PaymentOrderCreateArgs {
    pub user_id: i64,
    pub plan_id: i64,
    #[validate(length(min = 1, max = 20, message = "支付方式长度1-20"))]
    pub payment_method: String,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentOrderListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<i64>,
    pub status: Option<String>,
    pub order_no: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentCallbackArgs {
    pub payment_method: String,
    pub callback_data: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RefundArgs {
    pub order_id: i64,
    #[validate(length(min = 1, max = 500, message = "退款原因长度1-500"))]
    pub reason: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct APIKeyCreateArgs {
    pub user_id: i64,
    #[validate(length(min = 1, max = 100, message = "密钥名称长度1-100"))]
    pub name: String,
    pub permissions: Option<Vec<String>>,
    pub rate_limit: Option<i32>,
    pub daily_limit: Option<i32>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIKeyListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<i64>,
    pub is_active: Option<i8>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct APIKeyUpdateArgs {
    pub id: i64,
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub rate_limit: Option<i32>,
    pub daily_limit: Option<i32>,
    pub is_active: Option<i8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIKeyDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIKeyValidateArgs {
    pub api_key: String,
    pub api_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIUsageLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub api_key_id: Option<i64>,
    pub user_id: Option<i64>,
    pub endpoint: Option<String>,
    pub status_code: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIUsageStatsArgs {
    pub api_key_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
