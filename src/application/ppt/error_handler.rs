use crate::common::error::Error;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn format_error(error: &Error) -> UserFriendlyError {
        match error {
            Error::Message(msg) => UserFriendlyError {
                code: 500,
                title: "操作失败".to_string(),
                message: msg.clone(),
                suggestion: Some("请稍后重试".to_string()),
                action: None,
            },
            Error::WithStatus(status, msg) => {
                Self::handle_status_error(*status, msg)
            }
        }
    }

    fn handle_status_error(status: StatusCode, msg: &str) -> UserFriendlyError {
        match status {
            StatusCode::BAD_REQUEST => UserFriendlyError {
                code: 400,
                title: "请求参数错误".to_string(),
                message: msg.to_string(),
                suggestion: Some("请检查输入参数是否正确".to_string()),
                action: Some(ErrorAction::Retry),
            },
            StatusCode::UNAUTHORIZED => UserFriendlyError {
                code: 401,
                title: "未授权".to_string(),
                message: "请先登录后再操作".to_string(),
                suggestion: Some("点击这里登录".to_string()),
                action: Some(ErrorAction::Login),
            },
            StatusCode::FORBIDDEN => UserFriendlyError {
                code: 403,
                title: "权限不足".to_string(),
                message: "您没有权限执行此操作".to_string(),
                suggestion: Some("请联系管理员获取权限".to_string()),
                action: Some(ErrorAction::ContactSupport),
            },
            StatusCode::NOT_FOUND => UserFriendlyError {
                code: 404,
                title: "资源不存在".to_string(),
                message: msg.to_string(),
                suggestion: Some("请检查资源ID是否正确".to_string()),
                action: Some(ErrorAction::GoBack),
            },
            StatusCode::PAYLOAD_TOO_LARGE => UserFriendlyError {
                code: 413,
                title: "文件过大".to_string(),
                message: "上传的文件大小超过限制".to_string(),
                suggestion: Some("请压缩文件后重试，或联系客服提升额度".to_string()),
                action: Some(ErrorAction::Retry),
            },
            StatusCode::UNSUPPORTED_MEDIA_TYPE => UserFriendlyError {
                code: 415,
                title: "不支持的文件格式".to_string(),
                message: msg.to_string(),
                suggestion: Some("支持的格式: PPTX, DOCX, PDF, MD, TXT".to_string()),
                action: Some(ErrorAction::Retry),
            },
            StatusCode::TOO_MANY_REQUESTS => UserFriendlyError {
                code: 429,
                title: "请求过于频繁".to_string(),
                message: "您的请求次数已达上限".to_string(),
                suggestion: Some("请稍后再试，或升级套餐获取更多额度".to_string()),
                action: Some(ErrorAction::UpgradePlan),
            },
            StatusCode::INTERNAL_SERVER_ERROR => UserFriendlyError {
                code: 500,
                title: "服务器错误".to_string(),
                message: "服务器处理请求时发生错误".to_string(),
                suggestion: Some("我们正在修复，请稍后重试".to_string()),
                action: Some(ErrorAction::ContactSupport),
            },
            StatusCode::SERVICE_UNAVAILABLE => UserFriendlyError {
                code: 503,
                title: "服务暂时不可用".to_string(),
                message: "AI服务正在维护中".to_string(),
                suggestion: Some("请稍后重试".to_string()),
                action: Some(ErrorAction::Retry),
            },
            _ => UserFriendlyError {
                code: status.as_u16(),
                title: "操作失败".to_string(),
                message: msg.to_string(),
                suggestion: None,
                action: None,
            },
        }
    }

    pub fn format_validation_error(field: &str, message: &str) -> UserFriendlyError {
        UserFriendlyError {
            code: 400,
            title: "参数验证失败".to_string(),
            message: format!("字段 '{}': {}", field, message),
            suggestion: Some("请检查输入格式是否正确".to_string()),
            action: Some(ErrorAction::Retry),
        }
    }

    pub fn format_ai_error(provider: &str, error: &str) -> UserFriendlyError {
        UserFriendlyError {
            code: 502,
            title: "AI服务错误".to_string(),
            message: format!("AI服务商({})处理失败: {}", provider, error),
            suggestion: Some("我们正在尝试备用服务，请稍后重试".to_string()),
            action: Some(ErrorAction::Retry),
        }
    }

    pub fn format_quota_exceeded(quota_type: &str, remaining: u32) -> UserFriendlyError {
        UserFriendlyError {
            code: 402,
            title: "额度不足".to_string(),
            message: format!("您的{}额度已用完，剩余额度: {}", quota_type, remaining),
            suggestion: Some("请升级套餐获取更多额度".to_string()),
            action: Some(ErrorAction::UpgradePlan),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFriendlyError {
    pub code: u16,
    pub title: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub action: Option<ErrorAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorAction {
    Retry,
    Login,
    GoBack,
    ContactSupport,
    UpgradePlan,
}

impl ErrorAction {
    pub fn display_name(&self) -> &str {
        match self {
            ErrorAction::Retry => "重试",
            ErrorAction::Login => "登录",
            ErrorAction::GoBack => "返回",
            ErrorAction::ContactSupport => "联系客服",
            ErrorAction::UpgradePlan => "升级套餐",
        }
    }
}
