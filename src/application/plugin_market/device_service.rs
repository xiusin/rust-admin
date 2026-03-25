use crate::domain::entity::p_device::*;
use crate::domain::entity::p_device::Entity as DeviceEntity;
use crate::domain::entity::p_license::Entity as LicenseEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub async fn register(license_id: i64, params: RegisterDeviceParams) -> Result<i64, Error> {
    let db = DB().await;

    let license = LicenseEntity::find_by_id(license_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("License不存在"))?;

    if license.status != 1 {
        return Err(Error::bad_request("License未激活"));
    }

    let existing = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(license_id))
        .filter(p_device::Column::DeviceId.eq(params.device_id.clone()))
        .one(db)
        .await?;

    if let Some(d) = existing {
        let mut active_model: p_device::ActiveModel = d.into();
        active_model.last_active_time = Set(Some(Utc::now().naive_utc()));
        active_model.status = Set(1);
        active_model.update(db).await?;
        return Ok(d.id);
    }

    let device_count = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(license_id))
        .count(db)
        .await?;

    if device_count >= license.max_devices as i64 {
        return Err(Error::bad_request("设备数量已达上限"));
    }

    let max_id: Option<i64> = DeviceEntity::find()
        .select_only()
        .column_as(p_device::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let device = p_device::ActiveModel {
        id: Set(new_id),
        user_id: Set(license.user_id),
        license_id: Set(license_id),
        device_id: Set(params.device_id.clone()),
        device_name: Set(params.device_info.device_name),
        device_type: Set(params.device_info.device_type),
        os_version: Set(params.device_info.os_version),
        mac_address: Set(params.device_info.mac_address),
        ip_address: Set(params.ip_address),
        last_active_time: Set(Some(Utc::now().naive_utc())),
        status: Set(1),
        created_at: Set(Some(Utc::now().naive_utc())),
    };

    device.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceParams {
    pub device_id: String,
    pub device_info: DeviceInfo,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
}

pub async fn list(license_id: i64) -> Result<Vec<DeviceItem>, Error> {
    let db = DB().await;

    let devices = DeviceEntity::find()
        .filter(p_device::Column::LicenseId.eq(license_id))
        .order_by_desc(p_device::Column::LastActiveTime)
        .all(db)
        .await?;

    let items: Vec<DeviceItem> = devices
        .into_iter()
        .map(|d| DeviceItem {
            id: d.id,
            device_id: d.device_id.clone(),
            device_name: d.device_name,
            device_type: d.device_type,
            os_version: d.os_version,
            last_active_time: d.last_active_time,
            status: d.status,
        })
        .collect();

    Ok(items)
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceItem {
    pub id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub last_active_time: Option<chrono::NaiveDateTime>,
    pub status: i32,
}

pub async fn update_status(device_id: String, status: i32) -> Result<(), Error> {
    let db = DB().await;

    let device = DeviceEntity::find()
        .filter(p_device::Column::DeviceId.eq(device_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("设备不存在"))?;

    let mut active_model: p_device::ActiveModel = device.into();
    active_model.status = Set(status);
    active_model.update(db).await?;

    Ok(())
}

pub async fn heartbeat(device_id: String) -> Result<(), Error> {
    let db = DB().await;

    let device = DeviceEntity::find()
        .filter(p_device::Column::DeviceId.eq(device_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("设备不存在"))?;

    let mut active_model: p_device::ActiveModel = device.into();
    active_model.last_active_time = Set(Some(Utc::now().naive_utc()));
    active_model.status = Set(1);
    active_model.update(db).await?;

    Ok(())
}