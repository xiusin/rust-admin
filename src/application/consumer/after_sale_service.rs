use crate::common::error::Error;
use crate::domain::model::m_after_sale::*;
use crate::domain::entity::prelude::{
    CAfterSale, CAfterSaleItem, CAfterSaleRefund, CAfterSaleLogistics,
    CAfterSaleStatusLog, CAfterSaleTimeoutConfig,
};
use crate::domain::entity::{
    c_after_sale, c_after_sale_item, c_after_sale_refund, c_after_sale_logistics,
    c_after_sale_status_log, c_after_sale_timeout_config,
};
use crate::model::prelude::*;
use sea_orm::*;
use rust_decimal::Decimal;
use chrono::{Local, Duration};

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_after_sale_no() -> String {
    format!("AS{}{}", chrono::Utc::now().format("%Y%m%d%H%M%S"), generate_id() % 10000)
}

#[allow(dead_code)]
fn generate_refund_no() -> String {
    format!("RF{}{}", chrono::Utc::now().format("%Y%m%d%H%M%S"), generate_id() % 10000)
}

pub async fn apply(params: ApplyAfterSaleParams) -> Result<AfterSaleModel> {
    let db = DB_WRITE().await;

    let existing = CAfterSale::find()
        .filter(c_after_sale::Column::OrderId.eq(params.order_id))
        .filter(c_after_sale::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_after_sale::Column::Status.is_in(vec!["pending", "processing", "agreed"]))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("该订单已存在进行中的售后申请"));
    }

    let total_refund: Decimal = params.items.iter().map(|i| i.refund_amount).sum();

    let now = Local::now().naive_local();
    let after_sale_no = generate_after_sale_no();
    let type_str = serde_json::to_string(&params.r#type).unwrap_or_else(|_| "\"refund_only\"".to_string());

    let timeout_config = CAfterSaleTimeoutConfig::find()
        .filter(c_after_sale_timeout_config::Column::Stage.eq("apply_audit"))
        .filter(c_after_sale_timeout_config::Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let timeout_at = timeout_config.map(|c| now + Duration::hours(c.timeout_hours as i64));

    let after_sale_id = generate_id();
    let after_sale = c_after_sale::ActiveModel {
        id: Set(after_sale_id),
        after_sale_no: Set(after_sale_no.clone()),
        order_id: Set(params.order_id),
        order_no: Set(params.order_no.clone()),
        consumer_id: Set(params.consumer_id),
        r#type: Set(type_str.clone()),
        reason: Set(params.reason.clone()),
        description: Set(params.description.clone()),
        evidence_urls: Set(params.evidence_urls.as_ref().map(|urls| serde_json::to_value(urls).unwrap_or_default())),
        refund_amount: Set(total_refund),
        status: Set("pending".to_string()),
        apply_at: Set(now),
        process_at: Set(None),
        complete_at: Set(None),
        close_at: Set(None),
        reject_reason: Set(None),
        processor_id: Set(None),
        processor_name: Set(None),
        timeout_at: Set(timeout_at),
        is_timeout: Set(false),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    CAfterSale::insert(after_sale)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    for item in params.items {
        let item_model = c_after_sale_item::ActiveModel {
            id: Set(generate_id()),
            after_sale_id: Set(after_sale_id),
            order_item_id: Set(item.order_item_id),
            product_id: Set(item.product_id),
            product_name: Set(item.product_name),
            sku_id: Set(item.sku_id),
            sku_name: Set(item.sku_name),
            quantity: Set(item.quantity),
            unit_price: Set(item.unit_price),
            refund_amount: Set(item.refund_amount),
            created_at: Set(Some(now)),
        };
        let _ = CAfterSaleItem::insert(item_model).exec(db).await;
    }

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(after_sale_id),
        old_status: Set(None),
        new_status: Set("pending".to_string()),
        operator_type: Set("consumer".to_string()),
        operator_id: Set(Some(params.consumer_id)),
        operator_name: Set(None),
        remark: Set(Some("提交售后申请".to_string())),
        created_at: Set(Some(now)),
    };
    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(AfterSaleModel {
        id: after_sale_id,
        after_sale_no,
        order_id: params.order_id,
        order_no: params.order_no.clone(),
        consumer_id: params.consumer_id,
        r#type: type_str,
        reason: params.reason.clone(),
        description: params.description.clone(),
        evidence_urls: params.evidence_urls.as_ref().map(|urls| serde_json::to_value(urls).unwrap_or_default()),
        refund_amount: total_refund.to_string(),
        status: "pending".to_string(),
        apply_at: now,
        process_at: None,
        complete_at: None,
        close_at: None,
        reject_reason: None,
        processor_id: None,
        processor_name: None,
        timeout_at,
        is_timeout: false,
        created_at: Some(now),
    })
}

pub async fn audit(params: AuditAfterSaleParams) -> Result<AfterSaleModel> {
    let db = DB_WRITE().await;

    let after_sale = CAfterSale::find_by_id(params.after_sale_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("售后单不存在"))?;

    if after_sale.status != "pending" {
        return Err(Error::bad_request("售后单状态不正确"));
    }

    let now = Local::now().naive_local();
    let old_status = after_sale.status.clone();
    let new_status = if params.agree { "agreed" } else { "rejected" };

    let mut active_model: c_after_sale::ActiveModel = after_sale.clone().into();
    active_model.status = Set(new_status.to_string());
    active_model.process_at = Set(Some(now));
    active_model.processor_id = Set(Some(params.processor_id));
    active_model.processor_name = Set(Some(params.processor_name.clone()));
    active_model.reject_reason = Set(params.reject_reason.clone());
    active_model.updated_at = Set(Some(now));

    if !params.agree {
        active_model.close_at = Set(Some(now));
    }

    let updated = CAfterSale::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(params.after_sale_id),
        old_status: Set(Some(old_status)),
        new_status: Set(new_status.to_string()),
        operator_type: Set("admin".to_string()),
        operator_id: Set(Some(params.processor_id)),
        operator_name: Set(Some(params.processor_name)),
        remark: Set(params.reject_reason.clone().or(Some(if params.agree { "审核通过" } else { "审核拒绝" }.to_string()))),
        created_at: Set(Some(now)),
    };
    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(AfterSaleModel {
        id: updated.id,
        after_sale_no: updated.after_sale_no,
        order_id: updated.order_id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        r#type: updated.r#type,
        reason: updated.reason,
        description: updated.description,
        evidence_urls: updated.evidence_urls,
        refund_amount: updated.refund_amount.to_string(),
        status: updated.status,
        apply_at: updated.apply_at,
        process_at: updated.process_at,
        complete_at: updated.complete_at,
        close_at: updated.close_at,
        reject_reason: updated.reject_reason,
        processor_id: updated.processor_id,
        processor_name: updated.processor_name,
        timeout_at: updated.timeout_at,
        is_timeout: updated.is_timeout,
        created_at: updated.created_at,
    })
}

