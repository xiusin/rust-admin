use crate::common::error::Error;
use crate::domain::args::a_product_category::*;
use crate::domain::entity::p_category;
use crate::domain::model::m_product_category::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: CategoryListArgs) -> Result<ListData<CategoryListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_category::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_category::Column::Name.contains(name));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_category::Column::Status.eq(status));
    }
    if let Some(parent_id) = args.parent_id {
        conditions = conditions.add(p_category::Column::ParentId.eq(parent_id));
    }

    let total = p_category::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_category::Entity::find()
        .filter(conditions)
        .order_by_asc(p_category::Column::Sort)
        .order_by_desc(p_category::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<CategoryListItem> = items
        .into_iter()
        .map(|item| CategoryListItem {
            id: item.id,
            parent_id: item.parent_id,
            name: item.name,
            icon: item.icon,
            image: item.image,
            sort: item.sort,
            level: item.level,
            path: item.path,
            status: item.status,
            show_in_nav: item.show_in_nav,
            created_at: item.created_at,
            parent_name: None,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn tree(args: CategoryTreeArgs) -> Result<Vec<CategoryTreeItem>> {
    let db = DB().await;

    let mut conditions = Condition::all();
    conditions = conditions.add(p_category::Column::DeletedAt.is_null());

    if let Some(status) = &args.status {
        conditions = conditions.add(p_category::Column::Status.eq(status));
    }

    let items = p_category::Entity::find()
        .filter(conditions)
        .order_by_asc(p_category::Column::Sort)
        .all(db)
        .await?;

    let tree = build_category_tree(&items, 0);

    Ok(tree)
}

fn build_category_tree(items: &[p_category::Model], parent_id: i64) -> Vec<CategoryTreeItem> {
    items
        .iter()
        .filter(|item| item.parent_id == parent_id)
        .map(|item| CategoryTreeItem {
            id: item.id,
            parent_id: item.parent_id,
            name: item.name.clone(),
            icon: item.icon.clone(),
            image: item.image.clone(),
            sort: item.sort,
            level: item.level,
            path: item.path.clone(),
            status: item.status.clone(),
            show_in_nav: item.show_in_nav,
            children: if item.level < 4 {
                Some(build_category_tree(items, item.id))
            } else {
                None
            },
        })
        .collect()
}

pub async fn detail(id: i64) -> Result<CategoryDetail> {
    let db = DB().await;

    let item = p_category::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    Ok(CategoryDetail {
        id: item.id,
        parent_id: item.parent_id,
        name: item.name,
        icon: item.icon,
        image: item.image,
        sort: item.sort,
        level: item.level,
        path: item.path,
        status: item.status,
        show_in_nav: item.show_in_nav,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: CategoryAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let (level, path) = if let Some(parent_id) = args.parent_id {
        if parent_id == 0 {
            (1, id.to_string())
        } else {
            let parent = p_category::Entity::find_by_id(parent_id)
                .one(db)
                .await?
                .ok_or_else(|| Error::bad_request("父分类不存在"))?;

            if parent.level >= 4 {
                return Err(Error::bad_request("分类层级不能超过4级"));
            }

            let level = parent.level + 1;
            let path = format!("{},{}", parent.path, id);
            (level, path)
        }
    } else {
        (1, id.to_string())
    };

    let model = p_category::ActiveModel {
        id: Set(id),
        parent_id: Set(args.parent_id.unwrap_or(0)),
        name: Set(args.name),
        icon: Set(args.icon),
        image: Set(args.image),
        sort: Set(args.sort.unwrap_or(0)),
        level: Set(level),
        path: Set(path),
        status: Set(args.status.unwrap_or_else(|| "0".to_string())),
        show_in_nav: Set(args.show_in_nav.unwrap_or(0)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: CategoryEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_category::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let mut model: p_category::ActiveModel = item.into();
    model.name = Set(args.name);
    model.icon = Set(args.icon);
    model.image = Set(args.image);
    model.sort = Set(args.sort.unwrap_or(0));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.show_in_nav = Set(args.show_in_nav.unwrap_or(0));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(args: CategoryDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_category::Entity::find_by_id(id)
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_category::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;
        }
    }

    Ok(())
}

pub async fn update_status(id: i64, status: String) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_category::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let mut model: p_category::ActiveModel = item.into();
    model.status = Set(status);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}
