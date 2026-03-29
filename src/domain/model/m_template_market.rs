use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateMarketListItem {
    pub id: i64,
    pub name: String,
    pub thumbnail_url: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub downloads: i32,
    pub rating: Decimal,
    pub rating_count: i32,
    pub collection_count: i32,
    pub is_free: i8,
    pub price: Option<Decimal>,
    pub uploader_name: String,
    pub tags: Vec<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateMarketDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub style_params: Option<Json>,
    pub preview_urls: Vec<String>,
    pub template_file: Option<String>,
    pub downloads: i32,
    pub rating: Decimal,
    pub rating_count: i32,
    pub collection_count: i32,
    pub is_free: i8,
    pub price: Option<Decimal>,
    pub uploader: CreatorInfo,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreatorInfo {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateRatingItem {
    pub id: i64,
    pub user_name: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateCollectionItem {
    pub id: i64,
    pub template_id: i64,
    pub template_name: String,
    pub thumbnail_url: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateStatistics {
    pub total_templates: i64,
    pub free_count: i64,
    pub paid_count: i64,
    pub total_downloads: i64,
}
