use crate::common::error::Result;
use super::page_generator::{GeneratedPage, PageElement, ElementType, BackgroundType};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPTProject {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpg,
    WebP,
}

impl ImageFormat {
    pub fn as_str(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpg => "jpg",
            ImageFormat::WebP => "webp",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Png => ".png",
            ImageFormat::Jpg => ".jpg",
            ImageFormat::WebP => ".webp",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpg => "image/jpeg",
            ImageFormat::WebP => "image/webp",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Pptx,
    Pdf,
    Images(ImageFormat),
    Html,
    Json,
}

pub struct PptExporter {
    output_dir: PathBuf,
}

impl PptExporter {
    pub fn new(output_dir: impl AsRef<Path>) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    pub fn default_output_dir() -> Self {
        Self {
            output_dir: PathBuf::from("uploads/ppt_output"),
        }
    }

    pub async fn export_pptx(
        &self,
        project: &PPTProject,
        pages: &[GeneratedPage],
        output_path: &Path,
    ) -> Result<PathBuf> {
        let full_path = self.output_dir.join(output_path);
        
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let _pptx_content = self.generate_pptx_content(project, pages)?;
        
        let file = std::fs::File::create(&full_path)?;
        let mut zip = zip::ZipWriter::new(file);
        
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(self.get_content_types_xml().as_bytes())?;

        zip.start_file("_rels/.rels", options)?;
        zip.write_all(self.get_rels_xml().as_bytes())?;

        zip.start_file("docProps/app.xml", options)?;
        zip.write_all(self.get_app_xml(project, pages.len()).as_bytes())?;

        zip.start_file("docProps/core.xml", options)?;
        zip.write_all(self.get_core_xml(project).as_bytes())?;

        zip.start_file("ppt/presentation.xml", options)?;
        zip.write_all(self.get_presentation_xml(pages.len()).as_bytes())?;

        zip.start_file("ppt/_rels/presentation.xml.rels", options)?;
        zip.write_all(self.get_presentation_rels_xml(pages.len()).as_bytes())?;

        for (index, page) in pages.iter().enumerate() {
            let slide_num = index + 1;
            let slide_path = format!("ppt/slides/slide{}.xml", slide_num);
            zip.start_file(&slide_path, options)?;
            zip.write_all(self.generate_slide_xml(page)?.as_bytes())?;
        }

        zip.start_file("ppt/slideLayouts/slideLayout1.xml", options)?;
        zip.write_all(self.get_slide_layout_xml().as_bytes())?;

        zip.start_file("ppt/slideMasters/slideMaster1.xml", options)?;
        zip.write_all(self.get_slide_master_xml().as_bytes())?;

        zip.start_file("ppt/theme/theme1.xml", options)?;
        zip.write_all(self.get_theme_xml().as_bytes())?;

        zip.finish()?;

        Ok(full_path)
    }

    pub async fn export_pdf(
        &self,
        project: &PPTProject,
        pages: &[GeneratedPage],
        output_path: &Path,
    ) -> Result<PathBuf> {
        let full_path = self.output_dir.join(output_path);
        
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let html_content = self.generate_pdf_html(project, pages)?;
        let html_path = full_path.with_extension("html");
        tokio::fs::write(&html_path, html_content).await?;

        Ok(html_path)
    }

    pub async fn export_images(
        &self,
        _project: &PPTProject,
        pages: &[GeneratedPage],
        output_dir: &Path,
        format: ImageFormat,
    ) -> Result<Vec<PathBuf>> {
        let full_dir = self.output_dir.join(output_dir);
        tokio::fs::create_dir_all(&full_dir).await?;

        let mut output_paths = Vec::new();

        for (index, page) in pages.iter().enumerate() {
            let image_data = self.render_page_to_image(page, &format)?;
            let filename = format!("slide_{}{}", index + 1, format.extension());
            let file_path = full_dir.join(&filename);
            
            tokio::fs::write(&file_path, &image_data).await?;
            output_paths.push(file_path);
        }

        Ok(output_paths)
    }

