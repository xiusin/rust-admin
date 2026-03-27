use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct NumberFieldPlugin;

impl NumberFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for NumberFieldPlugin {
    fn id(&self) -> &str {
        "builtin.number"
    }

    fn name(&self) -> &str {
        "数字字段"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "数字字段类型，支持整数和小数"
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
impl FieldTypePlugin for NumberFieldPlugin {
    fn field_type(&self) -> &str {
        "number"
    }

    fn field_type_name(&self) -> &str {
        "数字"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "input-number",
            "props": {
                "placeholder": config.placeholder,
                "disabled": config.disabled,
                "precision": 2,
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "number",
            "props": {
                "width": config.width,
                "align": config.align.unwrap_or_else(|| "right".to_string()),
            }
        })
    }

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error> {
        if rules.required && value.is_empty() {
            return Err(Error::validation_error("此字段为必填项"));
        }

        if !value.is_empty() {
            let num: f64 = value.parse().map_err(|_| Error::validation_error("请输入有效的数字"))?;

            if let Some(min) = &rules.min_length {
                let min_val = *min as f64;
                if num < min_val {
                    return Err(Error::validation_error(&format!("值不能小于{}", min)));
                }
            }

            if let Some(max) = &rules.max_length {
                let max_val = *max as f64;
                if num > max_val {
                    return Err(Error::validation_error(&format!("值不能大于{}", max)));
                }
            }
        }

        Ok(())
    }

    fn to_frontend(&self, value: &str) -> serde_json::Value {
        value.parse::<f64>().ok().map(serde_json::json).unwrap_or(serde_json::json!(null))
    }

    fn to_database(&self, value: &serde_json::Value) -> String {
        if value.is_number() {
            value.to_string()
        } else if let Some(s) = value.as_str() {
            s.to_string()
        } else {
            "0".to_string()
        }
    }
}

impl std::fmt::Debug for NumberFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumberFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(NumberFieldPlugin::new())
}
