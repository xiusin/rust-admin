use crate::domain::model::m_style::*;
use super::page_generator::{
    GeneratedPage, Background, BackgroundType, 
    Gradient, GradientColorStop,
};

pub struct StyleApplier;

impl StyleApplier {
    pub fn new() -> Self {
        Self
    }

    pub fn apply_color_scheme(
        &self,
        page: &mut GeneratedPage,
        scheme: &ColorScheme,
    ) {
        if let Some(ref mut background) = page.background {
            background.color = Some(scheme.background_color.clone());
        }

        for element in page.elements.iter_mut() {
            if let Some(ref mut text) = element.content.text {
                if text.color.as_deref() == Some("primary") {
                    text.color = Some(scheme.primary_color.clone());
                } else if text.color.as_deref() == Some("secondary") {
                    text.color = Some(scheme.secondary_color.clone());
                } else if text.color.as_deref() == Some("accent") {
                    text.color = Some(scheme.accent_color.clone());
                } else if text.color.as_deref() == Some("text") {
                    text.color = Some(scheme.text_color.clone());
                } else if text.color.as_deref() == Some("text_secondary") {
                    text.color = Some(scheme.text_secondary_color.clone());
                }
            }

            if let Some(ref mut shape) = element.content.shape {
                if shape.fill_color.as_deref() == Some("primary") {
                    shape.fill_color = Some(scheme.primary_color.clone());
                } else if shape.fill_color.as_deref() == Some("secondary") {
                    shape.fill_color = Some(scheme.secondary_color.clone());
                }
            }

            if let Some(ref mut chart) = element.content.chart {
                for (idx, dataset) in chart.data.datasets.iter_mut().enumerate() {
                    if dataset.color.is_none() {
                        let color_idx = idx % scheme.color_palette.len();
                        dataset.color = Some(scheme.color_palette[color_idx].clone());
                    }
                }
            }
        }
    }

    pub fn apply_font_scheme(
        &self,
        page: &mut GeneratedPage,
        scheme: &FontScheme,
    ) {
        for element in page.elements.iter_mut() {
            if let Some(ref mut text) = element.content.text {
                if text.font_family.is_none() {
                    text.font_family = Some(scheme.body_font.family.clone());
                }

                if text.font_size.is_none() {
                    text.font_size = Some(scheme.body_font.size);
                }

                if text.font_weight.is_none() {
                    text.font_weight = Some(scheme.body_font.weight.clone());
                }

                if text.line_height.is_none() {
                    text.line_height = Some(scheme.body_font.line_height);
                }

                if text.letter_spacing.is_none() {
                    text.letter_spacing = Some(scheme.body_font.letter_spacing);
                }
            }
        }
    }

    pub fn apply_layout_rules(
        &self,
        page: &mut GeneratedPage,
        rules: &LayoutRules,
    ) {
        let page_width = 1920u32;
        let page_height = 1080u32;
        let content_width = page_width - rules.page_margin.left - rules.page_margin.right;

        for element in page.elements.iter_mut() {
            if element.position.x < rules.page_margin.left {
                element.position.x = rules.page_margin.left;
            }
            if element.position.y < rules.page_margin.top {
                element.position.y = rules.page_margin.top;
            }

            if element.position.x + element.position.width > page_width - rules.page_margin.right {
                element.position.width = content_width;
            }

            if element.position.y + element.position.height > page_height - rules.page_margin.bottom {
                element.position.height = page_height - rules.page_margin.bottom - element.position.y;
            }
        }
    }

    pub fn apply_component_styles(
        &self,
        page: &mut GeneratedPage,
        styles: &ComponentStyles,
    ) {
        for element in page.elements.iter_mut() {
            match element.element_type {
                super::page_generator::ElementType::Text => {
                    if let Some(ref mut text) = element.content.text {
                        if text.alignment.is_none() {
                            text.alignment = Some(styles.text_style.alignment.clone());
                        }
                    }
                }
                super::page_generator::ElementType::Image => {
                    element.style.border = Some(super::page_generator::Border {
                        width: 0,
                        color: "transparent".to_string(),
                        style: "solid".to_string(),
                        radius: Some(styles.image_style.border_radius),
                    });
                    element.style.shadow = styles.image_style.shadow.clone();
                }
                super::page_generator::ElementType::Shape => {
                    if let Some(ref mut shape) = element.content.shape {
                        shape.border_radius = Some(styles.shape_style.border_radius);
                        shape.border_width = Some(styles.shape_style.border_width);
                        shape.border_color = Some(styles.shape_style.border_color.clone());
                    }
                    element.style.shadow = styles.shape_style.shadow.clone();
                }
                super::page_generator::ElementType::Chart => {
                    element.style.border = Some(super::page_generator::Border {
                        width: 1,
                        color: "#E5E5E5".to_string(),
                        style: "solid".to_string(),
                        radius: Some(styles.chart_style.border_radius),
                    });

                    if let Some(ref mut chart) = element.content.chart {
                        if chart.options.is_none() {
                            chart.options = Some(super::page_generator::ChartOptions {
                                show_legend: Some(styles.chart_style.show_legend),
                                legend_position: Some(styles.chart_style.legend_position.clone()),
                                show_grid: Some(styles.chart_style.show_grid),
                                animation: Some(true),
                            });
                        }
                    }
                }
                super::page_generator::ElementType::Table => {
                    element.style.border = Some(super::page_generator::Border {
                        width: styles.table_style.border_style.width,
                        color: styles.table_style.border_style.color.clone(),
                        style: styles.table_style.border_style.style.clone(),
                        radius: None,
                    });
                }
                _ => {}
            }
        }
    }

