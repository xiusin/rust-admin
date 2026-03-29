use crate::common::error::{Error, Result};
use crate::domain::model::m_document::ParsedDocument;
use crate::domain::model::m_style::*;
use super::document_parse_service::{DocumentParseService, DocType};
use super::style_generation_service::StyleGenerationService;
use super::page_generator::{PageGenerator, GeneratedPage, OutlineItem, TitleContent, ChartContent};
use super::content_filler::ContentFiller;
use super::style_applier::StyleApplier;
use super::ppt_exporter::{PptExporter, PPTProject, OutputFormat};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    pub output_format: OutputFormat,
    pub output_path: Option<PathBuf>,
    pub style_preset: Option<String>,
    pub custom_style: Option<StyleParams>,
    pub include_toc: bool,
    pub include_end_page: bool,
    pub max_pages: Option<usize>,
    pub industry: Option<String>,
    pub mood: Option<String>,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Pptx,
            output_path: None,
            style_preset: None,
            custom_style: None,
            include_toc: true,
            include_end_page: true,
            max_pages: None,
            industry: None,
            mood: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub project_id: i64,
    pub output_path: PathBuf,
    pub page_count: usize,
    pub style: StyleParams,
    pub analysis: Option<ContentAnalysis>,
    pub generation_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    pub keywords: Vec<(String, f32)>,
    pub industry: Option<String>,
    pub industry_confidence: Option<f32>,
    pub word_count: usize,
    pub section_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub task_id: String,
    pub stage: GenerationStage,
    pub progress: u8,
    pub message: String,
    pub result: Option<GenerationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenerationStage {
    Parsing,
    Analyzing,
    StyleGenerating,
    PageGenerating,
    Exporting,
    Completed,
    Failed,
}

pub struct PPTGenerationService {
    style_generator: Arc<StyleGenerationService>,
    page_generator: Arc<PageGenerator>,
    #[allow(dead_code)]
    content_filler: Arc<ContentFiller>,
    style_applier: Arc<StyleApplier>,
    exporter: Arc<PptExporter>,
}

impl PPTGenerationService {
    pub fn new() -> Self {
        Self {
            style_generator: Arc::new(StyleGenerationService::new()),
            page_generator: Arc::new(PageGenerator::new()),
            content_filler: Arc::new(ContentFiller::new()),
            style_applier: Arc::new(StyleApplier::new()),
            exporter: Arc::new(PptExporter::default()),
        }
    }

    pub fn with_exporter(exporter: PptExporter) -> Self {
        Self {
            style_generator: Arc::new(StyleGenerationService::new()),
            page_generator: Arc::new(PageGenerator::new()),
            content_filler: Arc::new(ContentFiller::new()),
            style_applier: Arc::new(StyleApplier::new()),
            exporter: Arc::new(exporter),
        }
    }

