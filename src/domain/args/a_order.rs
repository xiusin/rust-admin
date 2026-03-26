use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub coupon_id: Option<i64>,
    pub payment_method: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderFromCartParams {
    pub cart_ids: Vec<i64>,
    pub coupon_id: Option<i64>,
    pub payment_method: i32,
}

#[derive(Debug, Deserialize)]
pub struct PayParams {
    pub order_id: i64,
    pub payment_method: i32,
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderParams {
    pub order_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RefundParams {
    pub order_id: i64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderSearchParams {
    pub status: Option<i32>,
    pub payment_method: Option<i32>,
    pub keyword: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AddCartParams {
    pub plugin_id: i64,
    pub plan_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemoveCartParams {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ClearCartParams {
    pub user_id: i64,
}