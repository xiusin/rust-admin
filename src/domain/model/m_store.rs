use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StoreListItem {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub business_hours: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StoreDetail {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub business_hours: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StoreSimple {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub contact_phone: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StoreStockItem {
    pub id: i64,
    pub store_id: i64,
    pub store_name: String,
    pub product_id: i64,
    pub product_name: String,
    pub product_image: String,
    pub sku_id: Option<i64>,
    pub sku_code: Option<String>,
    pub spec_text: Option<String>,
    pub stock: i32,
    pub alert_stock: i32,
    pub is_alert: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StoreStatistics {
    pub total_stores: i64,
    pub active_stores: i64,
    pub inactive_stores: i64,
}
