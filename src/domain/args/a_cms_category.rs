use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsCategoryAddReq {
    #[serde(default)]
    pub parent_id: i64,
    pub model_id: i64,
    #[validate(length(min = 1, max = 50, message = "分类名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "分类编码长度1-50"))]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_detail: Option<String>,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "default_category_status")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsCategoryEditReq {
    pub id: i64,
    pub parent_id: i64,
    #[validate(length(min = 1, max = 50, message = "分类名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_detail: Option<String>,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "default_category_status")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CmsCategoryListReq {
    pub model_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CmsCategoryTreeReq {
    pub model_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CategorySortReq {
    pub items: Vec<CategorySortItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategorySortItem {
    pub id: i64,
    pub sort: i32,
}

fn default_category_status() -> String {
    "0".to_string()
}

fn default_page_size() -> i32 {
    10
}
