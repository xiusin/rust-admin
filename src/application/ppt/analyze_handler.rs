use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::application::ppt::industry_classifier::IndustryClassifier;
use crate::application::ppt::text_processor::TextProcessor;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AnalyzeIndustryRequest {
    #[validate(length(min = 10, message = "内容长度至少10个字符"))]
    pub content: String,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IndustryAnalysisResponse {
    pub industry: String,
    pub confidence: f32,
    pub sub_industries: Vec<IndustryMatch>,
    pub keywords: Vec<String>,
    pub style_suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct IndustryMatch {
    pub industry: String,
    pub confidence: f32,
}

pub async fn analyze_industry(VJson(req): VJson<AnalyzeIndustryRequest>) -> axum::response::Response {
    if let Err(e) = req.validate() {
        return ApiResponse::bad_request(e.to_string());
    }
    
    let classifier = IndustryClassifier::new();
    let result = match classifier.classify(&req.content).await {
        Ok(r) => r,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let processor = TextProcessor::new();
    let processed = match processor.process(&req.content) {
        Ok(p) => p,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let keywords: Vec<String> = processed.keywords.iter().take(10).map(|k| k.keyword.clone()).collect();
    
    let response = IndustryAnalysisResponse {
        industry: result.industry,
        confidence: result.confidence,
        sub_industries: result.alternatives.iter().map(|a| IndustryMatch {
            industry: a.industry.clone(),
            confidence: a.confidence,
        }).collect(),
        keywords,
        style_suggestions: vec![
            "现代简约风格".to_string(),
            "专业商务风格".to_string(),
        ],
    };
    
    ApiResponse::success(response).into_response()
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct ExtractKeywordsRequest {
    pub content: String,
    #[serde(default = "default_top_n")]
    pub top_n: usize,
}

fn default_top_n() -> usize { 10 }

#[derive(Debug, Serialize)]
pub struct KeywordsResponse {
    pub keywords: Vec<KeywordItem>,
}

#[derive(Debug, Serialize)]
pub struct KeywordItem {
    pub keyword: String,
    pub score: f32,
}

pub async fn extract_keywords(VJson(req): VJson<ExtractKeywordsRequest>) -> axum::response::Response {
    let processor = TextProcessor::new();
    let processed = match processor.process(&req.content) {
        Ok(p) => p,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let items: Vec<KeywordItem> = processed.keywords.iter()
        .take(req.top_n)
        .map(|k| KeywordItem {
            keyword: k.keyword.clone(),
            score: k.score as f32,
        })
        .collect();
    
    ApiResponse::success(KeywordsResponse { keywords: items }).into_response()
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct AnalyzeStructureRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct StructureResponse {
    pub sections: Vec<SectionInfo>,
    pub total_sections: usize,
    pub heading_count: usize,
    pub paragraph_count: usize,
}

#[derive(Debug, Serialize)]
pub struct SectionInfo {
    pub level: u8,
    pub title: String,
    pub word_count: usize,
}

pub async fn analyze_structure(VJson(_req): VJson<AnalyzeStructureRequest>) -> impl axum::response::IntoResponse {
    let response = StructureResponse {
        sections: vec![],
        total_sections: 0,
        heading_count: 0,
        paragraph_count: 0,
    };
    
    ApiResponse::success(response)
}

#[derive(Debug, Serialize)]
pub struct SentimentResponse {
    pub sentiment: String,
    pub score: f32,
    pub emotions: Vec<EmotionScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionScore {
    pub emotion: String,
    pub score: f32,
}

pub async fn analyze_sentiment(VJson(_req): VJson<AnalyzeStructureRequest>) -> impl axum::response::IntoResponse {
    let response = SentimentResponse {
        sentiment: "neutral".to_string(),
        score: 0.5,
        emotions: vec![
            EmotionScore { emotion: "positive".to_string(), score: 0.3 },
            EmotionScore { emotion: "negative".to_string(), score: 0.1 },
            EmotionScore { emotion: "neutral".to_string(), score: 0.6 },
        ],
    };
    
    ApiResponse::success(response)
}
