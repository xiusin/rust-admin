use super::*;
use crate::domain::model::m_style::{Position, StyleParams};

pub struct ContentPageTemplate {
    layouts: Vec<ContentLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentLayout {
    pub name: String,
    pub layout_type: ContentLayoutType,
    pub title_region: TextRegion,
    pub body_region: Option<TextRegion>,
    pub image_regions: Vec<ImageRegion>,
    pub shape_regions: Vec<ShapeRegion>,
    pub background_type: BackgroundType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentLayoutType {
    TitleOnly,
    TitleBody,
    TitleBodyImage,
    TwoColumn,
    ImageLeft,
    ImageRight,
    Comparison,
    Quote,
    BulletPoints,
    ImageGallery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRegion {
    pub position: Position,
    pub size: Size,
    pub font_size: Option<u32>,
    pub alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRegion {
    pub position: Position,
    pub size: Size,
    pub border_radius: Option<u32>,
    pub shadow: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeRegion {
    pub position: Position,
    pub size: Size,
    pub shape_type: String,
    pub fill_color: String,
}

impl ContentPageTemplate {
    pub fn new() -> Self {
        Self {
            layouts: Self::create_default_layouts(),
        }
    }

    fn create_default_layouts() -> Vec<ContentLayout> {
        vec![
            ContentLayout {
                name: "title_only".to_string(),
                layout_type: ContentLayoutType::TitleOnly,
                title_region: TextRegion {
                    position: Position { x: 960, y: 450 },
                    size: Size::new(1200, 200),
                    font_size: Some(48),
                    alignment: "center".to_string(),
                },
                body_region: None,
                image_regions: vec![],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "title_body".to_string(),
                layout_type: ContentLayoutType::TitleBody,
                title_region: TextRegion {
                    position: Position { x: 150, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "left".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(1620, 600),
                    font_size: Some(18),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "title_body_image_right".to_string(),
                layout_type: ContentLayoutType::TitleBodyImage,
                title_region: TextRegion {
                    position: Position { x: 150, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "left".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(900, 600),
                    font_size: Some(18),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![ImageRegion {
                    position: Position { x: 1100, y: 200 },
                    size: Size::new(670, 500),
                    border_radius: Some(8),
                    shadow: Some(true),
                }],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "title_body_image_left".to_string(),
                layout_type: ContentLayoutType::TitleBodyImage,
                title_region: TextRegion {
                    position: Position { x: 150, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "left".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 870, y: 200 },
                    size: Size::new(900, 600),
                    font_size: Some(18),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![ImageRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(670, 500),
                    border_radius: Some(8),
                    shadow: Some(true),
                }],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "two_column".to_string(),
                layout_type: ContentLayoutType::TwoColumn,
                title_region: TextRegion {
                    position: Position { x: 960, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "center".to_string(),
                },
                body_region: None,
                image_regions: vec![],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "image_left".to_string(),
                layout_type: ContentLayoutType::ImageLeft,
                title_region: TextRegion {
                    position: Position { x: 960, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "center".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 1020, y: 200 },
                    size: Size::new(750, 600),
                    font_size: Some(18),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![ImageRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(800, 600),
                    border_radius: Some(8),
                    shadow: Some(true),
                }],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "image_right".to_string(),
                layout_type: ContentLayoutType::ImageRight,
                title_region: TextRegion {
                    position: Position { x: 960, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "center".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(750, 600),
                    font_size: Some(18),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![ImageRegion {
                    position: Position { x: 970, y: 200 },
                    size: Size::new(800, 600),
                    border_radius: Some(8),
                    shadow: Some(true),
                }],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "comparison".to_string(),
                layout_type: ContentLayoutType::Comparison,
                title_region: TextRegion {
                    position: Position { x: 960, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "center".to_string(),
                },
                body_region: None,
                image_regions: vec![],
                shape_regions: vec![
                    ShapeRegion {
                        position: Position { x: 150, y: 200 },
                        size: Size::new(750, 600),
                        shape_type: "rectangle".to_string(),
                        fill_color: "#F5F5F5".to_string(),
                    },
                    ShapeRegion {
                        position: Position { x: 1020, y: 200 },
                        size: Size::new(750, 600),
                        shape_type: "rectangle".to_string(),
                        fill_color: "#E8F4F8".to_string(),
                    },
                ],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "quote".to_string(),
                layout_type: ContentLayoutType::Quote,
                title_region: TextRegion {
                    position: Position { x: 960, y: 400 },
                    size: Size::new(1200, 300),
                    font_size: Some(32),
                    alignment: "center".to_string(),
                },
                body_region: None,
                image_regions: vec![],
                shape_regions: vec![
                    ShapeRegion {
                        position: Position { x: 300, y: 350 },
                        size: Size::new(80, 80),
                        shape_type: "quote_mark".to_string(),
                        fill_color: "#E0E0E0".to_string(),
                    },
                ],
                background_type: BackgroundType::Gradient {
                    start_color: "#FFFFFF".to_string(),
                    end_color: "#F8F8F8".to_string(),
                    angle: 180,
                },
            },
            ContentLayout {
                name: "bullet_points".to_string(),
                layout_type: ContentLayoutType::BulletPoints,
                title_region: TextRegion {
                    position: Position { x: 150, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "left".to_string(),
                },
                body_region: Some(TextRegion {
                    position: Position { x: 150, y: 200 },
                    size: Size::new(1620, 600),
                    font_size: Some(20),
                    alignment: "left".to_string(),
                }),
                image_regions: vec![],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ContentLayout {
                name: "image_gallery".to_string(),
                layout_type: ContentLayoutType::ImageGallery,
                title_region: TextRegion {
                    position: Position { x: 960, y: 80 },
                    size: Size::new(1620, 80),
                    font_size: Some(36),
                    alignment: "center".to_string(),
                },
                body_region: None,
                image_regions: vec![
                    ImageRegion {
                        position: Position { x: 150, y: 200 },
                        size: Size::new(500, 350),
                        border_radius: Some(8),
                        shadow: Some(true),
                    },
                    ImageRegion {
                        position: Position { x: 710, y: 200 },
                        size: Size::new(500, 350),
                        border_radius: Some(8),
                        shadow: Some(true),
                    },
                    ImageRegion {
                        position: Position { x: 1270, y: 200 },
                        size: Size::new(500, 350),
                        border_radius: Some(8),
                        shadow: Some(true),
                    },
                ],
                shape_regions: vec![],
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
        ]
    }

    pub fn select_layout(&self, content: &SectionContent, _style: &StyleParams) -> ContentLayout {
        let layout_type = self.determine_layout_type(content);
        
        self.layouts
            .iter()
            .find(|l| std::mem::discriminant(&l.layout_type) == std::mem::discriminant(&layout_type))
            .cloned()
            .unwrap_or_else(|| self.layouts[1].clone())
    }

    fn determine_layout_type(&self, content: &SectionContent) -> ContentLayoutType {
        if let Some(hint) = &content.layout_hint {
            match hint.as_str() {
                "quote" => return ContentLayoutType::Quote,
                "comparison" => return ContentLayoutType::Comparison,
                "gallery" => return ContentLayoutType::ImageGallery,
                "two_column" => return ContentLayoutType::TwoColumn,
                _ => {}
            }
        }

        let has_images = content.images.as_ref().map_or(false, |imgs| !imgs.is_empty());
        let has_bullets = content.bullet_points.as_ref().map_or(false, |bps| !bps.is_empty());
        let has_body = content.body.is_some();

        if has_images && has_bullets {
            ContentLayoutType::ImageRight
        } else if has_images && has_body {
            ContentLayoutType::TitleBodyImage
        } else if has_images {
            ContentLayoutType::ImageGallery
        } else if has_bullets {
            ContentLayoutType::BulletPoints
        } else if has_body {
            ContentLayoutType::TitleBody
        } else {
            ContentLayoutType::TitleOnly
        }
    }

    pub fn apply_content(&self, layout: &ContentLayout, content: &SectionContent) -> Vec<PageElement> {
        let mut elements = Vec::new();
        let mut z_index = 0;

        elements.push(PageElement {
            element_type: "text".to_string(),
            content: ElementContent::Text {
                text: content.title.clone(),
                font_size: layout.title_region.font_size,
                font_weight: Some("bold".to_string()),
            },
            position: layout.title_region.position.clone(),
            size: layout.title_region.size.clone(),
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

        if let Some(body_region) = &layout.body_region {
            if let Some(body) = &content.body {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: body.clone(),
                        font_size: body_region.font_size,
                        font_weight: Some("normal".to_string()),
                    },
                    position: body_region.position.clone(),
                    size: body_region.size.clone(),
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
            } else if let Some(bullets) = &content.bullet_points {
                elements.push(PageElement {
                    element_type: "list".to_string(),
                    content: ElementContent::List {
                        items: bullets.clone(),
                        bullet_style: Some("circle".to_string()),
                    },
                    position: body_region.position.clone(),
                    size: body_region.size.clone(),
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

        if let Some(images) = &content.images {
            for (idx, img_region) in layout.image_regions.iter().enumerate() {
                if idx < images.len() {
                    elements.push(PageElement {
                        element_type: "image".to_string(),
                        content: ElementContent::Image {
                            url: images[idx].url.clone(),
                            alt: images[idx].alt.clone(),
                        },
                        position: img_region.position.clone(),
                        size: img_region.size.clone(),
                        style: Some(ElementStyle {
                            color: None,
                            background_color: None,
                            border_radius: img_region.border_radius,
                            opacity: None,
                            shadow: img_region.shadow.map(|_| Shadow {
                                x: 0,
                                y: 4,
                                blur: 8,
                                color: "rgba(0,0,0,0.1)".to_string(),
                            }),
                        }),
                        z_index,
                    });
                    z_index += 1;
                }
            }
        }

        for shape_region in &layout.shape_regions {
            elements.push(PageElement {
                element_type: "shape".to_string(),
                content: ElementContent::Shape {
                    shape_type: shape_region.shape_type.clone(),
                    fill_color: shape_region.fill_color.clone(),
                },
                position: shape_region.position.clone(),
                size: shape_region.size.clone(),
                style: Some(ElementStyle {
                    color: None,
                    background_color: Some(shape_region.fill_color.clone()),
                    border_radius: Some(8),
                    opacity: None,
                    shadow: None,
                }),
                z_index,
            });
            z_index += 1;
        }

        elements
    }

    pub fn get_all_layouts(&self) -> &[ContentLayout] {
        &self.layouts
    }

    pub fn get_layout_by_name(&self, name: &str) -> Option<&ContentLayout> {
        self.layouts.iter().find(|l| l.name == name)
    }
}

impl PageTemplate for ContentPageTemplate {
    fn get_page_type(&self) -> PageType {
        PageType::Content
    }

    fn get_layout(&self, _style: &StyleParams) -> Layout {
        let content_layout = &self.layouts[1];
        
        Layout {
            name: content_layout.name.clone(),
            page_type: PageType::Content,
            regions: vec![
                Region {
                    name: "title".to_string(),
                    position: content_layout.title_region.position.clone(),
                    size: content_layout.title_region.size.clone(),
                    region_type: RegionType::Text,
                    constraints: vec![
                        RegionConstraint::Alignment(content_layout.title_region.alignment.clone()),
                    ],
                },
            ],
            background: content_layout.background_type.clone(),
            grid: None,
        }
    }

    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement> {
        let section_content = SectionContent {
            title: content.title.clone().unwrap_or_default(),
            body: content.body.clone(),
            bullet_points: content.items.clone(),
            images: content.images.clone(),
            layout_hint: None,
        };

        let content_layout = self.get_layout_by_name(&layout.name)
            .cloned()
            .unwrap_or_else(|| self.layouts[1].clone());

        self.apply_content(&content_layout, &section_content)
    }
}

impl Default for ContentPageTemplate {
    fn default() -> Self {
        Self::new()
    }
}
