use crate::common::error::{Error, Result};
use crate::domain::model::m_style::StyleParams;
use crate::infrastructure::ai::types::LLMService;
use crate::application::ppt::prompt_extractor::{
    ExtractedFeatures, ColorTemperature, AlignmentStyle, SpacingStyle,
};
use crate::application::ppt::ai_edit_service::Language;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct PromptGenerator {
    llm_service: Arc<dyn LLMService>,
}

impl PromptGenerator {
    pub fn new(llm_service: Arc<dyn LLMService>) -> Self {
        Self { llm_service }
    }

    pub async fn generate_prompt(
        &self,
        features: &ExtractedFeatures,
        mode: PromptMode,
        language: Language,
    ) -> Result<GeneratedPrompt> {
        match mode {
            PromptMode::Detailed => self.generate_detailed_prompt(features, language).await,
            PromptMode::Concise => self.generate_concise_prompt(features, language).await,
            PromptMode::Technical => self.generate_technical_prompt(features, language).await,
        }
    }

    async fn generate_detailed_prompt(
        &self,
        features: &ExtractedFeatures,
        language: Language,
    ) -> Result<GeneratedPrompt> {
        let prompt = self.build_detailed_prompt_text(features, language);
        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_prompt_response(&response, features)
    }

    fn build_detailed_prompt_text(&self, features: &ExtractedFeatures, language: Language) -> String {
        let lang_name = language.display_name();

        let color_temp = match features.colors.color_temperature {
            ColorTemperature::Warm => "暖色",
            ColorTemperature::Cool => "冷色",
            ColorTemperature::Neutral => "中性",
        };

        let title_font = features.fonts.title_font.as_deref().unwrap_or("默认");
        let body_font = features.fonts.body_font.as_deref().unwrap_or("默认");

        let min_size = features.fonts.font_sizes.iter().min().unwrap_or(&12);
        let max_size = features.fonts.font_sizes.iter().max().unwrap_or(&24);

        let font_weights = if features.fonts.font_weights.is_empty() {
            "normal, bold".to_string()
        } else {
            features.fonts.font_weights.join(", ")
        };

        let layout_types = if features.layout.layout_types.is_empty() {
            "现代简约".to_string()
        } else {
            features.layout.layout_types.join(", ")
        };

        format!(
            r#"请根据以下PPT特征，生成一份详细的{}风格提示词，用于生成类似风格的PPT：

## 颜色特征
- 主色调：{}
- 辅助色：{}
- 强调色：{}
- 背景色：{}
- 色温：{}

## 字体特征
- 标题字体：{}
- 正文字体：{}
- 字号范围：{}-{}px
- 字重：{}

## 布局特征
- 布局类型：{}
- 对齐方式：{}
- 间距风格：{}

## 风格特征
- 整体风格：{}
- 图标风格：{}
- 装饰风格：{}

请生成一份完整的提示词，包含：
1. 整体风格描述
2. 颜色方案建议
3. 字体规范
4. 布局规则
5. 元素样式建议

请按以下JSON格式返回结果（不要包含markdown代码块标记）：
{{
  "prompt": "完整的提示词文本",
  "style_params": {{
    "color_scheme": {{
      "primary_color": "主色",
      "secondary_color": "辅助色",
      "accent_color": "强调色",
      "background_color": "背景色",
      "text_color": "文本色",
      "text_secondary_color": "次要文本色",
      "gradient_colors": ["渐变色数组"],
      "color_palette": ["调色板数组"]
    }},
    "font_scheme": {{
      "title_font": {{ "family": "字体", "size": 字号, "weight": "字重", "line_height": 行高, "letter_spacing": 字间距 }},
      "subtitle_font": {{ "family": "字体", "size": 字号, "weight": "字重", "line_height": 行高, "letter_spacing": 字间距 }},
      "body_font": {{ "family": "字体", "size": 字号, "weight": "字重", "line_height": 行高, "letter_spacing": 字间距 }},
      "caption_font": {{ "family": "字体", "size": 字号, "weight": "字重", "line_height": 行高, "letter_spacing": 字间距 }}
    }}
  }},
  "confidence": 0.85,
  "suggestions": ["改进建议1", "改进建议2"]
}}"#,
            lang_name,
            features.colors.primary_color,
            features.colors.secondary_colors.join(", "),
            features.colors.accent_color.as_deref().unwrap_or("无"),
            features.colors.background_colors.join(", "),
            color_temp,
            title_font,
            body_font,
            min_size,
            max_size,
            font_weights,
            layout_types,
            features.layout.alignment,
            features.layout.spacing,
            features.style.overall_style,
            features.style.icon_style,
            features.style.decoration_style,
        )
    }

