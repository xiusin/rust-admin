use crate::common::error::{Error, Result};
use crate::domain::model::m_document::*;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use std::collections::HashMap;

pub struct MarkdownParser;

impl MarkdownParser {
    pub async fn parse(content: &str) -> Result<ParsedDocument> {
        let parser = Parser::new(content);
        let events: Vec<Event> = parser.collect();

        let sections = Self::parse_sections(&events)?;
        let title = sections
            .first()
            .map(|s| s.title.clone())
            .unwrap_or_else(|| "未命名文档".to_string());

        let word_count = content.split_whitespace().count() as u32;

        Ok(ParsedDocument {
            title,
            language: "zh".to_string(),
            industry_hint: None,
            sections,
            images: Vec::new(),
            metadata: DocumentMetadata {
                author: None,
                created_at: None,
                modified_at: None,
                page_count: None,
                word_count: Some(word_count),
                file_size: None,
                custom: HashMap::new(),
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

    fn parse_sections(events: &[Event]) -> Result<Vec<Section>> {
        let mut sections = Vec::new();
        let mut current_section: Option<Section> = None;
        let mut i = 0;

        while i < events.len() {
            match &events[i] {
                Event::Start(Tag::Heading { level, .. }) => {
                    if let Some(section) = current_section.take() {
                        sections.push(section);
                    }

                    let mut title_text = String::new();
                    i += 1;
                    while i < events.len() {
                        match &events[i] {
                            Event::Text(text) => {
                                title_text.push_str(text);
                            }
                            Event::End(TagEnd::Heading(_)) => {
                                break;
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    current_section = Some(Section {
                        level: Self::heading_level_to_u8(*level),
                        title: title_text.trim().to_string(),
                        content: Vec::new(),
                        keywords: Vec::new(),
                    });
                }
                Event::Start(Tag::Paragraph) => {
                    let mut text = String::new();
                    i += 1;
                    while i < events.len() {
                        match &events[i] {
                            Event::Text(t) => text.push_str(t),
                            Event::End(TagEnd::Paragraph) => {
                                break;
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    if !text.trim().is_empty() {
                        let block = ContentBlock::Text(TextBlock {
                            text: text.trim().to_string(),
                            style: None,
                        });

                        if let Some(ref mut section) = current_section {
                            section.content.push(block);
                        } else {
                            current_section = Some(Section {
                                level: 1,
                                title: "文档内容".to_string(),
                                content: vec![block],
                                keywords: Vec::new(),
                            });
                        }
                    }
                }
                Event::Start(Tag::List(_)) => {
                    let list_block = Self::parse_list(events, &mut i)?;
                    if let Some(ref mut section) = current_section {
                        section.content.push(list_block);
                    } else {
                        current_section = Some(Section {
                            level: 1,
                            title: "文档内容".to_string(),
                            content: vec![list_block],
                            keywords: Vec::new(),
                        });
                    }
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    let code_block = Self::parse_code_block(events, &mut i)?;
                    if let Some(ref mut section) = current_section {
                        section.content.push(code_block);
                    } else {
                        current_section = Some(Section {
                            level: 1,
                            title: "文档内容".to_string(),
                            content: vec![code_block],
                            keywords: Vec::new(),
                        });
                    }
                }
                Event::Start(Tag::Image { dest_url, title, .. }) => {
                    let image_block = ContentBlock::Image(ImageBlock {
                        data: None,
                        url: Some(dest_url.to_string()),
                        alt_text: if title.is_empty() { None } else { Some(title.to_string()) },
                        width: None,
                        height: None,
                        format: None,
                    });

                    if let Some(ref mut section) = current_section {
                        section.content.push(image_block);
                    }
                    i += 1;
                }
                Event::Start(Tag::Table(_)) => {
                    let table_block = Self::parse_table(events, &mut i)?;
                    if let Some(ref mut section) = current_section {
                        section.content.push(table_block);
                    } else {
                        current_section = Some(Section {
                            level: 1,
                            title: "文档内容".to_string(),
                            content: vec![table_block],
                            keywords: Vec::new(),
                        });
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if let Some(section) = current_section {
            sections.push(section);
        }

        if sections.is_empty() {
            sections.push(Section {
                level: 1,
                title: "文档内容".to_string(),
                content: Vec::new(),
                keywords: Vec::new(),
            });
        }

        Ok(sections)
    }

    fn parse_list(events: &[Event], i: &mut usize) -> Result<ContentBlock> {
        let mut items = Vec::new();
        let mut current_text = String::new();
        let mut level: i32 = 0;
        let mut ordered = false;

        while *i < events.len() {
            match &events[*i] {
                Event::Start(Tag::List(list_type)) => {
                    ordered = list_type.is_some();
                    level += 1;
                }
                Event::End(TagEnd::List(_)) => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                Event::Start(Tag::Item) => {
                    if !current_text.trim().is_empty() {
                        items.push(ListItem {
                            text: current_text.trim().to_string(),
                            level: level.saturating_sub(1) as u8,
                        });
                        current_text.clear();
                    }
                }
                Event::End(TagEnd::Item) => {
                    if !current_text.trim().is_empty() {
                        items.push(ListItem {
                            text: current_text.trim().to_string(),
                            level: level.saturating_sub(1) as u8,
                        });
                        current_text.clear();
                    }
                }
                Event::Text(text) => {
                    current_text.push_str(text);
                }
                _ => {}
            }
            *i += 1;
        }

        Ok(ContentBlock::List(ListBlock { items, ordered }))
    }

    fn parse_code_block(events: &[Event], i: &mut usize) -> Result<ContentBlock> {
        let mut code = String::new();
        let mut language = None;

        if let Event::Start(Tag::CodeBlock(lang)) = &events[*i] {
            language = match lang {
                pulldown_cmark::CodeBlockKind::Fenced(lang_str) => {
                    if lang_str.is_empty() { None } else { Some(lang_str.to_string()) }
                }
                pulldown_cmark::CodeBlockKind::Indented => None,
            };
        }

        *i += 1;
        while *i < events.len() {
            match &events[*i] {
                Event::Text(text) => {
                    code.push_str(text);
                }
                Event::End(TagEnd::CodeBlock) => {
                    break;
                }
                _ => {}
            }
            *i += 1;
        }

        Ok(ContentBlock::Code(CodeBlock {
            code: code.trim().to_string(),
            language,
        }))
    }

    fn parse_table(events: &[Event], i: &mut usize) -> Result<ContentBlock> {
        let mut rows = Vec::new();
        let mut current_row = TableRow { cells: Vec::new() };
        let mut current_cell = String::new();
        let mut headers: Option<Vec<String>> = None;
        let mut is_header_row = false;

        while *i < events.len() {
            match &events[*i] {
                Event::Start(Tag::TableHead) => {
                    is_header_row = true;
                    current_row = TableRow { cells: Vec::new() };
                }
                Event::End(TagEnd::TableHead) => {
                    if is_header_row && !current_row.cells.is_empty() {
                        headers = Some(current_row.cells.clone());
                    }
                    is_header_row = false;
                }
                Event::Start(Tag::TableRow) => {
                    current_row = TableRow { cells: Vec::new() };
                }
                Event::End(TagEnd::TableRow) => {
                    if !current_row.cells.is_empty() && !is_header_row {
                        rows.push(current_row.clone());
                    }
                }
                Event::Start(Tag::TableCell) => {
                    current_cell.clear();
                }
                Event::End(TagEnd::TableCell) => {
                    current_row.cells.push(current_cell.trim().to_string());
                }
                Event::Text(text) => {
                    current_cell.push_str(text);
                }
                Event::End(TagEnd::Table) => {
                    break;
                }
                _ => {}
            }
            *i += 1;
        }

        Ok(ContentBlock::Table(TableBlock { rows, headers }))
    }

    fn heading_level_to_u8(level: HeadingLevel) -> u8 {
        match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }

    pub async fn parse_from_file(file_path: &std::path::Path) -> Result<ParsedDocument> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| Error::internal_error(format!("无法读取文件: {}", e)))?;
        Self::parse(&content).await
    }
}
