use crate::common::error::Error;
use crate::domain::model::m_after_sale::*;
use crate::domain::entity::prelude::{CAfterSale, CAfterSaleLogistics};
use crate::domain::entity::{c_after_sale, c_after_sale_logistics};
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

pub async fn submit_logistics(params: SubmitLogisticsParams) -> Result<AfterSaleLogisticsModel> {
    let db = DB_WRITE().await;

    let after_sale = CAfterSale::find_by_id(params.after_sale_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("售后单不存在"))?;

    if after_sale.status != "agreed" {
        return Err(Error::bad_request("售后单状态不正确，无法提交物流"));
    }

    if after_sale.r#type != "\"return_refund\"" && after_sale.r#type != "return_refund" {
        return Err(Error::bad_request("该售后类型无需退货"));
    }

    let existing = CAfterSaleLogistics::find()
        .filter(c_after_sale_logistics::Column::AfterSaleId.eq(params.after_sale_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let now = Local::now().naive_local();

    let logistics_model = if let Some(existing_log) = existing {
        let mut active_model: c_after_sale_logistics::ActiveModel = existing_log.into();
        active_model.logistics_company = Set(Some(params.logistics_company.clone()));
        active_model.tracking_no = Set(Some(params.tracking_no.clone()));
        active_model.sender_name = Set(params.sender_name.clone());
        active_model.sender_phone = Set(params.sender_phone.clone());
        active_model.sender_address = Set(params.sender_address.clone());
        active_model.status = Set("shipped".to_string());
        active_model.shipped_at = Set(Some(now));
        active_model.updated_at = Set(Some(now));

        CAfterSaleLogistics::update(active_model)
            .exec(db)
            .await
            .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?
    } else {
        let new_logistics = c_after_sale_logistics::ActiveModel {
            id: Set(generate_id()),
            after_sale_id: Set(params.after_sale_id),
            logistics_type: Set("return".to_string()),
            logistics_company: Set(Some(params.logistics_company.clone())),
            tracking_no: Set(Some(params.tracking_no.clone())),
            sender_name: Set(params.sender_name.clone()),
            sender_phone: Set(params.sender_phone.clone()),
            sender_address: Set(params.sender_address.clone()),
            receiver_name: Set(None),
            receiver_phone: Set(None),
            receiver_address: Set(None),
            status: Set("shipped".to_string()),
            shipped_at: Set(Some(now)),
            received_at: Set(None),
            tracking_info: Set(None),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        };

        CAfterSaleLogistics::insert(new_logistics)
            .exec_with_returning(db)
            .await
            .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?
    };

    let mut as_model: c_after_sale::ActiveModel = after_sale.into();
    as_model.status = Set("processing".to_string());
    as_model.updated_at = Set(Some(now));

    let _ = CAfterSale::update(as_model).exec(db).await;

    Ok(AfterSaleLogisticsModel {
        id: logistics_model.id,
        after_sale_id: logistics_model.after_sale_id,
        logistics_type: logistics_model.logistics_type,
        logistics_company: logistics_model.logistics_company,
        tracking_no: logistics_model.tracking_no,
        sender_name: logistics_model.sender_name,
        sender_phone: logistics_model.sender_phone,
        sender_address: logistics_model.sender_address,
        receiver_name: logistics_model.receiver_name,
        receiver_phone: logistics_model.receiver_phone,
        receiver_address: logistics_model.receiver_address,
        status: logistics_model.status,
        shipped_at: logistics_model.shipped_at,
        received_at: logistics_model.received_at,
        tracking_info: logistics_model.tracking_info,
    })
}

pub async fn confirm_receive(params: ConfirmReceiveParams) -> Result<AfterSaleLogisticsModel> {
    let db = DB_WRITE().await;

    let logistics = CAfterSaleLogistics::find()
        .filter(c_after_sale_logistics::Column::AfterSaleId.eq(params.after_sale_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("物流信息不存在"))?;

    if logistics.status != "shipped" {
        return Err(Error::bad_request("物流状态不正确"));
    }

    let now = Local::now().naive_local();

    let mut active_model: c_after_sale_logistics::ActiveModel = logistics.into();
    active_model.status = Set("received".to_string());
    active_model.received_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    let updated = CAfterSaleLogistics::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(AfterSaleLogisticsModel {
        id: updated.id,
        after_sale_id: updated.after_sale_id,
        logistics_type: updated.logistics_type,
        logistics_company: updated.logistics_company,
        tracking_no: updated.tracking_no,
        sender_name: updated.sender_name,
        sender_phone: updated.sender_phone,
        sender_address: updated.sender_address,
        receiver_name: updated.receiver_name,
        receiver_phone: updated.receiver_phone,
        receiver_address: updated.receiver_address,
        status: updated.status,
        shipped_at: updated.shipped_at,
        received_at: updated.received_at,
        tracking_info: updated.tracking_info,
    })
}

pub async fn get_logistics(after_sale_id: i64) -> Result<Option<AfterSaleLogisticsModel>> {
    let db = DB_WRITE().await;

    let logistics = CAfterSaleLogistics::find()
        .filter(c_after_sale_logistics::Column::AfterSaleId.eq(after_sale_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(logistics.map(|l| AfterSaleLogisticsModel {
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
    }))
}

pub async fn update_tracking_info(after_sale_id: i64, tracking_info: serde_json::Value) -> Result<()> {
    let db = DB_WRITE().await;

    let logistics = CAfterSaleLogistics::find()
        .filter(c_after_sale_logistics::Column::AfterSaleId.eq(after_sale_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("物流信息不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_after_sale_logistics::ActiveModel = logistics.into();
    active_model.tracking_info = Set(Some(tracking_info));
    active_model.updated_at = Set(Some(now));

    CAfterSaleLogistics::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}
