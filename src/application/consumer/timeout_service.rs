use crate::common::error::Error;
use crate::domain::model::m_after_sale::*;
use crate::domain::entity::prelude::{CAfterSale, CAfterSaleTimeoutConfig, CAfterSaleStatusLog};
use crate::domain::entity::{c_after_sale, c_after_sale_timeout_config, c_after_sale_status_log};
use crate::model::prelude::*;
use sea_orm::*;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn get_timeout_configs() -> Result<Vec<TimeoutConfigModel>> {
    let db = DB_WRITE().await;

    let configs = CAfterSaleTimeoutConfig::find()
        .filter(c_after_sale_timeout_config::Column::IsActive.eq(true))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(configs
        .into_iter()
        .map(|c| TimeoutConfigModel {
            stage: c.stage,
            timeout_hours: c.timeout_hours,
            auto_action: c.auto_action,
        })
        .collect())
}

pub async fn process_timeout_after_sales() -> Result<u64> {
    let db = DB_WRITE().await;
    let now = Local::now().naive_local();

    let timeout_after_sales = CAfterSale::find()
        .filter(c_after_sale::Column::TimeoutAt.lt(now))
        .filter(c_after_sale::Column::IsTimeout.eq(false))
        .filter(c_after_sale::Column::Status.is_in(vec!["pending", "processing", "agreed"]))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut count = 0u64;

    for after_sale in timeout_after_sales {
        let config = get_timeout_config_for_status(db, &after_sale.status).await?;

        if let Some(cfg) = config {
            let result = match cfg.auto_action.as_str() {
                "auto_agree" => auto_agree_after_sale(db, after_sale).await,
                "auto_close" => auto_close_after_sale(db, after_sale).await,
                "auto_complete" => auto_complete_after_sale(db, after_sale).await,
                _ => Ok(()),
            };

            if result.is_ok() {
                count += 1;
            }
        }
    }

    Ok(count)
}

async fn get_timeout_config_for_status(db: &DatabaseConnection, status: &str) -> Result<Option<c_after_sale_timeout_config::Model>> {
    let stage = match status {
        "pending" => "apply_audit",
        "processing" => "return_receive",
        "agreed" => "return_ship",
        _ => return Ok(None),
    };

    let config = CAfterSaleTimeoutConfig::find()
        .filter(c_after_sale_timeout_config::Column::Stage.eq(stage))
        .filter(c_after_sale_timeout_config::Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(config)
}

async fn auto_agree_after_sale(db: &DatabaseConnection, after_sale: c_after_sale::Model) -> Result<()> {
    let now = Local::now().naive_local();
    let old_status = after_sale.status.clone();
    let after_sale_id = after_sale.id;

    let mut active_model: c_after_sale::ActiveModel = after_sale.into();
    active_model.status = Set("agreed".to_string());
    active_model.process_at = Set(Some(now));
    active_model.is_timeout = Set(true);
    active_model.processor_name = Set(Some("系统自动".to_string()));
    active_model.updated_at = Set(Some(now));

    CAfterSale::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(after_sale_id),
        old_status: Set(Some(old_status)),
        new_status: Set("agreed".to_string()),
        operator_type: Set("system".to_string()),
        operator_id: Set(None),
        operator_name: Set(Some("系统自动".to_string())),
        remark: Set(Some("审核超时自动同意".to_string())),
        created_at: Set(Some(now)),
    };

    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(())
}

async fn auto_close_after_sale(db: &DatabaseConnection, after_sale: c_after_sale::Model) -> Result<()> {
    let now = Local::now().naive_local();
    let old_status = after_sale.status.clone();
    let after_sale_id = after_sale.id;

    let mut active_model: c_after_sale::ActiveModel = after_sale.into();
    active_model.status = Set("closed".to_string());
    active_model.close_at = Set(Some(now));
    active_model.is_timeout = Set(true);
    active_model.updated_at = Set(Some(now));

    CAfterSale::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(after_sale_id),
        old_status: Set(Some(old_status)),
        new_status: Set("closed".to_string()),
        operator_type: Set("system".to_string()),
        operator_id: Set(None),
        operator_name: Set(Some("系统自动".to_string())),
        remark: Set(Some("退货超时自动关闭".to_string())),
        created_at: Set(Some(now)),
    };

    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(())
}

async fn auto_complete_after_sale(db: &DatabaseConnection, after_sale: c_after_sale::Model) -> Result<()> {
    let now = Local::now().naive_local();
    let old_status = after_sale.status.clone();
    let after_sale_id = after_sale.id;

    let mut active_model: c_after_sale::ActiveModel = after_sale.into();
    active_model.status = Set("completed".to_string());
    active_model.complete_at = Set(Some(now));
    active_model.is_timeout = Set(true);
    active_model.updated_at = Set(Some(now));

    CAfterSale::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    let status_log = c_after_sale_status_log::ActiveModel {
        id: Set(generate_id()),
        after_sale_id: Set(after_sale_id),
        old_status: Set(Some(old_status)),
        new_status: Set("completed".to_string()),
        operator_type: Set("system".to_string()),
        operator_id: Set(None),
        operator_name: Set(Some("系统自动".to_string())),
        remark: Set(Some("确认收货超时自动完成".to_string())),
        created_at: Set(Some(now)),
    };

    let _ = CAfterSaleStatusLog::insert(status_log).exec(db).await;

    Ok(())
}

pub async fn update_timeout_at(after_sale_id: i64, stage: &str) -> Result<()> {
    let db = DB_WRITE().await;

    let config = CAfterSaleTimeoutConfig::find()
        .filter(c_after_sale_timeout_config::Column::Stage.eq(stage))
        .filter(c_after_sale_timeout_config::Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if let Some(cfg) = config {
        let now = Local::now().naive_local();
        let new_timeout = now + chrono::Duration::hours(cfg.timeout_hours as i64);

        let after_sale = CAfterSale::find_by_id(after_sale_id)
            .one(db)
            .await
            .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::not_found("售后单不存在"))?;

        let mut active_model: c_after_sale::ActiveModel = after_sale.into();
        active_model.timeout_at = Set(Some(new_timeout));
        active_model.updated_at = Set(Some(now));

        CAfterSale::update(active_model)
            .exec(db)
            .await
            .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;
    }

    Ok(())
}

pub async fn get_after_sale_statistics() -> Result<AfterSaleStatistics> {
    let db = DB_WRITE().await;

    let total_count = CAfterSale::find()
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let pending_count = CAfterSale::find()
        .filter(c_after_sale::Column::Status.eq("pending"))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let processing_count = CAfterSale::find()
        .filter(c_after_sale::Column::Status.is_in(vec!["processing", "agreed"]))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let completed_count = CAfterSale::find()
        .filter(c_after_sale::Column::Status.eq("completed"))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let completed_after_sales = CAfterSale::find()
        .filter(c_after_sale::Column::Status.eq("completed"))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let total_refund_amount: rust_decimal::Decimal = completed_after_sales
        .iter()
        .map(|a| a.refund_amount)
        .sum();

    Ok(AfterSaleStatistics {
        total_count,
        pending_count,
        processing_count,
        completed_count,
        total_refund_amount: total_refund_amount.to_string(),
    })
}
