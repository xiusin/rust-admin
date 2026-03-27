use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CodeGenItem {
    pub id: i64,
    pub model_id: i64,
    pub gen_type: i32,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CodeGenDetail {
    pub id: i64,
    pub model_id: i64,
    pub gen_type: i32,
    pub template_path: Option<String>,
    pub output_path: Option<String>,
    pub status: i32,
    pub config: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GeneratedFile {
    pub file_path: String,
    pub file_name: String,
    pub content: String,
    pub language: String,
}
