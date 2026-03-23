use crate::model::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Recharge,
    Consume,
    Withdraw,
    Refund,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FinanceAccountModel {
    pub id: i64,
    pub consumer_id: i64,
    pub balance: Decimal,
    pub frozen_balance: Decimal,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TransactionModel {
    pub id: i64,
    pub consumer_id: i64,
    pub transaction_no: String,
    pub transaction_type: String,
    pub amount: Decimal,
    pub balance_before: Decimal,
    pub balance_after: Decimal,
    pub description: Option<String>,
    pub related_order_no: Option<String>,
    pub operator_id: Option<i64>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RechargeParams {
    pub consumer_id: i64,
    pub amount: Decimal,
    pub related_order_no: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithdrawParams {
    pub consumer_id: i64,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithdrawApproveParams {
    pub transaction_id: i64,
    pub approve: bool,
    pub reason: Option<String>,
    pub operator_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumeParams {
    pub consumer_id: i64,
    pub amount: Decimal,
    pub description: Option<String>,
    pub related_order_no: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefundParams {
    pub consumer_id: i64,
    pub amount: Decimal,
    pub description: Option<String>,
    pub related_order_no: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AccountInfoResp {
    pub id: i64,
    pub consumer_id: i64,
    pub balance: Decimal,
    pub frozen_balance: Decimal,
    pub available_balance: Decimal,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FinanceStatistics {
    pub balance: Decimal,
    pub frozen_balance: Decimal,
    pub available_balance: Decimal,
    pub total_recharge: Decimal,
    pub total_withdraw: Decimal,
    pub total_consume: Decimal,
    pub total_refund: Decimal,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FreightCalculateResp {
    pub template_id: i64,
    pub base_price: Decimal,
    pub additional_price: Decimal,
    pub total_price: Decimal,
    pub free_shipping: bool,
}

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalculateFreightParams {
    pub template_id: i64,
    pub weight: f64,
    pub province: String,
    pub city: String,
    pub district: String,
    pub order_amount: Option<Decimal>,
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

#[derive(Debug, Serialize, Clone, Deserialize)]
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
pub struct FreightTemplateListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub calculation_type: Option<CalculationType>,
}
