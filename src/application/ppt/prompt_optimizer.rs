use crate::common::error::{Error, Result};
use crate::infrastructure::ai::types::LLMService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct PromptOptimizer {
    llm_service: Arc<dyn LLMService>,
}

impl PromptOptimizer {
    pub fn new(llm_service: Arc<dyn LLMService>) -> Self {
        Self { llm_service }
    }

    pub async fn validate_prompt(&self, prompt: &str) -> Result<ValidationResult> {
        let mut missing_elements = Vec::new();
        let mut issues = Vec::new();
        let mut quality_score = 1.0_f32;

        let prompt_lower = prompt.to_lowercase();

        if !prompt_lower.contains("颜色") && !prompt_lower.contains("color") {
            missing_elements.push("颜色方案".to_string());
            quality_score -= 0.15;
        }

        if !prompt_lower.contains("字体") && !prompt_lower.contains("font") {
            missing_elements.push("字体规范".to_string());
            quality_score -= 0.15;
        }

        if !prompt_lower.contains("布局") && !prompt_lower.contains("layout") {
            missing_elements.push("布局规则".to_string());
            quality_score -= 0.15;
        }

        if !prompt_lower.contains("风格") && !prompt_lower.contains("style") {
            missing_elements.push("风格描述".to_string());
            quality_score -= 0.10;
        }

        if prompt.len() < 50 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                message: "提示词过短，可能缺少详细信息".to_string(),
                location: None,
            });
            quality_score -= 0.10;
        }

        if prompt.len() > 2000 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                message: "提示词过长，可能影响生成效率".to_string(),
                location: None,
            });
        }

        if !prompt.contains('#') && !prompt_lower.contains("rgb") {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Info,
                message: "建议添加具体的颜色代码（如 #0066FF）".to_string(),
                location: None,
            });
            quality_score -= 0.05;
        }

        let is_valid = missing_elements.is_empty() && quality_score > 0.5;

        Ok(ValidationResult {
            is_valid,
            missing_elements,
            quality_score: quality_score.max(0.0),
            issues,
        })
    }

    pub async fn optimize_prompt(&self, prompt: &str) -> Result<OptimizedPrompt> {
        let validation = self.validate_prompt(prompt).await?;

        if validation.is_valid && validation.quality_score > 0.8 {
            return Ok(OptimizedPrompt {
                original: prompt.to_string(),
                optimized: prompt.to_string(),
                changes: vec![],
            });
        }

        let optimization_prompt = self.build_optimization_prompt(prompt, &validation);
        let response = self.llm_service.generate(&optimization_prompt, Default::default()).await?;
        self.parse_optimization_result(prompt, &response)
    }

    fn build_optimization_prompt(&self, prompt: &str, validation: &ValidationResult) -> String {
        let missing = validation.missing_elements.join(", ");
        let issues: Vec<String> = validation.issues.iter()
            .map(|i| format!("- {}", i.message))
            .collect();

        format!(
            r#"请优化以下PPT生成提示词，使其更加完整和有效。

原始提示词：
{}

需要补充的元素：
{}

存在的问题：
{}

请按以下要求优化：
1. 补充缺失的元素
2. 改进表达方式，使其更加清晰
3. 添加具体的颜色代码和字体规范
4. 保持原始风格意图

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "optimized": "优化后的提示词",
  "changes": [
    {{
      "type": "添加|修改|删除",
      "description": "变更描述",
      "reason": "变更原因"
    }}
  ]
}}"#,
            prompt,
            missing,
            issues.join("\n")
        )
    }

    fn parse_optimization_result(&self, original: &str, response: &str) -> Result<OptimizedPrompt> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct OptimizationResponse {
            optimized: String,
            #[serde(default)]
            changes: Vec<PromptChange>,
        }

        let parsed: OptimizationResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析优化结果失败: {}", e)))?;

        Ok(OptimizedPrompt {
            original: original.to_string(),
            optimized: parsed.optimized,
            changes: parsed.changes,
        })
    }

    pub async fn suggest_improvements(&self, prompt: &str) -> Result<Vec<ImprovementSuggestion>> {
        let validation = self.validate_prompt(prompt).await?;

        let mut suggestions = Vec::new();

        for missing in &validation.missing_elements {
            suggestions.push(ImprovementSuggestion {
                category: "完整性".to_string(),
                suggestion: format!("添加{}相关描述", missing),
                priority: 9,
            });
        }

        for issue in &validation.issues {
            let priority = match issue.severity {
                IssueSeverity::Error => 10,
                IssueSeverity::Warning => 7,
                IssueSeverity::Info => 5,
            };
            suggestions.push(ImprovementSuggestion {
                category: "质量".to_string(),
                suggestion: issue.message.clone(),
                priority,
            });
        }

        if !prompt.contains("主色") && !prompt.contains("primary") {
            suggestions.push(ImprovementSuggestion {
                category: "颜色".to_string(),
                suggestion: "明确指定主色调及其十六进制代码".to_string(),
                priority: 8,
            });
        }

        if !prompt.contains("标题字体") && !prompt.contains("title font") {
            suggestions.push(ImprovementSuggestion {
                category: "字体".to_string(),
                suggestion: "指定标题字体和正文字体".to_string(),
                priority: 8,
            });
        }

        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(suggestions)
    }

    pub async fn test_prompt(&self, prompt: &str) -> Result<TestResult> {
        let test_prompt = format!(
            r#"基于以下提示词，生成一个PPT页面的示例：

{}

请生成一个标题页的示例，并评估提示词的有效性。

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "sample_pages": [
    {{
      "page_type": "title",
      "title": "示例标题",
      "subtitle": "示例副标题",
      "background_color": "背景色",
      "title_style": {{ "font": "字体", "size": 字号, "color": "颜色" }}
    }}
  ],
  "quality_score": 0.85,
  "feedback": "对提示词的评价和改进建议"
}}"#,
            prompt
        );

        let response = self.llm_service.generate(&test_prompt, Default::default()).await?;
        self.parse_test_result(&response)
    }

    fn parse_test_result(&self, response: &str) -> Result<TestResult> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct TestResponse {
            #[serde(default)]
            sample_pages: Vec<GeneratedPage>,
            #[serde(default = "default_quality")]
            quality_score: f32,
            #[serde(default)]
            feedback: String,
        }

        fn default_quality() -> f32 { 0.75 }

        let parsed: TestResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析测试结果失败: {}", e)))?;

        Ok(TestResult {
            sample_pages: parsed.sample_pages,
            quality_score: parsed.quality_score,
            feedback: parsed.feedback,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub missing_elements: Vec<String>,
    pub quality_score: f32,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedPrompt {
    pub original: String,
    pub optimized: String,
    pub changes: Vec<PromptChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChange {
    #[serde(rename = "type")]
    pub change_type: String,
    pub description: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    pub category: String,
    pub suggestion: String,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub sample_pages: Vec<GeneratedPage>,
    pub quality_score: f32,
    pub feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPage {
    pub page_type: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub background_color: Option<String>,
    pub title_style: Option<TitleStyleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleStyleInfo {
    pub font: String,
    pub size: u32,
    pub color: String,
}
