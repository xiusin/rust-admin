use crate::common::error::Error;
use crate::domain::args::a_verification::*;
use crate::domain::entity::{p_verification_code, p_verification_log, p_product, p_store};
use crate::domain::model::m_verification::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: VerificationCodeListArgs) -> Result<ListData<VerificationCodeListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();

    if let Some(code) = &args.code {
        conditions = conditions.add(p_verification_code::Column::Code.contains(code));
    }
    if let Some(order_no) = &args.order_no {
        conditions = conditions.add(p_verification_code::Column::OrderNo.contains(order_no));
    }
    if let Some(status) = args.status {
        conditions = conditions.add(p_verification_code::Column::Status.eq(status));
    }
    if let Some(store_id) = args.store_id {
        conditions = conditions.add(p_verification_code::Column::StoreId.eq(store_id));
    }
    if let Some(start_time) = &args.start_time {
        if let Ok(start) = start_time.parse::<DateTime>() {
            conditions = conditions.add(p_verification_code::Column::CreatedAt.gte(start));
        }
    }
    if let Some(end_time) = &args.end_time {
        if let Ok(end) = end_time.parse::<DateTime>() {
            conditions = conditions.add(p_verification_code::Column::CreatedAt.lte(end));
        }
    }

    let total = p_verification_code::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_verification_code::Entity::find()
        .filter(conditions)
        .order_by_desc(p_verification_code::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let product_ids: Vec<i64> = items.iter().filter_map(|i| i.product_id).collect();

    let products = p_product::Entity::find()
        .filter(p_product::Column::Id.is_in(product_ids.clone()))
        .filter(p_product::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    let product_map: HashMap<i64, p_product::Model> = products.into_iter().map(|p| (p.id, p)).collect();

    let list: Vec<VerificationCodeListItem> = items
        .into_iter()
        .map(|item| {
            let product = product_map.get(&item.product_id.unwrap_or(0));
            VerificationCodeListItem {
                id: item.id,
                order_id: item.order_id,
                order_no: item.order_no,
                product_id: item.product_id,
                product_name: product.map(|p| p.name.clone()).unwrap_or_default(),
                product_image: product.map(|p| p.cover_image.clone()).unwrap_or_default(),
                sku_id: item.sku_id,
                sku_code: None,
                spec_text: None,
                code: item.code,
                qr_code: item.qr_code,
                total_count: item.total_count,
                used_count: item.used_count,
                remain_count: item.total_count - item.used_count,
                status: item.status,
                status_name: get_status_name(item.status),
                expire_at: item.expire_at,
                store_name: item.store_name,
                created_at: item.created_at,
            }
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn verify(args: VerificationArgs, user_id: i64, user_name: &str) -> Result<VerificationResult> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let code = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Code.eq(&args.code))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("核销码不存在"))?;

    if code.status != 1 {
        return Err(Error::bad_request("核销码已核销"));
    }
    if code.status == 3 {
        return Err(Error::bad_request("核销码已过期"));
    }
    if code.used_count >= code.total_count {
        return Err(Error::bad_request("核销次数已用完"));
    }

    let product = if let Some(pid) = code.product_id {
        p_product::Entity::find_by_id(pid)
            .filter(p_product::Column::DeletedAt.is_null())
            .one(db)
            .await?
    } else {
        None
    };

    let mut model: p_verification_code::ActiveModel = code.clone().into();
    model.used_count = Set(code.used_count + 1);
    model.status = Set(2);
    model.store_id = Set(args.store_id);
    model.update(db).await?;

    let store_name = if let Some(store_id) = args.store_id {
        p_store::Entity::find_by_id(store_id)
            .filter(p_store::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .map(|s| s.name.clone())
    } else {
        None
    };

    let log_id = GID().await;
    let log = p_verification_log::ActiveModel {
        id: Set(log_id),
        verification_code_id: Set(code.id),
        code: Set(code.code.clone()),
        order_no: Set(code.order_no.clone().unwrap_or_default()),
        product_name: Set(product.as_ref().map(|p| p.name.clone()).unwrap_or_default()),
        store_id: Set(args.store_id),
        store_name: Set(store_name.clone()),
        verified_by: Set(Some(user_id)),
        verified_by_name: Set(Some(user_name.to_string())),
        remark: Set(args.remark),
        created_at: Set(Some(now)),
    };
    log.insert(db).await?;

    Ok(VerificationResult {
        success: true,
        message: "核销成功".to_string(),
        code: code.code,
        product_name: product.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
        spec_text: None,
        order_no: code.order_no,
        remain_count: code.total_count - code.used_count,
        store_name,
        verified_at: Some(now),
    })
}

pub async fn query(args: VerificationQueryArgs) -> Result<VerificationQueryResult> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let code = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Code.eq(&args.code))
        .one(db)
        .await?;

    match code {
        Some(code) => {
            let product = if let Some(pid) = code.product_id {
                p_product::Entity::find_by_id(pid)
                    .filter(p_product::Column::DeletedAt.is_null())
                    .one(db)
                    .await?
            } else {
                None
            };

            let is_valid = code.status == 1 && code.used_count < code.total_count;
            let expire_valid = code.expire_at.map(|e| e > now).unwrap_or(true);
            let status_name = get_status_name(code.status);
            let message = if is_valid && expire_valid {
                "核销码有效".to_string()
            } else if code.status == 2 {
                "核销码已核销".to_string()
            } else if code.status == 3 {
                "核销码已过期".to_string()
            } else if code.status == 4 {
                "核销码已退款".to_string()
            } else {
                "核销码无效".to_string()
            };

            Ok(VerificationQueryResult {
                code: code.code,
                product_name: product.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
                product_image: product.as_ref().map(|p| p.cover_image.clone()).unwrap_or_default(),
                spec_text: None,
                order_no: code.order_no,
                total_count: code.total_count,
                used_count: code.used_count,
                remain_count: code.total_count - code.used_count,
                status: code.status,
                status_name,
                expire_at: code.expire_at,
                is_valid,
                message,
            })
        }
        None => Ok(VerificationQueryResult {
            code: args.code,
            product_name: String::new(),
            product_image: String::new(),
            spec_text: None,
            order_no: None,
            total_count: 0,
            used_count: 0,
            remain_count: 0,
            status: 0,
            status_name: String::new(),
            expire_at: None,
            is_valid: false,
            message: "核销码不存在".to_string(),
        }),
    }
}