    pub async fn export_html(
        &self,
        project: &PPTProject,
        pages: &[GeneratedPage],
        output_path: &Path,
    ) -> Result<PathBuf> {
        let full_path = self.output_dir.join(output_path);
        
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let html_content = self.generate_html_presentation(project, pages)?;
        tokio::fs::write(&full_path, html_content).await?;

        Ok(full_path)
    }

    pub async fn export_json(
        &self,
        project: &PPTProject,
        pages: &[GeneratedPage],
        output_path: &Path,
    ) -> Result<PathBuf> {
        let full_path = self.output_dir.join(output_path);
        
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let json_data = serde_json::to_string_pretty(&serde_json::json!({
            "project": project,
            "pages": pages,
        }))?;
        
        tokio::fs::write(&full_path, json_data).await?;

        Ok(full_path)
    }

    fn generate_pptx_content(&self, _project: &PPTProject, pages: &[GeneratedPage]) -> Result<String> {
        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<presentation xmlns="http://schemas.openxmlformats.org/presentationml/2006/main">
    <sldMasterIdLst/>
    <sldIdLst>
        {}
    </sldIdLst>
</presentation>"#,
            pages.iter().enumerate().map(|(i, _)| {
                format!(r#"<sldId id="{}" r:id="rId{}"/>"#, i + 256, i + 2)
            }).collect::<Vec<_>>().join("\n        ")
        ))
    }

    fn get_content_types_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    <Default Extension="xml" ContentType="application/xml"/>
    <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
    <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
    <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#.to_string()
    }

    fn get_rels_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
    <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#.to_string()
    }

    fn get_app_xml(&self, project: &PPTProject, page_count: usize) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">
    <Application>AI PPT Generator</Application>
    <Slides>{}</Slides>
    <Title>{}</Title>
</Properties>"#,
            page_count,
            project.title
        )
    }

    fn get_core_xml(&self, project: &PPTProject) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>{}</dc:title>
    <dc:description>{}</dc:description>
    <dcterms:created xmlns:dcterms="http://purl.org/dc/terms/">{}</dcterms:created>
