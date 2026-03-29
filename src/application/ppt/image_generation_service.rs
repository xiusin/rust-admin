use crate::common::error::{Error, Result};
use crate::infrastructure::ai::types::{ImageOptions, GeneratedImage};
use crate::infrastructure::ai::config::{AIConfig, ProviderConfig, CircuitBreakerConfig};
use crate::infrastructure::ai::circuit_breaker::ImageServicePool;
use crate::infrastructure::ai::image::{StabilityAI, DalleService};
use crate::infrastructure::ai::ImageService;
use std::sync::Arc;
use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};

pub struct ImageGenerationService {
    service_pool: Arc<ImageServicePool>,
    storage_path: PathBuf,
}

impl ImageGenerationService {
    pub fn new(config: &AIConfig) -> Result<Self> {
        let storage_path = config.image.storage_path
            .as_ref()
            .map(|p| PathBuf::from(p))
            .unwrap_or_else(|| PathBuf::from("data/ai_images"));
        
        let primary = create_image_service(&config.image.primary)?;
        let fallback: Vec<Box<dyn ImageService>> = config.image.fallback
            .iter()
            .filter_map(|c| create_image_service(c).ok())
            .collect();
        
        let circuit_config = CircuitBreakerConfig::default();
        let service_pool = Arc::new(ImageServicePool::new(primary, fallback, circuit_config));
        
        Ok(Self {
            service_pool,
            storage_path,
        })
    }

    pub async fn generate_background(
        &self,
        style: &StyleParams,
        page_type: PageType,
    ) -> Result<String> {
        let prompt = self.build_background_prompt(style, page_type);
        
        let options = ImageOptions {
            width: 1920,
            height: 1080,
            style: style.style_preset.clone(),
            negative_prompt: style.negative_prompt.clone(),
            steps: 30,
            cfg_scale: 7.0,
            seed: None,
            model: None,
        };
        
        let image = self.service_pool.generate_image(&prompt, options).await?;
        self.save_image(image).await
    }

    pub async fn generate_style_consistent_images(
        &self,
        style: &StyleParams,
        prompts: Vec<String>,
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();
        
        for prompt in prompts {
            let full_prompt = format!(
                "{}, {}, {}",
                prompt,
                style.style_description.as_deref().unwrap_or(""),
                style.color_palette.as_deref().unwrap_or("")
            );
            
            let options = ImageOptions {
                width: 1024,
                height: 1024,
                style: style.style_preset.clone(),
                negative_prompt: style.negative_prompt.clone(),
                steps: 30,
                cfg_scale: 7.0,
                seed: style.seed,
                model: None,
            };
            
            let image = self.service_pool.generate_image(&full_prompt, options).await?;
            let url = self.save_image(image).await?;
            results.push(url);
        }
        
        Ok(results)
    }

    pub async fn generate_icon(
        &self,
        description: &str,
        style: &StyleParams,
    ) -> Result<String> {
        let prompt = format!(
            "Simple icon, {}, minimalist design, {}",
            description,
            style.color_palette.as_deref().unwrap_or("professional colors")
        );
        
        let options = ImageOptions {
            width: 256,
            height: 256,
            style: Some("minimalist".to_string()),
            negative_prompt: Some("complex, detailed, text, watermark".to_string()),
            steps: 20,
            cfg_scale: 8.0,
            seed: None,
            model: None,
        };
        
        let image = self.service_pool.generate_image(&prompt, options).await?;
        self.save_image(image).await
    }

    pub async fn generate_illustration(
        &self,
        description: &str,
        style: &StyleParams,
        width: u32,
        height: u32,
    ) -> Result<String> {
        let prompt = format!(
            "{}, {}, {}",
            description,
            style.style_description.as_deref().unwrap_or(""),
            style.color_palette.as_deref().unwrap_or("")
        );
        
        let options = ImageOptions {
            width,
            height,
            style: style.style_preset.clone(),
            negative_prompt: style.negative_prompt.clone(),
            steps: 30,
            cfg_scale: 7.0,
            seed: style.seed,
            model: None,
        };
        
        let image = self.service_pool.generate_image(&prompt, options).await?;
        self.save_image(image).await
    }

