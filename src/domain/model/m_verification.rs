use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct SendCodeParams {
    pub license_id: i64,
    pub plugin_id: i64,
    pub purpose: i32,
    pub device_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckCodeParams {
    pub license_id: i64,
    pub code: String,
    pub device_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyCodeResponse {
    pub success: bool,
    pub license_key: Option<String>,
    pub expire_time: Option<DateTime>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObfuscationConfig {
    pub plugin_id: i64,
    pub version: String,
    pub obfuscation_level: i32,
    pub enabled_modules: Vec<String>,
    pub license_check_interval: i64,
    pub max_offline_days: i32,
    pub anti_debug_enabled: bool,
    pub code_virtualization: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub valid: bool,
    pub next_check_time: i64,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseVerifyResponse {
    pub valid: bool,
    pub license_key: String,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plan_id: i64,
    pub plan_name: String,
    pub expire_time: DateTime,
    pub max_devices: i32,
    pub features: Vec<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationCodeListItem {
    pub id: i64,
    pub order_id: Option<i64>,
    pub order_no: Option<String>,
    pub product_id: Option<i64>,
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
    pub store_name: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationResult {
    pub success: bool,
    pub message: String,
    pub code: String,
    pub product_name: String,
    pub spec_text: Option<String>,
    pub order_no: Option<String>,
    pub remain_count: i32,
    pub store_name: Option<String>,
    pub verified_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationQueryResult {
    pub code: String,
    pub product_name: String,
    pub product_image: String,
    pub spec_text: Option<String>,
    pub order_no: Option<String>,
    pub total_count: i32,
    pub used_count: i32,
    pub remain_count: i32,
    pub status: i32,
    pub status_name: String,
    pub expire_at: Option<DateTime>,
    pub is_valid: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationStatistics {
    pub total_codes: i64,
    pub pending_count: i64,
    pub verified_count: i64,
    pub expired_count: i64,
    pub refunded_count: i64,
    pub today_verified_count: i64,
}