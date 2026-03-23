use crate::common::error::Error;
use crate::domain::model::m_after_sale::*;
use crate::domain::entity::prelude::{CAfterSale, CAfterSaleRefund};
use crate::domain::entity::{c_after_sale, c_after_sale_refund};
use crate::model::prelude::*;
use sea_orm::*;
use rust_decimal::Decimal;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_refund_no() -> String {
    format!("RF{}{}", chrono::Utc::now().format("%Y%m%d%H%M%S"), generate_id() % 10000)
}

pub async fn create_refund(params: CreateRefundParams) -> Result<AfterSaleRefundModel> {
    let db = DB_WRITE().await;

    let after_sale = CAfterSale::find_by_id(params.after_sale_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("售后单不存在"))?;

    if !["agreed", "processing"].contains(&after_sale.status.as_str()) {
        return Err(Error::bad_request("售后单状态不正确，无法退款"));
    }

    let now = Local::now().naive_local();
    let refund_no = generate_refund_no();

    let refund = c_after_sale_refund::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(params.after_sale_id),
        refund_no: Set(refund_no.clone()),
        transaction_id: Set(params.transaction_id.clone()),
        refund_channel: Set(params.refund_channel.clone()),
        refund_amount: Set(params.refund_amount),
        status: Set("pending".to_string()),
        refund_at: Set(None),
        fail_reason: Set(None),
        retry_count: Set(0),
        next_retry_at: Set(None),
        callback_data: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    let inserted = CAfterSaleRefund::insert(refund)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    let mut active_model: c_after_sale::ActiveModel = after_sale.into();
    active_model.status = Set("processing".to_string());
    active_model.updated_at = Set(Some(now));

    let _ = CAfterSale::update(active_model).exec(db).await;

    Ok(AfterSaleRefundModel {
        id: inserted.last_insert_id,
        after_sale_id: params.after_sale_id,
        refund_no,
        transaction_id: params.transaction_id,
        refund_channel: params.refund_channel,
        refund_amount: params.refund_amount.to_string(),
        status: "pending".to_string(),
        refund_at: None,
        fail_reason: None,
        retry_count: 0,
    })
}

pub async fn handle_callback(params: RefundCallbackParams) -> Result<AfterSaleRefundModel> {
    let db = DB_WRITE().await;

    let refund = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::RefundNo.eq(&params.refund_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("退款单不存在"))?;

    let now = Local::now().naive_local();

    let mut active_model: c_after_sale_refund::ActiveModel = refund.into();
    active_model.status = Set(params.status.clone());
    active_model.callback_data = Set(params.callback_data);
    active_model.fail_reason = Set(params.fail_reason.clone());
    active_model.updated_at = Set(Some(now));

    if params.status == "success" {
        active_model.refund_at = Set(Some(now));
        active_model.transaction_id = Set(params.transaction_id);
    }

    let updated = CAfterSaleRefund::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    if params.status == "success" {
        let all_refunds = CAfterSaleRefund::find()
            .filter(c_after_sale_refund::Column::AfterSaleId.eq(updated.after_sale_id))
            .all(db)
            .await
            .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

        let all_success = all_refunds.iter().all(|r| r.status == "success");

        if all_success {
            let after_sale = CAfterSale::find_by_id(updated.after_sale_id)
                .one(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            if let Some(a) = after_sale {
                let mut as_model: c_after_sale::ActiveModel = a.into();
                as_model.status = Set("completed".to_string());
                as_model.complete_at = Set(Some(now));
                as_model.updated_at = Set(Some(now));
                let _ = CAfterSale::update(as_model).exec(db).await;
            }
        }
    }

    Ok(AfterSaleRefundModel {
        id: updated.id,
        after_sale_id: updated.after_sale_id,
        refund_no: updated.refund_no,
        transaction_id: updated.transaction_id,
        refund_channel: updated.refund_channel,
        refund_amount: updated.refund_amount.to_string(),
        status: updated.status,
        refund_at: updated.refund_at,
        fail_reason: updated.fail_reason,
        retry_count: updated.retry_count,
    })
}

pub async fn retry_failed_refunds() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let failed_refunds = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::Status.eq("failed"))
        .filter(c_after_sale_refund::Column::RetryCount.lt(3))
        .filter(
            Condition::any()
                .add(c_after_sale_refund::Column::NextRetryAt.is_null())
                .add(c_after_sale_refund::Column::NextRetryAt.lte(now))
        )
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut count = 0u64;
    for refund in failed_refunds {
        let mut active_model: c_after_sale_refund::ActiveModel = refund.into();
        active_model.status = Set("pending".to_string());
        active_model.retry_count = Set(active_model.retry_count.unwrap() + 1);
        active_model.next_retry_at = Set(Some(now + chrono::Duration::hours(1)));
        active_model.updated_at = Set(Some(now));

        if CAfterSaleRefund::update(active_model).exec(db).await.is_ok() {
            count += 1;
        }
    }

    Ok(count)
}

pub async fn get_refund_by_no(refund_no: &str) -> Result<Option<AfterSaleRefundModel>> {
    let db = DB_WRITE().await;

    let refund = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::RefundNo.eq(refund_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(refund.map(|r| AfterSaleRefundModel {
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
    }))
}

pub async fn get_after_sale_refunds(after_sale_id: i64) -> Result<Vec<AfterSaleRefundModel>> {
    let db = DB_WRITE().await;

    let refunds = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::AfterSaleId.eq(after_sale_id))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(refunds
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
        .collect())
}

pub async fn calculate_total_refund_amount(after_sale_id: i64) -> Result<Decimal> {
    let db = DB_WRITE().await;

    let refunds = CAfterSaleRefund::find()
        .filter(c_after_sale_refund::Column::AfterSaleId.eq(after_sale_id))
        .filter(c_after_sale_refund::Column::Status.eq("success"))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(refunds.iter().map(|r| r.refund_amount).sum())
}

#[derive(Debug, Clone)]
pub struct RefundListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
    pub refund_no: Option<String>,
}

pub async fn list_refunds(params: RefundListParams) -> Result<(Vec<AfterSaleRefundModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(status) = &params.status {
        conditions = conditions.add(c_after_sale_refund::Column::Status.eq(status.clone()));
    }
    if let Some(refund_no) = &params.refund_no {
        conditions = conditions.add(c_after_sale_refund::Column::RefundNo.like(format!("%{}%", refund_no)));
    }

    let paginator = CAfterSaleRefund::find()
        .filter(conditions)
        .order_by_desc(c_after_sale_refund::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let refunds = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<AfterSaleRefundModel> = refunds
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
        .collect();

    Ok((items, total))
}
