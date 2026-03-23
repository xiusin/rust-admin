use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShippingTemplateListItem {
    pub id: i64,
    pub name: String,
    pub charge_type: i32,
    pub charge_type_name: String,
    pub is_free: i32,
    pub free_condition_type: i32,
    pub free_condition_value: f64,
    pub status: String,
    pub product_count: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShippingTemplateDetail {
    pub id: i64,
    pub name: String,
    pub charge_type: i32,
    pub is_free: i32,
    pub free_condition_type: i32,
    pub free_condition_value: f64,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub regions: Vec<ShippingRegionItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShippingRegionItem {
    pub id: i64,
    pub template_id: i64,
    pub region_type: i32,
    pub region_ids: Option<String>,
    pub region_names: Option<String>,
    pub first_unit: i32,
    pub first_fee: f64,
    pub continue_unit: i32,
    pub continue_fee: f64,
    pub is_free: i32,
    pub free_condition_value: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShippingFeeResult {
    pub fee: f64,
    pub template_name: String,
    pub calculation_detail: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShippingTemplateSimple {
    pub id: i64,
    pub name: String,
    pub charge_type: i32,
}
