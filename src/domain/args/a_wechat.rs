use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct WechatAuthUrlArgs {
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatCallbackArgs {
    pub code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct WechatUserInfoArgs {
    #[validate(length(min = 1, message = "访问令牌不能为空"))]
    pub access_token: String,
    #[validate(length(min = 1, message = "OpenID不能为空"))]
    pub openid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SendTemplateMessageArgs {
    #[validate(length(min = 1, message = "接收人OpenID不能为空"))]
    pub touser: String,
    #[validate(length(min = 1, message = "模板ID不能为空"))]
    pub template_id: String,
    pub data: Option<String>,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct MiniProgramLoginArgs {
    #[validate(length(min = 1, message = "临时登录凭证不能为空"))]
    pub code: String,
}