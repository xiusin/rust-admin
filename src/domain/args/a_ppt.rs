use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptProjectAddArgs {
    pub user_id: i64,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 20, message = "来源类型长度1-20"))]
    pub source_type: String,
    pub source_file: Option<String>,
    pub style_template_id: Option<i64>,
    pub industry: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptProjectEditArgs {
    pub id: i64,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    pub description: Option<String>,
    pub source_type: Option<String>,
    pub source_file: Option<String>,
    pub style_template_id: Option<i64>,
    pub industry: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptProjectListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub source_type: Option<String>,
    pub status: Option<String>,
    pub industry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PptProjectDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptProjectStatusArgs {
    pub ids: Vec<i64>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptPageAddArgs {
    pub project_id: i64,
    pub page_order: i32,
    #[validate(length(min = 1, max = 20, message = "页面类型长度1-20"))]
    pub page_type: String,
    pub title: Option<String>,
    pub layout_config: Option<serde_json::Value>,
    pub style_config: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PptPageEditArgs {
    pub id: i64,
    pub page_order: Option<i32>,
    pub page_type: Option<String>,
    pub title: Option<String>,
    pub layout_config: Option<serde_json::Value>,
    pub style_config: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PptPageListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub project_id: Option<i64>,
    pub page_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PptPageDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PageElementAddArgs {
    pub page_id: i64,
    #[validate(length(min = 1, max = 20, message = "元素类型长度1-20"))]
    pub element_type: String,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
    pub rotation: Option<rust_decimal::Decimal>,
    pub z_index: Option<i32>,
    pub locked: Option<i8>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PageElementEditArgs {
    pub id: i64,
    pub element_type: Option<String>,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
    pub rotation: Option<rust_decimal::Decimal>,
    pub z_index: Option<i32>,
    pub locked: Option<i8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageElementListArgs {
    pub page_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageElementDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StyleTemplateAddArgs {
    #[validate(length(min = 1, max = 100, message = "模板名称长度1-100"))]
    pub name: String,
    pub industry: Option<String>,
    pub color_scheme: Option<serde_json::Value>,
    pub font_scheme: Option<serde_json::Value>,
    pub layout_rules: Option<serde_json::Value>,
    pub component_styles: Option<serde_json::Value>,
    pub preview_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct StyleTemplateEditArgs {
    pub id: i64,
    pub name: Option<String>,
    pub industry: Option<String>,
    pub color_scheme: Option<serde_json::Value>,
    pub font_scheme: Option<serde_json::Value>,
    pub layout_rules: Option<serde_json::Value>,
    pub component_styles: Option<serde_json::Value>,
    pub preview_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StyleTemplateListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub industry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StyleTemplateDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AiGenerationLogListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub project_id: Option<i64>,
    pub task_type: Option<String>,
    pub ai_provider: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AiGenerationLogDeleteArgs {
    pub ids: Vec<i64>,
}