    async fn generate_concise_prompt(
        &self,
        features: &ExtractedFeatures,
        language: Language,
    ) -> Result<GeneratedPrompt> {
        let lang_name = language.display_name();

        let prompt = format!(
            r#"根据以下PPT特征，生成一份简洁的{}风格提示词：

主色调：{}
辅助色：{}
字体：{} / {}
布局：{}
风格：{}

请生成简洁的提示词（不超过200字），按以下JSON格式返回（不要包含markdown代码块标记）：
{{
  "prompt": "简洁的提示词",
  "confidence": 0.85,
  "suggestions": ["建议1"]
}}"#,
            lang_name,
            features.colors.primary_color,
            features.colors.secondary_colors.join(", "),
            features.fonts.title_font.as_deref().unwrap_or("默认"),
            features.fonts.body_font.as_deref().unwrap_or("默认"),
            features.layout.alignment,
            features.style.overall_style,
        );

        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        self.parse_prompt_response(&response, features)
    }

    async fn generate_technical_prompt(
        &self,
        features: &ExtractedFeatures,
        _language: Language,
    ) -> Result<GeneratedPrompt> {
        let style_params = self.build_style_params_from_features(features);

        let prompt = format!(
            r#"将以下PPT特征转换为技术参数格式：

特征数据：
{}

请按以下JSON格式返回技术参数（不要包含markdown代码块标记）：
{{
  "prompt": "技术参数JSON字符串",
  "style_params": {{ ... }},
  "confidence": 0.90,
  "suggestions": []
}}"#,
            serde_json::to_string_pretty(&features).unwrap_or_default()
        );

        let response = self.llm_service.generate(&prompt, Default::default()).await?;
        let mut result = self.parse_prompt_response(&response, features)?;
        result.style_params = style_params;
        Ok(result)
    }

    fn build_style_params_from_features(&self, features: &ExtractedFeatures) -> StyleParams {
        use crate::domain::model::m_style::*;

        let primary = features.colors.primary_color.clone();
        let secondary = features.colors.secondary_colors.first()
            .cloned()
            .unwrap_or_else(|| "#FF6B00".to_string());
        let accent = features.colors.accent_color.clone()
            .unwrap_or_else(|| "#00D4FF".to_string());
        let background = features.colors.background_colors.first()
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string());

        let title_font_family = features.fonts.title_font.clone()
            .unwrap_or_else(|| "思源黑体".to_string());
        let body_font_family = features.fonts.body_font.clone()
            .unwrap_or_else(|| "思源黑体".to_string());

        let title_size = features.fonts.font_sizes.iter().max().copied().unwrap_or(44);
        let body_size = features.fonts.font_sizes.iter().filter(|&&s| s < title_size).max().copied().unwrap_or(18);

        let margin = match features.layout.spacing {
            SpacingStyle::Compact => Margin::new(60),
            SpacingStyle::Normal => Margin::new(80),
            SpacingStyle::Relaxed => Margin::new(100),
        };

        let alignment = match features.layout.alignment {
            AlignmentStyle::Left => "left",
            AlignmentStyle::Center => "center",
            AlignmentStyle::Right => "right",
            AlignmentStyle::Justify => "justify",
        }.to_string();

