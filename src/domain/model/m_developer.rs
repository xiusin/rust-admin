use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeveloperProfile {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub contact: Option<String>,
    pub plugins_count: i32,
    pub total_downloads: i64,
    pub rating: f64,
    pub status: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeveloperStats {
    pub total_plugins: i64,
    pub active_plugins: i64,
    pub total_downloads: i64,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub pending_orders: i64,
    pub rating: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeveloperSalesData {
    pub date: String,
    pub orders: i64,
    pub revenue: f64,
    pub downloads: i64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeveloperParams {
    pub name: String,
    pub description: Option<String>,
    pub contact: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeveloperParams {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub contact: Option<String>,
}