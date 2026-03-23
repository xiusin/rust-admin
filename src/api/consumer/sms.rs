use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::sms_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::model::prelude::ListData;
use crate::domain::args::a_sms::*;
use crate::domain::model::m_sms_log::*;
use axum::{extract::Query, Json};
use validator::Validate;

pub async fn send_code(
    Json(args): Json<SendCodeArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    args.validate()?;

    sms_service::send_code(SendCodeParams {
        phone: args.phone,
        sms_type: match args.sms_type.as_str() {
            "verification" => SMSType::Verification,
            _ => SMSType::Notification,
        },
    })
    .await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn verify_code(
    Json(args): Json<VerifyCodeArgs>,
) -> Result<Json<ApiResponse<VerifyCodeResp>>, Error> {
    args.validate()?;

    let valid = sms_service::verify_code(VerifyCodeParams {
        phone: args.phone,
        code: args.code,
    })
    .await?;

    Ok(Json(ApiResponse::success(VerifyCodeResp { valid })))
}

pub async fn list_logs(
    Query(params): Query<SMSLogListArgs>,
) -> Result<Json<ApiResponse<ListData<SMSLogModel>>>, Error> {
    let page_size = params.page_size;
    let page_num = params.page_num;
    let (items, total) = sms_service::list_logs(
        SMSLogSearchParams {
            page_num: params.page_num,
            page_size: params.page_size,
            phone: params.phone,
            sms_type: params.sms_type,
            status: None,
            start_time: None,
            end_time: None,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: (total + page_size.unwrap_or(10) as u64 - 1) / page_size.unwrap_or(10) as u64,
        page_num: page_num.unwrap_or(1) as u64,
    })))
}

#[derive(Debug, serde::Serialize)]
pub struct VerifyCodeResp {
    pub valid: bool,
}

pub fn sms_api() -> WebPath {
    WebPath::new()
        .route("/send-code", WebPathType::Post, Some("发送验证码"), post(send_code))
        .route("/verify-code", WebPathType::Post, Some("验证验证码"), post(verify_code))
        .route("/logs", WebPathType::Get, Some("短信日志列表"), get(list_logs))
}