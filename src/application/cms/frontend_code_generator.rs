use crate::domain::entity::{cms_field, cms_form_config, cms_model, cms_table_config};
use crate::domain::model::m_code_gen::GeneratedFile;
use crate::service::prelude::*;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use sea_orm::QueryOrder;
use tera::Context;
use tera::Tera;

lazy_static! {
    static ref FRONTEND_TERA: Tera = {
        let mut tera = Tera::default();
        
        tera.add_raw_template("vue_list", include_str!("templates/vue-list.vue.tera"))
            .expect("Failed to add vue_list template");
        tera.add_raw_template("vue_form", include_str!("templates/vue-form.vue.tera"))
            .expect("Failed to add vue_form template");
        tera.add_raw_template("vue_detail", include_str!("templates/vue-detail.vue.tera"))
            .expect("Failed to add vue_detail template");
        tera.add_raw_template("vue_api", include_str!("templates/vue-api.ts.tera"))
            .expect("Failed to add vue_api template");
        tera.add_raw_template("vue_router", include_str!("templates/vue-router.ts.tera"))
            .expect("Failed to add vue_router template");
        
        tera
    };
}

pub async fn generate_frontend_files(model_id: i64) -> Result<Vec<GeneratedFile>> {
    let db = DB().await;
    
    let model = cms_model::Entity::find_by_id(model_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("Model not found"))?;
    
    let fields: Vec<cms_field::Model> = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(model_id))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;
    
    let table_config = cms_table_config::Entity::find()
        .filter(cms_table_config::Column::ModelId.eq(model_id))
        .filter(cms_table_config::Column::IsDefault.eq("1"))
        .one(db)
        .await?;
    
    let form_config = cms_form_config::Entity::find()
        .filter(cms_form_config::Column::ModelId.eq(model_id))
        .filter(cms_form_config::Column::IsDefault.eq("1"))
        .one(db)
        .await?;
    
    let mut files = Vec::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let _model_name_pascal = model.code.to_case(Case::Pascal);
    
    if let Some(ref table_cfg) = table_config {
        files.push(GeneratedFile {
            file_path: format!("src/views/cms/{}/index.vue", model_code_kebab),
            file_name: "index.vue".to_string(),
            content: generate_vue_list(&model, &fields, table_cfg),
            language: "vue".to_string(),
        });
    }
    
    if let Some(ref form_cfg) = form_config {
        files.push(GeneratedFile {
            file_path: format!("src/views/cms/{}/components/FormDialog.vue", model_code_kebab),
            file_name: "FormDialog.vue".to_string(),
            content: generate_vue_form(&model, &fields, form_cfg),
            language: "vue".to_string(),
        });
        
        files.push(GeneratedFile {
            file_path: format!("src/views/cms/{}/components/DetailDialog.vue", model_code_kebab),
            file_name: "DetailDialog.vue".to_string(),
            content: generate_vue_detail(&model, &fields, form_cfg),
            language: "vue".to_string(),
        });
    }
    
    files.push(GeneratedFile {
        file_path: format!("src/api/cms/{}.ts", model_code_kebab),
        file_name: format!("{}.ts", model_code_kebab),
        content: generate_api_file(&model, &fields),
        language: "typescript".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("src/router/modules/cms/{}.ts", model_code_kebab),
        file_name: format!("{}.ts", model_code_kebab),
        content: generate_router_file(&model),
        language: "typescript".to_string(),
    });
    
    Ok(files)
}

pub fn generate_vue_list(
    model: &cms_model::Model,
    fields: &[cms_field::Model],
    table_config: &cms_table_config::Model,
) -> String {
    let mut context = Context::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let model_name_pascal = model.code.to_case(Case::Pascal);
    let model_name_camel = model.code.to_case(Case::Camel);
    
    context.insert("model_code_kebab", &model_code_kebab);
    context.insert("model_name_pascal", &model_name_pascal);
    context.insert("model_name_camel", &model_name_camel);
    context.insert("model_name", &model.name);
    
    let list_fields: Vec<serde_json::Value> = fields
        .iter()
        .filter(|f| f.is_list_show == "1")
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "field_type": f.field_type,
                "is_sortable": f.is_sortable == "1",
                "is_filterable": f.is_filterable == "1",
            })
        })
        .collect();
    
    let search_fields: Vec<serde_json::Value> = fields
        .iter()
        .filter(|f| f.is_searchable == "1")
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "field_type": f.field_type,
            })
        })
        .collect();
    
    context.insert("list_fields", &list_fields);
    context.insert("search_fields", &search_fields);
    
    if let Some(ref columns) = table_config.columns {
        context.insert("table_columns", columns);
    }
    
    FRONTEND_TERA.render("vue_list", &context).unwrap_or_default()
}

