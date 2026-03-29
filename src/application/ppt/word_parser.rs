use crate::common::error::{Error, Result};
use crate::domain::model::m_document::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use zip::ZipArchive;

pub struct DocxParser;

impl DocxParser {
    pub async fn parse(file_path: &Path) -> Result<ParsedDocument> {
        let file = File::open(file_path).map_err(|e| Error::internal_error(format!("无法打开文件: {}", e)))?;
        let mut archive = ZipArchive::new(file).map_err(|e| Error::internal_error(format!("无效的DOCX文件: {}", e)))?;

        let paragraphs = Self::extract_paragraphs(&mut archive)?;
        let tables = Self::extract_tables(&mut archive)?;
        let images = Self::extract_images(&mut archive)?;
        let metadata = Self::extract_metadata(&mut archive)?;

        let title = paragraphs
            .first()
            .and_then(|p| {
                if let ContentBlock::Text(tb) = p {
                    if tb.text.len() < 100 {
                        Some(tb.text.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "未命名文档".to_string());

        let sections = Self::convert_to_sections(&paragraphs, &tables);

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

    fn extract_paragraphs(archive: &mut ZipArchive<File>) -> Result<Vec<ContentBlock>> {
        let mut paragraphs = Vec::new();

        let mut document_file = archive
            .by_name("word/document.xml")
            .map_err(|e| Error::internal_error(format!("无法读取document.xml: {}", e)))?;

        let mut content = String::new();
        document_file
            .read_to_string(&mut content)
            .map_err(|e| Error::internal_error(format!("读取文档内容失败: {}", e)))?;

        let reader = Cursor::new(content.as_bytes());
        let mut xml_reader = quick_xml::Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(false);

        let mut current_text = String::new();
        let mut in_paragraph = false;
        let mut in_text = false;
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "p" || name_str == "w:p" {
                        in_paragraph = true;
                        current_text.clear();
                    } else if name_str == "t" || name_str == "w:t" {
                        in_text = true;
                    }
                }
                Ok(quick_xml::events::Event::End(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "p" || name_str == "w:p" {
                        in_paragraph = false;
                        if !current_text.trim().is_empty() {
                            paragraphs.push(ContentBlock::Text(TextBlock {
                                text: current_text.trim().to_string(),
                                style: None,
                            }));
                        }
                        current_text.clear();
                    } else if name_str == "t" || name_str == "w:t" {
                        in_text = false;
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) => {
                    if in_text && in_paragraph {
                        current_text.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => {
                    return Err(Error::internal_error(format!("XML解析错误: {:?}", e)));
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(paragraphs)
    }

    fn extract_tables(archive: &mut ZipArchive<File>) -> Result<Vec<TableBlock>> {
        let mut tables = Vec::new();

        let mut document_file = archive
            .by_name("word/document.xml")
            .map_err(|e| Error::internal_error(format!("无法读取document.xml: {}", e)))?;

        let mut content = String::new();
        document_file
            .read_to_string(&mut content)
            .map_err(|e| Error::internal_error(format!("读取文档内容失败: {}", e)))?;

        let reader = Cursor::new(content.as_bytes());
        let mut xml_reader = quick_xml::Reader::from_reader(reader);

        let mut _in_table = false;
        let mut _in_row = false;
        let mut in_cell = false;
        let mut current_table = TableBlock {
            rows: Vec::new(),
            headers: None,
        };
        let mut current_row = TableRow { cells: Vec::new() };
        let mut current_cell = String::new();
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    match name_str.as_ref() {
                        "tbl" | "w:tbl" => {
                            _in_table = true;
                            current_table = TableBlock {
                                rows: Vec::new(),
                                headers: None,
                            };
                        }
                        "tr" | "w:tr" => {
                            _in_row = true;
                            current_row = TableRow { cells: Vec::new() };
                        }
                        "tc" | "w:tc" => {
                            in_cell = true;
                            current_cell.clear();
                        }
                        "t" | "w:t" if in_cell => {}
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::End(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    match name_str.as_ref() {
                        "tbl" | "w:tbl" => {
                            _in_table = false;
                            if !current_table.rows.is_empty() {
                                tables.push(current_table.clone());
                            }
                        }
                        "tr" | "w:tr" => {
                            _in_row = false;
                            if !current_row.cells.is_empty() {
                                current_table.rows.push(current_row.clone());
                            }
                        }
                        "tc" | "w:tc" => {
                            in_cell = false;
                            current_row.cells.push(current_cell.trim().to_string());
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) => {
                    if in_cell {
                        current_cell.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(tables)
    }

    fn extract_images(archive: &mut ZipArchive<File>) -> Result<Vec<ExtractedImage>> {
        let mut images = Vec::new();
        let mut image_index = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();

            if name.starts_with("word/media/") {
                let format = name
                    .rsplit('.')
                    .next()
                    .unwrap_or("unknown")
                    .to_lowercase();

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| Error::internal_error(format!("读取图片失败: {}", e)))?;

                let data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buffer);

                images.push(ExtractedImage {
                    data,
                    format,
                    width: None,
                    height: None,
                    original_name: Some(name.clone()),
                    index: image_index,
                });
                image_index += 1;
            }
        }

        Ok(images)
    }

    fn extract_metadata(archive: &mut ZipArchive<File>) -> Result<DocumentMetadata> {
        let mut metadata = DocumentMetadata {
            author: None,
            created_at: None,
            modified_at: None,
            page_count: None,
            word_count: None,
            file_size: None,
            custom: HashMap::new(),
        };

        if let Ok(mut file) = archive.by_name("docProps/core.xml") {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                let reader = Cursor::new(content.as_bytes());
                let mut xml_reader = quick_xml::Reader::from_reader(reader);

                let mut current_element = String::new();
                let mut buf = Vec::new();

                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(quick_xml::events::Event::Start(ref e)) => {
                            let local_name = e.local_name();
                            current_element = String::from_utf8_lossy(local_name.as_ref()).to_string();
                        }
                        Ok(quick_xml::events::Event::Text(ref e)) => {
                            let text = e.unescape().unwrap_or_default();
                            match current_element.as_str() {
                                "dc:creator" => metadata.author = Some(text.to_string()),
                                "dcterms:created" => metadata.created_at = Some(text.to_string()),
                                "dcterms:modified" => metadata.modified_at = Some(text.to_string()),
                                _ => {}
                            }
                        }
                        Ok(quick_xml::events::Event::End(_)) => {
                            current_element.clear();
                        }
                        Ok(quick_xml::events::Event::Eof) => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(metadata)
    }

    fn convert_to_sections(paragraphs: &[ContentBlock], tables: &[TableBlock]) -> Vec<Section> {
        let mut sections = Vec::new();
        let mut current_section = Section {
            level: 1,
            title: "正文".to_string(),
            content: Vec::new(),
            keywords: Vec::new(),
        };

        for block in paragraphs {
            if let ContentBlock::Text(tb) = block {
                if tb.text.len() < 100 && !tb.text.is_empty() {
                    if !current_section.content.is_empty() {
                        sections.push(current_section.clone());
                    }
                    current_section = Section {
                        level: 1,
                        title: tb.text.clone(),
                        content: Vec::new(),
                        keywords: Vec::new(),
                    };
                } else {
                    current_section.content.push(block.clone());
                }
            }
        }

        for table in tables {
            current_section.content.push(ContentBlock::Table(table.clone()));
        }

        if !current_section.content.is_empty() {
            sections.push(current_section);
        }

        if sections.is_empty() {
            sections.push(Section {
                level: 1,
                title: "文档内容".to_string(),
                content: paragraphs.to_vec(),
                keywords: Vec::new(),
            });
        }

        sections
    }

    pub async fn parse_from_bytes(bytes: &[u8]) -> Result<ParsedDocument> {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| Error::internal_error(format!("无效的DOCX数据: {}", e)))?;

        let paragraphs = Self::extract_paragraphs_from_archive(&mut archive)?;
        let tables = Self::extract_tables_from_archive(&mut archive)?;
        let images = Self::extract_images_from_archive(&mut archive)?;
        let metadata = Self::extract_metadata_from_archive(&mut archive)?;

        let title = paragraphs
            .first()
            .and_then(|p| {
                if let ContentBlock::Text(tb) = p {
                    if tb.text.len() < 100 {
                        Some(tb.text.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "未命名文档".to_string());

        let sections = Self::convert_to_sections(&paragraphs, &tables);

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

    fn extract_paragraphs_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<ContentBlock>> {
        let mut paragraphs = Vec::new();

        let mut document_file = archive
            .by_name("word/document.xml")
            .map_err(|e| Error::internal_error(format!("无法读取document.xml: {}", e)))?;

        let mut content = String::new();
        document_file
            .read_to_string(&mut content)
            .map_err(|e| Error::internal_error(format!("读取文档内容失败: {}", e)))?;

        let reader = Cursor::new(content.as_bytes());
        let mut xml_reader = quick_xml::Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(false);

        let mut current_text = String::new();
        let mut in_paragraph = false;
        let mut in_text = false;
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "p" || name_str == "w:p" {
                        in_paragraph = true;
                        current_text.clear();
                    } else if name_str == "t" || name_str == "w:t" {
                        in_text = true;
                    }
                }
                Ok(quick_xml::events::Event::End(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "p" || name_str == "w:p" {
                        in_paragraph = false;
                        if !current_text.trim().is_empty() {
                            paragraphs.push(ContentBlock::Text(TextBlock {
                                text: current_text.trim().to_string(),
                                style: None,
                            }));
                        }
                        current_text.clear();
                    } else if name_str == "t" || name_str == "w:t" {
                        in_text = false;
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) => {
                    if in_text && in_paragraph {
                        current_text.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(paragraphs)
    }

    fn extract_tables_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<TableBlock>> {
        let mut tables = Vec::new();

        let mut document_file = archive
            .by_name("word/document.xml")
            .map_err(|e| Error::internal_error(format!("无法读取document.xml: {}", e)))?;

        let mut content = String::new();
        document_file
            .read_to_string(&mut content)
            .map_err(|e| Error::internal_error(format!("读取文档内容失败: {}", e)))?;

        let reader = Cursor::new(content.as_bytes());
        let mut xml_reader = quick_xml::Reader::from_reader(reader);

        let mut _in_table = false;
        let mut _in_row = false;
        let mut in_cell = false;
        let mut current_table = TableBlock {
            rows: Vec::new(),
            headers: None,
        };
        let mut current_row = TableRow { cells: Vec::new() };
        let mut current_cell = String::new();
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    match name_str.as_ref() {
                        "tbl" | "w:tbl" => {
                            _in_table = true;
                            current_table = TableBlock {
                                rows: Vec::new(),
                                headers: None,
                            };
                        }
                        "tr" | "w:tr" => {
                            _in_row = true;
                            current_row = TableRow { cells: Vec::new() };
                        }
                        "tc" | "w:tc" => {
                            in_cell = true;
                            current_cell.clear();
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::End(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    match name_str.as_ref() {
                        "tbl" | "w:tbl" => {
                            _in_table = false;
                            if !current_table.rows.is_empty() {
                                tables.push(current_table.clone());
                            }
                        }
                        "tr" | "w:tr" => {
                            _in_row = false;
                            if !current_row.cells.is_empty() {
                                current_table.rows.push(current_row.clone());
                            }
                        }
                        "tc" | "w:tc" => {
                            in_cell = false;
                            current_row.cells.push(current_cell.trim().to_string());
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) => {
                    if in_cell {
                        current_cell.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(tables)
    }

    fn extract_images_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<ExtractedImage>> {
        let mut images = Vec::new();
        let mut image_index = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();

            if name.starts_with("word/media/") {
                let format = name
                    .rsplit('.')
                    .next()
                    .unwrap_or("unknown")
                    .to_lowercase();

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| Error::internal_error(format!("读取图片失败: {}", e)))?;

                let data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buffer);

                images.push(ExtractedImage {
                    data,
                    format,
                    width: None,
                    height: None,
                    original_name: Some(name.clone()),
                    index: image_index,
                });
                image_index += 1;
            }
        }

        Ok(images)
    }

    fn extract_metadata_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<DocumentMetadata> {
        let mut metadata = DocumentMetadata {
            author: None,
            created_at: None,
            modified_at: None,
            page_count: None,
            word_count: None,
            file_size: None,
            custom: HashMap::new(),
        };

        if let Ok(mut file) = archive.by_name("docProps/core.xml") {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                let reader = Cursor::new(content.as_bytes());
                let mut xml_reader = quick_xml::Reader::from_reader(reader);

                let mut current_element = String::new();
                let mut buf = Vec::new();

                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(quick_xml::events::Event::Start(ref e)) => {
                            let local_name = e.local_name();
                            current_element = String::from_utf8_lossy(local_name.as_ref()).to_string();
                        }
                        Ok(quick_xml::events::Event::Text(ref e)) => {
                            let text = e.unescape().unwrap_or_default();
                            match current_element.as_str() {
                                "dc:creator" => metadata.author = Some(text.to_string()),
                                "dcterms:created" => metadata.created_at = Some(text.to_string()),
                                "dcterms:modified" => metadata.modified_at = Some(text.to_string()),
                                _ => {}
                            }
                        }
                        Ok(quick_xml::events::Event::End(_)) => {
                            current_element.clear();
                        }
                        Ok(quick_xml::events::Event::Eof) => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(metadata)
    }
}
