use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationCodeListItem {
    pub id: i64,
    pub order_id: i64,
    pub order_no: String,
    pub product_id: i64,
    pub product_name: String,
    pub product_image: String,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub code: String,
    pub qr_code: Option<String>,
    pub total_count: i32,
    pub used_count: i32,
    pub remain_count: i32,
    pub status: i32,
    pub status_name: String,
    pub expire_at: Option<DateTime>,
    pub verified_at: Option<DateTime>,
    pub store_name: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationCodeDetail {
    pub id: i64,
    pub order_id: i64,
    pub order_no: String,
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub product_image: String,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub code: String,
    pub qr_code: Option<String>,
    pub total_count: i32,
    pub used_count: i32,
    pub remain_count: i32,
    pub status: i32,
    pub expire_at: Option<DateTime>,
    pub verified_at: Option<DateTime>,
    pub verified_by: Option<i64>,
    pub verified_by_name: Option<String>,
    pub store_id: Option<i64>,
    pub store_name: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub verification_logs: Vec<VerificationLogItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationLogItem {
    pub id: i64,
    pub verification_code_id: i64,
    pub code: String,
    pub order_no: String,
    pub product_name: String,
    pub store_id: Option<i64>,
    pub store_name: Option<String>,
    pub verified_by: Option<i64>,
    pub verified_by_name: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
    pub message: String,
    pub code: String,
    pub product_name: String,
    pub spec_text: Option<String>,
    pub order_no: String,
    pub remain_count: i32,
    pub store_name: Option<String>,
    pub verified_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationQueryResult {
    pub code: String,
    pub product_name: String,
    pub product_image: String,
    pub spec_text: Option<String>,
    pub order_no: String,
    pub total_count: i32,
    pub used_count: i32,
    pub remain_count: i32,
    pub status: i32,
    pub status_name: String,
    pub expire_at: Option<DateTime>,
    pub is_valid: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerificationStatistics {
    pub total_codes: i64,
    pub pending_count: i64,
    pub verified_count: i64,
    pub expired_count: i64,
    pub refunded_count: i64,
    pub today_verified_count: i64,
}
