use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ParsedDocument {
    pub title: String,
    pub language: String,
    pub industry_hint: Option<String>,
    pub sections: Vec<Section>,
    pub images: Vec<ExtractedImage>,
    pub metadata: DocumentMetadata,
    pub style_hints: StyleHints,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Section {
    pub level: u8,
    pub title: String,
    pub content: Vec<ContentBlock>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(TextBlock),
    Image(ImageBlock),
    Table(TableBlock),
    List(ListBlock),
    Code(CodeBlock),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TextBlock {
    pub text: String,
    pub style: Option<TextStyle>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TextStyle {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ImageBlock {
    pub data: Option<String>,
    pub url: Option<String>,
    pub alt_text: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TableBlock {
    pub rows: Vec<TableRow>,
    pub headers: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TableRow {
    pub cells: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ListBlock {
    pub items: Vec<ListItem>,
    pub ordered: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ListItem {
    pub text: String,
    pub level: u8,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CodeBlock {
    pub code: String,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ExtractedImage {
    pub data: String,
    pub format: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub original_name: Option<String>,
    pub index: usize,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DocumentMetadata {
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub page_count: Option<u32>,
    pub word_count: Option<u32>,
    pub file_size: Option<u64>,
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StyleHints {
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub font_family: Option<String>,
    pub heading_style: Option<String>,
    pub layout_style: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Slide {
    pub slide_number: u32,
    pub title: Option<String>,
    pub content: Vec<ContentBlock>,
    pub notes: Option<String>,
    pub layout: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ParseProgress {
    pub task_id: String,
    pub status: String,
    pub progress: u8,
    pub message: Option<String>,
    pub result: Option<ParsedDocument>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DocumentUploadResult {
    pub file_id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: u64,
    pub upload_path: String,
}
