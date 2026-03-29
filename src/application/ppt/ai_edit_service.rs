use crate::common::error::{Error, Result};
use crate::infrastructure::ai::types::LLMService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolishStyle {
    Professional,
    Casual,
    Academic,
    Creative,
}

impl std::fmt::Display for PolishStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolishStyle::Professional => write!(f, "professional"),
            PolishStyle::Casual => write!(f, "casual"),
            PolishStyle::Academic => write!(f, "academic"),
            PolishStyle::Creative => write!(f, "creative"),
        }
    }
}

impl std::str::FromStr for PolishStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "professional" => Ok(PolishStyle::Professional),
            "casual" => Ok(PolishStyle::Casual),
            "academic" => Ok(PolishStyle::Academic),
            "creative" => Ok(PolishStyle::Creative),
            _ => Err(format!("Unknown polish style: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContinueDirection {
    Forward,
    Expand,
    Detail,
}

impl std::fmt::Display for ContinueDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContinueDirection::Forward => write!(f, "forward"),
            ContinueDirection::Expand => write!(f, "expand"),
            ContinueDirection::Detail => write!(f, "detail"),
        }
    }
}

impl std::str::FromStr for ContinueDirection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(ContinueDirection::Forward),
            "expand" => Ok(ContinueDirection::Expand),
            "detail" => Ok(ContinueDirection::Detail),
            _ => Err(format!("Unknown continue direction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentLength {
    Short,
    Medium,
    Long,
}

impl std::fmt::Display for ContentLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentLength::Short => write!(f, "short"),
            ContentLength::Medium => write!(f, "medium"),
            ContentLength::Long => write!(f, "long"),
        }
    }
}

impl std::str::FromStr for ContentLength {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "short" => Ok(ContentLength::Short),
            "medium" => Ok(ContentLength::Medium),
            "long" => Ok(ContentLength::Long),
            _ => Err(format!("Unknown content length: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SummaryLength {
    OneLine,
    Brief,
    Detailed,
}

impl std::fmt::Display for SummaryLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummaryLength::OneLine => write!(f, "one_line"),
            SummaryLength::Brief => write!(f, "brief"),
            SummaryLength::Detailed => write!(f, "detailed"),
        }
    }
}

