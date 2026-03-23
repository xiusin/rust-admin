use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BrandListItem {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub product_count: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BrandDetail {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BrandSimple {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
}