</cp:coreProperties>"#,
            project.title,
            project.description.as_deref().unwrap_or(""),
            project.created_at.as_deref().unwrap_or("")
        )
    }

    fn get_presentation_xml(&self, slide_count: usize) -> String {
        let slide_ids: Vec<String> = (1..=slide_count)
            .map(|i| format!(r#"<p:sldId id="{}" r:id="rId{}"/>"#, 255 + i, i + 1))
            .collect();

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <p:sldIdLst>
        {}
    </p:sldIdLst>
    <p:sldSz cx="19200000" cy="10800000"/>
    <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#,
            slide_ids.join("\n        ")
        )
    }

    fn get_presentation_rels_xml(&self, slide_count: usize) -> String {
        let mut relationships: Vec<String> = (1..=slide_count)
            .map(|i| {
                format!(
                    r#"<Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide{}.xml"/>"#,
                    i + 1, i
                )
            })
            .collect();

        relationships.insert(0, r#"<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>"#.to_string());

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    {}
</Relationships>"#,
            relationships.join("\n    ")
        )
    }

    fn generate_slide_xml(&self, page: &GeneratedPage) -> Result<String> {
        let mut shapes = String::new();

        for (index, element) in page.elements.iter().enumerate() {
            let shape_xml = self.element_to_shape_xml(element, index)?;
            shapes.push_str(&shape_xml);
            shapes.push('\n');
        }

        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <p:cSld>
        <p:spTree>
            <p:nvGrpSpPr>
                <p:cNvPr id="1" name=""/>
                <p:nvGrpSpPr/>
            </p:nvGrpSpPr>
            <p:grpSpPr>
                <a:xfrm xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                    <a:off x="0" y="0"/>
                    <a:ext cx="0" cy="0"/>
                    <a:chOff x="0" y="0"/>
                    <a:chExt cx="0" cy="0"/>
                </a:xfrm>
            </p:grpSpPr>
            {}
        </p:spTree>
    </p:cSld>
</p:sld>"#,
            shapes
        ))
    }

    fn element_to_shape_xml(&self, element: &PageElement, index: usize) -> Result<String> {
        let shape_id = index + 2;
        let x = element.position.x * 914400 / 254;
        let y = element.position.y * 914400 / 254;
        let cx = element.position.width * 914400 / 254;
        let cy = element.position.height * 914400 / 254;

        match element.element_type {
            ElementType::Text => {
                let text = element.content.text.as_ref()
                    .map(|t| t.text.clone())
                    .unwrap_or_default();
                let font_size = element.content.text.as_ref()
                    .and_then(|t| t.font_size)
                    .unwrap_or(18) * 100;
                let color = element.content.text.as_ref()
                    .and_then(|t| t.color.clone())
                    .unwrap_or_else(|| "#000000".to_string());
                let color_hex = color.trim_start_matches('#');

                Ok(format!(
                    r#"<p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="{}" name="Text {}"/>
                    <p:nvSpPr txBox="1"/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:off x="{}" y="{}"/>
                        <a:ext cx="{}" cy="{}"/>
                    </a:xfrm>
                    <a:prstTxWrap xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:prstTxWrap/>
                    </a:prstTxWrap>
                </p:spPr>
                <p:txBody>
                    <a:bodyPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" wrap="square"/>
                    <a:p xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:r>
                            <a:rPr lang="zh-CN" sz="{}">
                                <a:solidFill>
                                    <a:srgbClr val="{}"/>
                                </a:solidFill>
                            </a:rPr>
                            <a:t>{}</a:t>
                        </a:r>
                    </a:p>
                </p:txBody>
            </p:sp>"#,
                    shape_id, index, x, y, cx, cy, font_size, color_hex, text
                ))
            }
            ElementType::Image => {
                Ok(format!(
                    r#"<p:pic>
                <p:nvPicPr>
                    <p:cNvPr id="{}" name="Image {}"/>
                    <p:cNvPicPr/>
                    <p:nvPr/>
                </p:nvPicPr>
                <p:blipFill>
                    <a:blip xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" r:embed="rId1"/>
                    <a:stretch xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:fillRect/>
                    </a:stretch>
                </p:blipFill>
                <p:spPr>
                    <a:xfrm xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:off x="{}" y="{}"/>
                        <a:ext cx="{}" cy="{}"/>
                    </a:xfrm>
                </p:spPr>
            </p:pic>"#,
                    shape_id, index, x, y, cx, cy
                ))
            }
            _ => {
                Ok(format!(
                    r#"<p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="{}" name="Shape {}"/>
                    <p:nvSpPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
                        <a:off x="{}" y="{}"/>
                        <a:ext cx="{}" cy="{}"/>
                    </a:xfrm>
                </p:spPr>
            </p:sp>"#,
                    shape_id, index, x, y, cx, cy
                ))
            }
        }
    }

    fn get_slide_layout_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank">
    <p:cSld>
        <p:spTree>
            <p:nvGrpSpPr>
                <p:cNvPr id="1" name=""/>
                <p:nvGrpSpPr/>
            </p:nvGrpSpPr>
            <p:grpSpPr/>
        </p:spTree>
    </p:cSld>
</p:sldLayout>"#.to_string()
    }

    fn get_slide_master_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
    <p:cSld>
        <p:bg/>
        <p:spTree>
            <p:nvGrpSpPr>
                <p:cNvPr id="1" name=""/>
                <p:nvGrpSpPr/>
            </p:nvGrpSpPr>
            <p:grpSpPr/>
        </p:spTree>
    </p:cSld>
