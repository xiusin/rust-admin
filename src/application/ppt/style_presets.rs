use crate::domain::model::m_style::*;

pub fn get_preset_styles() -> Vec<StyleParams> {
    vec![
        create_tech_blue_style(),
        create_business_blue_style(),
        create_education_green_style(),
        create_medical_blue_style(),
        create_ecommerce_orange_style(),
        create_government_red_style(),
        create_minimalist_white_style(),
        create_creative_gradient_style(),
    ]
}

pub fn get_preset_style_by_name(name: &str) -> Option<StyleParams> {
    let styles = get_preset_styles();
    styles.into_iter().find(|s| {
        let style_name = format!("{:?}", s.color_scheme.primary_color);
        style_name.contains(name) || name.contains(&style_name)
    })
}

pub fn get_preset_style_by_industry(industry: &str) -> Option<StyleParams> {
    let styles = get_preset_styles();
    styles.into_iter().find(|s| {
        matches!(
            industry,
            "科技" | "技术" if s.color_scheme.primary_color == "#0066FF"
        ) || matches!(
            industry,
            "金融" | "商务" if s.color_scheme.primary_color == "#1B4F72"
        ) || matches!(
            industry,
            "教育" | "培训" if s.color_scheme.primary_color == "#27AE60"
        ) || matches!(
            industry,
            "医疗" | "健康" if s.color_scheme.primary_color == "#3498DB"
        ) || matches!(
            industry,
            "电商" | "零售" if s.color_scheme.primary_color == "#FF6B6B"
        ) || matches!(
            industry,
            "政府" | "公共" if s.color_scheme.primary_color == "#C0392B"
        )
    })
}

