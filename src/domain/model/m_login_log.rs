use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, FromQueryResult)]
pub struct LoginLogModel {
    pub id: i64,
    pub tenant_id: i64,
    pub consumer_id: Option<i64>,
    pub phone: Option<String>,
    pub login_type: String,
    pub success: bool,
    pub fail_reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: Option<String>,
    pub login_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginLogSearchParams {
    pub tenant_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}