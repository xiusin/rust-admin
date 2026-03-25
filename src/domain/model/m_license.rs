use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LicenseListItem {
    pub id: i64,
    pub license_key: String,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plan_id: i64,
    pub plan_name: String,
    pub device_id: String,
    pub status: i32,
    pub status_name: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub is_expired: bool,
    pub days_remaining: i32,
    pub verify_count: i32,
    pub max_devices: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LicenseDetail {
    pub id: i64,
    pub license_key: String,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub ip_address: Option<String>,
    pub bind_count: i32,
    pub max_devices: i32,
    pub verify_count: i32,
    pub last_verify_time: Option<DateTime>,
    pub status: i32,
    pub status_name: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub created_at: Option<DateTime>,
    pub devices: Vec<DeviceItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeviceItem {
    pub id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub last_active_time: Option<DateTime>,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct LicenseVerifyRequest {
    pub license_key: String,
    pub device_id: String,
    pub device_info: DeviceInfo,
    pub timestamp: i64,
    pub sign: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_name: String,
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub mac_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct BindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
    pub device_info: DeviceInfo,
}

#[derive(Debug, Deserialize)]
pub struct UnbindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RenewLicenseParams {
    pub license_id: i64,
    pub extend_days: i32,
}

pub fn get_license_status_name(status: i32) -> String {
    match status {
        0 => "禁用".to_string(),
        1 => "启用".to_string(),
        2 => "过期".to_string(),
        3 => "注销".to_string(),
        _ => "未知".to_string(),
    }
}