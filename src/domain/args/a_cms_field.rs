use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsFieldAddReq {
    pub model_id: i64,
    #[validate(length(min = 1, max = 50, message = "字段名称长度1-50"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "字段编码长度1-50"))]
    pub code: String,
    #[validate(length(min = 1, max = 30, message = "字段类型长度1-30"))]
    pub field_type: String,
    #[validate(length(min = 1, max = 30, message = "数据库类型长度1-30"))]
    pub db_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_required: i32,
    #[serde(default)]
    pub is_unique: i32,
    #[serde(default)]
    pub is_searchable: i32,
    #[serde(default)]
    pub is_sortable: i32,
    #[serde(default)]
    pub is_filterable: i32,
    #[serde(default = "default_field_show")]
    pub is_list_show: i32,
    #[serde(default = "default_field_show")]
    pub is_form_show: i32,
    #[serde(default = "default_field_show")]
    pub is_detail_show: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsFieldEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "字段名称长度1-50"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_required: i32,
    #[serde(default)]
    pub is_unique: i32,
    #[serde(default)]
    pub is_searchable: i32,
    #[serde(default)]
    pub is_sortable: i32,
    #[serde(default)]
    pub is_filterable: i32,
    #[serde(default = "default_field_show")]
    pub is_list_show: i32,
    #[serde(default = "default_field_show")]
    pub is_form_show: i32,
    #[serde(default = "default_field_show")]
    pub is_detail_show: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CmsFieldSortReq {
    pub model_id: i64,
    pub field_ids: Vec<i64>,
}

fn default_field_show() -> i32 {
    1
}
