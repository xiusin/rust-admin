use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SendCodeArgs {
    #[validate(length(min = 11, max = 11, message = "手机号长度必须为11位"))]
    pub phone: String,
    pub sms_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct VerifyCodeArgs {
    #[validate(length(min = 11, max = 11, message = "手机号长度必须为11位"))]
    pub phone: String,
    #[validate(length(min = 4, max = 6, message = "验证码长度必须为4-6位"))]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SendNotificationArgs {
    #[validate(length(min = 11, max = 11, message = "手机号长度必须为11位"))]
    pub phone: String,
    #[validate(length(min = 1, message = "短信内容不能为空"))]
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SMSLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub phone: Option<String>,
    pub sms_type: Option<String>,
}