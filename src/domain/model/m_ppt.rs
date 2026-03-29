use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PptProjectListItem {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub source_type: String,
    pub source_file: Option<String>,
    pub style_template_id: Option<i64>,
    pub style_template_name: Option<String>,
    pub industry: Option<String>,
    pub industry_confidence: Option<Decimal>,
    pub status: String,
    pub page_count: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PptProjectDetail {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub source_type: String,
    pub source_file: Option<String>,
    pub style_template_id: Option<i64>,
    pub style_template_name: Option<String>,
    pub industry: Option<String>,
    pub industry_confidence: Option<Decimal>,
    pub status: String,
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub pages: Vec<PptPageWithElements>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PptPageWithElements {
    pub id: i64,
    pub project_id: i64,
    pub page_order: i32,
    pub page_type: String,
    pub title: Option<String>,
    pub layout_config: Option<Json>,
    pub style_config: Option<Json>,
    pub elements: Vec<PageElementModel>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageElementModel {
    pub id: i64,
    pub page_id: i64,
    pub element_type: String,
    pub content: Option<Json>,
    pub style: Option<Json>,
    pub position: Option<Json>,
    pub size: Option<Json>,
    pub rotation: Option<Decimal>,
    pub z_index: i32,
    pub locked: i8,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PptPageListItem {
    pub id: i64,
    pub project_id: i64,
    pub page_order: i32,
    pub page_type: String,
    pub title: Option<String>,
    pub element_count: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StyleTemplateListItem {
    pub id: i64,
    pub name: String,
    pub industry: Option<String>,
    pub color_scheme: Option<Json>,
    pub font_scheme: Option<Json>,
    pub preview_url: Option<String>,
    pub usage_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StyleTemplateDetail {
    pub id: i64,
    pub name: String,
    pub industry: Option<String>,
    pub color_scheme: Option<Json>,
    pub font_scheme: Option<Json>,
    pub layout_rules: Option<Json>,
    pub component_styles: Option<Json>,
    pub preview_url: Option<String>,
    pub usage_count: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AiGenerationLogListItem {
    pub id: i64,
    pub project_id: Option<i64>,
    pub project_title: Option<String>,
    pub task_type: String,
    pub ai_provider: String,
    pub model: String,
    pub tokens_used: i32,
    pub cost: Option<Decimal>,
    pub duration_ms: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AiGenerationLogDetail {
    pub id: i64,
    pub project_id: Option<i64>,
    pub project_title: Option<String>,
    pub task_type: String,
    pub ai_provider: String,
    pub model: String,
    pub input_data: Option<Json>,
    pub output_data: Option<Json>,
    pub tokens_used: i32,
    pub cost: Option<Decimal>,
    pub duration_ms: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PptProjectStatistics {
    pub total_projects: i64,
    pub draft_count: i64,
    pub generating_count: i64,
    pub completed_count: i64,
}
