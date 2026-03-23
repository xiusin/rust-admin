use crate::common::error::Error;
use crate::domain::model::m_consumer::{UserTagBrief, *};
use crate::domain::args::a_consumer::{ConsumerListArgs, LoginLogListArgs};
use crate::domain::entity::prelude::{
    CConsumer, CLoginLog, CFinanceAccount, CUserLevel, CUserLevelConfig,
    CUserOauth, CUserTag, CUserTagRelation, CConsumerStatistics,
};
use crate::domain::entity::{
    c_consumer, c_login_log, c_finance_account, c_user_level, c_user_level_config,
    c_user_oauth, c_user_tag, c_user_tag_relation, c_consumer_statistics,
};
use crate::model::prelude::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use rust_decimal::Decimal;
use sea_orm::*;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn hash_password(password: &str) -> std::result::Result<String, argon2::password_hash::Error> {
    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let hash_string = password_hash.to_string();
    if hash_string.is_empty() {
        return Err(argon2::password_hash::Error::Password);
    }
    Ok(hash_string)
}

pub async fn register(params: ConsumerRegisterParams) -> Result<ConsumerResp> {
    let db = DB_WRITE().await;

    let existing = CConsumer::find()
        .filter(c_consumer::Column::Phone.eq(&params.phone))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("手机号已被注册"));
    }

    let password_hash = hash_password(&params.password)
        .map_err(|e| Error::internal_error(format!("Password hash error: {}", e)))?;

    let now = Local::now().naive_local();
    let id = generate_id();

    let consumer = c_consumer::ActiveModel {
        id: Set(id),
        phone: Set(params.phone.clone()),
        password_hash: Set(password_hash),
        status: Set("normal".to_string()),
        risk_score: Set(0),
        login_fail_count: Set(0),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    CConsumer::insert(consumer)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    let _ = CFinanceAccount::insert(c_finance_account::ActiveModel {
        id: Set(generate_id()),
        consumer_id: Set(id),
        balance: Set(Decimal::ZERO),
        frozen_balance: Set(Decimal::ZERO),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    })
    .exec(db)
    .await;

    Ok(ConsumerResp {
        id,
        phone: params.phone,
        nickname: None,
        email: None,
        avatar: None,
        status: ConsumerStatus::Normal,
        risk_score: 0,
        wechat_openid: None,
        wechat_unionid: None,
        last_login_at: None,
        last_login_ip: None,
        created_at: Some(now),
        updated_at: Some(now),
    })
}