impl std::str::FromStr for SummaryLength {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "one_line" | "oneline" => Ok(SummaryLength::OneLine),
            "brief" => Ok(SummaryLength::Brief),
            "detailed" => Ok(SummaryLength::Detailed),
            _ => Err(format!("Unknown summary length: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
    French,
    German,
    Spanish,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "english"),
            Language::Chinese => write!(f, "chinese"),
            Language::Japanese => write!(f, "japanese"),
            Language::Korean => write!(f, "korean"),
            Language::French => write!(f, "french"),
            Language::German => write!(f, "german"),
            Language::Spanish => write!(f, "spanish"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "english" | "en" => Ok(Language::English),
            "chinese" | "zh" | "cn" => Ok(Language::Chinese),
            "japanese" | "ja" | "jp" => Ok(Language::Japanese),
            "korean" | "ko" | "kr" => Ok(Language::Korean),
            "french" | "fr" => Ok(Language::French),
            "german" | "de" => Ok(Language::German),
            "spanish" | "es" => Ok(Language::Spanish),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "英文",
            Language::Chinese => "中文",
            Language::Japanese => "日文",
            Language::Korean => "韩文",
            Language::French => "法文",
            Language::German => "德文",
            Language::Spanish => "西班牙文",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub position: usize,
    pub original: String,
    pub replacement: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolishResult {
    pub original: String,
    pub polished: String,
    pub changes: Vec<TextChange>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueResult {
    pub continued_text: String,
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeResult {
    pub summary: String,
    pub key_points: Vec<String>,
    pub word_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateResult {
    pub original: String,
    pub translated: String,
    pub source_language: Language,
    pub target_language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    Grammar,
    Style,
    Structure,
    Content,
}

impl std::fmt::Display for SuggestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestionType::Grammar => write!(f, "grammar"),
            SuggestionType::Style => write!(f, "style"),
            SuggestionType::Structure => write!(f, "structure"),
            SuggestionType::Content => write!(f, "content"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub suggestion_type: SuggestionType,
    pub content: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestResult {
    pub suggestions: Vec<Suggestion>,
}

pub struct AIEditService {
    llm_service: Arc<dyn LLMService>,
}

impl AIEditService {
    pub fn new(llm_service: Arc<dyn LLMService>) -> Self {
        Self { llm_service }
    }

    pub async fn polish_text(
        &self,
        text: &str,
        style: PolishStyle,
    ) -> Result<PolishResult> {
        if text.trim().is_empty() {
            return Err(Error::bad_request("文本不能为空"));
        }

        let prompt = self.build_polish_prompt(text, style);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_polish_result(text, &response)
    }

    fn build_polish_prompt(&self, text: &str, style: PolishStyle) -> String {
        let style_desc = match style {
            PolishStyle::Professional => "专业、正式的商务风格，保持原意不变",
            PolishStyle::Casual => "轻松、活泼的风格，适合演讲展示",
            PolishStyle::Academic => "学术、严谨的风格",
            PolishStyle::Creative => "创意、有吸引力的风格",
        };

        format!(
            r#"请将以下文本润色为{}。

要求：
1. 保持原文的核心意思不变
2. 优化语言表达，使其更加流畅
3. 调整语气和风格以符合目标风格
4. 返回JSON格式结果

原文：
{}

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "polished": "润色后的文本",
  "changes": [
    {{
      "position": 位置索引,
      "original": "原文片段",
      "replacement": "替换后的片段",
      "reason": "修改原因"
    }}
  ],
  "suggestions": ["其他改进建议"]
}}"#,
            style_desc, text
        )
    }

    fn parse_polish_result(&self, original: &str, response: &str) -> Result<PolishResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct PolishResponse {
            polished: String,
            #[serde(default)]
            changes: Vec<TextChange>,
            #[serde(default)]
            suggestions: Vec<String>,
        }

        let parsed: PolishResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析润色结果失败: {}", e)))?;

        Ok(PolishResult {
            original: original.to_string(),
            polished: parsed.polished,
            changes: parsed.changes,
            suggestions: parsed.suggestions,
        })
    }

    pub async fn continue_content(
        &self,
        context: &str,
        direction: ContinueDirection,
        length: ContentLength,
    ) -> Result<ContinueResult> {
        if context.trim().is_empty() {
            return Err(Error::bad_request("上下文不能为空"));
        }

        let prompt = self.build_continue_prompt(context, direction, length);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_continue_result(&response)
    }

    fn build_continue_prompt(
        &self,
        context: &str,
        direction: ContinueDirection,
        length: ContentLength,
    ) -> String {
        let (action_desc, length_hint) = match (direction, length) {
            (ContinueDirection::Forward, ContentLength::Short) => 
                ("继续往后写", "简短（1-2句话）"),
            (ContinueDirection::Forward, ContentLength::Medium) => 
                ("继续往后写", "适中（3-5句话）"),
            (ContinueDirection::Forward, ContentLength::Long) => 
                ("继续往后写", "详细（6-10句话）"),
            (ContinueDirection::Expand, ContentLength::Short) => 
                ("扩展内容", "简短（1-2句话）"),
            (ContinueDirection::Expand, ContentLength::Medium) => 
                ("扩展内容", "适中（3-5句话）"),
            (ContinueDirection::Expand, ContentLength::Long) => 
                ("扩展内容", "详细（6-10句话）"),
            (ContinueDirection::Detail, ContentLength::Short) => 
                ("添加细节", "简短（1-2句话）"),
            (ContinueDirection::Detail, ContentLength::Medium) => 
                ("添加细节", "适中（3-5句话）"),
            (ContinueDirection::Detail, ContentLength::Long) => 
                ("添加细节", "详细（6-10句话）"),
        };

        format!(
            r#"基于以下内容，{}{}的内容。

原文：
{}

要求：
1. 保持与原文的风格和语气一致
2. 内容要自然衔接，逻辑连贯
3. 返回JSON格式结果

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "continued_text": "续写的内容",
  "key_points": ["关键要点1", "关键要点2"]
}}"#,
            action_desc, length_hint, context
        )
    }

    fn parse_continue_result(&self, response: &str) -> Result<ContinueResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct ContinueResponse {
            continued_text: String,
            #[serde(default)]
            key_points: Vec<String>,
        }

        let parsed: ContinueResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析续写结果失败: {}", e)))?;

        Ok(ContinueResult {
            continued_text: parsed.continued_text,
            key_points: parsed.key_points,
        })
    }

    pub async fn summarize_content(
        &self,
        content: &str,
        length: SummaryLength,
    ) -> Result<SummarizeResult> {
        if content.trim().is_empty() {
            return Err(Error::bad_request("内容不能为空"));
        }

        let prompt = self.build_summarize_prompt(content, length);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_summarize_result(content, &response)
    }

    fn build_summarize_prompt(&self, content: &str, length: SummaryLength) -> String {
        let length_desc = match length {
            SummaryLength::OneLine => "用一句话",
            SummaryLength::Brief => "用2-3句话简要",
            SummaryLength::Detailed => "详细",
        };

        format!(
            r#"请{}总结以下内容，提取关键要点。

内容：
{}

要求：
1. 准确概括核心内容
2. 提取关键要点
3. 返回JSON格式结果

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "summary": "总结内容",
  "key_points": ["关键要点1", "关键要点2", "关键要点3"]
}}"#,
            length_desc, content
        )
    }

    fn parse_summarize_result(&self, content: &str, response: &str) -> Result<SummarizeResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct SummarizeResponse {
            summary: String,
            #[serde(default)]
            key_points: Vec<String>,
        }

        let parsed: SummarizeResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析总结结果失败: {}", e)))?;

        let word_count = content.chars().count();

        Ok(SummarizeResult {
            summary: parsed.summary,
            key_points: parsed.key_points,
            word_count,
        })
    }

    pub async fn translate_content(
        &self,
        content: &str,
        target_language: Language,
        preserve_terms: bool,
    ) -> Result<TranslateResult> {
        if content.trim().is_empty() {
            return Err(Error::bad_request("内容不能为空"));
        }

        let source_language = self.detect_language(content);
        let prompt = self.build_translate_prompt(content, &target_language, preserve_terms);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        
        self.parse_translate_result(content, &response, source_language, &target_language)
    }

    fn detect_language(&self, content: &str) -> Language {
        let chinese_chars = content.chars().filter(|c| '\u{4E00}' <= *c && *c <= '\u{9FFF}').count();
        let total_chars = content.chars().filter(|c| !c.is_whitespace()).count();
        
        if total_chars > 0 && (chinese_chars as f32 / total_chars as f32) > 0.3 {
            Language::Chinese
        } else {
            Language::English
        }
    }

    fn build_translate_prompt(
        &self,
        content: &str,
        target_language: &Language,
        preserve_terms: bool,
    ) -> String {
        let lang_name = target_language.display_name();
        
        let instruction = if preserve_terms {
            format!("请将以下内容翻译成{}，保留专业术语不翻译", lang_name)
        } else {
            format!("请将以下内容翻译成{}", lang_name)
        };

        format!(
            r#"{}

内容：
{}

要求：
1. 准确翻译，保持原意
2. 语言流畅自然
3. 返回JSON格式结果

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "translated": "翻译后的内容"
}}"#,
            instruction, content
        )
    }

    fn parse_translate_result(
        &self,
        original: &str,
        response: &str,
        source_language: Language,
        target_language: &Language,
    ) -> Result<TranslateResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct TranslateResponse {
            translated: String,
        }

        let parsed: TranslateResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析翻译结果失败: {}", e)))?;

        Ok(TranslateResult {
            original: original.to_string(),
            translated: parsed.translated,
            source_language,
            target_language: target_language.clone(),
        })
    }

    pub async fn get_suggestions(
        &self,
        content: &str,
        context: Option<&str>,
    ) -> Result<SuggestResult> {
        if content.trim().is_empty() {
            return Err(Error::bad_request("内容不能为空"));
        }

        let prompt = self.build_suggest_prompt(content, context);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_suggest_result(&response)
    }

    fn build_suggest_prompt(&self, content: &str, context: Option<&str>) -> String {
        let context_part = if let Some(ctx) = context {
            format!("\n\n上下文信息：\n{}", ctx)
        } else {
            String::new()
        };

        format!(
            r#"请分析以下内容并提供改进建议。{}

内容：
{}

请从以下几个方面提供建议：
1. 语法：语法错误、标点符号使用等
2. 风格：语言风格、表达方式等
3. 结构：段落结构、逻辑顺序等
4. 内容：内容完整性、准确性等

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "suggestions": [
    {{
      "suggestion_type": "grammar|style|structure|content",
      "content": "具体建议内容",
      "confidence": 0.0-1.0之间的置信度
    }}
  ]
}}"#,
            context_part, content
        )
    }

    fn parse_suggest_result(&self, response: &str) -> Result<SuggestResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct SuggestResponse {
            #[serde(default)]
            suggestions: Vec<Suggestion>,
        }

        let parsed: SuggestResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析建议结果失败: {}", e)))?;

        Ok(SuggestResult {
            suggestions: parsed.suggestions,
        })
    }
}

