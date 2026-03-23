use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AttributeTemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AttributeTemplateAddArgs {
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    pub category_id: Option<i64>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    pub attributes: Option<Vec<AttributeItemArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AttributeTemplateEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    pub category_id: Option<i64>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    pub attributes: Option<Vec<AttributeItemArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AttributeItemArgs {
    pub name: String,
    pub attr_type: Option<i32>,
    pub is_required: Option<i32>,
    pub is_filter: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    pub values: Option<Vec<AttributeValueArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AttributeValueArgs {
    pub value: String,
    pub sort: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AttributeTemplateDeleteArgs {
    pub ids: Vec<i64>,
}
