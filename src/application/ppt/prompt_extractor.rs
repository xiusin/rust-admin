use crate::common::error::Result;
use crate::domain::model::m_document::{ContentBlock, ParsedDocument, Section};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct PromptExtractor;

impl PromptExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_features(&self, ppt: &ParsedDocument) -> ExtractedFeatures {
        let mut features = ExtractedFeatures::default();

        features.colors = self.extract_color_features(ppt);
        features.fonts = self.extract_font_features(ppt);
        features.layout = self.extract_layout_features(ppt);
        features.style = self.extract_style_features(ppt);

        features
    }

    fn extract_color_features(&self, ppt: &ParsedDocument) -> ColorFeatures {
        let mut color_counts: HashMap<String, usize> = HashMap::new();
        let mut all_colors: Vec<String> = Vec::new();

        if let Some(ref primary) = ppt.style_hints.primary_color {
            color_counts.entry(primary.clone()).and_modify(|c| *c += 10).or_insert(10);
            all_colors.push(primary.clone());
        }

        if let Some(ref secondary) = ppt.style_hints.secondary_color {
            color_counts.entry(secondary.clone()).and_modify(|c| *c += 5).or_insert(5);
            all_colors.push(secondary.clone());
        }

        for section in &ppt.sections {
            self.extract_colors_from_section(section, &mut color_counts, &mut all_colors);
        }

        let mut sorted_colors: Vec<(String, usize)> = color_counts.into_iter().collect();
        sorted_colors.sort_by(|a, b| b.1.cmp(&a.1));

        let primary_color = sorted_colors.first()
            .map(|(c, _)| c.clone())
            .unwrap_or_else(|| "#0066FF".to_string());

        let secondary_colors: Vec<String> = sorted_colors.iter()
            .skip(1)
            .take(3)
            .map(|(c, _)| c.clone())
            .collect();

        let accent_color = sorted_colors.iter()
            .skip(4)
            .next()
            .map(|(c, _)| c.clone());

        let background_colors = self.detect_background_colors(&all_colors);

        let color_distribution: HashMap<String, f32> = {
            let total: usize = sorted_colors.iter().map(|(_, count)| count).sum();
            sorted_colors.iter()
                .map(|(color, count)| {
                    let percentage = if total > 0 {
                        (*count as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    };
                    (color.clone(), percentage)
                })
                .collect()
        };

        let color_temperature = self.calculate_color_temperature(&primary_color);

        ColorFeatures {
            primary_color,
            secondary_colors,
            accent_color,
            background_colors,
            color_distribution,
            color_temperature,
        }
    }

    fn extract_colors_from_section(
        &self,
        section: &Section,
        color_counts: &mut HashMap<String, usize>,
        all_colors: &mut Vec<String>,
    ) {
        for block in &section.content {
            if let ContentBlock::Text(text_block) = block {
                if let Some(ref style) = text_block.style {
                    if let Some(ref color) = style.color {
                        color_counts.entry(color.clone()).and_modify(|c| *c += 1).or_insert(1);
                        all_colors.push(color.clone());
                    }
                }
            }
        }
    }

    fn detect_background_colors(&self, colors: &[String]) -> Vec<String> {
        let mut bg_colors = Vec::new();

        for color in colors {
            if let Ok(luminance) = self.calculate_luminance(color) {
                if luminance > 0.8 || luminance < 0.2 {
                    bg_colors.push(color.clone());
                }
            }
        }

        if bg_colors.is_empty() {
            bg_colors.push("#FFFFFF".to_string());
        }

        bg_colors
    }

    fn calculate_luminance(&self, hex: &str) -> Result<f32> {
        let hex = hex.trim_start_matches('#');
        if hex.len() < 6 {
            return Ok(0.5);
        }

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128) as f32 / 255.0;

        let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
        let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
        let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };

        Ok(0.2126 * r + 0.7152 * g + 0.0722 * b)
    }

    fn calculate_color_temperature(&self, hex: &str) -> ColorTemperature {
        let hex = hex.trim_start_matches('#');
        if hex.len() < 6 {
            return ColorTemperature::Neutral;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
        let _g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);

        let warm_score = (r as i32 - b as i32).abs();
        let cool_score = (b as i32 - r as i32).abs();

        if warm_score > cool_score + 30 {
            ColorTemperature::Warm
        } else if cool_score > warm_score + 30 {
            ColorTemperature::Cool
        } else {
            ColorTemperature::Neutral
        }
    }

    fn extract_font_features(&self, ppt: &ParsedDocument) -> FontFeatures {
        let mut font_families: HashSet<String> = HashSet::new();
        let mut font_sizes: Vec<u32> = Vec::new();
        let mut font_weights: HashSet<String> = HashSet::new();
        let mut line_heights: Vec<f32> = Vec::new();

        if let Some(ref font) = ppt.style_hints.font_family {
            font_families.insert(font.clone());
        }

        for section in &ppt.sections {
            self.extract_fonts_from_section(
                section,
                &mut font_families,
                &mut font_sizes,
                &mut font_weights,
                &mut line_heights,
            );
        }

        let title_font = font_families.iter().next().cloned();
        let body_font = font_families.iter().skip(1).next().cloned()
            .or_else(|| title_font.clone());

        let mut unique_sizes: Vec<u32> = font_sizes.into_iter().collect();
        unique_sizes.sort();
        unique_sizes.dedup();

        let weights: Vec<String> = font_weights.into_iter().collect();
        let heights: Vec<f32> = line_heights.into_iter().collect();

        FontFeatures {
            title_font,
            body_font,
            font_sizes: unique_sizes,
            font_weights: weights,
            line_heights: heights,
        }
    }

    fn extract_fonts_from_section(
        &self,
        section: &Section,
        font_families: &mut HashSet<String>,
        font_sizes: &mut Vec<u32>,
        font_weights: &mut HashSet<String>,
        _line_heights: &mut Vec<f32>,
    ) {
        for block in &section.content {
            if let ContentBlock::Text(text_block) = block {
                if let Some(ref style) = text_block.style {
                    if let Some(ref family) = style.font_family {
                        font_families.insert(family.clone());
                    }
                    if let Some(size) = style.font_size {
                        font_sizes.push(size as u32);
                    }
                    if let Some(ref _weight) = style.font_family {
                        if style.bold.unwrap_or(false) {
                            font_weights.insert("bold".to_string());
                        } else {
                            font_weights.insert("normal".to_string());
                        }
                    }
                }
            }
        }
    }

    fn extract_layout_features(&self, ppt: &ParsedDocument) -> LayoutFeatures {
        let mut layout_types: Vec<String> = Vec::new();
        let mut alignment_styles: Vec<String> = Vec::new();
        let has_grid = false;

        if let Some(ref layout_style) = ppt.style_hints.layout_style {
            layout_types.push(layout_style.clone());
        }

        let section_count = ppt.sections.len();
        if section_count > 5 {
            layout_types.push("multi_section".to_string());
        }

        for section in &ppt.sections {
            let content_types: HashSet<String> = section.content.iter()
                .map(|block| match block {
                    ContentBlock::Text(_) => "text".to_string(),
                    ContentBlock::Image(_) => "image".to_string(),
                    ContentBlock::Table(_) => "table".to_string(),
                    ContentBlock::List(_) => "list".to_string(),
                    ContentBlock::Code(_) => "code".to_string(),
                })
                .collect();

            if content_types.len() > 2 {
                layout_types.push("mixed".to_string());
            }

            for block in &section.content {
                if let ContentBlock::Text(text_block) = block {
                    if let Some(ref style) = text_block.style {
                        if style.bold.unwrap_or(false) && text_block.text.len() < 50 {
                            alignment_styles.push("title_center".to_string());
                        }
                    }
                }
            }
        }

        layout_types.dedup();

        let alignment = self.determine_alignment(&alignment_styles);
        let spacing = self.determine_spacing(section_count);
        let grid_system = if has_grid {
            Some(GridSystem {
                columns: 12,
                gap: 20,
            })
        } else {
            None
        };

        LayoutFeatures {
            layout_types,
            alignment,
            spacing,
            grid_system,
        }
    }

    fn determine_alignment(&self, alignment_styles: &[String]) -> AlignmentStyle {
        let center_count = alignment_styles.iter().filter(|s| *s == "title_center").count();

        if center_count > alignment_styles.len() / 2 {
            AlignmentStyle::Center
        } else {
            AlignmentStyle::Left
        }
    }

    fn determine_spacing(&self, section_count: usize) -> SpacingStyle {
        if section_count > 10 {
            SpacingStyle::Compact
        } else if section_count > 5 {
            SpacingStyle::Normal
        } else {
            SpacingStyle::Relaxed
        }
    }

    fn extract_style_features(&self, ppt: &ParsedDocument) -> StyleFeatures {
        let overall_style = self.detect_overall_style(ppt);
        let icon_style = self.detect_icon_style(ppt);
        let decoration_style = self.detect_decoration_style(ppt);
        let animation_style = None;

        StyleFeatures {
            overall_style,
            icon_style,
            decoration_style,
            animation_style,
        }
    }

    fn detect_overall_style(&self, ppt: &ParsedDocument) -> String {
        let mut style_hints = Vec::new();

        if let Some(ref heading_style) = ppt.style_hints.heading_style {
            style_hints.push(heading_style.clone());
        }

        if let Some(ref industry) = ppt.industry_hint {
            style_hints.push(industry.clone());
        }

        let has_images = ppt.images.len() > 0;
        let has_tables = ppt.sections.iter().any(|s| {
            s.content.iter().any(|b| matches!(b, ContentBlock::Table(_)))
        });

        if has_images && has_tables {
            style_hints.push("professional".to_string());
        } else if has_images {
            style_hints.push("visual".to_string());
        } else if has_tables {
            style_hints.push("data_driven".to_string());
        }

        if style_hints.is_empty() {
            "modern".to_string()
        } else {
            style_hints.join("_")
        }
    }

    fn detect_icon_style(&self, ppt: &ParsedDocument) -> String {
        let has_code = ppt.sections.iter().any(|s| {
            s.content.iter().any(|b| matches!(b, ContentBlock::Code(_)))
        });

        if has_code {
            "technical".to_string()
        } else {
            "minimal".to_string()
        }
    }

    fn detect_decoration_style(&self, ppt: &ParsedDocument) -> String {
        if ppt.sections.len() > 8 {
            "minimal".to_string()
        } else {
            "moderate".to_string()
        }
    }
}

