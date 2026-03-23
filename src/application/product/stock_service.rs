use crate::common::error::Error;
use crate::domain::args::a_stock::*;
use crate::domain::entity::{p_product, p_sku, p_stock_log, p_stock_alert};
use crate::domain::model::m_stock::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: StockListArgs) -> Result<ListData<StockListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_product::Column::DeletedAt.is_null());

    if let Some(name) = &args.product_name {
        conditions = conditions.add(p_product::Column::Name.contains(name));
    }
    if let Some(category_id) = args.category_id {
        conditions = conditions.add(p_product::Column::CategoryId.eq(category_id));
    }

    let total = p_product::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let products = p_product::Entity::find()
        .filter(conditions)
        .order_by_desc(p_product::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let product_ids: Vec<i64> = products.iter().map(|p| p.id).collect();

    let skus = p_sku::Entity::find()
        .filter(p_sku::Column::ProductId.is_in(product_ids.clone()))
        .filter(p_sku::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    let sku_map: HashMap<i64, Vec<p_sku::Model>> = skus
        .into_iter()
        .fold(HashMap::new(), |mut acc, sku| {
            acc.entry(sku.product_id).or_default().push(sku);
            acc
        });

    let alerts = p_stock_alert::Entity::find()
        .filter(p_stock_alert::Column::ProductId.is_in(product_ids.clone()))
        .all(db)
        .await?;

    let alert_map: HashMap<(i64, Option<i64>), i32> = alerts
        .into_iter()
        .map(|a| ((a.product_id, a.sku_id), a.alert_stock))
        .collect();

    let mut list: Vec<StockListItem> = Vec::new();

    for product in products {
        let product_skus = sku_map.get(&product.id).cloned().unwrap_or_default();

        if product_skus.is_empty() {
            let alert_key = alert_map.get(&(product.id, None)).copied();
            let is_alert = alert_key.map(|a| product.stock <= a).unwrap_or(false);
            list.push(StockListItem {
                product_id: product.id,
                product_name: product.name.clone(),
                product_image: product.cover_image.clone(),
                category_name: None,
                sku_id: None,
                sku_code: None,
                spec_text: None,
                stock: product.stock,
                sales: product.sales,
                alert_stock: alert_key,
                is_alert,
                status: product.status,
            });
        } else {
            for sku in product_skus {
                let alert_key = alert_map.get(&(product.id, Some(sku.id))).copied();
                let is_alert = alert_key.map(|a| sku.stock <= a).unwrap_or(false);
                list.push(StockListItem {
                    product_id: product.id,
                    product_name: product.name.clone(),
                    product_image: sku.image.clone().unwrap_or(product.cover_image.clone()),
                    category_name: None,
                    sku_id: Some(sku.id),
                    sku_code: Some(sku.sku_code.clone()),
                    spec_text: Some(sku.spec_text.clone()),
                    stock: sku.stock,
                    sales: sku.sales,
                    alert_stock: alert_key,
                    is_alert,
                    status: sku.status,
                });
            }
        }
    }

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

fn get_change_type_name(change_type: i32) -> String {
    match change_type {
        0 => "增加".to_string(),
        1 => "减少".to_string(),
        2 => "预扣".to_string(),
        3 => "释放".to_string(),
        4 => "扣减".to_string(),
        5 => "退货入库".to_string(),
        6 => "盘点调整".to_string(),
        _ => "未知".to_string(),
    }
}

pub async fn statistics() -> Result<StockStatistics> {
    let db = DB().await;

    let products = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Status.eq(1))
        .all(db)
        .await?;

    let total_stock: i64 = products.iter().map(|p| p.stock as i64).sum();

    let alert_count = p_stock_alert::Entity::find()
        .filter(p_stock_alert::Column::IsAlert.eq(1))
        .count(db)
        .await?;

    let out_of_stock_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Stock.eq(0))
        .count(db)
        .await?;

    let low_stock_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Stock.lte(10))
        .filter(p_product::Column::Stock.gt(0))
        .count(db)
        .await?;

    Ok(StockStatistics {
        total_stock,
        alert_count: alert_count as i64,
        out_of_stock_count: out_of_stock_count as i64,
        low_stock_count: low_stock_count as i64,
    })
}

