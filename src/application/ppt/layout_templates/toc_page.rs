use super::*;
use crate::domain::model::m_style::{Position, StyleParams};

pub struct TocPageTemplate {
    layout_variants: Vec<TocLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocLayout {
    pub name: String,
    pub title_position: Position,
    pub content_start_position: Position,
    pub item_height: u32,
    pub item_spacing: u32,
    pub columns: u32,
    pub background_type: BackgroundType,
    pub alignment: String,
}

impl TocPageTemplate {
    pub fn new() -> Self {
        Self {
            layout_variants: Self::create_default_layouts(),
        }
    }

    fn create_default_layouts() -> Vec<TocLayout> {
        vec![
            TocLayout {
                name: "single_column".to_string(),
                title_position: Position { x: 960, y: 100 },
                content_start_position: Position { x: 300, y: 200 },
                item_height: 60,
                item_spacing: 20,
                columns: 1,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                alignment: "left".to_string(),
            },
            TocLayout {
                name: "two_column".to_string(),
                title_position: Position { x: 960, y: 100 },
                content_start_position: Position { x: 200, y: 200 },
                item_height: 50,
                item_spacing: 15,
                columns: 2,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                alignment: "left".to_string(),
            },
            TocLayout {
                name: "three_column".to_string(),
                title_position: Position { x: 960, y: 80 },
                content_start_position: Position { x: 150, y: 180 },
                item_height: 45,
                item_spacing: 12,
                columns: 3,
                background_type: BackgroundType::Solid("#F9F9F9".to_string()),
                alignment: "left".to_string(),
            },
            TocLayout {
                name: "card_style".to_string(),
                title_position: Position { x: 960, y: 100 },
                content_start_position: Position { x: 200, y: 220 },
                item_height: 80,
                item_spacing: 20,
                columns: 2,
                background_type: BackgroundType::Gradient {
                    start_color: "#FFFFFF".to_string(),
                    end_color: "#F0F0F0".to_string(),
                    angle: 180,
                },
                alignment: "center".to_string(),
            },
            TocLayout {
                name: "numbered_list".to_string(),
                title_position: Position { x: 960, y: 100 },
                content_start_position: Position { x: 400, y: 200 },
                item_height: 55,
                item_spacing: 18,
                columns: 1,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                alignment: "left".to_string(),
            },
        ]
    }

    pub fn get_layout(&self, item_count: usize, _style: &StyleParams) -> TocLayout {
        let layout_name = if item_count <= 5 {
            "single_column"
        } else if item_count <= 10 {
            "two_column"
        } else if item_count <= 15 {
            "three_column"
        } else if item_count <= 20 {
            "card_style"
        } else {
            "numbered_list"
        };

        self.layout_variants
            .iter()
            .find(|l| l.name == layout_name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone())
    }

    pub fn apply_content(&self, layout: &TocLayout, content: &TocContent) -> Vec<PageElement> {
        let mut elements = Vec::new();
        let mut z_index = 0;

        if let Some(title) = &content.title {
            elements.push(PageElement {
                element_type: "text".to_string(),
                content: ElementContent::Text {
                    text: title.clone(),
                    font_size: Some(36),
                    font_weight: Some("bold".to_string()),
                },
                position: layout.title_position.clone(),
                size: Size::new(600, 60),
                style: Some(ElementStyle {
                    color: Some("#333333".to_string()),
                    background_color: None,
                    border_radius: None,
                    opacity: None,
                    shadow: None,
                }),
                z_index,
            });
            z_index += 1;
        }

        let page_width = 1920u32;
        let column_width = if layout.columns > 0 {
            (page_width - 400) / layout.columns
        } else {
            page_width - 400
        };

        for (idx, item) in content.items.iter().enumerate() {
            let column = if layout.columns > 0 {
                idx % layout.columns as usize
            } else {
                0
            };
            let row = if layout.columns > 0 {
                idx / layout.columns as usize
            } else {
                idx
            };

            let x = layout.content_start_position.x 
                + (column as i32) * (column_width as i32 + 50);
            let y = layout.content_start_position.y 
                + (row as i32) * ((layout.item_height + layout.item_spacing) as i32);

            let mut item_text = item.title.clone();
            if let Some(page_num) = item.page_number {
                if layout.name == "numbered_list" {
                    item_text = format!("{}. {} ... {}", idx + 1, item.title, page_num);
                } else {
                    item_text = format!("{} ... {}", item.title, page_num);
                }
            }

            let font_size = match item.level {
                1 => 24,
                2 => 20,
                _ => 18,
            };

            let indent = (item.level - 1) * 30;

            elements.push(PageElement {
                element_type: "text".to_string(),
                content: ElementContent::Text {
                    text: item_text,
                    font_size: Some(font_size),
                    font_weight: Some(if item.level == 1 { "bold" } else { "normal" }.to_string()),
                },
                position: Position { 
                    x: x + indent as i32, 
                    y 
                },
                size: Size::new(column_width - indent, layout.item_height),
                style: Some(ElementStyle {
                    color: Some(if item.level == 1 { "#333333" } else { "#666666" }.to_string()),
                    background_color: if layout.name == "card_style" {
                        Some("#FFFFFF".to_string())
                    } else {
                        None
                    },
                    border_radius: if layout.name == "card_style" {
                        Some(8)
                    } else {
                        None
                    },
                    opacity: None,
                    shadow: if layout.name == "card_style" {
                        Some(Shadow {
                            x: 0,
                            y: 2,
                            blur: 4,
                            color: "rgba(0,0,0,0.1)".to_string(),
                        })
                    } else {
                        None
                    },
                }),
                z_index,
            });
            z_index += 1;
        }

        elements
    }

    pub fn get_all_layouts(&self) -> &[TocLayout] {
        &self.layout_variants
    }

    pub fn get_layout_by_name(&self, name: &str) -> Option<&TocLayout> {
        self.layout_variants.iter().find(|l| l.name == name)
    }
}

impl PageTemplate for TocPageTemplate {
    fn get_page_type(&self) -> PageType {
        PageType::Toc
    }

    fn get_layout(&self, style: &StyleParams) -> Layout {
        let toc_layout = self.get_layout(10, style);
        
        Layout {
            name: toc_layout.name.clone(),
            page_type: PageType::Toc,
            regions: vec![
                Region {
                    name: "title".to_string(),
                    position: toc_layout.title_position.clone(),
                    size: Size::new(600, 60),
                    region_type: RegionType::Text,
                    constraints: vec![
                        RegionConstraint::Alignment("center".to_string()),
                    ],
                },
                Region {
                    name: "content".to_string(),
                    position: toc_layout.content_start_position.clone(),
                    size: Size::new(1520, 700),
                    region_type: RegionType::Container,
                    constraints: vec![
                        RegionConstraint::Spacing(toc_layout.item_spacing),
                    ],
                },
            ],
            background: toc_layout.background_type.clone(),
            grid: Some(LayoutGrid {
                columns: toc_layout.columns,
                rows: 10,
                gap: toc_layout.item_spacing,
            }),
        }
    }

    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement> {
        let items = content.items.clone().unwrap_or_default();
        let toc_content = TocContent {
            title: content.title.clone(),
            items: items.into_iter().map(|item| TocItem {
                title: item,
                page_number: None,
                level: 1,
            }).collect(),
        };

        let toc_layout = self.get_layout_by_name(&layout.name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone());

        self.apply_content(&toc_layout, &toc_content)
    }
}

impl Default for TocPageTemplate {
    fn default() -> Self {
        Self::new()
    }
}
