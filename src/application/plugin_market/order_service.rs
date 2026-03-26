use crate::domain::model::m_order::*;
use crate::domain::entity::p_order;
use crate::domain::entity::p_order::Entity as OrderEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::domain::entity::p_plan::Entity as PlanEntity;
use crate::domain::args::a_order::OrderSearchParams;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use chrono::{Duration, Utc};
use serde::Deserialize;

pub async fn create(user_id: i64, params: CreateOrderParams) -> Result<i64, Error> {
    let db = DB().await;

    let _plugin = PluginEntity::find_by_id(params.plugin_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    let plan = PlanEntity::find_by_id(params.plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;

    if plan.plugin_id != params.plugin_id {
        return Err(Error::bad_request("套餐不属于该插件"));
    }

    let order_no = generate_order_no();

    let max_id: Option<i64> = OrderEntity::find()
        .select_only()
        .column_as(p_order::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let price = plan.price.to_string().parse::<f64>().unwrap_or(0.0);

    let order = p_order::ActiveModel {
        id: Set(new_id),
        order_no: Set(order_no),
        user_id: Set(user_id),
        plugin_id: Set(params.plugin_id),
        plan_id: Set(params.plan_id),
        amount: Set(rust_decimal::Decimal::from_str_exact(&price.to_string()).unwrap_or_default()),
        original_amount: Set(plan.original_price),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        coupon_id: Set(params.coupon_id),
        payment_method: Set(params.payment_method),
        payment_time: Set(None),
        status: Set(0),
        expire_time: Set(Utc::now().naive_utc() + Duration::minutes(30)),
        created_at: Set(Some(Utc::now().naive_utc())),
        updated_at: Set(Some(Utc::now().naive_utc())),
    };

    order.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub coupon_id: Option<i64>,
    pub payment_method: i32,
}

pub async fn list(user_id: i64, params: OrderSearchParams) -> Result<(Vec<OrderListItem>, i64), Error> {
    let db = DB().await;

    let mut query = OrderEntity::find()
        .filter(p_order::Column::UserId.eq(user_id));

    if let Some(status) = params.status {
        query = query.filter(p_order::Column::Status.eq(status));
    }

    if let Some(payment_method) = params.payment_method {
        query = query.filter(p_order::Column::PaymentMethod.eq(payment_method));
    }

    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let offset = (page_num - 1) * page_size;

    let total = query.clone().count(db).await?;
    let orders = query
        .order_by_desc(p_order::Column::CreatedAt)
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let mut items: Vec<OrderListItem> = Vec::new();

    for order in orders {
        let plugin = PluginEntity::find_by_id(order.plugin_id).one(db).await?;
        let plan = PlanEntity::find_by_id(order.plan_id).one(db).await?;

        let plugin_name = plugin.as_ref().map(|p| p.name.clone()).unwrap_or_default();
        let plugin_cover = plugin.and_then(|p| p.cover_image);

        items.push(OrderListItem {
            id: order.id,
            order_no: order.order_no.clone(),
            user_id: order.user_id,
            user_name: None,
            plugin_id: order.plugin_id,
            plugin_name,
            plugin_cover,
            plan_id: order.plan_id,
            plan_name: plan.map(|p| p.name).unwrap_or_default(),
            amount: order.amount.to_string().parse().unwrap_or(0.0),
            original_amount: order.original_amount.to_string().parse().unwrap_or(0.0),
            discount_amount: order.discount_amount.to_string().parse().unwrap_or(0.0),
            payment_method: order.payment_method,
            payment_method_name: get_payment_method_name(order.payment_method),
            payment_time: order.payment_time,
            status: order.status,
            status_name: get_order_status_name(order.status),
            expire_time: order.expire_time,
            created_at: order.created_at,
        });
    }

    Ok((items, total as i64))
}

pub async fn detail(id: i64) -> Result<Option<p_order::Model>, Error> {
    let db = DB().await;
    let order = OrderEntity::find_by_id(id).one(db).await?;
    Ok(order)
}

pub async fn cancel(id: i64, user_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let order = OrderEntity::find()
        .filter(p_order::Column::Id.eq(id))
        .filter(p_order::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != 0 {
        return Err(Error::bad_request("只能取消待支付的订单"));
    }

    let mut active_model: p_order::ActiveModel = order.into();
    active_model.status = Set(2);
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn pay_callback(order_id: i64, payment_method: i32) -> Result<(), Error> {
    let db = DB().await;

    let order = OrderEntity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != 0 {
        return Err(Error::bad_request("订单状态不是待支付"));
    }

    let mut active_model: p_order::ActiveModel = order.into();
    active_model.status = Set(1);
    active_model.payment_method = Set(payment_method);
    active_model.payment_time = Set(Some(Utc::now().naive_utc()));
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

fn generate_order_no() -> String {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let random: u32 = rand::random();
    format!("PLM{}{:06}", timestamp, random % 1000000)
}