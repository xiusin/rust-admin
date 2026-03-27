use crate::common::error::{Error, Result};
use crate::domain::entity::cms_field;
use crate::domain::entity::cms_model;
use crate::service::prelude::*;
use sea_orm::*;

pub fn validate_table_name(table_name: &str) -> Result<()> {
    if !table_name.starts_with("cms_content_") {
        return Err(Error::bad_request("Invalid table name: must start with 'cms_content_'"));
    }
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::bad_request("Invalid table name format: only alphanumeric and underscore allowed"));
    }
    if table_name.len() > 64 {
        return Err(Error::bad_request("Table name too long: maximum 64 characters"));
    }
    Ok(())
}

pub fn validate_field_name(field_name: &str) -> Result<()> {
    if !field_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::bad_request("Invalid field name format: only alphanumeric and underscore allowed"));
    }
    if field_name.len() > 64 {
        return Err(Error::bad_request("Field name too long: maximum 64 characters"));
    }
    if field_name.is_empty() {
        return Err(Error::bad_request("Field name cannot be empty"));
    }
    if field_name.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        return Err(Error::bad_request("Field name cannot start with a number"));
    }
    Ok(())
}

pub async fn create_table(
    model: &cms_model::Model,
    fields: &[cms_field::Model],
) -> Result<()> {
    validate_table_name(&model.table_name)?;
    
    for field in fields {
        validate_field_name(&field.code)?;
    }
    
    let db = DB().await;

    if table_exists(&model.table_name).await? {
        return Err(Error::bad_request("表已存在"));
    }

    let mut sql = format!(
        "CREATE TABLE `{}` (\n  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',\n",
        model.table_name
    );

    for field in fields {
        let column_def = build_column_definition(field)?;
        sql.push_str(&format!("  {},\n", column_def));
    }

    sql.push_str("  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',\n");
    sql.push_str("  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',\n");
    sql.push_str("  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',\n");
    sql.push_str(&format!(
        "  PRIMARY KEY (`id`)\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='{}';",
        model.name
    ));

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    db.execute(stmt).await?;

    Ok(())
}

