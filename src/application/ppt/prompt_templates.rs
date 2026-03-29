use crate::domain::model::m_style::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub industry: String,
    pub description: String,
    pub template: String,
    pub style_params: StyleParams,
    pub tags: Vec<String>,
    pub preview_url: Option<String>,
}

pub fn get_prompt_templates() -> Vec<PromptTemplate> {
    vec![
        PromptTemplate {
            id: "tech_blue".to_string(),
            name: "科技蓝风格".to_string(),
            industry: "科技互联网".to_string(),
            description: "现代、科技感、简洁的蓝色主题风格".to_string(),
            template: r#"生成一份科技互联网风格的PPT演示文稿。

整体风格：现代、科技感、简洁
主色调：科技蓝 #0066FF（占比60%）
辅助色：活力橙 #FF6B00（占比30%）
强调色：科技青 #00D4FF（占比10%）
背景：渐变科技感背景

字体规范：
- 标题：思源黑体 Bold，44px
- 正文：思源黑体 Regular，18px
- 说明：思源黑体 Light，14px

布局规则：
- 采用现代简约布局
- 大量留白
- 元素居中对齐
- 使用几何图形装饰

视觉元素：
- 图标：扁平化线性图标
- 图片：科技感图片，带蓝色滤镜
- 装饰：渐变线条、几何形状"#.to_string(),
            style_params: get_tech_blue_style_params(),
            tags: vec!["科技".to_string(), "现代".to_string(), "蓝色".to_string()],
            preview_url: None,
        },
        PromptTemplate {
            id: "business_pro".to_string(),
            name: "商务专业风格".to_string(),
            industry: "商务金融".to_string(),
            description: "专业、稳重、大气的商务风格".to_string(),
            template: r#"生成一份商务专业风格的PPT演示文稿。

整体风格：专业、稳重、大气
主色调：商务蓝 #1E3A8A（占比50%）
辅助色：金色 #D97706（占比30%）
强调色：深灰 #374151（占比20%）
背景：纯白或浅灰渐变

字体规范：
- 标题：微软雅黑 Bold，40px
- 正文：微软雅黑 Regular，16px
- 说明：微软雅黑 Light，12px

布局规则：
- 采用对称式布局
- 适度留白
- 左对齐为主
- 使用商务图表

视觉元素：
- 图标：线性图标
- 图片：商务场景图片
- 装饰：简约线条、数据图表"#.to_string(),
            style_params: get_business_pro_style_params(),
            tags: vec!["商务".to_string(), "专业".to_string(), "蓝色".to_string()],
            preview_url: None,
        },
        PromptTemplate {
            id: "creative_colorful".to_string(),
            name: "创意多彩风格".to_string(),
            industry: "创意设计".to_string(),
            description: "活泼、创意、多彩的设计风格".to_string(),
            template: r#"生成一份创意多彩风格的PPT演示文稿。

整体风格：活泼、创意、多彩
主色调：活力紫 #8B5CF6（占比40%）
辅助色：明黄 #FBBF24（占比30%）
强调色：粉红 #EC4899（占比30%）
背景：渐变彩色背景

字体规范：
- 标题：思源宋体 Bold，48px
- 正文：思源黑体 Regular，18px
- 说明：思源黑体 Light，14px

布局规则：
- 采用自由式布局
- 动态留白
- 混合对齐
- 使用创意形状

视觉元素：
- 图标：彩色填充图标
- 图片：创意插画
- 装饰：几何形状、波浪线条"#.to_string(),
            style_params: get_creative_colorful_style_params(),
            tags: vec!["创意".to_string(), "多彩".to_string(), "活泼".to_string()],
            preview_url: None,
        },
        PromptTemplate {
            id: "education_green".to_string(),
            name: "教育清新风格".to_string(),
            industry: "教育培训".to_string(),
            description: "清新、友好、易读的教育风格".to_string(),
            template: r#"生成一份教育清新风格的PPT演示文稿。

整体风格：清新、友好、易读
主色调：清新绿 #10B981（占比50%）
辅助色：天蓝 #3B82F6（占比30%）
强调色：橙色 #F59E0B（占比20%）
背景：浅米色或白色

字体规范：
- 标题：思源黑体 Bold，42px
- 正文：思源宋体 Regular，18px
- 说明：思源黑体 Regular，14px

布局规则：
- 采用卡片式布局
- 充足留白
- 居中对齐
- 使用圆角矩形

视觉元素：
- 图标：圆润可爱图标
- 图片：教育场景插画
- 装饰：圆点、波浪线"#.to_string(),
            style_params: get_education_green_style_params(),
            tags: vec!["教育".to_string(), "清新".to_string(), "绿色".to_string()],
            preview_url: None,
        },
        PromptTemplate {
            id: "medical_blue".to_string(),
            name: "医疗健康风格".to_string(),
            industry: "医疗健康".to_string(),
            description: "专业、可信、温暖的健康风格".to_string(),
            template: r#"生成一份医疗健康风格的PPT演示文稿。

整体风格：专业、可信、温暖
主色调：医疗蓝 #0EA5E9（占比50%）
辅助色：健康绿 #22C55E（占比30%）
强调色：温暖橙 #FB923C（占比20%）
背景：浅蓝渐变或纯白

字体规范：
- 标题：思源黑体 Medium，40px
- 正文：思源黑体 Regular，16px
- 说明：思源黑体 Light，12px

布局规则：
- 采用模块化布局
- 清晰留白
- 左对齐
- 使用医疗图标

视觉元素：
- 图标：医疗健康图标
- 图片：医疗场景图片
- 装饰：圆角卡片、数据图表"#.to_string(),
            style_params: get_medical_blue_style_params(),
            tags: vec!["医疗".to_string(), "健康".to_string(), "蓝色".to_string()],
            preview_url: None,
        },
        PromptTemplate {
            id: "minimalist_white".to_string(),
            name: "极简白风格".to_string(),
            industry: "通用".to_string(),
            description: "极简、优雅、高端的白色主题".to_string(),
            template: r#"生成一份极简白风格的PPT演示文稿。

整体风格：极简、优雅、高端
主色调：深灰 #18181B（占比60%）
辅助色：浅灰 #71717A（占比30%）
强调色：黑色 #000000（占比10%）
背景：纯白 #FFFFFF

字体规范：
- 标题：Helvetica Neue Bold，48px
- 正文：Helvetica Neue Regular，16px
- 说明：Helvetica Neue Light，12px

布局规则：
- 采用极简布局
- 大量留白
- 居中对齐
- 无多余装饰

视觉元素：
- 图标：细线图标
- 图片：黑白或单色图片
- 装饰：极简线条"#.to_string(),
            style_params: get_minimalist_white_style_params(),
            tags: vec!["极简".to_string(), "白色".to_string(), "高端".to_string()],
            preview_url: None,
        },
    ]
}

