use crate::common::error::Error;
use crate::domain::model::m_plugin_category::*;
use crate::domain::entity::p_plugin_category::Entity as PluginCategoryEntity;
use crate::domain::entity::p_plugin_category::Model as PluginCategory;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct CategoryTree {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub parent_id: i64,
    pub sort: i32,
    pub plugin_count: i32,
    pub children: Vec<CategoryTree>,
}

pub async fn list() -> Result<Vec<PluginCategory>, Error> {
    let db = DB().await;
    let categories = PluginCategoryEntity::find()
        .order_by_asc(p_plugin_category::Column::Sort)
        .all(db)
        .await?;
    Ok(categories)
}

pub async fn tree() -> Result<Vec<CategoryTree>, Error> {
    let db = DB().await;
    let categories = PluginCategoryEntity::find()
        .filter(p_plugin_category::Column::Status.eq(1))
        .order_by_asc(p_plugin_category::Column::Sort)
        .all(db)
        .await?;

    let mut result: Vec<CategoryTree> = Vec::new();

    for category in categories.iter() {
        if category.parent_id == 0 {
            let children = build_children(&categories, category.id);
            result.push(CategoryTree {
                id: category.id,
                name: category.name.clone(),
                icon: category.icon.clone(),
                parent_id: category.parent_id,
                sort: category.sort,
                plugin_count: category.plugin_count,
                children,
            });
        }
    }

    Ok(result)
}

fn build_children(categories: &[PluginCategory], parent_id: i64) -> Vec<CategoryTree> {
    let mut children: Vec<CategoryTree> = Vec::new();

    for category in categories.iter() {
        if category.parent_id == parent_id {
            children.push(CategoryTree {
                id: category.id,
                name: category.name.clone(),
                icon: category.icon.clone(),
                parent_id: category.parent_id,
                sort: category.sort,
                plugin_count: category.plugin_count,
                children: build_children(categories, category.id),
            });
        }
    }

    children.sort_by(|a, b| a.sort.cmp(&b.sort));
    children
}

pub async fn detail(id: i64) -> Result<Option<PluginCategory>, Error> {
    let db = DB().await;
    let category = PluginCategoryEntity::find_by_id(id)
        .one(db)
        .await?;
    Ok(category)
}

pub async fn create(params: CreateCategoryParams) -> Result<i64, Error> {
    let db = DB().await;

    let max_id: Option<i64> = PluginCategoryEntity::find()
        .select_only()
        .column_as(p_plugin_category::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let category = p_plugin_category::ActiveModel {
        id: Set(new_id),
        name: Set(params.name),
        icon: Set(params.icon),
        parent_id: Set(params.parent_id),
        sort: Set(params.sort.unwrap_or(0)),
        plugin_count: Set(0),
        status: Set(params.status.unwrap_or(1)),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    category.insert(db).await?;
    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryParams {
    pub name: String,
    pub icon: Option<String>,
    pub parent_id: i64,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn update(id: i64, params: UpdateCategoryParams) -> Result<(), Error> {
    let db = DB().await;

    let category = PluginCategoryEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let mut active_model: p_plugin_category::ActiveModel = category.into();
    if let Some(name) = params.name {
        active_model.name = Set(name);
    }
    if let Some(icon) = params.icon {
        active_model.icon = Set(Some(icon));
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
pub struct UpdateCategoryParams {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn delete(id: i64) -> Result<(), Error> {
    let db = DB().await;

    let child_count = PluginCategoryEntity::find()
        .filter(p_plugin_category::Column::ParentId.eq(id))
        .count(db)
        .await?;

    if child_count > 0 {
        return Err(Error::bad_request("存在子分类，无法删除"));
    }

    let category = PluginCategoryEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let active_model: p_plugin_category::ActiveModel = category.into();
    active_model.delete(db).await?;

    Ok(())
}

pub async fn update_plugin_count(id: i64, delta: i32) -> Result<(), Error> {
    let db = DB().await;

    let category = PluginCategoryEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let mut active_model: p_plugin_category::ActiveModel = category.into();
    active_model.plugin_count = Set((category.plugin_count as i32 + delta) as i32);

    active_model.update(db).await?;
    Ok(())
}
