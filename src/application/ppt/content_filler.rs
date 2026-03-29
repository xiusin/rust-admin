use crate::domain::model::m_style::*;
use super::page_generator::{
    PageElement, TextContent, ImageContent, 
    ChartContent, TableContent, Border,
};

pub struct ContentFiller;

impl ContentFiller {
    pub fn new() -> Self {
        Self
    }

    pub fn fill_text(
        &self,
        element: &mut PageElement,
        text_content: &TextContent,
        style: &StyleParams,
    ) {
        if let Some(ref mut content) = element.content.text {
            if text_content.font_family.is_none() {
                content.font_family = Some(style.font_scheme.body_font.family.clone());
            }
            if text_content.font_size.is_none() {
                content.font_size = Some(style.font_scheme.body_font.size);
            }
            if text_content.font_weight.is_none() {
                content.font_weight = Some("normal".to_string());
            }
            if text_content.color.is_none() {
                content.color = Some(style.color_scheme.text_color.clone());
            }
            if text_content.alignment.is_none() {
                content.alignment = Some(style.component_styles.text_style.alignment.clone());
            }
            if text_content.line_height.is_none() {
                content.line_height = Some(style.font_scheme.body_font.line_height);
            }
            if text_content.letter_spacing.is_none() {
                content.letter_spacing = Some(style.font_scheme.body_font.letter_spacing);
            }
            
            content.text = text_content.text.clone();
        } else {
            element.content.text = Some(TextContent {
                text: text_content.text.clone(),
                font_family: text_content.font_family.clone()
                    .or_else(|| Some(style.font_scheme.body_font.family.clone())),
                font_size: text_content.font_size.clone()
                    .or_else(|| Some(style.font_scheme.body_font.size)),
                font_weight: text_content.font_weight.clone()
                    .or_else(|| Some("normal".to_string())),
                color: text_content.color.clone()
                    .or_else(|| Some(style.color_scheme.text_color.clone())),
                alignment: text_content.alignment.clone()
                    .or_else(|| Some(style.component_styles.text_style.alignment.clone())),
                line_height: text_content.line_height.clone()
                    .or_else(|| Some(style.font_scheme.body_font.line_height)),
                letter_spacing: text_content.letter_spacing.clone()
                    .or_else(|| Some(style.font_scheme.body_font.letter_spacing)),
            });
        }
    }

    pub fn fill_title(
        &self,
        element: &mut PageElement,
        title_text: &str,
        style: &StyleParams,
    ) {
        element.content.text = Some(TextContent {
            text: title_text.to_string(),
            font_family: Some(style.font_scheme.title_font.family.clone()),
            font_size: Some(style.font_scheme.title_font.size),
            font_weight: Some(style.font_scheme.title_font.weight.clone()),
            color: Some(style.color_scheme.primary_color.clone()),
            alignment: Some(style.component_styles.title_style.alignment.clone()),
            line_height: Some(style.font_scheme.title_font.line_height),
            letter_spacing: Some(style.font_scheme.title_font.letter_spacing),
        });
        
        element.style.shadow = Some(Shadow::small());
    }

    pub fn fill_subtitle(
        &self,
        element: &mut PageElement,
        subtitle_text: &str,
        style: &StyleParams,
    ) {
        element.content.text = Some(TextContent {
            text: subtitle_text.to_string(),
            font_family: Some(style.font_scheme.subtitle_font.family.clone()),
            font_size: Some(style.font_scheme.subtitle_font.size),
            font_weight: Some(style.font_scheme.subtitle_font.weight.clone()),
            color: Some(style.color_scheme.text_secondary_color.clone()),
            alignment: Some(style.component_styles.title_style.alignment.clone()),
            line_height: Some(style.font_scheme.subtitle_font.line_height),
            letter_spacing: Some(style.font_scheme.subtitle_font.letter_spacing),
        });
    }

    pub fn fill_body_text(
        &self,
        element: &mut PageElement,
        body_text: &str,
        style: &StyleParams,
    ) {
        element.content.text = Some(TextContent {
            text: body_text.to_string(),
            font_family: Some(style.font_scheme.body_font.family.clone()),
            font_size: Some(style.font_scheme.body_font.size),
            font_weight: Some("normal".to_string()),
            color: Some(style.color_scheme.text_color.clone()),
            alignment: Some(style.component_styles.text_style.alignment.clone()),
            line_height: Some(style.font_scheme.body_font.line_height),
            letter_spacing: Some(style.font_scheme.body_font.letter_spacing),
        });
    }

    pub fn fill_image(
        &self,
        element: &mut PageElement,
        image_content: &ImageContent,
        style: &StyleParams,
    ) {
        element.content.image = Some(ImageContent {
            url: image_content.url.clone(),
            data: image_content.data.clone(),
            alt_text: image_content.alt_text.clone(),
            fit_mode: image_content.fit_mode.clone()
                .or_else(|| Some("contain".to_string())),
        });

        element.style.shadow = style.component_styles.image_style.shadow.clone();
        element.style.border = Some(Border {
            width: 0,
            color: "transparent".to_string(),
            style: "solid".to_string(),
            radius: Some(style.component_styles.image_style.border_radius),
        });

        if let Some(ref filter) = style.component_styles.image_style.filter {
            if filter != "none" {
                element.style.transform = Some(super::page_generator::Transform {
                    scale_x: None,
                    scale_y: None,
                    skew_x: None,
                    skew_y: None,
                });
            }
        }
    }