</p:sldMaster>"#.to_string()
    }

    fn get_theme_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
    <a:themeElements>
        <a:clrScheme name="Office">
            <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
            <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
            <a:dk2><a:srgbClr val="1F497D"/></a:dk2>
            <a:lt2><a:srgbClr val="EEECE1"/></a:lt2>
            <a:accent1><a:srgbClr val="4F81BD"/></a:accent1>
            <a:accent2><a:srgbClr val="C0504D"/></a:accent2>
            <a:accent3><a:srgbClr val="9BBB59"/></a:accent3>
            <a:accent4><a:srgbClr val="8064A2"/></a:accent4>
            <a:accent5><a:srgbClr val="4BACC6"/></a:accent5>
            <a:accent6><a:srgbClr val="F79646"/></a:accent6>
            <a:hlink><a:srgbClr val="0000FF"/></a:hlink>
            <a:folHlink><a:srgbClr val="800080"/></a:folHlink>
        </a:clrScheme>
        <a:fontScheme name="Office">
            <a:majorFont>
                <a:latin typeface="Arial"/>
                <a:ea typeface=""/>
                <a:cs typeface=""/>
            </a:majorFont>
            <a:minorFont>
                <a:latin typeface="Arial"/>
                <a:ea typeface=""/>
                <a:cs typeface=""/>
            </a:minorFont>
        </a:fontScheme>
        <a:fmtScheme name="Office">
            <a:fillStyleLst>
                <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
            </a:fillStyleLst>
            <a:lnStyleLst>
                <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
                    <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
                </a:ln>
            </a:lnStyleLst>
        </a:fmtScheme>
    </a:themeElements>
