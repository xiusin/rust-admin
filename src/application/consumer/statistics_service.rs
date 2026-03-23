use crate::common::error::Error;
use crate::domain::model::m_user_extension::*;
use crate::domain::entity::prelude::CConsumerStatistics;
use crate::domain::entity::c_consumer_statistics;
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

pub async fn get_statistics(consumer_id: i64) -> Result<ConsumerStatisticsModel> {
    let db = DB_WRITE().await;

    let now = Local::now().naive_local();
    let current_month = now.format("%Y-%m").to_string();

    let stats = CConsumerStatistics::find()
        .filter(c_consumer_statistics::Column::ConsumerId.eq(consumer_id))
        .filter(c_consumer_statistics::Column::StatisticsMonth.eq(&current_month))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match stats {
        Some(s) => Ok(ConsumerStatisticsModel {
            consumer_id: s.consumer_id,
            total_consume: s.total_consume.to_string(),
            month_consume: s.month_consume.to_string(),
            year_consume: s.year_consume.to_string(),
            order_count: s.order_count,
            refund_count: s.refund_count,
            refund_amount: s.refund_amount.to_string(),
            last_order_at: s.last_order_at,
        }),
        None => {
            let total_stats = CConsumerStatistics::find()
                .filter(c_consumer_statistics::Column::ConsumerId.eq(consumer_id))
                .all(db)
                .await
                .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

            let (total_consume, order_count, refund_count, refund_amount, last_order_at) = total_stats
                .iter()
                .fold(
                    (Decimal::ZERO, 0i32, 0i32, Decimal::ZERO, None),
                    |(tc, oc, rc, ra, lo), s| {
                        (
                            tc + s.total_consume,
                            oc + s.order_count,
                            rc + s.refund_count,
                            ra + s.refund_amount,
                            s.last_order_at.or(lo),
                        )
                    },
                );

            Ok(ConsumerStatisticsModel {
                consumer_id,
                total_consume: total_consume.to_string(),
                month_consume: "0".to_string(),
                year_consume: "0".to_string(),
                order_count,
                refund_count,
                refund_amount: refund_amount.to_string(),
                last_order_at,
            })
        }
    }
}

pub async fn update_statistics(params: UpdateStatisticsParams) -> Result<()> {
    let db = DB_WRITE().await;

    let now = Local::now().naive_local();
    let current_month = now.format("%Y-%m").to_string();
    let _current_year = now.format("%Y").to_string();

    let stats = CConsumerStatistics::find()
        .filter(c_consumer_statistics::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_consumer_statistics::Column::StatisticsMonth.eq(&current_month))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let order_amount = params
        .order_amount
        .and_then(|a| a.parse::<Decimal>().ok())
        .unwrap_or(Decimal::ZERO);
    let refund_amt = params
        .refund_amount
        .and_then(|a| a.parse::<Decimal>().ok())
        .unwrap_or(Decimal::ZERO);

    match stats {
        Some(s) => {
            let mut active_model: c_consumer_statistics::ActiveModel = s.into();

            if params.is_refund {
                active_model.refund_count = Set(active_model.refund_count.unwrap() + 1);
                active_model.refund_amount = Set(active_model.refund_amount.unwrap() + refund_amt);
            } else if order_amount > Decimal::ZERO {
                active_model.order_count = Set(active_model.order_count.unwrap() + 1);
                active_model.month_consume = Set(active_model.month_consume.unwrap() + order_amount);
                active_model.year_consume = Set(active_model.year_consume.unwrap() + order_amount);
                active_model.total_consume = Set(active_model.total_consume.unwrap() + order_amount);
                active_model.last_order_at = Set(Some(now));
            }

            active_model.updated_at = Set(Some(now));

            CConsumerStatistics::update(active_model)
                .exec(db)
                .await
                .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;
        }
        None => {
            let (order_count, refund_count, total_consume, month_consume, year_consume, refund_amount) =
                if params.is_refund {
                    (0, 1, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, refund_amt)
                } else {
                    (1, 0, order_amount, order_amount, order_amount, Decimal::ZERO)
                };

            let new_stats = c_consumer_statistics::ActiveModel {
                id: Set(generate_id()),
                consumer_id: Set(params.consumer_id),
                total_consume: Set(total_consume),
                month_consume: Set(month_consume),
                year_consume: Set(year_consume),
                order_count: Set(order_count),
                refund_count: Set(refund_count),
                refund_amount: Set(refund_amount),
                last_order_at: Set(if params.is_refund { None } else { Some(now) }),
                statistics_month: Set(Some(current_month)),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };

            CConsumerStatistics::insert(new_stats)
                .exec(db)
                .await
                .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;
        }
    }

    Ok(())
}

pub async fn get_consume_trend(
    consumer_id: i64,
    months: u32,
) -> Result<Vec<(String, Decimal)>> {
    let db = DB_WRITE().await;

    let now = Local::now().naive_local();
    let mut result = Vec::new();

    for i in 0..months {
        let target_date = now - chrono::Duration::days((i * 30) as i64);
        let target_month = target_date.format("%Y-%m").to_string();

        let stats = CConsumerStatistics::find()
            .filter(c_consumer_statistics::Column::ConsumerId.eq(consumer_id))
            .filter(c_consumer_statistics::Column::StatisticsMonth.eq(&target_month))
            .one(db)
            .await
            .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

        let consume = stats.map(|s| s.month_consume).unwrap_or(Decimal::ZERO);
        result.push((target_month, consume));
    }

    result.reverse();
    Ok(result)
}

pub async fn get_year_statistics(consumer_id: i64, year: String) -> Result<ConsumerStatisticsModel> {
    let db = DB_WRITE().await;

    let stats = CConsumerStatistics::find()
        .filter(c_consumer_statistics::Column::ConsumerId.eq(consumer_id))
        .filter(c_consumer_statistics::Column::StatisticsMonth.like(format!("{}-%", year)))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let (total_consume, order_count, refund_count, refund_amount, last_order_at) = stats
        .iter()
        .fold(
            (Decimal::ZERO, 0i32, 0i32, Decimal::ZERO, None),
            |(tc, oc, rc, ra, lo), s| {
                (
                    tc + s.month_consume,
                    oc + s.order_count,
                    rc + s.refund_count,
                    ra + s.refund_amount,
                    s.last_order_at.or(lo),
                )
            },
        );

    Ok(ConsumerStatisticsModel {
        consumer_id,
        total_consume: total_consume.to_string(),
        month_consume: "0".to_string(),
        year_consume: total_consume.to_string(),
        order_count,
        refund_count,
        refund_amount: refund_amount.to_string(),
        last_order_at,
    })
}

pub async fn reset_month_statistics() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();
    let current_month = now.format("%Y-%m").to_string();

    let existing = CConsumerStatistics::find()
        .filter(c_consumer_statistics::Column::StatisticsMonth.eq(&current_month))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(existing)
}
