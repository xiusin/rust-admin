use crate::common::error::Result;
use crate::domain::model::m_document::{ContentBlock, ParsedDocument, Section};
use crate::domain::model::m_style::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct LayoutEngine {
    page_width: u32,
    page_height: u32,
    grid_columns: u32,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            page_width: 1920,
            page_height: 1080,
            grid_columns: 12,
        }
    }

    pub fn with_dimensions(width: u32, height: u32) -> Self {
        Self {
            page_width: width,
            page_height: height,
            grid_columns: 12,
        }
    }

    pub fn calculate_element_position(
        &self,
        grid_x: u32,
        grid_y: u32,
        grid_width: u32,
        grid_height: u32,
    ) -> ElementPosition {
        let cell_width = self.page_width / self.grid_columns;
        let cell_height = self.page_height / 8;

        ElementPosition {
            x: grid_x * cell_width,
            y: grid_y * cell_height,
            width: grid_width * cell_width,
            height: grid_height * cell_height,
        }
    }

    pub fn calculate_center_position(&self, element_width: u32, element_height: u32) -> ElementPosition {
        ElementPosition {
            x: (self.page_width - element_width) / 2,
            y: (self.page_height - element_height) / 2,
            width: element_width,
            height: element_height,
        }
    }

    pub fn get_title_position(&self) -> ElementPosition {
        ElementPosition {
            x: 120,
            y: 400,
            width: self.page_width - 240,
            height: 120,
        }
    }

    pub fn get_subtitle_position(&self) -> ElementPosition {
        ElementPosition {
            x: 120,
            y: 540,
            width: self.page_width - 240,
            height: 60,
        }
    }

    pub fn get_content_position(&self, index: usize) -> ElementPosition {
        let start_y = 200u32;
        let spacing = 120u32;
        let item_height = 100u32;

        ElementPosition {
            x: 120,
            y: start_y + (index as u32 * (item_height + spacing)),
            width: self.page_width - 240,
            height: item_height,
        }
    }

    pub fn get_toc_item_position(&self, index: usize, total: usize) -> ElementPosition {
        let padding = 120u32;
        let available_height = self.page_height - 2 * padding;
        let item_height = (available_height / total as u32).min(80);

        ElementPosition {
            x: padding,
            y: padding + (index as u32 * item_height),
            width: self.page_width - 2 * padding,
            height: item_height,
        }
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPage {
    pub page_type: PageType,
    pub page_order: usize,
    pub elements: Vec<PageElement>,
    pub background: Option<Background>,
    pub notes: Option<String>,
    pub layout_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageType {
    Title,
    Toc,
    Content,
    Chart,
    Image,
    Table,
    End,
    Custom(String),
}

impl PageType {
    pub fn as_str(&self) -> &str {
        match self {
            PageType::Title => "title",
            PageType::Toc => "toc",
            PageType::Content => "content",
            PageType::Chart => "chart",
            PageType::Image => "image",
            PageType::Table => "table",
            PageType::End => "end",
            PageType::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "title" => PageType::Title,
            "toc" => PageType::Toc,
            "content" => PageType::Content,
            "chart" => PageType::Chart,
            "image" => PageType::Image,
            "table" => PageType::Table,
            "end" => PageType::End,
            other => PageType::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageElement {
    pub element_type: ElementType,
    pub content: ElementContent,
    pub style: ElementStyle,
    pub position: ElementPosition,
    pub rotation: Option<f32>,
    pub z_index: i32,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Text,
    Image,
    Shape,
    Chart,
    Table,
    Line,
    Group,
}

impl ElementType {
    pub fn as_str(&self) -> &str {
        match self {
            ElementType::Text => "text",
            ElementType::Image => "image",
            ElementType::Shape => "shape",
            ElementType::Chart => "chart",
            ElementType::Table => "table",
            ElementType::Line => "line",
            ElementType::Group => "group",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementContent {
    pub text: Option<TextContent>,
    pub image: Option<ImageContent>,
    pub chart: Option<ChartContent>,
    pub table: Option<TableContent>,
    pub shape: Option<ShapeContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    pub font_family: Option<String>,
    pub font_size: Option<u32>,
    pub font_weight: Option<String>,
    pub color: Option<String>,
    pub alignment: Option<String>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub url: Option<String>,
    pub data: Option<String>,
    pub alt_text: Option<String>,
    pub fit_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartContent {
    pub chart_type: String,
    pub title: Option<String>,
    pub data: ChartData,
    pub options: Option<ChartOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataset {
    pub label: String,
    pub data: Vec<f64>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartOptions {
    pub show_legend: Option<bool>,
    pub legend_position: Option<String>,
    pub show_grid: Option<bool>,
    pub animation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableContent {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub column_widths: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeContent {
    pub shape_type: String,
    pub fill_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<u32>,
    pub border_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStyle {
    pub opacity: Option<f32>,
    pub shadow: Option<Shadow>,
    pub border: Option<Border>,
    pub transform: Option<Transform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    pub width: u32,
    pub color: String,
    pub style: String,
    pub radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub skew_x: Option<f32>,
    pub skew_y: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub background_type: BackgroundType,
    pub color: Option<String>,
    pub gradient: Option<Gradient>,
    pub image: Option<BackgroundImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundType {
    Solid,
    Gradient,
    Image,
    Pattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub gradient_type: String,
    pub colors: Vec<GradientColorStop>,
    pub angle: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientColorStop {
    pub color: String,
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundImage {
    pub url: String,
    pub fit_mode: String,
    pub opacity: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleContent {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineItem {
    pub title: String,
    pub level: u8,
    pub page_number: Option<usize>,
    pub children: Vec<OutlineItem>,
}

pub struct PageGenerator {
    layout_engine: Arc<LayoutEngine>,
}

impl PageGenerator {
    pub fn new() -> Self {
        Self {
            layout_engine: Arc::new(LayoutEngine::new()),
        }
    }

    pub fn with_layout_engine(layout_engine: Arc<LayoutEngine>) -> Self {
        Self { layout_engine }
    }

    pub async fn generate_title_page(
        &self,
        content: &TitleContent,
        style: &StyleParams,
    ) -> Result<GeneratedPage> {
        let mut elements = Vec::new();

        let title_position = self.layout_engine.get_title_position();
        let title_element = PageElement {
            element_type: ElementType::Text,
            content: ElementContent {
                text: Some(TextContent {
                    text: content.title.clone(),
                    font_family: Some(style.font_scheme.title_font.family.clone()),
                    font_size: Some(style.font_scheme.title_font.size),
                    font_weight: Some(style.font_scheme.title_font.weight.clone()),
                    color: Some(style.color_scheme.primary_color.clone()),
                    alignment: Some(style.component_styles.title_style.alignment.clone()),
                    line_height: Some(style.font_scheme.title_font.line_height),
                    letter_spacing: Some(style.font_scheme.title_font.letter_spacing),
                }),
                image: None,
                chart: None,
                table: None,
                shape: None,
            },
            style: ElementStyle {
                opacity: Some(1.0),
                shadow: Some(Shadow::small()),
                border: None,
                transform: None,
            },
            position: title_position,
            rotation: None,
            z_index: 1,
            locked: false,
        };
        elements.push(title_element);

        if let Some(ref subtitle) = content.subtitle {
            let subtitle_position = self.layout_engine.get_subtitle_position();
            let subtitle_element = PageElement {
                element_type: ElementType::Text,
                content: ElementContent {
                    text: Some(TextContent {
                        text: subtitle.clone(),
                        font_family: Some(style.font_scheme.subtitle_font.family.clone()),
                        font_size: Some(style.font_scheme.subtitle_font.size),
                        font_weight: Some(style.font_scheme.subtitle_font.weight.clone()),
                        color: Some(style.color_scheme.text_secondary_color.clone()),
                        alignment: Some(style.component_styles.title_style.alignment.clone()),
                        line_height: Some(style.font_scheme.subtitle_font.line_height),
                        letter_spacing: Some(style.font_scheme.subtitle_font.letter_spacing),
                    }),
                    image: None,
                    chart: None,
                    table: None,
                    shape: None,
                },
                style: ElementStyle {
                    opacity: Some(1.0),
                    shadow: None,
                    border: None,
                    transform: None,
                },
                position: subtitle_position,
                rotation: None,
                z_index: 2,
                locked: false,
            };
            elements.push(subtitle_element);
        }

        let mut info_y = 700u32;
        if let Some(ref author) = content.author {
            let author_position = ElementPosition {
                x: 120,
                y: info_y,
                width: self.layout_engine.page_width - 240,
                height: 40,
            };
            let author_element = PageElement {
                element_type: ElementType::Text,
                content: ElementContent {
                    text: Some(TextContent {
                        text: format!("作者: {}", author),
                        font_family: Some(style.font_scheme.body_font.family.clone()),
                        font_size: Some(style.font_scheme.body_font.size),
                        font_weight: Some("normal".to_string()),
                        color: Some(style.color_scheme.text_color.clone()),
                        alignment: Some("center".to_string()),
                        line_height: Some(style.font_scheme.body_font.line_height),
                        letter_spacing: None,
                    }),
                    image: None,
                    chart: None,
                    table: None,
                    shape: None,
                },
                style: ElementStyle {
                    opacity: Some(0.8),
                    shadow: None,
                    border: None,
                    transform: None,
                },
                position: author_position,
                rotation: None,
                z_index: 3,
                locked: false,
            };
            elements.push(author_element);
            info_y += 50;
        }

        if let Some(ref date) = content.date {
            let date_position = ElementPosition {
                x: 120,
                y: info_y,
                width: self.layout_engine.page_width - 240,
                height: 40,
            };
            let date_element = PageElement {
                element_type: ElementType::Text,
                content: ElementContent {
                    text: Some(TextContent {
                        text: date.clone(),
                        font_family: Some(style.font_scheme.caption_font.family.clone()),
                        font_size: Some(style.font_scheme.caption_font.size),
                        font_weight: Some("normal".to_string()),
                        color: Some(style.color_scheme.text_secondary_color.clone()),
                        alignment: Some("center".to_string()),
                        line_height: Some(style.font_scheme.caption_font.line_height),
                        letter_spacing: None,
                    }),
                    image: None,
                    chart: None,
                    table: None,
                    shape: None,
                },
                style: ElementStyle {
                    opacity: Some(0.6),
                    shadow: None,
                    border: None,
                    transform: None,
                },
                position: date_position,
                rotation: None,
                z_index: 4,
                locked: false,
            };
            elements.push(date_element);
        }

        let background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(style.color_scheme.background_color.clone()),
            gradient: None,
            image: None,
        });

        Ok(GeneratedPage {
            page_type: PageType::Title,
            page_order: 0,
            elements,
            background,
            notes: None,
            layout_config: None,
        })
    }

    pub async fn generate_toc_page(
        &self,
        outline: &[OutlineItem],
        style: &StyleParams,
    ) -> Result<GeneratedPage> {
        let mut elements = Vec::new();

        let title_position = ElementPosition {
            x: 120,
            y: 80,
            width: self.layout_engine.page_width - 240,
            height: 80,
        };
        let title_element = PageElement {
            element_type: ElementType::Text,
            content: ElementContent {
                text: Some(TextContent {
                    text: "目录".to_string(),
                    font_family: Some(style.font_scheme.title_font.family.clone()),
                    font_size: Some(style.font_scheme.title_font.size),
                    font_weight: Some(style.font_scheme.title_font.weight.clone()),
                    color: Some(style.color_scheme.primary_color.clone()),
                    alignment: Some("center".to_string()),
                    line_height: Some(style.font_scheme.title_font.line_height),
                    letter_spacing: None,
                }),
                image: None,
                chart: None,
                table: None,
                shape: None,
            },
            style: ElementStyle {
                opacity: Some(1.0),
                shadow: None,
                border: None,
                transform: None,
            },
            position: title_position,
            rotation: None,
            z_index: 1,
            locked: false,
        };
        elements.push(title_element);

        let flat_items = self.flatten_outline(outline);
        let total_items = flat_items.len();

        for (index, item) in flat_items.iter().enumerate() {
            let item_position = self.layout_engine.get_toc_item_position(index, total_items);

            let indent = item.level as u32 * 40;
            let adjusted_position = ElementPosition {
                x: item_position.x + indent,
                y: item_position.y,
                width: item_position.width - indent,
                height: item_position.height,
            };

            let font_size = match item.level {
                0 => style.font_scheme.subtitle_font.size,
                1 => style.font_scheme.body_font.size,
                _ => style.font_scheme.caption_font.size,
            };

            let text = if let Some(page_num) = item.page_number {
                format!("{} .................. {}", item.title, page_num)
            } else {
                item.title.clone()
            };

            let toc_element = PageElement {
                element_type: ElementType::Text,
                content: ElementContent {
                    text: Some(TextContent {
                        text,
                        font_family: Some(style.font_scheme.body_font.family.clone()),
                        font_size: Some(font_size),
                        font_weight: Some(if item.level == 0 { "bold" } else { "normal" }.to_string()),
                        color: Some(style.color_scheme.text_color.clone()),
                        alignment: Some("left".to_string()),
                        line_height: Some(1.5),
                        letter_spacing: None,
                    }),
                    image: None,
                    chart: None,
                    table: None,
                    shape: None,
                },
                style: ElementStyle {
                    opacity: Some(1.0),
                    shadow: None,
                    border: None,
                    transform: None,
                },
                position: adjusted_position,
                rotation: None,
                z_index: 2 + index as i32,
                locked: false,
            };
            elements.push(toc_element);
        }

        let background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(style.color_scheme.background_color.clone()),
            gradient: None,
            image: None,
        });

        Ok(GeneratedPage {
            page_type: PageType::Toc,
            page_order: 1,
            elements,
            background,
            notes: None,
            layout_config: None,
        })
    }

    pub async fn generate_content_page(
        &self,
        section: &Section,
        style: &StyleParams,
        page_index: usize,
    ) -> Result<GeneratedPage> {
        let mut elements = Vec::new();

        let title_position = ElementPosition {
            x: style.layout_rules.page_margin.left,
            y: style.layout_rules.page_margin.top,
            width: self.layout_engine.page_width
                - style.layout_rules.page_margin.left
                - style.layout_rules.page_margin.right,
            height: 80,
        };
        let title_element = PageElement {
            element_type: ElementType::Text,
            content: ElementContent {
                text: Some(TextContent {
                    text: section.title.clone(),
                    font_family: Some(style.font_scheme.subtitle_font.family.clone()),
                    font_size: Some(style.font_scheme.subtitle_font.size),
                    font_weight: Some(style.font_scheme.subtitle_font.weight.clone()),
                    color: Some(style.color_scheme.primary_color.clone()),
                    alignment: Some("left".to_string()),
                    line_height: Some(style.font_scheme.subtitle_font.line_height),
                    letter_spacing: None,
                }),
                image: None,
                chart: None,
                table: None,
                shape: None,
            },
            style: ElementStyle {
                opacity: Some(1.0),
                shadow: None,
                border: None,
                transform: None,
            },
            position: title_position,
            rotation: None,
            z_index: 1,
            locked: false,
        };
        elements.push(title_element);

        let mut content_y = style.layout_rules.page_margin.top + 120;
        let content_width = self.layout_engine.page_width
            - style.layout_rules.page_margin.left
            - style.layout_rules.page_margin.right;
        let available_height = self.layout_engine.page_height
            - content_y
            - style.layout_rules.page_margin.bottom;

        for (block_index, block) in section.content.iter().enumerate() {
            match block {
                ContentBlock::Text(text_block) => {
                    let text_height = self.calculate_text_height(&text_block.text, content_width, style);
                    
                    if content_y + text_height > self.layout_engine.page_height - style.layout_rules.page_margin.bottom {
                        break;
                    }

                    let text_position = ElementPosition {
                        x: style.layout_rules.page_margin.left,
                        y: content_y,
                        width: content_width,
                        height: text_height,
                    };

                    let text_element = PageElement {
                        element_type: ElementType::Text,
                        content: ElementContent {
                            text: Some(TextContent {
                                text: text_block.text.clone(),
                                font_family: Some(style.font_scheme.body_font.family.clone()),
                                font_size: Some(style.font_scheme.body_font.size),
                                font_weight: Some("normal".to_string()),
                                color: Some(style.color_scheme.text_color.clone()),
                                alignment: Some(style.component_styles.text_style.alignment.clone()),
                                line_height: Some(style.font_scheme.body_font.line_height),
                                letter_spacing: Some(style.font_scheme.body_font.letter_spacing),
                            }),
                            image: None,
                            chart: None,
                            table: None,
                            shape: None,
                        },
                        style: ElementStyle {
                            opacity: Some(1.0),
                            shadow: None,
                            border: None,
                            transform: None,
                        },
                        position: text_position,
                        rotation: None,
                        z_index: 2 + block_index as i32,
                        locked: false,
                    };
                    elements.push(text_element);
                    content_y += text_height + style.layout_rules.element_spacing.medium;
                }
                ContentBlock::Image(image_block) => {
                    let image_height = image_block.height.unwrap_or(400).min(available_height);
                    
                    let image_position = ElementPosition {
                        x: style.layout_rules.page_margin.left,
                        y: content_y,
                        width: image_block.width.unwrap_or(content_width),
                        height: image_height,
                    };

                    let image_element = PageElement {
                        element_type: ElementType::Image,
                        content: ElementContent {
                            text: None,
                            image: Some(ImageContent {
                                url: image_block.url.clone(),
                                data: image_block.data.clone(),
                                alt_text: image_block.alt_text.clone(),
                                fit_mode: Some("contain".to_string()),
                            }),
                            chart: None,
                            table: None,
                            shape: None,
                        },
                        style: ElementStyle {
                            opacity: Some(1.0),
                            shadow: style.component_styles.image_style.shadow.clone(),
                            border: Some(Border {
                                width: 0,
                                color: "transparent".to_string(),
                                style: "solid".to_string(),
                                radius: Some(style.component_styles.image_style.border_radius),
                            }),
                            transform: None,
                        },
                        position: image_position,
                        rotation: None,
                        z_index: 2 + block_index as i32,
                        locked: false,
                    };
                    elements.push(image_element);
                    content_y += image_height + style.layout_rules.element_spacing.medium;
                }
                ContentBlock::Table(table_block) => {
                    let table_height = self.calculate_table_height(&table_block.rows);
                    
                    let table_position = ElementPosition {
                        x: style.layout_rules.page_margin.left,
                        y: content_y,
                        width: content_width,
                        height: table_height,
                    };

                    let headers = table_block.headers.clone().unwrap_or_default();
                    let rows: Vec<Vec<String>> = table_block.rows.iter()
                        .map(|row| row.cells.clone())
                        .collect();

                    let table_element = PageElement {
                        element_type: ElementType::Table,
                        content: ElementContent {
                            text: None,
                            image: None,
                            chart: None,
                            table: Some(TableContent {
                                headers,
                                rows,
                                column_widths: None,
                            }),
                            shape: None,
                        },
                        style: ElementStyle {
                            opacity: Some(1.0),
                            shadow: None,
                            border: Some(Border {
                                width: style.component_styles.table_style.border_style.width,
                                color: style.component_styles.table_style.border_style.color.clone(),
                                style: style.component_styles.table_style.border_style.style.clone(),
                                radius: None,
                            }),
                            transform: None,
                        },
                        position: table_position,
                        rotation: None,
                        z_index: 2 + block_index as i32,
                        locked: false,
                    };
                    elements.push(table_element);
                    content_y += table_height + style.layout_rules.element_spacing.medium;
                }
                ContentBlock::List(list_block) => {
                    let list_text = list_block.items.iter()
                        .map(|item| {
                            let indent = "  ".repeat(item.level as usize);
                            let bullet = if list_block.ordered { "•" } else { "•" };
                            format!("{}{} {}", indent, bullet, item.text)
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    let list_height = self.calculate_text_height(&list_text, content_width, style);
                    
                    let list_position = ElementPosition {
                        x: style.layout_rules.page_margin.left,
                        y: content_y,
                        width: content_width,
                        height: list_height,
                    };

                    let list_element = PageElement {
                        element_type: ElementType::Text,
                        content: ElementContent {
                            text: Some(TextContent {
                                text: list_text,
                                font_family: Some(style.font_scheme.body_font.family.clone()),
                                font_size: Some(style.font_scheme.body_font.size),
                                font_weight: Some("normal".to_string()),
                                color: Some(style.color_scheme.text_color.clone()),
                                alignment: Some("left".to_string()),
                                line_height: Some(style.font_scheme.body_font.line_height),
                                letter_spacing: None,
                            }),
                            image: None,
                            chart: None,
                            table: None,
                            shape: None,
                        },
                        style: ElementStyle {
                            opacity: Some(1.0),
                            shadow: None,
                            border: None,
                            transform: None,
                        },
                        position: list_position,
                        rotation: None,
                        z_index: 2 + block_index as i32,
                        locked: false,
                    };
                    elements.push(list_element);
                    content_y += list_height + style.layout_rules.element_spacing.medium;
                }
                ContentBlock::Code(code_block) => {
                    let code_text = format!("```\n{}\n```", code_block.code);
                    let code_height = self.calculate_text_height(&code_text, content_width, style);
                    
                    let code_position = ElementPosition {
                        x: style.layout_rules.page_margin.left,
                        y: content_y,
                        width: content_width,
                        height: code_height,
                    };

                    let code_element = PageElement {
                        element_type: ElementType::Text,
                        content: ElementContent {
                            text: Some(TextContent {
                                text: code_text,
                                font_family: Some("Courier New".to_string()),
                                font_size: Some(style.font_scheme.body_font.size - 2),
                                font_weight: Some("normal".to_string()),
                                color: Some("#2C3E50".to_string()),
                                alignment: Some("left".to_string()),
                                line_height: Some(1.4),
                                letter_spacing: None,
                            }),
                            image: None,
                            chart: None,
                            table: None,
                            shape: None,
                        },
                        style: ElementStyle {
                            opacity: Some(1.0),
                            shadow: None,
                            border: Some(Border {
                                width: 1,
                                color: "#E5E5E5".to_string(),
                                style: "solid".to_string(),
                                radius: Some(4),
                            }),
                            transform: None,
                        },
                        position: code_position,
                        rotation: None,
                        z_index: 2 + block_index as i32,
                        locked: false,
                    };
                    elements.push(code_element);
                    content_y += code_height + style.layout_rules.element_spacing.medium;
                }
            }
        }

        let background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(style.color_scheme.background_color.clone()),
            gradient: None,
            image: None,
        });

        Ok(GeneratedPage {
            page_type: PageType::Content,
            page_order: page_index,
            elements,
            background,
            notes: None,
            layout_config: None,
        })
    }

    pub async fn generate_chart_page(
        &self,
        chart_data: &ChartContent,
        style: &StyleParams,
        page_index: usize,
    ) -> Result<GeneratedPage> {
        let mut elements = Vec::new();

        if let Some(ref title) = chart_data.title {
            let title_position = ElementPosition {
                x: style.layout_rules.page_margin.left,
                y: style.layout_rules.page_margin.top,
                width: self.layout_engine.page_width
                    - style.layout_rules.page_margin.left
                    - style.layout_rules.page_margin.right,
                height: 60,
            };
            let title_element = PageElement {
                element_type: ElementType::Text,
                content: ElementContent {
                    text: Some(TextContent {
                        text: title.clone(),
                        font_family: Some(style.font_scheme.subtitle_font.family.clone()),
                        font_size: Some(style.font_scheme.subtitle_font.size),
                        font_weight: Some(style.font_scheme.subtitle_font.weight.clone()),
                        color: Some(style.color_scheme.primary_color.clone()),
                        alignment: Some("center".to_string()),
                        line_height: Some(style.font_scheme.subtitle_font.line_height),
                        letter_spacing: None,
                    }),
                    image: None,
                    chart: None,
                    table: None,
                    shape: None,
                },
                style: ElementStyle {
                    opacity: Some(1.0),
                    shadow: None,
                    border: None,
                    transform: None,
                },
                position: title_position,
                rotation: None,
                z_index: 1,
                locked: false,
            };
            elements.push(title_element);
        }

        let chart_position = ElementPosition {
            x: style.layout_rules.page_margin.left,
            y: style.layout_rules.page_margin.top + 100,
            width: self.layout_engine.page_width
                - style.layout_rules.page_margin.left
                - style.layout_rules.page_margin.right,
            height: self.layout_engine.page_height
                - style.layout_rules.page_margin.top
                - style.layout_rules.page_margin.bottom
                - 150,
        };

        let chart_element = PageElement {
            element_type: ElementType::Chart,
            content: ElementContent {
                text: None,
                image: None,
                chart: Some(chart_data.clone()),
                table: None,
                shape: None,
            },
            style: ElementStyle {
                opacity: Some(1.0),
                shadow: Some(Shadow::medium()),
                border: Some(Border {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                    radius: Some(style.component_styles.chart_style.border_radius),
                }),
                transform: None,
            },
            position: chart_position,
            rotation: None,
            z_index: 2,
            locked: false,
        };
        elements.push(chart_element);

        let background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(style.color_scheme.background_color.clone()),
            gradient: None,
            image: None,
        });

        Ok(GeneratedPage {
            page_type: PageType::Chart,
            page_order: page_index,
            elements,
            background,
            notes: None,
            layout_config: None,
        })
    }

    pub async fn generate_end_page(
        &self,
        style: &StyleParams,
        page_index: usize,
    ) -> Result<GeneratedPage> {
        let mut elements = Vec::new();

        let center_position = self.layout_engine.calculate_center_position(400, 100);
        let thank_element = PageElement {
            element_type: ElementType::Text,
            content: ElementContent {
                text: Some(TextContent {
                    text: "谢谢观看".to_string(),
                    font_family: Some(style.font_scheme.title_font.family.clone()),
                    font_size: Some(style.font_scheme.title_font.size),
                    font_weight: Some(style.font_scheme.title_font.weight.clone()),
                    color: Some(style.color_scheme.primary_color.clone()),
                    alignment: Some("center".to_string()),
                    line_height: Some(style.font_scheme.title_font.line_height),
                    letter_spacing: None,
                }),
                image: None,
                chart: None,
                table: None,
                shape: None,
            },
            style: ElementStyle {
                opacity: Some(1.0),
                shadow: Some(Shadow::medium()),
                border: None,
                transform: None,
            },
            position: center_position,
            rotation: None,
            z_index: 1,
            locked: false,
        };
        elements.push(thank_element);

        let background = Some(Background {
            background_type: BackgroundType::Solid,
            color: Some(style.color_scheme.background_color.clone()),
            gradient: None,
            image: None,
        });

        Ok(GeneratedPage {
            page_type: PageType::End,
            page_order: page_index,
            elements,
            background,
            notes: None,
            layout_config: None,
        })
    }

    pub async fn generate_all_pages(
        &self,
        document: &ParsedDocument,
        style: &StyleParams,
        include_toc: bool,
        include_end: bool,
    ) -> Result<Vec<GeneratedPage>> {
        let mut pages = Vec::new();
        let mut page_index = 0;

        let title_content = TitleContent {
            title: document.title.clone(),
            subtitle: None,
            author: document.metadata.author.clone(),
            date: document.metadata.created_at.clone(),
        };
        let title_page = self.generate_title_page(&title_content, style).await?;
        pages.push(title_page);
        page_index += 1;

        if include_toc {
            let outline = self.build_outline_from_document(document);
            let toc_page = self.generate_toc_page(&outline, style).await?;
            pages.push(GeneratedPage {
                page_order: page_index,
                ..toc_page
            });
            page_index += 1;
        }

        for section in &document.sections {
            let content_page = self.generate_content_page(section, style, page_index).await?;
            pages.push(content_page);
            page_index += 1;
        }

        if include_end {
            let end_page = self.generate_end_page(style, page_index).await?;
            pages.push(end_page);
        }

        Ok(pages)
    }

    fn flatten_outline(&self, outline: &[OutlineItem]) -> Vec<OutlineItem> {
        let mut result = Vec::new();
        for item in outline {
            result.push(item.clone());
            result.extend(self.flatten_outline(&item.children));
        }
        result
    }

    fn calculate_text_height(&self, text: &str, width: u32, style: &StyleParams) -> u32 {
        let font_size = style.font_scheme.body_font.size;
        let line_height = style.font_scheme.body_font.line_height;
        
        let chars_per_line = (width as f32 / (font_size as f32 * 0.6)) as usize;
        let line_count = (text.len() / chars_per_line.max(1)).max(1);
        
        (line_count as f32 * font_size as f32 * line_height) as u32
    }

    fn calculate_table_height(&self, rows: &[crate::domain::model::m_document::TableRow]) -> u32 {
        let row_height = 40u32;
        let header_height = 50u32;
        header_height + (rows.len() as u32 * row_height)
    }

    fn build_outline_from_document(&self, document: &ParsedDocument) -> Vec<OutlineItem> {
        document.sections.iter().enumerate().map(|(index, section)| {
            OutlineItem {
                title: section.title.clone(),
                level: section.level,
                page_number: Some(index + 2),
                children: Vec::new(),
            }
        }).collect()
    }
}

impl Default for PageGenerator {
    fn default() -> Self {
        Self::new()
    }
}
