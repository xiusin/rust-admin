use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ShippingTemplateAddArgs {
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    pub charge_type: Option<i32>,
    pub is_free: Option<i32>,
    pub free_condition_type: Option<i32>,
    pub free_condition_value: Option<f64>,
    pub status: Option<String>,
    pub regions: Option<Vec<ShippingRegionArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShippingRegionArgs {
    pub region_type: Option<i32>,
    pub region_ids: Option<String>,
    pub region_names: Option<String>,
    pub first_unit: Option<i32>,
    pub first_fee: Option<f64>,
    pub continue_unit: Option<i32>,
    pub continue_fee: Option<f64>,
    pub is_free: Option<i32>,
    pub free_condition_value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ShippingTemplateEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    pub charge_type: Option<i32>,
    pub is_free: Option<i32>,
    pub free_condition_type: Option<i32>,
    pub free_condition_value: Option<f64>,
    pub status: Option<String>,
    pub regions: Option<Vec<ShippingRegionArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShippingTemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShippingTemplateDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShippingCalculateArgs {
    pub template_id: i64,
    pub province: String,
    pub city: Option<String>,
    pub total_weight: Option<f64>,
    pub total_volume: Option<f64>,
    pub total_quantity: Option<i32>,
    pub total_amount: Option<f64>,
}
