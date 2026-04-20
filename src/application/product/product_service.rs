use crate::common::error::Error;
use crate::domain::args::a_product::*;
use crate::domain::entity::{p_product, p_sku, p_spec, p_spec_value, p_product_group_relation, p_product_category, p_product_attribute};
use crate::domain::model::m_product::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: ProductListArgs) -> Result<ListData<ProductListItem>> {
    let db = DB().await;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);

    let mut conditions = Condition::all();
    conditions = conditions.add(p_product::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(p_product::Column::Name.contains(name));
    }
    if let Some(category_id) = args.category_id {
        conditions = conditions.add(p_product::Column::CategoryId.eq(category_id));
    }
    if let Some(brand_id) = args.brand_id {
        conditions = conditions.add(p_product::Column::BrandId.eq(brand_id));
    }
    if let Some(status) = args.status {
        conditions = conditions.add(p_product::Column::Status.eq(status));
    }
    if let Some(audit_status) = args.audit_status {
        conditions = conditions.add(p_product::Column::AuditStatus.eq(audit_status));
    }
    if let Some(product_type) = args.product_type {
        conditions = conditions.add(p_product::Column::ProductType.eq(product_type));
    }
    if let Some(is_hot) = args.is_hot {
        conditions = conditions.add(p_product::Column::IsHot.eq(is_hot));
    }
    if let Some(is_new) = args.is_new {
        conditions = conditions.add(p_product::Column::IsNew.eq(is_new));
    }
    if let Some(is_recommend) = args.is_recommend {
        conditions = conditions.add(p_product::Column::IsRecommend.eq(is_recommend));
    }

    let total = p_product::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = p_product::Entity::find()
        .filter(conditions)
        .order_by_desc(p_product::Column::Sort)
        .order_by_desc(p_product::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<ProductListItem> = items
        .into_iter()
        .map(|item| ProductListItem {
            id: item.id,
            category_id: item.category_id,
            category_name: None,
            brand_id: item.brand_id,
            brand_name: None,
            name: item.name,
            subtitle: item.subtitle,
            cover_image: item.cover_image,
            images: item.images.and_then(|v| serde_json::from_value(v).ok()),
            product_type: item.product_type,
            product_type_name: get_product_type_name(item.product_type),
            status: item.status,
            status_name: get_status_name(item.status),
            audit_status: item.audit_status,
            audit_status_name: get_audit_status_name(item.audit_status),
            sale_status: item.sale_status,
            sale_status_name: get_sale_status_name(item.sale_status),
            line_price: item.line_price.to_string().parse().unwrap_or(0.0),
            sale_price: item.sale_price.to_string().parse().unwrap_or(0.0),
            cost_price: item.cost_price.to_string().parse().unwrap_or(0.0),
            stock: item.stock,
            sales: item.sales,
            virtual_sales: item.virtual_sales,
            is_multi_spec: item.is_multi_spec,
            is_hot: item.is_hot,
            is_new: item.is_new,
            is_recommend: item.is_recommend,
            sort: item.sort,
            created_at: item.created_at,
            updated_at: item.updated_at,
            sku_count: 0,
            group_names: None,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn detail(id: i64) -> Result<ProductDetail> {
    let db = DB().await;

    let item = p_product::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let specs = get_product_specs(id, db).await?;
    let skus = get_product_skus(id, db).await?;
    let attributes = get_product_attributes(id, db).await?;
    let group_ids = get_product_group_ids(id, db).await?;
    let category_ids = get_product_category_ids(id, db).await?;

    Ok(ProductDetail {
        id: item.id,
        category_id: item.category_id,
        category_name: None,
        brand_id: item.brand_id,
        brand_name: None,
        name: item.name,
        subtitle: item.subtitle,
        cover_image: item.cover_image,
        images: item.images.and_then(|v| serde_json::from_value(v).ok()),
        video: item.video,
        detail: item.detail,
        product_type: item.product_type,
        status: item.status,
        audit_status: item.audit_status,
        audit_remark: item.audit_remark,
        sale_status: item.sale_status,
        sale_time: item.sale_time,
        line_price: item.line_price.to_string().parse().unwrap_or(0.0),
        sale_price: item.sale_price.to_string().parse().unwrap_or(0.0),
        cost_price: item.cost_price.to_string().parse().unwrap_or(0.0),
        stock: item.stock,
        sales: item.sales,
        virtual_sales: item.virtual_sales,
        limit_buy: item.limit_buy,
        limit_type: item.limit_type,
        shipping_method: item.shipping_method,
        shipping_template_id: item.shipping_template_id,
        shipping_template_name: None,
        weight: item.weight.to_string().parse().unwrap_or(0.0),
        volume: item.volume.to_string().parse().unwrap_or(0.0),
        unit: item.unit,
        sort: item.sort,
        is_multi_spec: item.is_multi_spec,
        is_hot: item.is_hot,
        is_new: item.is_new,
        is_recommend: item.is_recommend,
        keywords: item.keywords,
        description: item.description,
        created_at: item.created_at,
        updated_at: item.updated_at,
        category_ids: Some(category_ids),
        group_ids: Some(group_ids),
        specs,
        skus,
        attributes,
    })
}

async fn get_product_specs(product_id: i64, db: &DatabaseConnection) -> Result<Vec<SpecItem>> {
    let specs = p_spec::Entity::find()
        .filter(p_spec::Column::ProductId.eq(product_id))
        .order_by_asc(p_spec::Column::Sort)
        .all(db)
        .await?;

    if specs.is_empty() {
        return Ok(vec![]);
    }

    let spec_ids: Vec<i64> = specs.iter().map(|s| s.id).collect();

    let values = p_spec_value::Entity::find()
        .filter(p_spec_value::Column::SpecId.is_in(spec_ids))
        .order_by_asc(p_spec_value::Column::Sort)
        .all(db)
        .await?;

    let mut values_map: std::collections::HashMap<i64, Vec<p_spec_value::Model>> = std::collections::HashMap::new();
    for v in values {
        values_map.entry(v.spec_id).or_default().push(v);
    }

    let mut result = Vec::new();
    for spec in specs {
        let spec_values = values_map.remove(&spec.id).unwrap_or_default();
        result.push(SpecItem {
            id: spec.id,
            name: spec.name,
            sort: spec.sort,
            values: spec_values
                .into_iter()
                .map(|v| SpecValueItem {
                    id: v.id,
                    value: v.value,
                    image: v.image,
                    color_code: v.color_code,
                    sort: v.sort,
                })
                .collect(),
        });
    }

    Ok(result)
}

async fn get_product_skus(product_id: i64, db: &DatabaseConnection) -> Result<Vec<SkuItem>> {
    let skus = p_sku::Entity::find()
        .filter(p_sku::Column::ProductId.eq(product_id))
        .filter(p_sku::Column::DeletedAt.is_null())
        .order_by_asc(p_sku::Column::Id)
        .all(db)
        .await?;

    Ok(skus
        .into_iter()
        .map(|sku| SkuItem {
            id: sku.id,
            sku_code: sku.sku_code,
            spec_value_ids: sku.spec_value_ids,
            spec_text: sku.spec_text,
            image: sku.image,
            sale_price: sku.sale_price.to_string().parse().unwrap_or(0.0),
            line_price: sku.line_price.to_string().parse().unwrap_or(0.0),
            cost_price: sku.cost_price.to_string().parse().unwrap_or(0.0),
            stock: sku.stock,
            sales: sku.sales,
            weight: sku.weight.to_string().parse().unwrap_or(0.0),
            volume: sku.volume.to_string().parse().unwrap_or(0.0),
            status: sku.status,
        })
        .collect())
}

async fn get_product_attributes(product_id: i64, db: &DatabaseConnection) -> Result<Vec<ProductAttributeItem>> {
    let attrs = p_product_attribute::Entity::find()
        .filter(p_product_attribute::Column::ProductId.eq(product_id))
        .all(db)
        .await?;

    Ok(attrs
        .into_iter()
        .map(|attr| ProductAttributeItem {
            id: attr.id,
            attribute_id: attr.attribute_id,
            attribute_name: attr.attribute_name,
            attribute_value: attr.attribute_value,
        })
        .collect())
}

async fn get_product_group_ids(product_id: i64, db: &DatabaseConnection) -> Result<Vec<i64>> {
    let relations = p_product_group_relation::Entity::find()
        .filter(p_product_group_relation::Column::ProductId.eq(product_id))
        .all(db)
        .await?;

    Ok(relations.into_iter().map(|r| r.group_id).collect())
}

async fn get_product_category_ids(product_id: i64, db: &DatabaseConnection) -> Result<Vec<i64>> {
    let relations = p_product_category::Entity::find()
        .filter(p_product_category::Column::ProductId.eq(product_id))
        .all(db)
        .await?;

    Ok(relations.into_iter().map(|r| r.category_id).collect())
}

pub async fn add(args: ProductAddArgs, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let images_json = args.images.and_then(|v| serde_json::to_value(v).ok());

    let model = p_product::ActiveModel {
        id: Set(id),
        category_id: Set(args.category_id),
        brand_id: Set(args.brand_id),
        name: Set(args.name),
        subtitle: Set(args.subtitle),
        cover_image: Set(args.cover_image),
        images: Set(images_json),
        video: Set(args.video),
        detail: Set(args.detail),
        product_type: Set(args.product_type.unwrap_or(1)),
        status: Set(2),
        audit_status: Set(1),
        audit_remark: Set(None),
        sale_status: Set(2),
        sale_time: Set(None),
        line_price: Set(args.line_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        sale_price: Set(args.sale_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        cost_price: Set(args.cost_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        stock: Set(args.stock.unwrap_or(0)),
        sales: Set(0),
        virtual_sales: Set(args.virtual_sales.unwrap_or(0)),
        limit_buy: Set(args.limit_buy.unwrap_or(0)),
        limit_type: Set(args.limit_type.unwrap_or(1)),
        shipping_method: Set(args.shipping_method.unwrap_or(1)),
        shipping_template_id: Set(args.shipping_template_id),
        weight: Set(args.weight.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        volume: Set(args.volume.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        unit: Set(args.unit.unwrap_or_else(|| "件".to_string())),
        sort: Set(args.sort.unwrap_or(0)),
        is_multi_spec: Set(args.is_multi_spec.unwrap_or(0)),
        is_hot: Set(args.is_hot.unwrap_or(0)),
        is_new: Set(args.is_new.unwrap_or(0)),
        is_recommend: Set(args.is_recommend.unwrap_or(0)),
        keywords: Set(args.keywords),
        description: Set(args.description),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    if let Some(group_ids) = args.group_ids {
        save_product_groups(id, &group_ids, db).await?;
    }

    if let Some(category_ids) = args.category_ids {
        save_product_categories(id, &category_ids, db).await?;
    }

    if let Some(specs) = args.specs {
        save_product_specs(id, &specs, db).await?;
    }

    if let Some(skus) = args.skus {
        save_product_skus(id, &skus, db).await?;
    }

    if let Some(attributes) = args.attributes {
        save_product_attributes(id, &attributes, db).await?;
    }

    Ok(id)
}

async fn save_product_groups(product_id: i64, group_ids: &[i64], db: &DatabaseConnection) -> Result<()> {
    if group_ids.is_empty() {
        return Ok(());
    }
    let mut models = Vec::new();
    for group_id in group_ids {
        let id = GID().await;
        models.push(p_product_group_relation::ActiveModel {
            id: Set(id),
            product_id: Set(product_id),
            group_id: Set(*group_id),
            created_at: Set(Some(Local::now().naive_local())),
        });
    }
    p_product_group_relation::Entity::insert_many(models).exec(db).await?;
    Ok(())
}

async fn save_product_categories(product_id: i64, category_ids: &[i64], db: &DatabaseConnection) -> Result<()> {
    if category_ids.is_empty() {
        return Ok(());
    }
    let mut models = Vec::new();
    for category_id in category_ids {
        let id = GID().await;
        models.push(p_product_category::ActiveModel {
            id: Set(id),
            product_id: Set(product_id),
            category_id: Set(*category_id),
            created_at: Set(Some(Local::now().naive_local())),
        });
    }
    p_product_category::Entity::insert_many(models).exec(db).await?;
    Ok(())
}

async fn save_product_specs(product_id: i64, specs: &[SpecArgs], db: &DatabaseConnection) -> Result<()> {
    if specs.is_empty() {
        return Ok(());
    }
    let mut spec_models = Vec::new();
    let mut value_models = Vec::new();

    for spec in specs {
        let spec_id = GID().await;
        spec_models.push(p_spec::ActiveModel {
            id: Set(spec_id),
            product_id: Set(product_id),
            name: Set(spec.name.clone()),
            sort: Set(spec.sort.unwrap_or(0)),
            created_at: Set(Some(Local::now().naive_local())),
        });

        for value in &spec.values {
            let value_id = GID().await;
            value_models.push(p_spec_value::ActiveModel {
                id: Set(value_id),
                spec_id: Set(spec_id),
                product_id: Set(product_id),
                value: Set(value.value.clone()),
                image: Set(value.image.clone()),
                color_code: Set(value.color_code.clone()),
                sort: Set(value.sort.unwrap_or(0)),
                created_at: Set(Some(Local::now().naive_local())),
            });
        }
    }

    if !spec_models.is_empty() {
        p_spec::Entity::insert_many(spec_models).exec(db).await?;
    }
    if !value_models.is_empty() {
        p_spec_value::Entity::insert_many(value_models).exec(db).await?;
    }
    Ok(())
}

async fn save_product_skus(product_id: i64, skus: &[SkuAddArgs], db: &DatabaseConnection) -> Result<()> {
    if skus.is_empty() {
        return Ok(());
    }
    let mut models = Vec::new();
    for sku in skus {
        let id = GID().await;
        models.push(p_sku::ActiveModel {
            id: Set(id),
            product_id: Set(product_id),
            sku_code: Set(sku.sku_code.clone().unwrap_or_else(|| format!("SKU-{}", id))),
            spec_value_ids: Set(sku.spec_value_ids.clone().unwrap_or_default()),
            spec_text: Set(sku.spec_text.clone().unwrap_or_default()),
            image: Set(sku.image.clone()),
            sale_price: Set(rust_decimal::Decimal::try_from(sku.sale_price).unwrap_or(rust_decimal::Decimal::ZERO)),
            line_price: Set(sku.line_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
            cost_price: Set(sku.cost_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
            stock: Set(sku.stock.unwrap_or(0)),
            sales: Set(0),
            weight: Set(sku.weight.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
            volume: Set(sku.volume.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
            status: Set(sku.status.unwrap_or(1)),
            created_at: Set(Some(Local::now().naive_local())),
            updated_at: Set(Some(Local::now().naive_local())),
            deleted_at: Set(None),
        });
    }
    p_sku::Entity::insert_many(models).exec(db).await?;
    Ok(())
}

async fn save_product_attributes(product_id: i64, attributes: &[ProductAttributeArgs], db: &DatabaseConnection) -> Result<()> {
    if attributes.is_empty() {
        return Ok(());
    }
    let mut models = Vec::new();
    for attr in attributes {
        let id = GID().await;
        models.push(p_product_attribute::ActiveModel {
            id: Set(id),
            product_id: Set(product_id),
            attribute_id: Set(attr.attribute_id.unwrap_or(0)),
            attribute_name: Set(attr.attribute_name.clone()),
            attribute_value: Set(attr.attribute_value.clone()),
            created_at: Set(Some(Local::now().naive_local())),
        });
    }
    p_product_attribute::Entity::insert_many(models).exec(db).await?;
    Ok(())
}

pub async fn edit(args: ProductEditArgs, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_product::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let images_json = args.images.and_then(|v| serde_json::to_value(v).ok());

    let mut model: p_product::ActiveModel = item.into();
    model.category_id = Set(args.category_id);
    model.brand_id = Set(args.brand_id);
    model.name = Set(args.name);
    model.subtitle = Set(args.subtitle);
    model.cover_image = Set(args.cover_image);
    model.images = Set(images_json);
    model.video = Set(args.video);
    model.detail = Set(args.detail);
    model.product_type = Set(args.product_type.unwrap_or(1));
    model.line_price = Set(args.line_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.sale_price = Set(args.sale_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.cost_price = Set(args.cost_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.stock = Set(args.stock.unwrap_or(0));
    model.virtual_sales = Set(args.virtual_sales.unwrap_or(0));
    model.limit_buy = Set(args.limit_buy.unwrap_or(0));
    model.limit_type = Set(args.limit_type.unwrap_or(1));
    model.shipping_method = Set(args.shipping_method.unwrap_or(1));
    model.shipping_template_id = Set(args.shipping_template_id);
    model.weight = Set(args.weight.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.volume = Set(args.volume.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO));
    model.unit = Set(args.unit.unwrap_or_else(|| "件".to_string()));
    model.sort = Set(args.sort.unwrap_or(0));
    model.is_hot = Set(args.is_hot.unwrap_or(0));
    model.is_new = Set(args.is_new.unwrap_or(0));
    model.is_recommend = Set(args.is_recommend.unwrap_or(0));
    model.keywords = Set(args.keywords);
    model.description = Set(args.description);
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    if let Some(group_ids) = args.group_ids {
        p_product_group_relation::Entity::delete_many()
            .filter(p_product_group_relation::Column::ProductId.eq(args.id))
            .exec(db)
            .await?;
        save_product_groups(args.id, &group_ids, db).await?;
    }

    if let Some(category_ids) = args.category_ids {
        p_product_category::Entity::delete_many()
            .filter(p_product_category::Column::ProductId.eq(args.id))
            .exec(db)
            .await?;
        save_product_categories(args.id, &category_ids, db).await?;
    }

    Ok(())
}

pub async fn delete(args: ProductDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    if args.ids.is_empty() {
        return Ok(());
    }

    p_product::Entity::update_many()
        .col_expr(p_product::Column::DeletedAt, Expr::value(now))
        .filter(p_product::Column::Id.is_in(args.ids))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn update_status(id: i64, status: i32) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_product::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let mut model: p_product::ActiveModel = item.into();
    model.status = Set(status);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn audit(id: i64, audit_status: i32, audit_remark: Option<String>) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = p_product::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let mut model: p_product::ActiveModel = item.into();
    model.audit_status = Set(audit_status);
    model.audit_remark = Set(audit_remark);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn batch_update_status(ids: Vec<i64>, status: i32) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    if ids.is_empty() {
        return Ok(());
    }

    p_product::Entity::update_many()
        .col_expr(p_product::Column::Status, Expr::value(status))
        .col_expr(p_product::Column::UpdatedAt, Expr::value(now))
        .filter(p_product::Column::Id.is_in(ids))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn statistics() -> Result<ProductStatistics> {
    let db = DB().await;

    let total_products = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    let on_sale_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Status.eq(1))
        .count(db)
        .await?;

    let off_sale_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Status.eq(2))
        .count(db)
        .await?;

    let out_of_stock_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::Stock.eq(0))
        .count(db)
        .await?;

    let pending_audit_count = p_product::Entity::find()
        .filter(p_product::Column::DeletedAt.is_null())
        .filter(p_product::Column::AuditStatus.eq(1))
        .count(db)
        .await?;

    Ok(ProductStatistics {
        total_products: total_products as i64,
        on_sale_count: on_sale_count as i64,
        off_sale_count: off_sale_count as i64,
        out_of_stock_count: out_of_stock_count as i64,
        pending_audit_count: pending_audit_count as i64,
    })
}

fn get_product_type_name(product_type: i32) -> String {
    match product_type {
        1 => "实物商品".to_string(),
        2 => "虚拟商品".to_string(),
        3 => "核销商品".to_string(),
        _ => "未知".to_string(),
    }
}

fn get_status_name(status: i32) -> String {
    match status {
        1 => "上架".to_string(),
        2 => "下架".to_string(),
        3 => "回收站".to_string(),
        _ => "未知".to_string(),
    }
}

fn get_audit_status_name(audit_status: i32) -> String {
    match audit_status {
        1 => "待审核".to_string(),
        2 => "已审核".to_string(),
        3 => "审核不通过".to_string(),
        _ => "未知".to_string(),
    }
}

fn get_sale_status_name(sale_status: i32) -> String {
    match sale_status {
        1 => "开售".to_string(),
        2 => "未开售".to_string(),
        _ => "未知".to_string(),
    }
}

pub async fn batch_delete(args: ProductBatchDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    if args.ids.is_empty() {
        return Ok(());
    }

    p_product::Entity::update_many()
        .col_expr(p_product::Column::DeletedAt, Expr::value(now))
        .filter(p_product::Column::Id.is_in(args.ids.clone()))
        .filter(p_product::Column::DeletedAt.is_null())
        .exec(db)
        .await?;

    Ok(())
}

pub async fn simple_list(args: Option<ProductSimpleListArgs>) -> Result<Vec<ProductSimple>> {
    let db = DB().await;

    let mut conditions = Condition::all();
    conditions = conditions.add(p_product::Column::DeletedAt.is_null());

    if let Some(ref a) = args {
        if let Some(ref name) = a.name {
            conditions = conditions.add(p_product::Column::Name.contains(name));
        }
        if let Some(status) = a.status {
            conditions = conditions.add(p_product::Column::Status.eq(status));
        }
    }

    let items = p_product::Entity::find()
        .filter(conditions)
        .order_by_asc(p_product::Column::Sort)
        .all(db)
        .await?;

    Ok(items.into_iter().map(|p| ProductSimple {
        id: p.id,
        name: p.name,
        cover_image: p.cover_image,
        sale_price: p.sale_price.to_string().parse().unwrap_or(0.0),
        stock: p.stock,
        status: p.status,
    }).collect())
}

pub async fn sku_list(product_id: i64) -> Result<Vec<SkuItem>> {
    let db = DB().await;

    let skus = p_sku::Entity::find()
        .filter(p_sku::Column::ProductId.eq(product_id))
        .filter(p_sku::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    Ok(skus.into_iter().map(|s| SkuItem {
        id: s.id,
        sku_code: s.sku_code,
        spec_value_ids: s.spec_value_ids,
        spec_text: s.spec_text,
        image: s.image,
        sale_price: s.sale_price.to_string().parse().unwrap_or(0.0),
        line_price: s.line_price.to_string().parse().unwrap_or(0.0),
        cost_price: s.cost_price.to_string().parse().unwrap_or(0.0),
        stock: s.stock,
        sales: s.sales,
        weight: s.weight.to_string().parse().unwrap_or(0.0),
        volume: s.volume.to_string().parse().unwrap_or(0.0),
        status: s.status,
    }).collect())
}

pub async fn sku_detail(id: i64) -> Result<SkuItem> {
    let db = DB().await;

    let sku = p_sku::Entity::find_by_id(id)
        .filter(p_sku::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("SKU不存在"))?;

    Ok(SkuItem {
        id: sku.id,
        sku_code: sku.sku_code,
        spec_value_ids: sku.spec_value_ids,
        spec_text: sku.spec_text,
        image: sku.image,
        sale_price: sku.sale_price.to_string().parse().unwrap_or(0.0),
        line_price: sku.line_price.to_string().parse().unwrap_or(0.0),
        cost_price: sku.cost_price.to_string().parse().unwrap_or(0.0),
        stock: sku.stock,
        sales: sku.sales,
        weight: sku.weight.to_string().parse().unwrap_or(0.0),
        volume: sku.volume.to_string().parse().unwrap_or(0.0),
        status: sku.status,
    })
}

pub async fn sku_add(args: SkuAddArgs) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = p_sku::ActiveModel {
        id: Set(id),
        product_id: Set(args.product_id),
        sku_code: Set(args.sku_code.unwrap_or_else(|| format!("SKU{}", id))),
        spec_value_ids: Set(args.spec_value_ids.unwrap_or_default()),
        spec_text: Set(args.spec_text.unwrap_or_default()),
        image: Set(args.image),
        sale_price: Set(rust_decimal::Decimal::try_from(args.sale_price).unwrap_or(rust_decimal::Decimal::ZERO)),
        line_price: Set(args.line_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        cost_price: Set(args.cost_price.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        stock: Set(args.stock.unwrap_or(0)),
        sales: Set(0),
        weight: Set(args.weight.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        volume: Set(args.volume.map(|v| rust_decimal::Decimal::try_from(v).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO)),
        status: Set(args.status.unwrap_or(0)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;
    Ok(id)
}

pub async fn sku_edit(args: SkuEditArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let sku = p_sku::Entity::find_by_id(args.id)
        .filter(p_sku::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("SKU不存在"))?;

    let mut model: p_sku::ActiveModel = sku.into();
    if let Some(sku_code) = args.sku_code {
        model.sku_code = Set(sku_code);
    }
    if let Some(spec_value_ids) = args.spec_value_ids {
        model.spec_value_ids = Set(spec_value_ids);
    }
    if let Some(spec_text) = args.spec_text {
        model.spec_text = Set(spec_text);
    }
    if let Some(image) = args.image {
        model.image = Set(Some(image));
    }
    if let Some(sale_price) = args.sale_price {
        model.sale_price = Set(rust_decimal::Decimal::try_from(sale_price).unwrap_or(rust_decimal::Decimal::ZERO));
    }
    if let Some(line_price) = args.line_price {
        model.line_price = Set(rust_decimal::Decimal::try_from(line_price).unwrap_or(rust_decimal::Decimal::ZERO));
    }
    if let Some(cost_price) = args.cost_price {
        model.cost_price = Set(rust_decimal::Decimal::try_from(cost_price).unwrap_or(rust_decimal::Decimal::ZERO));
    }
    if let Some(stock) = args.stock {
        model.stock = Set(stock);
    }
    if let Some(weight) = args.weight {
        model.weight = Set(rust_decimal::Decimal::try_from(weight).unwrap_or(rust_decimal::Decimal::ZERO));
    }
    if let Some(volume) = args.volume {
        model.volume = Set(rust_decimal::Decimal::try_from(volume).unwrap_or(rust_decimal::Decimal::ZERO));
    }
    if let Some(status) = args.status {
        model.status = Set(status);
    }
    model.updated_at = Set(Some(now));

    model.update(db).await?;
    Ok(())
}

pub async fn sku_delete(args: SkuDeleteArgs) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    if args.ids.is_empty() {
        return Ok(());
    }

    p_sku::Entity::update_many()
        .col_expr(p_sku::Column::DeletedAt, Expr::value(now))
        .filter(p_sku::Column::Id.is_in(args.ids.clone()))
        .filter(p_sku::Column::DeletedAt.is_null())
        .exec(db)
        .await?;

    Ok(())
}

pub async fn sku_generate(args: SkuGenerateArgs) -> Result<Vec<SkuItem>> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let product = p_product::Entity::find_by_id(args.product_id)
        .filter(p_product::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("商品不存在"))?;

    let sku_id = GID().await;
    let spec_text = args.specs.iter().map(|s| {
        let values = s.values.iter().map(|v| v.value.clone()).collect::<Vec<_>>().join(",");
        format!("{}:{}", s.name, values)
    }).collect::<Vec<_>>().join(";");

    let sku_model = p_sku::ActiveModel {
        id: Set(sku_id),
        product_id: Set(args.product_id),
        sku_code: Set(format!("SKU{}", sku_id)),
        spec_value_ids: Set(String::new()),
        spec_text: Set(spec_text.clone()),
        image: Set(None),
        sale_price: Set(product.sale_price),
        line_price: Set(product.line_price),
        cost_price: Set(product.cost_price),
        stock: Set(args.stock.unwrap_or(0)),
        sales: Set(0),
        weight: Set(product.weight),
        volume: Set(product.volume),
        status: Set(0),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    sku_model.insert(db).await?;

    Ok(vec![SkuItem {
        id: sku_id,
        sku_code: format!("SKU{}", sku_id),
        spec_value_ids: String::new(),
        spec_text,
        image: None,
        sale_price: product.sale_price.to_string().parse().unwrap_or(0.0),
        line_price: product.line_price.to_string().parse().unwrap_or(0.0),
        cost_price: product.cost_price.to_string().parse().unwrap_or(0.0),
        stock: args.stock.unwrap_or(0),
        sales: 0,
        weight: product.weight.to_string().parse().unwrap_or(0.0),
        volume: product.volume.to_string().parse().unwrap_or(0.0),
        status: 0,
    }])
}
