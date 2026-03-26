use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UnbindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyLicenseParams {
    pub license_key: String,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub mac_address: Option<String>,
    pub timestamp: i64,
    pub sign: String,
}

#[derive(Debug, Deserialize)]
pub struct HeartbeatParams {
    pub license_key: String,
    pub device_id: String,
    pub timestamp: i64,
    pub sign: String,
}

#[derive(Debug, Deserialize)]
pub struct RenewLicenseParams {
    pub license_id: i64,
    pub extend_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct RevokeLicenseParams {
    pub license_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LicenseSearchParams {
    pub plugin_id: Option<i64>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceSearchParams {
    pub license_id: Option<i64>,
    pub status: Option<i32>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SendVerifyCodeParams {
    pub license_id: i64,
    pub plugin_id: i64,
    pub purpose: i32,
    pub device_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckVerifyCodeParams {
    pub license_id: i64,
    pub code: String,
    pub device_hash: Option<String>,
}