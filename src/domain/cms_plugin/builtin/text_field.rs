use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct TextFieldPlugin;

impl TextFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for TextFieldPlugin {
    fn id(&self) -> &str {
        "builtin.text"
    }

    fn name(&self) -> &str {
        "文本字段"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "基础文本字段类型"
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
impl FieldTypePlugin for TextFieldPlugin {
    fn field_type(&self) -> &str {
        "text"
    }

    fn field_type_name(&self) -> &str {
        "单行文本"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "input",
            "props": {
                "placeholder": config.placeholder,
                "maxLength": config.max_length,
                "disabled": config.disabled,
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "text",
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
        if let Some(max) = rules.max_length {
            if value.len() > max as usize {
                return Err(Error::validation_error(&format!("长度不能超过{}", max)));
            }
        }
        if let Some(min) = rules.min_length {
            if value.len() < min as usize {
                return Err(Error::validation_error(&format!("长度不能少于{}", min)));
            }
        }
        if let Some(pattern) = &rules.pattern {
            let regex = regex::Regex::new(pattern).map_err(|e| Error::validation_error(e.to_string()))?;
            if !regex.is_match(value) {
                return Err(Error::validation_error("格式不正确"));
            }
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

impl std::fmt::Debug for TextFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(TextFieldPlugin::new())
}
