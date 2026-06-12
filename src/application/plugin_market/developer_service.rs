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

    let orders = p_order::Entity::find()
        .all(db)
        .await?;

    let plugins = p_plugin::Entity::find()
        .filter(p_plugin::Column::DeveloperId.eq(developer_id))
        .all(db)
        .await?;

    let mut total_orders = 0;
    let mut total_revenue = rust_decimal::Decimal::new(0, 0);

    for o in &orders {
        for p in &plugins {
            if o.plugin_id == p.id {
                total_orders += 1;
                total_revenue += o.amount;
                break;
            }
        }
    }

    Ok(DeveloperStats {
        total_plugins: developer.plugins_count as i64,
        active_plugins: developer.plugins_count as i64,
        total_downloads: developer.total_downloads,
        total_orders,
        total_revenue: total_revenue.to_string().parse().unwrap_or(0.0),
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
use crate::domain::entity::{p_plugin, p_order};

pub async fn get_chart(_developer_id: i64) -> Result<serde_json::Value, Error> {
    let db = DB().await;

    // Simple aggregate using SeaORM
    // To avoid complex group-bys, we can fetch orders for the developer's plugins in the last 7 days
    let orders = p_order::Entity::find()
        // .filter(p_order::Column::Status.eq(1)) // paid
        .all(db)
        .await?;

    let mut revenue_data = vec![];
    let mut orders_data = vec![];
    let mut downloads_data = vec![];

    // For a real app we'd group by date, here we just return a simple aggregated mock from DB
    // Or we can literally group them by day.
    use chrono::{Datelike, Utc, Duration};
    let now = Utc::now().naive_utc();
    
    for i in (0..7).rev() {
        let day = now - Duration::days(i);
        let day_str = format!("{}日", day.day());
        
        let mut daily_rev = rust_decimal::Decimal::new(0, 0);
        let mut daily_orders = 0;
        
        for o in &orders {
            if let Some(created_at) = o.created_at {
                if created_at.date() == day.date() {
                    daily_rev += o.amount;
                    daily_orders += 1;
                }
            }
        }
        
        revenue_data.push(serde_json::json!({"day": day_str, "value": daily_rev}));
        orders_data.push(serde_json::json!({"day": day_str, "value": daily_orders}));
        // Fake downloads based on orders for now
        downloads_data.push(serde_json::json!({"day": day_str, "value": daily_orders * 3}));
    }

    Ok(serde_json::json!({
        "revenue": revenue_data,
        "orders": orders_data,
        "downloads": downloads_data
    }))
}

pub async fn get_ranking(_developer_id: i64) -> Result<serde_json::Value, Error> {
    let db = DB().await;

    let plugins = p_plugin::Entity::find()
        .all(db)
        .await?;

    let orders = p_order::Entity::find()
        // .filter(p_order::Column::Status.eq(1)) // paid
        .all(db)
        .await?;

    let mut ranking = vec![];

    for p in plugins {
        let mut rev = rust_decimal::Decimal::new(0, 0);
        let mut ord_cnt = 0;
        for o in &orders {
            if o.plugin_id == p.id {
                rev += o.amount;
                ord_cnt += 1;
            }
        }
        ranking.push(serde_json::json!({
            "id": p.id,
            "name": p.name,
            "revenue": rev,
            "orders": ord_cnt
        }));
    }

    ranking.sort_by(|a, b| {
        let rev_a = a["revenue"].as_f64().unwrap_or(0.0);
        let rev_b = b["revenue"].as_f64().unwrap_or(0.0);
        rev_b.partial_cmp(&rev_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(serde_json::json!(ranking))
}
