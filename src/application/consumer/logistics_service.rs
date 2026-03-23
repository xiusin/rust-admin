use crate::common::error::Error;
use crate::domain::model::m_logistics::*;
use crate::domain::entity::prelude::CLogisticsTracking;
use crate::domain::entity::c_logistics_tracking;
use crate::model::prelude::*;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn query(params: QueryLogisticsParams) -> Result<LogisticsDetailResp> {
    let db = DB_WRITE().await;

    let tracking = CLogisticsTracking::find()
        .filter(c_logistics_tracking::Column::TrackingNo.eq(&params.tracking_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    match tracking {
        Some(t) => {
            let traces: Vec<LogisticsTraceResp> = serde_json::from_value(t.tracking_info.clone())
                .unwrap_or_default();

            Ok(LogisticsDetailResp {
                tracking_no: t.tracking_no,
                courier_company: t.courier_company.unwrap_or_default(),
                status: t.status,
                traces,
            })
        }
        None => {
            let id = generate_id();
            let now = chrono::Local::now().naive_local();
            let tracking = c_logistics_tracking::ActiveModel {
                id: Set(id),
                tracking_no: Set(params.tracking_no.clone()),
                courier_company: Set(Some(params.courier_company.clone())),
                status: Set("pending".to_string()),
                tracking_info: Set(serde_json::json!([])),
                last_updated_at: Set(Some(now)),
                created_at: Set(Some(now)),
                ..Default::default()
            };

            CLogisticsTracking::insert(tracking)
                .exec(db)
                .await
                .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

            Ok(LogisticsDetailResp {
                tracking_no: params.tracking_no,
                courier_company: params.courier_company,
                status: "pending".to_string(),
                traces: vec![],
            })
        }
    }
}

pub async fn subscribe(params: SubscribeLogisticsParams) -> Result<LogisticsTrackingModel> {
    let db = DB_WRITE().await;
    let now = chrono::Local::now().naive_local();

    let existing = CLogisticsTracking::find()
        .filter(c_logistics_tracking::Column::TrackingNo.eq(&params.tracking_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if let Some(t) = existing {
        let traces: Vec<LogisticsTraceResp> = serde_json::from_value(t.tracking_info.clone())
            .unwrap_or_default();
        return Ok(LogisticsTrackingModel {
            id: t.id,
            tracking_no: t.tracking_no,
            courier_company: t.courier_company,
            status: t.status,
            traces,
            created_at: t.created_at,
        });
    }

    let id = generate_id();
    let tracking = c_logistics_tracking::ActiveModel {
        id: Set(id),
        tracking_no: Set(params.tracking_no.clone()),
        courier_company: Set(Some(params.courier_company)),
        status: Set("subscribed".to_string()),
        tracking_info: Set(serde_json::json!([])),
        last_updated_at: Set(Some(now)),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    CLogisticsTracking::insert(tracking)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(LogisticsTrackingModel {
        id,
        tracking_no: params.tracking_no,
        courier_company: Some("".to_string()),
        status: "subscribed".to_string(),
        traces: vec![],
        created_at: Some(now),
    })
}

pub async fn get_history(
    params: LogisticsHistoryParams,
) -> Result<(Vec<LogisticsTraceResp>, u64)> {
    let db = DB_WRITE().await;

    let tracking = CLogisticsTracking::find()
        .filter(c_logistics_tracking::Column::TrackingNo.eq(&params.tracking_no))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let traces: Vec<LogisticsTraceResp> = tracking
        .and_then(|t| serde_json::from_value(t.tracking_info.clone()).ok())
        .unwrap_or_default();

    Ok((traces.clone(), traces.len() as u64))
}