use crate::domain::entity::p_cart;
use crate::domain::entity::p_cart::Entity as CartEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::domain::entity::p_plan::Entity as PlanEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::Deserialize;
use crate::domain::model::m_cart::*;

pub async fn list(user_id: i64) -> Result<Vec<CartItem>, Error> {
    let db = DB().await;

    let cart_items = CartEntity::find()
        .filter(p_cart::Column::UserId.eq(user_id))
        .order_by_desc(p_cart::Column::CreatedAt)
        .all(db)
        .await?;

    let mut items: Vec<CartItem> = Vec::new();

    for cart in cart_items {
        let plugin = PluginEntity::find_by_id(cart.plugin_id).one(db).await?;
        let plan = PlanEntity::find_by_id(cart.plan_id).one(db).await?;

        if let (Some(p), Some(pl)) = (plugin, plan) {
            items.push(CartItem {
                id: cart.id,
                user_id: cart.user_id,
                plugin_id: cart.plugin_id,
                plugin_name: p.name.clone(),
                plugin_cover: p.cover_image.clone(),
                plugin_version: p.version.clone(),
                plan_id: cart.plan_id,
                plan_name: pl.name.clone(),
                price: pl.price.to_string().parse().unwrap_or(0.0),
                original_price: pl.original_price.to_string().parse().unwrap_or(0.0),
                created_at: cart.created_at,
            });
        }
    }

    Ok(items)
}

pub async fn add(user_id: i64, params: AddCartParams) -> Result<i64, Error> {
    let db = DB().await;

    let existing = CartEntity::find()
        .filter(p_cart::Column::UserId.eq(user_id))
        .filter(p_cart::Column::PluginId.eq(params.plugin_id))
        .filter(p_cart::Column::PlanId.eq(params.plan_id))
        .one(db)
        .await?;

    if let Some(e) = existing {
        return Ok(e.id);
    }

    let max_id: Option<i64> = CartEntity::find()
        .select_only()
        .column_as(p_cart::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let cart = p_cart::ActiveModel {
        id: Set(new_id),
        user_id: Set(user_id),
        plugin_id: Set(params.plugin_id),
        plan_id: Set(params.plan_id),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    cart.insert(db).await?;
    Ok(new_id)
}

pub async fn remove(user_id: i64, ids: Vec<i64>) -> Result<(), Error> {
    let db = DB().await;

    for id in ids {
        let cart = CartEntity::find()
            .filter(p_cart::Column::Id.eq(id))
            .filter(p_cart::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if let Some(c) = cart {
            let active_model: p_cart::ActiveModel = c.into();
            active_model.delete(db).await?;
        }
    }

    Ok(())
}

pub async fn clear(user_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let carts = CartEntity::find()
        .filter(p_cart::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    for cart in carts {
        let active_model: p_cart::ActiveModel = cart.into();
        active_model.delete(db).await?;
    }

    Ok(())
}