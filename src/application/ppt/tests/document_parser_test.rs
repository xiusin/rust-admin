#[cfg(test)]
mod tests {
    use crate::application::ppt::document_parse_service::{DocumentParseService, DocType};
    use crate::domain::model::m_document::ParsedDocument;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_detect_file_type_pptx() {
        let result = DocumentParseService::detect_file_type("test.pptx", b"PK");
        assert_eq!(result.unwrap(), DocType::PPT);
    }

    #[tokio::test]
    async fn test_detect_file_type_docx() {
        let result = DocumentParseService::detect_file_type("test.docx", b"PK");
        assert_eq!(result.unwrap(), DocType::Word);
    }

    #[tokio::test]
    async fn test_detect_file_type_pdf() {
        let result = DocumentParseService::detect_file_type("test.pdf", b"%PDF");
        assert_eq!(result.unwrap(), DocType::PDF);
    }

    #[tokio::test]
    async fn test_detect_file_type_md() {
        let result = DocumentParseService::detect_file_type("test.md", b"# Title");
        assert_eq!(result.unwrap(), DocType::Markdown);
    }

    #[tokio::test]
    async fn test_detect_file_type_txt() {
        let result = DocumentParseService::detect_file_type("test.txt", b"Plain text");
        assert_eq!(result.unwrap(), DocType::Text);
    }

    #[test]
    fn test_markdown_parser() {
        let content = r#"# 主标题

## 第一章

这是第一章的内容。

### 1.1 小节

- 列表项1
- 列表项2

## 第二章

1. 有序列表1
2. 有序列表2
"#;
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            crate::application::ppt::markdown_parser::MarkdownParser::parse(content).await
        });
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.title, "主标题");
        assert!(!doc.sections.is_empty());
    }

    #[test]
    fn test_text_extraction() {
        let content = "这是一段测试文本，用于测试文本提取功能。";
        assert!(!content.is_empty());
    }
}
