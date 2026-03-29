use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleParams {
    pub color_scheme: ColorScheme,
    pub font_scheme: FontScheme,
    pub layout_rules: LayoutRules,
    pub component_styles: ComponentStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub background_color: String,
    pub text_color: String,
    pub text_secondary_color: String,
    pub gradient_colors: Option<Vec<String>>,
    pub color_palette: Vec<String>,
}

impl ColorScheme {
    pub fn validate_hex_color(color: &str) -> bool {
        let color = color.trim();
        if color.len() != 7 && color.len() != 4 {
            return false;
        }
        if !color.starts_with('#') {
            return false;
        }
        let hex_chars: &str = &color[1..];
        hex_chars.chars().all(|c| c.is_ascii_hexdigit())
    }

    pub fn calculate_luminance(hex: &str) -> f32 {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        
        let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
        let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
        let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn calculate_contrast_ratio(color1: &str, color2: &str) -> f32 {
        let l1 = Self::calculate_luminance(color1);
        let l2 = Self::calculate_luminance(color2);
        
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        
        (lighter + 0.05) / (darker + 0.05)
    }

    pub fn hex_to_hsv(hex: &str) -> (f32, f32, f32) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s * 100.0, v * 100.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontScheme {
    pub title_font: FontConfig,
    pub subtitle_font: FontConfig,
    pub body_font: FontConfig,
    pub caption_font: FontConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: u32,
    pub weight: String,
    pub line_height: f32,
    pub letter_spacing: f32,
}

impl FontConfig {
    pub fn validate_weight(weight: &str) -> bool {
        matches!(weight, "normal" | "bold" | "lighter" | "bolder" | "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900")
    }

    pub fn calculate_scale_ratio(&self, base: &FontConfig) -> f32 {
        self.size as f32 / base.size as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutRules {
    pub page_margin: Margin,
    pub content_padding: Padding,
    pub grid_columns: u32,
    pub grid_gap: u32,
    pub element_spacing: Spacing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Margin {
    pub fn new(all: u32) -> Self {
        Self {
            top: all,
            right: all,
            bottom: all,
            left: all,
        }
    }

    pub fn new_vertical_horizontal(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Padding {
    pub fn new(all: u32) -> Self {
        Self {
            top: all,
            right: all,
            bottom: all,
            left: all,
        }
    }

    pub fn new_vertical_horizontal(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub small: u32,
    pub medium: u32,
    pub large: u32,
    pub extra_large: u32,
}

impl Spacing {
    pub fn new(base: u32) -> Self {
        Self {
            small: base / 2,
            medium: base,
            large: base * 2,
            extra_large: base * 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyles {
    pub title_style: TitleStyle,
    pub text_style: TextStyle,
    pub image_style: ImageStyle,
    pub shape_style: ShapeStyle,
    pub chart_style: ChartStyle,
    pub table_style: TableStyle,
}

impl Default for ComponentStyles {
    fn default() -> Self {
        Self {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: None,
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.5,
                paragraph_spacing: 10,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 0,
                shadow: None,
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 0,
                border_width: 0,
                border_color: "#000000".to_string(),
                shadow: None,
                fill_type: "solid".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec![],
                border_radius: 0,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E0E0E0".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#F5F5F5".to_string(),
                    text_color: "#333333".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(8),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E0E0E0".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleStyle {
    pub alignment: String,
    pub position: Position,
    pub animation: Option<String>,
}

impl TitleStyle {
    pub fn validate_alignment(alignment: &str) -> bool {
        matches!(alignment, "left" | "center" | "right" | "justify")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub alignment: String,
    pub line_spacing: f32,
    pub paragraph_spacing: u32,
    pub indent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStyle {
    pub border_radius: u32,
    pub shadow: Option<Shadow>,
    pub filter: Option<String>,
}

impl ImageStyle {
    pub fn validate_filter(filter: &str) -> bool {
        matches!(filter, "none" | "grayscale" | "sepia" | "blur" | "brightness" | "contrast" | "saturate")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    pub border_radius: u32,
    pub border_width: u32,
    pub border_color: String,
    pub shadow: Option<Shadow>,
    pub fill_type: String,
}

impl ShapeStyle {
    pub fn validate_fill_type(fill_type: &str) -> bool {
        matches!(fill_type, "solid" | "gradient" | "pattern" | "none")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
    pub color_palette: Vec<String>,
    pub border_radius: u32,
    pub show_legend: bool,
    pub legend_position: String,
    pub show_grid: bool,
    pub grid_color: String,
}

impl ChartStyle {
    pub fn validate_legend_position(position: &str) -> bool {
        matches!(position, "top" | "bottom" | "left" | "right")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStyle {
    pub header_style: TableHeaderStyle,
    pub cell_style: TableCellStyle,
    pub border_style: TableBorderStyle,
    pub stripe_rows: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHeaderStyle {
    pub background_color: String,
    pub text_color: String,
    pub font_weight: String,
    pub alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCellStyle {
    pub padding: Padding,
    pub alignment: String,
    pub vertical_alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableBorderStyle {
    pub width: u32,
    pub color: String,
    pub style: String,
}

impl TableBorderStyle {
    pub fn validate_style(style: &str) -> bool {
        matches!(style, "solid" | "dashed" | "dotted" | "none")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    pub x: i32,
    pub y: i32,
    pub blur: u32,
    pub color: String,
}

impl Shadow {
    pub fn new(x: i32, y: i32, blur: u32, color: String) -> Self {
        Self { x, y, blur, color }
    }

    pub fn small() -> Self {
        Self {
            x: 0,
            y: 2,
            blur: 4,
            color: "rgba(0, 0, 0, 0.1)".to_string(),
        }
    }

    pub fn medium() -> Self {
        Self {
            x: 0,
            y: 4,
            blur: 8,
            color: "rgba(0, 0, 0, 0.15)".to_string(),
        }
    }

    pub fn large() -> Self {
        Self {
            x: 0,
            y: 8,
            blur: 16,
            color: "rgba(0, 0, 0, 0.2)".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylePreferences {
    pub industry: Option<String>,
    pub mood: Option<String>,
    pub color_preferences: Option<Vec<String>>,
    pub font_preferences: Option<Vec<String>>,
    pub layout_preference: Option<String>,
}

impl StylePreferences {
    pub fn validate_mood(mood: &str) -> bool {
        matches!(mood, "专业" | "活泼" | "简约" | "科技感" | "温馨" | "商务" | "教育" | "医疗")
    }

    pub fn validate_layout_preference(preference: &str) -> bool {
        matches!(preference, "居中" | "左对齐" | "网格" | "自由" | "对称")
    }
}