    pub fn fill_table(
        &self,
        element: &mut PageElement,
        table_content: &TableContent,
        style: &StyleParams,
    ) {
        element.content.table = Some(TableContent {
            headers: table_content.headers.clone(),
            rows: table_content.rows.clone(),
            column_widths: table_content.column_widths.clone(),
        });

        element.style.border = Some(Border {
            width: style.component_styles.table_style.border_style.width,
            color: style.component_styles.table_style.border_style.color.clone(),
            style: style.component_styles.table_style.border_style.style.clone(),
            radius: None,
        });
    }

    pub fn fill_chart(
        &self,
        element: &mut PageElement,
        chart_content: &ChartContent,
        style: &StyleParams,
    ) {
        let mut chart = chart_content.clone();
        
        if chart.options.is_none() {
            chart.options = Some(super::page_generator::ChartOptions {
                show_legend: Some(style.component_styles.chart_style.show_legend),
                legend_position: Some(style.component_styles.chart_style.legend_position.clone()),
                show_grid: Some(style.component_styles.chart_style.show_grid),
                animation: Some(true),
            });
        }

        for (idx, dataset) in chart.data.datasets.iter_mut().enumerate() {
            if dataset.color.is_none() {
                let color_idx = idx % style.component_styles.chart_style.color_palette.len();
                dataset.color = Some(style.component_styles.chart_style.color_palette[color_idx].clone());
            }
        }

        element.content.chart = Some(chart);
        
        element.style.shadow = Some(Shadow::medium());
        element.style.border = Some(Border {
            width: 1,
            color: "#E5E5E5".to_string(),
            style: "solid".to_string(),
            radius: Some(style.component_styles.chart_style.border_radius),
        });
    }

    pub fn apply_style_to_element(
        &self,
        element: &mut PageElement,
        style: &StyleParams,
    ) {
        match element.element_type {
            super::page_generator::ElementType::Text => {
                if let Some(ref text_content) = element.content.text.clone() {
                    self.fill_text(element, text_content, style);
                }
            }
            super::page_generator::ElementType::Image => {
                if let Some(ref image_content) = element.content.image.clone() {
                    self.fill_image(element, image_content, style);
                }
            }
            super::page_generator::ElementType::Table => {
                if let Some(ref table_content) = element.content.table.clone() {
                    self.fill_table(element, table_content, style);
                }
            }
            super::page_generator::ElementType::Chart => {
                if let Some(ref chart_content) = element.content.chart.clone() {
                    self.fill_chart(element, chart_content, style);
                }
            }
            super::page_generator::ElementType::Shape => {
                self.apply_shape_style(element, style);
            }
            _ => {}
        }
    }

    fn apply_shape_style(&self, element: &mut PageElement, style: &StyleParams) {
        if let Some(ref mut shape) = element.content.shape {
            if shape.fill_color.is_none() {
                shape.fill_color = Some(style.color_scheme.primary_color.clone());
            }
            if shape.border_color.is_none() {
                shape.border_color = Some(style.color_scheme.secondary_color.clone());
            }
            if shape.border_width.is_none() {
                shape.border_width = Some(style.component_styles.shape_style.border_width);
            }
            if shape.border_radius.is_none() {
                shape.border_radius = Some(style.component_styles.shape_style.border_radius);
            }
        }

        element.style.shadow = style.component_styles.shape_style.shadow.clone();
    }

    pub fn apply_text_effects(
        &self,
        element: &mut PageElement,
        effects: &TextEffects,
    ) {
        if let Some(ref mut text) = element.content.text {
            if let Some(color) = &effects.color {
                text.color = Some(color.clone());
            }
            if let Some(weight) = &effects.weight {
                text.font_weight = Some(weight.clone());
            }
            if let Some(size) = effects.size {
                text.font_size = Some(size);
            }
        }

        if let Some(shadow) = &effects.shadow {
            element.style.shadow = Some(shadow.clone());
        }
    }

    pub fn apply_image_effects(
        &self,
        element: &mut PageElement,
        effects: &ImageEffects,
    ) {
        if effects.border_radius.is_some() || effects.shadow.is_some() {
            element.style.border = Some(Border {
                width: 0,
                color: "transparent".to_string(),
                style: "solid".to_string(),
                radius: effects.border_radius,
            });
        }

        if let Some(shadow) = &effects.shadow {
            element.style.shadow = Some(shadow.clone());
        }

        if let Some(opacity) = effects.opacity {
            element.style.opacity = Some(opacity);
        }
    }

    pub fn fill_element_with_data(
        &self,
        element: &mut PageElement,
        data: &serde_json::Value,
        style: &StyleParams,
    ) {
        if let Some(text) = data.get("text").and_then(|v| v.as_str()) {
            self.fill_body_text(element, text, style);
        }

        if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
            element.content.image = Some(ImageContent {
                url: Some(url.to_string()),
                data: None,
                alt_text: data.get("alt").and_then(|v| v.as_str()).map(|s| s.to_string()),
                fit_mode: Some("contain".to_string()),
            });
        }

        if let Some(table_data) = data.get("table") {
            if let Ok(table) = serde_json::from_value::<TableContent>(table_data.clone()) {
                self.fill_table(element, &table, style);
            }
        }

        if let Some(chart_data) = data.get("chart") {
            if let Ok(chart) = serde_json::from_value::<ChartContent>(chart_data.clone()) {
                self.fill_chart(element, &chart, style);
            }
        }
    }
}

impl Default for ContentFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TextEffects {
    pub color: Option<String>,
    pub weight: Option<String>,
    pub size: Option<u32>,
    pub shadow: Option<Shadow>,
}

#[derive(Debug, Clone)]
pub struct ImageEffects {
    pub border_radius: Option<u32>,
    pub shadow: Option<Shadow>,
    pub opacity: Option<f32>,
    pub filter: Option<String>,
}
