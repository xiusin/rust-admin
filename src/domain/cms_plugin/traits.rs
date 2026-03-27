use async_trait::async_trait;
use std::any::Any;

use super::types::*;
use crate::common::error::Error;

#[async_trait]
pub trait CmsPlugin: Send + Sync {
    fn id(&self) -> &str;

    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn description(&self) -> &str;

    fn plugin_type(&self) -> PluginType;

    fn is_paid(&self) -> bool {
        false
    }

    fn price(&self) -> Option<f64> {
        None
    }

    async fn init(&self) -> Result<(), Error>;

    async fn enable(&self) -> Result<(), Error>;

    async fn disable(&self) -> Result<(), Error>;

    async fn uninstall(&self) -> Result<(), Error>;

    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait FieldTypePlugin: CmsPlugin {
    fn field_type(&self) -> &str;

    fn field_type_name(&self) -> &str;

    fn render_form_schema(&self, config: &FormFieldConfig) -> serde_json::Value;

    fn render_table_schema(&self, config: &TableFieldConfig) -> serde_json::Value;

    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<(), Error>;

    fn to_frontend(&self, value: &str) -> serde_json::Value;

    fn to_database(&self, value: &serde_json::Value) -> String;
}

#[async_trait]
pub trait WidgetPlugin: CmsPlugin {
    fn widget_name(&self) -> &str;

    fn render(&self, props: &serde_json::Value) -> serde_json::Value;

    fn config_form(&self) -> serde_json::Value;
}

#[async_trait]
pub trait TemplatePlugin: CmsPlugin {
    fn template_name(&self) -> &str;

    fn template_type(&self) -> TemplateType;

    fn render_content(&self, content: &Content, context: &RenderContext) -> String;

    fn render_list(&self, contents: &[Content], context: &RenderContext) -> String;
}

#[async_trait]
pub trait ExportPlugin: CmsPlugin {
    fn supported_formats(&self) -> Vec<ExportFormat>;

    async fn export(&self, data: &[Content], format: &ExportFormat) -> Result<Vec<u8>, Error>;

    async fn import(&self, data: &[u8], format: &ExportFormat) -> Result<Vec<Content>, Error>;
}
