use crate::common::error::Error;
use crate::domain::args::a_cms_model::{CmsModelAddReq, CmsModelEditReq, CmsModelListReq};
use crate::domain::entity::cms_field;
use crate::domain::entity::cms_model;
use crate::domain::model::m_cms_field::CmsFieldItem;
use crate::domain::model::m_cms_model::{CmsModelDetail, CmsModelList};
use crate::model::prelude::*;
use crate::service::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: CmsModelListReq) -> Result<ListData<CmsModelList>> {
    let db = DB().await;
    let page_num = args.page_num as u64;
    let page_size = args.page_size as u64;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_model::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(cms_model::Column::Name.contains(name));
    }
    if let Some(code) = &args.code {
        conditions = conditions.add(cms_model::Column::Code.contains(code));
    }
    if let Some(is_enabled) = args.is_enabled {
        conditions = conditions.add(cms_model::Column::IsEnabled.eq(is_enabled.to_string()));
    }

    let total = cms_model::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = cms_model::Entity::find()
        .filter(conditions)
        .order_by_asc(cms_model::Column::Sort)
        .order_by_desc(cms_model::Column::CreatedAt)
        .paginate(db, page_size)
        .fetch_page(page_num - 1)
        .await?;

    let model_ids: Vec<i64> = items.iter().map(|m| m.id).collect();

    let field_counts: std::collections::HashMap<i64, i32> = if !model_ids.is_empty() {
        let fields = cms_field::Entity::find()
            .filter(cms_field::Column::ModelId.is_in(model_ids))
            .all(db)
            .await?;

        let mut counts = std::collections::HashMap::new();
        for field in fields {
            *counts.entry(field.model_id).or_insert(0) += 1;
        }
        counts
    } else {
        std::collections::HashMap::new()
    };

    let list: Vec<CmsModelList> = items
        .into_iter()
        .map(|item| CmsModelList {
            id: item.id,
            name: item.name,
            code: item.code,
            table_name: item.table_name,
            icon: item.icon,
            is_enabled: item.is_enabled.parse().unwrap_or(0),
            sort: item.sort,
            field_count: *field_counts.get(&item.id).unwrap_or(&0),
            created_at: item.created_at,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })
}

pub async fn detail(id: i64) -> Result<CmsModelDetail> {
    let db = DB().await;

    let model = cms_model::Entity::find_by_id(id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(id))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;

    let field_items: Vec<CmsFieldItem> = fields
        .into_iter()
        .map(|f| CmsFieldItem {
            id: f.id,
            name: f.name,
            code: f.code,
            field_type: f.field_type,
            db_type: f.db_type,
            is_required: f.is_required.parse().unwrap_or(0),
            sort: f.sort,
        })
        .collect();

    Ok(CmsModelDetail {
        id: model.id,
        name: model.name,
        code: model.code,
        table_name: model.table_name,
        icon: model.icon,
        description: model.description,
        is_enabled: model.is_enabled.parse().unwrap_or(0),
        sort: model.sort,
        created_at: model.created_at,
        updated_at: model.updated_at,
        fields: field_items,
    })
}

pub async fn add(args: CmsModelAddReq, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let existing = cms_model::Entity::find()
        .filter(cms_model::Column::Code.eq(&args.code))
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("模型编码已存在"));
    }

    let existing_table = cms_model::Entity::find()
        .filter(cms_model::Column::TableName.eq(&args.table_name))
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if existing_table.is_some() {
        return Err(Error::bad_request("表名已存在"));
    }

    let model = cms_model::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        code: Set(args.code),
        table_name: Set(args.table_name),
        description: Set(args.description),
        icon: Set(args.icon),
        category_id: Set(args.category_id),
        is_system: Set(args.is_system.to_string()),
        is_enabled: Set(args.is_enabled.to_string()),
        sort: Set(args.sort),
        config: Set(args.config),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: CmsModelEditReq, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_model::Entity::find_by_id(args.id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let mut model: cms_model::ActiveModel = item.into();
    model.name = Set(args.name);
    model.description = Set(args.description);
    model.icon = Set(args.icon);
    model.category_id = Set(args.category_id);
    model.is_enabled = Set(args.is_enabled.to_string());
    model.sort = Set(args.sort);
    model.config = Set(args.config);
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(id: i64, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_model::Entity::find_by_id(id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    if item.is_system == "1" {
        return Err(Error::bad_request("系统模型不能删除"));
    }

    let content_count = crate::domain::entity::cms_content::Entity::find()
        .filter(crate::domain::entity::cms_content::Column::ModelId.eq(id))
        .filter(crate::domain::entity::cms_content::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    if content_count > 0 {
        return Err(Error::bad_request("模型下存在内容，无法删除"));
    }

    let txn = db.begin().await?;

    cms_field::Entity::delete_many()
        .filter(cms_field::Column::ModelId.eq(id))
        .exec(&txn)
        .await?;

    let mut model: cms_model::ActiveModel = item.into();
    model.deleted_at = Set(Some(now));
    model.update(&txn).await?;

    txn.commit().await?;

    Ok(())
}

pub async fn enable(id: i64, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_model::Entity::find_by_id(id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let mut model: cms_model::ActiveModel = item.into();
    model.is_enabled = Set("1".to_string());
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn disable(id: i64, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_model::Entity::find_by_id(id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let mut model: cms_model::ActiveModel = item.into();
    model.is_enabled = Set("0".to_string());
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn copy(id: i64, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let new_id = GID().await;

    let item = cms_model::Entity::find_by_id(id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(id))
        .all(db)
        .await?;

    let new_code = format!("{}_copy", item.code);
    let new_table_name = format!("{}_copy", item.table_name);
    let new_name = format!("{} (副本)", item.name);

    let txn = db.begin().await?;

    let new_model = cms_model::ActiveModel {
        id: Set(new_id),
        name: Set(new_name),
        code: Set(new_code),
        table_name: Set(new_table_name),
        description: Set(item.description.clone()),
        icon: Set(item.icon.clone()),
        category_id: Set(item.category_id),
        is_system: Set("0".to_string()),
        is_enabled: Set("0".to_string()),
        sort: Set(item.sort),
        config: Set(item.config.clone()),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    new_model.insert(&txn).await?;

    for field in fields {
        let field_id = GID().await;
        let new_field = cms_field::ActiveModel {
            id: Set(field_id),
            model_id: Set(new_id),
            name: Set(field.name),
            code: Set(field.code),
            field_type: Set(field.field_type),
            db_type: Set(field.db_type),
            default_value: Set(field.default_value.clone()),
            is_required: Set(field.is_required),
            is_unique: Set(field.is_unique),
            is_searchable: Set(field.is_searchable),
            is_sortable: Set(field.is_sortable),
            is_filterable: Set(field.is_filterable),
            is_list_show: Set(field.is_list_show),
            is_form_show: Set(field.is_form_show),
            is_detail_show: Set(field.is_detail_show),
            form_config: Set(field.form_config.clone()),
            table_config: Set(field.table_config.clone()),
            validation: Set(field.validation.clone()),
            sort: Set(field.sort),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        };

        new_field.insert(&txn).await?;
    }

    txn.commit().await?;

    Ok(new_id)
}