pub async fn login(
    params: ConsumerLoginParams,
    ip_address: String,
    user_agent: String,
) -> Result<(ConsumerResp, String)> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find()
        .filter(c_consumer::Column::Phone.eq(&params.phone))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::unauthorized("手机号或密码错误"))?;

    if consumer.status == "locked" {
        if let Some(locked_until) = consumer.locked_until {
            if locked_until > Local::now().naive_local() {
                return Err(Error::forbidden("账号已被锁定，请稍后再试"));
            }
        }
    }

    if consumer.status == "deactivated" {
        return Err(Error::forbidden("账号已注销"));
    }

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&consumer.password_hash)
        .map_err(|e| Error::internal_error(format!("Password parse error: {}", e)))?;

    let verify_result = argon2.verify_password(params.password.as_bytes(), &parsed_hash);

    if verify_result.is_err() {
        let mut active_model: c_consumer::ActiveModel = consumer.clone().into();
        active_model.login_fail_count = Set(consumer.login_fail_count + 1);
        active_model.updated_at = Set(Some(Local::now().naive_local()));

        if consumer.login_fail_count + 1 >= 5 {
            let lock_until = Local::now().naive_local() + chrono::Duration::minutes(30);
            active_model.locked_until = Set(Some(lock_until));
            active_model.status = Set("locked".to_string());
        }

        let _ = CConsumer::update(active_model).exec(db).await;

        let _ = CLoginLog::insert(c_login_log::ActiveModel {
            id: Set(generate_id()),
            consumer_id: Set(consumer.id),
            phone: Set(Some(params.phone.clone())),
            login_type: Set("phone".to_string()),
            success: Set(false),
            fail_reason: Set(Some("密码错误".to_string())),
            ip_address: Set(Some(ip_address)),
            user_agent: Set(Some(user_agent)),
            device_type: Set(None),
            login_at: Set(Local::now().naive_local()),
            ..Default::default()
        })
        .exec(db)
        .await;

        return Err(Error::unauthorized("手机号或密码错误"));
    }

    let now = Local::now().naive_local();

    let mut active_model: c_consumer::ActiveModel = consumer.clone().into();
    active_model.updated_at = Set(Some(now));
    active_model.login_fail_count = Set(0);
    active_model.locked_until = Set(None);
    active_model.last_login_at = Set(Some(now));
    active_model.last_login_ip = Set(Some(ip_address.clone()));
    if consumer.status == "locked" {
        active_model.status = Set("normal".to_string());
    }

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let _ = CLoginLog::insert(c_login_log::ActiveModel {
        id: Set(generate_id()),
        consumer_id: Set(consumer.id),
        phone: Set(Some(params.phone.clone())),
        login_type: Set("phone".to_string()),
        success: Set(true),
        fail_reason: Set(None),
        ip_address: Set(Some(ip_address)),
        user_agent: Set(Some(user_agent)),
        device_type: Set(None),
        login_at: Set(now),
        ..Default::default()
    })
    .exec(db)
    .await;

    let token = format!("consumer_token_{}", consumer.id);

    Ok((
        ConsumerResp {
            id: consumer.id,
            phone: consumer.phone.clone(),
            nickname: consumer.nickname,
            email: consumer.email,
            avatar: consumer.avatar,
            status: match consumer.status.as_str() {
                "normal" => ConsumerStatus::Normal,
                "locked" => ConsumerStatus::Locked,
                _ => ConsumerStatus::Deactivated,
            },
            risk_score: consumer.risk_score,
            wechat_openid: consumer.wechat_openid,
            wechat_unionid: consumer.wechat_unionid,
            last_login_at: consumer.last_login_at,
            last_login_ip: consumer.last_login_ip,
            created_at: consumer.created_at,
            updated_at: consumer.updated_at,
        },
        token,
    ))
}

pub async fn get_info(consumer_id: i64) -> Result<ConsumerResp> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    Ok(ConsumerResp {
        id: consumer.id,
        phone: consumer.phone.clone(),
        nickname: consumer.nickname,
        email: consumer.email,
        avatar: consumer.avatar,
        status: match consumer.status.as_str() {
            "normal" => ConsumerStatus::Normal,
            "locked" => ConsumerStatus::Locked,
            _ => ConsumerStatus::Deactivated,
        },
        risk_score: consumer.risk_score,
        wechat_openid: consumer.wechat_openid,
        wechat_unionid: consumer.wechat_unionid,
        last_login_at: consumer.last_login_at,
        last_login_ip: consumer.last_login_ip,
        created_at: consumer.created_at,
        updated_at: consumer.updated_at,
    })
}

