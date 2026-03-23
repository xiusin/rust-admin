use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RechargeArgs {
    pub amount: f64,
    pub payment_order_no: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct WithdrawArgs {
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct WithdrawApproveArgs {
    pub transaction_id: i64,
    pub approve: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub transaction_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub consumer_id: Option<i64>,
}