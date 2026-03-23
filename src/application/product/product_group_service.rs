use crate::common::error::Error;
use crate::domain::args::a_product_group::*;
use crate::domain::entity::{p_product_group, p_product_group_relation};
use crate::domain::model::m_product_group::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: ProductGroupListArgs) -> Result<ListData<ProductGroupListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_product_group::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_product_group::Column::Name.contains(name));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_product_group::Column::Status.eq(status));
    }

    let total = p_product_group::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_product_group::Entity::find()
        .filter(conditions)
        .order_by_asc(p_product_group::Column::Sort)
        .order_by_desc(p_product_group::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let group_ids: Vec<i64> = items.iter().map(|item| item.id).collect();
    let product_counts = get_product_count_by_group_ids(&group_ids, db).await?;

    let list: Vec<ProductGroupListItem> = items
        .into_iter()
        .map(|item| {
            let product_count = product_counts.get(&item.id).copied().unwrap_or(0);
            ProductGroupListItem {
                id: item.id,
                name: item.name,
                description: item.description,
                sort: item.sort,
                status: item.status,
                product_count,
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

async fn get_product_count_by_group_ids(group_ids: &[i64], db: &DatabaseConnection) -> Result<HashMap<i64, i64>> {
    if group_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let relations = p_product_group_relation::Entity::find()
        .filter(p_product_group_relation::Column::GroupId.is_in(group_ids.to_vec()))
        .all(db)
        .await?;

    let mut result: HashMap<i64, i64> = HashMap::new();
    for item in relations {
        *result.entry(item.group_id).or_insert(0) += 1;
    }

    Ok(result)
}

pub async fn detail(id: i64) -> Result<ProductGroupDetail> {
    let db = DB().await;

    let item = p_product_group::Entity::find_by_id(id)
        .filter(p_product_group::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分组不存在"))?;

    Ok(ProductGroupDetail {
        id: item.id,
        name: item.name,
        description: item.description,
        sort: item.sort,
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: ProductGroupAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_product_group::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        description: Set(args.description),
        sort: Set(args.sort.unwrap_or(0)),
        status: Set(args.status.unwrap_or_else(|| "0".to_string())),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: ProductGroupEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_product_group::Entity::find_by_id(args.id)
        .filter(p_product_group::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分组不存在"))?;

    let mut model: p_product_group::ActiveModel = item.into();
    model.name = Set(args.name);
    model.description = Set(args.description);
    model.sort = Set(args.sort.unwrap_or(0));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(args: ProductGroupDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_product_group::Entity::find_by_id(id)
            .filter(p_product_group::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_product_group::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;

            p_product_group_relation::Entity::delete_many()
                .filter(p_product_group_relation::Column::GroupId.eq(id))
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn simple_list() -> Result<Vec<ProductGroupSimple>> {
    let db = DB().await;

    let items = p_product_group::Entity::find()
        .filter(p_product_group::Column::DeletedAt.is_null())
        .filter(p_product_group::Column::Status.eq("0"))
        .order_by_asc(p_product_group::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<ProductGroupSimple> = items
        .into_iter()
        .map(|item| ProductGroupSimple {
            id: item.id,
            name: item.name,
        })
        .collect();

    Ok(list)
}
