use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StoreAddArgs {
    #[validate(length(min = 1, max = 100, message = "门店名称长度1-100"))]
    pub name: String,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub contact_name: Option<String>,
    #[validate(length(max = 20, message = "联系电话长度最大20"))]
    pub contact_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub business_hours: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StoreEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "门店名称长度1-100"))]
    pub name: String,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub contact_name: Option<String>,
    #[validate(length(max = 20, message = "联系电话长度最大20"))]
    pub contact_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub business_hours: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreStockListArgs {
    pub store_id: i64,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub product_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StoreStockAdjustArgs {
    pub store_id: i64,
    pub product_id: i64,
    pub sku_id: Option<i64>,
    pub change_num: i32,
    pub remark: Option<String>,
}
