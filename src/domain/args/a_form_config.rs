use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct FormConfigAddReq {
    pub model_id: i64,
    #[validate(length(min = 1, max = 50, message = "配置名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "配置编码长度1-50"))]
    pub code: String,
    #[serde(default = "default_form_type")]
    pub form_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<String>,
    #[serde(default)]
    pub is_default: i32,
    #[serde(default = "default_config_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct FormConfigEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "配置名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<String>,
    #[serde(default)]
    pub is_default: i32,
    #[serde(default = "default_config_status")]
    pub status: String,
}

fn default_form_type() -> String {
    "default".to_string()
}

fn default_config_status() -> String {
    "0".to_string()
}
