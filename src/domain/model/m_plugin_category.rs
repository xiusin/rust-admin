use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CategoryItem {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub parent_id: i64,
    pub sort: i32,
    pub plugin_count: i32,
    pub status: i32,
    pub children: Option<Vec<CategoryItem>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CategoryTreeItem {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub parent_id: i64,
    pub sort: i32,
    pub plugin_count: i32,
    pub children: Vec<CategoryTreeItem>,
}