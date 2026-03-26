use crate::domain::entity::p_license;
use crate::domain::entity::p_license::Entity as LicenseEntity;
use crate::domain::entity::p_device;
use crate::domain::entity::p_device::Entity as DeviceEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use chrono::{Duration, Utc};
use uuid::Uuid;
use md5;
use serde::{Deserialize, Serialize};

pub async fn generate(user_id: i64, plugin_id: i64, plan_id: i64, order_id: Option<i64>) -> Result<String, Error> {
    let db = DB().await;

    let license_key = Uuid::new_v4().to_string();

    let max_id: Option<i64> = LicenseEntity::find()
        .select_only()
        .column_as(p_license::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;
    let now = Utc::now().naive_utc();
    let end_time = now + Duration::days(365);

    let license = p_license::ActiveModel {
        id: Set(new_id),
        license_key: Set(license_key.clone()),
        user_id: Set(user_id),
        plugin_id: Set(plugin_id),
        plan_id: Set(plan_id),
        order_id: Set(order_id),
        device_id: Set(String::new()),
        device_name: Set(None),
        device_type: Set(None),
        os_version: Set(None),
        app_version: Set(None),
        ip_address: Set(None),
        bind_count: Set(0),
        max_devices: Set(5),
        verify_count: Set(0),
        last_verify_time: Set(None),
        status: Set(1),
        start_time: Set(now),
        end_time: Set(end_time),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    license.insert(db).await?;
    Ok(license_key)
}

pub async fn list(user_id: i64, page_num: u32, page_size: u32) -> Result<(Vec<LicenseItem>, i64), Error> {
    let db = DB().await;
    let offset = (page_num - 1) * page_size;

    let total = LicenseEntity::find()
        .filter(p_license::Column::UserId.eq(user_id))
        .count(db)
        .await?;

    let licenses = LicenseEntity::find()
        .filter(p_license::Column::UserId.eq(user_id))
        .order_by_desc(p_license::Column::CreatedAt)
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let items: Vec<LicenseItem> = licenses
        .into_iter()
        .map(|l| LicenseItem {
            id: l.id,
            license_key: l.license_key.clone(),
            user_id: l.user_id,
            plugin_id: l.plugin_id,
            plan_id: l.plan_id,
            device_id: l.device_id.clone(),
            status: l.status,
            start_time: l.start_time,
            end_time: l.end_time,
            verify_count: l.verify_count,
            max_devices: l.max_devices,
            created_at: l.created_at,
        })
        .collect();

    Ok((items, total as i64))
}

#[derive(Debug, Serialize, Clone)]
pub struct LicenseItem {
    pub id: i64,
    pub license_key: String,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub device_id: String,
    pub status: i32,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub verify_count: i32,
    pub max_devices: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub async fn detail(id: i64) -> Result<Option<p_license::Model>, Error> {
    let db = DB().await;
    let license = LicenseEntity::find_by_id(id).one(db).await?;
    Ok(license)
}

pub async fn bind_device(id: i64, params: BindDeviceParams) -> Result<(), Error> {
    let db = DB().await;

    let license = LicenseEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    if license.status != 1 {
        return Err(Error::bad_request("License未激活"));
    }

    let device_count = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(id))
        .count(db)
        .await?;

    if device_count >= license.max_devices as u64 {
        return Err(Error::bad_request("设备数量已达上限"));
    }

    let existing_device = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(id))
        .filter(p_device::Column::DeviceId.eq(params.device_id.clone()))
        .one(db)
        .await?;

    if existing_device.is_some() {
        return Ok(());
    }

    let max_id: Option<i64> = DeviceEntity::find()
        .select_only()
        .column_as(p_device::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let device_name_clone = params.device_name.clone();
    let device_type_clone = params.device_type.clone();
    let device_id_clone = params.device_id.clone();

    let device = p_device::ActiveModel {
        id: Set(new_id),
        user_id: Set(license.user_id),
        license_id: Set(id),
        device_id: Set(params.device_id.clone()),
        device_name: Set(params.device_name),
        device_type: Set(params.device_type),
        os_version: Set(params.os_version),
        mac_address: Set(params.mac_address),
        ip_address: Set(params.ip_address),
        last_active_time: Set(Some(Utc::now().naive_utc())),
        status: Set(1),
        created_at: Set(Some(Utc::now().naive_utc())),
    };

    device.insert(db).await?;

    let bind_count = license.bind_count;
    let mut active_model: p_license::ActiveModel = license.into();
    active_model.bind_count = Set(bind_count + 1);
    active_model.device_id = Set(device_id_clone);
    if device_name_clone.is_some() {
        active_model.device_name = Set(device_name_clone);
    }
    if device_type_clone.is_some() {
        active_model.device_type = Set(device_type_clone);
    }
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct BindDeviceParams {
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
}

pub async fn unbind_device(id: i64, device_id: String) -> Result<(), Error> {
    let db = DB().await;

    let license = LicenseEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    let device = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(id))
        .filter(p_device::Column::DeviceId.eq(device_id.clone()))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("设备不存在"))?;

    let active_model: p_device::ActiveModel = device.into();
    active_model.delete(db).await?;

    let bind_count = license.bind_count;
    let mut active_model: p_license::ActiveModel = license.into();
    active_model.bind_count = Set((bind_count - 1).max(0));
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn renew(id: i64, extend_days: i32) -> Result<(), Error> {
    let db = DB().await;

    let license = LicenseEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    let new_end_time = license.end_time + Duration::days(extend_days as i64);

    let mut active_model: p_license::ActiveModel = license.into();
    active_model.end_time = Set(new_end_time);
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn revoke(id: i64) -> Result<(), Error> {
    let db = DB().await;

    let license = LicenseEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    let mut active_model: p_license::ActiveModel = license.into();
    active_model.status = Set(0);
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn verify(license_key: String, device_id: String) -> Result<VerifyResult, Error> {
    let db = DB().await;

    let license = LicenseEntity::find()
        .filter(p_license::Column::LicenseKey.eq(license_key.clone()))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    if license.status != 1 {
        return Ok(VerifyResult {
            valid: false,
            message: Some("License已被禁用".to_string()),
        });
    }

    if license.end_time < Utc::now().naive_utc() {
        return Ok(VerifyResult {
            valid: false,
            message: Some("License已过期".to_string()),
        });
    }

    if !license.device_id.is_empty() && license.device_id != device_id {
        return Ok(VerifyResult {
            valid: false,
            message: Some("设备ID不匹配".to_string()),
        });
    }

    let mut active_model: p_license::ActiveModel = license.clone().into();
    active_model.verify_count = Set(license.verify_count + 1);
    active_model.last_verify_time = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(VerifyResult {
        valid: true,
        message: None,
    })
}

#[derive(Debug, Serialize)]
pub struct VerifyResult {
    pub valid: bool,
    pub message: Option<String>,
}

pub fn generate_verify_sign(license_key: &str, device_id: &str, timestamp: i64, nonce: &str) -> String {
    let data = format!("{}:{}:{}:{}", license_key, device_id, timestamp, nonce);
    let key = md5::compute(license_key.as_bytes());
    format!("{:x}{}", key, data)
}

pub fn validate_sign(license_key: &str, device_id: &str, timestamp: i64, nonce: &str, sign: &str) -> bool {
    let expected = generate_verify_sign(license_key, device_id, timestamp, nonce);
    expected == sign
}