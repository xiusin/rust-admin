use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CodeGenPreviewReq {
    pub model_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CodeGenDownloadReq {
    pub model_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CodeGenApplyReq {
    pub model_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_path: Option<String>,
    #[serde(default)]
    pub overwrite: i32,
}
