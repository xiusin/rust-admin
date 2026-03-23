use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SMSType {
    Verification,
    Notification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SMSChannel {
    Aliyun,
    Tencent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SMSStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SMSLogModel {
    pub id: i64,
    pub phone: String,
    pub sms_type: String,
    pub content: Option<String>,
    pub code: Option<String>,
    pub channel: String,
    pub status: String,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SendCodeParams {
    pub phone: String,
    pub sms_type: SMSType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerifyCodeParams {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SMSLogSearchParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SMSStatistics {
    pub total_sent: u64,
    pub total_success: u64,
    pub total_failed: u64,
    pub verification_count: u64,
    pub notification_count: u64,
}
