use crate::common::error::Error;
use crate::domain::model::m_user_extension::*;
use crate::domain::entity::prelude::{CUserBan, CConsumer};
use crate::domain::entity::{c_user_ban, c_consumer};
use crate::model::prelude::*;
use sea_orm::*;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn ban_user(params: BanUserParams) -> Result<UserBanModel> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(params.consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let active_ban = CUserBan::find()
        .filter(c_user_ban::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_user_ban::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if active_ban.is_some() {
        return Err(Error::bad_request("用户已被禁用"));
    }

    let now = Local::now().naive_local();

    let ban = c_user_ban::ActiveModel {
        id: Set(generate_id()),
        consumer_id: Set(params.consumer_id),
        ban_type: Set(params.ban_type.clone()),
        reason: Set(params.reason.clone()),
        start_at: Set(now),
        end_at: Set(params.end_at),
        banned_by: Set(params.banned_by),
        unban_at: Set(None),
        unban_by: Set(None),
        unban_reason: Set(None),
        status: Set("active".to_string()),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    let inserted = CUserBan::insert(ban)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    let mut active_model: c_consumer::ActiveModel = consumer.into();
    active_model.status = Set("banned".to_string());
    active_model.updated_at = Set(Some(now));

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update consumer error: {}", e)))?;

    Ok(UserBanModel {
        id: inserted.last_insert_id,
        consumer_id: params.consumer_id,
        ban_type: params.ban_type,
        reason: params.reason,
        start_at: now,
        end_at: params.end_at,
        banned_by: params.banned_by,
        unban_at: None,
        unban_by: None,
        unban_reason: None,
        status: "active".to_string(),
        created_at: Some(now),
    })
}

pub async fn unban_user(params: UnbanUserParams) -> Result<()> {
    let db = DB_WRITE().await;

    let active_ban = CUserBan::find()
        .filter(c_user_ban::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_user_ban::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("未找到有效的禁用记录"))?;

    let now = Local::now().naive_local();

    let mut active_model: c_user_ban::ActiveModel = active_ban.into();
    active_model.status = Set("cancelled".to_string());
    active_model.unban_at = Set(Some(now));
    active_model.unban_by = Set(params.unban_by);
    active_model.unban_reason = Set(params.unban_reason.clone());
    active_model.updated_at = Set(Some(now));

    CUserBan::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let consumer = CConsumer::find_by_id(params.consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let mut consumer_model: c_consumer::ActiveModel = consumer.into();
    consumer_model.status = Set("normal".to_string());
    consumer_model.updated_at = Set(Some(now));

    CConsumer::update(consumer_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update consumer error: {}", e)))?;

    Ok(())
}

pub async fn check_ban_status(consumer_id: i64) -> Result<Option<UserBanModel>> {
    let db = DB_WRITE().await;

    let now = Local::now().naive_local();

    let active_ban = CUserBan::find()
        .filter(c_user_ban::Column::ConsumerId.eq(consumer_id))
        .filter(c_user_ban::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match active_ban {
        Some(ban) => {
            if ban.ban_type == "temporary" {
                if let Some(end_at) = ban.end_at {
                    if end_at < now {
                        let mut active_model: c_user_ban::ActiveModel = ban.clone().into();
                        active_model.status = Set("expired".to_string());
                        active_model.updated_at = Set(Some(now));

                        let _ = CUserBan::update(active_model).exec(db).await;

                        let consumer = CConsumer::find_by_id(consumer_id)
                            .one(db)
                            .await
                            .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

                        if let Some(c) = consumer {
                            let mut consumer_model: c_consumer::ActiveModel = c.into();
                            consumer_model.status = Set("normal".to_string());
                            consumer_model.updated_at = Set(Some(now));
                            let _ = CConsumer::update(consumer_model).exec(db).await;
                        }

                        return Ok(None);
                    }
                }
            }

            Ok(Some(UserBanModel {
                id: ban.id,
                consumer_id: ban.consumer_id,
                ban_type: ban.ban_type,
                reason: ban.reason,
                start_at: ban.start_at,
                end_at: ban.end_at,
                banned_by: ban.banned_by,
                unban_at: ban.unban_at,
                unban_by: ban.unban_by,
                unban_reason: ban.unban_reason,
                status: ban.status,
                created_at: ban.created_at,
            }))
        }
        None => Ok(None),
    }
}

pub async fn get_ban_history(params: BanListParams) -> Result<(Vec<UserBanModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(consumer_id) = params.consumer_id {
        conditions = conditions.add(c_user_ban::Column::ConsumerId.eq(consumer_id));
    }
    if let Some(status) = &params.status {
        conditions = conditions.add(c_user_ban::Column::Status.eq(status.clone()));
    }

    let paginator = CUserBan::find()
        .filter(conditions)
        .order_by_desc(c_user_ban::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let bans = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<UserBanModel> = bans
        .into_iter()
        .map(|b| UserBanModel {
            id: b.id,
            consumer_id: b.consumer_id,
            ban_type: b.ban_type,
            reason: b.reason,
            start_at: b.start_at,
            end_at: b.end_at,
            banned_by: b.banned_by,
            unban_at: b.unban_at,
            unban_by: b.unban_by,
            unban_reason: b.unban_reason,
            status: b.status,
            created_at: b.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn process_expired_bans() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let expired_bans = CUserBan::find()
        .filter(c_user_ban::Column::Status.eq("active"))
        .filter(c_user_ban::Column::BanType.eq("temporary"))
        .filter(c_user_ban::Column::EndAt.lt(now))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut count = 0u64;

    for ban in expired_bans {
        let mut active_model: c_user_ban::ActiveModel = ban.clone().into();
        active_model.status = Set("expired".to_string());
        active_model.updated_at = Set(Some(now));

        if CUserBan::update(active_model).exec(db).await.is_ok() {
            let consumer = CConsumer::find_by_id(ban.consumer_id)
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            if let Some(c) = consumer {
                let mut consumer_model: c_consumer::ActiveModel = c.into();
                consumer_model.status = Set("normal".to_string());
                consumer_model.updated_at = Set(Some(now));
                let _ = CConsumer::update(consumer_model).exec(db).await;
            }
            count += 1;
        }
    }

    Ok(count)
}
