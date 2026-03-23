use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductGroupListItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub product_count: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductGroupDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductGroupSimple {
    pub id: i64,
    pub name: String,
}
