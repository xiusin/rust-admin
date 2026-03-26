use crate::domain::entity::p_developer;
use crate::domain::entity::p_developer::Entity as DeveloperEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub async fn register(user_id: i64, params: RegisterDeveloperParams) -> Result<i64, Error> {
    let db = DB().await;

    let existing = DeveloperEntity::find()
        .filter(p_developer::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("用户已是开发者"));
    }

    let max_id: Option<i64> = DeveloperEntity::find()
        .select_only()
        .column_as(p_developer::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let developer = p_developer::ActiveModel {
        id: Set(new_id),
        user_id: Set(user_id),
        name: Set(params.name),
        logo: Set(None),
        description: Set(params.description),
        contact: Set(params.contact),
        plugins_count: Set(0),
        total_downloads: Set(0),
        rating: Set(rust_decimal::Decimal::from_str_exact("5.00").unwrap_or_default()),
        status: Set(1),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
        updated_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    developer.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeveloperParams {
    pub name: String,
    pub description: Option<String>,
    pub contact: Option<String>,
}

pub async fn get_by_user_id(user_id: i64) -> Result<Option<p_developer::Model>, Error> {
    let db = DB().await;
    let developer = DeveloperEntity::find()
        .filter(p_developer::Column::UserId.eq(user_id))
        .one(db)
        .await?;
    Ok(developer)
}

pub async fn update(user_id: i64, params: UpdateDeveloperParams) -> Result<(), Error> {
    let db = DB().await;

    let developer = DeveloperEntity::find()
        .filter(p_developer::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("开发者不存在"))?;

    let mut active_model: p_developer::ActiveModel = developer.into();

    if let Some(name) = params.name {
        active_model.name = Set(name);
    }
    if let Some(logo) = params.logo {
        active_model.logo = Set(Some(logo));
    }
    if let Some(description) = params.description {
        active_model.description = Set(Some(description));
    }
    if let Some(contact) = params.contact {
        active_model.contact = Set(Some(contact));
    }

    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeveloperParams {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub contact: Option<String>,
}

pub async fn get_stats(developer_id: i64) -> Result<DeveloperStats, Error> {
    let db = DB().await;

    let developer = DeveloperEntity::find_by_id(developer_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("开发者不存在"))?;

    Ok(DeveloperStats {
        total_plugins: developer.plugins_count as i64,
        active_plugins: developer.plugins_count as i64,
        total_downloads: developer.total_downloads,
        total_orders: 0,
        total_revenue: 0.0,
        pending_orders: 0,
        rating: developer.rating.to_string().parse().unwrap_or(5.0),
    })
}

#[derive(Debug, Serialize)]
pub struct DeveloperStats {
    pub total_plugins: i64,
    pub active_plugins: i64,
    pub total_downloads: i64,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub pending_orders: i64,
    pub rating: f64,
}

pub async fn increment_plugin_count(developer_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let developer = DeveloperEntity::find_by_id(developer_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("开发者不存在"))?;

    let count = developer.plugins_count;
    let mut active_model: p_developer::ActiveModel = developer.into();
    active_model.plugins_count = Set(count + 1);
    active_model.update(db).await?;

    Ok(())
}

pub async fn increment_downloads(developer_id: i64, delta: i64) -> Result<(), Error> {
    let db = DB().await;

    let developer = DeveloperEntity::find_by_id(developer_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("开发者不存在"))?;

    let downloads = developer.total_downloads;
    let mut active_model: p_developer::ActiveModel = developer.into();
    active_model.total_downloads = Set(downloads + delta);
    active_model.update(db).await?;

    Ok(())
}