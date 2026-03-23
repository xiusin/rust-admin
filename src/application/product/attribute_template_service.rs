use crate::common::error::Error;
use crate::domain::args::a_attribute_template::*;
use crate::domain::entity::{p_attribute_template, p_attribute, p_attribute_value};
use crate::domain::model::m_attribute_template::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: AttributeTemplateListArgs) -> Result<ListData<AttributeTemplateListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_attribute_template::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_attribute_template::Column::Name.contains(name));
    }
    if let Some(category_id) = args.category_id {
        conditions = conditions.add(p_attribute_template::Column::CategoryId.eq(category_id));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_attribute_template::Column::Status.eq(status));
    }

    let total = p_attribute_template::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_attribute_template::Entity::find()
        .filter(conditions)
        .order_by_asc(p_attribute_template::Column::Sort)
        .order_by_desc(p_attribute_template::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let template_ids: Vec<i64> = items.iter().map(|item| item.id).collect();
    let attribute_counts = get_attribute_count_by_template_ids(&template_ids, db).await?;

    let list: Vec<AttributeTemplateListItem> = items
        .into_iter()
        .map(|item| {
            let attribute_count = attribute_counts.get(&item.id).copied().unwrap_or(0) as i32;
            AttributeTemplateListItem {
                id: item.id,
                name: item.name,
                category_id: item.category_id,
                category_name: None,
                sort: item.sort,
                status: item.status,
                attribute_count,
                created_at: item.created_at,
            }
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

async fn get_attribute_count_by_template_ids(template_ids: &[i64], db: &DatabaseConnection) -> Result<HashMap<i64, i64>> {
    let counts = p_attribute::Entity::find()
        .filter(p_attribute::Column::TemplateId.is_in(template_ids.to_vec()))
        .group_by(p_attribute::Column::TemplateId)
        .all(db)
        .await?;

    let mut result = HashMap::new();
    for item in counts {
        let count = result.entry(item.template_id).or_insert(0);
        *count += 1;
    }

    Ok(result)
}

pub async fn detail(id: i64) -> Result<AttributeTemplateDetail> {
    let db = DB().await;

    let item = p_attribute_template::Entity::find_by_id(id)
        .filter(p_attribute_template::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("属性模板不存在"))?;

    let attributes = p_attribute::Entity::find()
        .filter(p_attribute::Column::TemplateId.eq(id))
        .order_by_asc(p_attribute::Column::Sort)
        .all(db)
        .await?;

    let attribute_ids: Vec<i64> = attributes.iter().map(|a| a.id).collect();
    let values = p_attribute_value::Entity::find()
        .filter(p_attribute_value::Column::AttributeId.is_in(attribute_ids.clone()))
        .order_by_asc(p_attribute_value::Column::Sort)
        .all(db)
        .await?;

    let mut value_map: HashMap<i64, Vec<p_attribute_value::Model>> = HashMap::new();
    for v in values {
        value_map.entry(v.attribute_id).or_default().push(v);
    }

    let attribute_items: Vec<AttributeItem> = attributes
        .into_iter()
        .map(|attr| {
            let attr_values = value_map.get(&attr.id).cloned().unwrap_or_default();
            AttributeItem {
                id: attr.id,
                name: attr.name,
                attr_type: attr.attr_type,
                is_required: attr.is_required,
                is_filter: attr.is_filter,
                sort: attr.sort,
                status: attr.status,
                values: attr_values
                    .into_iter()
                    .map(|v| AttributeValueItem {
                        id: v.id,
                        value: v.value,
                        sort: v.sort,
                    })
                    .collect(),
            }
        })
        .collect();

    Ok(AttributeTemplateDetail {
        id: item.id,
        name: item.name,
        category_id: item.category_id,
        category_name: None,
        sort: item.sort,
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
        attributes: attribute_items,
    })
}

pub async fn add(args: AttributeTemplateAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_attribute_template::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        category_id: Set(args.category_id),
        sort: Set(args.sort.unwrap_or(0)),
        status: Set(args.status.unwrap_or_else(|| "0".to_string())),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    if let Some(attributes) = args.attributes {
        for attr in attributes {
            let attr_id = GID().await;
            let attr_model = p_attribute::ActiveModel {
                id: Set(attr_id),
                template_id: Set(id),
                name: Set(attr.name),
                attr_type: Set(attr.attr_type.unwrap_or(1)),
                is_required: Set(attr.is_required.unwrap_or(0)),
                is_filter: Set(attr.is_filter.unwrap_or(0)),
                sort: Set(attr.sort.unwrap_or(0)),
                status: Set(attr.status.unwrap_or_else(|| "0".to_string())),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                deleted_at: Set(None),
            };
            attr_model.insert(db).await?;

            if let Some(values) = attr.values {
                for v in values {
                    let v_id = GID().await;
                    let v_model = p_attribute_value::ActiveModel {
                        id: Set(v_id),
                        attribute_id: Set(attr_id),
                        value: Set(v.value),
                        sort: Set(v.sort.unwrap_or(0)),
                        created_at: Set(Some(now)),
                        updated_at: Set(Some(now)),
                        deleted_at: Set(None),
                    };
                    v_model.insert(db).await?;
                }
            }
        }
    }

    Ok(id)
}

pub async fn edit(args: AttributeTemplateEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_attribute_template::Entity::find_by_id(args.id)
        .filter(p_attribute_template::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("属性模板不存在"))?;

    let mut model: p_attribute_template::ActiveModel = item.into();
    model.name = Set(args.name);
    model.category_id = Set(args.category_id);
    model.sort = Set(args.sort.unwrap_or(0));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    p_attribute::Entity::delete_many()
        .filter(p_attribute::Column::TemplateId.eq(args.id))
        .exec(db)
        .await?;

    if let Some(attributes) = args.attributes {
        for attr in attributes {
            let attr_id = GID().await;
            let attr_model = p_attribute::ActiveModel {
                id: Set(attr_id),
                template_id: Set(args.id),
                name: Set(attr.name),
                attr_type: Set(attr.attr_type.unwrap_or(1)),
                is_required: Set(attr.is_required.unwrap_or(0)),
                is_filter: Set(attr.is_filter.unwrap_or(0)),
                sort: Set(attr.sort.unwrap_or(0)),
                status: Set(attr.status.unwrap_or_else(|| "0".to_string())),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                deleted_at: Set(None),
            };
            attr_model.insert(db).await?;

            if let Some(values) = attr.values {
                for v in values {
                    let v_id = GID().await;
                    let v_model = p_attribute_value::ActiveModel {
                        id: Set(v_id),
                        attribute_id: Set(attr_id),
                        value: Set(v.value),
                        sort: Set(v.sort.unwrap_or(0)),
                        created_at: Set(Some(now)),
                        updated_at: Set(Some(now)),
                        deleted_at: Set(None),
                    };
                    v_model.insert(db).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn delete(args: AttributeTemplateDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_attribute_template::Entity::find_by_id(id)
            .filter(p_attribute_template::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_attribute_template::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;

            p_attribute::Entity::delete_many()
                .filter(p_attribute::Column::TemplateId.eq(id))
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn by_category(category_id: i64) -> Result<Vec<AttributeTemplateSimple>> {
    let db = DB().await;

    let items = p_attribute_template::Entity::find()
        .filter(p_attribute_template::Column::DeletedAt.is_null())
        .filter(p_attribute_template::Column::Status.eq("0"))
        .filter(
            p_attribute_template::Column::CategoryId.eq(category_id)
                .or(p_attribute_template::Column::CategoryId.is_null())
        )
        .order_by_asc(p_attribute_template::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<AttributeTemplateSimple> = items
        .into_iter()
        .map(|item| AttributeTemplateSimple {
            id: item.id,
            name: item.name,
        })
        .collect();

    Ok(list)
}