</a:theme>"#.to_string()
    }

    fn generate_html_presentation(&self, project: &PPTProject, pages: &[GeneratedPage]) -> Result<String> {
        let slides_html: Vec<String> = pages.iter()
            .enumerate()
            .map(|(index, page)| self.page_to_html(page, index))
            .collect();

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: Arial, sans-serif; background: #f0f0f0; }}
        .presentation {{ width: 100%; max-width: 1920px; margin: 0 auto; }}
        .slide {{ 
            width: 100%; 
            aspect-ratio: 16/9; 
            background: white; 
            margin: 20px 0; 
            position: relative;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        .element {{ position: absolute; }}
        .text {{ overflow: hidden; }}
        .image {{ object-fit: contain; }}
        .navigation {{ 
            position: fixed; 
            bottom: 20px; 
            right: 20px; 
            background: white; 
            padding: 10px 20px;
            border-radius: 5px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.2);
        }}
        .navigation button {{ 
            padding: 8px 16px; 
            margin: 0 5px; 
            cursor: pointer;
            border: none;
            background: #007bff;
            color: white;
            border-radius: 3px;
        }}
        .navigation button:hover {{ background: #0056b3; }}
    </style>
</head>
<body>
    <div class="presentation">
        {}
    </div>
    <div class="navigation">
        <button onclick="prevSlide()">上一页</button>
        <span id="pageInfo">1 / {}</span>
        <button onclick="nextSlide()">下一页</button>
    </div>
    <script>
        let currentSlide = 0;
        const slides = document.querySelectorAll('.slide');
        
        function showSlide(index) {{
            slides.forEach((slide, i) => {{
                slide.style.display = i === index ? 'block' : 'none';
            }});
            document.getElementById('pageInfo').textContent = `${{index + 1}} / ${{slides.length}}`;
        }}
        
        function nextSlide() {{
            currentSlide = (currentSlide + 1) % slides.length;
            showSlide(currentSlide);
        }}
        
        function prevSlide() {{
            currentSlide = (currentSlide - 1 + slides.length) % slides.length;
            showSlide(currentSlide);
        }}
        
        showSlide(0);
    </script>
</body>
</html>"#,
            project.title,
            slides_html.join("\n        "),
            pages.len()
        ))
    }

    fn page_to_html(&self, page: &GeneratedPage, _index: usize) -> String {
        let bg_style = if let Some(ref bg) = page.background {
            match bg.background_type {
                BackgroundType::Solid => {
                    format!("background-color: {};", bg.color.as_deref().unwrap_or("#FFFFFF"))
                }
                BackgroundType::Gradient => {
                    if let Some(ref gradient) = bg.gradient {
                        let colors: Vec<String> = gradient.colors.iter()
                            .map(|c| c.color.clone())
                            .collect();
                        format!("background: linear-gradient({}deg, {});", 
                            gradient.angle.unwrap_or(0.0),
                            colors.join(", "))
                    } else {
                        "background-color: #FFFFFF;".to_string()
                    }
                }
                BackgroundType::Image => {
                    if let Some(ref img) = bg.image {
                        format!("background-image: url('{}'); background-size: {}; opacity: {};", 
                            img.url, img.fit_mode, img.opacity.unwrap_or(1.0))
                    } else {
                        "background-color: #FFFFFF;".to_string()
                    }
                }
                _ => "background-color: #FFFFFF;".to_string()
            }
        } else {
            "background-color: #FFFFFF;".to_string()
        };

        let elements_html: Vec<String> = page.elements.iter()
            .map(|e| self.element_to_html(e))
            .collect();

        format!(
            r#"<div class="slide" style="{}">
            {}
        </div>"#,
            bg_style,
            elements_html.join("\n            ")
        )
    }

    fn element_to_html(&self, element: &PageElement) -> String {
        let style = format!(
            "left: {}px; top: {}px; width: {}px; height: {}px;",
            element.position.x,
            element.position.y,
            element.position.width,
            element.position.height
        );

        match element.element_type {
            ElementType::Text => {
                if let Some(ref text) = element.content.text {
                    let font_style = format!(
                        "font-family: {}; font-size: {}px; font-weight: {}; color: {}; text-align: {}; line-height: {};",
                        text.font_family.as_deref().unwrap_or("Arial"),
                        text.font_size.unwrap_or(18),
                        text.font_weight.as_deref().unwrap_or("normal"),
                        text.color.as_deref().unwrap_or("#000000"),
                        text.alignment.as_deref().unwrap_or("left"),
                        text.line_height.unwrap_or(1.5)
                    );
                    format!(r#"<div class="element text" style="{} {}">{}</div>"#, style, font_style, text.text)
                } else {
                    String::new()
                }
            }
            ElementType::Image => {
                if let Some(ref img) = element.content.image {
                    let src = img.url.as_deref()
                        .or_else(|| img.data.as_deref())
                        .unwrap_or("");
                    format!(r#"<img class="element image" style="{}" src="{}" alt="{}"/>"#, 
                        style, src, img.alt_text.as_deref().unwrap_or(""))
                } else {
                    String::new()
                }
            }
            _ => format!(r#"<div class="element" style="{}"></div>"#, style)
        }
    }

    fn generate_pdf_html(&self, project: &PPTProject, pages: &[GeneratedPage]) -> Result<String> {
        self.generate_html_presentation(project, pages)
    }

    fn render_page_to_image(&self, page: &GeneratedPage, _format: &ImageFormat) -> Result<Vec<u8>> {
        let html = self.page_to_html(page, 0);
        Ok(html.as_bytes().to_vec())
    }

    pub fn get_output_path(&self, project_id: i64, format: &OutputFormat) -> PathBuf {
        let extension = match format {
            OutputFormat::Pptx => "pptx",
            OutputFormat::Pdf => "pdf",
            OutputFormat::Images(img_fmt) => img_fmt.as_str(),
            OutputFormat::Html => "html",
            OutputFormat::Json => "json",
        };

        PathBuf::from(format!("project_{}.{}", project_id, extension))
    }

    pub async fn ensure_output_dir(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.output_dir).await?;
        Ok(())
    }
}

impl Default for PptExporter {
    fn default() -> Self {
        Self::default_output_dir()
    }
}
