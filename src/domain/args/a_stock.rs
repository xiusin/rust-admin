use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub product_name: Option<String>,
    pub category_id: Option<i64>,
    pub stock_status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub product_id: Option<i64>,
    pub sku_id: Option<i64>,
    pub change_type: Option<i32>,
    pub order_no: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StockAdjustArgs {
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub change_type: i32,
    pub change_num: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockAlertListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub product_name: Option<String>,
    pub is_alert: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StockAlertConfigArgs {
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub alert_stock: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockAlertBatchConfigArgs {
    pub configs: Vec<StockAlertConfigArgs>,
}
