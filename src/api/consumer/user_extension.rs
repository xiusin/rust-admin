use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::application::consumer::oauth_service;
use crate::application::consumer::level_service;
use crate::application::consumer::tag_service;
use crate::application::consumer::ban_service;
use crate::application::consumer::statistics_service;
use crate::application::consumer::consumer_service;
use crate::domain::model::m_user_extension::*;
use crate::api::web_path::{WebPath, WebPathType};
use crate::model::prelude::DateTime;
use axum::routing::{delete, get, post, put};
use axum::{
    extract::Query,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use rust_decimal::Decimal;

fn validate_phone(phone: &str) -> bool {
    phone.len() == 11 && phone.starts_with('1') && phone[1..].chars().all(|c| c.is_ascii_digit())
}

pub async fn bind_oauth(
    Json(args): Json<BindOauthArgs>,
) -> Result<Json<ApiResponse<UserOauthModel>>, Error> {
    args.validate()?;
    let params = BindOauthParams {
        consumer_id: args.consumer_id,
        oauth_type: args.oauth_type,
        oauth_id: args.oauth_id,
        oauth_name: args.oauth_name,
        oauth_avatar: args.oauth_avatar,
        oauth_token: args.oauth_token,
        refresh_token: args.refresh_token,
        token_expires_at: args.token_expires_at,
    };
    let result = oauth_service::bind_oauth(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn unbind_oauth(
    Json(args): Json<UnbindOauthArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    let params = UnbindOauthParams {
        consumer_id: args.consumer_id,
        oauth_type: args.oauth_type,
    };
    oauth_service::unbind_oauth(params).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn list_oauth(
    Query(args): Query<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<Vec<UserOauthModel>>>, Error> {
    let result = oauth_service::list_oauth(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn set_primary_bind(
    Json(args): Json<SetPrimaryBindArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    oauth_service::set_primary_bind(args.consumer_id, args.oauth_id).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_user_level(
    Query(args): Query<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<UserLevelModel>>, Error> {
    let result = level_service::get_user_level(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn add_exp(
    Json(args): Json<AddExpArgs>,
) -> Result<Json<ApiResponse<UserLevelModel>>, Error> {
    let params = AddExpParams {
        consumer_id: args.consumer_id,
        exp: args.exp,
        source: args.source,
        source_id: args.source_id,
        remark: args.remark,
    };
    let result = level_service::add_exp(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_level_configs() -> Result<Json<ApiResponse<Vec<LevelConfigModel>>>, Error> {
    let result = level_service::get_level_configs().await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_level_records(
    Query(args): Query<LevelRecordListArgs>,
) -> Result<Json<ApiResponse<LevelRecordListResp>>, Error> {
    let params = LevelRecordListParams {
        consumer_id: args.consumer_id,
        page_num: args.page_num,
        page_size: args.page_size,
    };
    let (list, total) = level_service::get_level_records(params).await?;
    Ok(Json(ApiResponse::success(LevelRecordListResp { list, total })))
}

pub async fn create_level_config(
    Json(args): Json<CreateLevelConfigArgs>,
) -> Result<Json<ApiResponse<LevelConfigModel>>, Error> {
    let result = level_service::create_level_config(
        args.level,
        args.level_name,
        args.min_exp,
        args.max_exp,
        args.icon,
        args.color,
        args.discount_rate,
        args.privileges,
    ).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_tag(
    Json(args): Json<CreateTagArgs>,
) -> Result<Json<ApiResponse<UserTagModel>>, Error> {
    args.validate()?;
    let params = CreateTagParams {
        name: args.name,
        tag_type: args.tag_type,
        category: args.category,
        color: args.color,
        icon: args.icon,
        description: args.description,
        created_by: args.created_by,
    };
    let result = tag_service::create_tag(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn update_tag(
    Json(args): Json<UpdateTagArgs>,
) -> Result<Json<ApiResponse<UserTagModel>>, Error> {
    let result = tag_service::update_tag(
        args.id,
        args.name,
        args.category,
        args.color,
        args.icon,
        args.description,
    ).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn delete_tag(
    Query(args): Query<TagIdArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    tag_service::delete_tag(args.id).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn list_tags(
    Query(args): Query<UserTagListArgs>,
) -> Result<Json<ApiResponse<UserTagListResp>>, Error> {
    let params = UserTagListParams {
        page_num: args.page_num,
        page_size: args.page_size,
        tag_type: args.tag_type,
        category: args.category,
        name: args.name,
    };
    let (list, total) = tag_service::list_tags(params).await?;
    Ok(Json(ApiResponse::success(UserTagListResp { list, total })))
}

pub async fn add_user_tags(
    Json(args): Json<AddUserTagArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    let params = AddUserTagParams {
        consumer_id: args.consumer_id,
        tag_ids: args.tag_ids,
        source: args.source,
        source_desc: args.source_desc,
    };
    tag_service::add_user_tags(params).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn remove_user_tags(
    Json(args): Json<RemoveUserTagArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    let params = RemoveUserTagParams {
        consumer_id: args.consumer_id,
        tag_ids: args.tag_ids,
    };
    tag_service::remove_user_tags(params).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_user_tags(
    Query(args): Query<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<Vec<UserTagModel>>>, Error> {
    let result = tag_service::get_user_tags(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn ban_user(
    Json(args): Json<BanUserArgs>,
) -> Result<Json<ApiResponse<UserBanModel>>, Error> {
    args.validate()?;
    let params = BanUserParams {
        consumer_id: args.consumer_id,
        ban_type: args.ban_type,
        reason: args.reason,
        end_at: args.end_at,
        banned_by: args.banned_by,
    };
    let result = ban_service::ban_user(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn unban_user(
    Json(args): Json<UnbanUserArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    let params = UnbanUserParams {
        consumer_id: args.consumer_id,
        unban_reason: args.unban_reason,
        unban_by: args.unban_by,
    };
    ban_service::unban_user(params).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn check_ban_status(
    Query(args): Query<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<Option<UserBanModel>>>, Error> {
    let result = ban_service::check_ban_status(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_ban_history(
    Query(args): Query<BanListArgs>,
) -> Result<Json<ApiResponse<BanListResp>>, Error> {
    let params = BanListParams {
        page_num: args.page_num,
        page_size: args.page_size,
        consumer_id: args.consumer_id,
        status: args.status,
    };
    let (list, total) = ban_service::get_ban_history(params).await?;
    Ok(Json(ApiResponse::success(BanListResp { list, total })))
}

pub async fn get_statistics(
    Query(args): Query<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<ConsumerStatisticsModel>>, Error> {
    let result = statistics_service::get_statistics(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_consume_trend(
    Query(args): Query<ConsumeTrendArgs>,
) -> Result<Json<ApiResponse<Vec<ConsumeTrendItem>>>, Error> {
    let result = statistics_service::get_consume_trend(args.consumer_id, args.months.unwrap_or(12)).await?;
    Ok(Json(ApiResponse::success(result.into_iter().map(|(month, amount)| ConsumeTrendItem {
        month,
        amount: amount.to_string(),
    }).collect())))
}

pub async fn get_year_statistics(
    Query(args): Query<YearStatisticsArgs>,
) -> Result<Json<ApiResponse<ConsumerStatisticsModel>>, Error> {
    let result = statistics_service::get_year_statistics(args.consumer_id, args.year).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn update_phone(
    Json(args): Json<UpdatePhoneArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    args.validate()?;
    if !validate_phone(&args.phone) {
        return Err(Error::bad_request("手机号格式不正确"));
    }
    consumer_service::update_phone(args.consumer_id, args.phone).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn update_email(
    Json(args): Json<UpdateEmailArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    args.validate()?;
    consumer_service::update_email(args.consumer_id, args.email).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn reset_password(
    Json(args): Json<ResetPasswordArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    args.validate()?;
    consumer_service::reset_password(args.consumer_id, args.new_password).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn deactivate(
    Json(args): Json<ConsumerIdArgs>,
) -> Result<Json<ApiResponse<()>>, Error> {
    consumer_service::deactivate(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(())))
}

pub fn user_extension_api() -> WebPath {
    WebPath::new()
        .route("/oauth/bind", WebPathType::Post, Some("绑定第三方账号"), post(bind_oauth))
        .route("/oauth/unbind", WebPathType::Post, Some("解绑第三方账号"), post(unbind_oauth))
        .route("/oauth/list", WebPathType::Get, Some("获取绑定列表"), get(list_oauth))
        .route("/oauth/set-primary", WebPathType::Post, Some("设置主绑定"), post(set_primary_bind))
        .route("/level/get", WebPathType::Get, Some("获取用户等级"), get(get_user_level))
        .route("/level/add-exp", WebPathType::Post, Some("增加经验值"), post(add_exp))
        .route("/level/configs", WebPathType::Get, Some("等级配置列表"), get(get_level_configs))
        .route("/level/records", WebPathType::Get, Some("等级变更记录"), get(get_level_records))
        .route("/level/create-config", WebPathType::Post, Some("创建等级配置"), post(create_level_config))
        .route("/tag/create", WebPathType::Post, Some("创建标签"), post(create_tag))
        .route("/tag/update", WebPathType::Put, Some("更新标签"), put(update_tag))
        .route("/tag/delete", WebPathType::Delete, Some("删除标签"), delete(delete_tag))
        .route("/tag/list", WebPathType::Get, Some("标签列表"), get(list_tags))
        .route("/tag/add-user", WebPathType::Post, Some("给用户添加标签"), post(add_user_tags))
        .route("/tag/remove-user", WebPathType::Post, Some("移除用户标签"), post(remove_user_tags))
        .route("/tag/user-tags", WebPathType::Get, Some("获取用户标签"), get(get_user_tags))
        .route("/ban/user", WebPathType::Post, Some("禁用用户"), post(ban_user))
        .route("/ban/unban", WebPathType::Post, Some("解禁用户"), post(unban_user))
        .route("/ban/check", WebPathType::Get, Some("检查禁用状态"), get(check_ban_status))
        .route("/ban/history", WebPathType::Get, Some("禁用历史"), get(get_ban_history))
        .route("/statistics/get", WebPathType::Get, Some("获取消费统计"), get(get_statistics))
        .route("/statistics/trend", WebPathType::Get, Some("消费趋势"), get(get_consume_trend))
        .route("/statistics/year", WebPathType::Get, Some("年度统计"), get(get_year_statistics))
        .route("/update-phone", WebPathType::Post, Some("更新手机号"), post(update_phone))
        .route("/update-email", WebPathType::Post, Some("更新邮箱"), post(update_email))
        .route("/reset-password", WebPathType::Post, Some("重置密码"), post(reset_password))
        .route("/deactivate", WebPathType::Post, Some("注销账号"), post(deactivate))
}

#[derive(Debug, Deserialize)]
pub struct ConsumerIdArgs {
    pub consumer_id: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BindOauthArgs {
    pub consumer_id: i64,
    pub oauth_type: OAuthType,
    pub oauth_id: String,
    pub oauth_name: Option<String>,
    pub oauth_avatar: Option<String>,
    pub oauth_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UnbindOauthArgs {
    pub consumer_id: i64,
    pub oauth_type: OAuthType,
}

#[derive(Debug, Deserialize)]
pub struct SetPrimaryBindArgs {
    pub consumer_id: i64,
    pub oauth_id: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddExpArgs {
    pub consumer_id: i64,
    #[validate(range(min = 1, message = "经验值必须大于0"))]
    pub exp: i32,
    pub source: String,
    pub source_id: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LevelRecordListArgs {
    pub consumer_id: i64,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLevelConfigArgs {
    pub level: i32,
    #[validate(length(min = 1, max = 50, message = "等级名称1-50"))]
    pub level_name: String,
    pub min_exp: i32,
    pub max_exp: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub discount_rate: Decimal,
    pub privileges: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTagArgs {
    #[validate(length(min = 1, max = 50, message = "标签名称1-50"))]
    pub name: String,
    pub tag_type: String,
    pub category: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagArgs {
    pub id: i64,
    pub name: Option<String>,
    pub category: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TagIdArgs {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserTagListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub tag_type: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddUserTagArgs {
    pub consumer_id: i64,
    pub tag_ids: Vec<i64>,
    pub source: Option<String>,
    pub source_desc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveUserTagArgs {
    pub consumer_id: i64,
    pub tag_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BanUserArgs {
    pub consumer_id: i64,
    pub ban_type: String,
    #[validate(length(min = 1, max = 500, message = "原因长度1-500"))]
    pub reason: String,
    pub end_at: Option<DateTime>,
    pub banned_by: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UnbanUserArgs {
    pub consumer_id: i64,
    pub unban_reason: Option<String>,
    pub unban_by: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BanListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConsumeTrendArgs {
    pub consumer_id: i64,
    pub months: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct YearStatisticsArgs {
    pub consumer_id: i64,
    pub year: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePhoneArgs {
    pub consumer_id: i64,
    #[validate(length(min = 11, max = 11, message = "手机号必须为11位"))]
    pub phone: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmailArgs {
    pub consumer_id: i64,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordArgs {
    pub consumer_id: i64,
    #[validate(length(min = 6, max = 20, message = "密码长度6-20"))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct LevelRecordListResp {
    pub list: Vec<LevelRecordModel>,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct UserTagListResp {
    pub list: Vec<UserTagModel>,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct BanListResp {
    pub list: Vec<UserBanModel>,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct ConsumeTrendItem {
    pub month: String,
    pub amount: String,
}
