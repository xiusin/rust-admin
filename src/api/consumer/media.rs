use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::media_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::model::prelude::ListData;
use crate::domain::args::a_media::*;
use crate::domain::model::m_media::*;
use axum::{extract::Query, Json};
use validator::Validate;

pub async fn generate_upload_url(
    Json(args): Json<GenerateUploadUrlArgs>,
) -> Result<Json<ApiResponse<UploadUrlResp>>, Error> {
    args.validate()?;

    let result = media_service::generate_upload_url(GenerateUploadUrlParams {
        file_name: args.file_name,
        file_type: match args.file_type.as_str() {
            "video" => FileType::Video,
            _ => FileType::Image,
        },
        consumer_id: args.consumer_id,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn confirm_upload(
    Json(args): Json<ConfirmUploadArgs>,
) -> Result<Json<ApiResponse<MediaFileModel>>, Error> {
    args.validate()?;

    let result = media_service::confirm_upload(ConfirmUploadParams {
        oss_key: args.oss_key,
        file_name: args.file_name,
        file_type: match args.file_type.as_str() {
            "video" => FileType::Video,
            _ => FileType::Image,
        },
        file_size: args.file_size,
        consumer_id: args.consumer_id,
        mime_type: None,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn list_files(
    Query(params): Query<MediaListArgs>,
) -> Result<Json<ApiResponse<ListData<MediaFileModel>>>, Error> {
    let (items, total) = media_service::list_files(
        MediaFileListParams {
            consumer_id: params.consumer_id,
            file_type: params.file_type,
            file_name: None,
            start_time: params.start_time,
            end_time: params.end_time,
        },
    )
    .await?;

    let page_size = params.page_size.unwrap_or(10) as u64;
    let page_num = params.page_num.unwrap_or(1) as u64;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })))
}

pub fn media_api() -> WebPath {
    WebPath::new()
        .route("/upload-url", WebPathType::Post, Some("生成上传URL"), post(generate_upload_url))
        .route("/confirm", WebPathType::Post, Some("确认上传"), post(confirm_upload))
        .route("/list", WebPathType::Get, Some("媒体文件列表"), get(list_files))
}