        StyleParams {
            color_scheme: ColorScheme {
                primary_color: primary,
                secondary_color: secondary,
                accent_color: accent,
                background_color: background,
                text_color: "#333333".to_string(),
                text_secondary_color: "#666666".to_string(),
                gradient_colors: None,
                color_palette: features.colors.secondary_colors.clone(),
            },
            font_scheme: FontScheme {
                title_font: FontConfig {
                    family: title_font_family.clone(),
                    size: title_size,
                    weight: "bold".to_string(),
                    line_height: 1.2,
                    letter_spacing: 0.0,
                },
                subtitle_font: FontConfig {
                    family: title_font_family.clone(),
                    size: (title_size as f32 * 0.6) as u32,
                    weight: "normal".to_string(),
                    line_height: 1.4,
                    letter_spacing: 0.0,
                },
                body_font: FontConfig {
                    family: body_font_family.clone(),
                    size: body_size,
                    weight: "normal".to_string(),
                    line_height: 1.6,
                    letter_spacing: 0.0,
                },
                caption_font: FontConfig {
                    family: body_font_family,
                    size: (body_size as f32 * 0.8) as u32,
                    weight: "lighter".to_string(),
                    line_height: 1.5,
                    letter_spacing: 0.0,
                },
            },
            layout_rules: LayoutRules {
                page_margin: margin,
                content_padding: Padding::new(20),
                grid_columns: features.layout.grid_system.as_ref().map(|g| g.columns).unwrap_or(12),
                grid_gap: features.layout.grid_system.as_ref().map(|g| g.gap).unwrap_or(20),
                element_spacing: Spacing::new(20),
            },
            component_styles: ComponentStyles {
                title_style: TitleStyle {
                    alignment: alignment.clone(),
                    position: Position { x: 0, y: 0 },
                    animation: None,
                },
                text_style: crate::domain::model::m_style::TextStyle {
                    alignment,
                    line_spacing: 1.6,
                    paragraph_spacing: 10,
                    indent: 0,
                },
                image_style: ImageStyle {
                    border_radius: 8,
                    shadow: Some(Shadow::small()),
                    filter: None,
                },
                shape_style: ShapeStyle {
                    border_radius: 8,
                    border_width: 0,
                    border_color: "transparent".to_string(),
                    shadow: None,
                    fill_type: "solid".to_string(),
                },
                chart_style: ChartStyle {
                    color_palette: features.colors.secondary_colors.clone(),
                    border_radius: 8,
                    show_legend: true,
                    legend_position: "bottom".to_string(),
                    show_grid: true,
                    grid_color: "#E0E0E0".to_string(),
                },
                table_style: TableStyle {
                    header_style: TableHeaderStyle {
                        background_color: features.colors.primary_color.clone(),
                        text_color: "#FFFFFF".to_string(),
                        font_weight: "bold".to_string(),
                        alignment: "center".to_string(),
                    },
                    cell_style: TableCellStyle {
                        padding: Padding::new(8),
                        alignment: "left".to_string(),
                        vertical_alignment: "middle".to_string(),
                    },
                    border_style: TableBorderStyle {
                        width: 1,
                        color: "#E0E0E0".to_string(),
                        style: "solid".to_string(),
                    },
                    stripe_rows: false,
                },
            },
        }
    }

    fn parse_prompt_response(
        &self,
        response: &str,
        features: &ExtractedFeatures,
    ) -> Result<GeneratedPrompt> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct PromptResponse {
            prompt: String,
            #[serde(default)]
            style_params: Option<serde_json::Value>,
            #[serde(default = "default_confidence")]
            confidence: f32,
            #[serde(default)]
            suggestions: Vec<String>,
        }

        fn default_confidence() -> f32 { 0.85 }

        let parsed: PromptResponse = serde_json::from_str(cleaned)
            .map_err(|e| Error::internal_error(format!("解析提示词响应失败: {}", e)))?;

        let style_params = if let Some(ref params_value) = parsed.style_params {
            serde_json::from_value(params_value.clone())
                .unwrap_or_else(|_| self.build_style_params_from_features(features))
        } else {
            self.build_style_params_from_features(features)
        };

        Ok(GeneratedPrompt {
            prompt: parsed.prompt,
            style_params,
            confidence: parsed.confidence,
            suggestions: parsed.suggestions,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PromptMode {
    Detailed,
    Concise,
    Technical,
}

impl std::fmt::Display for PromptMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptMode::Detailed => write!(f, "detailed"),
            PromptMode::Concise => write!(f, "concise"),
            PromptMode::Technical => write!(f, "technical"),
        }
    }
}

impl std::str::FromStr for PromptMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "detailed" | "detail" => Ok(PromptMode::Detailed),
            "concise" | "simple" => Ok(PromptMode::Concise),
            "technical" | "tech" => Ok(PromptMode::Technical),
            _ => Err(format!("Unknown prompt mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPrompt {
    pub prompt: String,
    pub style_params: StyleParams,
    pub confidence: f32,
    pub suggestions: Vec<String>,
}
