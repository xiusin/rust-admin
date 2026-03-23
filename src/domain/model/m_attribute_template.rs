use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AttributeTemplateListItem {
    pub id: i64,
    pub name: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub sort: i32,
    pub status: String,
    pub attribute_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AttributeTemplateDetail {
    pub id: i64,
    pub name: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub attributes: Vec<AttributeItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AttributeItem {
    pub id: i64,
    pub name: String,
    pub attr_type: i32,
    pub is_required: i32,
    pub is_filter: i32,
    pub sort: i32,
    pub status: String,
    pub values: Vec<AttributeValueItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AttributeValueItem {
    pub id: i64,
    pub value: String,
    pub sort: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AttributeTemplateSimple {
    pub id: i64,
    pub name: String,
}
