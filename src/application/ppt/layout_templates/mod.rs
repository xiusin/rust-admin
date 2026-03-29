pub mod title_page;
pub mod toc_page;
pub mod content_page;
pub mod chart_page;
pub mod end_page;

pub use title_page::TitlePageTemplate;
pub use toc_page::TocPageTemplate;
pub use content_page::ContentPageTemplate;
pub use chart_page::ChartPageTemplate;
pub use end_page::EndPageTemplate;

use serde::{Deserialize, Serialize};
use crate::domain::model::m_style::{Position, StyleParams};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageElement {
    pub element_type: String,
    pub content: ElementContent,
    pub position: Position,
    pub size: Size,
    pub style: Option<ElementStyle>,
    pub z_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementContent {
    Text { text: String, font_size: Option<u32>, font_weight: Option<String> },
    Image { url: String, alt: Option<String> },
    Shape { shape_type: String, fill_color: String },
    Chart { chart_type: String, data: serde_json::Value },
    List { items: Vec<String>, bullet_style: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStyle {
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub border_radius: Option<u32>,
    pub opacity: Option<f32>,
    pub shadow: Option<Shadow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    pub x: i32,
    pub y: i32,
    pub blur: u32,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundType {
    Solid(String),
    Gradient { start_color: String, end_color: String, angle: u32 },
    Image { url: String, opacity: f32 },
    Pattern { pattern_type: String, color: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PageType {
    Title,
    Toc,
    Content,
    Chart,
    End,
}

pub trait PageTemplate: Send + Sync {
    fn get_page_type(&self) -> PageType;
    fn get_layout(&self, style: &StyleParams) -> Layout;
    fn apply_content(&self, layout: &Layout, content: &PageContent) -> Vec<PageElement>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub name: String,
    pub page_type: PageType,
    pub regions: Vec<Region>,
    pub background: BackgroundType,
    pub grid: Option<LayoutGrid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub position: Position,
    pub size: Size,
    pub region_type: RegionType,
    pub constraints: Vec<RegionConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionType {
    Text,
    Image,
    Shape,
    Chart,
    List,
    Container,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionConstraint {
    MinWidth(u32),
    MaxWidth(u32),
    MinHeight(u32),
    MaxHeight(u32),
    AspectRatio(f32),
    Alignment(String),
    Spacing(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutGrid {
    pub columns: u32,
    pub rows: u32,
    pub gap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContent {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub body: Option<String>,
    pub items: Option<Vec<String>>,
    pub images: Option<Vec<ImageContent>>,
    pub charts: Option<Vec<ChartContent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub url: String,
    pub alt: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartContent {
    pub chart_type: String,
    pub title: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleContent {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub logo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocContent {
    pub title: Option<String>,
    pub items: Vec<TocItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocItem {
    pub title: String,
    pub page_number: Option<u32>,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionContent {
    pub title: String,
    pub body: Option<String>,
    pub bullet_points: Option<Vec<String>>,
    pub images: Option<Vec<ImageContent>>,
    pub layout_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPageContent {
    pub title: String,
    pub charts: Vec<ChartContent>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndContent {
    pub title: Option<String>,
    pub message: Option<String>,
    pub contact_info: Option<ContactInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}
