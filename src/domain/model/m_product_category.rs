use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CategoryTreeItem {
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: i32,
    pub level: i32,
    pub path: String,
    pub status: String,
    pub show_in_nav: i32,
    pub children: Option<Vec<CategoryTreeItem>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CategoryListItem {
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: i32,
    pub level: i32,
    pub path: String,
    pub status: String,
    pub show_in_nav: i32,
    pub created_at: Option<DateTime>,
    pub parent_name: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CategoryDetail {
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: i32,
    pub level: i32,
    pub path: String,
    pub status: String,
    pub show_in_nav: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
