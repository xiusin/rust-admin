use super::*;
use crate::domain::model::m_style::{Position, StyleParams};

pub struct TitlePageTemplate {
    layout_variants: Vec<TitleLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleLayout {
    pub name: String,
    pub title_position: Position,
    pub subtitle_position: Option<Position>,
    pub author_position: Option<Position>,
    pub date_position: Option<Position>,
    pub logo_position: Option<Position>,
    pub background_type: BackgroundType,
    pub title_size: Size,
    pub subtitle_size: Option<Size>,
    pub alignment: String,
}

impl TitlePageTemplate {
    pub fn new() -> Self {
        Self {
            layout_variants: Self::create_default_layouts(),
        }
    }

    fn create_default_layouts() -> Vec<TitleLayout> {
        vec![
            TitleLayout {
                name: "centered".to_string(),
                title_position: Position { x: 960, y: 400 },
                subtitle_position: Some(Position { x: 960, y: 500 }),
                author_position: Some(Position { x: 960, y: 650 }),
                date_position: Some(Position { x: 960, y: 720 }),
                logo_position: Some(Position { x: 960, y: 200 }),
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                title_size: Size::new(800, 100),
                subtitle_size: Some(Size::new(600, 60)),
                alignment: "center".to_string(),
            },
            TitleLayout {
                name: "left_aligned".to_string(),
                title_position: Position { x: 200, y: 400 },
                subtitle_position: Some(Position { x: 200, y: 520 }),
                author_position: Some(Position { x: 200, y: 650 }),
                date_position: Some(Position { x: 200, y: 720 }),
                logo_position: Some(Position { x: 200, y: 150 }),
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                title_size: Size::new(800, 120),
                subtitle_size: Some(Size::new(600, 80)),
                alignment: "left".to_string(),
            },
            TitleLayout {
                name: "gradient_background".to_string(),
                title_position: Position { x: 960, y: 450 },
                subtitle_position: Some(Position { x: 960, y: 560 }),
                author_position: Some(Position { x: 960, y: 680 }),
                date_position: Some(Position { x: 960, y: 750 }),
                logo_position: None,
                background_type: BackgroundType::Gradient {
                    start_color: "#667eea".to_string(),
                    end_color: "#764ba2".to_string(),
                    angle: 135,
                },
                title_size: Size::new(900, 120),
                subtitle_size: Some(Size::new(700, 70)),
                alignment: "center".to_string(),
            },
            TitleLayout {
                name: "bottom_heavy".to_string(),
                title_position: Position { x: 960, y: 600 },
                subtitle_position: Some(Position { x: 960, y: 720 }),
                author_position: Some(Position { x: 960, y: 820 }),
                date_position: Some(Position { x: 960, y: 880 }),
                logo_position: Some(Position { x: 960, y: 300 }),
                background_type: BackgroundType::Solid("#F5F5F5".to_string()),
                title_size: Size::new(850, 100),
                subtitle_size: Some(Size::new(650, 60)),
                alignment: "center".to_string(),
            },
            TitleLayout {
                name: "split_layout".to_string(),
                title_position: Position { x: 480, y: 400 },
                subtitle_position: Some(Position { x: 480, y: 520 }),
                author_position: Some(Position { x: 480, y: 650 }),
                date_position: Some(Position { x: 480, y: 720 }),
                logo_position: None,
                background_type: BackgroundType::Gradient {
                    start_color: "#FFFFFF".to_string(),
                    end_color: "#F0F0F0".to_string(),
                    angle: 90,
                },
                title_size: Size::new(700, 120),
                subtitle_size: Some(Size::new(500, 80)),
                alignment: "left".to_string(),
            },
        ]
    }

    pub fn get_layout(&self, style: &StyleParams) -> TitleLayout {
        let layout_preference = &style.layout_rules;
        
        let alignment_hint = layout_preference.element_spacing.small;
        
        let preferred_layout = if alignment_hint < 10 {
            "centered"
        } else if alignment_hint < 20 {
            "left_aligned"
        } else if alignment_hint < 30 {
            "gradient_background"
        } else if alignment_hint < 40 {
            "bottom_heavy"
        } else {
            "split_layout"
        };

        self.layout_variants
            .iter()
            .find(|l| l.name == preferred_layout)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone())
    }

