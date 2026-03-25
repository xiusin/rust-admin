use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CartItem {
    pub id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plugin_name: String,
    pub plugin_cover: Option<String>,
    pub plugin_version: String,
    pub plan_id: i64,
    pub plan_name: String,
    pub price: f64,
    pub original_price: f64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
pub struct AddCartParams {
    pub plugin_id: i64,
    pub plan_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemoveCartParams {
    pub ids: Vec<i64>,
}