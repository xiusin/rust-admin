pub mod category_handler;
pub mod code_gen_handler;
pub mod content_handler;
pub mod field_handler;
pub mod form_handler;
pub mod model_handler;
pub mod table_handler;
pub mod tag_handler;

use axum::routing::{delete, get, post, put};

use crate::api::web_path::{WebPath, WebPathType};

pub fn cms_model() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取模型列表"),
            get(model_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取模型详情"),
            get(model_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建模型"),
            post(model_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑模型"),
            put(model_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除模型"),
            delete(model_handler::delete),
        )
        .route(
            "/enable/{id}",
            WebPathType::Post,
            Some("启用模型"),
            post(model_handler::enable),
        )
        .route(
            "/disable/{id}",
            WebPathType::Post,
            Some("禁用模型"),
            post(model_handler::disable),
        )
        .route(
            "/copy/{id}",
            WebPathType::Post,
            Some("复制模型"),
            post(model_handler::copy),
        )
}

pub fn cms_field() -> WebPath {
    WebPath::new()
        .route(
            "/list/{model_id}",
            WebPathType::Get,
            Some("获取字段列表"),
            get(field_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取字段详情"),
            get(field_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建字段"),
            post(field_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑字段"),
            put(field_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除字段"),
            delete(field_handler::delete),
        )
        .route(
            "/sort",
            WebPathType::Post,
            Some("字段排序"),
            post(field_handler::sort),
        )
}

pub fn cms_content() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取内容列表"),
            get(content_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取内容详情"),
            get(content_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建内容"),
            post(content_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑内容"),
            put(content_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除内容"),
            delete(content_handler::delete),
        )
        .route(
            "/restore/{id}",
            WebPathType::Post,
            Some("恢复内容"),
            post(content_handler::restore),
        )
        .route(
            "/publish/{id}",
            WebPathType::Post,
            Some("发布内容"),
            post(content_handler::publish),
        )
        .route(
            "/offline/{id}",
            WebPathType::Post,
            Some("下线内容"),
            post(content_handler::offline),
        )
        .route(
            "/audit",
            WebPathType::Post,
            Some("审核内容"),
            post(content_handler::audit),
        )
        .route(
            "/version/{id}",
            WebPathType::Get,
            Some("获取版本历史"),
            get(content_handler::version),
        )
        .route(
            "/version/rollback",
            WebPathType::Post,
            Some("版本回滚"),
            post(content_handler::version_rollback),
        )
}

pub fn cms_category() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取分类列表"),
            get(category_handler::list),
        )
        .route(
            "/tree",
            WebPathType::Get,
            Some("获取分类树"),
            get(category_handler::tree),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取分类详情"),
            get(category_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建分类"),
            post(category_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑分类"),
            put(category_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除分类"),
            delete(category_handler::delete),
        )
        .route(
            "/sort",
            WebPathType::Post,
            Some("分类排序"),
            post(category_handler::sort),
        )
}

pub fn cms_tag() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取标签列表"),
            get(tag_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取标签详情"),
            get(tag_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建标签"),
            post(tag_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑标签"),
            put(tag_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除标签"),
            delete(tag_handler::delete),
        )
        .route(
            "/batch-add",
            WebPathType::Post,
            Some("批量添加标签"),
            post(tag_handler::batch_add),
        )
        .route(
            "/cloud",
            WebPathType::Get,
            Some("获取标签云"),
            get(tag_handler::cloud),
        )
}

pub fn cms_form() -> WebPath {
    WebPath::new()
        .route(
            "/list/{model_id}",
            WebPathType::Get,
            Some("获取表单配置列表"),
            get(form_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取表单配置详情"),
            get(form_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建表单配置"),
            post(form_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑表单配置"),
            put(form_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除表单配置"),
            delete(form_handler::delete),
        )
        .route(
            "/preview/{id}",
            WebPathType::Get,
            Some("预览表单Schema"),
            get(form_handler::preview),
        )
}

pub fn cms_table() -> WebPath {
    WebPath::new()
        .route(
            "/list/{model_id}",
            WebPathType::Get,
            Some("获取表格配置列表"),
            get(table_handler::list),
        )
        .route(
            "/detail/{id}",
            WebPathType::Get,
            Some("获取表格配置详情"),
            get(table_handler::detail),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("创建表格配置"),
            post(table_handler::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑表格配置"),
            put(table_handler::edit),
        )
        .route(
            "/delete/{id}",
            WebPathType::Delete,
            Some("删除表格配置"),
            delete(table_handler::delete),
        )
        .route(
            "/preview/{id}",
            WebPathType::Get,
            Some("预览表格Schema"),
            get(table_handler::preview),
        )
}

pub fn cms_code_gen() -> WebPath {
    WebPath::new()
        .route(
            "/preview/{model_id}",
            WebPathType::Get,
            Some("预览生成代码"),
            get(code_gen_handler::preview),
        )
        .route(
            "/download/{model_id}",
            WebPathType::Get,
            Some("下载代码包"),
            get(code_gen_handler::download),
        )
        .route(
            "/file-tree/{model_id}",
            WebPathType::Get,
            Some("获取文件树"),
            get(code_gen_handler::file_tree),
        )
}

pub fn router_cms() -> WebPath {
    WebPath::new().nest(
        "/cms",
        WebPath::new()
            .nest("/model", cms_model())
            .nest("/field", cms_field())
            .nest("/content", cms_content())
            .nest("/category", cms_category())
            .nest("/tag", cms_tag())
            .nest("/form", cms_form())
            .nest("/table", cms_table())
            .nest("/code-gen", cms_code_gen()),
    )
}
