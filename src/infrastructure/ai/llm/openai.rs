use crate::common::error::{Error, Result};
use crate::infrastructure::ai::config::ProviderConfig;
use crate::infrastructure::ai::types::{GenerateOptions, LLMService, ChatMessage};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct OpenAILLM {
    config: ProviderConfig,
    client: Client,
}

impl OpenAILLM {
    pub fn new(config: ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(60000))
            .build()
            .map_err(|e| Error::internal_error(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self { config, client })
    }

    fn get_base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1")
    }

    async fn call_chat_api(&self, messages: Vec<ChatMessage>, options: &GenerateOptions) -> Result<OpenAIChatResponse> {
        let url = format!("{}/chat/completions", self.get_base_url());
        
        let model = options.model.as_ref().unwrap_or(&self.config.model);
        
        let request_body = OpenAIChatRequest {
            model: model.clone(),
            messages: messages.into_iter().map(|m| OpenAIMessage {
                role: m.role,
                content: m.content,
            }).collect(),
            max_tokens: Some(options.max_tokens),
            temperature: Some(options.temperature),
            top_p: Some(options.top_p),
            stop: if options.stop_sequences.is_empty() {
                None
            } else {
                Some(options.stop_sequences.clone())
            },
            stream: Some(false),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::internal_error(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::internal_error(format!(
                "OpenAI API error ({}): {}",
                status, error_text
            )));
        }

        let chat_response: OpenAIChatResponse = response
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Failed to parse OpenAI response: {}", e)))?;

        Ok(chat_response)
    }
}

#[async_trait]
impl LLMService for OpenAILLM {
    async fn generate(&self, prompt: &str, options: GenerateOptions) -> Result<String> {
        let messages = vec![ChatMessage::user(prompt)];
        let response = self.call_chat_api(messages, &options).await?;
        
        response.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| Error::internal_error("No response from OpenAI"))
    }

    async fn generate_with_system(&self, system_prompt: &str, user_prompt: &str, options: GenerateOptions) -> Result<String> {
        let messages = vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ];
        let response = self.call_chat_api(messages, &options).await?;
        
        response.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| Error::internal_error("No response from OpenAI"))
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/embeddings", self.get_base_url());
        
        let request_body = OpenAIEmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input: text.to_string(),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::internal_error(format!("OpenAI embedding request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::internal_error(format!(
                "OpenAI embedding API error ({}): {}",
                status, error_text
            )));
        }

        let embedding_response: OpenAIEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Failed to parse OpenAI embedding response: {}", e)))?;

        embedding_response.data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| Error::internal_error("No embedding returned from OpenAI"))
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}

#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIChatResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessageResponse,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIMessageResponse {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize)]
struct OpenAIEmbeddingRequest {
    model: String,
    input: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIEmbeddingResponse {
    object: String,
    data: Vec<OpenAIEmbeddingData>,
    model: String,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIEmbeddingData {
    object: String,
    embedding: Vec<f32>,
    index: u32,
}
