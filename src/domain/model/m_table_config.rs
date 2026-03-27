use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, sea_orm::FromQueryResult)]
pub struct TableConfigItem {
    pub id: i64,
    pub model_id: i64,
    pub name: String,
    pub code: String,
    pub is_default: i32,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize, sea_orm::FromQueryResult)]
pub struct TableConfigDetail {
    pub id: i64,
    pub model_id: i64,
    pub name: String,
    pub code: String,
    pub is_default: i32,
    pub status: i32,
    pub config: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
