use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentUploadArgs {
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentParseArgs {
    pub file_path: String,
    pub file_type: Option<String>,
    pub extract_images: Option<bool>,
    pub detect_language: Option<bool>,
    pub detect_industry: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentParseFromBytesArgs {
    pub file_name: String,
    pub file_type: Option<String>,
    pub extract_images: Option<bool>,
    pub detect_language: Option<bool>,
    pub detect_industry: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParseProgressQueryArgs {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct DocumentConvertArgs {
    pub source_file: String,
    pub target_format: String,
    pub style_template_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocType {
    pub extension: String,
    pub mime_type: String,
    pub category: String,
}
