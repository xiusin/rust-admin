use crate::service::prelude::*;
use crate::domain::entity::{ppt_subscription_plan, ppt_user_subscription, ppt_payment_order};
use crate::domain::model::m_ppt_subscription::*;
use crate::domain::args::a_ppt_subscription::*;
use crate::common::error::{Error, Result};
use rust_decimal::Decimal;

pub async fn create_order(
    user_id: i64,
    plan_id: i64,
    payment_method: String,
    coupon_code: Option<String>,
) -> Result<PaymentOrderInfo> {
    let db = DB().await;
    
    let plan = ppt_subscription_plan::Entity::find_by_id(plan_id)
        .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
        .filter(ppt_subscription_plan::Column::IsActive.eq(1))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在或已下架"))?;
    
    let now = Local::now().naive_utc();
    let order_no = generate_order_no();
    let expires_at = now + chrono::Duration::hours(2);
    
    let (amount, discount_amount) = calculate_order_amount(&plan, &coupon_code);
    
    let id = GID().await;
    
    let order = ppt_payment_order::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        order_no: sea_orm::ActiveValue::Set(order_no.clone()),
        user_id: sea_orm::ActiveValue::Set(user_id),
        plan_id: sea_orm::ActiveValue::Set(plan_id),
        subscription_id: sea_orm::ActiveValue::Set(None),
        amount: sea_orm::ActiveValue::Set(amount),
        original_amount: sea_orm::ActiveValue::Set(plan.price),
        discount_amount: sea_orm::ActiveValue::Set(discount_amount),
        payment_method: sea_orm::ActiveValue::Set(payment_method.clone()),
        payment_channel: sea_orm::ActiveValue::Set(None),
        transaction_id: sea_orm::ActiveValue::Set(None),
        callback_data: sea_orm::ActiveValue::Set(None),
        paid_at: sea_orm::ActiveValue::Set(None),
        refunded_at: sea_orm::ActiveValue::Set(None),
        refund_amount: sea_orm::ActiveValue::Set(None),
        refund_reason: sea_orm::ActiveValue::Set(None),
        status: sea_orm::ActiveValue::Set("pending".to_string()),
        expires_at: sea_orm::ActiveValue::Set(expires_at),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        deleted_at: sea_orm::ActiveValue::Set(None),
    };
    
    let order = order.insert(db).await?;
    
    let (payment_url, qr_code) = generate_payment_info(&order_no, &payment_method, amount)?;
    
    Ok(PaymentOrderInfo {
        id: order.id,
        order_no: order.order_no,
        user_id: order.user_id,
        plan_id: order.plan_id,
        plan_name: plan.name,
        subscription_id: order.subscription_id,
        amount: order.amount,
        original_amount: order.original_amount,
        discount_amount: order.discount_amount,
        payment_method: order.payment_method,
        payment_channel: order.payment_channel,
        transaction_id: order.transaction_id,
        paid_at: order.paid_at,
        refunded_at: order.refunded_at,
        refund_amount: order.refund_amount,
        refund_reason: order.refund_reason,
        status: order.status,
        expires_at: order.expires_at,
        payment_url: Some(payment_url),
        qr_code: Some(qr_code),
        created_at: order.created_at,
    })
}

