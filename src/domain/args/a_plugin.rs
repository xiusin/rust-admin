use serde::{Deserialize, Serialize};
use crate::domain::model::m_plugin::*;

#[derive(Debug, Deserialize)]
pub struct CreatePluginParams {
    pub code: String,
    pub name: String,
    pub category_id: i64,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub version: String,
    pub price_type: i32,
    pub verify_level: i32,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePluginParams {
    pub id: i64,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct DeletePluginParams {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuditPluginParams {
    pub id: i64,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct PublishVersionParams {
    pub plugin_id: i64,
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_hash: String,
    pub file_size: i64,
    pub min_app_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanParams {
    pub plugin_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub period_type: i32,
    pub period_days: i32,
    pub price: f64,
    pub original_price: f64,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: i32,
    pub max_users: i32,
    pub storage_limit: i64,
    pub api_calls_limit: i64,
    pub support_level: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanParams {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub period_type: Option<i32>,
    pub period_days: Option<i32>,
    pub price: Option<f64>,
    pub original_price: Option<f64>,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: Option<i32>,
    pub max_users: Option<i32>,
    pub storage_limit: Option<i64>,
    pub api_calls_limit: Option<i64>,
    pub support_level: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct DeletePlanParams {
    pub id: i64,
}