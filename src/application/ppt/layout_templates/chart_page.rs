use super::*;
use crate::domain::model::m_style::{Position, StyleParams};

pub struct ChartPageTemplate {
    layout_variants: Vec<ChartLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartLayout {
    pub name: String,
    pub title_position: Position,
    pub chart_regions: Vec<ChartRegion>,
    pub description_region: Option<TextRegion>,
    pub background_type: BackgroundType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartRegion {
    pub position: Position,
    pub size: Size,
    pub chart_type: Option<String>,
    pub show_legend: bool,
    pub legend_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRegion {
    pub position: Position,
    pub size: Size,
    pub font_size: Option<u32>,
    pub alignment: String,
}

impl ChartPageTemplate {
    pub fn new() -> Self {
        Self {
            layout_variants: Self::create_default_layouts(),
        }
    }

    fn create_default_layouts() -> Vec<ChartLayout> {
        vec![
            ChartLayout {
                name: "single_chart".to_string(),
                title_position: Position { x: 960, y: 80 },
                chart_regions: vec![ChartRegion {
                    position: Position { x: 260, y: 180 },
                    size: Size::new(1400, 650),
                    chart_type: None,
                    show_legend: true,
                    legend_position: "bottom".to_string(),
                }],
                description_region: None,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ChartLayout {
                name: "chart_with_description".to_string(),
                title_position: Position { x: 960, y: 80 },
                chart_regions: vec![ChartRegion {
                    position: Position { x: 150, y: 180 },
                    size: Size::new(1100, 600),
                    chart_type: None,
                    show_legend: true,
                    legend_position: "right".to_string(),
                }],
                description_region: Some(TextRegion {
                    position: Position { x: 1300, y: 180 },
                    size: Size::new(470, 600),
                    font_size: Some(16),
                    alignment: "left".to_string(),
                }),
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ChartLayout {
                name: "two_charts_horizontal".to_string(),
                title_position: Position { x: 960, y: 80 },
                chart_regions: vec![
                    ChartRegion {
                        position: Position { x: 150, y: 180 },
                        size: Size::new(750, 600),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 1020, y: 180 },
                        size: Size::new(750, 600),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                ],
                description_region: None,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ChartLayout {
                name: "two_charts_vertical".to_string(),
                title_position: Position { x: 960, y: 60 },
                chart_regions: vec![
                    ChartRegion {
                        position: Position { x: 260, y: 150 },
                        size: Size::new(1400, 350),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "right".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 260, y: 530 },
                        size: Size::new(1400, 350),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "right".to_string(),
                    },
                ],
                description_region: None,
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
            ChartLayout {
                name: "three_charts".to_string(),
                title_position: Position { x: 960, y: 60 },
                chart_regions: vec![
                    ChartRegion {
                        position: Position { x: 100, y: 150 },
                        size: Size::new(540, 400),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 690, y: 150 },
                        size: Size::new(540, 400),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 1280, y: 150 },
                        size: Size::new(540, 400),
                        chart_type: None,
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                ],
                description_region: Some(TextRegion {
                    position: Position { x: 260, y: 600 },
                    size: Size::new(1400, 150),
                    font_size: Some(14),
                    alignment: "center".to_string(),
                }),
                background_type: BackgroundType::Solid("#F9F9F9".to_string()),
            },
            ChartLayout {
                name: "dashboard_style".to_string(),
                title_position: Position { x: 960, y: 60 },
                chart_regions: vec![
                    ChartRegion {
                        position: Position { x: 100, y: 140 },
                        size: Size::new(850, 380),
                        chart_type: Some("bar".to_string()),
                        show_legend: true,
                        legend_position: "top".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 970, y: 140 },
                        size: Size::new(850, 380),
                        chart_type: Some("line".to_string()),
                        show_legend: true,
                        legend_position: "top".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 100, y: 560 },
                        size: Size::new(550, 320),
                        chart_type: Some("pie".to_string()),
                        show_legend: true,
                        legend_position: "right".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 700, y: 560 },
                        size: Size::new(550, 320),
                        chart_type: Some("doughnut".to_string()),
                        show_legend: true,
                        legend_position: "right".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 1300, y: 560 },
                        size: Size::new(520, 320),
                        chart_type: Some("radar".to_string()),
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                ],
                description_region: None,
                background_type: BackgroundType::Gradient {
                    start_color: "#F5F7FA".to_string(),
                    end_color: "#FFFFFF".to_string(),
                    angle: 180,
                },
            },
            ChartLayout {
                name: "comparison_charts".to_string(),
                title_position: Position { x: 960, y: 80 },
                chart_regions: vec![
                    ChartRegion {
                        position: Position { x: 150, y: 180 },
                        size: Size::new(750, 600),
                        chart_type: Some("bar".to_string()),
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                    ChartRegion {
                        position: Position { x: 1020, y: 180 },
                        size: Size::new(750, 600),
                        chart_type: Some("bar".to_string()),
                        show_legend: true,
                        legend_position: "bottom".to_string(),
                    },
                ],
                description_region: Some(TextRegion {
                    position: Position { x: 960, y: 850 },
                    size: Size::new(1620, 50),
                    font_size: Some(14),
                    alignment: "center".to_string(),
                }),
                background_type: BackgroundType::Solid("#FFFFFF".to_string()),
            },
        ]
    }

    pub fn get_layout(&self, chart_count: usize, _style: &StyleParams) -> ChartLayout {
        let layout_name = match chart_count {
            1 => "single_chart",
            2 => "two_charts_horizontal",
            3 => "three_charts",
            4..=5 => "dashboard_style",
            _ => "comparison_charts",
        };

        self.layout_variants
            .iter()
            .find(|l| l.name == layout_name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone())
    }

    pub fn apply_content(&self, layout: &ChartLayout, content: &ChartPageContent) -> Vec<PageElement> {
        let mut elements = Vec::new();
        let mut z_index = 0;

        elements.push(PageElement {
            element_type: "text".to_string(),
            content: ElementContent::Text {
                text: content.title.clone(),
                font_size: Some(36),
                font_weight: Some("bold".to_string()),
            },
            position: layout.title_position.clone(),
            size: Size::new(1200, 80),
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

        for (idx, chart_region) in layout.chart_regions.iter().enumerate() {
            if idx < content.charts.len() {
                let chart = &content.charts[idx];
                
                elements.push(PageElement {
                    element_type: "chart".to_string(),
                    content: ElementContent::Chart {
                        chart_type: chart_region.chart_type.clone().unwrap_or_else(|| chart.chart_type.clone()),
                        data: chart.data.clone(),
                    },
                    position: chart_region.position.clone(),
                    size: chart_region.size.clone(),
                    style: Some(ElementStyle {
                        color: None,
                        background_color: Some("#FFFFFF".to_string()),
                        border_radius: Some(8),
                        opacity: None,
                        shadow: Some(Shadow {
                            x: 0,
                            y: 2,
                            blur: 8,
                            color: "rgba(0,0,0,0.08)".to_string(),
                        }),
                    }),
                    z_index,
                });
                z_index += 1;
            }
        }

        if let Some(desc_region) = &layout.description_region {
            if let Some(description) = &content.description {
                elements.push(PageElement {
                    element_type: "text".to_string(),
                    content: ElementContent::Text {
                        text: description.clone(),
                        font_size: desc_region.font_size,
                        font_weight: Some("normal".to_string()),
                    },
                    position: desc_region.position.clone(),
                    size: desc_region.size.clone(),
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

    pub fn get_all_layouts(&self) -> &[ChartLayout] {
        &self.layout_variants
    }

    pub fn get_layout_by_name(&self, name: &str) -> Option<&ChartLayout> {
        self.layout_variants.iter().find(|l| l.name == name)
    }
}

impl PageTemplate for ChartPageTemplate {
    fn get_page_type(&self) -> PageType {
        PageType::Chart
    }

    fn get_layout(&self, _style: &StyleParams) -> Layout {
        let chart_layout = &self.layout_variants[0];
        
        Layout {
            name: chart_layout.name.clone(),
            page_type: PageType::Chart,
            regions: vec![
                Region {
                    name: "title".to_string(),
                    position: chart_layout.title_position.clone(),
                    size: Size::new(1200, 80),
                    region_type: RegionType::Text,
                    constraints: vec![
                        RegionConstraint::Alignment("center".to_string()),
                    ],
                },
            ],
            background: chart_layout.background_type.clone(),
            grid: None,
        }
    }

    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement> {
        let chart_content = ChartPageContent {
            title: content.title.clone().unwrap_or_default(),
            charts: content.charts.clone().unwrap_or_default(),
            description: None,
        };

        let chart_layout = self.get_layout_by_name(&layout.name)
            .cloned()
            .unwrap_or_else(|| self.layout_variants[0].clone());

        self.apply_content(&chart_layout, &chart_content)
    }
}

impl Default for ChartPageTemplate {
    fn default() -> Self {
        Self::new()
    }
}
