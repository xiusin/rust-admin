use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PluginListItem {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub developer_id: i64,
    pub developer_name: Option<String>,
    pub summary: Option<String>,
    pub cover_image: Option<String>,
    pub version: String,
    pub price_type: i32,
    pub price_type_name: String,
    pub verify_level: i32,
    pub verify_level_name: String,
    pub rating: f64,
    pub download_count: i32,
    pub status: i32,
    pub status_name: String,
    pub tags: Option<Vec<String>>,
    pub is_official: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PluginDetail {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub developer_id: i64,
    pub developer_name: Option<String>,
    pub developer_logo: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub version: String,
    pub price_type: i32,
    pub price_type_name: String,
    pub verify_level: i32,
    pub verify_level_name: String,
    pub rating: f64,
    pub download_count: i32,
    pub status: i32,
    pub tags: Option<Vec<String>>,
    pub is_official: i32,
    pub plans: Vec<PlanItem>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PlanItem {
    pub id: i64,
    pub plugin_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub period_type: i32,
    pub period_type_name: String,
    pub period_days: i32,
    pub price: f64,
    pub original_price: f64,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: i32,
    pub max_users: i32,
    pub storage_limit: i64,
    pub api_calls_limit: i64,
    pub support_level: i32,
    pub support_level_name: String,
    pub sort: i32,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FeatureItem {
    pub code: String,
    pub name: String,
    pub included: bool,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PluginStatistics {
    pub total: i64,
    pub active: i64,
    pub pending: i64,
    pub inactive: i64,
}

#[derive(Debug, Deserialize)]
pub struct PluginSearchParams {
    pub keyword: Option<String>,
    pub category_id: Option<i64>,
    pub price_type: Option<i32>,
    pub verify_level: Option<i32>,
    pub is_official: Option<i32>,
    pub sort: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ReviewItem {
    pub id: i64,
    pub plugin_id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
    pub rating: i32,
    pub content: Option<String>,
    pub reply_content: Option<String>,
    pub reply_time: Option<DateTime>,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VersionListItem {
    pub id: i64,
    pub plugin_id: i64,
    pub version: String,
    pub changelog: Option<String>,
    pub is_latest: i32,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VersionDetail {
    pub id: i64,
    pub plugin_id: i64,
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_hash: String,
    pub file_size: i64,
    pub min_app_version: Option<String>,
    pub is_latest: i32,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

pub fn get_price_type_name(price_type: i32) -> String {
    match price_type {
        0 => "免费".to_string(),
        1 => "一次性".to_string(),
        2 => "订阅".to_string(),
        _ => "未知".to_string(),
    }
}

pub fn get_verify_level_name(level: i32) -> String {
    match level {
        0 => "无验证".to_string(),
        1 => "基础验证".to_string(),
        2 => "高级验证".to_string(),
        _ => "未知".to_string(),
    }
}

pub fn get_status_name(status: i32) -> String {
    match status {
        0 => "待审核".to_string(),
        1 => "上架".to_string(),
        2 => "下架".to_string(),
        3 => "回收站".to_string(),
        _ => "未知".to_string(),
    }
}

pub fn get_period_type_name(period_type: i32) -> String {
    match period_type {
        0 => "月付".to_string(),
        1 => "季付".to_string(),
        2 => "年付".to_string(),
        3 => "永久".to_string(),
        _ => "未知".to_string(),
    }
}

pub fn get_support_level_name(level: i32) -> String {
    match level {
        0 => "基础支持".to_string(),
        1 => "高级支持".to_string(),
        2 => "专属客服".to_string(),
        _ => "未知".to_string(),
    }
}