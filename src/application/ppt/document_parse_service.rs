use crate::common::error::{Error, Result};
use crate::domain::model::m_document::*;
use std::path::Path;

use super::{PptxParser, DocxParser, PdfParser, MarkdownParser};

#[derive(Debug, Clone, PartialEq)]
pub enum DocType {
    Pptx,
    Docx,
    Pdf,
    Markdown,
    Txt,
    Unknown,
}

impl DocType {
    pub fn as_str(&self) -> &str {
        match self {
            DocType::Pptx => "pptx",
            DocType::Docx => "docx",
            DocType::Pdf => "pdf",
            DocType::Markdown => "md",
            DocType::Txt => "txt",
            DocType::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pptx" => DocType::Pptx,
            "docx" => DocType::Docx,
            "pdf" => DocType::Pdf,
            "md" | "markdown" => DocType::Markdown,
            "txt" => DocType::Txt,
            _ => DocType::Unknown,
        }
    }
}

pub struct DocumentParseService;

impl DocumentParseService {
    pub async fn parse_document(
        file_path: &Path,
        file_type: DocType,
    ) -> Result<ParsedDocument> {
        match file_type {
            DocType::Pptx => PptxParser::parse(file_path).await,
            DocType::Docx => DocxParser::parse(file_path).await,
            DocType::Pdf => PdfParser::parse(file_path).await,
            DocType::Markdown => MarkdownParser::parse_from_file(file_path).await,
            DocType::Txt => Self::parse_txt_file(file_path).await,
            DocType::Unknown => Err(Error::bad_request("不支持的文件类型")),
        }
    }

    pub async fn parse_from_bytes(
        bytes: &[u8],
        file_type: DocType,
        original_name: &str,
    ) -> Result<ParsedDocument> {
        match file_type {
            DocType::Pptx => PptxParser::parse_from_bytes(bytes).await,
            DocType::Docx => DocxParser::parse_from_bytes(bytes).await,
            DocType::Pdf => PdfParser::parse_from_bytes(bytes).await,
            DocType::Markdown | DocType::Txt => {
                let content = String::from_utf8(bytes.to_vec())
                    .map_err(|e| Error::bad_request(format!("文件编码错误: {}", e)))?;
                MarkdownParser::parse(&content).await
            }
            DocType::Unknown => {
                let detected_type = Self::detect_file_type(original_name, bytes)?;
                if detected_type == DocType::Unknown {
                    return Err(Error::bad_request("无法识别文件类型"));
                }
                Box::pin(Self::parse_from_bytes(bytes, detected_type, original_name)).await
            }
        }
    }

    pub fn detect_file_type(file_name: &str, _content: &[u8]) -> Result<DocType> {
        let extension = file_name
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        let doc_type = match extension.as_str() {
            "pptx" => DocType::Pptx,
            "docx" => DocType::Docx,
            "pdf" => DocType::Pdf,
            "md" | "markdown" => DocType::Markdown,
            "txt" => DocType::Txt,
            _ => DocType::Unknown,
        };

        Ok(doc_type)
    }

    async fn parse_txt_file(file_path: &Path) -> Result<ParsedDocument> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| Error::internal_error(format!("无法读取文件: {}", e)))?;

        let title = content
            .lines()
            .next()
            .map(|s| {
                if s.len() > 100 {
                    s.chars().take(100).collect()
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "未命名文档".to_string());

        let word_count = content.split_whitespace().count() as u32;

        let text_block = TextBlock {
            text: content,
            style: None,
        };

        let section = Section {
            level: 1,
            title: title.clone(),
            content: vec![ContentBlock::Text(text_block)],
            keywords: Vec::new(),
        };

        Ok(ParsedDocument {
            title,
            language: "zh".to_string(),
            industry_hint: None,
            sections: vec![section],
            images: Vec::new(),
            metadata: DocumentMetadata {
                author: None,
                created_at: None,
                modified_at: None,
                page_count: None,
                word_count: Some(word_count),
                file_size: None,
                custom: std::collections::HashMap::new(),
            },
            style_hints: StyleHints {
                primary_color: None,
                secondary_color: None,
                font_family: None,
                heading_style: None,
                layout_style: None,
            },
        })
    }

    pub fn get_supported_types() -> Vec<String> {
        vec![
            "pptx".to_string(),
            "docx".to_string(),
            "pdf".to_string(),
            "md".to_string(),
            "txt".to_string(),
        ]
    }

    pub fn get_mime_type(file_type: &DocType) -> &'static str {
        match file_type {
            DocType::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            DocType::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            DocType::Pdf => "application/pdf",
            DocType::Markdown => "text/markdown",
            DocType::Txt => "text/plain",
            DocType::Unknown => "application/octet-stream",
        }
    }
}
