use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use crate::common::error::Error;
use crate::domain::cms_plugin::*;

pub struct EditorFieldPlugin;

impl EditorFieldPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CmsPlugin for EditorFieldPlugin {
    fn id(&self) -> &str {
        "builtin.editor"
    }

    fn name(&self) -> &str {
        "富文本编辑器"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "富文本编辑器字段类型"
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
impl FieldTypePlugin for EditorFieldPlugin {
    fn field_type(&self) -> &str {
        "editor"
    }

    fn field_type_name(&self) -> &str {
        "富文本"
    }

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "editor",
            "props": {
                "placeholder": config.placeholder,
                "disabled": config.disabled,
                "height": 400,
            }
        })
    }

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value {
        serde_json::json!({
            "type": "html",
            "props": {
                "width": config.width,
                "align": config.align,
                "truncate": 100,
            }
        })
    }

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error> {
        if rules.required && value.is_empty() {
            return Err(Error::validation_error("此字段为必填项"));
        }

        if let Some(max) = rules.max_length {
            let plain_text = strip_html_tags(value);
            if plain_text.len() > max as usize {
                return Err(Error::validation_error(&format!("内容长度不能超过{}个字符", max)));
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

fn strip_html_tags(html: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

impl std::fmt::Debug for EditorFieldPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditorFieldPlugin")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}

pub fn register() -> Arc<dyn FieldTypePlugin> {
    Arc::new(EditorFieldPlugin::new())
}
