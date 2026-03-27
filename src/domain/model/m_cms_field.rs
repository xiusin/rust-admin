use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsFieldItem {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub field_type: String,
    pub db_type: String,
    pub is_required: i32,
    pub sort: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CmsFieldDetail {
    pub id: i64,
    pub model_id: i64,
    pub name: String,
    pub code: String,
    pub field_type: String,
    pub db_type: String,
    pub is_required: i32,
    pub is_searchable: i32,
    pub is_list_show: i32,
    pub is_form_show: i32,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub validation_rule: Option<String>,
    pub sort: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
