use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::freight_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::domain::args::a_freight::*;
use crate::domain::model::m_freight::*;
use crate::model::prelude::ListData;
use axum::{extract::Query, Json};
use rust_decimal::Decimal;
use validator::Validate;

pub async fn calculate(
    Json(args): Json<CalculateFreightArgs>,
) -> Result<Json<ApiResponse<FreightCalculateResp>>, Error> {
    let result = freight_service::calculate_freight(CalculateFreightParams {
        template_id: args.template_id,
        weight: args.weight,
        province: args.province.unwrap_or_default(),
        city: args.city.unwrap_or_default(),
        district: args.district.unwrap_or_default(),
        order_amount: args.order_amount.map(|v| Decimal::try_from(v).unwrap_or(Decimal::ZERO)),
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_template(
    Json(args): Json<CreateFreightTemplateArgs>,
) -> Result<Json<ApiResponse<FreightTemplateModel>>, Error> {
    args.validate()?;

    let region_rules: Vec<RegionRule> = args.region_rules
        .and_then(|r| serde_json::from_str(&r).ok())
        .unwrap_or_default();
    let free_shipping_rules: Vec<FreeShippingRule> = args.free_shipping_rules
        .and_then(|r| serde_json::from_str(&r).ok())
        .unwrap_or_default();

    let result = freight_service::create_template(CreateFreightTemplateParams {
        name: args.name,
        calculation_type: match args.calculation_type.as_str() {
            "by_distance" => CalculationType::ByDistance,
            _ => CalculationType::ByWeight,
        },
        first_weight: Decimal::try_from(args.first_weight.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
        first_price: Decimal::try_from(args.first_price.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
        additional_weight: Decimal::try_from(args.additional_weight.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
        additional_price: Decimal::try_from(args.additional_price.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
        region_rules,
        free_shipping_rules,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn list_templates(
    Query(params): Query<FreightTemplateListArgs>,
) -> Result<Json<ApiResponse<ListData<FreightTemplateModel>>>, Error> {
    let page_size = params.page_size;
    let page_num = params.page_num;
    let (items, total) = freight_service::list_templates(
        FreightTemplateListParams {
            page_num: params.page_num,
            page_size: params.page_size,
            name: params.name,
            calculation_type: None,
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

pub fn freight_api() -> WebPath {
    WebPath::new()
        .route("/calculate", WebPathType::Post, Some("计算运费"), post(calculate))
        .route("/template", WebPathType::Post, Some("创建运费模板"), post(create_template))
        .route("/templates", WebPathType::Get, Some("运费模板列表"), get(list_templates))
}