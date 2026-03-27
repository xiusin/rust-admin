use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct SelectFieldPlugin;

impl SelectFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for SelectFieldPlugin {
    fn id(&self) -> &str {
        "builtin.select"
    }

    fn name(&self) -> &str {
        "选择字段"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "下拉选择字段类型"
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
impl FieldTypePlugin for SelectFieldPlugin {
    fn field_type(&self) -> &str {
        "select"
    }

    fn field_type_name(&self) -> &str {
        "下拉选择"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "select",
            "props": {
                "placeholder": config.placeholder,
                "disabled": config.disabled,
                "clearable": true,
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "tag",
            "props": {
                "width": config.width,
                "align": config.align,
            }
        })
    }

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error> {
        if rules.required && value.is_empty() {
            return Err(Error::validation_error("此字段为必填项"));
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

impl std::fmt::Debug for SelectFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(SelectFieldPlugin::new())
}
