use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct ImageFieldPlugin;

impl ImageFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for ImageFieldPlugin {
    fn id(&self) -> &str {
        "builtin.image"
    }

    fn name(&self) -> &str {
        "图片字段"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "图片上传字段类型"
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
impl FieldTypePlugin for ImageFieldPlugin {
    fn field_type(&self) -> &str {
        "image"
    }

    fn field_type_name(&self) -> &str {
        "图片"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "upload",
            "props": {
                "placeholder": config.placeholder,
                "disabled": config.disabled,
                "listType": "picture-card",
                "accept": "image/*",
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "image",
            "props": {
                "width": config.width,
                "align": config.align,
            }
        })
    }

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error> {
        if rules.required && value.is_empty() {
            return Err(Error::validation_error("请上传图片"));
        }
        Ok(())
    }

    fn to_frontend(&self, value: &str) -> serde_json::Value {
        if value.is_empty() {
            serde_json::json!(null)
        } else {
            let urls: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
            serde_json::json!(urls)
        }
    }

    fn to_database(&self, value: &serde_json::Value) -> String {
        if let Some(arr) = value.as_array() {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(",")
        } else if let Some(s) = value.as_str() {
            s.to_string()
        } else {
            String::new()
        }
    }
}

impl std::fmt::Debug for ImageFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(ImageFieldPlugin::new())
}
