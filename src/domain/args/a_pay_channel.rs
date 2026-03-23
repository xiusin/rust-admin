use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PayChannelAdd {
    #[validate(length(min = 1, max = 100, message = "渠道名称长度1-100"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "渠道标识长度1-50"))]
    pub code: String,
    #[validate(length(min = 1, max = 20, message = "渠道类型长度1-20"))]
    pub channel_type: String,
    pub scenes: Vec<String>,
    pub sort: Option<i32>,
    pub is_active: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PayChannelEdit {
    pub id: i64,
    #[validate(length(min = 1, max = 100, message = "渠道名称长度1-100"))]
    pub name: String,
    #[validate(length(min = 1, max = 50, message = "渠道标识长度1-50"))]
    pub code: String,
    #[validate(length(min = 1, max = 20, message = "渠道类型长度1-20"))]
    pub channel_type: String,
    pub scenes: Vec<String>,
    pub sort: Option<i32>,
    pub is_active: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayChannelConfig {
    pub id: i64,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayChannelSearch {
    pub name: Option<String>,
    pub channel_type: Option<String>,
    pub is_active: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdQuery {
    pub id: i64,
}