impl Default for PromptExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractedFeatures {
    pub colors: ColorFeatures,
    pub fonts: FontFeatures,
    pub layout: LayoutFeatures,
    pub style: StyleFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeatures {
    pub primary_color: String,
    pub secondary_colors: Vec<String>,
    pub accent_color: Option<String>,
    pub background_colors: Vec<String>,
    pub color_distribution: HashMap<String, f32>,
    pub color_temperature: ColorTemperature,
}

impl Default for ColorFeatures {
    fn default() -> Self {
        Self {
            primary_color: "#0066FF".to_string(),
            secondary_colors: vec!["#FF6B00".to_string()],
            accent_color: Some("#00D4FF".to_string()),
            background_colors: vec!["#FFFFFF".to_string()],
            color_distribution: HashMap::new(),
            color_temperature: ColorTemperature::Neutral,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorTemperature {
    Warm,
    Cool,
    Neutral,
}

impl std::fmt::Display for ColorTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorTemperature::Warm => write!(f, "暖色"),
            ColorTemperature::Cool => write!(f, "冷色"),
            ColorTemperature::Neutral => write!(f, "中性"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFeatures {
    pub title_font: Option<String>,
    pub body_font: Option<String>,
    pub font_sizes: Vec<u32>,
    pub font_weights: Vec<String>,
    pub line_heights: Vec<f32>,
}

impl Default for FontFeatures {
    fn default() -> Self {
        Self {
            title_font: Some("思源黑体".to_string()),
            body_font: Some("思源黑体".to_string()),
            font_sizes: vec![14, 18, 24, 36, 44],
            font_weights: vec!["normal".to_string(), "bold".to_string()],
            line_heights: vec![1.5, 1.6, 1.8],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutFeatures {
    pub layout_types: Vec<String>,
    pub alignment: AlignmentStyle,
    pub spacing: SpacingStyle,
    pub grid_system: Option<GridSystem>,
}

impl Default for LayoutFeatures {
    fn default() -> Self {
        Self {
            layout_types: vec!["modern".to_string()],
            alignment: AlignmentStyle::Center,
            spacing: SpacingStyle::Normal,
            grid_system: Some(GridSystem {
                columns: 12,
                gap: 20,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlignmentStyle {
    Left,
    Center,
    Right,
    Justify,
}

impl std::fmt::Display for AlignmentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignmentStyle::Left => write!(f, "左对齐"),
            AlignmentStyle::Center => write!(f, "居中对齐"),
            AlignmentStyle::Right => write!(f, "右对齐"),
            AlignmentStyle::Justify => write!(f, "两端对齐"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpacingStyle {
    Compact,
    Normal,
    Relaxed,
}

impl std::fmt::Display for SpacingStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpacingStyle::Compact => write!(f, "紧凑"),
            SpacingStyle::Normal => write!(f, "适中"),
            SpacingStyle::Relaxed => write!(f, "宽松"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridSystem {
    pub columns: u32,
    pub gap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleFeatures {
    pub overall_style: String,
    pub icon_style: String,
    pub decoration_style: String,
    pub animation_style: Option<String>,
}

impl Default for StyleFeatures {
    fn default() -> Self {
        Self {
            overall_style: "modern".to_string(),
            icon_style: "minimal".to_string(),
            decoration_style: "moderate".to_string(),
            animation_style: None,
        }
    }
}
