use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogisticsStatus {
    Pending,
    InTransit,
    Delivering,
    Delivered,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogisticsTrackingModel {
    pub id: i64,
    pub tracking_no: String,
    pub courier_company: Option<String>,
    pub status: String,
    pub traces: Vec<LogisticsTraceResp>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryLogisticsParams {
    pub tracking_no: String,
    pub courier_company: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubscribeLogisticsParams {
    pub order_no: String,
    pub tracking_no: String,
    pub courier_company: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogisticsHistoryParams {
    pub tracking_no: String,
    pub courier_company: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LogisticsDetailResp {
    pub tracking_no: String,
    pub courier_company: String,
    pub status: String,
    pub traces: Vec<LogisticsTraceResp>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LogisticsNode {
    pub status: String,
    pub location: String,
    pub description: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LogisticsTraceResp {
    pub tracking_no: String,
    pub courier_company: String,
    pub nodes: Vec<LogisticsNode>,
}