fn get_tech_blue_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#0066FF".to_string(),
            secondary_color: "#FF6B00".to_string(),
            accent_color: "#00D4FF".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#333333".to_string(),
            text_secondary_color: "#666666".to_string(),
            gradient_colors: Some(vec!["#0066FF".to_string(), "#00D4FF".to_string()]),
            color_palette: vec!["#0066FF".to_string(), "#FF6B00".to_string(), "#00D4FF".to_string()],
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
                size: 24,
                weight: "normal".to_string(),
                line_height: 1.4,
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
                weight: "lighter".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(80),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles::default(),
    }
}

fn get_business_pro_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#1E3A8A".to_string(),
            secondary_color: "#D97706".to_string(),
            accent_color: "#374151".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#1F2937".to_string(),
            text_secondary_color: "#6B7280".to_string(),
            gradient_colors: None,
            color_palette: vec!["#1E3A8A".to_string(), "#D97706".to_string(), "#374151".to_string()],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "微软雅黑".to_string(),
                size: 40,
                weight: "bold".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "微软雅黑".to_string(),
                size: 22,
                weight: "normal".to_string(),
                line_height: 1.4,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "微软雅黑".to_string(),
                size: 16,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "微软雅黑".to_string(),
                size: 12,
                weight: "lighter".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(100),
            content_padding: Padding::new(24),
            grid_columns: 12,
            grid_gap: 24,
            element_spacing: Spacing::new(24),
        },
        component_styles: ComponentStyles::default(),
    }
}

