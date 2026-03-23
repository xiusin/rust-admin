use crate::common::error::Error;
use crate::domain::entity::prelude::*;
use crate::domain::entity::{c_sms_config, c_sms_template, c_oss_config, c_wechat_mp_config, c_wechat_mini_config, c_logistics_company};
use crate::model::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsConfigRes {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub access_secret: String,
    pub sign_name: String,
    pub region: Option<String>,
    pub is_default: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmsConfigAdd {
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub access_secret: String,
    pub sign_name: String,
    pub region: Option<String>,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmsConfigEdit {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub access_secret: String,
    pub sign_name: String,
    pub region: Option<String>,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsTemplateRes {
    pub id: i64,
    pub name: String,
    pub template_code: String,
    pub template_type: String,
    pub content: Option<String>,
    pub params: Option<serde_json::Value>,
    pub status: i32,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmsTemplateAdd {
    pub name: String,
    pub template_code: String,
    pub template_type: String,
    pub content: Option<String>,
    pub params: Option<serde_json::Value>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmsTemplateEdit {
    pub id: i64,
    pub name: String,
    pub template_code: String,
    pub template_type: String,
    pub content: Option<String>,
    pub params: Option<serde_json::Value>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatMpConfigRes {
    pub id: i64,
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub mp_type: String,
    pub verified: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatMpConfigAdd {
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub mp_type: Option<String>,
    pub verified: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatMpConfigEdit {
    pub id: i64,
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub mp_type: Option<String>,
    pub verified: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatMiniConfigRes {
    pub id: i64,
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub original_id: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatMiniConfigAdd {
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub original_id: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatMiniConfigEdit {
    pub id: i64,
    pub name: String,
    pub app_id: String,
    pub app_secret: String,
    pub original_id: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OssConfigRes {
    pub id: i64,
    pub name: String,
    pub storage_type: String,
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub domain: Option<String>,
    pub is_default: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OssConfigAdd {
    pub name: String,
    pub storage_type: String,
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub domain: Option<String>,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OssConfigEdit {
    pub id: i64,
    pub name: String,
    pub storage_type: String,
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub domain: Option<String>,
    pub is_default: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogisticsCompanyRes {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub logo: Option<String>,
    pub status: i32,
    pub sort: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogisticsCompanyAdd {
    pub name: String,
    pub code: String,
    pub logo: Option<String>,
    pub status: Option<i32>,
    pub sort: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogisticsCompanyEdit {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub logo: Option<String>,
    pub status: Option<i32>,
    pub sort: Option<i32>,
}

fn format_datetime(dt: Option<DateTime>) -> Option<String> {
    dt.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub async fn list_sms_config() -> Result<Vec<SmsConfigRes>> {
    let db = DB().await;
    let configs = CSmsConfig::find()
        .filter(c_sms_config::Column::DeletedAt.is_null())
        .order_by_desc(c_sms_config::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs.into_iter().map(|c| SmsConfigRes {
        id: c.id,
        name: c.name,
        provider: c.provider,
        access_key: c.access_key,
        access_secret: c.access_secret,
        sign_name: c.sign_name,
        region: c.region,
        is_default: c.is_default,
        status: c.status,
        remark: c.remark,
        created_at: format_datetime(c.created_at),
    }).collect())
}

pub async fn add_sms_config(arg: SmsConfigAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let config = c_sms_config::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        provider: Set(arg.provider),
        access_key: Set(arg.access_key),
        access_secret: Set(arg.access_secret),
        sign_name: Set(arg.sign_name),
        region: Set(arg.region),
        is_default: Set(arg.is_default.unwrap_or(0)),
        status: Set(arg.status.unwrap_or(1)),
        remark: Set(arg.remark),
        ..Default::default()
    };

    CSmsConfig::insert(config)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_sms_config(arg: SmsConfigEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CSmsConfig::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_sms_config::ActiveModel = config.into();
    active_model.name = Set(arg.name);
    active_model.provider = Set(arg.provider);
    active_model.access_key = Set(arg.access_key);
    active_model.access_secret = Set(arg.access_secret);
    active_model.sign_name = Set(arg.sign_name);
    active_model.region = Set(arg.region);
    active_model.is_default = Set(arg.is_default.unwrap_or(0));
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.remark = Set(arg.remark);

    CSmsConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_sms_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CSmsConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_sms_config::ActiveModel = config.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    CSmsConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn toggle_sms_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CSmsConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_sms_config::ActiveModel = config.into();
    active_model.status = Set(if active_model.status.unwrap() == 1 { 0 } else { 1 });

    CSmsConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list_sms_template() -> Result<Vec<SmsTemplateRes>> {
    let db = DB().await;
    let templates = CSmsTemplate::find()
        .filter(c_sms_template::Column::DeletedAt.is_null())
        .order_by_desc(c_sms_template::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(templates.into_iter().map(|t| SmsTemplateRes {
        id: t.id,
        name: t.name,
        template_code: t.template_code,
        template_type: t.template_type,
        content: t.content,
        params: t.params,
        status: t.status,
        remark: t.remark,
        created_at: format_datetime(t.created_at),
    }).collect())
}

pub async fn add_sms_template(arg: SmsTemplateAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let template = c_sms_template::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        template_code: Set(arg.template_code),
        template_type: Set(arg.template_type),
        content: Set(arg.content),
        params: Set(arg.params),
        status: Set(arg.status.unwrap_or(1)),
        remark: Set(arg.remark),
        ..Default::default()
    };

    CSmsTemplate::insert(template)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_sms_template(arg: SmsTemplateEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let template = CSmsTemplate::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("模板不存在"))?;

    let mut active_model: c_sms_template::ActiveModel = template.into();
    active_model.name = Set(arg.name);
    active_model.template_code = Set(arg.template_code);
    active_model.template_type = Set(arg.template_type);
    active_model.content = Set(arg.content);
    active_model.params = Set(arg.params);
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.remark = Set(arg.remark);

    CSmsTemplate::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_sms_template(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let template = CSmsTemplate::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("模板不存在"))?;

    let mut active_model: c_sms_template::ActiveModel = template.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    CSmsTemplate::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn list_wechat_mp_config() -> Result<Vec<WechatMpConfigRes>> {
    let db = DB().await;
    let configs = CWechatMpConfig::find()
        .filter(c_wechat_mp_config::Column::DeletedAt.is_null())
        .order_by_desc(c_wechat_mp_config::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs.into_iter().map(|c| WechatMpConfigRes {
        id: c.id,
        name: c.name,
        app_id: c.app_id,
        app_secret: c.app_secret,
        token: c.token,
        encoding_aes_key: c.encoding_aes_key,
        mp_type: c.mp_type,
        verified: c.verified,
        status: c.status,
        remark: c.remark,
        created_at: format_datetime(c.created_at),
    }).collect())
}

pub async fn add_wechat_mp_config(arg: WechatMpConfigAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let config = c_wechat_mp_config::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        app_id: Set(arg.app_id),
        app_secret: Set(arg.app_secret),
        token: Set(arg.token),
        encoding_aes_key: Set(arg.encoding_aes_key),
        mp_type: Set(arg.mp_type.unwrap_or_else(|| "service".to_string())),
        verified: Set(arg.verified.unwrap_or(0)),
        status: Set(arg.status.unwrap_or(1)),
        remark: Set(arg.remark),
        ..Default::default()
    };

    CWechatMpConfig::insert(config)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_wechat_mp_config(arg: WechatMpConfigEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMpConfig::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mp_config::ActiveModel = config.into();
    active_model.name = Set(arg.name);
    active_model.app_id = Set(arg.app_id);
    active_model.app_secret = Set(arg.app_secret);
    active_model.token = Set(arg.token);
    active_model.encoding_aes_key = Set(arg.encoding_aes_key);
    active_model.mp_type = Set(arg.mp_type.unwrap_or_else(|| "service".to_string()));
    active_model.verified = Set(arg.verified.unwrap_or(0));
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.remark = Set(arg.remark);

    CWechatMpConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_wechat_mp_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMpConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mp_config::ActiveModel = config.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    CWechatMpConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn toggle_wechat_mp_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMpConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mp_config::ActiveModel = config.into();
    active_model.status = Set(if active_model.status.unwrap() == 1 { 0 } else { 1 });

    CWechatMpConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list_wechat_mini_config() -> Result<Vec<WechatMiniConfigRes>> {
    let db = DB().await;
    let configs = CWechatMiniConfig::find()
        .filter(c_wechat_mini_config::Column::DeletedAt.is_null())
        .order_by_desc(c_wechat_mini_config::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs.into_iter().map(|c| WechatMiniConfigRes {
        id: c.id,
        name: c.name,
        app_id: c.app_id,
        app_secret: c.app_secret,
        original_id: c.original_id,
        status: c.status,
        remark: c.remark,
        created_at: format_datetime(c.created_at),
    }).collect())
}

pub async fn add_wechat_mini_config(arg: WechatMiniConfigAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let config = c_wechat_mini_config::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        app_id: Set(arg.app_id),
        app_secret: Set(arg.app_secret),
        original_id: Set(arg.original_id),
        status: Set(arg.status.unwrap_or(1)),
        remark: Set(arg.remark),
        ..Default::default()
    };

    CWechatMiniConfig::insert(config)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_wechat_mini_config(arg: WechatMiniConfigEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMiniConfig::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mini_config::ActiveModel = config.into();
    active_model.name = Set(arg.name);
    active_model.app_id = Set(arg.app_id);
    active_model.app_secret = Set(arg.app_secret);
    active_model.original_id = Set(arg.original_id);
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.remark = Set(arg.remark);

    CWechatMiniConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_wechat_mini_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMiniConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mini_config::ActiveModel = config.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    CWechatMiniConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn toggle_wechat_mini_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CWechatMiniConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_wechat_mini_config::ActiveModel = config.into();
    active_model.status = Set(if active_model.status.unwrap() == 1 { 0 } else { 1 });

    CWechatMiniConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list_oss_config() -> Result<Vec<OssConfigRes>> {
    let db = DB().await;
    let configs = COssConfig::find()
        .filter(c_oss_config::Column::DeletedAt.is_null())
        .order_by_desc(c_oss_config::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs.into_iter().map(|c| OssConfigRes {
        id: c.id,
        name: c.name,
        storage_type: c.storage_type,
        endpoint: c.endpoint,
        bucket: c.bucket,
        access_key: c.access_key,
        secret_key: c.secret_key,
        region: c.region,
        domain: c.domain,
        is_default: c.is_default,
        status: c.status,
        remark: c.remark,
        created_at: format_datetime(c.created_at),
    }).collect())
}

pub async fn add_oss_config(arg: OssConfigAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let config = c_oss_config::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        storage_type: Set(arg.storage_type),
        endpoint: Set(arg.endpoint),
        bucket: Set(arg.bucket),
        access_key: Set(arg.access_key),
        secret_key: Set(arg.secret_key),
        region: Set(arg.region),
        domain: Set(arg.domain),
        is_default: Set(arg.is_default.unwrap_or(0)),
        status: Set(arg.status.unwrap_or(1)),
        remark: Set(arg.remark),
        ..Default::default()
    };

    COssConfig::insert(config)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_oss_config(arg: OssConfigEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let config = COssConfig::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_oss_config::ActiveModel = config.into();
    active_model.name = Set(arg.name);
    active_model.storage_type = Set(arg.storage_type);
    active_model.endpoint = Set(arg.endpoint);
    active_model.bucket = Set(arg.bucket);
    active_model.access_key = Set(arg.access_key);
    active_model.secret_key = Set(arg.secret_key);
    active_model.region = Set(arg.region);
    active_model.domain = Set(arg.domain);
    active_model.is_default = Set(arg.is_default.unwrap_or(0));
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.remark = Set(arg.remark);

    COssConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_oss_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = COssConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_oss_config::ActiveModel = config.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    COssConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn toggle_oss_config(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let config = COssConfig::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("配置不存在"))?;

    let mut active_model: c_oss_config::ActiveModel = config.into();
    active_model.status = Set(if active_model.status.unwrap() == 1 { 0 } else { 1 });

    COssConfig::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list_logistics_company() -> Result<Vec<LogisticsCompanyRes>> {
    let db = DB().await;
    let companies = CLogisticsCompany::find()
        .filter(c_logistics_company::Column::DeletedAt.is_null())
        .order_by_asc(c_logistics_company::Column::Sort)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(companies.into_iter().map(|c| LogisticsCompanyRes {
        id: c.id,
        name: c.name,
        code: c.code,
        logo: c.logo,
        status: c.status,
        sort: c.sort,
        created_at: format_datetime(c.created_at),
    }).collect())
}

pub async fn add_logistics_company(arg: LogisticsCompanyAdd) -> Result<i64> {
    let db = DB_WRITE().await;
    let id = generate_id();
    
    let company = c_logistics_company::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        code: Set(arg.code),
        logo: Set(arg.logo),
        status: Set(arg.status.unwrap_or(1)),
        sort: Set(arg.sort.unwrap_or(0)),
        ..Default::default()
    };

    CLogisticsCompany::insert(company)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(id)
}

pub async fn edit_logistics_company(arg: LogisticsCompanyEdit) -> Result<()> {
    let db = DB_WRITE().await;

    let company = CLogisticsCompany::find_by_id(arg.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("公司不存在"))?;

    let mut active_model: c_logistics_company::ActiveModel = company.into();
    active_model.name = Set(arg.name);
    active_model.code = Set(arg.code);
    active_model.logo = Set(arg.logo);
    active_model.status = Set(arg.status.unwrap_or(1));
    active_model.sort = Set(arg.sort.unwrap_or(0));

    CLogisticsCompany::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_logistics_company(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let company = CLogisticsCompany::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("公司不存在"))?;

    let mut active_model: c_logistics_company::ActiveModel = company.into();
    active_model.deleted_at = Set(Some(chrono::Local::now().naive_local()));

    CLogisticsCompany::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn toggle_logistics_company(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let company = CLogisticsCompany::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("公司不存在"))?;

    let mut active_model: c_logistics_company::ActiveModel = company.into();
    active_model.status = Set(if active_model.status.unwrap() == 1 { 0 } else { 1 });

    CLogisticsCompany::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}
