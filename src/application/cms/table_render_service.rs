use crate::domain::entity::{cms_field, cms_model, cms_table_config};
use crate::service::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableSchema {
    pub columns: Vec<ColumnSchema>,
    pub search: Option<SearchSchema>,
    pub filter: Option<FilterSchema>,
    pub actions: Vec<ActionSchema>,
    pub batch_actions: Vec<BatchActionSchema>,
    pub toolbar: Vec<ToolbarActionSchema>,
    pub pagination: PaginationSchema,
    pub features: TableFeaturesSchema,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnSchema {
    pub field: String,
    pub title: String,
    pub width: Option<u32>,
    pub align: String,
    pub fixed: Option<String>,
    pub sortable: bool,
    pub render: RenderSchema,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderSchema {
    pub render_type: String,
    pub props: Option<JsonValue>,
    pub options: Option<Vec<RenderOption>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderOption {
    pub label: String,
    pub value: JsonValue,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSchema {
    pub fields: Vec<SearchField>,
    pub layout: String,
    pub collapsed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchField {
    pub field: String,
    pub label: String,
    pub component: String,
    pub placeholder: Option<String>,
    pub operator: String,
    pub props: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterSchema {
    pub fields: Vec<FilterField>,
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterField {
    pub field: String,
    pub label: String,
    pub filter_type: String,
    pub options: Option<Vec<FilterOption>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterOption {
    pub label: String,
    pub value: JsonValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionSchema {
    pub action_type: String,
    pub label: String,
    pub code: String,
    pub icon: Option<String>,
    pub visible: Option<JsonValue>,
    pub disabled: Option<JsonValue>,
    pub props: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchActionSchema {
    pub action_type: String,
    pub label: String,
    pub code: String,
    pub icon: Option<String>,
    pub confirm: Option<String>,
    pub props: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolbarActionSchema {
    pub action_type: String,
    pub label: String,
    pub code: String,
    pub icon: Option<String>,
    pub props: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationSchema {
    pub enabled: bool,
    pub page_sizes: Vec<u32>,
    pub default_page_size: u32,
    pub layout: String,
}

impl Default for PaginationSchema {
    fn default() -> Self {
        Self {
            enabled: true,
            page_sizes: vec![10, 20, 50, 100],
            default_page_size: 20,
            layout: "total, sizes, prev, pager, next, jumper".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableFeaturesSchema {
    pub selection: bool,
    pub index: bool,
    pub expand: bool,
    pub stripe: bool,
    pub border: bool,
    pub highlight_current_row: bool,
    pub show_header: bool,
    pub show_summary: bool,
}

impl Default for TableFeaturesSchema {
    fn default() -> Self {
        Self {
            selection: true,
            index: true,
            expand: false,
            stripe: true,
            border: true,
            highlight_current_row: false,
            show_header: true,
            show_summary: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderTypeItem {
    pub code: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

pub async fn render_schema(table_id: i64) -> Result<TableSchema> {
    let db = DB().await;
    
    let table_config = cms_table_config::Entity::find_by_id(table_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表格配置不存在"))?;
    
    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(table_config.model_id))
        .filter(cms_field::Column::IsListShow.eq("1"))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;
    
    let columns = parse_columns(&table_config.columns, &fields)?;
    let search = parse_search(&table_config.search, &fields)?;
    let filter = parse_filter(&table_config.filter, &fields)?;
    let actions = parse_actions(&table_config.actions)?;
    let batch_actions = parse_batch_actions(&table_config.batch_actions)?;
    let toolbar = parse_toolbar(&table_config.toolbar)?;
    let pagination = parse_pagination(&table_config.pagination)?;
    let features = parse_features(&table_config.features)?;
    
    Ok(TableSchema {
        columns,
        search,
        filter,
        actions,
        batch_actions,
        toolbar,
        pagination,
        features,
    })
}

pub async fn render_by_model(model_id: i64) -> Result<TableSchema> {
    let db = DB().await;
    
    let default_config = cms_table_config::Entity::find()
        .filter(cms_table_config::Column::ModelId.eq(model_id))
        .filter(cms_table_config::Column::IsDefault.eq("1"))
        .filter(cms_table_config::Column::Status.eq("0"))
        .one(db)
        .await?;
    
    if let Some(config) = default_config {
        return render_schema(config.id).await;
    }
    
    let _model = cms_model::Entity::find_by_id(model_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;
    
    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(model_id))
        .filter(cms_field::Column::IsListShow.eq("1"))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;
    
    let columns: Vec<ColumnSchema> = fields
        .iter()
        .map(|f| build_column_schema(f))
        .collect();
    
    let search_fields: Vec<SearchField> = fields
        .iter()
        .filter(|f| f.is_searchable == "1")
        .map(|f| build_search_field(f))
        .collect();
    
    let search = if search_fields.is_empty() {
        None
    } else {
        Some(SearchSchema {
            fields: search_fields,
            layout: "inline".to_string(),
            collapsed: true,
        })
    };
    
    let default_actions = vec![
        ActionSchema {
            action_type: "edit".to_string(),
            label: "编辑".to_string(),
            code: "edit".to_string(),
            icon: Some("edit".to_string()),
            visible: None,
            disabled: None,
            props: None,
        },
        ActionSchema {
            action_type: "delete".to_string(),
            label: "删除".to_string(),
            code: "delete".to_string(),
            icon: Some("delete".to_string()),
            visible: None,
            disabled: None,
            props: None,
        },
    ];
    
    let default_batch_actions = vec![BatchActionSchema {
        action_type: "delete".to_string(),
        label: "批量删除".to_string(),
        code: "batch_delete".to_string(),
        icon: Some("delete".to_string()),
        confirm: Some("确定要删除选中的数据吗？".to_string()),
        props: None,
    }];
    
    let default_toolbar = vec![
        ToolbarActionSchema {
            action_type: "add".to_string(),
            label: "新增".to_string(),
            code: "add".to_string(),
            icon: Some("plus".to_string()),
            props: None,
        },
        ToolbarActionSchema {
            action_type: "export".to_string(),
            label: "导出".to_string(),
            code: "export".to_string(),
            icon: Some("download".to_string()),
            props: None,
        },
    ];
    
    Ok(TableSchema {
        columns,
        search,
        filter: None,
        actions: default_actions,
        batch_actions: default_batch_actions,
        toolbar: default_toolbar,
        pagination: PaginationSchema::default(),
        features: TableFeaturesSchema::default(),
    })
}

pub async fn get_render_types() -> Result<Vec<RenderTypeItem>> {
    let render_types = vec![
        RenderTypeItem {
            code: "text".to_string(),
            name: "文本".to_string(),
            category: "basic".to_string(),
            description: Some("普通文本显示".to_string()),
        },
        RenderTypeItem {
            code: "link".to_string(),
            name: "链接".to_string(),
            category: "basic".to_string(),
            description: Some("超链接显示".to_string()),
        },
        RenderTypeItem {
            code: "image".to_string(),
            name: "图片".to_string(),
            category: "media".to_string(),
            description: Some("图片预览".to_string()),
        },
        RenderTypeItem {
            code: "tag".to_string(),
            name: "标签".to_string(),
            category: "status".to_string(),
            description: Some("标签状态显示".to_string()),
        },
        RenderTypeItem {
            code: "badge".to_string(),
            name: "徽章".to_string(),
            category: "status".to_string(),
            description: Some("徽章状态显示".to_string()),
        },
        RenderTypeItem {
            code: "progress".to_string(),
            name: "进度条".to_string(),
            category: "status".to_string(),
            description: Some("进度条显示".to_string()),
        },
        RenderTypeItem {
            code: "switch".to_string(),
            name: "开关".to_string(),
            category: "status".to_string(),
            description: Some("开关状态显示".to_string()),
        },
        RenderTypeItem {
            code: "date".to_string(),
            name: "日期".to_string(),
            category: "format".to_string(),
            description: Some("日期格式化显示".to_string()),
        },
        RenderTypeItem {
            code: "datetime".to_string(),
            name: "日期时间".to_string(),
            category: "format".to_string(),
            description: Some("日期时间格式化显示".to_string()),
        },
        RenderTypeItem {
            code: "number".to_string(),
            name: "数字".to_string(),
            category: "format".to_string(),
            description: Some("数字格式化显示".to_string()),
        },
        RenderTypeItem {
            code: "money".to_string(),
            name: "金额".to_string(),
            category: "format".to_string(),
            description: Some("金额格式化显示".to_string()),
        },
        RenderTypeItem {
            code: "percent".to_string(),
            name: "百分比".to_string(),
            category: "format".to_string(),
            description: Some("百分比格式化显示".to_string()),
        },
        RenderTypeItem {
            code: "dict".to_string(),
            name: "字典".to_string(),
            category: "mapping".to_string(),
            description: Some("字典映射显示".to_string()),
        },
        RenderTypeItem {
            code: "enum".to_string(),
            name: "枚举".to_string(),
            category: "mapping".to_string(),
            description: Some("枚举映射显示".to_string()),
        },
        RenderTypeItem {
            code: "custom".to_string(),
            name: "自定义".to_string(),
            category: "advanced".to_string(),
            description: Some("自定义渲染组件".to_string()),
        },
    ];
    
    Ok(render_types)
}

fn parse_columns(columns_str: &Option<String>, fields: &[cms_field::Model]) -> Result<Vec<ColumnSchema>> {
    match columns_str {
        Some(s) if !s.is_empty() => {
            let columns: Vec<ColumnSchema> = serde_json::from_str(s)
                .map_err(|e| Error::bad_request(format!("列配置解析失败: {}", e)))?;
            
            if columns.is_empty() {
                return Ok(fields.iter().map(|f| build_column_schema(f)).collect());
            }
            
            Ok(columns)
        }
        _ => Ok(fields.iter().map(|f| build_column_schema(f)).collect()),
    }
}

fn parse_search(search_str: &Option<String>, fields: &[cms_field::Model]) -> Result<Option<SearchSchema>> {
    match search_str {
        Some(s) if !s.is_empty() => {
            let search: SearchSchema = serde_json::from_str(s)
                .map_err(|e| Error::bad_request(format!("搜索配置解析失败: {}", e)))?;
            Ok(Some(search))
        }
        _ => {
            let search_fields: Vec<SearchField> = fields
                .iter()
                .filter(|f| f.is_searchable == "1")
                .map(|f| build_search_field(f))
                .collect();
            
            if search_fields.is_empty() {
                Ok(None)
            } else {
                Ok(Some(SearchSchema {
                    fields: search_fields,
                    layout: "inline".to_string(),
                    collapsed: true,
                }))
            }
        }
    }
}

fn parse_filter(filter_str: &Option<String>, _fields: &[cms_field::Model]) -> Result<Option<FilterSchema>> {
    match filter_str {
        Some(s) if !s.is_empty() => {
            let filter: FilterSchema = serde_json::from_str(s)
                .map_err(|e| Error::bad_request(format!("筛选配置解析失败: {}", e)))?;
            Ok(Some(filter))
        }
        _ => Ok(None),
    }
}

fn parse_actions(actions_str: &Option<String>) -> Result<Vec<ActionSchema>> {
    match actions_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("操作配置解析失败: {}", e)))
        }
        _ => Ok(vec![
            ActionSchema {
                action_type: "edit".to_string(),
                label: "编辑".to_string(),
                code: "edit".to_string(),
                icon: Some("edit".to_string()),
                visible: None,
                disabled: None,
                props: None,
            },
            ActionSchema {
                action_type: "delete".to_string(),
                label: "删除".to_string(),
                code: "delete".to_string(),
                icon: Some("delete".to_string()),
                visible: None,
                disabled: None,
                props: None,
            },
        ]),
    }
}

fn parse_batch_actions(batch_actions_str: &Option<String>) -> Result<Vec<BatchActionSchema>> {
    match batch_actions_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("批量操作配置解析失败: {}", e)))
        }
        _ => Ok(vec![BatchActionSchema {
            action_type: "delete".to_string(),
            label: "批量删除".to_string(),
            code: "batch_delete".to_string(),
            icon: Some("delete".to_string()),
            confirm: Some("确定要删除选中的数据吗？".to_string()),
            props: None,
        }]),
    }
}

fn parse_toolbar(toolbar_str: &Option<String>) -> Result<Vec<ToolbarActionSchema>> {
    match toolbar_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("工具栏配置解析失败: {}", e)))
        }
        _ => Ok(vec![
            ToolbarActionSchema {
                action_type: "add".to_string(),
                label: "新增".to_string(),
                code: "add".to_string(),
                icon: Some("plus".to_string()),
                props: None,
            },
            ToolbarActionSchema {
                action_type: "export".to_string(),
                label: "导出".to_string(),
                code: "export".to_string(),
                icon: Some("download".to_string()),
                props: None,
            },
        ]),
    }
}

fn parse_pagination(pagination_str: &Option<String>) -> Result<PaginationSchema> {
    match pagination_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("分页配置解析失败: {}", e)))
        }
        _ => Ok(PaginationSchema::default()),
    }
}

fn parse_features(features_str: &Option<String>) -> Result<TableFeaturesSchema> {
    match features_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("特性配置解析失败: {}", e)))
        }
        _ => Ok(TableFeaturesSchema::default()),
    }
}

