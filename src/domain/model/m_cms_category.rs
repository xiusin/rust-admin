use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::domain::args::a_cms_category::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsCategoryItem {
    pub id: i64,
    pub parent_id: i64,
    pub model_id: Option<i64>,
    pub name: String,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub page_size: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsCategoryTree {
    pub id: i64,
    pub parent_id: i64,
    pub model_id: Option<i64>,
    pub name: String,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub page_size: i32,
    pub children: Vec<CmsCategoryTree>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsCategoryDetail {
    pub id: i64,
    pub parent_id: i64,
    pub model_id: Option<i64>,
    pub name: String,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub template_list: Option<String>,
    pub template_detail: Option<String>,
    pub page_size: i32,
    pub sort: i32,
    pub status: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
