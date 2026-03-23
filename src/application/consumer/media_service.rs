use crate::common::error::Error;
use crate::domain::model::m_media::*;
use crate::domain::entity::prelude::CMediaFile;
use crate::domain::entity::c_media_file;
use crate::model::prelude::*;
use sea_orm::*;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

pub async fn generate_upload_url(params: GenerateUploadUrlParams) -> Result<UploadUrlResp> {
    let oss_key = format!(
        "media/{}/{}",
        params.consumer_id,
        generate_uuid()
    );

    let upload_url = format!("https://your-oss-bucket.oss.amazonaws.com/{}", oss_key);

    Ok(UploadUrlResp {
        oss_key: oss_key.clone(),
        upload_url,
        expires_in: 3600,
    })
}

pub async fn confirm_upload(params: ConfirmUploadParams) -> Result<MediaFileModel> {
    let db = DB_WRITE().await;
    let now = chrono::Local::now().naive_local();
    let id = generate_id();

    let file_name = params.file_name.clone();
    let oss_key = params.oss_key.clone();
    let file_url = format!("https://your-oss-bucket.oss.amazonaws.com/{}", oss_key);
    let file_type_str = params.file_type.to_string();

    let file = c_media_file::ActiveModel {
        id: Set(id),
        consumer_id: Set(params.consumer_id),
        file_name: Set(file_name.clone()),
        file_type: Set(file_type_str.clone()),
        file_size: Set(params.file_size),
        file_url: Set(Some(file_url.clone())),
        thumbnail_url: Set(None),
        oss_key: Set(Some(oss_key.clone())),
        oss_bucket: Set(Some("your-oss-bucket".to_string())),
        is_deleted: Set(false),
        created_at: Set(Some(now)),
        deleted_at: Set(None),
        ..Default::default()
    };

    CMediaFile::insert(file)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(MediaFileModel {
        id,
        consumer_id: params.consumer_id,
        file_name,
        file_type: file_type_str,
        file_format: None,
        file_size: params.file_size,
        file_url: Some(file_url),
        thumbnail_url: None,
        oss_bucket: Some("your-oss-bucket".to_string()),
        oss_key: Some(oss_key),
        is_deleted: false,
        created_at: Some(now),
        deleted_at: None,
    })
}

pub async fn list_files(
    params: MediaFileListParams,
) -> Result<(Vec<MediaFileModel>, u64)> {
    let db = DB_WRITE().await;

    let mut conditions = Condition::all();
    conditions = conditions.add(c_media_file::Column::IsDeleted.eq(false));

    if let Some(file_type) = &params.file_type {
        conditions = conditions.add(c_media_file::Column::FileType.eq(file_type.clone()));
    }
    if let Some(consumer_id) = params.consumer_id {
        conditions = conditions.add(c_media_file::Column::ConsumerId.eq(consumer_id));
    }

    let paginator = CMediaFile::find()
        .filter(conditions)
        .order_by_desc(c_media_file::Column::CreatedAt)
        .paginate(db, 10);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let files = paginator
        .fetch_page(0)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<MediaFileModel> = files
        .into_iter()
        .map(|f| MediaFileModel {
            id: f.id,
            consumer_id: f.consumer_id,
            file_name: f.file_name,
            file_type: f.file_type,
            file_format: f.file_format,
            file_size: f.file_size,
            file_url: f.file_url,
            thumbnail_url: f.thumbnail_url,
            oss_bucket: f.oss_bucket,
            oss_key: f.oss_key,
            is_deleted: f.is_deleted,
            created_at: f.created_at,
            deleted_at: f.deleted_at,
        })
        .collect();

    Ok((items, total))
}