pub async fn log_list(args: StockLogListArgs) -> Result<ListData<StockLogItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();

    if let Some(product_id) = args.product_id {
        conditions = conditions.add(p_stock_log::Column::ProductId.eq(product_id));
    }
    if let Some(sku_id) = args.sku_id {
        conditions = conditions.add(p_stock_log::Column::SkuId.eq(sku_id));
    }
    if let Some(change_type) = args.change_type {
        conditions = conditions.add(p_stock_log::Column::ChangeType.eq(change_type));
    }
    if let Some(order_no) = &args.order_no {
        conditions = conditions.add(p_stock_log::Column::OrderNo.contains(order_no));
    }
    if let Some(start_time) = &args.start_time {
        if let Ok(start) = start_time.parse::<DateTime>() {
            conditions = conditions.add(p_stock_log::Column::CreatedAt.gte(start));
        }
    }
    if let Some(end_time) = &args.end_time {
        if let Ok(end) = end_time.parse::<DateTime>() {
            conditions = conditions.add(p_stock_log::Column::CreatedAt.lte(end));
        }
    }

    let total = p_stock_log::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_stock_log::Entity::find()
        .filter(conditions)
        .order_by_desc(p_stock_log::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let product_ids: Vec<i64> = items.iter().map(|i| i.product_id).collect();
    let product_map = get_product_names(&product_ids, db).await?;

    let list: Vec<StockLogItem> = items
        .into_iter()
        .map(|item| {
            let product = product_map.get(&item.product_id);
            StockLogItem {
                id: item.id,
                product_id: item.product_id,
                product_name: product.map(|p| p.name.clone()).unwrap_or_default(),
                sku_id: item.sku_id,
                sku_code: None,
                spec_text: None,
                change_type: item.change_type,
                change_type_name: get_change_type_name(item.change_type),
                change_num: item.change_num,
                before_stock: item.before_stock,
                after_stock: item.after_stock,
                order_no: item.order_no,
                remark: item.remark,
                operator_name: item.operator_name,
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

async fn get_product_names(product_ids: &[i64], db: &DatabaseConnection) -> Result<HashMap<i64, p_product::Model>> {
    if product_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let products = p_product::Entity::find()
        .filter(p_product::Column::Id.is_in(product_ids.to_vec()))
        .all(db)
        .await?;

    Ok(products.into_iter().map(|p| (p.id, p)).collect())
}

pub async fn adjust(args: StockAdjustArgs, user_id: i64, user_name: &str) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let product = p_product::Entity::find_by_id(args.product_id)
        .filter(p_product::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let (before_stock, after_stock) = match args.sku_id {
        Some(sku_id) => {
            let sku = p_sku::Entity::find_by_id(sku_id)
                .filter(p_sku::Column::DeletedAt.is_null())
                .one(db)
                .await?
                .ok_or_else(|| Error::not_found("SKU不存在"))?;

            let before = sku.stock;
            let after = before + args.change_num;
            if after < 0 {
                return Err(Error::bad_request("库存不足"));
            }

            let mut sku_model: p_sku::ActiveModel = sku.into();
            sku_model.stock = Set(after);
            sku_model.updated_at = Set(Some(now));
            sku_model.update(db).await?;

            (before, after)
        }
        None => {
            let before = product.stock;
            let after = before + args.change_num;
            if after < 0 {
                return Err(Error::bad_request("库存不足"));
            }

            let mut product_model: p_product::ActiveModel = product.into();
            product_model.stock = Set(after);
            product_model.updated_at = Set(Some(now));
            product_model.update(db).await?;

            (before, after)
        }
    };

    let id = GID().await;
    let log = p_stock_log::ActiveModel {
        id: Set(id),
        product_id: Set(args.product_id),
        sku_id: Set(args.sku_id),
        change_type: Set(args.change_type),
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

    update_stock_alert(args.product_id, args.sku_id, after_stock, db).await?;

    Ok(())
}

async fn update_stock_alert(product_id: i64, sku_id: Option<i64>, current_stock: i32, db: &DatabaseConnection) -> Result<()> {
    let alert = p_stock_alert::Entity::find()
        .filter(p_stock_alert::Column::ProductId.eq(product_id))
        .filter(match sku_id {
            Some(id) => p_stock_alert::Column::SkuId.eq(id),
            None => p_stock_alert::Column::SkuId.is_null(),
        })
        .one(db)
        .await?;

    if let Some(alert) = alert {
        let alert_stock = alert.alert_stock;
        let mut model: p_stock_alert::ActiveModel = alert.into();
        let is_alert = if current_stock <= alert_stock { 1 } else { 0 };
        model.is_alert = Set(is_alert);
        if is_alert == 1 {
            model.last_alert_at = Set(Some(Local::now().naive_local()));
        }
        model.updated_at = Set(Some(Local::now().naive_local()));
        model.update(db).await?;
    }

    Ok(())
}

pub async fn alert_list(args: StockAlertListArgs) -> Result<ListData<StockAlertItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_product::Column::DeletedAt.is_null());

    if let Some(name) = &args.product_name {
        conditions = conditions.add(p_product::Column::Name.contains(name));
    }
    if let Some(is_alert) = args.is_alert {
        conditions = conditions.add(p_stock_alert::Column::IsAlert.eq(is_alert));
    }

    let total = p_stock_alert::Entity::find()
        .find_also_related(p_product::Entity)
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_stock_alert::Entity::find()
        .find_also_related(p_product::Entity)
        .filter(conditions)
        .order_by_desc(p_stock_alert::Column::LastAlertAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<StockAlertItem> = items
        .into_iter()
        .filter_map(|(alert, product)| {
            product.map(|p| StockAlertItem {
                id: alert.id,
                product_id: alert.product_id,
                product_name: p.name,
                product_image: p.cover_image,
                sku_id: alert.sku_id,
                sku_code: None,
                spec_text: None,
                stock: alert.alert_stock,
                alert_stock: alert.alert_stock,
                is_alert: alert.is_alert,
                last_alert_at: alert.last_alert_at,
            })
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

