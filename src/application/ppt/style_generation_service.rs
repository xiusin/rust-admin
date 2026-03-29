use crate::domain::model::m_style::*;
use crate::application::ppt::style_consistency_engine::{StyleConsistencyEngine, ConsistencyResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct StyleGenerationService {
    consistency_engine: StyleConsistencyEngine,
    industry_data: HashMap<String, IndustryStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryStyle {
    pub name: String,
    pub primary_colors: Vec<String>,
    pub secondary_colors: Vec<String>,
    pub font_families: Vec<String>,
    pub layout_style: String,
    pub mood: String,
}

impl StyleGenerationService {
    pub fn new() -> Self {
        let mut industry_data = HashMap::new();
        
        industry_data.insert("科技".to_string(), IndustryStyle {
            name: "科技".to_string(),
            primary_colors: vec!["#0066FF".to_string(), "#00D4FF".to_string(), "#6C5CE7".to_string()],
            secondary_colors: vec!["#FF6B00".to_string(), "#FF4757".to_string(), "#2ED573".to_string()],
            font_families: vec!["思源黑体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "现代简约".to_string(),
            mood: "科技感".to_string(),
        });

        industry_data.insert("金融".to_string(), IndustryStyle {
            name: "金融".to_string(),
            primary_colors: vec!["#1B4F72".to_string(), "#2874A6".to_string(), "#2E86AB".to_string()],
            secondary_colors: vec!["#F39C12".to_string(), "#E67E22".to_string(), "#D4AC0D".to_string()],
            font_families: vec!["思源宋体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "商务专业".to_string(),
            mood: "专业".to_string(),
        });

        industry_data.insert("教育".to_string(), IndustryStyle {
            name: "教育".to_string(),
            primary_colors: vec!["#27AE60".to_string(), "#2ECC71".to_string(), "#58D68D".to_string()],
            secondary_colors: vec!["#3498DB".to_string(), "#5DADE2".to_string(), "#F39C12".to_string()],
            font_families: vec!["思源黑体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "清晰易读".to_string(),
            mood: "温馨".to_string(),
        });

        industry_data.insert("医疗".to_string(), IndustryStyle {
            name: "医疗".to_string(),
            primary_colors: vec!["#3498DB".to_string(), "#5DADE2".to_string(), "#85C1E9".to_string()],
            secondary_colors: vec!["#2ECC71".to_string(), "#58D68D".to_string(), "#F39C12".to_string()],
            font_families: vec!["思源黑体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "简洁专业".to_string(),
            mood: "专业".to_string(),
        });

        industry_data.insert("电商".to_string(), IndustryStyle {
            name: "电商".to_string(),
            primary_colors: vec!["#FF6B6B".to_string(), "#FF8E53".to_string(), "#FFA502".to_string()],
            secondary_colors: vec!["#00D4FF".to_string(), "#2ED573".to_string(), "#9B59B6".to_string()],
            font_families: vec!["思源黑体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "活泼动感".to_string(),
            mood: "活泼".to_string(),
        });

        industry_data.insert("政府".to_string(), IndustryStyle {
            name: "政府".to_string(),
            primary_colors: vec!["#C0392B".to_string(), "#E74C3C".to_string(), "#D98880".to_string()],
            secondary_colors: vec!["#F1C40F".to_string(), "#F39C12".to_string(), "#3498DB".to_string()],
            font_families: vec!["思源宋体".to_string(), "微软雅黑".to_string(), "PingFang SC".to_string()],
            layout_style: "庄重大方".to_string(),
            mood: "专业".to_string(),
        });

        Self {
            consistency_engine: StyleConsistencyEngine::new(),
            industry_data,
        }
    }

    pub async fn generate_from_industry(
        &self,
        industry: &str,
        confidence: f32,
    ) -> crate::common::error::Result<StyleParams> {
        let industry_style = self.industry_data.get(industry)
            .ok_or_else(|| crate::common::error::Error::not_found(format!("未找到行业 '{}' 的风格配置", industry)))?;

        let primary_idx = (confidence * (industry_style.primary_colors.len() as f32 - 1.0)) as usize;
        let secondary_idx = (confidence * (industry_style.secondary_colors.len() as f32 - 1.0)) as usize;
        let font_idx = (confidence * (industry_style.font_families.len() as f32 - 1.0)) as usize;

        let primary_color = industry_style.primary_colors[primary_idx.min(industry_style.primary_colors.len() - 1)].clone();
        let secondary_color = industry_style.secondary_colors[secondary_idx.min(industry_style.secondary_colors.len() - 1)].clone();
        let font_family = industry_style.font_families[font_idx.min(industry_style.font_families.len() - 1)].clone();

        let mut style_params = StyleParams {
            color_scheme: ColorScheme {
                primary_color: primary_color.clone(),
                secondary_color: secondary_color.clone(),
                accent_color: industry_style.secondary_colors.get(1).cloned().unwrap_or_else(|| secondary_color.clone()),
                background_color: "#FFFFFF".to_string(),
                text_color: "#1A1A1A".to_string(),
                text_secondary_color: "#666666".to_string(),
                gradient_colors: Some(vec![primary_color.clone(), industry_style.primary_colors.get(1).cloned().unwrap_or_else(|| primary_color.clone())]),
                color_palette: vec![
                    primary_color.clone(),
                    secondary_color.clone(),
                    "#FFFFFF".to_string(),
                    "#F5F5F5".to_string(),
                ],
            },
            font_scheme: FontScheme {
                title_font: FontConfig {
                    family: font_family.clone(),
                    size: 44,
                    weight: "bold".to_string(),
                    line_height: 1.2,
                    letter_spacing: 0.0,
                },
                subtitle_font: FontConfig {
                    family: font_family.clone(),
                    size: 32,
                    weight: "600".to_string(),
                    line_height: 1.3,
                    letter_spacing: 0.0,
                },
                body_font: FontConfig {
                    family: font_family.clone(),
                    size: 18,
                    weight: "normal".to_string(),
                    line_height: 1.6,
                    letter_spacing: 0.0,
                },
                caption_font: FontConfig {
                    family: font_family,
                    size: 14,
                    weight: "normal".to_string(),
                    line_height: 1.5,
                    letter_spacing: 0.0,
                },
            },
            layout_rules: LayoutRules {
                page_margin: Margin::new_vertical_horizontal(40, 60),
                content_padding: Padding::new(20),
                grid_columns: 12,
                grid_gap: 20,
                element_spacing: Spacing::new(16),
            },
            component_styles: ComponentStyles {
                title_style: TitleStyle {
                    alignment: "center".to_string(),
                    position: Position { x: 0, y: 0 },
                    animation: Some("fadeIn".to_string()),
                },
                text_style: TextStyle {
                    alignment: "left".to_string(),
                    line_spacing: 1.6,
                    paragraph_spacing: 16,
                    indent: 0,
                },
                image_style: ImageStyle {
                    border_radius: 8,
                    shadow: Some(Shadow::medium()),
                    filter: None,
                },
                shape_style: ShapeStyle {
                    border_radius: 4,
                    border_width: 0,
                    border_color: "transparent".to_string(),
                    shadow: Some(Shadow::small()),
                    fill_type: "solid".to_string(),
                },
                chart_style: ChartStyle {
                    color_palette: vec![primary_color.clone(), secondary_color],
                    border_radius: 4,
                    show_legend: true,
                    legend_position: "bottom".to_string(),
                    show_grid: true,
                    grid_color: "#E5E5E5".to_string(),
                },
                table_style: TableStyle {
                    header_style: TableHeaderStyle {
                        background_color: primary_color,
                        text_color: "#FFFFFF".to_string(),
                        font_weight: "bold".to_string(),
                        alignment: "center".to_string(),
                    },
                    cell_style: TableCellStyle {
                        padding: Padding::new(12),
                        alignment: "left".to_string(),
                        vertical_alignment: "middle".to_string(),
                    },
                    border_style: TableBorderStyle {
                        width: 1,
                        color: "#E5E5E5".to_string(),
                        style: "solid".to_string(),
                    },
                    stripe_rows: true,
                },
            },
        };

        self.consistency_engine.adjust_for_consistency(&mut style_params);

        Ok(style_params)
    }

    pub async fn generate_from_content(
        &self,
        content: &str,
        keywords: &[(String, f32)],
    ) -> crate::common::error::Result<StyleParams> {
        let detected_industry = self.detect_industry_from_content(content, keywords);
        
        if let Some(industry) = detected_industry {
            let confidence = keywords.first().map(|(_, score)| *score).unwrap_or(0.5);
            self.generate_from_industry(&industry, confidence).await
        } else {
            self.generate_default_style().await
        }
    }

    pub async fn generate_from_reference(
        &self,
        reference_data: &HashMap<String, serde_json::Value>,
    ) -> crate::common::error::Result<StyleParams> {
        let mut style_params = self.generate_default_style().await?;

        if let Some(color_scheme) = reference_data.get("color_scheme") {
            if let Ok(parsed) = serde_json::from_value::<ColorScheme>(color_scheme.clone()) {
                style_params.color_scheme = parsed;
            }
        }

        if let Some(font_scheme) = reference_data.get("font_scheme") {
            if let Ok(parsed) = serde_json::from_value::<FontScheme>(font_scheme.clone()) {
                style_params.font_scheme = parsed;
            }
        }

        if let Some(layout_rules) = reference_data.get("layout_rules") {
            if let Ok(parsed) = serde_json::from_value::<LayoutRules>(layout_rules.clone()) {
                style_params.layout_rules = parsed;
            }
        }

        if let Some(component_styles) = reference_data.get("component_styles") {
            if let Ok(parsed) = serde_json::from_value::<ComponentStyles>(component_styles.clone()) {
                style_params.component_styles = parsed;
            }
        }

        self.consistency_engine.adjust_for_consistency(&mut style_params);

        Ok(style_params)
    }

    pub async fn generate_custom(
        &self,
        preferences: StylePreferences,
    ) -> crate::common::error::Result<StyleParams> {
        let mut style_params = if let Some(ref industry) = preferences.industry {
            self.generate_from_industry(industry, 0.5).await?
        } else {
            self.generate_default_style().await?
        };

        if let Some(ref mood) = preferences.mood {
            self.apply_mood(&mut style_params, mood);
        }

        if let Some(ref color_prefs) = preferences.color_preferences {
            if !color_prefs.is_empty() {
                style_params.color_scheme.primary_color = color_prefs[0].clone();
                if color_prefs.len() > 1 {
                    style_params.color_scheme.secondary_color = color_prefs[1].clone();
                }
                if color_prefs.len() > 2 {
                    style_params.color_scheme.accent_color = color_prefs[2].clone();
                }
                style_params.color_scheme.color_palette = color_prefs.clone();
            }
        }

        if let Some(ref font_prefs) = preferences.font_preferences {
            if !font_prefs.is_empty() {
                let font_family = font_prefs[0].clone();
                style_params.font_scheme.title_font.family = font_family.clone();
                style_params.font_scheme.subtitle_font.family = font_family.clone();
                style_params.font_scheme.body_font.family = font_family.clone();
                style_params.font_scheme.caption_font.family = font_family;
            }
        }

        if let Some(ref layout_pref) = preferences.layout_preference {
            self.apply_layout_preference(&mut style_params, layout_pref);
        }

        self.consistency_engine.adjust_for_consistency(&mut style_params);

        Ok(style_params)
    }

    fn detect_industry_from_content(&self, content: &str, keywords: &[(String, f32)]) -> Option<String> {
        let industry_keywords = vec![
            ("科技", vec!["技术", "创新", "数字化", "AI", "人工智能", "云计算", "大数据"]),
            ("金融", vec!["投资", "理财", "股票", "基金", "银行", "证券", "金融"]),
            ("教育", vec!["教学", "学习", "课程", "培训", "教育", "学生", "老师"]),
            ("医疗", vec!["医院", "治疗", "健康", "医疗", "诊断", "患者", "医生"]),
            ("电商", vec!["购物", "商品", "订单", "物流", "电商", "促销", "营销"]),
            ("政府", vec!["政策", "政府", "公共服务", "行政", "管理", "法规"]),
        ];

        let content_lower = content.to_lowercase();
        
        for (industry, words) in industry_keywords {
            let match_count = words.iter()
                .filter(|word| content_lower.contains(&word.to_lowercase()))
                .count();
            
            if match_count > 0 {
                return Some(industry.to_string());
            }
        }

        if !keywords.is_empty() {
            let top_keyword = &keywords[0].0;
            for (industry, _) in &self.industry_data {
                if top_keyword.contains(industry) || industry.contains(top_keyword) {
                    return Some(industry.clone());
                }
            }
        }

        None
    }

    fn apply_mood(&self, style_params: &mut StyleParams, mood: &str) {
        match mood {
            "专业" => {
                style_params.color_scheme.primary_color = "#1B4F72".to_string();
                style_params.font_scheme.title_font.weight = "bold".to_string();
                style_params.layout_rules.grid_columns = 12;
            }
            "活泼" => {
                style_params.color_scheme.primary_color = "#FF6B6B".to_string();
                style_params.font_scheme.title_font.weight = "600".to_string();
                style_params.component_styles.title_style.animation = Some("bounceIn".to_string());
            }
            "简约" => {
                style_params.color_scheme.primary_color = "#2C3E50".to_string();
                style_params.layout_rules.page_margin = Margin::new(60);
                style_params.component_styles.image_style.border_radius = 0;
            }
            "科技感" => {
                style_params.color_scheme.primary_color = "#0066FF".to_string();
                style_params.color_scheme.gradient_colors = Some(vec!["#0066FF".to_string(), "#00D4FF".to_string()]);
                style_params.component_styles.shape_style.fill_type = "gradient".to_string();
            }
            "温馨" => {
                style_params.color_scheme.primary_color = "#27AE60".to_string();
                style_params.color_scheme.background_color = "#FFFEF5".to_string();
                style_params.font_scheme.body_font.line_height = 1.8;
            }
            _ => {}
        }
    }

    fn apply_layout_preference(&self, style_params: &mut StyleParams, preference: &str) {
        match preference {
            "居中" => {
                style_params.component_styles.title_style.alignment = "center".to_string();
                style_params.component_styles.text_style.alignment = "center".to_string();
            }
            "左对齐" => {
                style_params.component_styles.title_style.alignment = "left".to_string();
                style_params.component_styles.text_style.alignment = "left".to_string();
            }
            "网格" => {
                style_params.layout_rules.grid_columns = 12;
                style_params.layout_rules.grid_gap = 24;
            }
            "自由" => {
                style_params.layout_rules.grid_columns = 24;
                style_params.layout_rules.grid_gap = 16;
            }
            "对称" => {
                style_params.layout_rules.page_margin = Margin::new_vertical_horizontal(40, 60);
                style_params.component_styles.title_style.alignment = "center".to_string();
            }
            _ => {}
        }
    }

    async fn generate_default_style(&self) -> crate::common::error::Result<StyleParams> {
        self.generate_from_industry("科技", 0.5).await
    }

    pub fn validate_style(&self, params: &StyleParams) -> ConsistencyResult {
        let score = self.consistency_engine.calculate_harmony_score(params);
        let all_results = self.consistency_engine.validate_all(params);
        
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();
        
        for result in all_results.values() {
            all_issues.extend(result.issues.clone());
            all_suggestions.extend(result.suggestions.clone());
        }

        ConsistencyResult {
            is_valid: !all_issues.iter().any(|i| matches!(i.severity, crate::application::ppt::style_consistency_engine::IssueSeverity::Error)),
            score,
            issues: all_issues,
            suggestions: all_suggestions,
        }
    }

    pub fn get_supported_industries(&self) -> Vec<String> {
        self.industry_data.keys().cloned().collect()
    }

    pub fn get_industry_style(&self, industry: &str) -> Option<&IndustryStyle> {
        self.industry_data.get(industry)
    }
}

impl Default for StyleGenerationService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleContext {
    pub industry: Option<String>,
    pub content: Option<String>,
    pub keywords: Vec<(String, f32)>,
    pub preferences: Option<StylePreferences>,
}

impl StyleContext {
    pub fn new() -> Self {
        Self {
            industry: None,
            content: None,
            keywords: Vec::new(),
            preferences: None,
        }
    }

    pub fn with_industry(mut self, industry: String) -> Self {
        self.industry = Some(industry);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<(String, f32)>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn with_preferences(mut self, preferences: StylePreferences) -> Self {
        self.preferences = Some(preferences);
        self
    }
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}
