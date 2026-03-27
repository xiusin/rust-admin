use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

use super::m_cms_field::CmsFieldItem;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsModelList {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub table_name: String,
    pub icon: Option<String>,
    pub is_enabled: i32,
    pub sort: i32,
    pub field_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsModelDetail {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub table_name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub is_enabled: i32,
    pub sort: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub fields: Vec<CmsFieldItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsModelSimple {
    pub id: i64,
    pub name: String,
    pub code: String,
}
