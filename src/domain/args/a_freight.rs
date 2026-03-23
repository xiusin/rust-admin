use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CalculateFreightArgs {
    pub template_id: i64,
    pub weight: f64,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub order_amount: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CreateFreightTemplateArgs {
    #[validate(length(min = 1, max = 100, message = "模板名称长度必须为1-100"))]
    pub name: String,
    #[validate(length(min = 1, message = "计算类型不能为空"))]
    pub calculation_type: String,
    pub first_weight: Option<f64>,
    pub first_price: Option<f64>,
    pub additional_weight: Option<f64>,
    pub additional_price: Option<f64>,
    pub region_rules: Option<String>,
    pub free_shipping_rules: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FreightTemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub calculation_type: Option<String>,
}