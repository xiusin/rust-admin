use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CategoryAddArgs {
    #[validate(length(min = 1, max = 100, message = "分类名称长度1-100"))]
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    pub show_in_nav: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CategoryEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "分类名称长度1-100"))]
    pub name: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    pub show_in_nav: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryTreeArgs {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryDeleteArgs {
    pub ids: Vec<i64>,
}
