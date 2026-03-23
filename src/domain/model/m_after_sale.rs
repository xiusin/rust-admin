use crate::model::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AfterSaleType {
    RefundOnly,
    ReturnRefund,
    Exchange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AfterSaleStatus {
    Pending,
    Processing,
    Agreed,
    Rejected,
    Closed,
    Completed,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleModel {
    pub id: i64,
    pub after_sale_no: String,
    pub order_id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub r#type: String,
    pub reason: String,
    pub description: Option<String>,
    pub evidence_urls: Option<serde_json::Value>,
    pub refund_amount: String,
    pub status: String,
    pub apply_at: DateTime,
    pub process_at: Option<DateTime>,
    pub complete_at: Option<DateTime>,
    pub close_at: Option<DateTime>,
    pub reject_reason: Option<String>,
    pub processor_id: Option<i64>,
    pub processor_name: Option<String>,
    pub timeout_at: Option<DateTime>,
    pub is_timeout: bool,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleItemModel {
    pub id: i64,
    pub after_sale_id: i64,
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_id: Option<i64>,
    pub sku_name: Option<String>,
    pub quantity: i32,
    pub unit_price: String,
    pub refund_amount: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleRefundModel {
    pub id: i64,
    pub after_sale_id: i64,
    pub refund_no: String,
    pub transaction_id: Option<String>,
    pub refund_channel: String,
    pub refund_amount: String,
    pub status: String,
    pub refund_at: Option<DateTime>,
    pub fail_reason: Option<String>,
    pub retry_count: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleLogisticsModel {
    pub id: i64,
    pub after_sale_id: i64,
    pub logistics_type: String,
    pub logistics_company: Option<String>,
    pub tracking_no: Option<String>,
    pub sender_name: Option<String>,
    pub sender_phone: Option<String>,
    pub sender_address: Option<String>,
    pub receiver_name: Option<String>,
    pub receiver_phone: Option<String>,
    pub receiver_address: Option<String>,
    pub status: String,
    pub shipped_at: Option<DateTime>,
    pub received_at: Option<DateTime>,
    pub tracking_info: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApplyAfterSaleParams {
    pub order_id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub r#type: AfterSaleType,
    pub reason: String,
    pub description: Option<String>,
    pub evidence_urls: Option<Vec<String>>,
    pub items: Vec<AfterSaleItemParams>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AfterSaleItemParams {
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_id: Option<i64>,
    pub sku_name: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub refund_amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuditAfterSaleParams {
    pub after_sale_id: i64,
    pub agree: bool,
    pub reject_reason: Option<String>,
    pub processor_id: i64,
    pub processor_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubmitLogisticsParams {
    pub after_sale_id: i64,
    pub logistics_company: String,
    pub tracking_no: String,
    pub sender_name: Option<String>,
    pub sender_phone: Option<String>,
    pub sender_address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfirmReceiveParams {
    pub after_sale_id: i64,
    pub operator_id: i64,
    pub operator_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateRefundParams {
    pub after_sale_id: i64,
    pub refund_channel: String,
    pub transaction_id: Option<String>,
    pub refund_amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AfterSaleListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub order_id: Option<i64>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefundCallbackParams {
    pub refund_no: String,
    pub status: String,
    pub transaction_id: Option<String>,
    pub callback_data: Option<String>,
    pub fail_reason: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleDetailModel {
    pub after_sale: AfterSaleModel,
    pub items: Vec<AfterSaleItemModel>,
    pub refunds: Vec<AfterSaleRefundModel>,
    pub logistics: Option<AfterSaleLogisticsModel>,
    pub status_logs: Vec<AfterSaleStatusLogModel>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleStatusLogModel {
    pub id: i64,
    pub after_sale_id: i64,
    pub old_status: Option<String>,
    pub new_status: String,
    pub operator_type: String,
    pub operator_id: Option<i64>,
    pub operator_name: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TimeoutConfigModel {
    pub stage: String,
    pub timeout_hours: i32,
    pub auto_action: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AfterSaleStatistics {
    pub total_count: u64,
    pub pending_count: u64,
    pub processing_count: u64,
    pub completed_count: u64,
    pub total_refund_amount: String,
}
