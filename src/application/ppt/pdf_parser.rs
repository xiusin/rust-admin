use crate::common::error::{Error, Result};
use crate::domain::model::m_document::*;
use lopdf::Document as PdfDocument;
use lopdf::ObjectId;
use std::collections::HashMap;
use std::path::Path;

pub struct PdfParser;

impl PdfParser {
    pub async fn parse_from_bytes(bytes: &[u8]) -> Result<ParsedDocument> {
        let pdf = PdfDocument::load_mem(bytes)
            .map_err(|e| Error::internal_error(format!("无法加载PDF数据: {}", e)))?;

        let text_blocks = Self::extract_text(&pdf)?;
        let images = Self::extract_images(&pdf)?;
        let metadata = Self::extract_metadata(&pdf)?;

        let title = text_blocks
            .first()
            .and_then(|tb| {
                if tb.text.len() < 100 {
                    Some(tb.text.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "未命名PDF文档".to_string());

        let sections = Self::convert_to_sections(&text_blocks);

        Ok(ParsedDocument {
            title,
            language: "zh".to_string(),
            industry_hint: None,
            sections,
            images,
            metadata,
            style_hints: StyleHints {
                primary_color: None,
                secondary_color: None,
                font_family: None,
                heading_style: None,
                layout_style: None,
            },
        })
    }

    pub async fn parse(file_path: &Path) -> Result<ParsedDocument> {
        let pdf = PdfDocument::load(file_path)
            .map_err(|e| Error::internal_error(format!("无法加载PDF文件: {}", e)))?;

        let text_blocks = Self::extract_text(&pdf)?;
        let images = Self::extract_images(&pdf)?;
        let metadata = Self::extract_metadata(&pdf)?;

        let title = text_blocks
            .first()
            .and_then(|tb| {
                if tb.text.len() < 100 {
                    Some(tb.text.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "未命名PDF文档".to_string());

        let sections = Self::convert_to_sections(&text_blocks);

        Ok(ParsedDocument {
            title,
            language: "zh".to_string(),
            industry_hint: None,
            sections,
            images,
            metadata,
            style_hints: StyleHints {
                primary_color: None,
                secondary_color: None,
                font_family: None,
                heading_style: None,
                layout_style: None,
            },
        })
    }

    fn extract_text(pdf: &PdfDocument) -> Result<Vec<TextBlock>> {
        let mut text_blocks = Vec::new();
        let pages = pdf.get_pages();

        for (page_num, object_id) in pages.iter() {
            let page_text = Self::extract_page_text(pdf, *object_id, *page_num)?;
            if !page_text.is_empty() {
                text_blocks.push(TextBlock {
                    text: page_text,
                    style: None,
                });
            }
        }

        Ok(text_blocks)
    }

    fn extract_page_text(pdf: &PdfDocument, object_id: ObjectId, _page_num: u32) -> Result<String> {
        let page = pdf.get_object(object_id)
            .map_err(|e| Error::internal_error(format!("无法读取页面: {}", e)))?;

        let page_dict = page.as_dict()
            .map_err(|e| Error::internal_error(format!("页面格式错误: {}", e)))?;

        let mut text_content = String::new();

        if let Ok(contents) = page_dict.get(b"Contents") {
            match contents {
                lopdf::Object::Array(content_array) => {
                    for content_obj in content_array.iter() {
                        if let Ok(content_id) = content_obj.as_reference() {
                            if let Ok(content) = pdf.get_object(content_id) {
                                if let Ok(stream) = content.as_stream() {
                                    if let Ok(decompressed) = stream.decompressed_content() {
                                        let content_text = Self::decode_pdf_content(&decompressed);
                                        text_content.push_str(&content_text);
                                        text_content.push(' ');
                                    }
                                }
                            }
                        }
                    }
                }
                lopdf::Object::Reference(content_ref) => {
                    if let Ok(content) = pdf.get_object(*content_ref) {
                        if let Ok(stream) = content.as_stream() {
                            if let Ok(decompressed) = stream.decompressed_content() {
                                let content_text = Self::decode_pdf_content(&decompressed);
                                text_content.push_str(&content_text);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(text_content.trim().to_string())
    }

    fn decode_pdf_content(content: &[u8]) -> String {
        let mut text = String::new();
        let mut in_text = false;
        let mut current_text = String::new();
        let mut escape = false;

        for &byte in content {
            if escape {
                current_text.push(byte as char);
                escape = false;
                continue;
            }

            match byte {
                b'\\' => {
                    escape = true;
                }
                b'(' => {
                    in_text = true;
                    current_text.clear();
                }
                b')' => {
                    if in_text {
                        text.push_str(&current_text);
                        text.push(' ');
                        in_text = false;
                    }
                }
                _ => {
                    if in_text {
                        if byte >= 32 && byte < 127 {
                            current_text.push(byte as char);
                        } else if byte >= 0xA0 && byte < 0xFF {
                            current_text.push(byte as char);
                        }
                    }
                }
            }
        }

        text
    }

    fn extract_images(pdf: &PdfDocument) -> Result<Vec<ExtractedImage>> {
        let mut images = Vec::new();
        let mut image_index = 0;

        for (object_id, object) in pdf.objects.iter() {
            if let Ok(stream) = object.as_stream() {
                let dict = &stream.dict;
                {
                    let subtype = dict.get(b"Subtype").ok().and_then(|s: &lopdf::Object| s.as_name_str().ok());
                    
                    if subtype == Some("Image") {
                        let format_str = dict.get(b"Filter")
                            .ok()
                            .and_then(|f: &lopdf::Object| f.as_name_str().ok())
                            .unwrap_or("Unknown");

                        let content = stream.content.clone();
                        
                        let data = base64::Engine::encode(
                            &base64::engine::general_purpose::STANDARD,
                            &content
                        );

                        let width = dict.get(b"Width")
                            .ok()
                            .and_then(|w: &lopdf::Object| w.as_i64().ok())
                            .map(|w| w as u32);
                        
                        let height = dict.get(b"Height")
                            .ok()
                            .and_then(|h: &lopdf::Object| h.as_i64().ok())
                            .map(|h| h as u32);

                        images.push(ExtractedImage {
                            data,
                            format: format_str.to_string(),
                            width,
                            height,
                            original_name: Some(format!("image_{}", object_id.0)),
                            index: image_index,
                        });
                        image_index += 1;
                    }
                }
            }
        }


        Ok(images)
    }

    fn extract_metadata(pdf: &PdfDocument) -> Result<DocumentMetadata> {
        let mut metadata = DocumentMetadata {
            author: None,
            created_at: None,
            modified_at: None,
            page_count: Some(pdf.get_pages().len() as u32),
            word_count: None,
            file_size: None,
            custom: HashMap::new(),
        };

        let trailer = &pdf.trailer;
        if let Ok(info_ref) = trailer.get(b"Info") {
            if let lopdf::Object::Reference(info_id) = info_ref {
                if let Ok(info_obj) = pdf.get_object(*info_id) {
                    if let Ok(info_dict) = info_obj.as_dict() {
                        if let Ok(author) = info_dict.get(b"Author") {
                            if let Ok(author_str) = author.as_str() {
                                metadata.author = Some(String::from_utf8_lossy(author_str).to_string());
                            }
                        }
                        if let Ok(created) = info_dict.get(b"CreationDate") {
                            if let Ok(created_str) = created.as_str() {
                                metadata.created_at = Some(String::from_utf8_lossy(created_str).to_string());
                            }
                        }
                        if let Ok(modified) = info_dict.get(b"ModDate") {
                            if let Ok(modified_str) = modified.as_str() {
                                metadata.modified_at = Some(String::from_utf8_lossy(modified_str).to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(metadata)
    }

    fn convert_to_sections(text_blocks: &[TextBlock]) -> Vec<Section> {
        let mut sections = Vec::new();
        let mut current_section = Section {
            level: 1,
            title: "文档内容".to_string(),
            content: Vec::new(),
            keywords: Vec::new(),
        };

        for tb in text_blocks {
            if tb.text.len() < 100 && !tb.text.is_empty() {
                if !current_section.content.is_empty() {
                    sections.push(current_section);
                    current_section = Section {
                        level: 1,
                        title: tb.text.clone(),
                        content: Vec::new(),
                        keywords: Vec::new(),
                    };
                } else {
                    current_section.title = tb.text.clone();
                }
            } else {
                current_section.content.push(ContentBlock::Text(TextBlock {
                    text: tb.text.clone(),
                    style: None,
                }));
            }
        }

        if !current_section.content.is_empty() {
            sections.push(current_section);
        }

        sections
    }
}
