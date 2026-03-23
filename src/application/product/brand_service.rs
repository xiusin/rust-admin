use crate::common::error::Error;
use crate::domain::args::a_product_brand::*;
use crate::domain::entity::p_brand;
use crate::domain::model::m_product_brand::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: BrandListArgs) -> Result<ListData<BrandListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_brand::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_brand::Column::Name.contains(name));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_brand::Column::Status.eq(status));
    }

    let total = p_brand::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_brand::Entity::find()
        .filter(conditions)
        .order_by_asc(p_brand::Column::Sort)
        .order_by_desc(p_brand::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<BrandListItem> = items
        .into_iter()
        .map(|item| BrandListItem {
            id: item.id,
            name: item.name,
            logo: item.logo,
            description: item.description,
            sort: item.sort,
            status: item.status,
            product_count: 0,
            created_at: item.created_at,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn detail(id: i64) -> Result<BrandDetail> {
    let db = DB().await;

    let item = p_brand::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("品牌不存在"))?;

    Ok(BrandDetail {
        id: item.id,
        name: item.name,
        logo: item.logo,
        description: item.description,
        sort: item.sort,
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: BrandAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_brand::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        logo: Set(args.logo),
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

pub async fn edit(args: BrandEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_brand::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("品牌不存在"))?;

    let mut model: p_brand::ActiveModel = item.into();
    model.name = Set(args.name);
    model.logo = Set(args.logo);
    model.description = Set(args.description);
    model.sort = Set(args.sort.unwrap_or(0));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(args: BrandDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_brand::Entity::find_by_id(id)
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_brand::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;
        }
    }

    Ok(())
}

pub async fn update_status(id: i64, status: String) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_brand::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("品牌不存在"))?;

    let mut model: p_brand::ActiveModel = item.into();
    model.status = Set(status);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn simple_list() -> Result<Vec<BrandSimple>> {
    let db = DB().await;

    let items = p_brand::Entity::find()
        .filter(p_brand::Column::DeletedAt.is_null())
        .filter(p_brand::Column::Status.eq("0"))
        .order_by_asc(p_brand::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<BrandSimple> = items
        .into_iter()
        .map(|item| BrandSimple {
            id: item.id,
            name: item.name,
            logo: item.logo,
        })
        .collect();

    Ok(list)
}
