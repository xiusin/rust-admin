use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TableConfigAddReq {
    pub model_id: i64,
    #[validate(length(min = 1, max = 50, message = "配置名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "配置编码长度1-50"))]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(default)]
    pub is_default: i32,
    #[serde(default = "default_config_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TableConfigEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "配置名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(default)]
    pub is_default: i32,
    #[serde(default = "default_config_status")]
    pub status: String,
}

fn default_config_status() -> String {
    "0".to_string()
}
