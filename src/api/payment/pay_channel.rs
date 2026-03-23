use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::domain::args::a_pay_channel::*;
use crate::application::payment::pay_channel_service as service;
use axum::{extract::Query, Json};

pub async fn list(
    Query(search): Query<PayChannelSearch>,
) -> Result<Json<ApiResponse<Vec<service::PayChannelRes>>>, Error> {
    let list = service::list(search).await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn add(
    Json(arg): Json<PayChannelAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn edit(
    Json(arg): Json<PayChannelEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn config(
    Json(arg): Json<PayChannelConfig>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::update_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_status(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub fn pay_channel() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("支付渠道列表"), get(list))
        .route("/add", WebPathType::Post, Some("新增支付渠道"), post(add))
        .route("/edit", WebPathType::Put, Some("编辑支付渠道"), put(edit))
        .route("/del", WebPathType::Delete, Some("删除支付渠道"), delete(del))
        .route("/config", WebPathType::Put, Some("配置支付渠道"), put(config))
        .route("/toggle", WebPathType::Put, Some("切换支付渠道状态"), put(toggle))
}