fn get_creative_colorful_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#8B5CF6".to_string(),
            secondary_color: "#FBBF24".to_string(),
            accent_color: "#EC4899".to_string(),
            background_color: "#FAFAFA".to_string(),
            text_color: "#1F2937".to_string(),
            text_secondary_color: "#6B7280".to_string(),
            gradient_colors: Some(vec!["#8B5CF6".to_string(), "#EC4899".to_string(), "#FBBF24".to_string()]),
            color_palette: vec!["#8B5CF6".to_string(), "#FBBF24".to_string(), "#EC4899".to_string()],
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
                family: "思源黑体".to_string(),
                size: 24,
                weight: "normal".to_string(),
                line_height: 1.4,
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
                weight: "lighter".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(60),
            content_padding: Padding::new(16),
            grid_columns: 12,
            grid_gap: 16,
            element_spacing: Spacing::new(16),
        },
        component_styles: ComponentStyles::default(),
    }
}

fn get_education_green_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#10B981".to_string(),
            secondary_color: "#3B82F6".to_string(),
            accent_color: "#F59E0B".to_string(),
            background_color: "#FFFBEB".to_string(),
            text_color: "#1F2937".to_string(),
            text_secondary_color: "#6B7280".to_string(),
            gradient_colors: None,
            color_palette: vec!["#10B981".to_string(), "#3B82F6".to_string(), "#F59E0B".to_string()],
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
                size: 22,
                weight: "normal".to_string(),
                line_height: 1.4,
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
                family: "思源黑体".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(80),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles::default(),
    }
}

fn get_medical_blue_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#0EA5E9".to_string(),
            secondary_color: "#22C55E".to_string(),
            accent_color: "#FB923C".to_string(),
            background_color: "#F0F9FF".to_string(),
            text_color: "#1F2937".to_string(),
            text_secondary_color: "#6B7280".to_string(),
            gradient_colors: None,
            color_palette: vec!["#0EA5E9".to_string(), "#22C55E".to_string(), "#FB923C".to_string()],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 40,
                weight: "500".to_string(),
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            subtitle_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 22,
                weight: "normal".to_string(),
                line_height: 1.4,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 16,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "思源黑体".to_string(),
                size: 12,
                weight: "lighter".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(80),
            content_padding: Padding::new(20),
            grid_columns: 12,
            grid_gap: 20,
            element_spacing: Spacing::new(20),
        },
        component_styles: ComponentStyles::default(),
    }
}

fn get_minimalist_white_style_params() -> StyleParams {
    StyleParams {
        color_scheme: ColorScheme {
            primary_color: "#18181B".to_string(),
            secondary_color: "#71717A".to_string(),
            accent_color: "#000000".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#18181B".to_string(),
            text_secondary_color: "#71717A".to_string(),
            gradient_colors: None,
            color_palette: vec!["#18181B".to_string(), "#71717A".to_string(), "#000000".to_string()],
        },
        font_scheme: FontScheme {
            title_font: FontConfig {
                family: "Helvetica Neue".to_string(),
                size: 48,
                weight: "bold".to_string(),
                line_height: 1.1,
                letter_spacing: -1.0,
            },
            subtitle_font: FontConfig {
                family: "Helvetica Neue".to_string(),
                size: 24,
                weight: "normal".to_string(),
                line_height: 1.3,
                letter_spacing: 0.0,
            },
            body_font: FontConfig {
                family: "Helvetica Neue".to_string(),
                size: 16,
                weight: "normal".to_string(),
                line_height: 1.6,
                letter_spacing: 0.0,
            },
            caption_font: FontConfig {
                family: "Helvetica Neue".to_string(),
                size: 12,
                weight: "lighter".to_string(),
                line_height: 1.5,
                letter_spacing: 0.0,
            },
        },
        layout_rules: LayoutRules {
            page_margin: Margin::new(120),
            content_padding: Padding::new(24),
            grid_columns: 12,
            grid_gap: 24,
            element_spacing: Spacing::new(24),
        },
        component_styles: ComponentStyles::default(),
    }
}

pub fn get_template_by_id(id: &str) -> Option<PromptTemplate> {
    get_prompt_templates().into_iter().find(|t| t.id == id)
}

pub fn get_templates_by_industry(industry: &str) -> Vec<PromptTemplate> {
    get_prompt_templates()
        .into_iter()
        .filter(|t| t.industry == industry || t.industry == "通用")
        .collect()
}

pub fn get_templates_by_tag(tag: &str) -> Vec<PromptTemplate> {
    get_prompt_templates()
        .into_iter()
        .filter(|t| t.tags.contains(&tag.to_string()))
        .collect()
}
