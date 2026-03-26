use crate::domain::entity::p_subscription;
use crate::domain::entity::p_subscription::Entity as SubscriptionEntity;
use crate::domain::entity::p_order::Entity as OrderEntity;
use crate::domain::entity::p_license;
use crate::domain::entity::p_license::Entity as LicenseEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use sea_orm::prelude::Expr;
use chrono::{Duration, Utc, NaiveDateTime};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

pub async fn create_from_order(order_id: i64) -> Result<i64, Error> {
    let db = DB().await;

    let order = OrderEntity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订单不存在"))?;

    if order.status != 1 {
        return Err(Error::bad_request("订单未支付"));
    }

    let max_id: Option<i64> = SubscriptionEntity::find()
        .select_only()
        .column_as(p_subscription::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;
    let now = Utc::now().naive_utc();
    let end_time = now + Duration::days(365);

    let subscription = p_subscription::ActiveModel {
        id: Set(new_id),
        user_id: Set(order.user_id),
        plugin_id: Set(order.plugin_id),
        plan_id: Set(order.plan_id),
        order_id: Set(Some(order_id)),
        license_id: Set(None),
        start_time: Set(now),
        end_time: Set(end_time),
        auto_renew: Set(0),
        status: Set(1),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    subscription.insert(db).await?;

    let license_key = Uuid::new_v4().to_string();

    let max_license_id: Option<i64> = LicenseEntity::find()
        .select_only()
        .column_as(p_license::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_license_id = max_license_id.unwrap_or(0) + 1;

    let license = p_license::ActiveModel {
        id: Set(new_license_id),
        license_key: Set(license_key),
        user_id: Set(order.user_id),
        plugin_id: Set(order.plugin_id),
        plan_id: Set(order.plan_id),
        order_id: Set(Some(order_id)),
        device_id: Set(String::new()),
        device_name: Set(None),
        device_type: Set(None),
        os_version: Set(None),
        app_version: Set(None),
        ip_address: Set(None),
        bind_count: Set(0),
        max_devices: Set(1),
        verify_count: Set(0),
        last_verify_time: Set(None),
        status: Set(1),
        start_time: Set(now),
        end_time: Set(end_time),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    license.insert(db).await?;

    SubscriptionEntity::update_many()
        .col_expr(p_subscription::Column::LicenseId, Expr::value(new_license_id))
        .filter(p_subscription::Column::Id.eq(new_id))
        .exec(db)
        .await?;

    Ok(new_id)
}

pub async fn list(user_id: i64, page_num: u32, page_size: u32) -> Result<(Vec<SubscriptionItem>, i64), Error> {
    let db = DB().await;

    let offset = (page_num - 1) * page_size;

    let total = SubscriptionEntity::find()
        .filter(p_subscription::Column::UserId.eq(user_id))
        .count(db)
        .await?;

    let subscriptions = SubscriptionEntity::find()
        .filter(p_subscription::Column::UserId.eq(user_id))
        .order_by_desc(p_subscription::Column::CreatedAt)
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let items: Vec<SubscriptionItem> = subscriptions
        .into_iter()
        .map(|s| SubscriptionItem {
            id: s.id,
            user_id: s.user_id,
            plugin_id: s.plugin_id,
            plan_id: s.plan_id,
            order_id: s.order_id,
            license_id: s.license_id,
            start_time: s.start_time,
            end_time: s.end_time,
            auto_renew: s.auto_renew,
            status: s.status,
            created_at: s.created_at,
        })
        .collect();

    Ok((items, total as i64))
}

#[derive(Debug, Serialize, Clone)]
pub struct SubscriptionItem {
    pub id: i64,
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub auto_renew: i32,
    pub status: i32,
    pub created_at: Option<NaiveDateTime>,
}

pub async fn renew(subscription_id: i64, extend_days: i32) -> Result<(), Error> {
    let db = DB().await;

    let subscription = SubscriptionEntity::find_by_id(subscription_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订阅不存在"))?;

    let new_end_time = subscription.end_time + Duration::days(extend_days as i64);
    let license_id = subscription.license_id;

    let mut active_model: p_subscription::ActiveModel = subscription.into();
    active_model.end_time = Set(new_end_time);
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    if let Some(license_id) = license_id {
        LicenseEntity::update_many()
            .col_expr(p_license::Column::EndTime, Expr::value(new_end_time))
            .col_expr(p_license::Column::UpdatedAt, Expr::value(Utc::now().naive_utc()))
            .filter(p_license::Column::Id.eq(license_id))
            .exec(db)
            .await?;
    }

    Ok(())
}

pub async fn cancel(subscription_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let subscription = SubscriptionEntity::find_by_id(subscription_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("订阅不存在"))?;

    let mut active_model: p_subscription::ActiveModel = subscription.into();
    active_model.status = Set(3);
    active_model.updated_at = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn check_expired() -> Result<(), Error> {
    let db = DB().await;
    let now = Utc::now().naive_utc();

    SubscriptionEntity::update_many()
        .col_expr(p_subscription::Column::Status, Expr::value(2))
        .filter(p_subscription::Column::EndTime.lt(now))
        .filter(p_subscription::Column::Status.eq(1))
        .exec(db)
        .await?;

    Ok(())
}