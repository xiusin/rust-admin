use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CreatePaymentArgs {
    pub consumer_id: i64,
    #[validate(length(min = 1, message = "支付方式不能为空"))]
    pub payment_method: String,
    #[validate(length(min = 1, message = "支付类型不能为空"))]
    pub payment_type: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentCallbackArgs {
    pub order_no: Option<String>,
    pub transaction_id: Option<String>,
    pub status: Option<String>,
    pub paid_amount: Option<f64>,
    pub callback_data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RefundArgs {
    #[validate(length(min = 1, message = "订单号不能为空"))]
    pub order_no: String,
    pub refund_amount: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloseOrderArgs {
    pub order_no: String,
}
