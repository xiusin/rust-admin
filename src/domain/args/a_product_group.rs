use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ProductGroupAddArgs {
    #[validate(length(min = 1, max = 100, message = "分组名称长度1-100"))]
    pub name: String,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ProductGroupEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "分组名称长度1-100"))]
    pub name: String,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductGroupListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductGroupDeleteArgs {
    pub ids: Vec<i64>,
}