pub fn generate_vue_form(
    model: &cms_model::Model,
    fields: &[cms_field::Model],
    form_config: &cms_form_config::Model,
) -> String {
    let mut context = Context::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let model_name_pascal = model.code.to_case(Case::Pascal);
    let model_name_camel = model.code.to_case(Case::Camel);
    
    context.insert("model_code_kebab", &model_code_kebab);
    context.insert("model_name_pascal", &model_name_pascal);
    context.insert("model_name_camel", &model_name_camel);
    context.insert("model_name", &model.name);
    
    let form_fields: Vec<serde_json::Value> = fields
        .iter()
        .filter(|f| f.is_form_show == "1")
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "field_type": f.field_type,
                "is_required": f.is_required == "1",
                "placeholder": format!("请输入{}", f.name),
                "form_config": f.form_config,
            })
        })
        .collect();
    
    context.insert("form_fields", &form_fields);
    
    if let Some(ref layout) = form_config.layout {
        context.insert("form_layout", layout);
    }
    
    FRONTEND_TERA.render("vue_form", &context).unwrap_or_default()
}

pub fn generate_vue_detail(
    model: &cms_model::Model,
    fields: &[cms_field::Model],
    _form_config: &cms_form_config::Model,
) -> String {
    let mut context = Context::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let model_name_pascal = model.code.to_case(Case::Pascal);
    let model_name_camel = model.code.to_case(Case::Camel);
    
    context.insert("model_code_kebab", &model_code_kebab);
    context.insert("model_name_pascal", &model_name_pascal);
    context.insert("model_name_camel", &model_name_camel);
    context.insert("model_name", &model.name);
    
    let detail_fields: Vec<serde_json::Value> = fields
        .iter()
        .filter(|f| f.is_detail_show == "1")
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "field_type": f.field_type,
            })
        })
        .collect();
    
    context.insert("detail_fields", &detail_fields);
    
    FRONTEND_TERA.render("vue_detail", &context).unwrap_or_default()
}

pub fn generate_api_file(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let model_name_pascal = model.code.to_case(Case::Pascal);
    let model_name_camel = model.code.to_case(Case::Camel);
    
    context.insert("model_code_kebab", &model_code_kebab);
    context.insert("model_name_pascal", &model_name_pascal);
    context.insert("model_name_camel", &model_name_camel);
    context.insert("model_name", &model.name);
    
    let api_fields: Vec<serde_json::Value> = fields
        .iter()
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "ts_type": db_type_to_ts_type(&f.db_type),
                "is_required": f.is_required == "1",
            })
        })
        .collect();
    
    context.insert("api_fields", &api_fields);
    
    FRONTEND_TERA.render("vue_api", &context).unwrap_or_default()
}

pub fn generate_router_file(model: &cms_model::Model) -> String {
    let mut context = Context::new();
    
    let model_code_kebab = model.code.to_case(Case::Kebab);
    let model_name_pascal = model.code.to_case(Case::Pascal);
    
    context.insert("model_code_kebab", &model_code_kebab);
    context.insert("model_name_pascal", &model_name_pascal);
    context.insert("model_name", &model.name);
    
    FRONTEND_TERA.render("vue_router", &context).unwrap_or_default()
}

fn db_type_to_ts_type(db_type: &str) -> String {
    match db_type.to_lowercase().as_str() {
        "int" | "integer" | "tinyint" | "smallint" | "mediumint" | "bigint" => "number".to_string(),
        "float" | "double" | "decimal" => "number".to_string(),
        "bool" | "boolean" => "boolean".to_string(),
        "date" | "datetime" | "timestamp" => "string".to_string(),
        "json" | "jsonb" => "Record<string, any>".to_string(),
        _ => "string".to_string(),
    }
}
