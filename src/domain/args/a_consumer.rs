use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerRegisterArgs {
    #[validate(length(min = 11, max = 11, message = "手机号长度必须为11位"))]
    pub phone: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须为6-20位"))]
    pub password: String,
    #[validate(length(min = 4, max = 6, message = "验证码长度必须为4-6位"))]
    pub sms_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerLoginArgs {
    #[validate(length(min = 11, max = 11, message = "手机号长度必须为11位"))]
    pub phone: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerWechatLoginArgs {
    #[validate(length(min = 1, message = "微信授权码不能为空"))]
    pub wechat_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerUpdateArgs {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdatePhoneArgs {
    #[validate(length(min = 11, max = 11, message = "新手机号长度必须为11位"))]
    pub new_phone: String,
    #[validate(length(min = 4, max = 6, message = "验证码长度必须为4-6位"))]
    pub sms_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdateEmailArgs {
    #[validate(email(message = "邮箱格式不正确"))]
    pub new_email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UploadAvatarArgs {
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct DeactivateArgs {
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub phone: Option<String>,
    pub login_type: Option<String>,
    pub success: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}