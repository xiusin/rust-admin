use crate::common::error::Error;
use crate::domain::args::a_shipping_template::*;
use crate::domain::entity::{p_product, p_shipping_template, p_shipping_template_region};
use crate::domain::model::m_shipping_template::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub async fn list(args: ShippingTemplateListArgs) -> Result<ListData<ShippingTemplateListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_shipping_template::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_shipping_template::Column::Name.contains(name));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(p_shipping_template::Column::Status.eq(status));
    }

    let total = p_shipping_template::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_shipping_template::Entity::find()
        .filter(conditions)
        .order_by_asc(p_shipping_template::Column::Id)
        .order_by_desc(p_shipping_template::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let template_ids: Vec<i64> = items.iter().map(|item| item.id).collect();
    let product_counts = get_product_count_by_template_ids(&template_ids, db).await?;

    let list: Vec<ShippingTemplateListItem> = items
        .into_iter()
        .map(|item| {
            let product_count = product_counts.get(&item.id).copied().unwrap_or(0);
            ShippingTemplateListItem {
                id: item.id,
                name: item.name,
                charge_type: item.charge_type,
                charge_type_name: get_charge_type_name(item.charge_type),
                is_free: item.is_free,
                free_condition_type: item.free_condition_type,
                free_condition_value: item.free_condition_value.to_string().parse().unwrap_or(0.0),
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

async fn get_product_count_by_template_ids(template_ids: &[i64], db: &DatabaseConnection) -> Result<HashMap<i64, i64>> {
    if template_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let products = p_product::Entity::find()
        .filter(p_product::Column::ShippingTemplateId.is_in(template_ids.to_vec()))
        .filter(p_product::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    let mut result = HashMap::new();
    for product in products {
        if let Some(template_id) = product.shipping_template_id {
            *result.entry(template_id).or_insert(0) += 1;
        }
    }

    Ok(result)
}

pub async fn detail(id: i64) -> Result<ShippingTemplateDetail> {
    let db = DB().await;

    let item = p_shipping_template::Entity::find_by_id(id)
        .filter(p_shipping_template::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("运费模板不存在"))?;

    let regions = p_shipping_template_region::Entity::find()
        .filter(p_shipping_template_region::Column::TemplateId.eq(id))
        .all(db)
        .await?;

    let region_items: Vec<ShippingRegionItem> = regions
        .into_iter()
        .map(|r| ShippingRegionItem {
            id: r.id,
            template_id: r.template_id,
            region_type: r.region_type,
            region_ids: r.region_ids,
            region_names: r.region_names,
            first_unit: r.first_unit,
            first_fee: r.first_fee.to_string().parse().unwrap_or(0.0),
            continue_unit: r.continue_unit,
            continue_fee: r.continue_fee.to_string().parse().unwrap_or(0.0),
            is_free: r.is_free,
            free_condition_value: r.free_condition_value.to_string().parse().unwrap_or(0.0),
        })
        .collect();

    Ok(ShippingTemplateDetail {
        id: item.id,
        name: item.name,
        charge_type: item.charge_type,
        is_free: item.is_free,
        free_condition_type: item.free_condition_type,
        free_condition_value: item.free_condition_value.to_string().parse().unwrap_or(0.0),
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
        regions: region_items,
    })
}

pub async fn add(args: ShippingTemplateAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_shipping_template::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        charge_type: Set(args.charge_type.unwrap_or(1)),
        is_free: Set(args.is_free.unwrap_or(0)),
        free_condition_type: Set(args.free_condition_type.unwrap_or(0)),
        free_condition_value: Set(args.free_condition_value.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        status: Set(args.status.unwrap_or_else(|| "0".to_string())),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    if let Some(regions) = args.regions {
        for region in regions {
            let region_id = GID().await;
            let region_model = p_shipping_template_region::ActiveModel {
                id: Set(region_id),
                template_id: Set(id),
                region_type: Set(region.region_type.unwrap_or(1)),
                region_ids: Set(region.region_ids),
                region_names: Set(region.region_names),
                first_unit: Set(region.first_unit.unwrap_or(1)),
                first_fee: Set(region.first_fee.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                continue_unit: Set(region.continue_unit.unwrap_or(1)),
                continue_fee: Set(region.continue_fee.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                is_free: Set(region.is_free.unwrap_or(0)),
                free_condition_value: Set(region.free_condition_value.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };
            region_model.insert(db).await?;
        }
    }

    Ok(id)
}

pub async fn edit(args: ShippingTemplateEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_shipping_template::Entity::find_by_id(args.id)
        .filter(p_shipping_template::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("运费模板不存在"))?;

    let mut model: p_shipping_template::ActiveModel = item.into();
    model.name = Set(args.name);
    model.charge_type = Set(args.charge_type.unwrap_or(1));
    model.is_free = Set(args.is_free.unwrap_or(0));
    model.free_condition_type = Set(args.free_condition_type.unwrap_or(0));
    model.free_condition_value = Set(args.free_condition_value.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.status = Set(args.status.unwrap_or_else(|| "0".to_string()));
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    p_shipping_template_region::Entity::delete_many()
        .filter(p_shipping_template_region::Column::TemplateId.eq(args.id))
        .exec(db)
        .await?;

    if let Some(regions) = args.regions {
        for region in regions {
            let region_id = GID().await;
            let region_model = p_shipping_template_region::ActiveModel {
                id: Set(region_id),
                template_id: Set(args.id),
                region_type: Set(region.region_type.unwrap_or(1)),
                region_ids: Set(region.region_ids),
                region_names: Set(region.region_names),
                first_unit: Set(region.first_unit.unwrap_or(1)),
                first_fee: Set(region.first_fee.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                continue_unit: Set(region.continue_unit.unwrap_or(1)),
                continue_fee: Set(region.continue_fee.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                is_free: Set(region.is_free.unwrap_or(0)),
                free_condition_value: Set(region.free_condition_value.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };
            region_model.insert(db).await?;
        }
    }

    Ok(())
}

pub async fn delete(args: ShippingTemplateDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for id in args.ids {
        let item = p_shipping_template::Entity::find_by_id(id)
            .filter(p_shipping_template::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(item) = item {
            let mut model: p_shipping_template::ActiveModel = item.into();
            model.deleted_at = Set(Some(now));
            model.update(db).await?;

            p_shipping_template_region::Entity::delete_many()
                .filter(p_shipping_template_region::Column::TemplateId.eq(id))
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn calculate(args: ShippingCalculateArgs) -> Result<ShippingFeeResult> {
    let db = DB().await;

    let template = p_shipping_template::Entity::find_by_id(args.template_id)
        .filter(p_shipping_template::Column::DeletedAt.is_null())
        .filter(p_shipping_template::Column::Status.eq("0"))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("运费模板不存在或已禁用"))?;

    if template.is_free == 1 {
        return Ok(ShippingFeeResult {
            fee: 0.0,
            template_name: template.name.clone(),
            calculation_detail: "包邮模板，免运费".to_string(),
        });
    }

    if template.free_condition_type > 0 {
        let total_value = match template.free_condition_type {
            1 => args.total_amount.unwrap_or(0.0),
            2 => args.total_quantity.unwrap_or(0) as f64,
            3 => args.total_weight.unwrap_or(0.0),
            _ => 0.0,
        };

        let free_value: f64 = template.free_condition_value.to_string().parse().unwrap_or(0.0);
        if total_value >= free_value {
            return Ok(ShippingFeeResult {
                fee: 0.0,
                template_name: template.name.clone(),
                calculation_detail: format!("满足包邮条件：{} >= {}", total_value, free_value),
            });
        }
    }

    let regions = p_shipping_template_region::Entity::find()
        .filter(p_shipping_template_region::Column::TemplateId.eq(args.template_id))
        .all(db)
        .await?;

    let mut matched_region: Option<p_shipping_template_region::Model> = None;
    let mut default_region: Option<p_shipping_template_region::Model> = None;

    for region in regions {
        if region.region_type == 1 {
            default_region = Some(region);
        } else if let Some(region_ids) = &region.region_ids {
            let ids: Vec<&str> = region_ids.split(',').collect();
            if ids.contains(&args.province.as_str()) {
                matched_region = Some(region);
                break;
            }
        }
    }

    let region = matched_region.or(default_region).ok_or_else(|| Error::bad_request("未找到匹配的运费规则"))?;

    let first_fee: f64 = region.first_fee.to_string().parse().unwrap_or(0.0);
    let continue_fee: f64 = region.continue_fee.to_string().parse().unwrap_or(0.0);
    let first_unit = region.first_unit;
    let continue_unit = region.continue_unit;

    let fee = match template.charge_type {
        1 => {
            let quantity = args.total_quantity.unwrap_or(1) as i32;
            let mut fee = first_fee;
            if quantity > first_unit {
                let extra = quantity - first_unit;
                let extra_units = (extra + continue_unit - 1) / continue_unit;
                fee += extra_units as f64 * continue_fee;
            }
            fee
        }
        2 => {
            let weight = args.total_weight.unwrap_or(0.0);
            let mut fee = first_fee;
            if weight > first_unit as f64 {
                let extra = weight - first_unit as f64;
                let extra_units = (extra / continue_unit as f64).ceil();
                fee += extra_units as f64 * continue_fee;
            }
            fee
        }
        3 => {
            let volume = args.total_volume.unwrap_or(0.0);
            let mut fee = first_fee;
            if volume > first_unit as f64 {
                let extra = volume - first_unit as f64;
                let extra_units = (extra / continue_unit as f64).ceil();
                fee += extra_units as f64 * continue_fee;
            }
            fee
        }
        _ => 0.0,
    };

    Ok(ShippingFeeResult {
        fee,
        template_name: template.name,
        calculation_detail: format!(
            "计费方式: {}, 首件: {}个/{}元，续费: {}个/{}元",
            get_charge_type_name(template.charge_type),
            first_unit,
            first_fee,
            continue_unit,
            continue_fee
        ),
    })
}

pub async fn simple_list() -> Result<Vec<ShippingTemplateSimple>> {
    let db = DB().await;

    let items = p_shipping_template::Entity::find()
        .filter(p_shipping_template::Column::DeletedAt.is_null())
        .filter(p_shipping_template::Column::Status.eq("0"))
        .order_by_asc(p_shipping_template::Column::Id)
        .all(db)
        .await?;

    let list: Vec<ShippingTemplateSimple> = items
        .into_iter()
        .map(|item| ShippingTemplateSimple {
            id: item.id,
            name: item.name,
            charge_type: item.charge_type,
        })
        .collect();

    Ok(list)
}

fn get_charge_type_name(charge_type: i32) -> String {
    match charge_type {
        1 => "按件计费".to_string(),
        2 => "按重量计费".to_string(),
        3 => "按体积计费".to_string(),
        _ => "未知".to_string(),
    }
}
