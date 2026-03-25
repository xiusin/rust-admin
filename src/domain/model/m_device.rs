use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeviceListItem {
    pub id: i64,
    pub user_id: i64,
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub last_active_time: Option<DateTime>,
    pub status: i32,
    pub status_name: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeviceDetail {
    pub id: i64,
    pub user_id: i64,
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub last_active_time: Option<DateTime>,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceParams {
    pub license_id: i64,
    pub device_id: String,
    pub device_info: DeviceInfo,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeviceStatusParams {
    pub device_id: String,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
}

pub fn get_device_status_name(status: i32) -> String {
    match status {
        0 => "离线".to_string(),
        1 => "在线".to_string(),
        _ => "未知".to_string(),
    }
}