pub async fn handle_payment_callback(
    payment_method: &str,
    callback_data: &str,
) -> Result<()> {
    let db = DB().await;
    
    let (order_no, transaction_id, amount) = parse_callback_data(payment_method, callback_data)?;
    
    let order = ppt_payment_order::Entity::find()
        .filter(ppt_payment_order::Column::OrderNo.eq(&order_no))
        .filter(ppt_payment_order::Column::Status.eq("pending"))
        .filter(ppt_payment_order::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在或已处理"))?;
    
    if order.amount != amount {
        return Err(Error::bad_request("支付金额不匹配"));
    }
    
    let now = Local::now().naive_utc();
    
    let mut order_model: ppt_payment_order::ActiveModel = order.into();
    order_model.transaction_id = sea_orm::ActiveValue::Set(Some(transaction_id));
    order_model.callback_data = sea_orm::ActiveValue::Set(Some(callback_data.to_string()));
    order_model.paid_at = sea_orm::ActiveValue::Set(Some(now));
    order_model.status = sea_orm::ActiveValue::Set("paid".to_string());
    order_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
    
    let order = order_model.update(db).await?;
    
    activate_subscription(order.user_id, order.plan_id, Some(order.id)).await?;
    
    Ok(())
}

pub async fn query_order_status(order_id: i64) -> Result<PaymentOrderInfo> {
    let db = DB().await;
    
    let order = ppt_payment_order::Entity::find_by_id(order_id)
        .filter(ppt_payment_order::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在"))?;
    
    let plan = ppt_subscription_plan::Entity::find_by_id(order.plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;
    
    Ok(PaymentOrderInfo {
        id: order.id,
        order_no: order.order_no,
        user_id: order.user_id,
        plan_id: order.plan_id,
        plan_name: plan.name,
        subscription_id: order.subscription_id,
        amount: order.amount,
        original_amount: order.original_amount,
        discount_amount: order.discount_amount,
        payment_method: order.payment_method,
        payment_channel: order.payment_channel,
        transaction_id: order.transaction_id,
        paid_at: order.paid_at,
        refunded_at: order.refunded_at,
        refund_amount: order.refund_amount,
        refund_reason: order.refund_reason,
        status: order.status,
        expires_at: order.expires_at,
        payment_url: None,
        qr_code: None,
        created_at: order.created_at,
    })
}

pub async fn query_order_by_no(order_no: &str) -> Result<PaymentOrderInfo> {
    let db = DB().await;
    
    let order = ppt_payment_order::Entity::find()
        .filter(ppt_payment_order::Column::OrderNo.eq(order_no))
        .filter(ppt_payment_order::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在"))?;
    
    query_order_status(order.id).await
}

pub async fn cancel_order(order_id: i64) -> Result<()> {
    let db = DB().await;
    
    let order = ppt_payment_order::Entity::find_by_id(order_id)
        .filter(ppt_payment_order::Column::Status.eq("pending"))
        .filter(ppt_payment_order::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在或已处理"))?;
    
    let now = Local::now().naive_utc();
    
    let mut order_model: ppt_payment_order::ActiveModel = order.into();
    order_model.status = sea_orm::ActiveValue::Set("cancelled".to_string());
    order_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
    
    order_model.update(db).await?;
    
    Ok(())
}

pub async fn refund_order(order_id: i64, reason: &str) -> Result<()> {
    let db = DB().await;
    
    let order = ppt_payment_order::Entity::find_by_id(order_id)
        .filter(ppt_payment_order::Column::Status.eq("paid"))
        .filter(ppt_payment_order::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在或未支付"))?;
    
    let now = Local::now().naive_utc();
    
    let refund_amount = order.amount;
    
    let saved_subscription_id = order.subscription_id;
    let mut order_model: ppt_payment_order::ActiveModel = order.into();
    order_model.refunded_at = sea_orm::ActiveValue::Set(Some(now));
    order_model.refund_amount = sea_orm::ActiveValue::Set(Some(refund_amount));
    order_model.refund_reason = sea_orm::ActiveValue::Set(Some(reason.to_string()));
    order_model.status = sea_orm::ActiveValue::Set("refunded".to_string());
    order_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
    
    order_model.update(db).await?;
    
    if let Some(subscription_id) = saved_subscription_id {
        let subscription = ppt_user_subscription::Entity::find_by_id(subscription_id)
            .one(db)
            .await?;
        
        if let Some(subscription) = subscription {
            let mut sub_model: ppt_user_subscription::ActiveModel = subscription.into();
            sub_model.status = sea_orm::ActiveValue::Set("cancelled".to_string());
            sub_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
            sub_model.update(db).await?;
        }
    }
    
    Ok(())
}

pub async fn list_orders(
    page: PageParams,
    search: PaymentOrderListArgs,
) -> Result<ListData<PaymentOrderListItem>> {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_payment_order::Column::DeletedAt.is_null());
    
    if let Some(user_id) = search.user_id {
        conditions = conditions.add(ppt_payment_order::Column::UserId.eq(user_id));
    }
    if let Some(ref status) = search.status {
        conditions = conditions.add(ppt_payment_order::Column::Status.eq(status));
    }
    if let Some(ref order_no) = search.order_no {
        conditions = conditions.add(ppt_payment_order::Column::OrderNo.contains(order_no));
    }
    
    let total = ppt_payment_order::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_payment_order::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_payment_order::Column::CreatedAt)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await?;
    
    let mut list = Vec::new();
    for item in items {
        let plan = ppt_subscription_plan::Entity::find_by_id(item.plan_id)
            .one(db)
            .await?;
        
        let plan_name = plan.map(|p| p.name).unwrap_or_default();
        
        list.push(PaymentOrderListItem {
            id: item.id,
            order_no: item.order_no,
            user_id: item.user_id,
            plan_id: item.plan_id,
            plan_name,
            amount: item.amount,
            payment_method: item.payment_method,
            status: item.status,
            paid_at: item.paid_at,
            created_at: item.created_at,
        });
    }
    
    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })
}

async fn activate_subscription(
    user_id: i64,
    plan_id: i64,
    order_id: Option<i64>,
) -> Result<()> {
    let db = DB().await;
    
    let plan = ppt_subscription_plan::Entity::find_by_id(plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;
    
    let now = Local::now().naive_utc();
    let expires_at = now + chrono::Duration::days(plan.duration_days as i64);
    
    let existing_subscription = ppt_user_subscription::Entity::find()
        .filter(ppt_user_subscription::Column::UserId.eq(user_id))
        .filter(ppt_user_subscription::Column::Status.eq("active"))
        .filter(ppt_user_subscription::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    
    if let Some(existing) = existing_subscription {
        let mut sub_model: ppt_user_subscription::ActiveModel = existing.into();
        sub_model.plan_id = sea_orm::ActiveValue::Set(plan_id);
        sub_model.order_id = sea_orm::ActiveValue::Set(order_id);
        sub_model.expires_at = sea_orm::ActiveValue::Set(expires_at);
        sub_model.ai_credits_total = sea_orm::ActiveValue::Set(plan.ai_credits);
        sub_model.ai_credits_remaining = sea_orm::ActiveValue::Set(plan.ai_credits);
        sub_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
        sub_model.update(db).await?;
    } else {
        let id = GID().await;
        
        let subscription = ppt_user_subscription::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            user_id: sea_orm::ActiveValue::Set(user_id),
            plan_id: sea_orm::ActiveValue::Set(plan_id),
            order_id: sea_orm::ActiveValue::Set(order_id),
            started_at: sea_orm::ActiveValue::Set(now),
            expires_at: sea_orm::ActiveValue::Set(expires_at),
            ai_credits_total: sea_orm::ActiveValue::Set(plan.ai_credits),
            ai_credits_used: sea_orm::ActiveValue::Set(0),
            ai_credits_remaining: sea_orm::ActiveValue::Set(plan.ai_credits),
            projects_count: sea_orm::ActiveValue::Set(0),
            auto_renew: sea_orm::ActiveValue::Set(0),
            status: sea_orm::ActiveValue::Set("active".to_string()),
            created_at: sea_orm::ActiveValue::Set(Some(now)),
            updated_at: sea_orm::ActiveValue::Set(Some(now)),
            deleted_at: sea_orm::ActiveValue::Set(None),
        };
        
        let subscription = subscription.insert(db).await?;
        
        if let Some(order_id) = order_id {
            let order = ppt_payment_order::Entity::find_by_id(order_id)
                .one(db)
                .await?;
            
            if let Some(order) = order {
                let mut order_model: ppt_payment_order::ActiveModel = order.into();
                order_model.subscription_id = sea_orm::ActiveValue::Set(Some(subscription.id));
                order_model.update(db).await?;
            }
        }
    }
    
    Ok(())
}

fn generate_order_no() -> String {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    let random: String = (0..6)
        .map(|_| rand::random::<u8>() % 10)
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect();
    format!("PPT{}{}", timestamp, random)
}

fn calculate_order_amount(
    plan: &ppt_subscription_plan::Model,
    _coupon_code: &Option<String>,
) -> (Decimal, Decimal) {
    let original_amount = plan.price;
    let discount_amount = plan.original_price
        .map(|op| op - plan.price)
        .unwrap_or(Decimal::ZERO);
    
    (original_amount, discount_amount)
}

fn generate_payment_info(
    order_no: &str,
    payment_method: &str,
    _amount: Decimal,
) -> Result<(String, String)> {
    match payment_method {
        "alipay" => {
            let payment_url = format!("https://openapi.alipay.com/gateway.do?order={}", order_no);
            let qr_code = format!("alipay://pay?order={}", order_no);
            Ok((payment_url, qr_code))
        }
        "wechat" => {
            let payment_url = format!("weixin://wxpay/bizpayurl?pr={}", order_no);
            let qr_code = format!("weixin://wxpay/bizpayurl?pr={}", order_no);
            Ok((payment_url, qr_code))
        }
        "balance" => {
            Ok((format!("/api/ppt/payment/balance/{}", order_no), String::new()))
        }
        _ => Err(Error::bad_request("不支持的支付方式")),
    }
}

fn parse_callback_data(
    payment_method: &str,
    callback_data: &str,
) -> Result<(String, String, Decimal)> {
    match payment_method {
        "alipay" => {
            let data: serde_json::Value = serde_json::from_str(callback_data)
                .map_err(|_| Error::bad_request("无效的回调数据"))?;
            
            let order_no = data["out_trade_no"]
                .as_str()
                .ok_or_else(|| Error::bad_request("缺少订单号"))?
                .to_string();
            let transaction_id = data["trade_no"]
                .as_str()
                .ok_or_else(|| Error::bad_request("缺少交易号"))?
                .to_string();
            let amount = data["total_amount"]
                .as_str()
                .and_then(|s| s.parse::<Decimal>().ok())
                .ok_or_else(|| Error::bad_request("缺少金额"))?;
            
            Ok((order_no, transaction_id, amount))
        }
        "wechat" => {
            let data: serde_json::Value = serde_json::from_str(callback_data)
                .map_err(|_| Error::bad_request("无效的回调数据"))?;
            
            let order_no = data["out_trade_no"]
                .as_str()
                .ok_or_else(|| Error::bad_request("缺少订单号"))?
                .to_string();
            let transaction_id = data["transaction_id"]
                .as_str()
                .ok_or_else(|| Error::bad_request("缺少交易号"))?
                .to_string();
            let amount = Decimal::from(data["total_fee"]
                .as_i64()
                .ok_or_else(|| Error::bad_request("缺少金额"))?) / Decimal::from(100);
            
            Ok((order_no, transaction_id, amount))
        }
        _ => Err(Error::bad_request("不支持的支付方式")),
    }
}
