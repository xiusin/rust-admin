use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerificationCodeListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub code: Option<String>,
    pub order_no: Option<String>,
    pub status: Option<i32>,
    pub store_id: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct VerificationArgs {
    #[validate(length(min = 1, message = "核销码不能为空"))]
    pub code: String,
    pub store_id: Option<i64>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerificationLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub code: Option<String>,
    pub order_no: Option<String>,
    pub store_id: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerificationQueryArgs {
    pub code: String,
}
