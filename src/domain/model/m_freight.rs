use crate::model::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationType {
    ByWeight,
    ByDistance,
}

impl std::fmt::Display for CalculationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationType::ByWeight => write!(f, "by_weight"),
            CalculationType::ByDistance => write!(f, "by_distance"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FreightTemplateModel {
    pub id: i64,
    pub name: String,
    pub calculation_type: String,
    pub first_weight: Decimal,
    pub first_price: Decimal,
    pub additional_weight: Decimal,
    pub additional_price: Decimal,
    pub region_rules: Vec<RegionRule>,
    pub free_shipping_rules: Vec<FreeShippingRule>,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalculateFreightParams {
    pub template_id: i64,
    pub weight: f64,
    pub province: String,
    pub city: String,
    pub district: String,
    pub order_amount: Option<Decimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateFreightTemplateParams {
    pub name: String,
    pub calculation_type: CalculationType,
    pub first_weight: Decimal,
    pub first_price: Decimal,
    pub additional_weight: Decimal,
    pub additional_price: Decimal,
    pub region_rules: Vec<RegionRule>,
    pub free_shipping_rules: Vec<FreeShippingRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionRule {
    pub province: String,
    pub city: String,
    pub district: Option<String>,
    pub first_weight: Decimal,
    pub first_price: Decimal,
    pub additional_weight: Decimal,
    pub additional_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FreeShippingRule {
    pub province: String,
    pub city: String,
    pub min_order_amount: Option<Decimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FreightTemplateListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub calculation_type: Option<CalculationType>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FreightCalculateResp {
    pub template_id: i64,
    pub base_price: Decimal,
    pub additional_price: Decimal,
    pub total_price: Decimal,
    pub free_shipping: bool,
}