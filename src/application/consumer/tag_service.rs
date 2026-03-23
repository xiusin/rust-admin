use crate::common::error::Error;
use crate::domain::model::m_user_extension::*;
use crate::domain::entity::prelude::{CUserTag, CUserTagRelation, CConsumer};
use crate::domain::entity::{c_user_tag, c_user_tag_relation};
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

pub async fn create_tag(params: CreateTagParams) -> Result<UserTagModel> {
    let db = DB_WRITE().await;

    let existing = CUserTag::find()
        .filter(c_user_tag::Column::Name.eq(&params.name))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("标签名称已存在"));
    }

    let now = Local::now().naive_local();
    let tag = c_user_tag::ActiveModel {
        id: Set(generate_id()),
        name: Set(params.name.clone()),
        tag_type: Set(params.tag_type.clone()),
        category: Set(params.category.clone()),
        color: Set(params.color.clone()),
        icon: Set(params.icon.clone()),
        description: Set(params.description.clone()),
        is_active: Set(true),
        sort_order: Set(0),
        created_by: Set(params.created_by),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    let inserted = CUserTag::insert(tag)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(UserTagModel {
        id: inserted.last_insert_id,
        name: params.name,
        tag_type: params.tag_type,
        category: params.category,
        color: params.color,
        icon: params.icon,
        description: params.description,
        is_active: true,
        created_at: Some(now),
    })
}

pub async fn update_tag(
    id: i64,
    name: Option<String>,
    category: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    description: Option<String>,
) -> Result<UserTagModel> {
    let db = DB_WRITE().await;

    let tag = CUserTag::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("标签不存在"))?;

    let now = Local::now().naive_local();
    let mut active_model: c_user_tag::ActiveModel = tag.into();

    if let Some(n) = name {
        active_model.name = Set(n);
    }
    if let Some(c) = category {
        active_model.category = Set(Some(c));
    }
    if let Some(c) = color {
        active_model.color = Set(Some(c));
    }
    if let Some(i) = icon {
        active_model.icon = Set(Some(i));
    }
    if let Some(d) = description {
        active_model.description = Set(Some(d));
    }
    active_model.updated_at = Set(Some(now));

    let updated = CUserTag::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(UserTagModel {
        id: updated.id,
        name: updated.name,
        tag_type: updated.tag_type,
        category: updated.category,
        color: updated.color,
        icon: updated.icon,
        description: updated.description,
        is_active: updated.is_active,
        created_at: updated.created_at,
    })
}

pub async fn delete_tag(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    CUserTagRelation::delete_many()
        .filter(c_user_tag_relation::Column::TagId.eq(id))
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete relations error: {}", e)))?;

    CUserTag::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    Ok(())
}

pub async fn list_tags(params: UserTagListParams) -> Result<(Vec<UserTagModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();
    conditions = conditions.add(c_user_tag::Column::IsActive.eq(true));

    if let Some(tag_type) = &params.tag_type {
        conditions = conditions.add(c_user_tag::Column::TagType.eq(tag_type.clone()));
    }
    if let Some(category) = &params.category {
        conditions = conditions.add(c_user_tag::Column::Category.eq(category.clone()));
    }
    if let Some(name) = &params.name {
        conditions = conditions.add(c_user_tag::Column::Name.like(format!("%{}%", name)));
    }

    let paginator = CUserTag::find()
        .filter(conditions)
        .order_by_asc(c_user_tag::Column::SortOrder)
        .order_by_desc(c_user_tag::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let tags = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<UserTagModel> = tags
        .into_iter()
        .map(|t| UserTagModel {
            id: t.id,
            name: t.name,
            tag_type: t.tag_type,
            category: t.category,
            color: t.color,
            icon: t.icon,
            description: t.description,
            is_active: t.is_active,
            created_at: t.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn add_user_tags(params: AddUserTagParams) -> Result<()> {
    let db = DB_WRITE().await;

    let _consumer = CConsumer::find_by_id(params.consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();

    for tag_id in params.tag_ids {
        let existing = CUserTagRelation::find()
            .filter(c_user_tag_relation::Column::ConsumerId.eq(params.consumer_id))
            .filter(c_user_tag_relation::Column::TagId.eq(tag_id))
            .one(db)
            .await
            .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

        if existing.is_some() {
            continue;
        }

        let relation = c_user_tag_relation::ActiveModel {
            id: Set(generate_id()),
            consumer_id: Set(params.consumer_id),
            tag_id: Set(tag_id),
            source: Set(params.source.clone()),
            source_desc: Set(params.source_desc.clone()),
            created_at: Set(Some(now)),
        };

        let _ = CUserTagRelation::insert(relation).exec(db).await;
    }

    Ok(())
}

pub async fn remove_user_tags(params: RemoveUserTagParams) -> Result<()> {
    let db = DB_WRITE().await;

    for tag_id in params.tag_ids {
        CUserTagRelation::delete_many()
            .filter(c_user_tag_relation::Column::ConsumerId.eq(params.consumer_id))
            .filter(c_user_tag_relation::Column::TagId.eq(tag_id))
            .exec(db)
            .await
            .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;
    }

    Ok(())
}

pub async fn get_user_tags(consumer_id: i64) -> Result<Vec<UserTagModel>> {
    let db = DB_WRITE().await;

    let relations = CUserTagRelation::find()
        .filter(c_user_tag_relation::Column::ConsumerId.eq(consumer_id))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let tag_ids: Vec<i64> = relations.iter().map(|r| r.tag_id).collect();

    if tag_ids.is_empty() {
        return Ok(vec![]);
    }

    let tags = CUserTag::find()
        .filter(c_user_tag::Column::Id.is_in(tag_ids))
        .filter(c_user_tag::Column::IsActive.eq(true))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(tags
        .into_iter()
        .map(|t| UserTagModel {
            id: t.id,
            name: t.name,
            tag_type: t.tag_type,
            category: t.category,
            color: t.color,
            icon: t.icon,
            description: t.description,
            is_active: t.is_active,
            created_at: t.created_at,
        })
        .collect())
}

pub async fn get_users_by_tag(tag_id: i64, page_num: u32, page_size: u32) -> Result<(Vec<i64>, u64)> {
    let db = DB_WRITE().await;

    let paginator = CUserTagRelation::find()
        .filter(c_user_tag_relation::Column::TagId.eq(tag_id))
        .paginate(db, page_size as u64);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let relations = paginator
        .fetch_page((page_num - 1) as u64)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let consumer_ids: Vec<i64> = relations.into_iter().map(|r| r.consumer_id).collect();

    Ok((consumer_ids, total))
}
