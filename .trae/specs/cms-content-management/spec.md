# CMS内容管理系统模块 - 技术规格文档

## 1. 背景与目标

### 1.1 项目定位

构建一个**低代码CMS内容管理系统**，参考 Dcat Admin 和 FastAdmin 的设计理念，通过可视化配置实现内容模型的动态定义、管理界面的自动生成、CRUD代码的一键生成与下载。系统采用插件化架构设计，核心功能解耦合，支持付费插件扩展。

### 1.2 核心价值

- **低代码开发**：通过可视化配置替代传统编码，减少80%重复代码
- **插件化架构**：核心功能解耦，支持付费插件市场
- **代码生成器**：一键生成完整CRUD代码，支持下载和在线预览
- **动态模型**：运行时动态创建数据表和模型，无需重启服务

### 1.3 参考框架设计理念

#### Dcat Admin 核心特性借鉴
- 可视化代码生成器：根据数据表一键生成增删改查页面
- 数据表格构建工具：内置丰富的表格常用功能
- 数据表单构建工具：内置丰富的表单类型
- 插件市场：在线安装、更新、卸载插件

#### FastAdmin 核心特性借鉴
- 一键生成CRUD：自动生成控制器、模型、视图、JS、语言包、菜单
- 强大的插件扩展功能
- 完善的前端功能组件

---

## 2. 系统架构设计

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           前端展示层 (Vue 3 + Element Plus)                    │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐│
│  │  模型设计器   │ │  内容管理    │ │  代码生成器   │ │  分类/标签管理       ││
│  │ ModelDesigner│ │ ContentMgr  │ │ CodeGenerator│ │ CategoryTag         ││
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────────────┘│
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐│
│  │  字段配置器   │ │  表单构建器   │ │  表格构建器   │ │  预览/下载中心       ││
│  │ FieldConfig  │ │ FormBuilder  │ │ TableBuilder │ │ PreviewDownload     ││
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────────────┘│
├─────────────────────────────────────────────────────────────────────────────┤
│                            API 网关层 (Axum)                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │                    统一鉴权 / 限流 / 日志 / 动态路由                       ││
│  └─────────────────────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────────────────────┤
│                           业务服务层 (Application)                            │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ model_service  │ │ content_service│ │ code_gen_service│ │ category_svc │ │
│  │ 模型配置服务    │ │ 内容管理服务    │ │ 代码生成服务    │ │ 分类服务     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ field_service  │ │ form_service   │ │ table_service  │ │ tag_service  │ │
│  │ 字段配置服务    │ │ 表单配置服务    │ │ 表格配置服务    │ │ 标签服务     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐                  │
│  │ schema_service │ │ template_svc   │ │ plugin_service │                  │
│  │ 动态Schema服务  │ │ 模板渲染服务    │ │ 插件管理服务    │                  │
│  └────────────────┘ └────────────────┘ └────────────────┘                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                           领域模型层 (Domain)                                  │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ CmsModel       │ │ CmsContent     │ │ CmsCategory    │ │ CmsTag       │ │
│  │ 内容模型实体    │ │ 内容实体        │ │ 分类实体        │ │ 标签实体     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ CmsField       │ │ CmsFormConfig  │ │ CmsTableConfig │ │ CmsTemplate  │ │
│  │ 字段定义实体    │ │ 表单配置实体    │ │ 表格配置实体    │ │ 模板实体     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│                           基础设施层 (Infrastructure)                          │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ DynamicTable   │ │ Redis Cache    │ │ Template Engine│ │ Code Builder │ │
│  │ 动态表管理      │ │ 缓存服务        │ │ 模板引擎        │ │ 代码构建器   │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 核心数据流

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           CMS 核心业务流程                                    │
└─────────────────────────────────────────────────────────────────────────────┘

