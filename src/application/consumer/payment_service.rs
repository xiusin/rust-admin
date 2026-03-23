use crate::common::error::Error;
use crate::domain::model::m_payment::*;
use crate::domain::entity::prelude::CPaymentOrder;
use crate::domain::entity::c_payment_order;
use crate::model::prelude::*;
use rust_decimal::Decimal;
use sea_orm::*;
use chrono::{Local, Duration};

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_order_no() -> String {
    format!("P{}{}", chrono::Utc::now().format("%Y%m%d%H%M%S"), generate_id() % 10000)
}

pub async fn create_order(params: CreatePaymentParams) -> Result<PaymentOrderModel> {
    let db = DB_WRITE().await;

    if params.amount <= Decimal::ZERO {
        return Err(Error::bad_request("支付金额必须大于0"));
    }

    let now = Local::now().naive_local();
    let expires_at = now + Duration::hours(2);
    let order_no = params.order_no.clone().unwrap_or_else(|| generate_order_no());

    let existing = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(&order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("订单号已存在"));
    }

    let id = generate_id();
    let order = c_payment_order::ActiveModel {
        id: Set(id),
        consumer_id: Set(params.consumer_id),
        order_no: Set(order_no.clone()),
        payment_method: Set(serde_json::to_string(&params.payment_method).unwrap_or_default()),
        payment_type: Set(serde_json::to_string(&params.payment_type).unwrap_or_default()),
        amount: Set(params.amount),
        status: Set("pending".to_string()),
        transaction_id: Set(None),
        callback_data: Set(None),
        paid_at: Set(None),
        closed_at: Set(None),
        expires_at: Set(Some(expires_at)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    CPaymentOrder::insert(order)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(PaymentOrderModel {
        id,
        order_no,
        consumer_id: params.consumer_id,
        payment_method: serde_json::to_string(&params.payment_method).unwrap_or_default(),
        payment_type: serde_json::to_string(&params.payment_type).unwrap_or_default(),
        amount: params.amount,
        status: "pending".to_string(),
        transaction_id: None,
        callback_data: None,
        paid_at: None,
        closed_at: None,
        expires_at: Some(expires_at),
        created_at: Some(now),
        updated_at: Some(now),
    })
}

pub async fn get_order(order_no: &str) -> Result<Option<PaymentOrderModel>> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(order.map(|o| PaymentOrderModel {
        id: o.id,
        order_no: o.order_no,
        consumer_id: o.consumer_id,
        payment_method: o.payment_method,
        payment_type: o.payment_type,
        amount: o.amount,
        status: o.status,
        transaction_id: o.transaction_id,
        callback_data: o.callback_data,
        paid_at: o.paid_at,
        closed_at: o.closed_at,
        expires_at: o.expires_at,
        created_at: o.created_at,
        updated_at: o.updated_at,
    }))
}

pub async fn get_order_by_id(id: i64) -> Result<Option<PaymentOrderModel>> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(order.map(|o| PaymentOrderModel {
        id: o.id,
        order_no: o.order_no,
        consumer_id: o.consumer_id,
        payment_method: o.payment_method,
        payment_type: o.payment_type,
        amount: o.amount,
        status: o.status,
        transaction_id: o.transaction_id,
        callback_data: o.callback_data,
        paid_at: o.paid_at,
        closed_at: o.closed_at,
        expires_at: o.expires_at,
        created_at: o.created_at,
        updated_at: o.updated_at,
    }))
}