pub async fn get_detail(after_sale_id: i64) -> Result<AfterSaleDetailModel> {
    let db = DB_WRITE().await;

    let after_sale = CAfterSale::find_by_id(after_sale_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("售后单不存在"))?;

    let items = CAfterSaleItem::find()
        .filter(c_after_sale_item::Column::AfterSaleId.eq(after_sale_id))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let refunds = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::AfterSaleId.eq(after_sale_id))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let logistics = CAfterSaleLogistics::find()
        .filter(c_after_sale_logistics::Column::AfterSaleId.eq(after_sale_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let status_logs = CAfterSaleStatusLog::find()
        .filter(c_after_sale_status_log::Column::AfterSaleId.eq(after_sale_id))
        .order_by_asc(c_after_sale_status_log::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(AfterSaleDetailModel {
        after_sale: AfterSaleModel {
            id: after_sale.id,
            after_sale_no: after_sale.after_sale_no,
            order_id: after_sale.order_id,
            order_no: after_sale.order_no,
            consumer_id: after_sale.consumer_id,
            r#type: after_sale.r#type,
            reason: after_sale.reason,
            description: after_sale.description,
            evidence_urls: after_sale.evidence_urls,
            refund_amount: after_sale.refund_amount.to_string(),
            status: after_sale.status,
            apply_at: after_sale.apply_at,
            process_at: after_sale.process_at,
            complete_at: after_sale.complete_at,
            close_at: after_sale.close_at,
            reject_reason: after_sale.reject_reason,
            processor_id: after_sale.processor_id,
            processor_name: after_sale.processor_name,
            timeout_at: after_sale.timeout_at,
            is_timeout: after_sale.is_timeout,
            created_at: after_sale.created_at,
        },
        items: items
            .into_iter()
            .map(|i| AfterSaleItemModel {
                id: i.id,
                after_sale_id: i.after_sale_id,
                order_item_id: i.order_item_id,
                product_id: i.product_id,
                product_name: i.product_name,
                sku_id: i.sku_id,
                sku_name: i.sku_name,
                quantity: i.quantity,
                unit_price: i.unit_price.to_string(),
                refund_amount: i.refund_amount.to_string(),
            })
            .collect(),
        refunds: refunds
            .into_iter()
            .map(|r| AfterSaleRefundModel {
                id: r.id,
                after_sale_id: r.after_sale_id,
                refund_no: r.refund_no,
                transaction_id: r.transaction_id,
                refund_channel: r.refund_channel,
                refund_amount: r.refund_amount.to_string(),
                status: r.status,
                refund_at: r.refund_at,
                fail_reason: r.fail_reason,
                retry_count: r.retry_count,
            })
            .collect(),
        logistics: logistics.map(|l| AfterSaleLogisticsModel {
            id: l.id,
            after_sale_id: l.after_sale_id,
            logistics_type: l.logistics_type,
            logistics_company: l.logistics_company,
            tracking_no: l.tracking_no,
            sender_name: l.sender_name,
            sender_phone: l.sender_phone,
            sender_address: l.sender_address,
            receiver_name: l.receiver_name,
            receiver_phone: l.receiver_phone,
            receiver_address: l.receiver_address,
            status: l.status,
            shipped_at: l.shipped_at,
            received_at: l.received_at,
            tracking_info: l.tracking_info,
        }),
        status_logs: status_logs
            .into_iter()
            .map(|s| AfterSaleStatusLogModel {
                id: s.id,
                after_sale_id: s.after_sale_id,
                old_status: s.old_status,
                new_status: s.new_status,
                operator_type: s.operator_type,
                operator_id: s.operator_id,
                operator_name: s.operator_name,
                remark: s.remark,
                created_at: s.created_at,
            })
            .collect(),
    })
}

pub async fn list(params: AfterSaleListParams) -> Result<(Vec<AfterSaleModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(consumer_id) = params.consumer_id {
        conditions = conditions.add(c_after_sale::Column::ConsumerId.eq(consumer_id));
    }
    if let Some(order_id) = params.order_id {
        conditions = conditions.add(c_after_sale::Column::OrderId.eq(order_id));
    }
    if let Some(status) = &params.status {
        conditions = conditions.add(c_after_sale::Column::Status.eq(status.clone()));
    }
    if let Some(r#type) = &params.r#type {
        conditions = conditions.add(c_after_sale::Column::Type.eq(r#type.clone()));
    }
    if let Some(start_time) = params.start_time {
        let naive = chrono::DateTime::from_timestamp(start_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_after_sale::Column::ApplyAt.gte(naive));
    }
    if let Some(end_time) = params.end_time {
        let naive = chrono::DateTime::from_timestamp(end_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_after_sale::Column::ApplyAt.lte(naive));
    }

    let paginator = CAfterSale::find()
        .filter(conditions)
        .order_by_desc(c_after_sale::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let after_sales = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<AfterSaleModel> = after_sales
        .into_iter()
        .map(|a| AfterSaleModel {
            id: a.id,
            after_sale_no: a.after_sale_no,
            order_id: a.order_id,
            order_no: a.order_no,
            consumer_id: a.consumer_id,
            r#type: a.r#type,
            reason: a.reason,
            description: a.description,
            evidence_urls: a.evidence_urls,
            refund_amount: a.refund_amount.to_string(),
            status: a.status,
            apply_at: a.apply_at,
            process_at: a.process_at,
            complete_at: a.complete_at,
            close_at: a.close_at,
            reject_reason: a.reject_reason,
            processor_id: a.processor_id,
            processor_name: a.processor_name,
            timeout_at: a.timeout_at,
            is_timeout: a.is_timeout,
            created_at: a.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn close(after_sale_id: i64, operator_type: &str, operator_id: Option<i64>, reason: Option<String>) -> Result<AfterSaleModel> {
    let db = DB_WRITE().await;

    let after_sale = CAfterSale::find_by_id(after_sale_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("售后单不存在"))?;

    if !["pending", "processing", "agreed"].contains(&after_sale.status.as_str()) {
        return Err(Error::bad_request("售后单状态不正确"));
    }

    let now = Local::now().naive_local();
    let old_status = after_sale.status.clone();

    let mut active_model: c_after_sale::ActiveModel = after_sale.into();
    active_model.status = Set("closed".to_string());
    active_model.close_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    let updated = CAfterSale::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(after_sale_id),
        old_status: Set(Some(old_status)),
        new_status: Set("closed".to_string()),
        operator_type: Set(operator_type.to_string()),
        operator_id: Set(operator_id),
        operator_name: Set(None),
        remark: Set(reason),
        created_at: Set(Some(now)),
    };
    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(AfterSaleModel {
        id: updated.id,
        after_sale_no: updated.after_sale_no,
        order_id: updated.order_id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        r#type: updated.r#type,
        reason: updated.reason,
        description: updated.description,
        evidence_urls: updated.evidence_urls,
        refund_amount: updated.refund_amount.to_string(),
        status: updated.status,
        apply_at: updated.apply_at,
        process_at: updated.process_at,
        complete_at: updated.complete_at,
        close_at: updated.close_at,
        reject_reason: updated.reject_reason,
        processor_id: updated.processor_id,
        processor_name: updated.processor_name,
        timeout_at: updated.timeout_at,
        is_timeout: updated.is_timeout,
        created_at: updated.created_at,
    })
}
