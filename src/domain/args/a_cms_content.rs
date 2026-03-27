use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsContentAddReq {
    pub model_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 200, message = "别名长度不超过200"))]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<String>,
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default = "default_content_status")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<DateTime>,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub is_top: i32,
    #[serde(default)]
    pub is_recommend: i32,
    #[serde(default)]
    pub is_hot: i32,
    #[serde(default = "default_allow_comment")]
    pub allow_comment: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsContentEditReq {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 200, message = "别名长度不超过200"))]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<String>,
    #[serde(default = "default_content_status")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<DateTime>,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub is_top: i32,
    #[serde(default)]
    pub is_recommend: i32,
    #[serde(default)]
    pub is_hot: i32,
    #[serde(default = "default_allow_comment")]
    pub allow_comment: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CmsContentListReq {
    pub model_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_top: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recommend: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hot: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default = "default_page_num")]
    pub page_num: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsContentDetailReq {
    pub id: i64,
}

fn default_content_type() -> String {
    "article".to_string()
}

fn default_content_status() -> String {
    "draft".to_string()
}

fn default_allow_comment() -> i32 {
    1
}

fn default_page_num() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}