【流程1：模型定义与配置】
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│ 创建模型  │───▶│ 配置字段  │───▶│ 设计表单  │───▶│ 配置表格  │───▶│ 生成代码  │
│          │    │          │    │          │    │          │    │          │
│ 模型名称  │    │ 字段类型  │    │ 表单布局  │    │ 列定义    │    │ CRUD代码 │
│ 表名      │    │ 验证规则  │    │ 字段组件  │    │ 搜索配置  │    │ 菜单权限 │
│ 描述      │    │ 默认值    │    │ 联动规则  │    │ 按钮配置  │    │ 下载/预览│
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
      │                                                              │
      │                                                              ▼
      │              ┌──────────────────────────────────────────────────┐
      │              │              代码生成引擎                         │
      │              │  ┌─────────────┐  ┌─────────────┐               │
      │              │  │ Rust Entity │  │ Rust Service│               │
      │              │  │ Sea-ORM代码 │  │ 业务逻辑代码 │               │
      │              │  └─────────────┘  └─────────────┘               │
      │              │  ┌─────────────┐  ┌─────────────┐               │
      │              │  │ Vue 页面    │  │ API 路由    │               │
      │              │  │ TypeScript  │  │ Axum Handler│               │
      │              │  └─────────────┘  └─────────────┘               │
      │              └──────────────────────────────────────────────────┘
      │
      ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                          动态表创建                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  CREATE TABLE `cms_content_{model_id}` (                            │ │
│  │    `id` bigint PRIMARY KEY AUTO_INCREMENT,                          │ │
│  │    `{field1}` {type} COMMENT '{label}',                             │ │
│  │    `{field2}` {type} COMMENT '{label}',                             │ │
│  │    ...                                                              │ │
│  │    `created_at` datetime DEFAULT CURRENT_TIMESTAMP,                 │ │
│  │    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE,       │ │
│  │  ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;                           │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────┘

【流程2：内容发布流程】
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│ 选择模型  │───▶│ 填写表单  │───▶│ 内容审核  │───▶│ 发布内容  │───▶│ 内容展示  │
│          │    │          │    │          │    │          │    │          │
│ 模型列表  │    │ 动态表单  │    │ 审核流程  │    │ 定时发布  │    │ 前台渲染  │
│ 分类筛选  │    │ 字段验证  │    │ 多级审核  │    │ 立即发布  │    │ API输出  │
│ 快速创建  │    │ 富文本    │    │ 审核记录  │    │ 草稿保存  │    │ 模板渲染  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘

【流程3：分类与标签管理】
┌──────────────────────────────────────────────────────────────────────────┐
│                           分类树形结构                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  📁 内容分类                                                         │ │
│  │    ├─ 📁 新闻中心 (news)                                             │ │
│  │    │   ├─ 📄 公司新闻 (company-news)                                 │ │
│  │    │   └─ 📄 行业动态 (industry-news)                                │ │
│  │    ├─ 📁 产品展示 (products)                                         │ │
│  │    │   ├─ 📄 新品发布 (new-products)                                 │ │
│  │    │   └─ 📄 产品评测 (reviews)                                      │ │
│  │    └─ 📁 帮助中心 (help)                                             │ │
│  │        ├─ 📄 使用教程 (tutorials)                                    │ │
│  │        └─ 📄 常见问题 (faq)                                          │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
│                           标签云结构                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  🏷️ 热门标签: [热门] [推荐] [置顶] [精华] [原创] [转载]              │ │
│  │  🏷️ 内容标签: [技术] [产品] [设计] [运营] [市场] [管理]              │ │
│  │  🏷️ 状态标签: [草稿] [待审] [已发布] [已下线] [回收站]               │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 3. 功能模块详细设计

### 3.1 模型配置模块 (Model Configuration)

#### 3.1.1 模型实体设计

```rust
/// CMS内容模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_model")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,                    // 模型名称：文章、产品、案例
    pub code: String,                    // 模型编码：article, product, case
    pub table_name: String,              // 数据表名：cms_content_article
    pub description: Option<String>,     // 模型描述
    pub icon: Option<String>,            // 模型图标
    pub category_id: Option<i64>,        // 所属分类
    pub is_system: bool,                 // 是否系统模型
    pub is_enabled: bool,                // 是否启用
    pub sort: i32,                       // 排序
    pub config: ModelConfig,             // 模型配置JSON
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

/// 模型配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub enable_category: bool,           // 是否启用分类
    pub enable_tag: bool,                // 是否启用标签
    pub enable_comment: bool,            // 是否启用评论
    pub enable_audit: bool,              // 是否启用审核
    pub enable_version: bool,            // 是否启用版本控制
    pub enable_recycle: bool,            // 是否启用回收站
    pub list_page_size: u32,             // 列表每页数量
    pub default_sort: String,            // 默认排序字段
    pub seo_config: SeoConfig,           // SEO配置
}

/// SEO配置
#[derive(Debug, Serialize, Deserialize)]
pub struct SeoConfig {
    pub title_field: Option<String>,     // 标题字段
    pub keywords_field: Option<String>,  // 关键词字段
    pub description_field: Option<String>, // 描述字段
}
```

#### 3.1.2 字段定义实体

```rust
/// CMS字段定义
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_field")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,                   // 所属模型ID
    pub name: String,                    // 字段名称
    pub code: String,                    // 字段编码
    pub field_type: FieldType,           // 字段类型
    pub db_type: String,                 // 数据库类型：varchar(255), text, int
    pub default_value: Option<String>,   // 默认值
    pub is_required: bool,               // 是否必填
    pub is_unique: bool,                 // 是否唯一
    pub is_searchable: bool,             // 是否可搜索
    pub is_sortable: bool,               // 是否可排序
    pub is_filterable: bool,             // 是否可筛选
    pub is_list_show: bool,              // 列表是否显示
    pub is_form_show: bool,              // 表单是否显示
    pub is_detail_show: bool,            // 详情是否显示
    pub form_config: FormFieldConfig,    // 表单配置
    pub table_config: TableFieldConfig,  // 表格配置
    pub validation: Option<String>,      // 验证规则JSON
    pub sort: i32,                       // 排序
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

/// 字段类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    // 基础类型
    Text,           // 单行文本
    Textarea,       // 多行文本
    Editor,         // 富文本编辑器
    Number,         // 数字
    Decimal,        // 小数
    DateTime,       // 日期时间
    Date,           // 日期
    Time,           // 时间
    
    // 选择类型
    Select,         // 下拉选择
    Radio,          // 单选
    Checkbox,       // 多选
    Switch,         // 开关
    
    // 媒体类型
    Image,          // 单图
    Images,         // 多图
    File,           // 文件
    Video,          // 视频
    Audio,          // 音频
    
    // 关联类型
    Category,       // 分类关联
    Tag,            // 标签关联
    Model,          // 模型关联（关联其他内容）
    User,           // 用户关联
    
    // 特殊类型
    Json,           // JSON数据
    Location,       // 地理位置
    Color,          // 颜色选择器
    Icon,           // 图标选择器
    Code,           // 代码编辑器
    Markdown,       // Markdown编辑器
}

/// 表单字段配置
#[derive(Debug, Serialize, Deserialize)]
pub struct FormFieldConfig {
    pub component: String,               // 组件类型：input, select, editor...
    pub placeholder: Option<String>,     // 占位符
    pub help_text: Option<String>,       // 帮助文本
    pub prefix: Option<String>,          // 前缀
    pub suffix: Option<String>,          // 后缀
    pub min_value: Option<f64>,          // 最小值
    pub max_value: Option<f64>,          // 最大值
    pub min_length: Option<u32>,         // 最小长度
    pub max_length: Option<u32>,         // 最大长度
    pub pattern: Option<String>,         // 正则表达式
    pub options: Vec<SelectOption>,      // 选项列表（用于select/radio/checkbox）
    pub upload_config: Option<UploadConfig>, // 上传配置
    pub linkage: Option<LinkageConfig>,  // 联动配置
    pub visible_condition: Option<String>, // 显示条件
    pub disabled_condition: Option<String>, // 禁用条件
}

/// 表格字段配置
#[derive(Debug, Serialize, Deserialize)]
pub struct TableFieldConfig {
    pub width: Option<u32>,              // 列宽度
    pub align: String,                   // 对齐方式：left, center, right
    pub fixed: Option<String>,           // 固定列：left, right
    pub sortable: bool,                  // 是否可排序
    pub filterable: bool,                // 是否可筛选
    pub filter_type: FilterType,         // 筛选类型
    pub render_type: RenderType,         // 渲染类型
    pub render_config: Option<RenderConfig>, // 渲染配置
}

/// 渲染类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderType {
    Text,           // 纯文本
    Html,           // HTML
    Image,          // 图片
    Link,           // 链接
    Tag,            // 标签
    Badge,          // 徽章
    Progress,       // 进度条
    Switch,         // 开关
    Button,         // 按钮
    Template,       // 自定义模板
}

/// 筛选类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    None,           // 无筛选
    Input,          // 输入框筛选
    Select,         // 下拉选择筛选
    Date,           // 日期筛选
    DateRange,      // 日期范围筛选
    NumberRange,    // 数字范围筛选
}
```

### 3.2 表单构建模块 (Form Builder)

#### 3.2.1 表单配置实体

```rust
/// CMS表单配置
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_form_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,                   // 所属模型ID
    pub name: String,                    // 表单名称
    pub code: String,                    // 表单编码
    pub form_type: FormType,             // 表单类型
    pub layout: FormLayout,              // 表单布局
    pub groups: Vec<FormGroup>,          // 表单分组
    pub actions: Vec<FormAction>,        // 表单操作
    pub rules: Vec<FormRule>,            // 表单验证规则
    pub hooks: Vec<FormHook>,            // 表单钩子
    pub is_default: bool,                // 是否默认表单
    pub status: bool,                    // 状态
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

/// 表单类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormType {
    Create,         // 新增表单
    Edit,           // 编辑表单
    Detail,         // 详情表单
    Search,         // 搜索表单
    Filter,         // 筛选表单
    Step,           // 分步表单
    Modal,          // 弹窗表单
    Drawer,         // 抽屉表单
}

/// 表单布局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormLayout {
    pub layout_type: LayoutType,         // 布局类型
    pub columns: u32,                    // 列数
    pub label_width: String,             // 标签宽度
    pub label_position: String,          // 标签位置：left, right, top
    pub size: String,                    // 尺寸：large, default, small
}

/// 布局类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Horizontal,     // 水平布局
    Vertical,       // 垂直布局
    Inline,         // 行内布局
    Grid,           // 网格布局
    Tabs,           // 标签页布局
    Collapse,       // 折叠面板布局
}

/// 表单分组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormGroup {
    pub id: String,                      // 分组ID
    pub name: String,                    // 分组名称
    pub code: String,                    // 分组编码
    pub icon: Option<String>,            // 分组图标
    pub fields: Vec<String>,             // 字段列表（字段code）
    pub visible: bool,                   // 是否显示
    pub collapsed: bool,                 // 是否折叠
    pub sort: i32,                       // 排序
}

/// 表单操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormAction {
    pub code: String,                    // 操作编码
    pub name: String,                    // 操作名称
    pub action_type: ActionType,         // 操作类型
    pub icon: Option<String>,            // 图标
    pub confirm: Option<String>,         // 确认提示
    pub api: Option<String>,             // API地址
    pub visible_condition: Option<String>, // 显示条件
    pub disabled_condition: Option<String>, // 禁用条件
    pub sort: i32,                       // 排序
}

/// 操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Submit,         // 提交
    Reset,          // 重置
    Cancel,         // 取消
    Save,           // 保存
    SaveAndNew,     // 保存并新增
    SaveAndClose,   // 保存并关闭
    Custom,         // 自定义
}
```

### 3.3 表格构建模块 (Table Builder)

#### 3.3.1 表格配置实体

```rust
/// CMS表格配置
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_table_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,                   // 所属模型ID
    pub name: String,                    // 表格名称
    pub code: String,                    // 表格编码
    pub columns: Vec<TableColumn>,       // 列配置
    pub search: TableSearch,             // 搜索配置
    pub filter: TableFilter,             // 筛选配置
    pub actions: Vec<TableAction>,       // 行操作
    pub batch_actions: Vec<BatchAction>, // 批量操作
    pub toolbar: Vec<ToolbarAction>,     // 工具栏操作
    pub pagination: Pagination,          // 分页配置
    pub features: TableFeatures,         // 表格特性
    pub is_default: bool,                // 是否默认表格
    pub status: bool,                    // 状态
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

/// 表格列配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub field: String,                   // 字段code
    pub title: String,                   // 列标题
    pub width: Option<u32>,              // 列宽度
    pub min_width: Option<u32>,          // 最小宽度
    pub fixed: Option<String>,           // 固定列
    pub align: String,                   // 对齐方式
    pub sortable: bool,                  // 是否可排序
    pub sort_order: Option<String>,      // 排序方式：asc, desc
    pub resizable: bool,                 // 是否可调整宽度
    pub show_overflow_tooltip: bool,     // 溢出显示tooltip
    pub render: ColumnRender,            // 渲染配置
    pub visible: bool,                   // 是否显示
    pub sort: i32,                       // 排序
}

/// 列渲染配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnRender {
    pub render_type: RenderType,         // 渲染类型
    pub render_config: Option<serde_json::Value>, // 渲染配置
}

/// 表格搜索配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSearch {
    pub enabled: bool,                   // 是否启用搜索
    pub fields: Vec<SearchField>,        // 搜索字段
    pub placeholder: String,             // 占位符
    pub search_on_enter: bool,           // 回车搜索
}

/// 搜索字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchField {
    pub field: String,                   // 字段code
    pub search_type: SearchType,         // 搜索类型
    pub weight: u32,                     // 权重（用于排序）
}

/// 搜索类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Like,           // 模糊搜索
    Equal,          // 精确匹配
    Start,          // 前缀匹配
    End,            // 后缀匹配
    FullText,       // 全文搜索
}

/// 表格筛选配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFilter {
    pub enabled: bool,                   // 是否启用筛选
    pub fields: Vec<FilterField>,        // 筛选字段
    pub layout: String,                  // 筛选布局：inline, fold
    pub fold_count: u32,                 // 折叠数量
}

/// 筛选字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterField {
    pub field: String,                   // 字段code
    pub filter_type: FilterType,         // 筛选类型
    pub options: Option<Vec<SelectOption>>, // 选项
    pub default_value: Option<serde_json::Value>, // 默认值
}

/// 行操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableAction {
    pub code: String,                    // 操作编码
    pub name: String,                    // 操作名称
    pub action_type: RowActionType,      // 操作类型
    pub icon: Option<String>,            // 图标
    pub color: Option<String>,           // 颜色
    pub confirm: Option<String>,         // 确认提示
    pub api: Option<String>,             // API地址
    pub visible_condition: Option<String>, // 显示条件
    pub disabled_condition: Option<String>, // 禁用条件
    pub sort: i32,                       // 排序
}

/// 行操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RowActionType {
    Edit,           // 编辑
    Detail,         // 详情
    Delete,         // 删除
    Copy,           // 复制
    Export,         // 导出
    Custom,         // 自定义
}

/// 批量操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAction {
    pub code: String,                    // 操作编码
    pub name: String,                    // 操作名称
    pub icon: Option<String>,            // 图标
    pub confirm: Option<String>,         // 确认提示
    pub api: Option<String>,             // API地址
    pub visible_condition: Option<String>, // 显示条件
    pub disabled_condition: Option<String>, // 禁用条件
}

/// 工具栏操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarAction {
    pub code: String,                    // 操作编码
    pub name: String,                    // 操作名称
    pub icon: Option<String>,            // 图标
    pub action_type: ToolbarActionType,  // 操作类型
    pub api: Option<String>,             // API地址
    pub visible_condition: Option<String>, // 显示条件
}

/// 工具栏操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolbarActionType {
    Create,         // 新增
    Import,         // 导入
    Export,         // 导出
    Refresh,        // 刷新
    ColumnSetting,  // 列设置
    FullScreen,     // 全屏
    Custom,         // 自定义
}

/// 分页配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub enabled: bool,                   // 是否启用分页
    pub page_sizes: Vec<u32>,            // 每页数量选项
    pub default_page_size: u32,          // 默认每页数量
    pub layout: String,                  // 分页布局
    pub background: bool,                // 是否有背景
}

/// 表格特性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFeatures {
    pub selection: bool,                 // 是否启用选择
    pub index: bool,                     // 是否显示序号
    pub expand: bool,                    // 是否启用展开行
    pub tree: bool,                      // 是否启用树形
    pub stripe: bool,                    // 是否斑马纹
    pub border: bool,                    // 是否边框
    pub highlight_current_row: bool,     // 是否高亮当前行
    pub show_summary: bool,              // 是否显示合计
    pub sum_text: Option<String>,        // 合计文本
}
```

### 3.4 内容管理模块 (Content Management)

#### 3.4.1 内容实体设计

```rust
/// CMS内容主表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_content")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,                   // 模型ID
    pub category_id: Option<i64>,        // 分类ID
    pub title: String,                   // 标题
    pub slug: Option<String>,            // URL别名
    pub keywords: Option<String>,        // 关键词
    pub description: Option<String>,     // 描述
    pub author_id: Option<i64>,          // 作者ID
    pub source: Option<String>,          // 来源
    pub thumbnail: Option<String>,       // 缩略图
    pub images: Option<String>,          // 图片集（JSON）
    pub attachments: Option<String>,     // 附件（JSON）
    pub content_type: ContentType,       // 内容类型
    pub status: ContentStatus,           // 状态
    pub publish_time: Option<DateTime>,  // 发布时间
    pub expire_time: Option<DateTime>,   // 过期时间
    pub sort: i32,                       // 排序
    pub view_count: i64,                 // 浏览次数
    pub like_count: i64,                 // 点赞次数
    pub comment_count: i64,              // 评论次数
    pub is_top: bool,                    // 是否置顶
    pub is_recommend: bool,              // 是否推荐
    pub is_hot: bool,                    // 是否热门
    pub allow_comment: bool,             // 是否允许评论
    pub password: Option<String>,        // 访问密码
    pub template: Option<String>,        // 模板文件
    pub extra_data: Option<String>,      // 扩展数据（JSON）
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

/// 内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Article,        // 文章
    Product,        // 产品
    Case,           // 案例
    Download,       // 下载
    Video,          // 视频
    Image,          // 图集
    Custom,         // 自定义
}

/// 内容状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentStatus {
    Draft,          // 草稿
    Pending,        // 待审核
    Rejected,       // 已拒绝
    Published,      // 已发布
    Offline,        // 已下线
    Trash,          // 回收站
}

/// 内容标签关联
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cms_content_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub content_id: i64,                 // 内容ID
    pub tag_id: i64,                     // 标签ID
    pub created_at: Option<DateTime>,
}

/// 内容版本历史
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_content_version")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub content_id: i64,                 // 内容ID
    pub version: i32,                    // 版本号
    pub data: String,                    // 内容快照（JSON）
    pub change_log: Option<String>,      // 变更日志
    pub operator_id: i64,                // 操作人ID
    pub created_at: Option<DateTime>,
}
```

### 3.5 分类管理模块 (Category Management)

```rust
/// CMS分类
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub parent_id: Option<i64>,          // 父分类ID
    pub model_id: Option<i64>,           // 关联模型ID
    pub name: String,                    // 分类名称
    pub code: String,                    // 分类编码
    pub slug: Option<String>,            // URL别名
    pub icon: Option<String>,            // 图标
    pub image: Option<String>,           // 图片
    pub description: Option<String>,     // 描述
    pub keywords: Option<String>,        // 关键词
    pub template_list: Option<String>,   // 列表模板
    pub template_detail: Option<String>, // 详情模板
    pub page_size: u32,                  // 每页数量
    pub sort: i32,                       // 排序
    pub status: bool,                    // 状态
    pub seo_title: Option<String>,       // SEO标题
    pub seo_keywords: Option<String>,    // SEO关键词
    pub seo_description: Option<String>, // SEO描述
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}
```

### 3.6 标签管理模块 (Tag Management)

```rust
/// CMS标签
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,                    // 标签名称
    pub code: String,                    // 标签编码
    pub slug: Option<String>,            // URL别名
    pub color: Option<String>,           // 颜色
    pub icon: Option<String>,            // 图标
    pub description: Option<String>,     // 描述
    pub seo_title: Option<String>,       // SEO标题
    pub seo_keywords: Option<String>,    // SEO关键词
    pub seo_description: Option<String>, // SEO描述
    pub content_count: i64,              // 内容数量
    pub sort: i32,                       // 排序
    pub status: bool,                    // 状态
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}
```

### 3.7 代码生成器模块 (Code Generator)

#### 3.7.1 代码生成配置

```rust
/// 代码生成配置
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_code_gen")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,                   // 模型ID
    pub gen_type: GenType,               // 生成类型
    pub output_path: String,             // 输出路径
    pub template_config: TemplateConfig, // 模板配置
    pub file_config: FileConfig,         // 文件配置
    pub status: bool,                    // 状态
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

/// 生成类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenType {
    Download,       // 下载代码包
    Preview,        // 在线预览
    DirectApply,    // 直接应用
}

/// 模板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub entity_template: String,         // Entity模板
    pub model_template: String,          // Model模板
    pub args_template: String,           // Args模板
    pub service_template: String,        // Service模板
    pub api_template: String,            // API模板
    pub vue_list_template: String,       // Vue列表模板
    pub vue_form_template: String,       // Vue表单模板
    pub vue_detail_template: String,     // Vue详情模板
}

/// 文件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub entity_file: String,             // Entity文件名
    pub model_file: String,              // Model文件名
    pub args_file: String,               // Args文件名
    pub service_file: String,            // Service文件名
    pub api_file: String,                // API文件名
    pub vue_list_file: String,           // Vue列表文件名
    pub vue_form_file: String,           // Vue表单文件名
    pub vue_detail_file: String,         // Vue详情文件名
}
```

#### 3.7.2 代码生成流程

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           代码生成流程                                       │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│ 选择模型  │───▶│ 配置生成  │───▶│ 预览代码  │───▶│ 下载/应用 │───▶│ 完成生成  │
│          │    │          │    │          │    │          │    │          │
│ 模型列表  │    │ 生成类型  │    │ 文件列表  │    │ 下载ZIP  │    │ 菜单生成  │
│ 模型详情  │    │ 输出路径  │    │ 代码预览  │    │ 直接应用  │    │ 权限生成  │
│ 字段列表  │    │ 模板选择  │    │ 语法高亮  │    │ 覆盖确认  │    │ 路由注册  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
      │                                                              │
      │                                                              ▼
      │              ┌──────────────────────────────────────────────────┐
      │              │              生成的代码文件                       │
      │              │  ┌─────────────────────────────────────────────┐ │
      │              │  │ 后端代码 (Rust)                              │ │
      │              │  │  ├─ src/domain/entity/cms_content_{id}.rs   │ │
      │              │  │  ├─ src/domain/model/m_content_{id}.rs      │ │
      │              │  │  ├─ src/domain/args/a_content_{id}.rs       │ │
      │              │  │  ├─ src/application/cms/content_service.rs   │ │
      │              │  │  └─ src/api/cms/content_handler.rs          │ │
      │              │  └─────────────────────────────────────────────┘ │
      │              │  ┌─────────────────────────────────────────────┐ │
      │              │  │ 前端代码 (Vue 3 + TypeScript)                │ │
      │              │  │  ├─ src/views/cms/{model}/list.vue          │ │
      │              │  │  ├─ src/views/cms/{model}/form.vue          │ │
      │              │  │  ├─ src/views/cms/{model}/detail.vue        │ │
      │              │  │  ├─ src/api/cms/{model}.ts                  │ │
      │              │  │  └─ src/router/modules/cms/{model}.ts       │ │
      │              │  └─────────────────────────────────────────────┘ │
      │              │  ┌─────────────────────────────────────────────┐ │
      │              │  │ 数据库迁移 (SQL)                             │ │
      │              │  │  └─ migrations/m{date}_{model}_create.sql   │ │
      │              │  └─────────────────────────────────────────────┘ │
      │              └──────────────────────────────────────────────────┘
      │
      ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                          代码模板示例                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  // Entity模板                                                       │ │
│  │  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]              │ │
│  │  #[sea_orm(table_name = "{{table_name}}")]                          │ │
│  │  pub struct Model {                                                  │ │
│  │      {{#each fields}}                                                │ │
│  │      pub {{code}}: {{rust_type}},                                    │ │
│  │      {{/each}}                                                       │ │
│  │  }                                                                   │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 4. API 接口设计

### 4.1 模型管理 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/model/list` | GET | 获取模型列表 |
| `/cms/model/detail/:id` | GET | 获取模型详情 |
| `/cms/model/add` | POST | 创建模型 |
| `/cms/model/edit` | PUT | 编辑模型 |
| `/cms/model/delete` | DELETE | 删除模型 |
| `/cms/model/fields/:id` | GET | 获取模型字段列表 |
| `/cms/model/field/add` | POST | 添加字段 |
| `/cms/model/field/edit` | PUT | 编辑字段 |
| `/cms/model/field/delete` | DELETE | 删除字段 |
| `/cms/model/field/sort` | POST | 字段排序 |

### 4.2 表单配置 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/form/list` | GET | 获取表单配置列表 |
| `/cms/form/detail/:id` | GET | 获取表单配置详情 |
| `/cms/form/add` | POST | 创建表单配置 |
| `/cms/form/edit` | PUT | 编辑表单配置 |
| `/cms/form/delete` | DELETE | 删除表单配置 |
| `/cms/form/preview/:id` | GET | 预览表单 |

### 4.3 表格配置 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/table/list` | GET | 获取表格配置列表 |
| `/cms/table/detail/:id` | GET | 获取表格配置详情 |
| `/cms/table/add` | POST | 创建表格配置 |
| `/cms/table/edit` | PUT | 编辑表格配置 |
| `/cms/table/delete` | DELETE | 删除表格配置 |
| `/cms/table/preview/:id` | GET | 预览表格 |

### 4.4 内容管理 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/content/list` | GET | 获取内容列表 |
| `/cms/content/detail/:id` | GET | 获取内容详情 |
| `/cms/content/add` | POST | 创建内容 |
| `/cms/content/edit` | PUT | 编辑内容 |
| `/cms/content/delete` | DELETE | 删除内容 |
| `/cms/content/publish` | POST | 发布内容 |
| `/cms/content/offline` | POST | 下线内容 |
| `/cms/content/move-trash` | POST | 移入回收站 |
| `/cms/content/restore` | POST | 恢复内容 |
| `/cms/content/audit` | POST | 审核内容 |
| `/cms/content/version/:id` | GET | 获取版本历史 |
| `/cms/content/version/rollback` | POST | 版本回滚 |

### 4.5 分类管理 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/category/list` | GET | 获取分类列表 |
| `/cms/category/tree` | GET | 获取分类树 |
| `/cms/category/detail/:id` | GET | 获取分类详情 |
| `/cms/category/add` | POST | 创建分类 |
| `/cms/category/edit` | PUT | 编辑分类 |
| `/cms/category/delete` | DELETE | 删除分类 |
| `/cms/category/sort` | POST | 分类排序 |

### 4.6 标签管理 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/tag/list` | GET | 获取标签列表 |
| `/cms/tag/detail/:id` | GET | 获取标签详情 |
| `/cms/tag/add` | POST | 创建标签 |
| `/cms/tag/edit` | PUT | 编辑标签 |
| `/cms/tag/delete` | DELETE | 删除标签 |
| `/cms/tag/batch-add` | POST | 批量添加标签 |

### 4.7 代码生成 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/cms/code-gen/preview/:modelId` | GET | 预览生成代码 |
| `/cms/code-gen/download/:modelId` | GET | 下载代码包 |
| `/cms/code-gen/apply/:modelId` | POST | 应用代码 |
| `/cms/code-gen/history/:modelId` | GET | 生成历史 |

---

## 5. 前端页面设计

### 5.1 页面结构

```
├── cms/
│   ├── model/                        # 模型管理
│   │   ├── list.vue                  # 模型列表
│   │   ├── design.vue                # 模型设计器
│   │   ├── field-config.vue          # 字段配置
│   │   └── components/
│   │       ├── field-editor.vue      # 字段编辑器
│   │       ├── field-type-select.vue # 字段类型选择
│   │       └── validation-rules.vue  # 验证规则配置
│   │
│   ├── form/                         # 表单配置
│   │   ├── list.vue                  # 表单列表
│   │   ├── builder.vue               # 表单构建器
│   │   └── components/
│   │       ├── form-preview.vue      # 表单预览
│   │       ├── field-layout.vue      # 字段布局
│   │       └── form-rules.vue        # 表单规则
│   │
│   ├── table/                        # 表格配置
│   │   ├── list.vue                  # 表格列表
│   │   ├── builder.vue               # 表格构建器
│   │   └── components/
│   │       ├── column-config.vue     # 列配置
│   │       ├── search-config.vue     # 搜索配置
│   │       └── action-config.vue     # 操作配置
│   │
│   ├── content/                      # 内容管理
│   │   ├── list.vue                  # 内容列表（动态）
│   │   ├── form.vue                  # 内容表单（动态）
│   │   ├── detail.vue                # 内容详情（动态）
│   │   ├── recycle.vue               # 回收站
│   │   └── components/
│   │       ├── content-filter.vue    # 内容筛选
│   │       ├── content-editor.vue    # 内容编辑器
│   │       └── version-history.vue   # 版本历史
│   │
│   ├── category/                     # 分类管理
│   │   ├── list.vue                  # 分类列表
│   │   └── tree.vue                  # 分类树
│   │
│   ├── tag/                          # 标签管理
│   │   ├── list.vue                  # 标签列表
│   │   └── cloud.vue                 # 标签云
│   │
│   └── code-gen/                     # 代码生成
│       ├── index.vue                 # 代码生成首页
│       ├── preview.vue               # 代码预览
│       └── components/
│           ├── file-tree.vue         # 文件树
│           ├── code-viewer.vue       # 代码查看器
│           └── gen-config.vue        # 生成配置
```

### 5.2 模型设计器页面设计

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  模型设计器                                              [保存] [预览] [生成]│
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐  ┌───────────────────────────────────────────────┐│
│  │ 基本信息             │  │ 字段列表                                       ││
│  │ ┌─────────────────┐ │  │ ┌───────────────────────────────────────────┐ ││
│  │ │ 模型名称: [    ]│ │  │ │ [+添加字段] [批量导入] [模板字段]          │ ││
│  │ │ 模型编码: [    ]│ │  │ └───────────────────────────────────────────┘ ││
│  │ │ 数据表名: [    ]│ │  │ ┌───────────────────────────────────────────┐ ││
│  │ │ 模型图标: [    ]│ │  │ │ □ id        主键ID    bigint    [编辑][删除]│ ││
│  │ │ 模型描述: [    ]│ │  │ │ □ title     标题      varchar   [编辑][删除]│ ││
│  │ │ 所属分类: [    ]│ │  │ │ □ content   内容      text      [编辑][删除]│ ││
│  │ └─────────────────┘ │  │ │ □ author    作者      varchar   [编辑][删除]│ ││
│  │                     │  │ │ □ status    状态      tinyint   [编辑][删除]│ ││
│  │ 模型配置             │  │ │ ...                                       │ ││
│  │ ┌─────────────────┐ │  │ └───────────────────────────────────────────┘ ││
│  │ │ ☑ 启用分类      │ │  │                                                 ││
│  │ │ ☑ 启用标签      │ │  │ 拖拽排序                                        ││
│  │ │ ☐ 启用评论      │ │  │                                                 ││
│  │ │ ☐ 启用审核      │ │  └───────────────────────────────────────────────┘│
│  │ │ ☑ 启用版本控制   │ │                                                    │
│  │ │ ☑ 启用回收站    │ │                                                    │
│  │ │ 列表数量: [20]  │ │                                                    │
│  │ │ 默认排序: [    ]│ │                                                    │
│  │ └─────────────────┘ │                                                    │
│  └─────────────────────┘                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.3 字段配置弹窗设计

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  字段配置                                                           [×]     │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │ [基本信息] [表单配置] [表格配置] [验证规则] [联动规则]                    ││
│  └─────────────────────────────────────────────────────────────────────────┘│
│                                                                             │
│  基本信息                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 字段名称: [标题            ]  字段编码: [title         ]            │   │
│  │ 字段类型: [单行文本       ▼]  数据类型: [varchar(255)  ▼]           │   │
│  │ 默认值:   [                ]  帮助文本: [请输入标题     ]            │   │
│  │                                                                     │   │
│  │ ☑ 必填    ☐ 唯一    ☑ 可搜索    ☐ 可排序    ☑ 可筛选               │   │
│  │ ☑ 列表显示  ☑ 表单显示  ☑ 详情显示                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  表单配置                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 组件类型: [输入框         ▼]  占位符: [请输入标题     ]             │   │
│  │ 前缀: [    ]  后缀: [    ]  最大长度: [255  ]                       │   │
│  │                                                                     │   │
│  │ 显示条件: [                                                            ]│
│  │ 禁用条件: [                                                            ]│
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│                                              [取消]  [确定]                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.4 代码预览页面设计

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  代码预览 - 文章模型                                   [下载代码] [应用代码] │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌───────────────────┐  ┌─────────────────────────────────────────────────┐│
│  │ 文件列表           │  │ 代码内容                                         ││
│  │ ┌───────────────┐ │  │ ┌─────────────────────────────────────────────┐ ││
│  │ │ 📁 后端代码    │ │  │ │ 文件: entity/cms_content_article.rs        │ ││
│  │ │  ├─ 📄 entity │ │  │ │ ─────────────────────────────────────────── │ ││
│  │ │  ├─ 📄 model  │ │  │ │  1 │ #[derive(Clone, Debug, PartialEq)]     │ ││
│  │ │  ├─ 📄 args   │ │  │ │  2 │ #[sea_orm(table_name = "cms_article")]  │ ││
│  │ │  ├─ 📄 service│ │  │ │  3 │ pub struct Model {                      │ ││
│  │ │  └─ 📄 api    │ │  │ │  4 │     #[sea_orm(primary_key)]              │ ││
│  │ │ 📁 前端代码    │ │  │ │  5 │     pub id: i64,                        │ ││
│  │ │  ├─ 📄 list   │ │  │ │  6 │     pub title: String,                  │ ││
│  │ │  ├─ 📄 form   │ │  │ │  7 │     pub content: String,                │ ││
│  │ │  ├─ 📄 detail │ │  │ │  8 │     pub author: Option<String>,         │ ││
│  │ │  └─ 📄 api    │ │  │ │  9 │     pub status: i32,                    │ ││
│  │ │ 📁 数据库      │ │  │ │ 10 │     pub created_at: Option<DateTime>,   │ ││
│  │ │  └─ 📄 migrate│ │  │ │ 11 │ }                                       │ ││
│  │ └───────────────┘ │  │ └─────────────────────────────────────────────┘ ││
│  │                     │  │                                                 ││
│  │                     │  │ [复制代码] [全选] [语法: Rust ▼]               ││
│  └───────────────────┘  └─────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 数据库设计

### 6.1 核心表结构

```sql
-- CMS模型表
CREATE TABLE `cms_model` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `name` varchar(50) NOT NULL COMMENT '模型名称',
  `code` varchar(50) NOT NULL COMMENT '模型编码',
  `table_name` varchar(100) NOT NULL COMMENT '数据表名',
  `description` varchar(500) DEFAULT NULL COMMENT '模型描述',
  `icon` varchar(100) DEFAULT NULL COMMENT '模型图标',
  `category_id` bigint(20) DEFAULT NULL COMMENT '所属分类',
  `is_system` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否系统模型',
  `is_enabled` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否启用',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `config` json DEFAULT NULL COMMENT '模型配置',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_code` (`code`),
  UNIQUE KEY `uk_table_name` (`table_name`),
  KEY `idx_category` (`category_id`),
  KEY `idx_enabled` (`is_enabled`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS模型表';

-- CMS字段表
CREATE TABLE `cms_field` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `model_id` bigint(20) NOT NULL COMMENT '所属模型ID',
  `name` varchar(50) NOT NULL COMMENT '字段名称',
  `code` varchar(50) NOT NULL COMMENT '字段编码',
  `field_type` varchar(30) NOT NULL COMMENT '字段类型',
  `db_type` varchar(50) NOT NULL COMMENT '数据库类型',
  `default_value` varchar(500) DEFAULT NULL COMMENT '默认值',
  `is_required` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否必填',
  `is_unique` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否唯一',
  `is_searchable` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否可搜索',
  `is_sortable` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否可排序',
  `is_filterable` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否可筛选',
  `is_list_show` tinyint(1) NOT NULL DEFAULT '1' COMMENT '列表是否显示',
  `is_form_show` tinyint(1) NOT NULL DEFAULT '1' COMMENT '表单是否显示',
  `is_detail_show` tinyint(1) NOT NULL DEFAULT '1' COMMENT '详情是否显示',
  `form_config` json DEFAULT NULL COMMENT '表单配置',
  `table_config` json DEFAULT NULL COMMENT '表格配置',
  `validation` json DEFAULT NULL COMMENT '验证规则',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_model_code` (`model_id`, `code`),
  KEY `idx_model` (`model_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS字段表';

-- CMS表单配置表
CREATE TABLE `cms_form_config` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `model_id` bigint(20) NOT NULL COMMENT '所属模型ID',
  `name` varchar(50) NOT NULL COMMENT '表单名称',
  `code` varchar(50) NOT NULL COMMENT '表单编码',
  `form_type` varchar(20) NOT NULL COMMENT '表单类型',
  `layout` json DEFAULT NULL COMMENT '表单布局',
  `groups` json DEFAULT NULL COMMENT '表单分组',
  `actions` json DEFAULT NULL COMMENT '表单操作',
  `rules` json DEFAULT NULL COMMENT '表单验证规则',
  `hooks` json DEFAULT NULL COMMENT '表单钩子',
  `is_default` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否默认表单',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_model_code` (`model_id`, `code`),
  KEY `idx_model` (`model_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS表单配置表';

-- CMS表格配置表
CREATE TABLE `cms_table_config` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `model_id` bigint(20) NOT NULL COMMENT '所属模型ID',
  `name` varchar(50) NOT NULL COMMENT '表格名称',
  `code` varchar(50) NOT NULL COMMENT '表格编码',
  `columns` json DEFAULT NULL COMMENT '列配置',
  `search` json DEFAULT NULL COMMENT '搜索配置',
  `filter` json DEFAULT NULL COMMENT '筛选配置',
  `actions` json DEFAULT NULL COMMENT '行操作',
  `batch_actions` json DEFAULT NULL COMMENT '批量操作',
  `toolbar` json DEFAULT NULL COMMENT '工具栏操作',
  `pagination` json DEFAULT NULL COMMENT '分页配置',
  `features` json DEFAULT NULL COMMENT '表格特性',
  `is_default` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否默认表格',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_model_code` (`model_id`, `code`),
  KEY `idx_model` (`model_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS表格配置表';

-- CMS内容主表
CREATE TABLE `cms_content` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `model_id` bigint(20) NOT NULL COMMENT '模型ID',
  `category_id` bigint(20) DEFAULT NULL COMMENT '分类ID',
  `title` varchar(200) NOT NULL COMMENT '标题',
  `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
  `keywords` varchar(500) DEFAULT NULL COMMENT '关键词',
  `description` varchar(1000) DEFAULT NULL COMMENT '描述',
  `author_id` bigint(20) DEFAULT NULL COMMENT '作者ID',
  `source` varchar(100) DEFAULT NULL COMMENT '来源',
  `thumbnail` varchar(500) DEFAULT NULL COMMENT '缩略图',
  `images` json DEFAULT NULL COMMENT '图片集',
  `attachments` json DEFAULT NULL COMMENT '附件',
  `content_type` varchar(20) NOT NULL DEFAULT 'article' COMMENT '内容类型',
  `status` tinyint(1) NOT NULL DEFAULT '0' COMMENT '状态：0草稿1待审2已拒3已发布4已下线5回收站',
  `publish_time` datetime DEFAULT NULL COMMENT '发布时间',
  `expire_time` datetime DEFAULT NULL COMMENT '过期时间',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `view_count` bigint(20) NOT NULL DEFAULT '0' COMMENT '浏览次数',
  `like_count` bigint(20) NOT NULL DEFAULT '0' COMMENT '点赞次数',
  `comment_count` bigint(20) NOT NULL DEFAULT '0' COMMENT '评论次数',
  `is_top` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否置顶',
  `is_recommend` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否推荐',
  `is_hot` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否热门',
  `allow_comment` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否允许评论',
  `password` varchar(100) DEFAULT NULL COMMENT '访问密码',
  `template` varchar(100) DEFAULT NULL COMMENT '模板文件',
  `extra_data` json DEFAULT NULL COMMENT '扩展数据',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (`id`),
  KEY `idx_model` (`model_id`),
  KEY `idx_category` (`category_id`),
  KEY `idx_author` (`author_id`),
  KEY `idx_status` (`status`),
  KEY `idx_publish_time` (`publish_time`),
  KEY `idx_slug` (`slug`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容主表';

-- CMS分类表
CREATE TABLE `cms_category` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `parent_id` bigint(20) DEFAULT NULL COMMENT '父分类ID',
  `model_id` bigint(20) DEFAULT NULL COMMENT '关联模型ID',
  `name` varchar(50) NOT NULL COMMENT '分类名称',
  `code` varchar(50) NOT NULL COMMENT '分类编码',
  `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
  `icon` varchar(100) DEFAULT NULL COMMENT '图标',
  `image` varchar(500) DEFAULT NULL COMMENT '图片',
  `description` varchar(500) DEFAULT NULL COMMENT '描述',
  `keywords` varchar(500) DEFAULT NULL COMMENT '关键词',
  `template_list` varchar(100) DEFAULT NULL COMMENT '列表模板',
  `template_detail` varchar(100) DEFAULT NULL COMMENT '详情模板',
  `page_size` int(11) NOT NULL DEFAULT '20' COMMENT '每页数量',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `seo_title` varchar(200) DEFAULT NULL COMMENT 'SEO标题',
  `seo_keywords` varchar(500) DEFAULT NULL COMMENT 'SEO关键词',
  `seo_description` varchar(1000) DEFAULT NULL COMMENT 'SEO描述',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_code` (`code`),
  KEY `idx_parent` (`parent_id`),
  KEY `idx_model` (`model_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS分类表';

-- CMS标签表
CREATE TABLE `cms_tag` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `name` varchar(50) NOT NULL COMMENT '标签名称',
  `code` varchar(50) NOT NULL COMMENT '标签编码',
  `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
  `color` varchar(20) DEFAULT NULL COMMENT '颜色',
  `icon` varchar(100) DEFAULT NULL COMMENT '图标',
  `description` varchar(500) DEFAULT NULL COMMENT '描述',
  `seo_title` varchar(200) DEFAULT NULL COMMENT 'SEO标题',
  `seo_keywords` varchar(500) DEFAULT NULL COMMENT 'SEO关键词',
  `seo_description` varchar(1000) DEFAULT NULL COMMENT 'SEO描述',
  `content_count` bigint(20) NOT NULL DEFAULT '0' COMMENT '内容数量',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_code` (`code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS标签表';

-- CMS内容标签关联表
CREATE TABLE `cms_content_tag` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `content_id` bigint(20) NOT NULL COMMENT '内容ID',
  `tag_id` bigint(20) NOT NULL COMMENT '标签ID',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_content_tag` (`content_id`, `tag_id`),
  KEY `idx_content` (`content_id`),
  KEY `idx_tag` (`tag_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容标签关联表';

-- CMS内容版本表
CREATE TABLE `cms_content_version` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `content_id` bigint(20) NOT NULL COMMENT '内容ID',
  `version` int(11) NOT NULL COMMENT '版本号',
  `data` json NOT NULL COMMENT '内容快照',
  `change_log` varchar(500) DEFAULT NULL COMMENT '变更日志',
  `operator_id` bigint(20) NOT NULL COMMENT '操作人ID',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_content` (`content_id`),
  KEY `idx_version` (`content_id`, `version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容版本表';

-- CMS代码生成配置表
CREATE TABLE `cms_code_gen` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `model_id` bigint(20) NOT NULL COMMENT '模型ID',
  `gen_type` varchar(20) NOT NULL DEFAULT 'download' COMMENT '生成类型',
  `output_path` varchar(500) NOT NULL COMMENT '输出路径',
  `template_config` json DEFAULT NULL COMMENT '模板配置',
  `file_config` json DEFAULT NULL COMMENT '文件配置',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_model` (`model_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS代码生成配置表';
```

---

## 7. 插件化架构设计

### 7.1 插件架构图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           CMS 插件化架构                                     │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                           插件管理层                                         │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ PluginManager  │ │ PluginLoader   │ │ PluginRegistry │ │ PluginMarket │ │
│  │ 插件管理器      │ │ 插件加载器      │ │ 插件注册中心    │ │ 插件市场     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│                           核心插件层                                         │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ FieldTypePlugin│ │ WidgetPlugin   │ │ TemplatePlugin │ │ ExportPlugin │ │
│  │ 字段类型插件    │ │ 组件插件        │ │ 模板插件        │ │ 导出插件     │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│                           扩展插件层                                         │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ SEOPlugin      │ │ AuditPlugin    │ │ WorkflowPlugin │ │ AIPlugin     │ │
│  │ SEO优化插件     │ │ 审核流程插件    │ │ 工作流插件      │ │ AI助手插件   │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│                           付费插件层                                         │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ┌──────────────┐ │
│  │ AdvancedForm   │ │ DataAnalysis   │ │ MultiLanguage  │ │ WechatSync   │ │
│  │ 高级表单插件    │ │ 数据分析插件    │ │ 多语言插件      │ │ 微信同步插件  │ │
│  └────────────────┘ └────────────────┘ └────────────────┘ └──────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘

插件接口定义：

```rust
/// CMS插件接口
pub trait CmsPlugin: Send + Sync {
    /// 插件ID
    fn id(&self) -> &str;
    
    /// 插件名称
    fn name(&self) -> &str;
    
    /// 插件版本
    fn version(&self) -> &str;
    
    /// 插件类型
    fn plugin_type(&self) -> PluginType;
    
    /// 是否付费
    fn is_paid(&self) -> bool;
    
    /// 插件价格
    fn price(&self) -> Option<Decimal>;
    
    /// 初始化插件
    fn init(&self, context: &PluginContext) -> Result<()>;
    
    /// 启用插件
    fn enable(&self) -> Result<()>;
    
    /// 禁用插件
    fn disable(&self) -> Result<()>;
    
    /// 卸载插件
    fn uninstall(&self) -> Result<()>;
}

/// 字段类型插件接口
pub trait FieldTypePlugin: CmsPlugin {
    /// 字段类型
    fn field_type(&self) -> FieldType;
    
    /// 渲染表单组件
    fn render_form(&self, config: &FormFieldConfig) -> String;
    
    /// 渲染表格列
    fn render_table(&self, config: &TableFieldConfig) -> String;
    
    /// 数据验证
    fn validate(&self, value: &str, rules: &ValidationRules) -> Result<()>;
    
    /// 数据转换（数据库 -> 前端）
    fn to_frontend(&self, value: &str) -> serde_json::Value;
    
    /// 数据转换（前端 -> 数据库）
    fn to_database(&self, value: &serde_json::Value) -> String;
}

/// 组件插件接口
pub trait WidgetPlugin: CmsPlugin {
    /// 组件名称
    fn widget_name(&self) -> &str;
    
    /// 渲染组件
    fn render(&self, props: &serde_json::Value) -> String;
    
    /// 组件配置表单
    fn config_form(&self) -> FormConfig;
}

/// 模板插件接口
pub trait TemplatePlugin: CmsPlugin {
    /// 模板名称
    fn template_name(&self) -> &str;
    
    /// 渲染内容
    fn render_content(&self, content: &Content, context: &RenderContext) -> String;
    
    /// 渲染列表
    fn render_list(&self, contents: &[Content], context: &RenderContext) -> String;
}
```

### 7.2 插件市场集成

CMS插件市场与已有的插件市场系统（plugin-market）集成：

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           插件市场集成                                       │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────────┐         ┌──────────────────┐         ┌──────────────────┐
│   CMS 插件市场    │         │   插件市场系统    │         │   License 验证   │
│                  │         │  (plugin-market) │         │                  │
│  ┌────────────┐  │         │  ┌────────────┐  │         │  ┌────────────┐  │
│  │ 字段类型    │  │         │  │ 插件列表    │  │         │  │ License    │  │
│  │ 组件插件    │  │ ◄─────▶ │  │ 订阅管理    │  │ ◄─────▶ │  │ 设备绑定   │  │
│  │ 模板插件    │  │         │  │ 支付流程    │  │         │  │ 验证码     │  │
│  │ 扩展插件    │  │         │  │ 开发者中心  │  │         │  │ 心跳检测   │  │
│  └────────────┘  │         │  └────────────┘  │         │  └────────────┘  │
└──────────────────┘         └──────────────────┘         └──────────────────┘

集成流程：
1. CMS插件发布到插件市场
2. 商户在CMS后台浏览/购买插件
3. 购买后获取License
4. CMS后台安装插件时验证License
5. 插件启用后注册到CMS插件系统
```

---

## 8. 影响范围

### 8.1 新增模块

- **后端**：cms模块（model、field、form、table、content、category、tag、codegen）
- **前端**：cms模块（model-designer、form-builder、table-builder、content-mgr、code-gen）
- **数据库**：8张核心表 + 动态内容表
- **API**：40+ 个新接口

### 8.2 系统改造

- 菜单权限体系扩展
- 动态路由注册
- 动态表管理
- 代码生成引擎
- 插件系统集成

---

## 9. 技术实现要点

### 9.1 动态表管理

```rust
/// 动态表管理服务
pub struct DynamicTableService;

impl DynamicTableService {
    /// 创建动态表
    pub async fn create_table(&self, model: &CmsModel, fields: &[CmsField]) -> Result<()> {
        let table_name = &model.table_name;
        let mut columns = vec![
            "`id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID'",
        ];
        
        for field in fields {
            columns.push(format!(
                "`{}` {} COMMENT '{}'",
                field.code, field.db_type, field.name
            ));
        }
        
        columns.extend(vec![
            "`created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'",
            "`updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'",
            "`deleted_at` datetime DEFAULT NULL COMMENT '删除时间'",
            "PRIMARY KEY (`id`)",
        ]);
        
        let sql = format!(
            "CREATE TABLE `{}` ({}) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='{}'",
            table_name,
            columns.join(",\n    "),
            model.name
        );
        
        // 执行SQL
        self.execute_sql(&sql).await?;
        Ok(())
    }
    
    /// 添加字段
    pub async fn add_column(&self, table_name: &str, field: &CmsField) -> Result<()> {
        let sql = format!(
            "ALTER TABLE `{}` ADD COLUMN `{}` {} COMMENT '{}'",
            table_name, field.code, field.db_type, field.name
        );
        self.execute_sql(&sql).await?;
        Ok(())
    }
    
    /// 删除字段
    pub async fn drop_column(&self, table_name: &str, field_code: &str) -> Result<()> {
        let sql = format!(
            "ALTER TABLE `{}` DROP COLUMN `{}`",
            table_name, field_code
        );
        self.execute_sql(&sql).await?;
        Ok(())
    }
}
```

### 9.2 代码生成器

```rust
/// 代码生成器
pub struct CodeGenerator;

impl CodeGenerator {
    /// 生成Entity代码
    pub fn generate_entity(&self, model: &CmsModel, fields: &[CmsField]) -> String {
        let template = r#"
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "{{table_name}}")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    {{#each fields}}
    {{#if nullable}}
    #[sea_orm(nullable)]
    {{/if}}
    pub {{code}}: {{rust_type}},
    {{/each}}
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}
"#;
        
        let mut tera = Tera::default();
        tera.add_raw_template("entity", template).unwrap();
        
        let mut context = Context::new();
        context.insert("table_name", &model.table_name);
        context.insert("fields", &fields);
        
        tera.render("entity", &context).unwrap()
    }
    
    /// 生成Service代码
    pub fn generate_service(&self, model: &CmsModel, fields: &[CmsField]) -> String {
        // 生成CRUD服务代码
        // ...
    }
    
    /// 生成Vue页面代码
    pub fn generate_vue_pages(&self, model: &CmsModel, form_config: &FormConfig, table_config: &TableConfig) -> HashMap<String, String> {
        let mut pages = HashMap::new();
        
        // 生成列表页
        pages.insert("list.vue".to_string(), self.generate_list_page(model, table_config));
        
        // 生成表单页
        pages.insert("form.vue".to_string(), self.generate_form_page(model, form_config));
        
        // 生成详情页
        pages.insert("detail.vue".to_string(), self.generate_detail_page(model, form_config));
        
        pages
    }
    
    /// 打包下载
    pub async fn pack_download(&self, model_id: i64) -> Result<Vec<u8>> {
        let model = self.get_model(model_id).await?;
        let fields = self.get_fields(model_id).await?;
        let form_config = self.get_form_config(model_id).await?;
        let table_config = self.get_table_config(model_id).await?;
        
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        
        // 添加后端代码
        zip.start_file("backend/entity.rs", FileOptions::default())?;
        zip.write_all(self.generate_entity(&model, &fields).as_bytes())?;
        
        zip.start_file("backend/service.rs", FileOptions::default())?;
        zip.write_all(self.generate_service(&model, &fields).as_bytes())?;
        
        // 添加前端代码
        let vue_pages = self.generate_vue_pages(&model, &form_config, &table_config);
        for (name, content) in vue_pages {
            zip.start_file(format!("frontend/{}", name), FileOptions::default())?;
            zip.write_all(content.as_bytes())?;
        }
        
        // 添加数据库迁移
        zip.start_file("migration/create_table.sql", FileOptions::default())?;
        zip.write_all(self.generate_migration(&model, &fields).as_bytes())?;
        
        let result = zip.finish()?;
        Ok(result.into_inner())
    }
}
```

---

## 10. 项目规划

整体项目分为四个阶段：

**第一阶段（核心基础）**：模型配置、字段定义、动态表管理
**第二阶段（构建器）**：表单构建器、表格构建器
**第三阶段（内容管理）**：内容发布、分类标签、版本控制
**第四阶段（代码生成）**：代码生成器、预览下载、插件集成
