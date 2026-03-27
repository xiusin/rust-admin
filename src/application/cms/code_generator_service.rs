use crate::domain::entity::{cms_field, cms_model};
use crate::domain::model::m_code_gen::GeneratedFile;
use crate::service::prelude::*;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use sea_orm::QueryOrder;
use tera::Context;
use tera::Tera;

lazy_static! {
    static ref TERA: Tera = {
        let mut tera = Tera::default();
        
        tera.add_raw_template("entity", include_str!("templates/entity.rs.tera"))
            .expect("Failed to add entity template");
        tera.add_raw_template("model", include_str!("templates/model.rs.tera"))
            .expect("Failed to add model template");
        tera.add_raw_template("args", include_str!("templates/args.rs.tera"))
            .expect("Failed to add args template");
        tera.add_raw_template("service", include_str!("templates/service.rs.tera"))
            .expect("Failed to add service template");
        tera.add_raw_template("api", include_str!("templates/api.rs.tera"))
            .expect("Failed to add api template");
        tera.add_raw_template("migration", include_str!("templates/migration.sql.tera"))
            .expect("Failed to add migration template");
        
        tera
    };
}

pub async fn preview(model_id: i64) -> Result<Vec<GeneratedFile>> {
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
    
    let mut files = Vec::new();
    
    files.push(GeneratedFile {
        file_path: format!("src/domain/entity/{}.rs", model.code.to_case(Case::Snake)),
        file_name: format!("{}.rs", model.code.to_case(Case::Snake)),
        content: generate_entity(&model, &fields),
        language: "rust".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("src/domain/model/m_{}.rs", model.code.to_case(Case::Snake)),
        file_name: format!("m_{}.rs", model.code.to_case(Case::Snake)),
        content: generate_model(&model, &fields),
        language: "rust".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("src/domain/args/a_{}.rs", model.code.to_case(Case::Snake)),
        file_name: format!("a_{}.rs", model.code.to_case(Case::Snake)),
        content: generate_args(&model, &fields),
        language: "rust".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("src/application/cms/{}_service.rs", model.code.to_case(Case::Snake)),
        file_name: format!("{}_service.rs", model.code.to_case(Case::Snake)),
        content: generate_service(&model, &fields),
        language: "rust".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("src/api/cms/{}.rs", model.code.to_case(Case::Snake)),
        file_name: format!("{}.rs", model.code.to_case(Case::Snake)),
        content: generate_api(&model),
        language: "rust".to_string(),
    });
    
    files.push(GeneratedFile {
        file_path: format!("migrations/m{}_{}_{}_create_table.sql", 
            Local::now().format("%Y%m%d"),
            "001",
            model.code.to_case(Case::Snake)
        ),
        file_name: format!("m{}_{}_{}_create_table.sql", 
            Local::now().format("%Y%m%d"),
            "001",
            model.code.to_case(Case::Snake)
        ),
        content: generate_migration(&model, &fields),
        language: "sql".to_string(),
    });
    
    Ok(files)
}

pub fn generate_entity(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    context.insert("table_name", &model.table_name);
    context.insert("entity_name", &model.code.to_case(Case::Pascal));
    
    let field_list: Vec<serde_json::Value> = fields
        .iter()
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Snake),
                "rust_type": db_type_to_rust_type(&f.db_type),
                "nullable": f.is_required == "0",
                "is_primary": false,
                "comment": f.name,
            })
        })
        .collect();
    
    context.insert("fields", &field_list);
    
    TERA.render("entity", &context).unwrap_or_default()
}

pub fn generate_model(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    context.insert("model_name", &model.code.to_case(Case::Pascal));
    context.insert("model_code", &model.code.to_case(Case::Snake));
    
    let field_list: Vec<serde_json::Value> = fields
        .iter()
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Camel),
                "name": f.name,
                "rust_type": db_type_to_rust_type(&f.db_type),
                "is_required": f.is_required == "1",
            })
        })
        .collect();
    
    context.insert("fields", &field_list);
    
    TERA.render("model", &context).unwrap_or_default()
}