pub async fn payment_callback(
    order_no: &str,
    transaction_id: String,
    callback_data: String,
) -> Result<PaymentOrderModel> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != "pending" {
        return Err(Error::bad_request(format!("订单状态不正确: {}", order.status)));
    }

    if let Some(expires_at) = order.expires_at {
        if expires_at < Local::now().naive_local() {
            return Err(Error::bad_request("订单已过期"));
        }
    }

    let now = Local::now().naive_local();
    let mut active_model: c_payment_order::ActiveModel = order.clone().into();
    active_model.status = Set("success".to_string());
    active_model.transaction_id = Set(Some(transaction_id));
    active_model.callback_data = Set(Some(callback_data));
    active_model.paid_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    let updated = CPaymentOrder::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(PaymentOrderModel {
        id: updated.id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        payment_method: updated.payment_method,
        payment_type: updated.payment_type,
        amount: updated.amount,
        status: updated.status,
        transaction_id: updated.transaction_id,
        callback_data: updated.callback_data,
        paid_at: updated.paid_at,
        closed_at: updated.closed_at,
        expires_at: updated.expires_at,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}

pub async fn close_order(order_no: &str) -> Result<PaymentOrderModel> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != "pending" {
        return Err(Error::bad_request(format!("订单状态不正确: {}", order.status)));
    }

    let now = Local::now().naive_local();
    let mut active_model: c_payment_order::ActiveModel = order.into();
    active_model.status = Set("closed".to_string());
    active_model.closed_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    let updated = CPaymentOrder::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(PaymentOrderModel {
        id: updated.id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        payment_method: updated.payment_method,
        payment_type: updated.payment_type,
        amount: updated.amount,
        status: updated.status,
        transaction_id: updated.transaction_id,
        callback_data: updated.callback_data,
        paid_at: updated.paid_at,
        closed_at: updated.closed_at,
        expires_at: updated.expires_at,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}

pub async fn refund(params: RefundParams) -> Result<PaymentOrderModel> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(&params.order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != "success" {
        return Err(Error::bad_request("只能退款已支付成功的订单"));
    }

    if params.refund_amount > order.amount {
        return Err(Error::bad_request("退款金额不能超过订单金额"));
    }

    if params.refund_amount <= Decimal::ZERO {
        return Err(Error::bad_request("退款金额必须大于0"));
    }

    let now = Local::now().naive_local();
    let mut active_model: c_payment_order::ActiveModel = order.into();
    active_model.status = Set("refunded".to_string());
    active_model.updated_at = Set(Some(now));

    let updated = CPaymentOrder::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(PaymentOrderModel {
        id: updated.id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        payment_method: updated.payment_method,
        payment_type: updated.payment_type,
        amount: updated.amount,
        status: updated.status,
        transaction_id: updated.transaction_id,
        callback_data: updated.callback_data,
        paid_at: updated.paid_at,
        closed_at: updated.closed_at,
        expires_at: updated.expires_at,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}

pub async fn mark_failed(order_no: &str, reason: Option<String>) -> Result<PaymentOrderModel> {
    let db = DB_WRITE().await;

    let order = CPaymentOrder::find()
        .filter(c_payment_order::Column::OrderNo.eq(order_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != "pending" {
        return Err(Error::bad_request(format!("订单状态不正确: {}", order.status)));
    }

    let now = Local::now().naive_local();
    let mut active_model: c_payment_order::ActiveModel = order.into();
    active_model.status = Set("failed".to_string());
    active_model.callback_data = Set(reason);
    active_model.updated_at = Set(Some(now));

    let updated = CPaymentOrder::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(PaymentOrderModel {
        id: updated.id,
        order_no: updated.order_no,
        consumer_id: updated.consumer_id,
        payment_method: updated.payment_method,
        payment_type: updated.payment_type,
        amount: updated.amount,
        status: updated.status,
        transaction_id: updated.transaction_id,
        callback_data: updated.callback_data,
        paid_at: updated.paid_at,
        closed_at: updated.closed_at,
        expires_at: updated.expires_at,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}

pub async fn close_expired_orders() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let expired_orders = CPaymentOrder::find()
        .filter(c_payment_order::Column::Status.eq("pending"))
        .filter(c_payment_order::Column::ExpiresAt.lt(now))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut count = 0u64;
    for order in expired_orders {
        let mut active_model: c_payment_order::ActiveModel = order.into();
        active_model.status = Set("closed".to_string());
        active_model.closed_at = Set(Some(now));
        active_model.updated_at = Set(Some(now));

        if CPaymentOrder::update(active_model)
            .exec(db)
            .await
            .is_ok()
        {
            count += 1;
        }
    }

    Ok(count)
}

pub async fn list_orders(
    params: PaymentOrderSearchParams,
) -> Result<(Vec<PaymentOrderModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(consumer_id) = params.consumer_id {
        conditions = conditions.add(c_payment_order::Column::ConsumerId.eq(consumer_id));
    }
    if let Some(status) = &params.status {
        conditions = conditions.add(c_payment_order::Column::Status.eq(status.clone()));
    }
    if let Some(order_no) = &params.order_no {
        conditions = conditions.add(c_payment_order::Column::OrderNo.like(format!("%{}%", order_no)));
    }
    if let Some(payment_method) = &params.payment_method {
        conditions = conditions.add(c_payment_order::Column::PaymentMethod.like(format!("%{}%", payment_method)));
    }
    if let Some(start_time) = params.start_time {
        let naive = chrono::DateTime::from_timestamp(start_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_payment_order::Column::CreatedAt.gte(naive));
    }
    if let Some(end_time) = params.end_time {
        let naive = chrono::DateTime::from_timestamp(end_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_payment_order::Column::CreatedAt.lte(naive));
    }

    let paginator = CPaymentOrder::find()
        .filter(conditions)
        .order_by_desc(c_payment_order::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let orders = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<PaymentOrderModel> = orders
        .into_iter()
        .map(|o| PaymentOrderModel {
            id: o.id,
            order_no: o.order_no,
            consumer_id: o.consumer_id,
            payment_method: o.payment_method,
            payment_type: o.payment_type,
            amount: o.amount,
            status: o.status,
            transaction_id: o.transaction_id,
            callback_data: o.callback_data,
            paid_at: o.paid_at,
            closed_at: o.closed_at,
            expires_at: o.expires_at,
            created_at: o.created_at,
            updated_at: o.updated_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn get_payment_statistics(consumer_id: Option<i64>) -> Result<PaymentStatistics> {
    let db = DB_WRITE().await;

    let mut conditions = Condition::all();
    if let Some(cid) = consumer_id {
        conditions = conditions.add(c_payment_order::Column::ConsumerId.eq(cid));
    }

    let orders = CPaymentOrder::find()
        .filter(conditions)
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut total_amount = Decimal::ZERO;
    let mut success_amount = Decimal::ZERO;
    let mut pending_amount = Decimal::ZERO;
    let mut refunded_amount = Decimal::ZERO;
    let mut success_count = 0u64;
    let mut pending_count = 0u64;
    let mut failed_count = 0u64;

    for order in orders {
        total_amount += order.amount;
        match order.status.as_str() {
            "success" => {
                success_amount += order.amount;
                success_count += 1;
            }
            "pending" => {
                pending_amount += order.amount;
                pending_count += 1;
            }
            "failed" | "closed" => {
                failed_count += 1;
            }
            "refunded" => {
                refunded_amount += order.amount;
            }
            _ => {}
        }
    }

    Ok(PaymentStatistics {
        total_amount,
        success_amount,
        pending_amount,
        refunded_amount,
        success_count,
        pending_count,
        failed_count,
    })
}
