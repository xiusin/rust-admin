use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::domain::args::a_cms_tag::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsTagItem {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub content_count: i64,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsTagDetail {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub content_count: i64,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
