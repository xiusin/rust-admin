use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct BrandAddArgs {
    #[validate(length(min = 1, max = 100, message = "品牌名称长度1-100"))]
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct BrandEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "品牌名称长度1-100"))]
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrandListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrandDeleteArgs {
    pub ids: Vec<i64>,
}