    pub fn apply_content(&self, layout: &TitleLayout, content: &TitleContent) -> Vec<PageElement> {
        let mut elements = Vec::new();
        let mut z_index = 0;

        elements.push(PageElement {
            element_type: "text".to_string(),
            content: ElementContent::Text {
                text: content.title.clone(),
                font_size: Some(48),
                font_weight: Some("bold".to_string()),
            },
            position: layout.title_position.clone(),
            size: layout.title_size.clone(),
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

        if let Some(subtitle) = &content.subtitle {
            if let (Some(pos), Some(size)) = (&layout.subtitle_position, &layout.subtitle_size) {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: subtitle.clone(),
                        font_size: Some(28),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: size.clone(),
                    style: Some(ElementStyle {
                        color: Some("#666666".to_string()),
                        background_color: None,
                        border_radius: None,
                        opacity: None,
                        shadow: None,
                    }),
                    z_index,
                });
                z_index += 1;
            }
        }

        if let Some(author) = &content.author {
            if let Some(pos) = &layout.author_position {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: author.clone(),
                        font_size: Some(20),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(400, 40),
                    style: Some(ElementStyle {
                        color: Some("#888888".to_string()),
                        background_color: None,
                        border_radius: None,
                        opacity: None,
                        shadow: None,
                    }),
                    z_index,
                });
                z_index += 1;
            }
        }

        if let Some(date) = &content.date {
            if let Some(pos) = &layout.date_position {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: date.clone(),
                        font_size: Some(16),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(300, 30),
                    style: Some(ElementStyle {
                        color: Some("#999999".to_string()),
                        background_color: None,
                        border_radius: None,
                        opacity: None,
                        shadow: None,
                    }),
                    z_index,
                });
                z_index += 1;
            }
        }

        if let Some(logo_url) = &content.logo {
            if let Some(pos) = &layout.logo_position {
                elements.push(PageElement {
                    element_type: "image".to_string(),
                    content: ElementContent::Image {
                        url: logo_url.clone(),
                        alt: Some("Logo".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(200, 80),
                    style: Some(ElementStyle {
                        color: None,
                        background_color: None,
                        border_radius: Some(4),
                        opacity: None,
                        shadow: None,
                    }),
                    z_index,
                });
            }
        }

        elements
    }

    pub fn get_all_layouts(&self) -> &[TitleLayout] {
        &self.layout_variants
    }

    pub fn get_layout_by_name(&self, name: &str) -> Option<&TitleLayout> {
        self.layout_variants.iter().find(|l| l.name == name)
    }
}

impl PageTemplate for TitlePageTemplate {
    fn get_page_type(&self) -> PageType {
        PageType::Title
    }

    fn get_layout(&self, style: &StyleParams) -> Layout {
        let title_layout = self.get_layout(style);
        
        Layout {
            name: title_layout.name.clone(),
            page_type: PageType::Title,
            regions: vec![
                Region {
                    name: "title".to_string(),
                    position: title_layout.title_position.clone(),
                    size: title_layout.title_size.clone(),
                    region_type: RegionType::Text,
                    constraints: vec![
                        RegionConstraint::MinWidth(400),
                        RegionConstraint::MaxWidth(1200),
                        RegionConstraint::Alignment(title_layout.alignment.clone()),
                    ],
                },
            ],
            background: title_layout.background_type.clone(),
            grid: None,
        }
    }

    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement> {
        let title_content = TitleContent {
            title: content.title.clone().unwrap_or_default(),
            subtitle: content.subtitle.clone(),
            author: None,
            date: None,
            logo: None,
        };

        let title_layout = self.get_layout_by_name(&layout.name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone());

        self.apply_content(&title_layout, &title_content)
    }
}

impl Default for TitlePageTemplate {
    fn default() -> Self {
        Self::new()
    }
}