pub async fn update(consumer_id: i64, params: ConsumerUpdateParams) -> Result<()> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_consumer::ActiveModel = consumer.into();

    if let Some(nickname) = params.nickname {
        active_model.nickname = Set(Some(nickname));
    }
    if let Some(email) = params.email {
        active_model.email = Set(Some(email));
    }
    if let Some(avatar) = params.avatar {
        active_model.avatar = Set(Some(avatar));
    }
    active_model.updated_at = Set(Some(now));

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn update_phone(consumer_id: i64, new_phone: String) -> Result<()> {
    let db = DB_WRITE().await;

    let existing = CConsumer::find()
        .filter(c_consumer::Column::Phone.eq(&new_phone))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("手机号已被使用"));
    }

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_consumer::ActiveModel = consumer.into();
    active_model.phone = Set(new_phone);
    active_model.updated_at = Set(Some(now));

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn update_email(consumer_id: i64, new_email: String) -> Result<()> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_consumer::ActiveModel = consumer.into();
    active_model.email = Set(Some(new_email));
    active_model.updated_at = Set(Some(now));

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn reset_password(consumer_id: i64, new_password: String) -> Result<()> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let password_hash = hash_password(&new_password)
        .map_err(|e| Error::internal_error(format!("Password hash error: {}", e)))?;

    let now = Local::now().naive_local();
    let mut active_model: c_consumer::ActiveModel = consumer.into();
    active_model.password_hash = Set(password_hash);
    active_model.updated_at = Set(Some(now));
    active_model.login_fail_count = Set(0);
    active_model.locked_until = Set(None);
    if active_model.status.clone() == Set("locked".to_string()) {
        active_model.status = Set("normal".to_string());
    }

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn deactivate(consumer_id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let consumer = CConsumer::find_by_id(consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_consumer::ActiveModel = consumer.into();
    active_model.status = Set("deactivated".to_string());
    active_model.deactivated_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    CConsumer::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list(
    args: ConsumerListArgs,
) -> Result<(Vec<ConsumerListItem>, u64)> {
    let db = DB_WRITE().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(phone) = &args.phone {
        conditions = conditions.add(c_consumer::Column::Phone.like(format!("%{}%", phone)));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(c_consumer::Column::Status.eq(status.clone()));
    }

    let paginator = CConsumer::find()
        .filter(conditions)
        .order_by_desc(c_consumer::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let consumers = paginator
        .fetch_page(page_num as u64 - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let consumer_ids: Vec<i64> = consumers.iter().map(|c| c.id).collect();

    let levels_map = get_users_levels(db, &consumer_ids).await?;
    let oauths_map = get_users_oauth_types(db, &consumer_ids).await?;
    let tags_map = get_users_tags(db, &consumer_ids).await?;
    let stats_map = get_users_statistics(db, &consumer_ids).await?;

    let items: Vec<ConsumerListItem> = consumers
        .into_iter()
        .map(|c| {
            let consumer_id = c.id;
            let level_info = levels_map.get(&consumer_id);
            let oauth_types = oauths_map.get(&consumer_id).cloned();
            let tags = tags_map.get(&consumer_id).cloned();
            let stats = stats_map.get(&consumer_id);

            ConsumerListItem {
                id: c.id,
                phone: c.phone.clone(),
                email: c.email,
                nickname: c.nickname,
                avatar: c.avatar,
                status: c.status,
                risk_score: c.risk_score,
                wechat_openid: c.wechat_openid,
                wechat_unionid: c.wechat_unionid,
                last_login_at: c.last_login_at,
                last_login_ip: c.last_login_ip,
                created_at: c.created_at,
                level: level_info.as_ref().map(|l| l.0),
                level_name: level_info.as_ref().map(|l| l.1.clone()),
                total_exp: level_info.as_ref().map(|l| l.2),
                total_consumption: stats.as_ref().map(|s| s.0.clone()),
                order_count: stats.as_ref().map(|s| s.1),
                oauth_types,
                tags,
            }
        })
        .collect();

    Ok((items, total))
}

async fn get_users_levels(
    db: &DatabaseConnection,
    consumer_ids: &[i64],
) -> Result<std::collections::HashMap<i64, (i32, String, i64)>> {
    use std::collections::HashMap;
    
    let levels = CUserLevel::find()
        .filter(c_user_level::Column::ConsumerId.is_in(consumer_ids.to_vec()))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let level_nums: Vec<i32> = levels.iter().map(|l| l.level).collect();
    
    let configs = CUserLevelConfig::find()
        .filter(c_user_level_config::Column::Level.is_in(level_nums))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let config_map: HashMap<i32, String> = configs
        .into_iter()
        .map(|c| (c.level, c.level_name))
        .collect();

    Ok(levels
        .into_iter()
        .filter_map(|l| {
            let level_name = config_map.get(&l.level).cloned().unwrap_or_else(|| "未知等级".to_string());
            Some((l.consumer_id, (l.level, level_name, l.total_exp)))
        })
        .collect())
}

async fn get_users_oauth_types(
    db: &DatabaseConnection,
    consumer_ids: &[i64],
) -> Result<std::collections::HashMap<i64, Vec<String>>> {
    use std::collections::HashMap;
    
    let oauths = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.is_in(consumer_ids.to_vec()))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut map: HashMap<i64, Vec<String>> = HashMap::new();
    for oauth in oauths {
        map.entry(oauth.consumer_id)
            .or_insert_with(Vec::new)
            .push(oauth.oauth_type);
    }

    Ok(map)
}

async fn get_users_tags(
    db: &DatabaseConnection,
    consumer_ids: &[i64],
) -> Result<std::collections::HashMap<i64, Vec<UserTagBrief>>> {
    use std::collections::HashMap;
    
    let relations = CUserTagRelation::find()
        .filter(c_user_tag_relation::Column::ConsumerId.is_in(consumer_ids.to_vec()))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let tag_ids: Vec<i64> = relations.iter().map(|r| r.tag_id).collect();
    
    let tags = CUserTag::find()
        .filter(c_user_tag::Column::Id.is_in(tag_ids))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let tag_map: HashMap<i64, UserTagBrief> = tags
        .into_iter()
        .map(|t| {
            (
                t.id,
                UserTagBrief {
                    id: t.id,
                    name: t.name,
                    color: t.color,
                    category: t.category,
                },
            )
        })
        .collect();

    let mut result: HashMap<i64, Vec<UserTagBrief>> = HashMap::new();
    for relation in relations {
        if let Some(tag) = tag_map.get(&relation.tag_id) {
            result
                .entry(relation.consumer_id)
                .or_insert_with(Vec::new)
                .push(tag.clone());
        }
    }

    Ok(result)
}

async fn get_users_statistics(
    db: &DatabaseConnection,
    consumer_ids: &[i64],
) -> Result<std::collections::HashMap<i64, (String, i32)>> {
    
    let stats = CConsumerStatistics::find()
        .filter(c_consumer_statistics::Column::ConsumerId.is_in(consumer_ids.to_vec()))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(stats
        .into_iter()
        .map(|s| (s.consumer_id, (s.total_consume.to_string(), s.order_count)))
        .collect())
}

pub async fn login_logs(
    args: LoginLogListArgs,
) -> Result<(Vec<LoginLogResp>, u64)> {
    let db = DB_WRITE().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(consumer_id) = args.consumer_id {
        conditions = conditions.add(c_login_log::Column::ConsumerId.eq(consumer_id));
    }
    if let Some(phone) = &args.phone {
        conditions = conditions.add(c_login_log::Column::Phone.like(format!("%{}%", phone)));
    }
    if let Some(login_type) = &args.login_type {
        conditions = conditions.add(c_login_log::Column::LoginType.eq(login_type.clone()));
    }
    if let Some(success) = args.success {
        conditions = conditions.add(c_login_log::Column::Success.eq(success));
    }

    let paginator = CLoginLog::find()
        .filter(conditions)
        .order_by_desc(c_login_log::Column::LoginAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let logs = paginator
        .fetch_page(page_num as u64 - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<LoginLogResp> = logs
        .into_iter()
        .map(|l| LoginLogResp {
            id: l.id,
            consumer_id: Some(l.consumer_id),
            phone: l.phone,
            login_type: l.login_type,
            success: l.success,
            fail_reason: l.fail_reason,
            ip_address: l.ip_address,
            user_agent: l.user_agent,
            device_type: l.device_type,
            login_at: Some(l.login_at),
        })
        .collect();

    Ok((items, total))
}