use crate::domain::model::m_style::*;
use std::collections::HashMap;

pub struct StyleConsistencyEngine;

impl StyleConsistencyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_colors(&self, scheme: &ColorScheme) -> ConsistencyResult {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        let mut score: f32 = 100.0;

        if !ColorScheme::validate_hex_color(&scheme.primary_color) {
            issues.push(ConsistencyIssue {
                category: "颜色格式".to_string(),
                description: "主色调格式不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 20.0;
        }

        if !ColorScheme::validate_hex_color(&scheme.secondary_color) {
            issues.push(ConsistencyIssue {
                category: "颜色格式".to_string(),
                description: "辅助色格式不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 20.0;
        }

        if !ColorScheme::validate_hex_color(&scheme.accent_color) {
            issues.push(ConsistencyIssue {
                category: "颜色格式".to_string(),
                description: "强调色格式不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 20.0;
        }

        let (primary_h, primary_s, _) = ColorScheme::hex_to_hsv(&scheme.primary_color);
        let (secondary_h, secondary_s, _) = ColorScheme::hex_to_hsv(&scheme.secondary_color);
        let (accent_h, accent_s, _) = ColorScheme::hex_to_hsv(&scheme.accent_color);

        let hue_diff_1 = (primary_h - secondary_h).abs();
        let hue_diff_2 = (primary_h - accent_h).abs();
        let hue_diff_3 = (secondary_h - accent_h).abs();

        if hue_diff_1 < 15.0 && hue_diff_2 < 15.0 && hue_diff_3 < 15.0 {
            issues.push(ConsistencyIssue {
                category: "色相一致性".to_string(),
                description: "颜色之间色相差异过小，可能导致视觉单调".to_string(),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议增加颜色之间的色相差异，使用互补色或类似色".to_string());
            score -= 10.0;
        }

        let sat_diff_1 = (primary_s - secondary_s).abs();
        let sat_diff_2 = (primary_s - accent_s).abs();
        
        if sat_diff_1 > 50.0 || sat_diff_2 > 50.0 {
            issues.push(ConsistencyIssue {
                category: "饱和度范围".to_string(),
                description: "颜色饱和度差异过大，可能导致视觉不协调".to_string(),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议保持颜色饱和度在相近范围内".to_string());
            score -= 10.0;
        }

        let contrast_ratio = ColorScheme::calculate_contrast_ratio(&scheme.text_color, &scheme.background_color);
        
        if contrast_ratio < 4.5 {
            issues.push(ConsistencyIssue {
                category: "对比度合规".to_string(),
                description: format!("文字与背景对比度为 {:.2}，低于 WCAG AA 标准 (4.5)", contrast_ratio),
                severity: IssueSeverity::Error,
            });
            suggestions.push("建议增加文字颜色与背景颜色的对比度".to_string());
            score -= 25.0;
        } else if contrast_ratio < 7.0 {
            issues.push(ConsistencyIssue {
                category: "对比度合规".to_string(),
                description: format!("文字与背景对比度为 {:.2}，符合 WCAG AA 标准，但未达到 AAA 标准 (7.0)", contrast_ratio),
                severity: IssueSeverity::Info,
            });
            suggestions.push("建议进一步增加对比度以达到 AAA 标准".to_string());
            score -= 5.0;
        }

        if let Some(ref gradient_colors) = scheme.gradient_colors {
            for color in gradient_colors {
                if !ColorScheme::validate_hex_color(color) {
                    issues.push(ConsistencyIssue {
                        category: "渐变色格式".to_string(),
                        description: format!("渐变色 {} 格式不正确", color),
                        severity: IssueSeverity::Error,
                    });
                    score -= 10.0;
                }
            }
        }

        for color in &scheme.color_palette {
            if !ColorScheme::validate_hex_color(color) {
                issues.push(ConsistencyIssue {
                    category: "调色板格式".to_string(),
                    description: format!("调色板颜色 {} 格式不正确", color),
                    severity: IssueSeverity::Error,
                });
                score -= 10.0;
            }
        }

        ConsistencyResult {
            is_valid: !issues.iter().any(|i| matches!(i.severity, IssueSeverity::Error)),
            score: score.max(0.0),
            issues,
            suggestions,
        }
    }

    pub fn validate_fonts(&self, scheme: &FontScheme) -> ConsistencyResult {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        let mut score: f32 = 100.0;

        if !FontConfig::validate_weight(&scheme.title_font.weight) {
            issues.push(ConsistencyIssue {
                category: "字体权重".to_string(),
                description: "标题字体权重值不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 15.0;
        }

        if !FontConfig::validate_weight(&scheme.subtitle_font.weight) {
            issues.push(ConsistencyIssue {
                category: "字体权重".to_string(),
                description: "副标题字体权重值不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 15.0;
        }

        if !FontConfig::validate_weight(&scheme.body_font.weight) {
            issues.push(ConsistencyIssue {
                category: "字体权重".to_string(),
                description: "正文字体权重值不正确".to_string(),
                severity: IssueSeverity::Error,
            });
            score -= 15.0;
        }

        let title_to_body = scheme.title_font.calculate_scale_ratio(&scheme.body_font);
        let subtitle_to_body = scheme.subtitle_font.calculate_scale_ratio(&scheme.body_font);
        let caption_to_body = scheme.caption_font.calculate_scale_ratio(&scheme.body_font);

        if title_to_body < 1.2 || title_to_body > 2.5 {
            issues.push(ConsistencyIssue {
                category: "字号层级".to_string(),
                description: format!("标题与正文字号比例 {:.2} 不在推荐范围 (1.2-2.5)", title_to_body),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议标题字号为正文字号的1.2-2.5倍".to_string());
            score -= 10.0;
        }

        if subtitle_to_body < 1.1 || subtitle_to_body > 1.5 {
            issues.push(ConsistencyIssue {
                category: "字号层级".to_string(),
                description: format!("副标题与正文字号比例 {:.2} 不在推荐范围 (1.1-1.5)", subtitle_to_body),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议副标题字号为正文字号的1.1-1.5倍".to_string());
            score -= 10.0;
        }

        if caption_to_body > 1.0 {
            issues.push(ConsistencyIssue {
                category: "字号层级".to_string(),
                description: "说明文字号不应大于正文字号".to_string(),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议说明文字号小于正文字号".to_string());
            score -= 10.0;
        }

        let fonts = vec![
            &scheme.title_font.family,
            &scheme.subtitle_font.family,
            &scheme.body_font.family,
            &scheme.caption_font.family,
        ];
        let unique_fonts: Vec<_> = fonts.iter().cloned().collect();
        let unique_count = unique_fonts.iter().filter(|f| ***f != scheme.body_font.family).count();

        if unique_count > 2 {
            issues.push(ConsistencyIssue {
                category: "字体家族".to_string(),
                description: "使用了过多的字体家族，建议不超过2种".to_string(),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议统一使用相同的字体家族，通过字重和字号区分层级".to_string());
            score -= 10.0;
        }

        let line_heights = vec![
            scheme.title_font.line_height,
            scheme.subtitle_font.line_height,
            scheme.body_font.line_height,
            scheme.caption_font.line_height,
        ];

        for (i, lh) in line_heights.iter().enumerate() {
            if *lh < 1.0 || *lh > 2.0 {
                let font_name = match i {
                    0 => "标题",
                    1 => "副标题",
                    2 => "正文",
                    _ => "说明文字",
                };
                issues.push(ConsistencyIssue {
                    category: "行高比例".to_string(),
                    description: format!("{}行高 {:.2} 不在推荐范围 (1.0-2.0)", font_name, lh),
                    severity: IssueSeverity::Warning,
                });
                score -= 5.0;
            }
        }

        ConsistencyResult {
            is_valid: !issues.iter().any(|i| matches!(i.severity, IssueSeverity::Error)),
            score: score.max(0.0),
            issues,
            suggestions,
        }
    }

    pub fn validate_layout(&self, rules: &LayoutRules) -> ConsistencyResult {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        let mut score: f32 = 100.0;

        if rules.grid_columns < 1 || rules.grid_columns > 24 {
            issues.push(ConsistencyIssue {
                category: "网格系统".to_string(),
                description: format!("网格列数 {} 不在推荐范围 (1-24)", rules.grid_columns),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议使用12或24列网格系统".to_string());
            score -= 10.0;
        }

        if rules.grid_gap > 50 {
            issues.push(ConsistencyIssue {
                category: "网格系统".to_string(),
                description: format!("网格间距 {} 过大", rules.grid_gap),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议网格间距不超过50px".to_string());
            score -= 10.0;
        }

        let margin_sum = rules.page_margin.top + rules.page_margin.right 
            + rules.page_margin.bottom + rules.page_margin.left;
        let margin_avg = margin_sum as f32 / 4.0;

        let margin_variance = (
            (rules.page_margin.top as f32 - margin_avg).powi(2) +
            (rules.page_margin.right as f32 - margin_avg).powi(2) +
            (rules.page_margin.bottom as f32 - margin_avg).powi(2) +
            (rules.page_margin.left as f32 - margin_avg).powi(2)
        ) / 4.0;

        if margin_variance > 100.0 {
            issues.push(ConsistencyIssue {
                category: "边距比例".to_string(),
                description: "页面边距差异过大，可能导致视觉不平衡".to_string(),
                severity: IssueSeverity::Warning,
            });
            suggestions.push("建议保持页面边距比例一致".to_string());
            score -= 10.0;
        }

        let spacing_ratio = rules.element_spacing.large as f32 / rules.element_spacing.small as f32;
        if spacing_ratio < 2.0 || spacing_ratio > 6.0 {
            issues.push(ConsistencyIssue {
                category: "元素间距".to_string(),
                description: format!("元素间距比例 {:.2} 不在推荐范围 (2.0-6.0)", spacing_ratio),
                severity: IssueSeverity::Info,
            });
            suggestions.push("建议元素间距遵循一定的比例关系".to_string());
            score -= 5.0;
        }

        ConsistencyResult {
            is_valid: !issues.iter().any(|i| matches!(i.severity, IssueSeverity::Error)),
            score: score.max(0.0),
            issues,
            suggestions,
        }
    }

    pub fn adjust_for_consistency(&self, params: &mut StyleParams) {
        let color_result = self.validate_colors(&params.color_scheme);
        if !color_result.is_valid {
            for issue in &color_result.issues {
                if matches!(issue.severity, IssueSeverity::Error) {
                    if issue.category == "对比度合规" {
                        params.color_scheme.text_color = "#1A1A1A".to_string();
                        params.color_scheme.background_color = "#FFFFFF".to_string();
                    }
                }
            }
        }

        let font_result = self.validate_fonts(&params.font_scheme);
        if !font_result.is_valid {
            for issue in &font_result.issues {
                if matches!(issue.severity, IssueSeverity::Error) {
                    if issue.category == "字体权重" {
                        params.font_scheme.title_font.weight = "bold".to_string();
                        params.font_scheme.subtitle_font.weight = "600".to_string();
                        params.font_scheme.body_font.weight = "normal".to_string();
                        params.font_scheme.caption_font.weight = "normal".to_string();
                    }
                }
            }

            let title_to_body = params.font_scheme.title_font.calculate_scale_ratio(&params.font_scheme.body_font);
            if title_to_body < 1.2 {
                params.font_scheme.title_font.size = (params.font_scheme.body_font.size as f32 * 1.5) as u32;
            } else if title_to_body > 2.5 {
                params.font_scheme.title_font.size = (params.font_scheme.body_font.size as f32 * 2.0) as u32;
            }
        }

        let layout_result = self.validate_layout(&params.layout_rules);
        if !layout_result.is_valid {
            for issue in &layout_result.issues {
                if matches!(issue.severity, IssueSeverity::Error) {
                    if issue.category == "网格系统" {
                        params.layout_rules.grid_columns = 12;
                        params.layout_rules.grid_gap = 20;
                    }
                }
            }
        }
    }

    pub fn calculate_harmony_score(&self, params: &StyleParams) -> f32 {
        let color_result = self.validate_colors(&params.color_scheme);
        let font_result = self.validate_fonts(&params.font_scheme);
        let layout_result = self.validate_layout(&params.layout_rules);

        let color_weight = 0.4;
        let font_weight = 0.35;
        let layout_weight = 0.25;

        let total_score = 
            color_result.score * color_weight +
            font_result.score * font_weight +
            layout_result.score * layout_weight;

        total_score.max(0.0).min(100.0)
    }

    pub fn validate_all(&self, params: &StyleParams) -> HashMap<String, ConsistencyResult> {
        let mut results = HashMap::new();
        results.insert("colors".to_string(), self.validate_colors(&params.color_scheme));
        results.insert("fonts".to_string(), self.validate_fonts(&params.font_scheme));
        results.insert("layout".to_string(), self.validate_layout(&params.layout_rules));
        results
    }
}

impl Default for StyleConsistencyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConsistencyResult {
    pub is_valid: bool,
    pub score: f32,
    pub issues: Vec<ConsistencyIssue>,
    pub suggestions: Vec<String>,
}

impl ConsistencyResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            score: 100.0,
            issues: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| matches!(i.severity, IssueSeverity::Error))
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| matches!(i.severity, IssueSeverity::Warning))
    }

    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.severity, IssueSeverity::Error)).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.severity, IssueSeverity::Warning)).count()
    }
}

impl Default for ConsistencyResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConsistencyIssue {
    pub category: String,
    pub description: String,
    pub severity: IssueSeverity,
}

impl ConsistencyIssue {
    pub fn new(category: String, description: String, severity: IssueSeverity) -> Self {
        Self {
            category,
            description,
            severity,
        }
    }

    pub fn error(category: String, description: String) -> Self {
        Self::new(category, description, IssueSeverity::Error)
    }

    pub fn warning(category: String, description: String) -> Self {
        Self::new(category, description, IssueSeverity::Warning)
    }

    pub fn info(category: String, description: String) -> Self {
        Self::new(category, description, IssueSeverity::Info)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Error => write!(f, "错误"),
            IssueSeverity::Warning => write!(f, "警告"),
            IssueSeverity::Info => write!(f, "信息"),
        }
    }
}
