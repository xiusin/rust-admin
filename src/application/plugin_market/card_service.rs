use crate::domain::entity::p_card::*;
use crate::domain::entity::p_card::Entity as CardEntity;
use crate::domain::entity::p_card_batch::*;
use crate::domain::entity::p_card_batch::Entity as BatchEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::domain::entity::p_plan::Entity as PlanEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use chrono::{Duration, Utc};
use uuid::Uuid;
use md5;
use rand::Rng;
use serde::{Deserialize, Serialize};

const CARD_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";

pub async fn generate_batch(plugin_id: i64, plan_id: i64, count: i32, price: f64, expire_days: i32, creator_id: i64) -> Result<i64, Error> {
    let db = DB().await;

    let plugin = PluginEntity::find_by_id(plugin_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    let plan = PlanEntity::find_by_id(plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;

    let batch_no = generate_batch_no();

    let max_batch_id: Option<i64> = BatchEntity::find()
        .select_only()
        .column_as(p_card_batch::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_batch_id = max_batch_id.unwrap_or(0) + 1;

    let expire_time = Utc::now().naive_utc() + Duration::days(expire_days as i64);

    let batch = p_card_batch::ActiveModel {
        id: Set(new_batch_id),
        batch_no: Set(batch_no),
        plugin_id: Set(plugin_id),
        plan_id: Set(plan_id),
        total_count: Set(count),
        used_count: Set(0),
        price: Set(rust_decimal::Decimal::from_str_exact(&price.to_string()).unwrap_or_default()),
        expire_time: Set(expire_time),
        creator_id: Set(creator_id),
        status: Set(1),
        created_at: Set(Some(Utc::now().naive_utc())),
    };

    batch.insert(db).await?;

    for _ in 0..count {
        let (card_no, card_pwd) = generate_card_pair();
        let hashed_pwd = hash_password(&card_pwd);

        let max_card_id: Option<i64> = CardEntity::find()
            .select_only()
            .column_as(p_card::Column::Id.max(), "max_id")
            .into_tuple()
            .one(db)
            .await?;

        let new_card_id = max_card_id.unwrap_or(0) + 1;

        let card = p_card::ActiveModel {
            id: Set(new_card_id),
            card_no: Set(card_no),
            card_pwd: Set(hashed_pwd),
            plugin_id: Set(plugin_id),
            plan_id: Set(plan_id),
            batch_id: Set(new_batch_id),
            face_value: Set(rust_decimal::Decimal::from_str_exact(&price.to_string()).unwrap_or_default()),
            status: Set(0),
            used_user_id: Set(None),
            used_order_id: Set(None),
            used_time: Set(None),
            expire_time: Set(expire_time),
            created_at: Set(Some(Utc::now().naive_utc())),
        };

        card.insert(db).await?;
    }

    Ok(new_batch_id)
}

fn generate_batch_no() -> String {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let random: u32 = rand::random();
    format!("BATCH{}{:04}", timestamp, random % 10000)
}

fn generate_card_pair() -> (String, String) {
    let card_no = generate_card_no();
    let card_pwd = generate_card_no();
    (card_no, card_pwd)
}

fn generate_card_no() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..16 {
        let idx = rng.random_range(0..CARD_CHARSET.len());
        s.push(CARD_CHARSET[idx] as char);
        if s.len() == 4 || s.len() == 9 || s.len() == 13 {
            s.push('-');
        }
    }
    s.trim_matches('-').to_string()
}

fn hash_password(password: &str) -> String {
    format!("{:x}", md5::compute(password.as_bytes()))
}

pub async fn list_batches(plugin_id: Option<i64>, page_num: u32, page_size: u32) -> Result<(Vec<BatchItem>, i64), Error> {
    let db = DB().await;
    let offset = (page_num - 1) * page_size;

    let mut query = BatchEntity::find();

    if let Some(pid) = plugin_id {
        query = query.filter(p_card_batch::Column::PluginId.eq(pid));
    }

    let total = query.clone().count(db).await?;

    let batches = query
        .order_by_desc(p_card_batch::Column::CreatedAt)
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let items: Vec<BatchItem> = batches
        .into_iter()
        .map(|b| BatchItem {
            id: b.id,
            batch_no: b.batch_no,
            plugin_id: b.plugin_id,
            plan_id: b.plan_id,
            total_count: b.total_count,
            used_count: b.used_count,
            remaining_count: b.total_count - b.used_count,
            price: b.price.to_string().parse().unwrap_or(0.0),
            expire_time: b.expire_time,
            status: b.status,
            created_at: b.created_at,
        })
        .collect();

    Ok((items, total))
}

#[derive(Debug, Serialize, Clone)]
pub struct BatchItem {
    pub id: i64,
    pub batch_no: String,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub total_count: i32,
    pub used_count: i32,
    pub remaining_count: i32,
    pub price: f64,
    pub expire_time: chrono::NaiveDateTime,
    pub status: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub async fn redeem(user_id: i64, card_no: String, card_pwd: String) -> Result<RedeemResult, Error> {
    let db = DB().await;

    let card = CardEntity::find()
        .filter(p_card::Column::CardNo.eq(card_no.clone()))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("卡密不存在"))?;

    if card.status == 1 {
        return Ok(RedeemResult {
            success: false,
            order_id: None,
            license_id: None,
            plugin_name: String::new(),
            plan_name: String::new(),
            expire_time: None,
            message: "卡密已使用".to_string(),
        });
    }

    if card.status == 2 || card.expire_time < Utc::now().naive_utc() {
        return Ok(RedeemResult {
            success: false,
            order_id: None,
            license_id: None,
            plugin_name: String::new(),
            plan_name: String::new(),
            expire_time: None,
            message: "卡密已过期".to_string(),
        });
    }

    if card.status == 3 {
        return Ok(RedeemResult {
            success: false,
            order_id: None,
            license_id: None,
            plugin_name: String::new(),
            plan_name: String::new(),
            expire_time: None,
            message: "卡密已被冻结".to_string(),
        });
    }

    let hashed_pwd = hash_password(&card_pwd);
    if card.card_pwd != hashed_pwd {
        return Ok(RedeemResult {
            success: false,
            order_id: None,
            license_id: None,
            plugin_name: String::new(),
            plan_name: String::new(),
            expire_time: None,
            message: "卡密密码错误".to_string(),
        });
    }

    let plugin = PluginEntity::find_by_id(card.plugin_id).one(db).await?;
    let plan = PlanEntity::find_by_id(card.plan_id).one(db).await?;

    let mut active_model: p_card::ActiveModel = card.clone().into();
    active_model.status = Set(1);
    active_model.used_user_id = Set(Some(user_id));
    active_model.used_time = Set(Some(Utc::now().naive_utc()));
    active_model.update(db).await?;

    BatchEntity::update_many()
        .col_expr(p_card_batch::Column::UsedCount, Value::Int(Some(card.batch_id)))
        .filter(p_card_batch::Column::Id.eq(card.batch_id))
        .exec(db)
        .await?;

    let (order_id, license_id) = (None, None);

    Ok(RedeemResult {
        success: true,
        order_id,
        license_id,
        plugin_name: plugin.map(|p| p.name).unwrap_or_default(),
        plan_name: plan.map(|p| p.name).unwrap_or_default(),
        expire_time: Some(card.expire_time),
        message: "兑换成功".to_string(),
    })
}

#[derive(Debug, Serialize)]
pub struct RedeemResult {
    pub success: bool,
    pub order_id: Option<i64>,
    pub license_id: Option<i64>,
    pub plugin_name: String,
    pub plan_name: String,
    pub expire_time: Option<chrono::NaiveDateTime>,
    pub message: String,
}

pub async fn export_batch(batch_id: i64) -> Result<Vec<CardExportItem>, Error> {
    let db = DB().await;

    let cards = CardEntity::find()
        .filter(p_card::Column::BatchId.eq(batch_id))
        .filter(p_card::Column::Status.eq(0))
        .all(db)
        .await?;

    let items: Vec<CardExportItem> = cards
        .into_iter()
        .map(|c| CardExportItem {
            card_no: c.card_no,
        })
        .collect();

    Ok(items)
}

#[derive(Debug, Serialize)]
pub struct CardExportItem {
    pub card_no: String,
}

pub async fn freeze(card_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let card = CardEntity::find_by_id(card_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("卡密不存在"))?;

    let mut active_model: p_card::ActiveModel = card.into();
    active_model.status = Set(3);
    active_model.update(db).await?;

    Ok(())
}

pub async fn unfreeze(card_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let card = CardEntity::find_by_id(card_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("卡密不存在"))?;

    if card.status != 3 {
        return Err(Error::bad_request("卡密未被冻结"));
    }

    let mut active_model: p_card::ActiveModel = card.into();
    active_model.status = Set(0);
    active_model.update(db).await?;

    Ok(())
}