    pub async fn generate_from_document(
        &self,
        file_path: &Path,
        file_type: DocType,
        options: GenerationOptions,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();
        
        let document = DocumentParseService::parse_document(file_path, file_type).await?;
        
        let analysis = self.analyze_content(&document).await?;
        
        let style = if let Some(ref custom_style) = options.custom_style {
            custom_style.clone()
        } else if let Some(ref industry) = options.industry {
            self.style_generator.generate_from_industry(industry, 0.8).await?
        } else if let Some(ref detected_industry) = analysis.industry {
            let confidence = analysis.industry_confidence.unwrap_or(0.5);
            self.style_generator.generate_from_industry(detected_industry, confidence).await?
        } else {
            self.style_generator.generate_from_content(
                &document.to_text(),
                &analysis.keywords,
            ).await?
        };

        let mut pages = self.page_generator.generate_all_pages(
            &document,
            &style,
            options.include_toc,
            options.include_end_page,
        ).await?;

        if let Some(max_pages) = options.max_pages {
            pages.truncate(max_pages);
        }

        for page in pages.iter_mut() {
            self.style_applier.apply_all_styles(page, &style);
        }

        let project = PPTProject {
            id: chrono::Utc::now().timestamp_millis(),
            title: document.title.clone(),
            description: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let output_path = options.output_path.clone()
            .unwrap_or_else(|| self.exporter.get_output_path(project.id, &options.output_format));

        let final_path = self.export_ppt(&project, &pages, &output_path, &options.output_format).await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(GenerationResult {
            project_id: project.id,
            output_path: final_path,
            page_count: pages.len(),
            style,
            analysis: Some(analysis),
            generation_time_ms,
        })
    }

    pub async fn generate_from_text(
        &self,
        text: &str,
        title: &str,
        options: GenerationOptions,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();

        let document = self.text_to_document(text, title);

        let analysis = self.analyze_content(&document).await?;

        let style = if let Some(ref custom_style) = options.custom_style {
            custom_style.clone()
        } else if let Some(ref industry) = options.industry {
            self.style_generator.generate_from_industry(industry, 0.8).await?
        } else {
            self.style_generator.generate_from_content(text, &analysis.keywords).await?
        };

        let mut pages = self.page_generator.generate_all_pages(
            &document,
            &style,
            options.include_toc,
            options.include_end_page,
        ).await?;

        if let Some(max_pages) = options.max_pages {
            pages.truncate(max_pages);
        }

        for page in pages.iter_mut() {
            self.style_applier.apply_all_styles(page, &style);
        }

        let project = PPTProject {
            id: chrono::Utc::now().timestamp_millis(),
            title: title.to_string(),
            description: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let output_path = options.output_path.clone()
            .unwrap_or_else(|| self.exporter.get_output_path(project.id, &options.output_format));

        let final_path = self.export_ppt(&project, &pages, &output_path, &options.output_format).await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(GenerationResult {
            project_id: project.id,
            output_path: final_path,
            page_count: pages.len(),
            style,
            analysis: Some(analysis),
            generation_time_ms,
        })
    }

    pub async fn generate_from_outline(
        &self,
        outline: &[OutlineItem],
        title: &str,
        style: Option<StyleParams>,
        options: GenerationOptions,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();

        let final_style = if let Some(s) = style {
            s
        } else if let Some(ref industry) = options.industry {
            self.style_generator.generate_from_industry(industry, 0.8).await?
        } else {
            self.style_generator.generate_from_industry("科技", 0.5).await?
        };

        let mut pages = Vec::new();

        let title_content = TitleContent {
            title: title.to_string(),
            subtitle: None,
            author: None,
            date: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
        };
        let title_page = self.page_generator.generate_title_page(&title_content, &final_style).await?;
        pages.push(title_page);

        if options.include_toc {
            let toc_page = self.page_generator.generate_toc_page(outline, &final_style).await?;
            pages.push(toc_page);
        }

        for (index, item) in outline.iter().enumerate() {
            let section = crate::domain::model::m_document::Section {
                level: item.level,
                title: item.title.clone(),
                content: vec![crate::domain::model::m_document::ContentBlock::Text(
                    crate::domain::model::m_document::TextBlock {
                        text: format!("{} 的内容", item.title),
                        style: None,
                    }
                )],
                keywords: Vec::new(),
            };
            let content_page = self.page_generator.generate_content_page(&section, &final_style, index + 2).await?;
            pages.push(content_page);
        }

        if options.include_end_page {
            let end_page = self.page_generator.generate_end_page(&final_style, pages.len()).await?;
            pages.push(end_page);
        }

        if let Some(max_pages) = options.max_pages {
            pages.truncate(max_pages);
        }

        for page in pages.iter_mut() {
            self.style_applier.apply_all_styles(page, &final_style);
        }

        let project = PPTProject {
            id: chrono::Utc::now().timestamp_millis(),
            title: title.to_string(),
            description: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let output_path = options.output_path.clone()
            .unwrap_or_else(|| self.exporter.get_output_path(project.id, &options.output_format));

        let final_path = self.export_ppt(&project, &pages, &output_path, &options.output_format).await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(GenerationResult {
            project_id: project.id,
            output_path: final_path,
            page_count: pages.len(),
            style: final_style,
            analysis: None,
            generation_time_ms,
        })
    }

    pub async fn regenerate_with_style(
        &self,
        project_id: i64,
        pages: &[GeneratedPage],
        new_style: StyleParams,
        output_format: OutputFormat,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();

        let mut updated_pages = pages.to_vec();
        for page in updated_pages.iter_mut() {
            self.style_applier.apply_all_styles(page, &new_style);
        }

        let project = PPTProject {
            id: project_id,
            title: "Regenerated PPT".to_string(),
            description: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let output_path = self.exporter.get_output_path(project_id, &output_format);
        let final_path = self.export_ppt(&project, &updated_pages, &output_path, &output_format).await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(GenerationResult {
            project_id,
            output_path: final_path,
            page_count: updated_pages.len(),
            style: new_style,
            analysis: None,
            generation_time_ms,
        })
    }

    pub async fn generate_chart_ppt(
        &self,
        chart_data: &ChartContent,
        title: &str,
        style: Option<StyleParams>,
        options: GenerationOptions,
    ) -> Result<GenerationResult> {
        let start_time = std::time::Instant::now();

        let final_style = if let Some(s) = style {
            s
        } else {
            self.style_generator.generate_from_industry("科技", 0.5).await?
        };

        let mut pages = Vec::new();

        let title_content = TitleContent {
            title: title.to_string(),
            subtitle: chart_data.title.clone(),
            author: None,
            date: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
        };
        let title_page = self.page_generator.generate_title_page(&title_content, &final_style).await?;
        pages.push(title_page);

        let chart_page = self.page_generator.generate_chart_page(chart_data, &final_style, 1).await?;
        pages.push(chart_page);

        if options.include_end_page {
            let end_page = self.page_generator.generate_end_page(&final_style, pages.len()).await?;
            pages.push(end_page);
        }

        for page in pages.iter_mut() {
            self.style_applier.apply_all_styles(page, &final_style);
        }

        let project = PPTProject {
            id: chrono::Utc::now().timestamp_millis(),
            title: title.to_string(),
            description: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let output_path = options.output_path.clone()
            .unwrap_or_else(|| self.exporter.get_output_path(project.id, &options.output_format));

        let final_path = self.export_ppt(&project, &pages, &output_path, &options.output_format).await?;

        let generation_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(GenerationResult {
            project_id: project.id,
            output_path: final_path,
            page_count: pages.len(),
            style: final_style,
            analysis: None,
            generation_time_ms,
        })
    }

    async fn analyze_content(&self, document: &ParsedDocument) -> Result<ContentAnalysis> {
        let text = document.to_text();
        let keywords = self.extract_keywords(&text);
        let industry = self.detect_industry(&text, &keywords);

        Ok(ContentAnalysis {
            keywords,
            industry: industry.clone(),
            industry_confidence: if industry.is_some() { Some(0.75) } else { None },
            word_count: text.split_whitespace().count(),
            section_count: document.sections.len(),
        })
    }

    fn extract_keywords(&self, text: &str) -> Vec<(String, f32)> {
        let mut word_count: HashMap<String, usize> = HashMap::new();
        
        for word in text.split_whitespace() {
            if word.len() >= 2 {
                *word_count.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        let total_words = text.split_whitespace().count() as f32;
        
        let mut keywords: Vec<(String, f32)> = word_count
            .into_iter()
            .map(|(word, count)| {
                let score = count as f32 / total_words;
                (word, score)
            })
            .collect();

        keywords.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        keywords.truncate(10);
        keywords
    }

    fn detect_industry(&self, text: &str, keywords: &[(String, f32)]) -> Option<String> {
        let industry_keywords = vec![
            ("科技", vec!["技术", "创新", "数字化", "AI", "人工智能", "云计算", "大数据", "软件"]),
            ("金融", vec!["投资", "理财", "股票", "基金", "银行", "证券", "金融", "资产"]),
            ("教育", vec!["教学", "学习", "课程", "培训", "教育", "学生", "老师", "知识"]),
            ("医疗", vec!["医院", "治疗", "健康", "医疗", "诊断", "患者", "医生", "临床"]),
            ("电商", vec!["购物", "商品", "订单", "物流", "电商", "促销", "营销", "零售"]),
            ("政府", vec!["政策", "政府", "公共服务", "行政", "管理", "法规", "政务"]),
        ];

        let text_lower = text.to_lowercase();
        
        for (industry, words) in industry_keywords {
            let match_count = words.iter()
                .filter(|word| text_lower.contains(&word.to_lowercase()))
                .count();
            
            if match_count >= 2 {
                return Some(industry.to_string());
            }
        }

        if !keywords.is_empty() {
            let top_keyword = &keywords[0].0;
            for (industry, _) in &[
                ("科技", vec!["技术", "创新"]),
                ("金融", vec!["投资", "理财"]),
                ("教育", vec!["教学", "学习"]),
                ("医疗", vec!["医院", "治疗"]),
                ("电商", vec!["购物", "商品"]),
            ] {
                if top_keyword.contains(industry) || industry.contains(top_keyword) {
                    return Some(industry.to_string());
                }
            }
        }

        None
    }

    fn text_to_document(&self, text: &str, title: &str) -> ParsedDocument {
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        
        let sections: Vec<crate::domain::model::m_document::Section> = paragraphs
            .into_iter()
            .enumerate()
            .map(|(index, para)| {
                let (level, title_text) = if para.starts_with("# ") {
                    (1, para.trim_start_matches("# ").to_string())
                } else if para.starts_with("## ") {
                    (2, para.trim_start_matches("## ").to_string())
                } else if para.starts_with("### ") {
                    (3, para.trim_start_matches("### ").to_string())
                } else {
                    (1, format!("第{}节", index + 1))
                };

                crate::domain::model::m_document::Section {
                    level,
                    title: title_text,
                    content: vec![crate::domain::model::m_document::ContentBlock::Text(
                        crate::domain::model::m_document::TextBlock {
                            text: para.to_string(),
                            style: None,
                        }
                    )],
                    keywords: Vec::new(),
                }
            })
            .collect();

        ParsedDocument {
            title: title.to_string(),
            language: "zh".to_string(),
            industry_hint: None,
            sections,
            images: Vec::new(),
            metadata: crate::domain::model::m_document::DocumentMetadata {
                author: None,
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                modified_at: None,
                page_count: None,
                word_count: Some(text.split_whitespace().count() as u32),
                file_size: None,
                custom: HashMap::new(),
            },
            style_hints: crate::domain::model::m_document::StyleHints {
                primary_color: None,
                secondary_color: None,
                font_family: None,
                heading_style: None,
                layout_style: None,
            },
        }
    }

    async fn export_ppt(
        &self,
        project: &PPTProject,
        pages: &[GeneratedPage],
        output_path: &Path,
        format: &OutputFormat,
    ) -> Result<PathBuf> {
        match format {
            OutputFormat::Pptx => {
                self.exporter.export_pptx(project, pages, output_path).await
            }
            OutputFormat::Pdf => {
                self.exporter.export_pdf(project, pages, output_path).await
            }
            OutputFormat::Images(img_format) => {
                let dir = output_path.parent().unwrap_or(Path::new("."));
                let paths = self.exporter.export_images(project, pages, dir, img_format.clone()).await?;
                paths.first().cloned().ok_or_else(|| Error::internal_error("导出图片失败"))
            }
            OutputFormat::Html => {
                self.exporter.export_html(project, pages, output_path).await
            }
            OutputFormat::Json => {
                self.exporter.export_json(project, pages, output_path).await
            }
        }
    }

    pub fn get_supported_industries(&self) -> Vec<String> {
        self.style_generator.get_supported_industries()
    }

    pub async fn preview_style(&self, industry: &str) -> Result<StyleParams> {
        self.style_generator.generate_from_industry(industry, 0.8).await
    }
}

impl Default for PPTGenerationService {
    fn default() -> Self {
        Self::new()
    }
}

trait ToText {
    fn to_text(&self) -> String;
}

impl ToText for ParsedDocument {
    fn to_text(&self) -> String {
        let mut text = self.title.clone();
        for section in &self.sections {
            text.push_str(&format!("\n{}", section.title));
            for content in &section.content {
                if let crate::domain::model::m_document::ContentBlock::Text(t) = content {
                    text.push_str(&format!("\n{}", t.text));
                }
            }
        }
        text
    }
}
