use crate::model::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Wechat,
    Alipay,
    Yeepay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    App,
    H5,
    Mini,
    Qrcode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
    Closed,
    Refunded,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PaymentOrderModel {
    pub id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub payment_method: String,
    pub payment_type: String,
    pub amount: Decimal,
    pub status: String,
    pub transaction_id: Option<String>,
    pub callback_data: Option<String>,
    pub paid_at: Option<DateTime>,
    pub closed_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePaymentParams {
    pub consumer_id: i64,
    pub payment_method: PaymentMethod,
    pub payment_type: PaymentType,
    pub amount: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefundParams {
    pub order_no: String,
    pub refund_amount: Decimal,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentOrderSearchParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PaymentStatistics {
    pub total_amount: Decimal,
    pub success_amount: Decimal,
    pub pending_amount: Decimal,
    pub refunded_amount: Decimal,
    pub success_count: u64,
    pub pending_count: u64,
    pub failed_count: u64,
}
