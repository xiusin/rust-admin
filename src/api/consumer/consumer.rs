use axum::routing::{get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::consumer_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::model::prelude::ListData;
use crate::domain::args::a_consumer::*;
use crate::domain::model::m_consumer::*;
use axum::{extract::{Path, Query}, Json};
use validator::Validate;

#[derive(Debug, serde::Serialize)]
pub struct LoginResp {
    pub consumer: ConsumerResp,
    pub token: String,
}

pub async fn register(
    Json(args): Json<ConsumerRegisterArgs>,
) -> Result<Json<ApiResponse<ConsumerResp>>, Error> {
    args.validate()?;

    let result = consumer_service::register(ConsumerRegisterParams {
        phone: args.phone,
        password: args.password,
        sms_code: args.sms_code,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn login(
    Json(args): Json<ConsumerLoginArgs>,
) -> Result<Json<ApiResponse<LoginResp>>, Error> {
    args.validate()?;

    let (consumer, token) = consumer_service::login(
        ConsumerLoginParams {
            phone: args.phone,
            password: args.password,
        },
        "127.0.0.1".to_string(),
        "Unknown".to_string(),
    )
    .await?;

    Ok(Json(ApiResponse::success(LoginResp { consumer, token })))
}

pub async fn get_info(
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ConsumerResp>>, Error> {
    let result = consumer_service::get_info(id).await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn update(
    Path(id): Path<i64>,
    Json(args): Json<ConsumerUpdateArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    consumer_service::update(
        id,
        ConsumerUpdateParams {
            nickname: args.nickname,
            email: args.email,
            avatar: args.avatar,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn list(
    Query(params): Query<ConsumerListArgs>,
) -> Result<Json<ApiResponse<ListData<ConsumerListItem>>>, Error> {
    let (items, total) = consumer_service::list(
        ConsumerListArgs {
            page_num: params.page_num,
            page_size: params.page_size,
            phone: params.phone,
            status: params.status,
        },
    )
    .await?;

    let page_size = params.page_size.unwrap_or(10) as u64;
    let page_num = params.page_num.unwrap_or(1) as u64;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })))
}

pub async fn login_logs(
    Query(params): Query<LoginLogListArgs>,
) -> Result<Json<ApiResponse<ListData<LoginLogResp>>>, Error> {
    let page_size = params.page_size.unwrap_or(10) as u64;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let (items, total) = consumer_service::login_logs(params).await?;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })))
}

pub fn consumer_api() -> WebPath {
    WebPath::new()
        .route("/register", WebPathType::Post, Some("用户注册"), post(register))
        .route("/login", WebPathType::Post, Some("用户登录"), post(login))
        .route("/info/{id}", WebPathType::Get, Some("获取用户信息"), get(get_info))
        .route("/update/{id}", WebPathType::Put, Some("更新用户信息"), put(update))
        .route("/list", WebPathType::Get, Some("用户列表"), get(list))
        .route("/login-logs", WebPathType::Get, Some("登录日志"), get(login_logs))
}