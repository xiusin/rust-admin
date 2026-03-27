use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsModelAddReq {
    #[validate(length(min = 1, max = 50, message = "模型名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "模型编码长度1-50"))]
    pub code: String,
    #[validate(length(min = 1, max = 50, message = "表名长度1-50"))]
    pub table_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500, message = "描述长度不超过500"))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub is_system: i32,
    #[serde(default = "default_is_enabled")]
    pub is_enabled: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsModelEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "模型名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500, message = "描述长度不超过500"))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default = "default_is_enabled")]
    pub is_enabled: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CmsModelListReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<i32>,
    #[serde(default = "default_page_num")]
    pub page_num: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsModelDetailReq {
    pub id: i64,
}

fn default_is_enabled() -> i32 {
    1
}

fn default_page_num() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}
