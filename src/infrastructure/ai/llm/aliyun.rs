use crate::common::error::{Error, Result};
use crate::infrastructure::ai::config::ProviderConfig;
use crate::infrastructure::ai::types::{GenerateOptions, LLMService, ChatMessage};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct AliyunLLM {
    config: ProviderConfig,
    client: Client,
}

impl AliyunLLM {
    pub fn new(config: ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(60000))
            .build()
            .map_err(|e| Error::internal_error(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self { config, client })
    }

    fn get_base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or("https://dashscope.aliyuncs.com/api/v1")
    }

    async fn call_chat_api(&self, messages: Vec<ChatMessage>, options: &GenerateOptions) -> Result<AliyunChatResponse> {
        let url = format!("{}/services/aigc/text-generation/generation", self.get_base_url());
        
        let model = options.model.as_ref().unwrap_or(&self.config.model);
        
        let request_body = AliyunChatRequest {
            model: model.clone(),
            input: AliyunInput {
                messages: messages.into_iter().map(|m| AliyunMessage {
                    role: m.role,
                    content: m.content,
                }).collect(),
            },
            parameters: Some(AliyunParameters {
                max_tokens: Some(options.max_tokens),
                temperature: Some(options.temperature),
                top_p: Some(options.top_p),
                stop: if options.stop_sequences.is_empty() {
                    None
                } else {
                    Some(options.stop_sequences.clone())
                },
            }),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::internal_error(format!("Aliyun API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::internal_error(format!(
                "Aliyun API error ({}): {}",
                status, error_text
            )));
        }

        let chat_response: AliyunChatResponse = response
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Failed to parse Aliyun response: {}", e)))?;

        Ok(chat_response)
    }
}

#[async_trait]
impl LLMService for AliyunLLM {
    async fn generate(&self, prompt: &str, options: GenerateOptions) -> Result<String> {
        let messages = vec![ChatMessage::user(prompt)];
        let response = self.call_chat_api(messages, &options).await?;
        
        response.output
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| Error::internal_error("No response from Aliyun"))
    }

    async fn generate_with_system(&self, system_prompt: &str, user_prompt: &str, options: GenerateOptions) -> Result<String> {
        let messages = vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ];
        let response = self.call_chat_api(messages, &options).await?;
        
        response.output
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| Error::internal_error("No response from Aliyun"))
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/services/embeddings/text-embedding/text-embedding", self.get_base_url());
        
        let request_body = AliyunEmbeddingRequest {
            model: "text-embedding-v2".to_string(),
            input: AliyunEmbeddingInput {
                texts: vec![text.to_string()],
            },
            parameters: Some(AliyunEmbeddingParameters {
                text_type: "query".to_string(),
            }),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::internal_error(format!("Aliyun embedding request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::internal_error(format!(
                "Aliyun embedding API error ({}): {}",
                status, error_text
            )));
        }

        let embedding_response: AliyunEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Failed to parse Aliyun embedding response: {}", e)))?;

        embedding_response.output
            .embeddings
            .first()
            .map(|e| e.embedding.clone())
            .ok_or_else(|| Error::internal_error("No embedding returned from Aliyun"))
    }

    fn provider_name(&self) -> &str {
        "aliyun"
    }
}

#[derive(Debug, Serialize)]
struct AliyunChatRequest {
    model: String,
    input: AliyunInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<AliyunParameters>,
}

#[derive(Debug, Serialize)]
struct AliyunInput {
    messages: Vec<AliyunMessage>,
}

#[derive(Debug, Serialize)]
struct AliyunMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct AliyunParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunChatResponse {
    output: AliyunOutput,
    usage: Option<AliyunUsage>,
    request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunOutput {
    choices: Vec<AliyunChoice>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunChoice {
    finish_reason: Option<String>,
    message: AliyunMessageResponse,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunMessageResponse {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunUsage {
    total_tokens: u32,
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Serialize)]
struct AliyunEmbeddingRequest {
    model: String,
    input: AliyunEmbeddingInput,
    parameters: Option<AliyunEmbeddingParameters>,
}

#[derive(Debug, Serialize)]
struct AliyunEmbeddingInput {
    texts: Vec<String>,
}

#[derive(Debug, Serialize)]
struct AliyunEmbeddingParameters {
    text_type: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunEmbeddingResponse {
    output: AliyunEmbeddingOutput,
    usage: Option<AliyunUsage>,
    request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AliyunEmbeddingOutput {
    embeddings: Vec<AliyunEmbeddingData>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AliyunEmbeddingData {
    text_index: u32,
    embedding: Vec<f32>,
}
