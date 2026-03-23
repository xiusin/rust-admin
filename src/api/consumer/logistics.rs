use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::logistics_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::domain::args::a_logistics::*;
use crate::domain::model::m_logistics::*;
use crate::model::prelude::ListData;
use axum::{extract::Query, Json};
use validator::Validate;

pub async fn query(
    Json(args): Json<QueryLogisticsArgs>,
) -> Result<Json<ApiResponse<LogisticsDetailResp>>, Error> {
    args.validate()?;

    let result = logistics_service::query(QueryLogisticsParams {
        tracking_no: args.tracking_no,
        courier_company: args.courier_company,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn subscribe(
    Json(args): Json<SubscribeLogisticsArgs>,
) -> Result<Json<ApiResponse<LogisticsTrackingModel>>, Error> {
    args.validate()?;

    let result = logistics_service::subscribe(SubscribeLogisticsParams {
        order_no: "".to_string(),
        tracking_no: args.tracking_no,
        courier_company: args.courier_company,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_history(
    Query(params): Query<LogisticsHistoryArgs>,
) -> Result<Json<ApiResponse<ListData<LogisticsTraceResp>>>, Error> {
    let (items, total) = logistics_service::get_history(
        LogisticsHistoryParams {
            tracking_no: params.tracking_no,
            courier_company: params.courier_company,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: 0,
        page_num: 1,
    })))
}

pub fn logistics_api() -> WebPath {
    WebPath::new()
        .route("/query", WebPathType::Post, Some("查询物流"), post(query))
        .route("/subscribe", WebPathType::Post, Some("订阅物流"), post(subscribe))
        .route("/history", WebPathType::Get, Some("物流历史"), get(get_history))
}