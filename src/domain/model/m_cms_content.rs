use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

use super::m_cms_tag::CmsTagItem;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsContentList {
    pub id: i64,
    pub model_id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub thumbnail: Option<String>,
    pub status: i32,
    pub publish_time: Option<DateTime>,
    pub view_count: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsContentDetail {
    pub id: i64,
    pub model_id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub subtitle: Option<String>,
    pub thumbnail: Option<String>,
    pub images: Option<Vec<String>>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub status: i32,
    pub publish_time: Option<DateTime>,
    pub view_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub is_top: i32,
    pub is_recommend: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub tags: Vec<CmsTagItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsContentSimple {
    pub id: i64,
    pub title: String,
}