    fn build_background_prompt(&self, style: &StyleParams, page_type: PageType) -> String {
        let page_description = match page_type {
            PageType::Cover => "title slide background, professional presentation cover",
            PageType::Content => "content slide background, clean and minimal",
            PageType::Section => "section divider background, impactful design",
            PageType::Conclusion => "conclusion slide background, summary design",
            PageType::Custom => "presentation slide background",
        };
        
        format!(
            "{}, {}, {}, {}",
            page_description,
            style.style_description.as_deref().unwrap_or(""),
            style.color_palette.as_deref().unwrap_or(""),
            style.mood.as_deref().unwrap_or("professional")
        )
    }

    async fn save_image(&self, image: GeneratedImage) -> Result<String> {
        fs::create_dir_all(&self.storage_path).await
            .map_err(|e| Error::internal_error(format!("Failed to create image directory: {}", e)))?;
        
        let filename = format!("{}.png", uuid::Uuid::new_v4());
        let file_path = self.storage_path.join(&filename);
        
        if let Some(base64) = &image.base64 {
            let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, base64)
                .map_err(|e| Error::internal_error(format!("Failed to decode base64 image: {}", e)))?;
            
            fs::write(&file_path, &bytes).await
                .map_err(|e| Error::internal_error(format!("Failed to write image file: {}", e)))?;
        } else if let Some(url) = &image.url {
            let response = reqwest::get(url).await
                .map_err(|e| Error::internal_error(format!("Failed to download image: {}", e)))?;
            
            let bytes = response.bytes().await
                .map_err(|e| Error::internal_error(format!("Failed to read image bytes: {}", e)))?;
            
            fs::write(&file_path, &bytes).await
                .map_err(|e| Error::internal_error(format!("Failed to write image file: {}", e)))?;
        } else {
            return Err(Error::internal_error("No image data available"));
        }
        
        Ok(format!("/api/static/ai_images/{}", filename))
    }
}

fn create_image_service(config: &ProviderConfig) -> Result<Box<dyn ImageService>> {
    if config.is_stability() {
        Ok(Box::new(StabilityAI::new(config.clone())?))
    } else if config.is_openai() {
        Ok(Box::new(DalleService::new(config.clone())?))
    } else {
        Err(Error::internal_error(format!("Unsupported image provider: {}", config.provider)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleParams {
    pub style_description: Option<String>,
    pub color_palette: Option<String>,
    pub style_preset: Option<String>,
    pub negative_prompt: Option<String>,
    pub mood: Option<String>,
    pub seed: Option<u64>,
}

impl Default for StyleParams {
    fn default() -> Self {
        Self {
            style_description: Some("modern corporate presentation style".to_string()),
            color_palette: Some("professional blue and white colors".to_string()),
            style_preset: Some("professional".to_string()),
            negative_prompt: Some("blurry, low quality, distorted, text, watermark".to_string()),
            mood: Some("professional".to_string()),
            seed: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageType {
    Cover,
    Content,
    Section,
    Conclusion,
    Custom,
}

impl std::fmt::Display for PageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageType::Cover => write!(f, "cover"),
            PageType::Content => write!(f, "content"),
            PageType::Section => write!(f, "section"),
            PageType::Conclusion => write!(f, "conclusion"),
            PageType::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for PageType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cover" | "title" => Ok(PageType::Cover),
            "content" => Ok(PageType::Content),
            "section" | "divider" => Ok(PageType::Section),
            "conclusion" | "summary" => Ok(PageType::Conclusion),
            "custom" => Ok(PageType::Custom),
            _ => Err(format!("Unknown page type: {}", s)),
        }
    }
}
