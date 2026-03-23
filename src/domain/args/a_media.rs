use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct GenerateUploadUrlArgs {
    pub consumer_id: i64,
    #[validate(length(min = 1, message = "文件名不能为空"))]
    pub file_name: String,
    #[validate(length(min = 1, message = "文件类型不能为空"))]
    pub file_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConfirmUploadArgs {
    pub consumer_id: i64,
    #[validate(length(min = 1, message = "OSS Key不能为空"))]
    pub oss_key: String,
    #[validate(length(min = 1, message = "文件名不能为空"))]
    pub file_name: String,
    #[validate(length(min = 1, message = "文件类型不能为空"))]
    pub file_type: String,
    pub file_size: i64,
    pub thumbnail_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub file_type: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}