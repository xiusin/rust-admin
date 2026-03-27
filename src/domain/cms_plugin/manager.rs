use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::traits::*;
use super::types::*;
use crate::common::error::Error;

pub struct PluginManager {
    plugins: RwLock<HashMap<String, Arc<dyn CmsPlugin>>>,
    field_type_plugins: RwLock<HashMap<String, Arc<dyn FieldTypePlugin>>>,
    widget_plugins: RwLock<HashMap<String, Arc<dyn WidgetPlugin>>>,
    template_plugins: RwLock<HashMap<String, Arc<dyn TemplatePlugin>>>,
    export_plugins: RwLock<HashMap<String, Arc<dyn ExportPlugin>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            field_type_plugins: RwLock::new(HashMap::new()),
            widget_plugins: RwLock::new(HashMap::new()),
            template_plugins: RwLock::new(HashMap::new()),
            export_plugins: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, plugin: Arc<dyn CmsPlugin>) -> Result<(), Error> {
        let id = plugin.id().to_string();
        let plugin_type = plugin.plugin_type();

        match plugin_type {
            PluginType::FieldType => {
                if let Some(field_plugin) = plugin.as_any().downcast_ref::<Arc<dyn FieldTypePlugin>>() {
                    let mut plugins = self.field_type_plugins.write();
                    plugins.insert(field_plugin.field_type().to_string(), field_plugin.clone());
                }
            }
            PluginType::Widget => {
                if let Some(widget_plugin) = plugin.as_any().downcast_ref::<Arc<dyn WidgetPlugin>>() {
                    let mut plugins = self.widget_plugins.write();
                    plugins.insert(widget_plugin.widget_name().to_string(), widget_plugin.clone());
                }
            }
            PluginType::Template => {
                if let Some(template_plugin) = plugin.as_any().downcast_ref::<Arc<dyn TemplatePlugin>>() {
                    let mut plugins = self.template_plugins.write();
                    plugins.insert(template_plugin.template_name().to_string(), template_plugin.clone());
                }
            }
            PluginType::Export => {
                if let Some(export_plugin) = plugin.as_any().downcast_ref::<Arc<dyn ExportPlugin>>() {
                    let mut plugins = self.export_plugins.write();
                    plugins.insert(export_plugin.id().to_string(), export_plugin.clone());
                }
            }
            PluginType::Extension => {}
        }

        let mut plugins = self.plugins.write();
        plugins.insert(id, plugin);

        Ok(())
    }

    pub fn get_field_type_plugin(&self, field_type: &str) -> Option<Arc<dyn FieldTypePlugin>> {
        let plugins = self.field_type_plugins.read();
        plugins.get(field_type).cloned()
    }

    pub fn get_all_field_types(&self) -> Vec<String> {
        let plugins = self.field_type_plugins.read();
        plugins.keys().cloned().collect()
    }

    pub fn get_widget_plugin(&self, widget_name: &str) -> Option<Arc<dyn WidgetPlugin>> {
        let plugins = self.widget_plugins.read();
        plugins.get(widget_name).cloned()
    }

    pub fn get_template_plugin(&self, template_name: &str) -> Option<Arc<dyn TemplatePlugin>> {
        let plugins = self.template_plugins.read();
        plugins.get(template_name).cloned()
    }

    pub fn get_export_plugin(&self, plugin_id: &str) -> Option<Arc<dyn ExportPlugin>> {
        let plugins = self.export_plugins.read();
        plugins.get(plugin_id).cloned()
    }

    pub fn get_all_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.read();
        plugins
            .values()
            .map(|p| PluginInfo {
                id: p.id().to_string(),
                name: p.name().to_string(),
                version: p.version().to_string(),
                description: p.description().to_string(),
                plugin_type: p.plugin_type(),
                status: PluginStatus::Installed,
                is_paid: p.is_paid(),
                price: p.price(),
                author: None,
                homepage: None,
                repository: None,
                license: None,
                created_at: None,
                updated_at: None,
            })
            .collect()
    }

    pub async fn enable_plugin(&self, plugin_id: &str) -> Result<(), Error> {
        let plugins = self.plugins.read();
        if let Some(plugin) = plugins.get(plugin_id) {
            plugin.enable().await?;
        }
        Ok(())
    }

    pub async fn disable_plugin(&self, plugin_id: &str) -> Result<(), Error> {
        let plugins = self.plugins.read();
        if let Some(plugin) = plugins.get(plugin_id) {
            plugin.disable().await?;
        }
        Ok(())
    }

    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<(), Error> {
        let plugins = self.plugins.read();
        if let Some(plugin) = plugins.get(plugin_id) {
            plugin.uninstall().await?;
        }

        let mut plugins = self.plugins.write();
        plugins.remove(plugin_id);

        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    pub static ref PLUGIN_MANAGER: PluginManager = PluginManager::new();
}

pub fn plugin_manager() -> &'static PluginManager {
    &PLUGIN_MANAGER
}
