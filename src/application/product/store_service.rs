use crate::common::error::Error;
use crate::domain::args::a_store::*;
use crate::domain::entity::{p_store, p_store_stock, p_product, p_stock_log};
use crate::domain::model::m_store::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: StoreListArgs) -> Result<ListData<StoreListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_store::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_store::Column::Name.contains(name));
    }
    if let Some(city) = &args.city {
        conditions = conditions.add(p_store::Column::City.contains(city));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_store::Column::Status.eq(status));
    }

    let total = p_store::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_store::Entity::find()
        .filter(conditions)
        .order_by_asc(p_store::Column::Sort)
        .order_by_desc(p_store::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<StoreListItem> = items
        .into_iter()
        .map(|item| StoreListItem {
            id: item.id,
            name: item.name,
            logo: item.logo,
            cover_image: item.cover_image,
            contact_name: item.contact_name,
            contact_phone: item.contact_phone,
            province: item.province,
            city: item.city,
            district: item.district,
            address: item.address,
            longitude: item.longitude.map(|v| v.to_string().parse().unwrap_or(0.0)),
            latitude: item.latitude.map(|v| v.to_string().parse().unwrap_or(0.0)),
            business_hours: item.business_hours,
            sort: item.sort,
            status: item.status,
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

pub async fn detail(id: i64) -> Result<StoreDetail> {
    let db = DB().await;

    let item = p_store::Entity::find_by_id(id)
        .filter(p_store::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("门店不存在"))?;

    Ok(StoreDetail {
        id: item.id,
        name: item.name,
        logo: item.logo,
        cover_image: item.cover_image,
        contact_name: item.contact_name,
        contact_phone: item.contact_phone,
        province: item.province,
        city: item.city,
        district: item.district,
        address: item.address,
        longitude: item.longitude.map(|v| v.to_string().parse().unwrap_or(0.0)),
        latitude: item.latitude.map(|v| v.to_string().parse().unwrap_or(0.0)),
        business_hours: item.business_hours,
        description: item.description,
        sort: item.sort,
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: StoreAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_store::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        logo: Set(args.logo),
        cover_image: Set(args.cover_image),
        contact_name: Set(args.contact_name),
        contact_phone: Set(args.contact_phone),
        province: Set(args.province),
        city: Set(args.city),
        district: Set(args.district),
        address: Set(args.address),
        longitude: Set(args.longitude.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO))),
        latitude: Set(args.latitude.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO))),
        business_hours: Set(args.business_hours),
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

pub async fn edit(args: StoreEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_store::Entity::find_by_id(args.id)
        .filter(p_store::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("门店不存在"))?;

    let mut model: p_store::ActiveModel = item.into();
    model.name = Set(args.name);
    model.logo = Set(args.logo);
    model.cover_image = Set(args.cover_image);
    model.contact_name = Set(args.contact_name);
    model.contact_phone = Set(args.contact_phone);
    model.province = Set(args.province);
    model.city = Set(args.city);
    model.district = Set(args.district);
    model.address = Set(args.address);
    model.longitude = Set(args.longitude.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)));
    model.latitude = Set(args.latitude.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)));
    model.business_hours = Set(args.business_hours);
    model.description = Set(args.description);
    model.sort = Set(args.sort.unwrap_or(0));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(args: StoreDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_store::Entity::find_by_id(id)
            .filter(p_store::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_store::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;

            p_store_stock::Entity::delete_many()
                .filter(p_store_stock::Column::StoreId.eq(id))
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn stock_list(args: StoreStockListArgs) -> Result<ListData<StoreStockItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let store = p_store::Entity::find_by_id(args.store_id)
        .filter(p_store::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("门店不存在"))?;

    let stocks = p_store_stock::Entity::find()
        .filter(p_store_stock::Column::StoreId.eq(args.store_id))
        .all(db)
        .await?;

    let product_ids: Vec<i64> = stocks.iter().map(|s| s.product_id).collect();

    let products = p_product::Entity::find()
        .filter(p_product::Column::Id.is_in(product_ids.clone()))
        .filter(p_product::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    let product_map: HashMap<i64, p_product::Model> = products.into_iter().map(|p| (p.id, p)).collect();

    let total_count = stocks.len() as u64;
    let store_name = store.name.clone();

    let list: Vec<StoreStockItem> = stocks
        .into_iter()
        .filter_map(|stock| {
            let product = product_map.get(&stock.product_id);
            product.map(|p| StoreStockItem {
                id: stock.id,
                store_id: stock.store_id,
                store_name: store_name.clone(),
                product_id: stock.product_id,
                product_name: p.name.clone(),
                product_image: p.cover_image.clone(),
                sku_id: stock.sku_id,
                sku_code: None,
                spec_text: None,
                stock: stock.stock,
                alert_stock: stock.alert_stock,
                is_alert: stock.stock <= stock.alert_stock,
            })
        })
        .collect();

    Ok(ListData {
        list,
        total: total_count,
        total_pages: (total_count + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn stock_adjust(args: StoreStockAdjustArgs, user_id: i64, user_name: &str) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let _store = p_store::Entity::find_by_id(args.store_id)
        .filter(p_store::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("门店不存在"))?;

    let stock = p_store_stock::Entity::find()
        .filter(p_store_stock::Column::StoreId.eq(args.store_id))
        .filter(p_store_stock::Column::ProductId.eq(args.product_id))
        .filter(match args.sku_id {
            Some(sku_id) => p_store_stock::Column::SkuId.eq(sku_id),
            None => p_store_stock::Column::SkuId.is_null(),
        })
        .one(db)
        .await?;

    let (before_stock, after_stock) = match stock {
        Some(stock) => {
            let before = stock.stock;
            let after = before + args.change_num;
            if after < 0 {
                return Err(Error::bad_request("库存不足"));
            }

            let mut model: p_store_stock::ActiveModel = stock.into();
            model.stock = Set(after);
            model.updated_at = Set(Some(now));
            model.update(db).await?;

            (before, after)
        }
        None => {
            let before = 0;
            let after = args.change_num;
            if after < 0 {
                return Err(Error::bad_request("库存不足"));
            }

            let id = GID().await;
            let model = p_store_stock::ActiveModel {
                id: Set(id),
                store_id: Set(args.store_id),
                product_id: Set(args.product_id),
                sku_id: Set(args.sku_id),
                stock: Set(after),
                alert_stock: Set(10),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };
            model.insert(db).await?;

            (before, after)
        }
    };

    let log_id = GID().await;
    let log = p_stock_log::ActiveModel {
        id: Set(log_id),
        product_id: Set(args.product_id),
        sku_id: Set(args.sku_id),
        change_type: Set(6),
        change_num: Set(args.change_num),
        before_stock: Set(before_stock),
        after_stock: Set(after_stock),
        order_no: Set(None),
        remark: Set(args.remark),
        operator_id: Set(Some(user_id)),
        operator_name: Set(Some(user_name.to_string())),
        created_at: Set(Some(now)),
    };
    log.insert(db).await?;

    Ok(())
}

pub async fn simple_list() -> Result<Vec<StoreSimple>> {
    let db = DB().await;

    let items = p_store::Entity::find()
        .filter(p_store::Column::DeletedAt.is_null())
        .filter(p_store::Column::Status.eq("0"))
        .order_by_asc(p_store::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<StoreSimple> = items
        .into_iter()
        .map(|item| StoreSimple {
            id: item.id,
            name: item.name,
            address: item.address,
            contact_phone: item.contact_phone,
        })
        .collect();

    Ok(list)
}

pub async fn statistics() -> Result<StoreStatistics> {
    let db = DB().await;

    let total_stores = p_store::Entity::find()
        .filter(p_store::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    let active_stores = p_store::Entity::find()
        .filter(p_store::Column::DeletedAt.is_null())
        .filter(p_store::Column::Status.eq("0"))
        .count(db)
        .await?;

    let inactive_stores = total_stores - active_stores;

    Ok(StoreStatistics {
        total_stores: total_stores as i64,
        active_stores: active_stores as i64,
        inactive_stores: inactive_stores as i64,
    })
}
