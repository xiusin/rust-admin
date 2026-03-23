use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Image,
    Video,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Image => write!(f, "image"),
            FileType::Video => write!(f, "video"),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MediaFileModel {
    pub id: i64,
    pub consumer_id: i64,
    pub file_name: String,
    pub file_type: String,
    pub file_format: Option<String>,
    pub file_size: i64,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub oss_bucket: Option<String>,
    pub oss_key: Option<String>,
    pub is_deleted: bool,
    pub created_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenerateUploadUrlParams {
    pub file_name: String,
    pub file_type: FileType,
    pub consumer_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfirmUploadParams {
    pub oss_key: String,
    pub file_name: String,
    pub file_type: FileType,
    pub file_size: i64,
    pub consumer_id: i64,
    pub mime_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaFileListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UploadUrlResp {
    pub oss_key: String,
    pub upload_url: String,
    pub expires_in: i64,
}