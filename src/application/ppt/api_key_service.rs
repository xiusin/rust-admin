use crate::service::prelude::*;
use crate::domain::entity::{ppt_api_key, ppt_api_usage_log};
use crate::domain::model::m_ppt_subscription::*;
use crate::domain::args::a_ppt_subscription::*;
use crate::common::error::{Error, Result};
use chrono::{DateTime, Local};

pub async fn generate_key(
    user_id: i64,
    name: String,
    permissions: Option<Vec<String>>,
    rate_limit: Option<i32>,
    daily_limit: Option<i32>,
    expires_at: Option<DateTime<Local>>,
) -> Result<APIKeyInfo> {
    let db = DB().await;
    
    let existing_count = ppt_api_key::Entity::find()
        .filter(ppt_api_key::Column::UserId.eq(user_id))
        .filter(ppt_api_key::Column::DeletedAt.is_null())
        .count(db)
        .await
        .unwrap_or(0);
    
    if existing_count >= 10 {
        return Err(Error::bad_request("最多只能创建10个API密钥"));
    }
    
    let api_key = format!("pk_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    let api_secret = format!("sk_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    
    let now = Local::now().naive_utc();
    let id = GID().await;
    
    let permissions_json = permissions.map(|p| {
        serde_json::to_value(p).unwrap_or(serde_json::Value::Null)
    });
    
    let key = ppt_api_key::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        user_id: sea_orm::ActiveValue::Set(user_id),
        name: sea_orm::ActiveValue::Set(name),
        api_key: sea_orm::ActiveValue::Set(api_key.clone()),
        api_secret: sea_orm::ActiveValue::Set(api_secret.clone()),
        permissions: sea_orm::ActiveValue::Set(permissions_json),
        rate_limit: sea_orm::ActiveValue::Set(rate_limit.unwrap_or(100)),
        daily_limit: sea_orm::ActiveValue::Set(daily_limit.unwrap_or(1000)),
        daily_used: sea_orm::ActiveValue::Set(0),
        total_requests: sea_orm::ActiveValue::Set(0),
        success_requests: sea_orm::ActiveValue::Set(0),
        failed_requests: sea_orm::ActiveValue::Set(0),
        last_used_at: sea_orm::ActiveValue::Set(None),
        last_reset_at: sea_orm::ActiveValue::Set(Some(now)),
        expires_at: sea_orm::ActiveValue::Set(expires_at.map(|dt| dt.naive_local())),
        is_active: sea_orm::ActiveValue::Set(1),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        deleted_at: sea_orm::ActiveValue::Set(None),
    };
    
    let key = key.insert(db).await?;
    
    Ok(APIKeyInfo {
        id: key.id,
        user_id: key.user_id,
        name: key.name,
        api_key: key.api_key,
        api_secret: Some(api_secret),
        permissions: key.permissions,
        rate_limit: key.rate_limit,
        daily_limit: key.daily_limit,
        daily_used: key.daily_used,
        total_requests: key.total_requests,
        success_requests: key.success_requests,
        failed_requests: key.failed_requests,
        last_used_at: key.last_used_at,
        expires_at: key.expires_at,
        is_active: key.is_active,
        created_at: key.created_at,
    })
}

pub async fn list_keys(
    page: PageParams,
    args: APIKeyListArgs,
) -> Result<ListData<APIKeyListItem>> {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_api_key::Column::DeletedAt.is_null());
    
    if let Some(user_id) = args.user_id {
        conditions = conditions.add(ppt_api_key::Column::UserId.eq(user_id));
    }
    if let Some(is_active) = args.is_active {
        conditions = conditions.add(ppt_api_key::Column::IsActive.eq(is_active));
    }
    
    let total = ppt_api_key::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_api_key::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_api_key::Column::CreatedAt)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await?;
    
    let list = items.into_iter().map(|item| {
        let api_key_prefix = if item.api_key.len() > 8 {
            format!("{}...{}", &item.api_key[..4], &item.api_key[item.api_key.len()-4..])
        } else {
            item.api_key.clone()
        };
        
        APIKeyListItem {
            id: item.id,
            name: item.name,
            api_key_prefix,
            permissions: item.permissions,
            rate_limit: item.rate_limit,
            daily_limit: item.daily_limit,
            daily_used: item.daily_used,
            total_requests: item.total_requests,
            last_used_at: item.last_used_at,
            expires_at: item.expires_at,
            is_active: item.is_active,
            created_at: item.created_at,
        }
    }).collect();
    
    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })
}