fn create_tech_blue_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#0066FF".to_string(),
            secondary_color: "#FF6B00".to_string(),
            accent_color: "#00D4FF".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#1A1A1A".to_string(),
            text_secondary_color: "#666666".to_string(),
            gradient_colors: Some(vec!["#0066FF".to_string(), "#00D4FF".to_string()]),
            color_palette: vec![
                "#0066FF".to_string(),
                "#FF6B00".to_string(),
                "#00D4FF".to_string(),
                "#FFFFFF".to_string(),
                "#F5F5F5".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 44,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 32,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(40, 60),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(16),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: Some("fadeIn".to_string()),
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.6,
                paragraph_spacing: 16,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 8,
                shadow: Some(Shadow::medium()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 4,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::small()),
                fill_type: "gradient".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#0066FF".to_string(), "#FF6B00".to_string(), "#00D4FF".to_string()],
                border_radius: 4,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#0066FF".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(12),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_business_blue_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#1B4F72".to_string(),
            secondary_color: "#F39C12".to_string(),
            accent_color: "#2874A6".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#1A1A1A".to_string(),
            text_secondary_color: "#666666".to_string(),
            gradient_colors: Some(vec!["#1B4F72".to_string(), "#2874A6".to_string()]),
            color_palette: vec![
                "#1B4F72".to_string(),
                "#F39C12".to_string(),
                "#2874A6".to_string(),
                "#FFFFFF".to_string(),
                "#F8F9FA".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 48,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 32,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.8,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(50, 70),
            content_padding: Padding::new(24),
            grid_columns: 12,
            grid_gap: 24,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: None,
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.8,
                paragraph_spacing: 20,
                indent: 24,
            },
            image_style: ImageStyle {
                border_radius: 4,
                shadow: Some(Shadow::small()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 2,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::small()),
                fill_type: "solid".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#1B4F72".to_string(), "#F39C12".to_string(), "#2874A6".to_string()],
                border_radius: 2,
                show_legend: true,
                legend_position: "right".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#1B4F72".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(16),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_education_green_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#27AE60".to_string(),
            secondary_color: "#3498DB".to_string(),
            accent_color: "#F39C12".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2C3E50".to_string(),
            text_secondary_color: "#7F8C8D".to_string(),
            gradient_colors: Some(vec!["#27AE60".to_string(), "#2ECC71".to_string()]),
            color_palette: vec![
                "#27AE60".to_string(),
                "#3498DB".to_string(),
                "#F39C12".to_string(),
                "#FFFFFF".to_string(),
                "#F0F9F4".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 40,
                weight: "bold".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 28,
                weight: "600".to_string(),
                line_height: 1.4,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.7,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(45, 65),
            content_padding: Padding::new(22),
            grid_columns: 12,
            grid_gap: 22,
            element_spacing: Spacing::new(18),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: Some("fadeIn".to_string()),
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.7,
                paragraph_spacing: 18,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 12,
                shadow: Some(Shadow::small()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 8,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::small()),
                fill_type: "solid".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#27AE60".to_string(), "#3498DB".to_string(), "#F39C12".to_string()],
                border_radius: 8,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#27AE60".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(14),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_medical_blue_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#3498DB".to_string(),
            secondary_color: "#2ECC71".to_string(),
            accent_color: "#5DADE2".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2C3E50".to_string(),
            text_secondary_color: "#7F8C8D".to_string(),
            gradient_colors: Some(vec!["#3498DB".to_string(), "#5DADE2".to_string()]),
            color_palette: vec![
                "#3498DB".to_string(),
                "#2ECC71".to_string(),
                "#5DADE2".to_string(),
                "#FFFFFF".to_string(),
                "#F0F8FF".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 42,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 30,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.7,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(45, 65),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(16),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: None,
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.7,
                paragraph_spacing: 16,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 8,
                shadow: Some(Shadow::small()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 6,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::small()),
                fill_type: "solid".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#3498DB".to_string(), "#2ECC71".to_string(), "#5DADE2".to_string()],
                border_radius: 6,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#3498DB".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(12),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_ecommerce_orange_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#FF6B6B".to_string(),
            secondary_color: "#00D4FF".to_string(),
            accent_color: "#FFA502".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2C3E50".to_string(),
            text_secondary_color: "#7F8C8D".to_string(),
            gradient_colors: Some(vec!["#FF6B6B".to_string(), "#FFA502".to_string()]),
            color_palette: vec![
                "#FF6B6B".to_string(),
                "#00D4FF".to_string(),
                "#FFA502".to_string(),
                "#FFFFFF".to_string(),
                "#FFF5F5".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 46,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 32,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(40, 60),
            content_padding: Padding::new(18),
            grid_columns: 12,
            grid_gap: 18,
            element_spacing: Spacing::new(14),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: Some("bounceIn".to_string()),
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.6,
                paragraph_spacing: 14,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 16,
                shadow: Some(Shadow::medium()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 12,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::medium()),
                fill_type: "gradient".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#FF6B6B".to_string(), "#00D4FF".to_string(), "#FFA502".to_string()],
                border_radius: 12,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#FF6B6B".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(12),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_government_red_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#C0392B".to_string(),
            secondary_color: "#F1C40F".to_string(),
            accent_color: "#E74C3C".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2C3E50".to_string(),
            text_secondary_color: "#7F8C8D".to_string(),
            gradient_colors: Some(vec!["#C0392B".to_string(), "#E74C3C".to_string()]),
            color_palette: vec![
                "#C0392B".to_string(),
                "#F1C40F".to_string(),
                "#E74C3C".to_string(),
                "#FFFFFF".to_string(),
                "#FFF8F0".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 48,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 34,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.8,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源宋体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(50, 70),
            content_padding: Padding::new(24),
            grid_columns: 12,
            grid_gap: 24,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: None,
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.8,
                paragraph_spacing: 20,
                indent: 28,
            },
            image_style: ImageStyle {
                border_radius: 4,
                shadow: Some(Shadow::small()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 2,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::small()),
                fill_type: "solid".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#C0392B".to_string(), "#F1C40F".to_string(), "#E74C3C".to_string()],
                border_radius: 2,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#C0392B".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(16),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}

fn create_minimalist_white_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#2C3E50".to_string(),
            secondary_color: "#95A5A6".to_string(),
            accent_color: "#34495E".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2C3E50".to_string(),
            text_secondary_color: "#7F8C8D".to_string(),
            gradient_colors: None,
            color_palette: vec![
                "#2C3E50".to_string(),
                "#95A5A6".to_string(),
                "#34495E".to_string(),
                "#FFFFFF".to_string(),
                "#F8F9FA".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 40,
                weight: "bold".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 28,
                weight: "500".to_string(),
                line_height: 1.4,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.7,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(60),
            content_padding: Padding::new(24),
            grid_columns: 12,
            grid_gap: 24,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "left".to_string(),
                position: Position { x: 0, y: 0 },
                animation: None,
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.7,
                paragraph_spacing: 18,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 0,
                shadow: None,
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 0,
                border_width: 1,
                border_color: "#E5E5E5".to_string(),
                shadow: None,
                fill_type: "none".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#2C3E50".to_string(), "#95A5A6".to_string(), "#34495E".to_string()],
                border_radius: 0,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#2C3E50".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "left".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(16),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: false,
            },
        },
    }
}

fn create_creative_gradient_style() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#6C5CE7".to_string(),
            secondary_color: "#FD79A8".to_string(),
            accent_color: "#00CEC9".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#2D3436".to_string(),
            text_secondary_color: "#636E72".to_string(),
            gradient_colors: Some(vec!["#6C5CE7".to_string(), "#FD79A8".to_string(), "#00CEC9".to_string()]),
            color_palette: vec![
                "#6C5CE7".to_string(),
                "#FD79A8".to_string(),
                "#00CEC9".to_string(),
                "#FFFFFF".to_string(),
                "#F8F9FA".to_string(),
            ],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 44,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 1.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 30,
                weight: "600".to_string(),
                line_height: 1.3,
                letter_spacing: 0.5,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new_vertical_horizontal(40, 60),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(16),
        },
        component_styles: ComponentStyles {
            title_style: TitleStyle {
                alignment: "center".to_string(),
                position: Position { x: 0, y: 0 },
                animation: Some("slideIn".to_string()),
            },
            text_style: TextStyle {
                alignment: "left".to_string(),
                line_spacing: 1.6,
                paragraph_spacing: 16,
                indent: 0,
            },
            image_style: ImageStyle {
                border_radius: 20,
                shadow: Some(Shadow::large()),
                filter: None,
            },
            shape_style: ShapeStyle {
                border_radius: 16,
                border_width: 0,
                border_color: "transparent".to_string(),
                shadow: Some(Shadow::large()),
                fill_type: "gradient".to_string(),
            },
            chart_style: ChartStyle {
                color_palette: vec!["#6C5CE7".to_string(), "#FD79A8".to_string(), "#00CEC9".to_string()],
                border_radius: 16,
                show_legend: true,
                legend_position: "bottom".to_string(),
                show_grid: true,
                grid_color: "#E5E5E5".to_string(),
            },
            table_style: TableStyle {
                header_style: TableHeaderStyle {
                    background_color: "#6C5CE7".to_string(),
                    text_color: "#FFFFFF".to_string(),
                    font_weight: "bold".to_string(),
                    alignment: "center".to_string(),
                },
                cell_style: TableCellStyle {
                    padding: Padding::new(14),
                    alignment: "left".to_string(),
                    vertical_alignment: "middle".to_string(),
                },
                border_style: TableBorderStyle {
                    width: 1,
                    color: "#E5E5E5".to_string(),
                    style: "solid".to_string(),
                },
                stripe_rows: true,
            },
        },
    }
}