fn build_column_schema(field: &cms_field::Model) -> ColumnSchema {
    let render = build_render_schema(field);
    
    ColumnSchema {
        field: field.code.clone(),
        title: field.name.clone(),
        width: get_column_width(field),
        align: get_column_align(field),
        fixed: None,
        sortable: field.is_sortable == "1",
        render,
        visible: true,
    }
}

fn build_render_schema(field: &cms_field::Model) -> RenderSchema {
    let render_type = get_render_type_by_field(field);
    let props = parse_table_config_props(&field.table_config);
    
    RenderSchema {
        render_type,
        props,
        options: None,
    }
}

fn get_render_type_by_field(field: &cms_field::Model) -> String {
    match field.field_type.as_str() {
        "boolean" | "switch" => "switch".to_string(),
        "date" => "date".to_string(),
        "datetime" => "datetime".to_string(),
        "image" | "file" => "image".to_string(),
        "integer" | "decimal" | "float" | "number" => "number".to_string(),
        "money" => "money".to_string(),
        "percent" => "percent".to_string(),
        "select" | "enum" | "radio" => "tag".to_string(),
        "status" => "badge".to_string(),
        "progress" => "progress".to_string(),
        "link" => "link".to_string(),
        _ => "text".to_string(),
    }
}

fn get_column_width(field: &cms_field::Model) -> Option<u32> {
    match field.field_type.as_str() {
        "boolean" | "switch" => Some(80),
        "date" => Some(120),
        "datetime" => Some(180),
        "image" => Some(100),
        "integer" | "decimal" | "float" | "number" | "money" => Some(120),
        "status" => Some(100),
        _ => None,
    }
}

