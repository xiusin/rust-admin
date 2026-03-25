use crate::domain::model::m_plugin::*;
use crate::domain::entity::p_plugin::*;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::domain::entity::p_plugin_category::Entity as CategoryEntity;
use crate::domain::entity::p_developer::Entity as DeveloperEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub async fn market_list(params: PluginSearchParams) -> Result<(Vec<PluginListItem>, i64), Error> {
    let db = DB().await;

    let mut query = PluginEntity::find()
        .inner_join(CategoryEntity)
        .inner_join(DeveloperEntity)
        .filter(p_plugin::Column::Status.eq(1))
        .filter(p_plugin::Column::DeletedAt.is_null());

    if let Some(keyword) = &params.keyword {
        query = query.filter(
            Column::Name.like(format!("%{}%", keyword))
                .or(Column::Summary.like(format!("%{}%", keyword)))
        );
    }

    if let Some(category_id) = params.category_id {
        query = query.filter(p_plugin::Column::CategoryId.eq(category_id));
    }

    if let Some(price_type) = params.price_type {
        query = query.filter(p_plugin::Column::PriceType.eq(price_type));
    }

    if let Some(verify_level) = params.verify_level {
        query = query.filter(p_plugin::Column::VerifyLevel.eq(verify_level));
    }

    if let Some(is_official) = params.is_official {
        query = query.filter(p_plugin::Column::IsOfficial.eq(is_official));
    }

    let sort = params.sort.unwrap_or_else(|| "created_at".to_string());
    let order = if sort.starts_with('-') {
        let s = sort.trim_start_matches('-');
        match s {
            "rating" => Order::Desc(p_plugin::Column::Rating),
            "download_count" => Order::Desc(p_plugin::Column::DownloadCount),
            "created_at" => Order::Desc(p_plugin::Column::CreatedAt),
            "name" => Order::Asc(p_plugin::Column::Name),
            _ => Order::Desc(p_plugin::Column::CreatedAt),
        }
    } else {
        match sort.as_str() {
            "rating" => Order::Asc(p_plugin::Column::Rating),
            "download_count" => Order::Asc(p_plugin::Column::DownloadCount),
            "created_at" => Order::Asc(p_plugin::Column::CreatedAt),
            "name" => Order::Asc(p_plugin::Column::Name),
            _ => Order::Desc(p_plugin::Column::CreatedAt),
        }
    };
    query = query.order_by(order);

    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let offset = (page_num - 1) * page_size;

    let total = query.count(db).await?;
    let plugins = query
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let items: Vec<PluginListItem> = plugins
        .into_iter()
        .map(|p| PluginListItem {
            id: p.id,
            code: p.code.clone(),
            name: p.name.clone(),
            category_id: p.category_id,
            category_name: None,
            developer_id: p.developer_id,
            developer_name: None,
            summary: p.summary.clone(),
            cover_image: p.cover_image.clone(),
            version: p.version.clone(),
            price_type: p.price_type,
            price_type_name: get_price_type_name(p.price_type),
            verify_level: p.verify_level,
            verify_level_name: get_verify_level_name(p.verify_level),
            rating: p.rating.to_string().parse().unwrap_or(5.0),
            download_count: p.download_count,
            status: p.status,
            status_name: get_status_name(p.status),
            tags: None,
            is_official: p.is_official,
            created_at: p.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn market_detail(id: i64) -> Result<Option<PluginDetail>, Error> {
    let db = DB().await;

    let plugin = PluginEntity::find_by_id(id)
        .one(db)
        .await?;

    if let Some(p) = plugin {
        let detail = PluginDetail {
            id: p.id,
            code: p.code.clone(),
            name: p.name.clone(),
            category_id: p.category_id,
            category_name: None,
            developer_id: p.developer_id,
            developer_name: None,
            developer_logo: None,
            summary: p.summary.clone(),
            description: p.description.clone(),
            cover_image: p.cover_image.clone(),
            screenshots: None,
            version: p.version.clone(),
            price_type: p.price_type,
            price_type_name: get_price_type_name(p.price_type),
            verify_level: p.verify_level,
            verify_level_name: get_verify_level_name(p.verify_level),
            rating: p.rating.to_string().parse().unwrap_or(5.0),
            download_count: p.download_count,
            status: p.status,
            tags: None,
            is_official: p.is_official,
            plans: vec![],
            created_at: p.created_at,
            updated_at: p.updated_at,
        };
        Ok(Some(detail))
    } else {
        Ok(None)
    }
}

pub async fn search(keyword: String, page_num: u32, page_size: u32) -> Result<(Vec<PluginListItem>, i64), Error> {
    market_list(PluginSearchParams {
        keyword: Some(keyword),
        category_id: None,
        price_type: None,
        verify_level: None,
        is_official: None,
        sort: None,
        page_num: Some(page_num),
        page_size: Some(page_size),
    }).await
}

pub async fn recommend(limit: i32) -> Result<Vec<PluginListItem>, Error> {
    let db = db();

    let plugins = PluginEntity::find()
        .filter(p_plugin::Column::Status.eq(1))
        .filter(p_plugin::Column::DeletedAt.is_null())
        .order_by_desc(p_plugin::Column::Rating)
        .limit(limit as u64)
        .all(db)
        .await?;

    let items: Vec<PluginListItem> = plugins
        .into_iter()
        .map(|p| PluginListItem {
            id: p.id,
            code: p.code.clone(),
            name: p.name.clone(),
            category_id: p.category_id,
            category_name: None,
            developer_id: p.developer_id,
            developer_name: None,
            summary: p.summary.clone(),
            cover_image: p.cover_image.clone(),
            version: p.version.clone(),
            price_type: p.price_type,
            price_type_name: get_price_type_name(p.price_type),
            verify_level: p.verify_level,
            verify_level_name: get_verify_level_name(p.verify_level),
            rating: p.rating.to_string().parse().unwrap_or(5.0),
            download_count: p.download_count,
            status: p.status,
            status_name: get_status_name(p.status),
            tags: None,
            is_official: p.is_official,
            created_at: p.created_at,
        })
        .collect();

    Ok(items)
}

pub async fn hot(limit: i32) -> Result<Vec<PluginListItem>, Error> {
    let db = db();

    let plugins = PluginEntity::find()
        .filter(p_plugin::Column::Status.eq(1))
        .filter(p_plugin::Column::DeletedAt.is_null())
        .order_by_desc(p_plugin::Column::DownloadCount)
        .limit(limit as u64)
        .all(db)
        .await?;

    let items: Vec<PluginListItem> = plugins
        .into_iter()
        .map(|p| PluginListItem {
            id: p.id,
            code: p.code.clone(),
            name: p.name.clone(),
            category_id: p.category_id,
            category_name: None,
            developer_id: p.developer_id,
            developer_name: None,
            summary: p.summary.clone(),
            cover_image: p.cover_image.clone(),
            version: p.version.clone(),
            price_type: p.price_type,
            price_type_name: get_price_type_name(p.price_type),
            verify_level: p.verify_level,
            verify_level_name: get_verify_level_name(p.verify_level),
            rating: p.rating.to_string().parse().unwrap_or(5.0),
            download_count: p.download_count,
            status: p.status,
            status_name: get_status_name(p.status),
            tags: None,
            is_official: p.is_official,
            created_at: p.created_at,
        })
        .collect();

    Ok(items)
}

pub async fn developer_list(developer_id: i64, page_num: u32, page_size: u32) -> Result<(Vec<PluginListItem>, i64), Error> {
    market_list(PluginSearchParams {
        keyword: None,
        category_id: None,
        price_type: None,
        verify_level: None,
        is_official: None,
        sort: None,
        page_num: Some(page_num),
        page_size: Some(page_size),
    }).await
}

pub async fn create(user_id: i64, params: CreatePluginParams) -> Result<i64, Error> {
    let db = db();

    let max_id: Option<i64> = PluginEntity::find()
        .select_only()
        .column_as(p_plugin::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let developer = DeveloperEntity::find()
        .filter(p_developer::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::bad_request("用户不是开发者"))?;

    let plugin = p_plugin::ActiveModel {
        id: Set(new_id),
        code: Set(params.code),
        name: Set(params.name),
        category_id: Set(params.category_id),
        developer_id: Set(developer.id),
        summary: Set(params.summary),
        description: Set(params.description),
        cover_image: Set(params.cover_image),
        screenshots: Set(params.screenshots.map(|s| serde_json::json!(s))),
        version: Set(params.version),
        price_type: Set(params.price_type),
        verify_level: Set(params.verify_level),
        download_count: Set(0),
        rating: Set(rust_decimal::Decimal::from_str_exact("5.00").unwrap_or_default()),
        status: Set(0),
        sort: Set(0),
        tags: Set(params.tags.map(|t| serde_json::json!(t))),
        is_official: Set(0),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
        updated_at: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    plugin.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct CreatePluginParams {
    pub code: String,
    pub name: String,
    pub category_id: i64,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub version: String,
    pub price_type: i32,
    pub verify_level: i32,
    pub tags: Option<Vec<String>>,
}

pub async fn update(id: i64, params: UpdatePluginParams) -> Result<(), Error> {
    let db = db();

    let plugin = PluginEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    let mut active_model: p_plugin::ActiveModel = plugin.into();

    if let Some(name) = params.name {
        active_model.name = Set(name);
    }
    if let Some(category_id) = params.category_id {
        active_model.category_id = Set(category_id);
    }
    if let Some(summary) = params.summary {
        active_model.summary = Set(Some(summary));
    }
    if let Some(description) = params.description {
        active_model.description = Set(Some(description));
    }
    if let Some(cover_image) = params.cover_image {
        active_model.cover_image = Set(Some(cover_image));
    }
    if let Some(sort) = params.sort {
        active_model.sort = Set(sort);
    }
    if let Some(status) = params.status {
        active_model.status = Set(status);
    }

    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UpdatePluginParams {
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn delete(id: i64) -> Result<(), Error> {
    let db = db();

    let plugin = PluginEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    let mut active_model: p_plugin::ActiveModel = plugin.into();
    active_model.status = Set(3);
    active_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn audit(id: i64, status: i32) -> Result<(), Error> {
    let db = db();

    if status != 1 && status != 2 {
        return Err(Error::bad_request("无效的审核状态"));
    }

    let plugin = PluginEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    if plugin.status != 0 {
        return Err(Error::bad_request("插件不在待审核状态"));
    }

    let mut active_model: p_plugin::ActiveModel = plugin.into();
    active_model.status = Set(status);
    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn increment_download_count(id: i64) -> Result<(), Error> {
    let db = db();

    let plugin = PluginEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("插件不存在"))?;

    let mut active_model: p_plugin::ActiveModel = plugin.into();
    active_model.download_count = Set(plugin.download_count + 1);
    active_model.update(db).await?;

    Ok(())
}