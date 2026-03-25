use crate::domain::entity::p_verification_code::*;
use crate::domain::entity::p_verification_code::Entity as VerifyCodeEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use chrono::{Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub async fn generate(license_id: i64, user_id: i64, plugin_id: i64, purpose: i32, device_hash: Option<String>) -> Result<String, Error> {
    let db = DB().await;

    VerifyCodeEntity::update_many()
        .col_expr(p_verification_code::Column::Status, Value::Int(Some(2)))
        .filter(p_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_verification_code::Column::Status.eq(0))
        .exec(db)
        .await?;

    let code = generate_random_code(6);

    let max_id: Option<i64> = VerifyCodeEntity::find()
        .select_only()
        .column_as(p_verification_code::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;
    let now = Utc::now().naive_utc();
    let expire_time = now + Duration::minutes(5);

    let verify_code = p_verification_code::ActiveModel {
        id: Set(new_id),
        code: Set(code.clone()),
        license_id: Set(license_id),
        user_id: Set(user_id),
        plugin_id: Set(plugin_id),
        purpose: Set(purpose),
        device_hash: Set(device_hash),
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
    let mut rng = rand::thread_rng();
    let code: u32 = rng.random_range(10_u32.pow(length as u32 - 1)..10_u32.pow(length as u32));
    code.to_string()
}

pub async fn send(license_id: i64, user_id: i64, plugin_id: i64, purpose: i32, device_hash: Option<String>) -> Result<String, Error> {
    generate(license_id, user_id, plugin_id, purpose, device_hash).await
}

pub async fn check(license_id: i64, code: String, device_hash: Option<String>) -> Result<CheckResult, Error> {
    let db = DB().await;

    let verify_code = VerifyCodeEntity::find()
        .filter(p_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_verification_code::Column::Code.eq(code.clone()))
        .filter(p_verification_code::Column::Status.eq(0))
        .one(db)
        .await?
        .ok_or_else(|| Error::bad_request("验证码不存在或已失效"))?;

    if verify_code.expire_time < Utc::now().naive_utc() {
        let mut active_model: p_verification_code::ActiveModel = verify_code.clone().into();
        active_model.status = Set(2);
        active_model.update(db).await?;
        return Err(Error::bad_request("验证码已过期"));
    }

    if verify_code.attempts >= 3 {
        let mut active_model: p_verification_code::ActiveModel = verify_code.clone().into();
        active_model.status = Set(3);
        active_model.update(db).await?;
        return Err(Error::bad_request("验证码尝试次数过多，已被封禁"));
    }

    let mut active_model: p_verification_code::ActiveModel = verify_code.clone().into();
    active_model.attempts = Set(verify_code.attempts + 1);
    active_model.verified_at = Set(Some(Utc::now().naive_utc()));
    active_model.status = Set(1);
    active_model.update(db).await?;

    Ok(CheckResult {
        success: true,
        message: "验证成功".to_string(),
    })
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub success: bool,
    pub message: String,
}

pub async fn block(license_id: i64, code: String) -> Result<(), Error> {
    let db = DB().await;

    let verify_code = VerifyCodeEntity::find()
        .filter(p_verification_code::Column::LicenseId.eq(license_id))
        .filter(p_verification_code::Column::Code.eq(code))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("验证码不存在"))?;

    let mut active_model: p_verification_code::ActiveModel = verify_code.into();
    active_model.status = Set(3);
    active_model.update(db).await?;

    Ok(())
}