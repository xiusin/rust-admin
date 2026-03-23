use crate::common::error::Error;
use crate::domain::model::m_user_extension::*;
use crate::domain::entity::prelude::{CUserLevel, CUserLevelConfig, CUserLevelRecord};
use crate::domain::entity::{c_user_level, c_user_level_config, c_user_level_record};
use crate::model::prelude::*;
use sea_orm::*;
use sea_orm::prelude::Json;
use rust_decimal::Decimal;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn get_user_level(consumer_id: i64) -> Result<UserLevelModel> {
    let db = DB_WRITE().await;

    let level = CUserLevel::find()
        .filter(c_user_level::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match level {
        Some(l) => {
            let config = CUserLevelConfig::find()
                .filter(c_user_level_config::Column::Level.eq(l.level))
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            let next_config = CUserLevelConfig::find()
                .filter(c_user_level_config::Column::Level.eq(l.level + 1))
                .filter(c_user_level_config::Column::IsActive.eq(true))
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            let level_name = config.as_ref().map(|c| c.level_name.clone()).unwrap_or_else(|| "未知等级".to_string());
            let discount_rate = config.as_ref().map(|c| c.discount_rate.to_string()).unwrap_or_else(|| "100".to_string());

            Ok(UserLevelModel {
                id: l.id,
                consumer_id: l.consumer_id,
                level: l.level,
                level_name,
                exp: l.exp,
                total_exp: l.total_exp,
                next_level_exp: next_config.map(|c| c.min_exp),
                discount_rate,
                level_up_at: l.level_up_at,
            })
        }
        None => {
            let config = CUserLevelConfig::find()
                .filter(c_user_level_config::Column::Level.eq(1))
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            let next_config = CUserLevelConfig::find()
                .filter(c_user_level_config::Column::Level.eq(2))
                .filter(c_user_level_config::Column::IsActive.eq(true))
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            let level_name = config.as_ref().map(|c| c.level_name.clone()).unwrap_or_else(|| "普通会员".to_string());
            let discount_rate = config.as_ref().map(|c| c.discount_rate.to_string()).unwrap_or_else(|| "100".to_string());

            Ok(UserLevelModel {
                id: 0,
                consumer_id,
                level: 1,
                level_name,
                exp: 0,
                total_exp: 0,
                next_level_exp: next_config.map(|c| c.min_exp),
                discount_rate,
                level_up_at: None,
            })
        }
    }
}

pub async fn add_exp(params: AddExpParams) -> Result<UserLevelModel> {
    let db = DB_WRITE().await;

    let level = CUserLevel::find()
        .filter(c_user_level::Column::ConsumerId.eq(params.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let now = Local::now().naive_local();

    let (old_level, old_exp, new_level, new_exp, _new_total_exp) = match level {
        Some(l) => {
            let new_exp = l.exp + params.exp;
            let new_total_exp = l.total_exp + params.exp as i64;

            let target_level = calculate_level(db, new_exp).await?;

            let mut active_model: c_user_level::ActiveModel = l.clone().into();
            active_model.exp = Set(new_exp);
            active_model.total_exp = Set(new_total_exp);

            if target_level != l.level {
                active_model.level = Set(target_level);
                active_model.level_up_at = Set(Some(now));
            }

            active_model.updated_at = Set(Some(now));

            CUserLevel::update(active_model)
                .exec(db)
                .await
                .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

            (Some(l.level), Some(l.exp), target_level, new_exp, new_total_exp)
        }
        None => {
            let target_level = calculate_level(db, params.exp).await?;

            let new_level = c_user_level::ActiveModel {
                id: Set(generate_id()),
                consumer_id: Set(params.consumer_id),
                level: Set(target_level),
                exp: Set(params.exp),
                total_exp: Set(params.exp as i64),
                level_up_at: Set(None),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };

            CUserLevel::insert(new_level)
                .exec(db)
                .await
                .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

            (None, None, target_level, params.exp, params.exp as i64)
        }
    };

    let change_type = if new_level > old_level.unwrap_or(1) {
        "level_up"
    } else if new_level < old_level.unwrap_or(1) {
        "level_down"
    } else if params.exp > 0 {
        "exp_add"
    } else {
        "exp_reduce"
    };

    let record = c_user_level_record::ActiveModel {
        id: Set(generate_id()),
        consumer_id: Set(params.consumer_id),
        old_level: Set(old_level),
        new_level: Set(new_level),
        old_exp: Set(old_exp),
        new_exp: Set(new_exp),
        exp_change: Set(params.exp),
        change_type: Set(change_type.to_string()),
        source: Set(Some(params.source)),
        source_id: Set(params.source_id),
        remark: Set(params.remark),
        created_at: Set(Some(now)),
    };

    let _ = CUserLevelRecord::insert(record).exec(db).await;

    get_user_level(params.consumer_id).await
}

async fn calculate_level(db: &DatabaseConnection, exp: i32) -> Result<i32> {
    let configs = CUserLevelConfig::find()
        .filter(c_user_level_config::Column::IsActive.eq(true))
        .order_by_asc(c_user_level_config::Column::Level)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut level = 1;
    for config in configs {
        if exp >= config.min_exp {
            level = config.level;
        } else {
            break;
        }
    }

    Ok(level)
}

pub async fn get_level_configs() -> Result<Vec<LevelConfigModel>> {
    let db = DB_WRITE().await;

    let configs = CUserLevelConfig::find()
        .filter(c_user_level_config::Column::IsActive.eq(true))
        .order_by_asc(c_user_level_config::Column::Level)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs
        .into_iter()
        .map(|c| LevelConfigModel {
            id: c.id,
            level: c.level,
            level_name: c.level_name,
            min_exp: c.min_exp,
            max_exp: c.max_exp,
            icon: c.icon,
            color: c.color,
            discount_rate: c.discount_rate.to_string(),
            privileges: c.privileges,
        })
        .collect())
}

pub async fn get_level_records(
    params: LevelRecordListParams,
) -> Result<(Vec<LevelRecordModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let paginator = CUserLevelRecord::find()
        .filter(c_user_level_record::Column::ConsumerId.eq(params.consumer_id))
        .order_by_desc(c_user_level_record::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let records = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<LevelRecordModel> = records
        .into_iter()
        .map(|r| LevelRecordModel {
            id: r.id,
            consumer_id: r.consumer_id,
            old_level: r.old_level,
            new_level: r.new_level,
            old_exp: r.old_exp,
            new_exp: r.new_exp,
            exp_change: r.exp_change,
            change_type: r.change_type,
            source: r.source,
            source_id: r.source_id,
            remark: r.remark,
            created_at: r.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn create_level_config(
    level: i32,
    level_name: String,
    min_exp: i32,
    max_exp: Option<i32>,
    icon: Option<String>,
    color: Option<String>,
    discount_rate: Decimal,
    privileges: Option<Json>,
) -> Result<LevelConfigModel> {
    let db = DB_WRITE().await;

    let existing = CUserLevelConfig::find()
        .filter(c_user_level_config::Column::Level.eq(level))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("该等级配置已存在"));
    }

    let now = Local::now().naive_local();
    let config = c_user_level_config::ActiveModel {
        id: Set(generate_id()),
        level: Set(level),
        level_name: Set(level_name.clone()),
        min_exp: Set(min_exp),
        max_exp: Set(max_exp),
        icon: Set(icon.clone()),
        color: Set(color.clone()),
        discount_rate: Set(discount_rate),
        privileges: Set(privileges.clone()),
        is_active: Set(true),
        sort_order: Set(level),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    let inserted = CUserLevelConfig::insert(config)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(LevelConfigModel {
        id: inserted.last_insert_id,
        level,
        level_name,
        min_exp,
        max_exp,
        icon,
        color,
        discount_rate: discount_rate.to_string(),
        privileges,
    })
}