pub async fn log_list(args: VerificationLogListArgs) -> Result<ListData<VerificationLogItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();

    if let Some(code) = &args.code {
        conditions = conditions.add(p_verification_log::Column::Code.contains(code));
    }
    if let Some(order_no) = &args.order_no {
        conditions = conditions.add(p_verification_log::Column::OrderNo.contains(order_no));
    }
    if let Some(store_id) = args.store_id {
        conditions = conditions.add(p_verification_log::Column::StoreId.eq(store_id));
    }
    if let Some(start_time) = &args.start_time {
        if let Ok(start) = start_time.parse::<DateTime>() {
            conditions = conditions.add(p_verification_log::Column::CreatedAt.gte(start));
        }
    }
    if let Some(end_time) = &args.end_time {
        if let Ok(end) = end_time.parse::<DateTime>() {
            conditions = conditions.add(p_verification_log::Column::CreatedAt.lte(end));
        }
    }

    let total = p_verification_log::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_verification_log::Entity::find()
        .filter(conditions)
        .order_by_desc(p_verification_log::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<VerificationLogItem> = items
        .into_iter()
        .map(|item| VerificationLogItem {
            id: item.id,
            verification_code_id: item.verification_code_id,
            code: item.code,
            order_no: item.order_no,
            product_name: item.product_name,
            store_id: item.store_id,
            store_name: item.store_name,
            verified_by: item.verified_by,
            verified_by_name: item.verified_by_name,
            remark: item.remark,
            created_at: item.created_at,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn statistics() -> Result<VerificationStatistics> {
    let db = DB().await;

    let total_codes = p_verification_code::Entity::find()
        .count(db)
        .await?;

    let pending_count = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Status.eq(1))
        .count(db)
        .await?;

    let verified_count = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Status.eq(2))
        .count(db)
        .await?;

    let expired_count = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Status.eq(3))
        .count(db)
        .await?;

    let refunded_count = p_verification_code::Entity::find()
        .filter(p_verification_code::Column::Status.eq(4))
        .count(db)
        .await?;

    let today = Local::now().date_naive();
    let today_verified_count = p_verification_log::Entity::find()
        .filter(p_verification_log::Column::CreatedAt.gte(today.and_hms_opt(0, 0, 0).unwrap()))
        .count(db)
        .await?;

    Ok(VerificationStatistics {
        total_codes: total_codes as i64,
        pending_count: pending_count as i64,
        verified_count: verified_count as i64,
        expired_count: expired_count as i64,
        refunded_count: refunded_count as i64,
        today_verified_count: today_verified_count as i64,
    })
}

fn get_status_name(status: i32) -> String {
    match status {
        1 => "待核销".to_string(),
        2 => "已核销".to_string(),
        3 => "已过期".to_string(),
        4 => "已退款".to_string(),
        _ => "未知".to_string(),
    }
}