pub async fn get_key_detail(key_id: i64) -> Result<APIKeyInfo> {
    let db = DB().await;
    
    let key = ppt_api_key::Entity::find_by_id(key_id)
        .filter(ppt_api_key::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("API密钥不存在"))?;
    
    Ok(APIKeyInfo {
        id: key.id,
        user_id: key.user_id,
        name: key.name,
        api_key: key.api_key,
        api_secret: None,
        permissions: key.permissions,
        rate_limit: key.rate_limit,
        daily_limit: key.daily_limit,
        daily_used: key.daily_used,
        total_requests: key.total_requests,
        success_requests: key.success_requests,
        failed_requests: key.failed_requests,
        last_used_at: key.last_used_at,
        expires_at: key.expires_at,
        is_active: key.is_active,
        created_at: key.created_at,
    })
}

pub async fn revoke_key(key_id: i64) -> Result<()> {
    let db = DB().await;
    
    let key = ppt_api_key::Entity::find_by_id(key_id)
        .filter(ppt_api_key::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("API密钥不存在"))?;
    
    let now = Local::now().naive_utc();
    
    let mut key_model: ppt_api_key::ActiveModel = key.into();
    key_model.is_active = sea_orm::ActiveValue::Set(0);
    key_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
    key_model.update(db).await?;
    
    Ok(())
}

pub async fn delete_keys(ids: Vec<i64>) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_utc();
    
    for id in ids {
        let key = ppt_api_key::Entity::find_by_id(id)
            .one(db)
            .await?;
        
        if let Some(key) = key {
            let mut key_model: ppt_api_key::ActiveModel = key.into();
            key_model.deleted_at = sea_orm::ActiveValue::Set(Some(now));
            key_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
            key_model.update(db).await?;
        }
    }
    
    Ok(())
}

