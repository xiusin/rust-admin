use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::application::consumer::config_service as service;
use crate::application::consumer::config_service::*;
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: i64,
}

pub async fn sms_config_list() -> Result<Json<ApiResponse<Vec<SmsConfigRes>>>, Error> {
    let list = service::list_sms_config().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn sms_config_add(
    Json(arg): Json<SmsConfigAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_sms_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_config_edit(
    Json(arg): Json<SmsConfigEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_sms_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_config_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_sms_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_config_toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_sms_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_template_list() -> Result<Json<ApiResponse<Vec<SmsTemplateRes>>>, Error> {
    let list = service::list_sms_template().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn sms_template_add(
    Json(arg): Json<SmsTemplateAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_sms_template(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_template_edit(
    Json(arg): Json<SmsTemplateEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_sms_template(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn sms_template_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_sms_template(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mp_list() -> Result<Json<ApiResponse<Vec<WechatMpConfigRes>>>, Error> {
    let list = service::list_wechat_mp_config().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn wechat_mp_add(
    Json(arg): Json<WechatMpConfigAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_wechat_mp_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mp_edit(
    Json(arg): Json<WechatMpConfigEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_wechat_mp_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mp_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_wechat_mp_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mp_toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_wechat_mp_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mini_list() -> Result<Json<ApiResponse<Vec<WechatMiniConfigRes>>>, Error> {
    let list = service::list_wechat_mini_config().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn wechat_mini_add(
    Json(arg): Json<WechatMiniConfigAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_wechat_mini_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mini_edit(
    Json(arg): Json<WechatMiniConfigEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_wechat_mini_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mini_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_wechat_mini_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn wechat_mini_toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_wechat_mini_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn oss_config_list() -> Result<Json<ApiResponse<Vec<OssConfigRes>>>, Error> {
    let list = service::list_oss_config().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn oss_config_add(
    Json(arg): Json<OssConfigAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_oss_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn oss_config_edit(
    Json(arg): Json<OssConfigEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_oss_config(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn oss_config_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_oss_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn oss_config_toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_oss_config(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn logistics_company_list() -> Result<Json<ApiResponse<Vec<LogisticsCompanyRes>>>, Error> {
    let list = service::list_logistics_company().await?;
    Ok(Json(ApiResponse::success(list)))
}

pub async fn logistics_company_add(
    Json(arg): Json<LogisticsCompanyAdd>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::add_logistics_company(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn logistics_company_edit(
    Json(arg): Json<LogisticsCompanyEdit>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::edit_logistics_company(arg).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn logistics_company_del(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::delete_logistics_company(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub async fn logistics_company_toggle(
    Query(query): Query<IdQuery>,
) -> Result<Json<ApiResponse<String>>, Error> {
    service::toggle_logistics_company(query.id).await?;
    Ok(Json(ApiResponse::success("success".to_string())))
}

pub fn config_api() -> WebPath {
    WebPath::new()
        .route("/sms-config/list", WebPathType::Get, Some("短信配置列表"), get(sms_config_list))
        .route("/sms-config/add", WebPathType::Post, Some("新增短信配置"), post(sms_config_add))
        .route("/sms-config/edit", WebPathType::Put, Some("编辑短信配置"), put(sms_config_edit))
        .route("/sms-config/del", WebPathType::Delete, Some("删除短信配置"), delete(sms_config_del))
        .route("/sms-config/toggle", WebPathType::Put, Some("切换短信配置状态"), put(sms_config_toggle))
        .route("/sms-template/list", WebPathType::Get, Some("短信模板列表"), get(sms_template_list))
        .route("/sms-template/add", WebPathType::Post, Some("新增短信模板"), post(sms_template_add))
        .route("/sms-template/edit", WebPathType::Put, Some("编辑短信模板"), put(sms_template_edit))
        .route("/sms-template/del", WebPathType::Delete, Some("删除短信模板"), delete(sms_template_del))
        .route("/wechat-mp/list", WebPathType::Get, Some("微信公众号列表"), get(wechat_mp_list))
        .route("/wechat-mp/add", WebPathType::Post, Some("新增微信公众号"), post(wechat_mp_add))
        .route("/wechat-mp/edit", WebPathType::Put, Some("编辑微信公众号"), put(wechat_mp_edit))
        .route("/wechat-mp/del", WebPathType::Delete, Some("删除微信公众号"), delete(wechat_mp_del))
        .route("/wechat-mp/toggle", WebPathType::Put, Some("切换微信公众号状态"), put(wechat_mp_toggle))
        .route("/wechat-mini/list", WebPathType::Get, Some("微信小程序列表"), get(wechat_mini_list))
        .route("/wechat-mini/add", WebPathType::Post, Some("新增微信小程序"), post(wechat_mini_add))
        .route("/wechat-mini/edit", WebPathType::Put, Some("编辑微信小程序"), put(wechat_mini_edit))
        .route("/wechat-mini/del", WebPathType::Delete, Some("删除微信小程序"), delete(wechat_mini_del))
        .route("/wechat-mini/toggle", WebPathType::Put, Some("切换微信小程序状态"), put(wechat_mini_toggle))
        .route("/oss-config/list", WebPathType::Get, Some("OSS配置列表"), get(oss_config_list))
        .route("/oss-config/add", WebPathType::Post, Some("新增OSS配置"), post(oss_config_add))
        .route("/oss-config/edit", WebPathType::Put, Some("编辑OSS配置"), put(oss_config_edit))
        .route("/oss-config/del", WebPathType::Delete, Some("删除OSS配置"), delete(oss_config_del))
        .route("/oss-config/toggle", WebPathType::Put, Some("切换OSS配置状态"), put(oss_config_toggle))
        .route("/logistics-company/list", WebPathType::Get, Some("物流公司列表"), get(logistics_company_list))
        .route("/logistics-company/add", WebPathType::Post, Some("新增物流公司"), post(logistics_company_add))
        .route("/logistics-company/edit", WebPathType::Put, Some("编辑物流公司"), put(logistics_company_edit))
        .route("/logistics-company/del", WebPathType::Delete, Some("删除物流公司"), delete(logistics_company_del))
        .route("/logistics-company/toggle", WebPathType::Put, Some("切换物流公司状态"), put(logistics_company_toggle))
}