pub async fn add_column(table_name: &str, field: &cms_field::Model) -> Result<()> {
    validate_table_name(table_name)?;
    validate_field_name(&field.code)?;
    
    let db = DB().await;

    if !table_exists(table_name).await? {
        return Err(Error::not_found("表不存在"));
    }

    if column_exists(table_name, &field.code).await? {
        return Err(Error::bad_request("字段已存在"));
    }

    let column_def = build_column_definition(field)?;
    let sql = format!(
        "ALTER TABLE `{}` ADD COLUMN {};",
        table_name, column_def
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    db.execute(stmt).await?;

    Ok(())
}

pub async fn drop_column(table_name: &str, field_code: &str) -> Result<()> {
    validate_table_name(table_name)?;
    validate_field_name(field_code)?;
    
    let db = DB().await;

    if !table_exists(table_name).await? {
        return Err(Error::not_found("表不存在"));
    }

    if !column_exists(table_name, field_code).await? {
        return Err(Error::not_found("字段不存在"));
    }

    let sql = format!(
        "ALTER TABLE `{}` DROP COLUMN `{}`;",
        table_name, field_code
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    db.execute(stmt).await?;

    Ok(())
}

pub async fn modify_column(table_name: &str, field: &cms_field::Model) -> Result<()> {
    validate_table_name(table_name)?;
    validate_field_name(&field.code)?;
    
    let db = DB().await;

    if !table_exists(table_name).await? {
        return Err(Error::not_found("表不存在"));
    }

    if !column_exists(table_name, &field.code).await? {
        return Err(Error::not_found("字段不存在"));
    }

    let column_def = build_column_definition(field)?;
    let sql = format!(
        "ALTER TABLE `{}` MODIFY COLUMN {};",
        table_name, column_def
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    db.execute(stmt).await?;

    Ok(())
}

pub async fn table_exists(table_name: &str) -> Result<bool> {
    validate_table_name(table_name)?;
    
    let db = DB().await;

    let sql = format!(
        "SELECT COUNT(*) as count FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = '{}'",
        table_name
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    let result = db.query_one(stmt).await?;

    if let Some(row) = result {
        let count: i64 = row.try_get("", "count")?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

pub async fn column_exists(table_name: &str, column_name: &str) -> Result<bool> {
    validate_table_name(table_name)?;
    validate_field_name(column_name)?;
    
    let db = DB().await;

    let sql = format!(
        "SELECT COUNT(*) as count FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = '{}' AND column_name = '{}'",
        table_name, column_name
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    let result = db.query_one(stmt).await?;

    if let Some(row) = result {
        let count: i64 = row.try_get("", "count")?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

pub async fn sync_table(model_id: i64) -> Result<()> {
    let db = DB().await;

    let model = cms_model::Entity::find_by_id(model_id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(model_id))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;

    if table_exists(&model.table_name).await? {
        let existing_columns = get_table_columns(&model.table_name).await?;

        for field in &fields {
            if !existing_columns.contains(&field.code) {
                add_column(&model.table_name, field).await?;
            } else {
                modify_column(&model.table_name, field).await?;
            }
        }

        for column in existing_columns {
            if !fields.iter().any(|f| f.code == column)
                && column != "id"
                && column != "created_at"
                && column != "updated_at"
                && column != "deleted_at"
            {
                drop_column(&model.table_name, &column).await?;
            }
        }
    } else {
        create_table(&model, &fields).await?;
    }

    Ok(())
}

async fn get_table_columns(table_name: &str) -> Result<Vec<String>> {
    validate_table_name(table_name)?;
    
    let db = DB().await;

    let sql = format!(
        "SELECT column_name FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = '{}' ORDER BY ordinal_position",
        table_name
    );

    let stmt = Statement::from_string(DbBackend::MySql, sql);
    let result = db.query_all(stmt).await?;

    let columns: Vec<String> = result
        .iter()
        .filter_map(|row| row.try_get("", "column_name").ok())
        .collect();

    Ok(columns)
}

fn build_column_definition(field: &cms_field::Model) -> Result<String> {
    let db_type = map_db_type(&field.db_type)?;

    let mut def = format!("`{}` {}", field.code, db_type);

    if field.is_required == "1" {
        def.push_str(" NOT NULL");
    }

    if let Some(default) = &field.default_value {
        if !default.is_empty() {
            if field.db_type.starts_with("varchar") || field.db_type.starts_with("text") {
                def.push_str(&format!(" DEFAULT '{}'", default));
            } else {
                def.push_str(&format!(" DEFAULT {}", default));
            }
        }
    } else if field.is_required != "1" {
        def.push_str(" DEFAULT NULL");
    }

    def.push_str(&format!(" COMMENT '{}'", field.name));

    Ok(def)
}

fn map_db_type(field_type: &str) -> Result<String> {
    let db_type = match field_type.to_lowercase().as_str() {
        "int" | "integer" => "int(11)",
        "bigint" => "bigint(20)",
        "smallint" => "smallint(6)",
        "tinyint" => "tinyint(4)",
        "decimal" | "money" => "decimal(10,2)",
        "float" => "float",
        "double" => "double",
        "varchar" | "string" => "varchar(255)",
        "text" => "text",
        "longtext" => "longtext",
        "mediumtext" => "mediumtext",
        "date" => "date",
        "datetime" | "timestamp" => "datetime",
        "time" => "time",
        "boolean" | "bool" => "tinyint(1)",
        "json" => "json",
        _ => {
            if field_type.starts_with("varchar(") || field_type.starts_with("int(") {
                field_type
            } else {
                return Err(Error::bad_request(&format!("不支持的字段类型: {}", field_type)));
            }
        }
    };

    Ok(db_type.to_string())
}
