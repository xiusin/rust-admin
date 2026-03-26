use crate::domain::entity::p_plugin_verification_code;
use crate::domain::entity::p_plugin_verification_code::Entity as PluginVerifyCodeEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use sea_orm::prelude::Expr;
use chrono::{Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub async fn send(license_id: i64, _user_id: i64, _plugin_id: i64, _purpose: String, _device_hash: Option<String>) -> Result<String, Error> {
    let db = DB().await;

    PluginVerifyCodeEntity::update_many()
        .col_expr(p_plugin_verification_code::Column::Status, Expr::value(2))
        .filter(p_plugin_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_plugin_verification_code::Column::Status.eq(0))
        .exec(db)
        .await?;

    let code = generate_random_code(6);

    let max_id: Option<i64> = PluginVerifyCodeEntity::find()
        .select_only()
        .column_as(p_plugin_verification_code::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;
    let now = Utc::now().naive_utc();
    let expire_time = now + Duration::minutes(5);

    let verify_code = p_plugin_verification_code::ActiveModel {
        id: Set(new_id),
        code: Set(code.clone()),
        license_id: Set(license_id),
        user_id: Set(0),
        plugin_id: Set(0),
        purpose: Set(0),
        device_hash: Set(None),
        status: Set(0),
        attempts: Set(0),
        expire_time: Set(expire_time),
        verified_at: Set(None),
        created_at: Set(Some(now)),
    };

    verify_code.insert(db).await?;
    Ok(code)
}

fn generate_random_code(length: usize) -> String {
    let mut rng = rand::rng();
    let code: u32 = rng.random_range(10_u32.pow(length as u32 - 1)..10_u32.pow(length as u32));
    code.to_string()
}

#[derive(Debug, Deserialize)]
pub struct SendCodeParams {
    pub license_id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub purpose: String,
    pub device_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub success: bool,
    pub message: String,
}

pub async fn check(license_id: i64, code: String, _device_hash: Option<String>) -> Result<CheckResult, Error> {
    let db = DB().await;

    let verify_code = PluginVerifyCodeEntity::find()
        .filter(p_plugin_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_plugin_verification_code::Column::Code.eq(&code))
        .one(db)
        .await?
        .ok_or_else(|| Error::bad_request("验证码错误"))?;

    if verify_code.status != 0 {
        return Ok(CheckResult {
            success: false,
            message: "验证码已失效".to_string(),
        });
    }

    let now = Utc::now().naive_utc();
    if verify_code.expire_time < now {
        return Ok(CheckResult {
            success: false,
            message: "验证码已过期".to_string(),
        });
    }

    if verify_code.attempts >= 5 {
        let mut active_model: p_plugin_verification_code::ActiveModel = verify_code.clone().into();
        active_model.status = Set(3);
        active_model.update(db).await?;
        return Ok(CheckResult {
            success: false,
            message: "验证失败次数过多，验证码已封禁".to_string(),
        });
    }

    let mut active_model: p_plugin_verification_code::ActiveModel = verify_code.into();
    active_model.status = Set(1);
    active_model.verified_at = Set(Some(now));
    active_model.update(db).await?;

    Ok(CheckResult {
        success: true,
        message: "验证成功".to_string(),
    })
}

pub async fn block(license_id: i64, code: String) -> Result<(), Error> {
    let db = DB().await;

    let verify_code = PluginVerifyCodeEntity::find()
        .filter(p_plugin_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_plugin_verification_code::Column::Code.eq(code))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("验证码不存在"))?;

    let mut active_model: p_plugin_verification_code::ActiveModel = verify_code.into();
    active_model.status = Set(3);
    active_model.update(db).await?;

    Ok(())
}