    pub fn apply_all_styles(
        &self,
        page: &mut GeneratedPage,
        style: &StyleParams,
    ) {
        self.apply_color_scheme(page, &style.color_scheme);
        self.apply_font_scheme(page, &style.font_scheme);
        self.apply_layout_rules(page, &style.layout_rules);
        self.apply_component_styles(page, &style.component_styles);
    }

    pub fn apply_gradient_background(
        &self,
        page: &mut GeneratedPage,
        colors: &[String],
        angle: Option<f32>,
    ) {
        if colors.len() < 2 {
            return;
        }

        let gradient = Gradient {
            gradient_type: "linear".to_string(),
            colors: colors.iter().enumerate().map(|(idx, color)| {
                GradientColorStop {
                    color: color.clone(),
                    position: idx as f32 / (colors.len() - 1) as f32,
                }
            }).collect(),
            angle,
        };

        page.background = Some(Background {
            background_type: BackgroundType::Gradient,
            color: None,
            gradient: Some(gradient),
            image: None,
        });
    }

    pub fn apply_solid_background(
        &self,
        page: &mut GeneratedPage,
        color: &str,
    ) {
        page.background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(color.to_string()),
            gradient: None,
            image: None,
        });
    }

    pub fn apply_image_background(
        &self,
        page: &mut GeneratedPage,
        url: &str,
        fit_mode: &str,
        opacity: Option<f32>,
    ) {
        page.background = Some(Background {
            background_type: BackgroundType::Image,
            color: None,
            gradient: None,
            image: Some(super::page_generator::BackgroundImage {
                url: url.to_string(),
                fit_mode: fit_mode.to_string(),
                opacity,
            }),
        });
    }

    pub fn apply_element_spacing(
        &self,
        page: &mut GeneratedPage,
        spacing: &Spacing,
    ) {
        let mut last_y = 0u32;
        let mut last_height = 0u32;

        for element in page.elements.iter_mut() {
            if element.position.y > last_y + last_height {
                let gap = element.position.y - last_y - last_height;
                if gap < spacing.small {
                    element.position.y = last_y + last_height + spacing.small;
                }
            }
            last_y = element.position.y;
            last_height = element.position.height;
        }
    }

    pub fn apply_consistent_alignment(
        &self,
        page: &mut GeneratedPage,
        alignment: &str,
    ) {
        for element in page.elements.iter_mut() {
            if let Some(ref mut text) = element.content.text {
                text.alignment = Some(alignment.to_string());
            }
        }
    }

    pub fn apply_color_to_elements(
        &self,
        page: &mut GeneratedPage,
        color: &str,
        element_types: &[&str],
    ) {
        for element in page.elements.iter_mut() {
            let type_str = element.element_type.as_str();
            if element_types.contains(&type_str) {
                if let Some(ref mut text) = element.content.text {
                    text.color = Some(color.to_string());
                }
                if let Some(ref mut shape) = element.content.shape {
                    shape.fill_color = Some(color.to_string());
                }
            }
        }
    }

    pub fn adjust_element_positions(
        &self,
        page: &mut GeneratedPage,
        offset_x: i32,
        offset_y: i32,
    ) {
        for element in page.elements.iter_mut() {
            if offset_x > 0 {
                element.position.x = element.position.x.saturating_add(offset_x as u32);
            } else {
                element.position.x = element.position.x.saturating_sub((-offset_x) as u32);
            }

            if offset_y > 0 {
                element.position.y = element.position.y.saturating_add(offset_y as u32);
            } else {
                element.position.y = element.position.y.saturating_sub((-offset_y) as u32);
            }
        }
    }

    pub fn scale_elements(
        &self,
        page: &mut GeneratedPage,
        scale_factor: f32,
    ) {
        for element in page.elements.iter_mut() {
            element.position.x = (element.position.x as f32 * scale_factor) as u32;
            element.position.y = (element.position.y as f32 * scale_factor) as u32;
            element.position.width = (element.position.width as f32 * scale_factor) as u32;
            element.position.height = (element.position.height as f32 * scale_factor) as u32;

            if let Some(ref mut text) = element.content.text {
                if let Some(size) = text.font_size {
                    text.font_size = Some((size as f32 * scale_factor) as u32);
                }
            }
        }
    }

    pub fn apply_theme(
        &self,
        page: &mut GeneratedPage,
        theme: &str,
        style: &StyleParams,
    ) {
        match theme {
            "minimal" => {
                self.apply_solid_background(page, "#FFFFFF");
                self.apply_consistent_alignment(page, "left");
            }
            "modern" => {
                self.apply_gradient_background(
                    page,
                    &[
                        style.color_scheme.primary_color.clone(),
                        style.color_scheme.secondary_color.clone(),
                    ],
                    Some(135.0),
                );
            }
            "professional" => {
                self.apply_solid_background(page, &style.color_scheme.background_color);
                self.apply_consistent_alignment(page, "center");
            }
            "creative" => {
                if let Some(ref colors) = style.color_scheme.gradient_colors {
                    self.apply_gradient_background(page, colors, Some(45.0));
                }
            }
            _ => {
                self.apply_all_styles(page, style);
            }
        }
    }
}

impl Default for StyleApplier {
    fn default() -> Self {
        Self::new()
    }
}
