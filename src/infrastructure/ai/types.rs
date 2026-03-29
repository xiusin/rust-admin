use crate::common::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait LLMService: Send + Sync {
    async fn generate(&self, prompt: &str, options: GenerateOptions) -> Result<String>;
    async fn generate_with_system(&self, system_prompt: &str, user_prompt: &str, options: GenerateOptions) -> Result<String>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn provider_name(&self) -> &str;
}

#[async_trait]
pub trait ImageService: Send + Sync {
    async fn generate_image(&self, prompt: &str, options: ImageOptions) -> Result<GeneratedImage>;
    async fn edit_image(&self, image: &[u8], prompt: &str, options: ImageOptions) -> Result<GeneratedImage>;
    fn provider_name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOptions {
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default)]
    pub stop_sequences: Vec<String>,
    #[serde(default)]
    pub model: Option<String>,
}

fn default_max_tokens() -> u32 { 2048 }
fn default_temperature() -> f32 { 0.7 }
fn default_top_p() -> f32 { 1.0 }

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            top_p: default_top_p(),
            stop_sequences: vec![],
            model: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageOptions {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    pub style: Option<String>,
    pub negative_prompt: Option<String>,
    #[serde(default = "default_steps")]
    pub steps: u32,
    #[serde(default = "default_cfg_scale")]
    pub cfg_scale: f32,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub model: Option<String>,
}

fn default_width() -> u32 { 1024 }
fn default_height() -> u32 { 1024 }
fn default_steps() -> u32 { 30 }
fn default_cfg_scale() -> f32 { 7.0 }

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            style: None,
            negative_prompt: None,
            steps: default_steps(),
            cfg_scale: default_cfg_scale(),
            seed: None,
            model: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub url: Option<String>,
    pub base64: Option<String>,
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
    pub content_type: Option<String>,
}

impl GeneratedImage {
    pub fn from_url(url: String, width: u32, height: u32) -> Self {
        Self {
            url: Some(url),
            base64: None,
            width,
            height,
            seed: None,
            content_type: Some("image/png".to_string()),
        }
    }

    pub fn from_base64(base64: String, width: u32, height: u32, content_type: Option<String>) -> Self {
        Self {
            url: None,
            base64: Some(base64),
            width,
            height,
            seed: None,
            content_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResult {
    pub embedding: Vec<f32>,
    pub tokens_used: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AIProvider {
    OpenAI,
    Azure,
    Aliyun,
    Baidu,
    Stability,
    Custom(String),
}

impl std::fmt::Display for AIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIProvider::OpenAI => write!(f, "openai"),
            AIProvider::Azure => write!(f, "azure"),
            AIProvider::Aliyun => write!(f, "aliyun"),
            AIProvider::Baidu => write!(f, "baidu"),
            AIProvider::Stability => write!(f, "stability"),
            AIProvider::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::str::FromStr for AIProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AIProvider::OpenAI),
            "azure" => Ok(AIProvider::Azure),
            "aliyun" | "alibaba" | "qwen" => Ok(AIProvider::Aliyun),
            "baidu" => Ok(AIProvider::Baidu),
            "stability" | "stabilityai" => Ok(AIProvider::Stability),
            other => Ok(AIProvider::Custom(other.to_string())),
        }
    }
}
