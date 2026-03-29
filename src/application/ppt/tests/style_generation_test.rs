#[cfg(test)]
mod tests {
    use crate::application::ppt::style_consistency_engine::StyleConsistencyEngine;
    use crate::domain::model::m_style::*;

    fn create_test_color_scheme() -> ColorScheme {
        ColorScheme {
            primary_color: "#0066FF".to_string(),
            secondary_color: "#FF6B00".to_string(),
            accent_color: Some("#00D4FF".to_string()),
            background_color: "#FFFFFF".to_string(),
            text_color: "#1A1A1A".to_string(),
            text_secondary_color: Some("#666666".to_string()),
            gradient_colors: Some(vec!["#0066FF".to_string(), "#00D4FF".to_string()]),
            color_palette: vec!["#0066FF".to_string(), "#FF6B00".to_string(), "#00D4FF".to_string()],
        }
    }

    fn create_test_font_scheme() -> FontScheme {
        FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 44,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 28,
                weight: "medium".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "regular".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 14,
                weight: "light".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        }
    }

    #[test]
    fn test_color_consistency_valid() {
        let engine = StyleConsistencyEngine::new();
        let scheme = create_test_color_scheme();
        
        let result = engine.validate_colors(&scheme);
        
        assert!(result.is_valid);
        assert!(result.score > 0.7);
    }

    #[test]
    fn test_color_consistency_invalid() {
        let engine = StyleConsistencyEngine::new();
        let scheme = ColorScheme {
            primary_color: "#FF0000".to_string(),
            secondary_color: "#00FF00".to_string(),
            accent_color: Some("#0000FF".to_string()),
            background_color: "#FFFF00".to_string(),
            text_color: "#FF00FF".to_string(),
            text_secondary_color: Some("#00FFFF".to_string()),
            gradient_colors: None,
            color_palette: vec![],
        };
        
        let result = engine.validate_colors(&scheme);
        
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_font_consistency_valid() {
        let engine = StyleConsistencyEngine::new();
        let scheme = create_test_font_scheme();
        
        let result = engine.validate_fonts(&scheme);
        
        assert!(result.is_valid);
        assert!(result.score > 0.8);
    }

    #[test]
    fn test_font_size_hierarchy() {
        let engine = StyleConsistencyEngine::new();
        let scheme = FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 18,
                weight: "regular".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 44,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 28,
                weight: "medium".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 36,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
        };
        
        let result = engine.validate_fonts(&scheme);
        
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_layout_consistency() {
        let engine = StyleConsistencyEngine::new();
        let rules = LayoutRules {
            page_margin: Margin {
                top: 40,
                right: 40,
                bottom: 40,
                left: 40,
            },
            content_padding: Padding {
                top: 20,
                right: 20,
                bottom: 20,
                left: 20,
            },
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing {
                small: 8,
                medium: 16,
                large: 24,
            },
        };
        
        let result = engine.validate_layout(&rules);
        
        assert!(result.is_valid);
    }

    #[test]
    fn test_harmony_score() {
        let engine = StyleConsistencyEngine::new();
        let params = StyleParams {
            color_scheme: create_test_color_scheme(),
            font_scheme: create_test_font_scheme(),
            layout_rules: LayoutRules::default(),
            component_styles: ComponentStyles::default(),
        };
        
        let score = engine.calculate_harmony_score(&params);
        
        assert!(score >= 0.0 && score <= 100.0);
    }
}