#[async_trait]
pub trait AIEditServiceTrait: Send + Sync {
    async fn polish_text(&self, text: &str, style: PolishStyle) -> Result<PolishResult>;
    async fn continue_content(&self, context: &str, direction: ContinueDirection, length: ContentLength) -> Result<ContinueResult>;
    async fn summarize_content(&self, content: &str, length: SummaryLength) -> Result<SummarizeResult>;
    async fn translate_content(&self, content: &str, target_language: Language, preserve_terms: bool) -> Result<TranslateResult>;
    async fn get_suggestions(&self, content: &str, context: Option<&str>) -> Result<SuggestResult>;
}

#[async_trait]
impl AIEditServiceTrait for AIEditService {
    async fn polish_text(&self, text: &str, style: PolishStyle) -> Result<PolishResult> {
        self.polish_text(text, style).await
    }

    async fn continue_content(&self, context: &str, direction: ContinueDirection, length: ContentLength) -> Result<ContinueResult> {
        self.continue_content(context, direction, length).await
    }

    async fn summarize_content(&self, content: &str, length: SummaryLength) -> Result<SummarizeResult> {
        self.summarize_content(content, length).await
    }

    async fn translate_content(&self, content: &str, target_language: Language, preserve_terms: bool) -> Result<TranslateResult> {
        self.translate_content(content, target_language, preserve_terms).await
    }

    async fn get_suggestions(&self, content: &str, context: Option<&str>) -> Result<SuggestResult> {
        self.get_suggestions(content, context).await
    }
}
