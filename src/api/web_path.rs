use std::collections::HashMap;

use axum::routing::MethodRouter;
use serde::{Deserialize, Serialize};
use tracing::info;
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum WebPathType {
    #[default]
    None,
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WebPath {
    pub final_path: String,
    pub webmethod: WebPathType,
    #[serde(skip)]
    pub method_router: Option<MethodRouter>,
    sub_paths: HashMap<String, WebPath>,
    pub apiname: Option<String>,
}

impl WebPath {
    pub fn new() -> Self {
        WebPath::default()
    }
    pub fn nest(mut self, path: &str, web_path: WebPath) -> Self {
        self.sub_paths.insert(String::from(path), web_path);
        self
    }

    pub fn merge(mut self, web_path: WebPath) -> Self {
        for (sub_key, sub_path) in web_path.sub_paths {
            self.sub_paths.insert(sub_key, sub_path);
        }
        self
    }
    pub fn route(
        mut self,
        path: &str,
        method: WebPathType,
        apiname: Option<&str>,
        method_router: MethodRouter,
    ) -> Self {
        self.sub_paths.insert(
            String::from(path),
            WebPath {
                webmethod: method,
                apiname: apiname.map(String::from),
                method_router: Some(method_router),
                sub_paths: HashMap::new(),
                ..Default::default()
            },
        );
        self
    }

    fn concat_sub_paths_final_paths(&mut self, parent_path: &str) {
        for (sub_key, sub_path) in &mut self.sub_paths {
            let converted_key = Self::convert_path_params(sub_key);
            let f_path = format!("{}{}", parent_path, converted_key);
            sub_path.concat_sub_paths_final_paths(&f_path);
            if sub_path.is_last_level() {
                sub_path.final_path = f_path;
            }
        }
    }

    fn convert_path_params(path: &str) -> String {
        // Axum 0.8+ 要求使用 {id} 而不是 :id 作为路径参数
        // 使用正则表达式匹配所有 :param_name 格式并转换为 {param_name}
        let mut result = path.to_string();
        
        // 匹配所有以冒号开头的路径参数，如 :id, :product_id 等
        // 使用简单的状态机来解析
        let mut converted = String::new();
        let chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == ':' && (i == 0 || chars[i - 1] == '/') {
                // 找到路径参数的开始
                let start = i + 1;
                let mut end = start;
                
                // 找到参数名的结束位置
                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                
                // 提取参数名
                let param_name: String = chars[start..end].iter().collect();
                converted.push('{');
                converted.push_str(&param_name);
                converted.push('}');
                i = end;
            } else {
                converted.push(chars[i]);
                i += 1;
            }
        }
        
        converted
    }
    pub fn final_to_path(mut self) -> Self {
        self.concat_sub_paths_final_paths("");
        self
    }

    fn is_last_level(&self) -> bool {
        self.sub_paths.is_empty()
    }

    pub fn get_last_level_paths(&self) -> Vec<&WebPath> {
        let mut last_level_paths = Vec::new();
        if self.is_last_level() && self.webmethod != WebPathType::None {
            last_level_paths.push(self);
        }
        for sub_path in self.sub_paths.values() {
            last_level_paths.extend(sub_path.get_last_level_paths());
        }
        last_level_paths
    }

    pub fn print_all_paths(&self) {
        for sub_path_data in self.sub_paths.values() {
            if sub_path_data.is_last_level() {
                info!("{}", sub_path_data.final_path);
            }

            sub_path_data.print_all_paths();
        }
    }
}

impl WebPathType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebPathType::None => "None",
            WebPathType::Get => "Get",
            WebPathType::Post => "Post",
            WebPathType::Put => "Put",
            WebPathType::Delete => "Delete",
        }
    }
}
