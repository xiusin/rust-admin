use crate::domain::entity::p_plan;
use crate::domain::entity::p_plan::Entity as PlanEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::domain::model::m_plugin::*;

pub async fn list(plugin_id: i64) -> Result<Vec<PlanItem>, Error> {
    let db = DB().await;

    let plans = PlanEntity::find()
        .filter(p_plan::Column::PluginId.eq(plugin_id))
        .filter(p_plan::Column::Status.eq(1))
        .order_by_asc(p_plan::Column::Sort)
        .all(db)
        .await?;

    let items: Vec<PlanItem> = plans
        .into_iter()
        .map(|p| PlanItem {
            id: p.id,
            plugin_id: p.plugin_id,
            name: p.name.clone(),
            description: p.description.clone(),
            period_type: p.period_type,
            period_type_name: get_period_type_name(p.period_type),
            period_days: p.period_days,
            price: p.price.to_string().parse().unwrap_or(0.0),
            original_price: p.original_price.to_string().parse().unwrap_or(0.0),
            features: None,
            max_devices: p.max_devices,
            max_users: p.max_users,
            storage_limit: p.storage_limit,
            api_calls_limit: p.api_calls_limit,
            support_level: p.support_level,
            support_level_name: get_support_level_name(p.support_level),
            sort: p.sort,
            status: p.status,
        })
        .collect();

    Ok(items)
}

pub async fn detail(id: i64) -> Result<Option<p_plan::Model>, Error> {
    let db = DB().await;
    let plan = PlanEntity::find_by_id(id).one(db).await?;
    Ok(plan)
}

pub async fn create(plugin_id: i64, params: CreatePlanParams) -> Result<i64, Error> {
    let db = DB().await;

    let max_id: Option<i64> = PlanEntity::find()
        .select_only()
        .column_as(p_plan::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let plan = p_plan::ActiveModel {
        id: Set(new_id),
        plugin_id: Set(plugin_id),
        name: Set(params.name),
        description: Set(params.description),
        period_type: Set(params.period_type),
        period_days: Set(params.period_days),
        price: Set(rust_decimal::Decimal::from_str_exact(&params.price.to_string()).unwrap_or_default()),
        original_price: Set(rust_decimal::Decimal::from_str_exact(&params.original_price.to_string()).unwrap_or_default()),
        features: Set(params.features.map(|f| serde_json::json!(f))),
        max_devices: Set(params.max_devices),
        max_users: Set(params.max_users),
        storage_limit: Set(params.storage_limit),
        api_calls_limit: Set(params.api_calls_limit),
        support_level: Set(params.support_level),
        sort: Set(params.sort.unwrap_or(0)),
        status: Set(1),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    plan.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanParams {
    pub name: String,
    pub description: Option<String>,
    pub period_type: i32,
    pub period_days: i32,
    pub price: f64,
    pub original_price: f64,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: i32,
    pub max_users: i32,
    pub storage_limit: i64,
    pub api_calls_limit: i64,
    pub support_level: i32,
    pub sort: Option<i32>,
}

pub async fn update(id: i64, params: UpdatePlanParams) -> Result<(), Error> {
    let db = DB().await;

    let plan = PlanEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;

    let mut active_model: p_plan::ActiveModel = plan.into();

    if let Some(name) = params.name {
        active_model.name = Set(name);
    }
    if let Some(description) = params.description {
        active_model.description = Set(Some(description));
    }
    if let Some(period_type) = params.period_type {
        active_model.period_type = Set(period_type);
    }
    if let Some(period_days) = params.period_days {
        active_model.period_days = Set(period_days);
    }
    if let Some(price) = params.price {
        active_model.price = Set(rust_decimal::Decimal::from_str_exact(&price.to_string()).unwrap_or_default());
    }
    if let Some(max_devices) = params.max_devices {
        active_model.max_devices = Set(max_devices);
    }
    if let Some(sort) = params.sort {
        active_model.sort = Set(sort);
    }
    if let Some(status) = params.status {
        active_model.status = Set(status);
    }

    active_model.update(db).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub period_type: Option<i32>,
    pub period_days: Option<i32>,
    pub price: Option<f64>,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: Option<i32>,
    pub max_users: Option<i32>,
    pub storage_limit: Option<i64>,
    pub api_calls_limit: Option<i64>,
    pub support_level: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn delete(id: i64) -> Result<(), Error> {
    let db = DB().await;

    let plan = PlanEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;

    let mut active_model: p_plan::ActiveModel = plan.into();
    active_model.status = Set(0);
    active_model.update(db).await?;

    Ok(())
}

pub async fn calculate_price(plan_id: i64, period_type: i32) -> Result<f64, Error> {
    let db = DB().await;

    let plan = PlanEntity::find_by_id(plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;

    let base_price = plan.price.to_string().parse::<f64>().unwrap_or(0.0);

    let multiplier = match period_type {
        0 => 1.0,
        1 => 2.8,
        2 => 10.0,
        3 => 25.0,
        _ => 1.0,
    };

    Ok(base_price * multiplier)
}