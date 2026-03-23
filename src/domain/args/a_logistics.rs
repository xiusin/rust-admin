use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct QueryLogisticsArgs {
    #[validate(length(min = 1, message = "物流单号不能为空"))]
    pub tracking_no: String,
    #[validate(length(min = 1, message = "快递公司不能为空"))]
    pub courier_company: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SubscribeLogisticsArgs {
    #[validate(length(min = 1, message = "物流单号不能为空"))]
    pub tracking_no: String,
    #[validate(length(min = 1, message = "快递公司不能为空"))]
    pub courier_company: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogisticsHistoryArgs {
    pub tracking_no: String,
    pub courier_company: String,
}