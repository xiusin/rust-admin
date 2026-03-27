use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsTagAddReq {
    #[validate(length(min = 1, max = 50, message = "标签名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "标签编码长度1-50"))]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "default_tag_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsTagEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "标签名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "default_tag_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CmsTagListReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default = "default_page_num")]
    pub page_num: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsTagBatchAddReq {
    #[validate(length(min = 1, message = "标签列表不能为空"))]
    pub tags: Vec<String>,
}

fn default_tag_status() -> String {
    "0".to_string()
}

fn default_page_num() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}