pub fn generate_args(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    context.insert("model_name", &model.code.to_case(Case::Pascal));
    context.insert("model_code", &model.code.to_case(Case::Snake));
    
    let field_list: Vec<serde_json::Value> = fields
        .iter()
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Snake),
                "name": f.name,
                "rust_type": db_type_to_rust_type(&f.db_type),
                "is_required": f.is_required == "1",
                "validation": generate_validation(&f),
            })
        })
        .collect();
    
    context.insert("fields", &field_list);
    
    TERA.render("args", &context).unwrap_or_default()
}

pub fn generate_service(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    context.insert("model_name", &model.code.to_case(Case::Pascal));
    context.insert("model_code", &model.code.to_case(Case::Snake));
    context.insert("table_name", &model.table_name);
    
    let searchable_fields: Vec<serde_json::Value> = fields
        .iter()
        .filter(|f| f.is_searchable == "1")
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Snake),
                "name": f.name,
            })
        })
        .collect();
    
    context.insert("searchable_fields", &searchable_fields);
    
    TERA.render("service", &context).unwrap_or_default()
}

pub fn generate_api(model: &cms_model::Model) -> String {
    let mut context = Context::new();
    context.insert("model_name", &model.code.to_case(Case::Pascal));
    context.insert("model_code", &model.code.to_case(Case::Snake));
    
    TERA.render("api", &context).unwrap_or_default()
}

pub fn generate_migration(model: &cms_model::Model, fields: &[cms_field::Model]) -> String {
    let mut context = Context::new();
    context.insert("table_name", &model.table_name);
    context.insert("model_name", &model.name);
    context.insert("model_code", &model.code);
    
    let field_list: Vec<serde_json::Value> = fields
        .iter()
        .map(|f| {
            json!({
                "code": f.code.to_case(Case::Snake),
                "db_type": convert_db_type_for_migration(&f.db_type),
                "nullable": f.is_required == "0",
                "default_value": f.default_value,
                "comment": f.name,
            })
        })
        .collect();
    
    context.insert("fields", &field_list);
    
    TERA.render("migration", &context).unwrap_or_default()
}

fn db_type_to_rust_type(db_type: &str) -> String {
    match db_type.to_lowercase().as_str() {
        "int" | "integer" | "tinyint" | "smallint" | "mediumint" => "i32".to_string(),
        "bigint" => "i64".to_string(),
        "float" | "double" | "decimal" => "f64".to_string(),
        "bool" | "boolean" => "bool".to_string(),
        "date" | "datetime" | "timestamp" => "DateTime".to_string(),
        "json" | "jsonb" => "serde_json::Value".to_string(),
        _ => "String".to_string(),
    }
}

fn convert_db_type_for_migration(db_type: &str) -> String {
    let db_type_lower = db_type.to_lowercase();
    
    if db_type_lower.starts_with("varchar") || db_type_lower.starts_with("char") {
        return db_type.to_string();
    }
    
    match db_type_lower.as_str() {
        "int" | "integer" => "int(11)".to_string(),
        "tinyint" => "tinyint(4)".to_string(),
        "smallint" => "smallint(6)".to_string(),
        "mediumint" => "mediumint(9)".to_string(),
        "bigint" => "bigint(20)".to_string(),
        "float" => "float".to_string(),
        "double" => "double".to_string(),
        "decimal" => "decimal(10,2)".to_string(),
        "bool" | "boolean" => "tinyint(1)".to_string(),
        "date" => "date".to_string(),
        "datetime" => "datetime".to_string(),
        "timestamp" => "timestamp".to_string(),
        "text" => "text".to_string(),
        "longtext" => "longtext".to_string(),
        "json" => "json".to_string(),
        _ => db_type.to_string(),
    }
}

fn generate_validation(field: &cms_field::Model) -> String {
    let mut validations = Vec::new();
    
    if field.is_required == "1" {
        validations.push(format!("length(min = 1, message = \"{}不能为空\")", field.name));
    }
    
    if let Some(ref validation) = field.validation {
        validations.push(validation.clone());
    }
    
    validations.join(", ")
}
