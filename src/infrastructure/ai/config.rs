use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub llm: LLMConfig,
    pub image: ImageConfig,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            llm: LLMConfig::default(),
            image: ImageConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub primary: ProviderConfig,
    #[serde(default)]
    pub fallback: Vec<ProviderConfig>,
    #[serde(default)]
    pub retry_attempts: u32,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_timeout_ms() -> u64 { 60000 }

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            primary: ProviderConfig::default_openai(),
            fallback: vec![],
            retry_attempts: 3,
            timeout_ms: default_timeout_ms(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub primary: ProviderConfig,
    #[serde(default)]
    pub fallback: Vec<ProviderConfig>,
    #[serde(default)]
    pub retry_attempts: u32,
    #[serde(default = "default_image_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub storage_path: Option<String>,
}

fn default_image_timeout_ms() -> u64 { 120000 }

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            primary: ProviderConfig::default_dalle(),
            fallback: vec![],
            retry_attempts: 3,
            timeout_ms: default_image_timeout_ms(),
            storage_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default = "default_max_tokens_config")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature_config")]
    pub temperature: f32,
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

fn default_max_tokens_config() -> u32 { 4096 }
fn default_temperature_config() -> f32 { 0.7 }

impl ProviderConfig {
    pub fn default_openai() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            api_key: String::new(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            max_tokens: 4096,
            temperature: 0.7,
            extra: HashMap::new(),
        }
    }

    pub fn default_dalle() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "dall-e-3".to_string(),
            api_key: String::new(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            max_tokens: 0,
            temperature: 0.0,
            extra: HashMap::new(),
        }
    }

    pub fn default_aliyun() -> Self {
        Self {
            provider: "aliyun".to_string(),
            model: "qwen-max".to_string(),
            api_key: String::new(),
            base_url: Some("https://dashscope.aliyuncs.com/api/v1".to_string()),
            max_tokens: 4096,
            temperature: 0.7,
            extra: HashMap::new(),
        }
    }

    pub fn default_stability() -> Self {
        Self {
            provider: "stability".to_string(),
            model: "stable-diffusion-xl-1024-v1-0".to_string(),
            api_key: String::new(),
            base_url: Some("https://api.stability.ai".to_string()),
            max_tokens: 0,
            temperature: 0.0,
            extra: HashMap::new(),
        }
    }

    pub fn is_openai(&self) -> bool {
        self.provider.to_lowercase() == "openai"
    }

    pub fn is_aliyun(&self) -> bool {
        matches!(self.provider.to_lowercase().as_str(), "aliyun" | "alibaba" | "qwen")
    }

    pub fn is_stability(&self) -> bool {
        matches!(self.provider.to_lowercase().as_str(), "stability" | "stabilityai")
    }

    pub fn is_azure(&self) -> bool {
        self.provider.to_lowercase() == "azure"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,
    #[serde(default = "default_recovery_timeout_ms")]
    pub recovery_timeout_ms: u64,
    #[serde(default = "default_half_open_max_calls")]
    pub half_open_max_calls: u32,
}

fn default_failure_threshold() -> u32 { 5 }
fn default_recovery_timeout_ms() -> u64 { 60000 }
fn default_half_open_max_calls() -> u32 { 3 }

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: default_failure_threshold(),
            recovery_timeout_ms: default_recovery_timeout_ms(),
            half_open_max_calls: default_half_open_max_calls(),
        }
    }
}
