use crate::common::error::{Error, Result};
use crate::domain::model::m_document::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use zip::ZipArchive;

pub struct PptxParser;

impl PptxParser {
    pub async fn parse(file_path: &Path) -> Result<ParsedDocument> {
        let file = File::open(file_path).map_err(|e| Error::internal_error(format!("无法打开文件: {}", e)))?;
        let mut archive = ZipArchive::new(file).map_err(|e| Error::internal_error(format!("无效的PPTX文件: {}", e)))?;

        let slides = Self::extract_slides(&mut archive)?;
        let images = Self::extract_images(&mut archive)?;
        let metadata = Self::extract_metadata(&mut archive)?;

        let title = slides
            .first()
            .and_then(|s| s.title.clone())
            .unwrap_or_else(|| "未命名演示文稿".to_string());

        let sections = Self::convert_slides_to_sections(&slides);

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

    fn extract_slides(archive: &mut ZipArchive<File>) -> Result<Vec<Slide>> {
        let mut slides = Vec::new();
        let mut slide_indices: Vec<(u32, String)> = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                if let Some(num_str) = name
                    .strip_prefix("ppt/slides/slide")
                    .and_then(|s| s.strip_suffix(".xml"))
                {
                    if let Ok(num) = num_str.parse::<u32>() {
                        slide_indices.push((num, name));
                    }
                }
            }
        }

        slide_indices.sort_by_key(|(num, _)| *num);

        for (slide_num, slide_path) in slide_indices {
            let mut file = archive
                .by_name(&slide_path)
                .map_err(|e| Error::internal_error(format!("读取幻灯片失败: {}", e)))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| Error::internal_error(format!("读取幻灯片内容失败: {}", e)))?;

            let slide = Self::parse_slide_xml(&content, slide_num)?;
            slides.push(slide);
        }

        Ok(slides)
    }

    fn parse_slide_xml(xml_content: &str, slide_number: u32) -> Result<Slide> {
        let reader = Cursor::new(xml_content.as_bytes());
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut content_blocks: Vec<ContentBlock> = Vec::new();
        let mut current_text = String::new();
        let mut in_text = false;
        let mut title: Option<String> = None;
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "t" || name_str == "a:t" {
                        in_text = true;
                    }
                }
                Ok(Event::End(ref e)) => {
                    let local_name = e.local_name();
                    let name_str = String::from_utf8_lossy(local_name.as_ref());
                    if name_str == "t" || name_str == "a:t" {
                        in_text = false;
                        if !current_text.trim().is_empty() {
                            let text = current_text.trim().to_string();
                            if title.is_none() && text.len() < 100 {
                                title = Some(text.clone());
                            }
                            content_blocks.push(ContentBlock::Text(TextBlock {
                                text,
                                style: None,
                            }));
                        }
                        current_text.clear();
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if in_text {
                        current_text.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(Error::internal_error(format!("XML解析错误: {:?}", e)));
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(Slide {
            slide_number,
            title,
            content: content_blocks,
            notes: None,
            layout: None,
        })
    }

    fn extract_images(archive: &mut ZipArchive<File>) -> Result<Vec<ExtractedImage>> {
        let mut images = Vec::new();
        let mut image_index = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();

            if name.starts_with("ppt/media/") {
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
                let mut xml_reader = Reader::from_reader(reader);
                xml_reader.config_mut().trim_text(true);

                let mut current_element = String::new();
                let mut buf = Vec::new();

                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            let local_name = e.local_name();
                            current_element = String::from_utf8_lossy(local_name.as_ref()).to_string();
                        }
                        Ok(Event::Text(ref e)) => {
                            let text = e.unescape().unwrap_or_default();
                            match current_element.as_str() {
                                "dc:creator" => metadata.author = Some(text.to_string()),
                                "dcterms:created" => metadata.created_at = Some(text.to_string()),
                                "dcterms:modified" => metadata.modified_at = Some(text.to_string()),
                                _ => {}
                            }
                        }
                        Ok(Event::End(_)) => {
                            current_element.clear();
                        }
                        Ok(Event::Eof) => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(metadata)
    }

    fn convert_slides_to_sections(slides: &[Slide]) -> Vec<Section> {
        slides
            .iter()
            .map(|slide| {
                let title = slide.title.clone().unwrap_or_else(|| format!("幻灯片 {}", slide.slide_number));
                Section {
                    level: 1,
                    title,
                    content: slide.content.clone(),
                    keywords: Vec::new(),
                }
            })
            .collect()
    }

    pub async fn parse_from_bytes(bytes: &[u8]) -> Result<ParsedDocument> {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| Error::internal_error(format!("无效的PPTX数据: {}", e)))?;

        let slides = Self::extract_slides_from_archive(&mut archive)?;
        let images = Self::extract_images_from_archive(&mut archive)?;
        let metadata = Self::extract_metadata_from_archive(&mut archive)?;

        let title = slides
            .first()
            .and_then(|s| s.title.clone())
            .unwrap_or_else(|| "未命名演示文稿".to_string());

        let sections = Self::convert_slides_to_sections(&slides);

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

    fn extract_slides_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<Slide>> {
        let mut slides = Vec::new();
        let mut slide_indices: Vec<(u32, String)> = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                if let Some(num_str) = name
                    .strip_prefix("ppt/slides/slide")
                    .and_then(|s| s.strip_suffix(".xml"))
                {
                    if let Ok(num) = num_str.parse::<u32>() {
                        slide_indices.push((num, name));
                    }
                }
            }
        }

        slide_indices.sort_by_key(|(num, _)| *num);

        for (slide_num, slide_path) in slide_indices {
            let mut file = archive
                .by_name(&slide_path)
                .map_err(|e| Error::internal_error(format!("读取幻灯片失败: {}", e)))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| Error::internal_error(format!("读取幻灯片内容失败: {}", e)))?;

            let slide = Self::parse_slide_xml(&content, slide_num)?;
            slides.push(slide);
        }

        Ok(slides)
    }

    fn extract_images_from_archive<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<ExtractedImage>> {
        let mut images = Vec::new();
        let mut image_index = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| Error::internal_error(format!("读取压缩包失败: {}", e)))?;
            let name = file.name().to_string();

            if name.starts_with("ppt/media/") {
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
                let mut xml_reader = Reader::from_reader(reader);
                xml_reader.config_mut().trim_text(true);

                let mut current_element = String::new();
                let mut buf = Vec::new();

                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            let local_name = e.local_name();
                            current_element = String::from_utf8_lossy(local_name.as_ref()).to_string();
                        }
                        Ok(Event::Text(ref e)) => {
                            let text = e.unescape().unwrap_or_default();
                            match current_element.as_str() {
                                "dc:creator" => metadata.author = Some(text.to_string()),
                                "dcterms:created" => metadata.created_at = Some(text.to_string()),
                                "dcterms:modified" => metadata.modified_at = Some(text.to_string()),
                                _ => {}
                            }
                        }
                        Ok(Event::End(_)) => {
                            current_element.clear();
                        }
                        Ok(Event::Eof) => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(metadata)
    }
}
