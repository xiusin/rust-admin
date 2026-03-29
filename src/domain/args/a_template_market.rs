use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TemplateUploadArgs {
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    #[validate(length(max = 500, message = "描述长度不超过500"))]
    pub description: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub style_params: Option<serde_json::Value>,
    pub preview_urls: Option<Vec<String>>,
    pub template_file: Option<String>,
    pub is_free: i8,
    pub price: Option<rust_decimal::Decimal>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TemplateEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: Option<String>,
    #[validate(length(max = 500, message = "描述长度不超过500"))]
    pub description: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub style_params: Option<serde_json::Value>,
    pub preview_urls: Option<Vec<String>>,
    pub template_file: Option<String>,
    pub is_free: Option<i8>,
    pub price: Option<rust_decimal::Decimal>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub is_free: Option<i8>,
    pub tags: Option<String>,
    pub sort_by: Option<String>,
    pub uploader_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateSearchArgs {
    pub keyword: Option<String>,
    pub industry: Option<String>,
    pub style: Option<String>,
    pub is_free: Option<i8>,
    pub tags: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TemplateRatingArgs {
    pub template_id: i64,
    #[validate(range(min = 1, max = 5, message = "评分范围1-5"))]
    pub rating: i32,
    #[validate(length(max = 500, message = "评论长度不超过500"))]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateCollectionArgs {
    pub template_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateApplyArgs {
    pub project_id: i64,
    pub template_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateRecommendArgs {
    pub industry: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MyTemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateStatusArgs {
    pub ids: Vec<i64>,
    pub status: String,
}
