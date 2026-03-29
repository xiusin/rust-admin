use crate::service::prelude::*;
use crate::domain::entity::ppt_template_market;
use serde::{Deserialize, Serialize};

pub struct TemplatePreviewService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

impl ThumbnailSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Small => (200, 150),
            ThumbnailSize::Medium => (400, 300),
            ThumbnailSize::Large => (800, 600),
        }
    }
}

impl TemplatePreviewService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn generate_thumbnail(
        &self,
        template: &ppt_template_market::Model,
        size: ThumbnailSize,
    ) -> Result<String> {
        let (width, height) = size.dimensions();
        
        if let Some(ref thumbnail_url) = template.thumbnail_url {
            if !thumbnail_url.is_empty() {
                return Ok(thumbnail_url.clone());
            }
        }
        
        let preview_urls: Vec<String> = template.preview_urls
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        if let Some(first_preview) = preview_urls.first() {
            return Ok(first_preview.clone());
        }
        
        self.generate_default_thumbnail(template, width, height).await
    }
    
    pub async fn generate_preview_pages(
        &self,
        template: &ppt_template_market::Model,
        page_count: u32,
    ) -> Result<Vec<String>> {
        let preview_urls: Vec<String> = template.preview_urls
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        if !preview_urls.is_empty() {
            let count = page_count as usize;
            if preview_urls.len() >= count {
                return Ok(preview_urls.into_iter().take(count).collect());
            }
        }
        
        self.generate_default_preview_pages(template, page_count).await
    }
    
    pub async fn render_preview_html(
        &self,
        template: &ppt_template_market::Model,
    ) -> Result<String> {
        let style_params = template.style_params
            .as_ref()
            .and_then(|v| serde_json::from_value::<StylePreviewParams>(v.clone()).ok())
            .unwrap_or_default();
        
        let preview_urls: Vec<String> = template.preview_urls
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} - 模板预览</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: {font_family};
            background-color: {bg_color};
            color: {text_color};
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 20px;
        }}
        .template-header {{
            text-align: center;
            margin-bottom: 30px;
        }}
        .template-header h1 {{
            font-size: 32px;
            color: {primary_color};
            margin-bottom: 10px;
        }}
        .template-header p {{
            font-size: 16px;
            color: {text_secondary_color};
        }}
        .preview-container {{
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            justify-content: center;
            max-width: 1200px;
        }}
        .preview-page {{
            width: 400px;
            height: 300px;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
            background: white;
            position: relative;
        }}
        .preview-page img {{
            width: 100%;
            height: 100%;
            object-fit: cover;
        }}
        .preview-page .page-number {{
            position: absolute;
            bottom: 10px;
            right: 10px;
            background: rgba(0, 0, 0, 0.5);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
        }}
        .style-info {{
            margin-top: 40px;
            padding: 20px;
            background: white;
            border-radius: 8px;
            max-width: 800px;
            width: 100%;
        }}
        .style-info h2 {{
            color: {primary_color};
            margin-bottom: 15px;
        }}
        .style-info .info-item {{
            display: flex;
            margin-bottom: 10px;
        }}
        .style-info .info-label {{
            font-weight: bold;
            width: 120px;
            color: {text_secondary_color};
        }}
        .style-info .info-value {{
            flex: 1;
        }}
    </style>
</head>
<body>
    <div class="template-header">
        <h1>{name}</h1>
        <p>{description}</p>
    </div>
    
    <div class="preview-container">
        {preview_pages}
    </div>
    
    <div class="style-info">
        <h2>样式信息</h2>
        <div class="info-item">
            <span class="info-label">行业：</span>
            <span class="info-value">{industry}</span>
        </div>
        <div class="info-item">
            <span class="info-label">风格：</span>
            <span class="info-value">{style}</span>
        </div>
        <div class="info-item">
            <span class="info-label">下载量：</span>
            <span class="info-value">{downloads}</span>
        </div>
        <div class="info-item">
            <span class="info-label">评分：</span>
            <span class="info-value">{rating} / 5.0 ({rating_count} 人评价)</span>
        </div>
    </div>
