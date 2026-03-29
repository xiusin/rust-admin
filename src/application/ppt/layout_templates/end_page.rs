use super::*;
use crate::domain::model::m_style::{Position, StyleParams};

pub struct EndPageTemplate {
    layout_variants: Vec<EndLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndLayout {
    pub name: String,
    pub title_position: Option<Position>,
    pub message_position: Option<Position>,
    pub contact_positions: ContactPositions,
    pub background_type: BackgroundType,
    pub decoration_shapes: Vec<DecorationShape>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPositions {
    pub email: Option<Position>,
    pub phone: Option<Position>,
    pub website: Option<Position>,
    pub address: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationShape {
    pub position: Position,
    pub size: Size,
    pub shape_type: String,
    pub color: String,
    pub opacity: f32,
}

impl EndPageTemplate {
    pub fn new() -> Self {
        Self {
            layout_variants: Self::create_default_layouts(),
        }
    }

    fn create_default_layouts() -> Vec<EndLayout> {
        vec![
            EndLayout {
                name: "centered_thank_you".to_string(),
                title_position: Some(Position { x: 960, y: 400 }),
                message_position: Some(Position { x: 960, y: 500 }),
                contact_positions: ContactPositions {
                    email: Some(Position { x: 960, y: 650 }),
                    phone: Some(Position { x: 960, y: 700 }),
                    website: Some(Position { x: 960, y: 750 }),
                    address: None,
                },
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                decoration_shapes: vec![],
            },
            EndLayout {
                name: "gradient_background".to_string(),
                title_position: Some(Position { x: 960, y: 380 }),
                message_position: Some(Position { x: 960, y: 480 }),
                contact_positions: ContactPositions {
                    email: Some(Position { x: 960, y: 620 }),
                    phone: Some(Position { x: 960, y: 670 }),
                    website: Some(Position { x: 960, y: 720 }),
                    address: Some(Position { x: 960, y: 770 }),
                },
                background_type: BackgroundType::Gradient {
                    start_color: "#667eea".to_string(),
                    end_color: "#764ba2".to_string(),
                    angle: 135,
                },
                decoration_shapes: vec![
                    DecorationShape {
                        position: Position { x: 200, y: 200 },
                        size: Size::new(100, 100),
                        shape_type: "circle".to_string(),
                        color: "#FFFFFF".to_string(),
                        opacity: 0.1,
                    },
                    DecorationShape {
                        position: Position { x: 1600, y: 700 },
                        size: Size::new(150, 150),
                        shape_type: "circle".to_string(),
                        color: "#FFFFFF".to_string(),
                        opacity: 0.1,
                    },
                ],
            },
            EndLayout {
                name: "minimalist".to_string(),
                title_position: Some(Position { x: 960, y: 450 }),
                message_position: None,
                contact_positions: ContactPositions {
                    email: Some(Position { x: 960, y: 600 }),
                    phone: None,
                    website: Some(Position { x: 960, y: 650 }),
                    address: None,
                },
                background_type: BackgroundType::Solid("#F8F9FA".to_string()),
                decoration_shapes: vec![
                    DecorationShape {
                        position: Position { x: 0, y: 900 },
                        size: Size::new(1920, 180),
                        shape_type: "rectangle".to_string(),
                        color: "#667eea".to_string(),
                        opacity: 1.0,
                    },
                ],
            },
            EndLayout {
                name: "contact_focused".to_string(),
                title_position: Some(Position { x: 960, y: 150 }),
                message_position: Some(Position { x: 960, y: 250 }),
                contact_positions: ContactPositions {
                    email: Some(Position { x: 600, y: 450 }),
                    phone: Some(Position { x: 1320, y: 450 }),
                    website: Some(Position { x: 600, y: 600 }),
                    address: Some(Position { x: 1320, y: 600 }),
                },
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
                decoration_shapes: vec![
                    DecorationShape {
                        position: Position { x: 150, y: 350 },
                        size: Size::new(700, 200),
                        shape_type: "rectangle".to_string(),
                        color: "#F5F5F5".to_string(),
                        opacity: 1.0,
                    },
                    DecorationShape {
                        position: Position { x: 1070, y: 350 },
                        size: Size::new(700, 200),
                        shape_type: "rectangle".to_string(),
                        color: "#F5F5F5".to_string(),
                        opacity: 1.0,
                    },
                    DecorationShape {
                        position: Position { x: 150, y: 500 },
                        size: Size::new(700, 200),
                        shape_type: "rectangle".to_string(),
                        color: "#F5F5F5".to_string(),
                        opacity: 1.0,
                    },
                    DecorationShape {
                        position: Position { x: 1070, y: 500 },
                        size: Size::new(700, 200),
                        shape_type: "rectangle".to_string(),
                        color: "#F5F5F5".to_string(),
                        opacity: 1.0,
                    },
                ],
            },
            EndLayout {
                name: "qr_code_style".to_string(),
                title_position: Some(Position { x: 960, y: 200 }),
                message_position: Some(Position { x: 960, y: 300 }),
                contact_positions: ContactPositions {
                    email: Some(Position { x: 960, y: 500 }),
                    phone: Some(Position { x: 960, y: 550 }),
                    website: Some(Position { x: 960, y: 600 }),
                    address: Some(Position { x: 960, y: 650 }),
                },
                background_type: BackgroundType::Gradient {
                    start_color: "#FFFFFF".to_string(),
                    end_color: "#F0F0F0".to_string(),
                    angle: 180,
                },
                decoration_shapes: vec![
                    DecorationShape {
                        position: Position { x: 810, y: 720 },
                        size: Size::new(300, 300),
                        shape_type: "rectangle".to_string(),
                        color: "#333333".to_string(),
                        opacity: 0.05,
                    },
                ],
            },
            EndLayout {
                name: "social_media_style".to_string(),
                title_position: Some(Position { x: 960, y: 350 }),
                message_position: Some(Position { x: 960, y: 450 }),
                contact_positions: ContactPositions {
                    email: Some(Position { x: 600, y: 600 }),
                    phone: Some(Position { x: 960, y: 600 }),
                    website: Some(Position { x: 1320, y: 600 }),
                    address: None,
                },
                background_type: BackgroundType::Gradient {
                    start_color: "#1a1a2e".to_string(),
                    end_color: "#16213e".to_string(),
                    angle: 135,
                },
                decoration_shapes: vec![
                    DecorationShape {
                        position: Position { x: 500, y: 550 },
                        size: Size::new(200, 100),
                        shape_type: "rounded_rectangle".to_string(),
                        color: "#4A90E2".to_string(),
                        opacity: 0.8,
                    },
                    DecorationShape {
                        position: Position { x: 860, y: 550 },
                        size: Size::new(200, 100),
                        shape_type: "rounded_rectangle".to_string(),
                        color: "#50C878".to_string(),
                        opacity: 0.8,
                    },
                    DecorationShape {
                        position: Position { x: 1220, y: 550 },
                        size: Size::new(200, 100),
                        shape_type: "rounded_rectangle".to_string(),
                        color: "#FF6B6B".to_string(),
                        opacity: 0.8,
                    },
                ],
            },
        ]
    }

    pub fn get_layout(&self, style: &StyleParams) -> EndLayout {
        let layout_index = style.layout_rules.grid_columns as usize % self.layout_variants.len();
        
        self.layout_variants
            .get(layout_index)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone())
    }

    pub fn apply_content(&self, layout: &EndLayout, content: &EndContent) -> Vec<PageElement> {
        let mut elements = Vec::new();
        let mut z_index = 0;

        for shape in &layout.decoration_shapes {
            elements.push(PageElement {
                element_type: "shape".to_string(),
                content: ElementContent::Shape {
                    shape_type: shape.shape_type.clone(),
                    fill_color: shape.color.clone(),
                },
                position: shape.position.clone(),
                size: shape.size.clone(),
                style: Some(ElementStyle {
                    color: None,
                    background_color: Some(shape.color.clone()),
                    border_radius: if shape.shape_type == "rounded_rectangle" { Some(12) } else { None },
                    opacity: Some(shape.opacity),
                    shadow: None,
                }),
                z_index,
            });
            z_index += 1;
        }

        if let Some(title) = &content.title {
            if let Some(pos) = &layout.title_position {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: title.clone(),
                        font_size: Some(48),
                        font_weight: Some("bold".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(800, 80),
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
        }

        if let Some(message) = &content.message {
            if let Some(pos) = &layout.message_position {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: message.clone(),
                        font_size: Some(24),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(800, 50),
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

        if let Some(contact) = &content.contact_info {
            if let (Some(email), Some(pos)) = (&contact.email, &layout.contact_positions.email) {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: format!("📧 {}", email),
                        font_size: Some(18),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(400, 40),
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

            if let (Some(phone), Some(pos)) = (&contact.phone, &layout.contact_positions.phone) {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: format!("📞 {}", phone),
                        font_size: Some(18),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(400, 40),
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

            if let (Some(website), Some(pos)) = (&contact.website, &layout.contact_positions.website) {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: format!("🌐 {}", website),
                        font_size: Some(18),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(400, 40),
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

            if let (Some(address), Some(pos)) = (&contact.address, &layout.contact_positions.address) {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: format!("📍 {}", address),
                        font_size: Some(18),
                        font_weight: Some("normal".to_string()),
                    },
                    position: pos.clone(),
                    size: Size::new(500, 40),
                    style: Some(ElementStyle {
                        color: Some("#666666".to_string()),
                        background_color: None,
                        border_radius: None,
                        opacity: None,
                        shadow: None,
                    }),
                    z_index,
                });
            }
        }

        elements
    }

    pub fn get_all_layouts(&self) -> &[EndLayout] {
        &self.layout_variants
    }

    pub fn get_layout_by_name(&self, name: &str) -> Option<&EndLayout> {
        self.layout_variants.iter().find(|l| l.name == name)
    }
}

impl PageTemplate for EndPageTemplate {
    fn get_page_type(&self) -> PageType {
        PageType::End
    }

    fn get_layout(&self, style: &StyleParams) -> Layout {
        let end_layout = self.get_layout(style);
        
        Layout {
            name: end_layout.name.clone(),
            page_type: PageType::End,
            regions: vec![],
            background: end_layout.background_type.clone(),
            grid: None,
        }
    }

    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement> {
        let end_content = EndContent {
            title: content.title.clone(),
            message: content.body.clone(),
            contact_info: None,
        };

        let end_layout = self.get_layout_by_name(&layout.name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone());

        self.apply_content(&end_layout, &end_content)
    }
}

impl Default for EndPageTemplate {
    fn default() -> Self {
        Self::new()
    }
}
