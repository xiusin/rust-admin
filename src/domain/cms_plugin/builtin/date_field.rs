use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct DateFieldPlugin;

impl DateFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for DateFieldPlugin {
    fn id(&self) -> &str {
        "builtin.date"
    }

    fn name(&self) -> &str {
        "日期字段"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "日期选择字段类型"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::FieldType
    }

    async fn init(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn enable(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn disable(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn uninstall(&self) -> Result<(), Error> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl FieldTypePlugin for DateFieldPlugin {
    fn field_type(&self) -> &str {
        "date"
    }

    fn field_type_name(&self) -> &str {
        "日期"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "date-picker",
            "props": {
                "placeholder": config.placeholder,
                "disabled": config.disabled,
                "format": "YYYY-MM-DD",
                "valueFormat": "YYYY-MM-DD",
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "date",
            "props": {
                "width": config.width,
                "align": config.align,
                "format": "YYYY-MM-DD",
            }
        })
    }

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error> {
        if rules.required && value.is_empty() {
            return Err(Error::validation_error("此字段为必填项"));
        }

        if !value.is_empty() {
            chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .map_err(|_| Error::validation_error("日期格式不正确，请使用 YYYY-MM-DD 格式"))?;
        }

        Ok(())
    }

    fn to_frontend(&self, value: &str) -> serde_json::Value {
        serde_json::json!(value)
    }

    fn to_database(&self, value: &serde_json::Value) -> String {
        value.as_str().unwrap_or("").to_string()
    }
}

impl std::fmt::Debug for DateFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DateFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(DateFieldPlugin::new())
}
