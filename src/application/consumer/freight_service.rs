use crate::common::error::Error;
use crate::domain::model::m_freight::*;
use crate::domain::entity::prelude::CFreightTemplate;
use crate::domain::entity::c_freight_template;
use crate::model::prelude::*;
use rust_decimal::Decimal;
use sea_orm::*;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn calculate_freight(params: CalculateFreightParams) -> Result<FreightCalculateResp> {
    let db = DB_WRITE().await;

    let template = CFreightTemplate::find_by_id(params.template_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("运费模板不存在"))?;

    let base_price = template.first_price.unwrap_or(Decimal::ZERO);
    let first_weight = template.first_weight.unwrap_or(Decimal::ZERO);
    let additional_price_per_unit = template.additional_price.unwrap_or(Decimal::ZERO);

    let weight_grams = Decimal::try_from(params.weight * 1000.0).unwrap_or(Decimal::ZERO);
    let additional_weight = (weight_grams - first_weight).max(Decimal::ZERO);
    let additional_price = additional_price_per_unit * additional_weight;

    let total_price = base_price + additional_price;

    let free_shipping = template
        .free_shipping_rules
        .as_array()
        .and_then(|rules| {
            serde_json::from_value::<Vec<FreeShippingRule>>(
                serde_json::json!(rules)
            ).ok()
        })
        .map(|rules| {
            rules.iter().any(|r| {
                r.province == params.province && r.city == params.city &&
                    r.min_order_amount.map_or(false, |ma| params.order_amount.map_or(false, |oa| oa >= ma))
            })
        })
        .unwrap_or(false);

    Ok(FreightCalculateResp {
        template_id: params.template_id,
        base_price,
        additional_price,
        total_price,
        free_shipping,
    })
}

pub async fn create_template(params: CreateFreightTemplateParams) -> Result<FreightTemplateModel> {
    let db = DB_WRITE().await;
    let now = chrono::Local::now().naive_local();
    let id = generate_id();

    let region_rules_json = serde_json::to_value(&params.region_rules).unwrap_or_default();
    let free_shipping_rules_json = serde_json::to_value(&params.free_shipping_rules).unwrap_or_default();

    let calc_type = match params.calculation_type {
        CalculationType::ByWeight => "by_weight",
        CalculationType::ByDistance => "by_distance",
    };

    let template = c_freight_template::ActiveModel {
        id: Set(id),
        name: Set(params.name.clone()),
        calculation_type: Set(calc_type.to_string()),
        first_weight: Set(Some(params.first_weight)),
        first_price: Set(Some(params.first_price)),
        additional_weight: Set(Some(params.additional_weight)),
        additional_price: Set(Some(params.additional_price)),
        region_rules: Set(region_rules_json),
        free_shipping_rules: Set(free_shipping_rules_json),
        is_active: Set(true),
        created_by: Set(None),
        updated_by: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    CFreightTemplate::insert(template)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(FreightTemplateModel {
        id,
        name: params.name,
        calculation_type: calc_type.to_string(),
        first_weight: params.first_weight,
        first_price: params.first_price,
        additional_weight: params.additional_weight,
        additional_price: params.additional_price,
        region_rules: params.region_rules,
        free_shipping_rules: params.free_shipping_rules,
        status: "active".to_string(),
        created_at: Some(now),
        updated_at: Some(now),
    })
}

pub async fn list_templates(
    params: FreightTemplateListParams,
) -> Result<(Vec<FreightTemplateModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(name) = &params.name {
        conditions = conditions.add(c_freight_template::Column::Name.like(format!("%{}%", name)));
    }
    if let Some(calculation_type) = &params.calculation_type {
        let calc_type_str = match calculation_type {
            CalculationType::ByWeight => "by_weight",
            CalculationType::ByDistance => "by_distance",
        };
        conditions = conditions.add(c_freight_template::Column::CalculationType.eq(calc_type_str));
    }

    let paginator = CFreightTemplate::find()
        .filter(conditions)
        .order_by_desc(c_freight_template::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let templates = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<FreightTemplateModel> = templates
        .into_iter()
        .map(|t| FreightTemplateModel {
            id: t.id,
            name: t.name,
            calculation_type: t.calculation_type,
            first_weight: t.first_weight.unwrap_or(Decimal::ZERO),
            first_price: t.first_price.unwrap_or(Decimal::ZERO),
            additional_weight: t.additional_weight.unwrap_or(Decimal::ZERO),
            additional_price: t.additional_price.unwrap_or(Decimal::ZERO),
            region_rules: t.region_rules.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::json!(arr)).ok())
                .unwrap_or_default(),
            free_shipping_rules: t.free_shipping_rules.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::json!(arr)).ok())
                .unwrap_or_default(),
            status: if t.is_active { "active".to_string() } else { "inactive".to_string() },
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn update_template(params: UpdateFreightTemplateParams) -> Result<()> {
    let db = DB_WRITE().await;
    let now = chrono::Local::now().naive_local();

    let template = CFreightTemplate::find_by_id(params.id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("运费模板不存在"))?;

    let region_rules_json = serde_json::to_value(&params.region_rules).unwrap_or_default();
    let free_shipping_rules_json = serde_json::to_value(&params.free_shipping_rules).unwrap_or_default();

    let calc_type = match params.calculation_type {
        CalculationType::ByWeight => "by_weight",
        CalculationType::ByDistance => "by_distance",
    };

    let mut active_model: c_freight_template::ActiveModel = template.into();
    active_model.name = Set(params.name);
    active_model.calculation_type = Set(calc_type.to_string());
    active_model.first_weight = Set(Some(params.first_weight));
    active_model.first_price = Set(Some(params.first_price));
    active_model.additional_weight = Set(Some(params.additional_weight));
    active_model.additional_price = Set(Some(params.additional_price));
    active_model.region_rules = Set(region_rules_json);
    active_model.free_shipping_rules = Set(free_shipping_rules_json);
    active_model.updated_at = Set(Some(now));

    active_model
        .update(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn delete_template(id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let result = CFreightTemplate::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Delete error: {}", e)))?;

    if result.rows_affected == 0 {
        return Err(Error::not_found("运费模板不存在"));
    }

    Ok(())
}