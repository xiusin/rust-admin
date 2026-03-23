use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StockListItem {
    pub product_id: i64,
    pub product_name: String,
    pub product_image: String,
    pub category_name: Option<String>,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub stock: i32,
    pub sales: i32,
    pub alert_stock: Option<i32>,
    pub is_alert: bool,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StockLogItem {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub change_type: i32,
    pub change_type_name: String,
    pub change_num: i32,
    pub before_stock: i32,
    pub after_stock: i32,
    pub order_no: Option<String>,
    pub remark: Option<String>,
    pub operator_name: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StockAlertItem {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub product_image: String,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub stock: i32,
    pub alert_stock: i32,
    pub is_alert: i32,
    pub last_alert_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StockStatistics {
    pub total_stock: i64,
    pub alert_count: i64,
    pub out_of_stock_count: i64,
    pub low_stock_count: i64,
}