</body>
</html>"#,
            name = template.name,
            description = template.description.as_deref().unwrap_or("暂无描述"),
            font_family = style_params.font_family.as_deref().unwrap_or("Arial, sans-serif"),
            bg_color = style_params.background_color.as_deref().unwrap_or("#f5f5f5"),
            text_color = style_params.text_color.as_deref().unwrap_or("#333333"),
            text_secondary_color = style_params.text_secondary_color.as_deref().unwrap_or("#666666"),
            primary_color = style_params.primary_color.as_deref().unwrap_or("#1890ff"),
            industry = template.industry.as_deref().unwrap_or("通用"),
            style = template.style.as_deref().unwrap_or("简约"),
            downloads = template.downloads,
            rating = format!("{:.1}", template.rating),
            rating_count = template.rating_count,
            preview_pages = self.generate_preview_pages_html(&preview_urls),
        );
        
        Ok(html)
    }
    
    fn generate_preview_pages_html(&self, preview_urls: &[String]) -> String {
        if preview_urls.is_empty() {
            return r#"<div class="preview-page">
                <div style="display: flex; align-items: center; justify-content: center; height: 100%; color: #999;">
                    暂无预览图
                </div>
            </div>"#.to_string();
        }
        
        preview_urls
            .iter()
            .enumerate()
            .map(|(i, url)| {
                format!(
                    r#"<div class="preview-page">
                <img src="{}" alt="预览图 {}">
                <span class="page-number">第 {} 页</span>
            </div>"#,
                    url,
                    i + 1,
                    i + 1
                )
            })
            .collect::<Vec<_>>()
            .join("\n        ")
    }
    
    async fn generate_default_thumbnail(
        &self,
        template: &ppt_template_market::Model,
        width: u32,
        height: u32,
    ) -> Result<String> {
        let style_params = template.style_params
            .as_ref()
            .and_then(|v| serde_json::from_value::<StylePreviewParams>(v.clone()).ok())
            .unwrap_or_default();
        
        let svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="{}"/>
    <text x="50%" y="50%" text-anchor="middle" dy=".3em" fill="{}" font-size="24" font-family="Arial">
        {}
    </text>
</svg>"#,
            width,
            height,
            style_params.background_color.as_deref().unwrap_or("#f5f5f5"),
            style_params.text_color.as_deref().unwrap_or("#333333"),
            template.name.chars().take(10).collect::<String>()
        );
        
        let server_config = APPCOFIG.server.clone();
        let now = chrono::Local::now();
        let file_path_t = format!(
            "{}/{}",
            server_config.static_dir.clone(),
            &now.format("%Y-%m")
        );
        
        tokio::fs::create_dir_all(&file_path_t).await?;
        
        let fid = GID().await;
        let file_name = format!("{}_{}{}", now.format("%d"), fid, ".svg");
        let file_path = format!("{}/{}", file_path_t, &file_name);
        
        tokio::fs::write(&file_path, svg).await?;
        
        let static_dir = if server_config.static_dir.starts_with("data/") {
            &server_config.static_dir["data/".len()..]
        } else {
            server_config.static_dir.as_str()
        };
        
        let url_path = format!(
            "{}/{}/{}/{}",
            server_config.domainname.clone(),
            static_dir,
            &now.format("%Y-%m"),
            &file_name
        );
        
        Ok(url_path)
    }
    
    async fn generate_default_preview_pages(
        &self,
        template: &ppt_template_market::Model,
        page_count: u32,
    ) -> Result<Vec<String>> {
        let mut urls = Vec::new();
        
        for _i in 0..page_count {
            let url = self.generate_default_thumbnail(template, 400, 300).await?;
            urls.push(url);
        }
        
        Ok(urls)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StylePreviewParams {
    primary_color: Option<String>,
    background_color: Option<String>,
    text_color: Option<String>,
    text_secondary_color: Option<String>,
    font_family: Option<String>,
}

impl Default for TemplatePreviewService {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn generate_thumbnail_handler(
    Path(id): Path<i64>,
    Query(size): Query<ThumbnailSizeQuery>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let service = TemplatePreviewService::new();
    let size = match size.size.as_deref() {
        Some("small") => ThumbnailSize::Small,
        Some("medium") => ThumbnailSize::Medium,
        Some("large") => ThumbnailSize::Large,
        _ => ThumbnailSize::Medium,
    };
    
    match service.generate_thumbnail(&template, size).await {
        Ok(url) => ApiResponse::ok(url),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn generate_preview_pages_handler(
    Path(id): Path<i64>,
    Query(params): Query<PreviewPagesQuery>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let service = TemplatePreviewService::new();
    let page_count = params.page_count.unwrap_or(5);
    
    match service.generate_preview_pages(&template, page_count).await {
        Ok(urls) => ApiResponse::ok(urls),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn render_preview_html_handler(
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let service = TemplatePreviewService::new();
    
    match service.render_preview_html(&template).await {
        Ok(html) => return Html(html).into_response(),
        Err(e) => return Html(format!("<html><body><h1>Error: {}</h1></body></html>", e)).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailSizeQuery {
    pub size: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewPagesQuery {
    pub page_count: Option<u32>,
}
