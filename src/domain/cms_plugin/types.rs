use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginType {
    FieldType,
    Widget,
    Template,
    Export,
    Extension,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TemplateType {
    List,
    Detail,
    Form,
    Email,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExportFormat {
    Excel,
    Csv,
    Json,
    Xml,
    Pdf,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginStatus {
    Installed,
    Enabled,
    Disabled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub status: PluginStatus,
    pub is_paid: bool,
    pub price: Option<f64>,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderContext {
    pub site_name: String,
    pub site_url: String,
    pub current_user: Option<String>,
    pub request_path: String,
    pub query_params: serde_json::Value,
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldConfig {
    pub placeholder: Option<String>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub default_value: Option<String>,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFieldConfig {
    pub width: Option<u32>,
    pub align: Option<String>,
    pub sortable: bool,
    pub filterable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub required: bool,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub pattern: Option<String>,
    pub custom: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub fields: serde_json::Value,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
