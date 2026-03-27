use crate::domain::model::m_code_gen::GeneratedFile;
use crate::service::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};
use zip::write::FileOptions;
use zip::ZipWriter;

use super::code_generator_service;
use super::frontend_code_generator;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<FileTreeNode>,
}

pub async fn pack(model_id: i64) -> Result<Vec<u8>> {
    let backend_files = code_generator_service::preview(model_id).await?;
    let frontend_files = frontend_code_generator::generate_frontend_files(model_id).await?;
    
    let all_files: Vec<GeneratedFile> = backend_files.into_iter()
        .chain(frontend_files.into_iter())
        .collect();
    
    create_zip(&all_files)
}

pub async fn get_file_tree(model_id: i64) -> Result<FileTreeNode> {
    let backend_files = code_generator_service::preview(model_id).await?;
    let frontend_files = frontend_code_generator::generate_frontend_files(model_id).await?;
    
    let all_files: Vec<GeneratedFile> = backend_files.into_iter()
        .chain(frontend_files.into_iter())
        .collect();
    
    Ok(build_file_tree(&all_files))
}

pub async fn download(model_id: i64) -> Result<Vec<u8>> {
    pack(model_id).await
}

fn create_zip(files: &[GeneratedFile]) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut buffer);
        let options = FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);
        
        for file in files {
            zip.start_file(&file.file_path, options)
                .map_err(|e| Error::internal_error(format!("Failed to create zip entry: {}", e)))?;
            
            zip.write_all(file.content.as_bytes())
                .map_err(|e| Error::internal_error(format!("Failed to write file content: {}", e)))?;
        }
        
        zip.finish()
            .map_err(|e| Error::internal_error(format!("Failed to finalize zip: {}", e)))?;
    }
    
    Ok(buffer.into_inner())
}

fn build_file_tree(files: &[GeneratedFile]) -> FileTreeNode {
    let mut root = FileTreeNode {
        name: "generated".to_string(),
        path: "".to_string(),
        is_dir: true,
        children: Vec::new(),
    };
    
    for file in files {
        let parts: Vec<&str> = file.file_path.split('/').collect();
        insert_into_tree(&mut root, &parts, 0);
    }
    
    root
}

fn insert_into_tree(node: &mut FileTreeNode, parts: &[&str], depth: usize) {
    if depth >= parts.len() {
        return;
    }
    
    let current_name = parts[depth].to_string();
    let current_path = parts[..=depth].join("/");
    
    if depth == parts.len() - 1 {
        if !node.children.iter().any(|c| c.name == current_name) {
            node.children.push(FileTreeNode {
                name: current_name,
                path: current_path,
                is_dir: false,
                children: Vec::new(),
            });
        }
    } else {
        if let Some(child) = node.children.iter_mut().find(|c| c.name == current_name) {
            insert_into_tree(child, parts, depth + 1);
        } else {
            let mut new_node = FileTreeNode {
                name: current_name.clone(),
                path: current_path,
                is_dir: true,
                children: Vec::new(),
            };
            insert_into_tree(&mut new_node, parts, depth + 1);
            node.children.push(new_node);
        }
    }
}

pub async fn preview_all(model_id: i64) -> Result<Vec<GeneratedFile>> {
    let backend_files = code_generator_service::preview(model_id).await?;
    let frontend_files = frontend_code_generator::generate_frontend_files(model_id).await?;
    
    let all_files: Vec<GeneratedFile> = backend_files.into_iter()
        .chain(frontend_files.into_iter())
        .collect();
    
    Ok(all_files)
}

pub async fn preview_backend(model_id: i64) -> Result<Vec<GeneratedFile>> {
    code_generator_service::preview(model_id).await
}

pub async fn preview_frontend(model_id: i64) -> Result<Vec<GeneratedFile>> {
    frontend_code_generator::generate_frontend_files(model_id).await
}