pub async fn validate_key(api_key: &str) -> Result<APIKeyValidation> {
    let db = DB().await;
    
    let key = ppt_api_key::Entity::find()
        .filter(ppt_api_key::Column::ApiKey.eq(api_key))
        .filter(ppt_api_key::Column::IsActive.eq(1))
        .filter(ppt_api_key::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    
    match key {
        Some(key) => {
            let now = Local::now().naive_utc();
            
            if let Some(expires_at) = key.expires_at {
                if expires_at < now {
                    return Ok(APIKeyValidation {
                        valid: false,
                        user_id: None,
                        key_id: None,
                        permissions: None,
                        rate_limit: None,
                        daily_limit: None,
                        message: Some("API密钥已过期".to_string()),
                    });
                }
            }
            
            if key.daily_used >= key.daily_limit {
                return Ok(APIKeyValidation {
                    valid: false,
                    user_id: None,
                    key_id: None,
                    permissions: None,
                    rate_limit: None,
                    daily_limit: None,
                    message: Some("今日请求次数已达上限".to_string()),
                });
            }
            
            let permissions: Vec<String> = key.permissions
                .and_then(|p| serde_json::from_value(p).ok())
                .unwrap_or_default();
            
            Ok(APIKeyValidation {
                valid: true,
                user_id: Some(key.user_id),
                key_id: Some(key.id),
                permissions: Some(permissions),
                rate_limit: Some(key.rate_limit),
                daily_limit: Some(key.daily_limit),
                message: None,
            })
        }
        None => Ok(APIKeyValidation {
            valid: false,
            user_id: None,
            key_id: None,
            permissions: None,
            rate_limit: None,
            daily_limit: None,
            message: Some("无效的API密钥".to_string()),
        }),
    }
}

pub async fn record_api_usage(
    key_id: i64,
    user_id: i64,
    endpoint: &str,
    method: &str,
    status_code: i32,
    response_time_ms: i32,
    tokens_used: i32,
    error_message: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<()> {
    let db = DB().await;
    
    let now = Local::now().naive_utc();
    
    let id = GID().await;
    
    let log = ppt_api_usage_log::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        api_key_id: sea_orm::ActiveValue::Set(key_id),
        user_id: sea_orm::ActiveValue::Set(user_id),
        endpoint: sea_orm::ActiveValue::Set(endpoint.to_string()),
        method: sea_orm::ActiveValue::Set(method.to_string()),
        status_code: sea_orm::ActiveValue::Set(status_code),
        response_time_ms: sea_orm::ActiveValue::Set(response_time_ms),
        tokens_used: sea_orm::ActiveValue::Set(tokens_used),
        error_message: sea_orm::ActiveValue::Set(error_message),
        ip_address: sea_orm::ActiveValue::Set(ip_address),
        user_agent: sea_orm::ActiveValue::Set(user_agent),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
    };
    
    log.insert(db).await?;
    
    let key = ppt_api_key::Entity::find_by_id(key_id)
        .one(db)
        .await?;
    
    if let Some(key) = key {
        let mut key_model: ppt_api_key::ActiveModel = key.into();
        key_model.total_requests = sea_orm::ActiveValue::Set(key_model.total_requests.unwrap() + 1);
        key_model.daily_used = sea_orm::ActiveValue::Set(key_model.daily_used.unwrap() + 1);
        key_model.last_used_at = sea_orm::ActiveValue::Set(Some(now));
        
        if status_code >= 200 && status_code < 400 {
            key_model.success_requests = sea_orm::ActiveValue::Set(key_model.success_requests.unwrap() + 1);
        } else {
            key_model.failed_requests = sea_orm::ActiveValue::Set(key_model.failed_requests.unwrap() + 1);
        }
        
        key_model.update(db).await?;
    }
    
    Ok(())
}

pub async fn get_usage_stats(key_id: i64, start_date: Option<String>, end_date: Option<String>) -> Result<APIUsageStats> {
    let db = DB().await;
    
    let _key = ppt_api_key::Entity::find_by_id(key_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("API密钥不存在"))?;
    
    let mut conditions = Condition::all()
        .add(ppt_api_usage_log::Column::ApiKeyId.eq(key_id));
    
    if let Some(ref start) = start_date {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&format!("{} 00:00:00", start), "%Y-%m-%d %H:%M:%S") {
            conditions = conditions.add(ppt_api_usage_log::Column::CreatedAt.gte(dt));
        }
    }
    
    if let Some(ref end) = end_date {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&format!("{} 23:59:59", end), "%Y-%m-%d %H:%M:%S") {
            conditions = conditions.add(ppt_api_usage_log::Column::CreatedAt.lte(dt));
        }
    }
    
    let logs = ppt_api_usage_log::Entity::find()
        .filter(conditions)
        .all(db)
        .await?;
    
    let total_requests = logs.len() as u64;
    let success_requests = logs.iter().filter(|l| l.status_code >= 200 && l.status_code < 400).count() as u64;
    let failed_requests = total_requests - success_requests;
    
    let avg_response_time = if total_requests > 0 {
        logs.iter().map(|l| l.response_time_ms as f32).sum::<f32>() / total_requests as f32
    } else {
        0.0
    };
    
    let success_rate = if total_requests > 0 {
        (success_requests as f32 / total_requests as f32) * 100.0
    } else {
        0.0
    };
    
    let mut daily_map: std::collections::HashMap<String, (u64, u64, f32)> = std::collections::HashMap::new();
    
    for log in &logs {
        let date = log.created_at.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_default();
        let entry = daily_map.entry(date).or_insert((0, 0, 0.0));
        entry.0 += 1;
        entry.1 += log.tokens_used as u64;
        entry.2 += log.response_time_ms as f32;
    }
    
    let mut daily_usage: Vec<DailyUsage> = daily_map.into_iter()
        .map(|(date, (requests, tokens, total_time))| {
            DailyUsage {
                date,
                requests,
                tokens_used: tokens,
                avg_response_time: if requests > 0 { total_time / requests as f32 } else { 0.0 },
            }
        })
        .collect();
    
    daily_usage.sort_by(|a, b| a.date.cmp(&b.date));
    
    Ok(APIUsageStats {
        total_requests,
        success_requests,
        failed_requests,
        avg_response_time,
        success_rate,
        daily_usage,
    })
}

pub async fn reset_daily_usage() -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_utc();
    
    let keys = ppt_api_key::Entity::find()
        .filter(ppt_api_key::Column::DeletedAt.is_null())
        .all(db)
        .await?;
    
    for key in keys {
        let mut key_model: ppt_api_key::ActiveModel = key.into();
        key_model.daily_used = sea_orm::ActiveValue::Set(0);
        key_model.last_reset_at = sea_orm::ActiveValue::Set(Some(now));
        key_model.update(db).await?;
    }
    
    Ok(())
}
