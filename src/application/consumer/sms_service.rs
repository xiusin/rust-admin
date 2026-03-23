use crate::common::error::Error;
use crate::domain::model::m_sms_log::*;
use crate::domain::entity::prelude::CSmsLog;
use crate::domain::entity::c_sms_log;
use crate::model::prelude::*;
use sea_orm::*;
use chrono::{Local, Duration};

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_code() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    hasher.write_i64(generate_id());
    format!("{:06}", hasher.finish() % 900000 + 100000)
}

const SMS_CODE_EXPIRE_MINUTES: i64 = 5;
const SMS_SEND_INTERVAL_SECONDS: i64 = 60;
const SMS_MAX_SEND_PER_DAY: u32 = 10;

pub async fn send_code(params: SendCodeParams) -> Result<String> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let one_minute_ago = now - Duration::seconds(SMS_SEND_INTERVAL_SECONDS);
    let recent = CSmsLog::find()
        .filter(c_sms_log::Column::Phone.eq(&params.phone))
        .filter(c_sms_log::Column::SmsType.eq(serde_json::to_string(&params.sms_type).unwrap_or_default()))
        .filter(c_sms_log::Column::SentAt.gte(one_minute_ago))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if recent.is_some() {
        return Err(Error::bad_request("发送频率过高，请稍后再试"));
    }

    let today_start = now.date().and_hms_opt(0, 0, 0).unwrap_or(now);
    let today_count = CSmsLog::find()
        .filter(c_sms_log::Column::Phone.eq(&params.phone))
        .filter(c_sms_log::Column::SentAt.gte(today_start))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if today_count >= SMS_MAX_SEND_PER_DAY as u64 {
        return Err(Error::bad_request("今日发送次数已达上限"));
    }

    let code = generate_code();
    let expires_at = now + Duration::minutes(SMS_CODE_EXPIRE_MINUTES);

    let log = c_sms_log::ActiveModel {
        id: Set(generate_id()),
        phone: Set(params.phone.clone()),
        sms_type: Set(serde_json::to_string(&params.sms_type).unwrap_or_default()),
        content: Set(Some(format!("您的验证码是：{}，有效期{}分钟", code, SMS_CODE_EXPIRE_MINUTES))),
        code: Set(Some(code.clone())),
        channel: Set("aliyun".to_string()),
        status: Set("sent".to_string()),
        error_message: Set(None),
        sent_at: Set(Some(now)),
        expires_at: Set(Some(expires_at)),
        ..Default::default()
    };

    CSmsLog::insert(log)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(code)
}

pub async fn verify_code(params: VerifyCodeParams) -> Result<bool> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let log = CSmsLog::find()
        .filter(
            Condition::all()
                .add(c_sms_log::Column::Phone.eq(&params.phone))
                .add(c_sms_log::Column::Code.eq(&params.code))
                .add(c_sms_log::Column::Status.eq("sent"))
        )
        .order_by_desc(c_sms_log::Column::SentAt)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match log {
        Some(l) => {
            if let Some(expires_at) = l.expires_at {
                if expires_at < now {
                    return Ok(false);
                }
            }

            let mut active_model: c_sms_log::ActiveModel = l.into();
            active_model.status = Set("verified".to_string());
            let _ = CSmsLog::update(active_model).exec(db).await;

            Ok(true)
        }
        None => Ok(false),
    }
}

pub async fn verify_code_with_type(
    phone: &str,
    code: &str,
    sms_type: SMSType,
) -> Result<bool> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let log = CSmsLog::find()
        .filter(
            Condition::all()
                .add(c_sms_log::Column::Phone.eq(phone))
                .add(c_sms_log::Column::Code.eq(code))
                .add(c_sms_log::Column::SmsType.eq(serde_json::to_string(&sms_type).unwrap_or_default()))
                .add(c_sms_log::Column::Status.eq("sent"))
        )
        .order_by_desc(c_sms_log::Column::SentAt)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match log {
        Some(l) => {
            if let Some(expires_at) = l.expires_at {
                if expires_at < now {
                    return Err(Error::bad_request("验证码已过期"));
                }
            }

            let mut active_model: c_sms_log::ActiveModel = l.into();
            active_model.status = Set("verified".to_string());
            let _ = CSmsLog::update(active_model).exec(db).await;

            Ok(true)
        }
        None => Err(Error::bad_request("验证码错误")),
    }
}

pub async fn mark_as_failed(log_id: i64, error_message: String) -> Result<()> {
    let db = DB_WRITE().await;

    let log = CSmsLog::find_by_id(log_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("短信记录不存在"))?;

    let mut active_model: c_sms_log::ActiveModel = log.into();
    active_model.status = Set("failed".to_string());
    active_model.error_message = Set(Some(error_message));

    CSmsLog::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn get_sms_statistics(phone: Option<String>) -> Result<SMSStatistics> {
    let db = DB_WRITE().await;

    let mut conditions = Condition::all();
    if let Some(p) = phone {
        conditions = conditions.add(c_sms_log::Column::Phone.eq(p));
    }

    let logs = CSmsLog::find()
        .filter(conditions)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut total_sent = 0u64;
    let mut total_success = 0u64;
    let mut total_failed = 0u64;
    let mut verification_count = 0u64;
    let mut notification_count = 0u64;

    for log in logs {
        total_sent += 1;
        match log.status.as_str() {
            "sent" | "verified" => total_success += 1,
            "failed" => total_failed += 1,
            _ => {}
        }
        if log.sms_type.contains("verification") {
            verification_count += 1;
        } else if log.sms_type.contains("notification") {
            notification_count += 1;
        }
    }

    Ok(SMSStatistics {
        total_sent,
        total_success,
        total_failed,
        verification_count,
        notification_count,
    })
}

pub async fn list_logs(
    params: SMSLogSearchParams,
) -> Result<(Vec<SMSLogModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(phone) = &params.phone {
        conditions = conditions.add(c_sms_log::Column::Phone.like(format!("%{}%", phone)));
    }
    if let Some(sms_type) = &params.sms_type {
        conditions = conditions.add(c_sms_log::Column::SmsType.like(format!("%{}%", sms_type)));
    }
    if let Some(status) = &params.status {
        conditions = conditions.add(c_sms_log::Column::Status.eq(status.clone()));
    }
    if let Some(start_time) = params.start_time {
        let naive = chrono::DateTime::from_timestamp(start_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_sms_log::Column::SentAt.gte(naive));
    }
    if let Some(end_time) = params.end_time {
        let naive = chrono::DateTime::from_timestamp(end_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_sms_log::Column::SentAt.lte(naive));
    }

    let paginator = CSmsLog::find()
        .filter(conditions)
        .order_by_desc(c_sms_log::Column::SentAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let logs = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<SMSLogModel> = logs
        .into_iter()
        .map(|l| SMSLogModel {
            id: l.id,
            phone: l.phone,
            sms_type: l.sms_type,
            content: l.content,
            code: l.code,
            channel: l.channel,
            status: l.status,
            error_message: l.error_message,
            sent_at: l.sent_at,
            expires_at: l.expires_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn cleanup_expired_codes() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let expired = CSmsLog::find()
        .filter(c_sms_log::Column::Status.eq("sent"))
        .filter(c_sms_log::Column::ExpiresAt.lt(now))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut count = 0u64;
    for log in expired {
        let mut active_model: c_sms_log::ActiveModel = log.into();
        active_model.status = Set("expired".to_string());

        if CSmsLog::update(active_model)
            .exec(db)
            .await
            .is_ok()
        {
            count += 1;
        }
    }

    Ok(count)
}
