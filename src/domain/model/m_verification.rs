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