use crate::domain::model::m_verification::*;
use crate::application::plugin_market::license_service;
use crate::common::error::Error;

pub async fn verify_license(license_key: String, device_id: String, _device_name: String, timestamp: i64, sign: String) -> Result<LicenseVerifyResponse, Error> {
    let nonce = "";
    if !license_service::validate_sign(&license_key, &device_id, timestamp, nonce, &sign) {
        return Err(Error::unauthorized("签名验证失败"));
    }

    let now = chrono::Utc::now().timestamp();
    if (now - timestamp).abs() > 300 {
        return Err(Error::bad_request("请求已过期"));
    }

    let result = license_service::verify(license_key.clone(), device_id.clone()).await?;

    if !result.valid {
        return Ok(LicenseVerifyResponse {
            valid: false,
            license_key,
            plugin_id: 0,
            plugin_name: String::new(),
            plan_id: 0,
            plan_name: String::new(),
            expire_time: chrono::Utc::now().naive_utc(),
            max_devices: 0,
            features: vec![],
            message: result.message,
        });
    }

    Ok(LicenseVerifyResponse {
        valid: true,
        license_key,
        plugin_id: 0,
        plugin_name: String::new(),
        plan_id: 0,
        plan_name: String::new(),
        expire_time: chrono::Utc::now().naive_utc(),
        max_devices: 5,
        features: vec![],
        message: None,
    })
}

pub async fn heartbeat(license_key: String, device_id: String, timestamp: i64, sign: String) -> Result<HeartbeatResponse, Error> {
    let nonce = "";
    if !license_service::validate_sign(&license_key, &device_id, timestamp, nonce, &sign) {
        return Err(Error::unauthorized("签名验证失败"));
    }

    let now = chrono::Utc::now().timestamp();
    if (now - timestamp).abs() > 60 {
        return Err(Error::bad_request("请求已过期"));
    }

    let result = license_service::verify(license_key.clone(), device_id).await?;

    if !result.valid {
        return Ok(HeartbeatResponse {
            valid: false,
            next_check_time: 0,
            message: result.message,
        });
    }

    let next_check = now + 3600;

    Ok(HeartbeatResponse {
        valid: true,
        next_check_time: next_check,
        message: None,
    })
}

pub async fn get_obfuscation_config(plugin_id: i64) -> Result<ObfuscationConfig, Error> {
    Ok(ObfuscationConfig {
        plugin_id,
        version: "1.0.0".to_string(),
        obfuscation_level: 1,
        enabled_modules: vec!["license_check".to_string(), "device_verify".to_string()],
        license_check_interval: 3600,
        max_offline_days: 7,
        anti_debug_enabled: true,
        code_virtualization: false,
    })
}