fn get_column_align(field: &cms_field::Model) -> String {
    match field.field_type.as_str() {
        "integer" | "decimal" | "float" | "number" | "money" | "percent" => "right".to_string(),
        "boolean" | "switch" | "status" => "center".to_string(),
        _ => "left".to_string(),
    }
}

fn build_search_field(field: &cms_field::Model) -> SearchField {
    SearchField {
        field: field.code.clone(),
        label: field.name.clone(),
        component: get_search_component(field),
        placeholder: Some(format!("请输入{}", field.name)),
        operator: get_search_operator(field),
        props: None,
    }
}

fn get_search_component(field: &cms_field::Model) -> String {
    match field.field_type.as_str() {
        "boolean" | "switch" => "select".to_string(),
        "date" => "date-picker".to_string(),
        "datetime" => "datetime-picker".to_string(),
        "select" | "enum" | "radio" => "select".to_string(),
        "integer" | "decimal" | "float" | "number" => "number-input".to_string(),
        _ => "input".to_string(),
    }
}

fn get_search_operator(field: &cms_field::Model) -> String {
    match field.field_type.as_str() {
        "string" | "text" | "textarea" | "longtext" => "like".to_string(),
        "select" | "enum" | "radio" | "boolean" | "switch" => "eq".to_string(),
        "date" | "datetime" => "between".to_string(),
        _ => "eq".to_string(),
    }
}

fn parse_table_config_props(table_config: &Option<String>) -> Option<JsonValue> {
    match table_config {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).ok()
        }
        _ => None,
    }
}
