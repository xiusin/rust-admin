# CMS内容管理系统模块 - 任务清单

## 概述

本任务清单详细规划了CMS内容管理系统模块的开发工作，按照模块化方式组织，任务之间标注了依赖关系。

---

## 阶段一：数据库层与核心实体

### 1.1 数据库迁移

- [x] **Task 1.1.1**: 创建CMS核心表迁移文件
  - 创建 `cms_model` 模型表
  - 创建 `cms_field` 字段表
  - 创建 `cms_category` 分类表
  - 创建 `cms_tag` 标签表
  - 创建索引和约束

- [x] **Task 1.1.2**: 创建CMS内容相关迁移文件
  - 创建 `cms_content` 内容主表
  - 创建 `cms_content_tag` 内容标签关联表
  - 创建 `cms_content_version` 内容版本表
  - 创建索引和约束

- [x] **Task 1.1.3**: 创建CMS配置相关迁移文件
  - 创建 `cms_form_config` 表单配置表
  - 创建 `cms_table_config` 表格配置表
  - 创建 `cms_code_gen` 代码生成配置表
  - 创建索引和约束

### 1.2 Domain 实体层

- [x] **Task 1.2.1**: 创建CMS模型相关实体
  - `cms_model.rs` - 模型实体
  - `cms_field.rs` - 字段实体
  - 实现关联关系

- [x] **Task 1.2.2**: 创建CMS内容相关实体
  - `cms_content.rs` - 内容实体
  - `cms_content_tag.rs` - 内容标签关联实体
  - `cms_content_version.rs` - 内容版本实体

- [x] **Task 1.2.3**: 创建CMS分类标签实体
  - `cms_category.rs` - 分类实体
  - `cms_tag.rs` - 标签实体

- [x] **Task 1.2.4**: 创建CMS配置相关实体
  - `cms_form_config.rs` - 表单配置实体
  - `cms_table_config.rs` - 表格配置实体
  - `cms_code_gen.rs` - 代码生成配置实体

### 1.3 Domain 模型层 (Model)

- [x] **Task 1.3.1**: 创建CMS模型相关模型
  - `m_cms_model.rs` - 模型列表项、详情模型
  - `m_cms_field.rs` - 字段模型

- [x] **Task 1.3.2**: 创建CMS内容相关模型
  - `m_cms_content.rs` - 内容模型
  - `m_cms_version.rs` - 版本模型

- [x] **Task 1.3.3**: 创建CMS分类标签模型
  - `m_cms_category.rs` - 分类模型（含树形结构）
  - `m_cms_tag.rs` - 标签模型

- [x] **Task 1.3.4**: 创建CMS配置相关模型
  - `m_form_config.rs` - 表单配置模型
  - `m_table_config.rs` - 表格配置模型
  - `m_code_gen.rs` - 代码生成模型

### 1.4 Domain 参数层 (Args)

- [x] **Task 1.4.1**: 创建CMS模型参数结构
  - `a_cms_model.rs` - 模型增删改查参数
  - `a_cms_field.rs` - 字段增删改查参数

- [x] **Task 1.4.2**: 创建CMS内容参数结构
  - `a_cms_content.rs` - 内容增删改查参数
  - `a_cms_version.rs` - 版本操作参数

- [x] **Task 1.4.3**: 创建CMS分类标签参数结构
  - `a_cms_category.rs` - 分类参数
  - `a_cms_tag.rs` - 标签参数

- [x] **Task 1.4.4**: 创建CMS配置参数结构
  - `a_form_config.rs` - 表单配置参数
  - `a_table_config.rs` - 表格配置参数
  - `a_code_gen.rs` - 代码生成参数

---

## 阶段二：后端业务服务层 (Application)

### 2.1 模型配置服务

- [x] **Task 2.1.1**: 创建模型管理服务
  - `cms_model_service.rs`
  - 模型CRUD
  - 模型启用/禁用
  - 模型复制

- [x] **Task 2.1.2**: 创建字段管理服务
  - `cms_field_service.rs`
  - 字段CRUD
  - 字段排序
  - 字段验证规则处理

- [x] **Task 2.1.3**: 创建动态表管理服务
  - `dynamic_table_service.rs`
  - 动态创建数据表
  - 动态添加/删除字段
  - 表结构同步

### 2.2 表单构建服务

- [x] **Task 2.2.1**: 创建表单配置服务
  - `form_config_service.rs`
  - 表单配置CRUD
  - 表单布局处理
  - 表单分组管理

- [x] **Task 2.2.2**: 创建表单渲染服务
  - `form_render_service.rs`
  - 根据配置生成表单Schema
  - 字段组件映射
  - 验证规则生成

### 2.3 表格构建服务

- [x] **Task 2.3.1**: 创建表格配置服务
  - `table_config_service.rs`
  - 表格配置CRUD
  - 列配置管理
  - 搜索筛选配置

- [x] **Task 2.3.2**: 创建表格渲染服务
  - `table_render_service.rs`
  - 根据配置生成表格Schema
  - 操作按钮配置
  - 分页配置

### 2.4 内容管理服务

- [x] **Task 2.4.1**: 创建内容基础服务
  - `cms_content_service.rs`
  - 内容CRUD（支持动态字段）
  - 内容状态管理
  - 内容审核流程

- [x] **Task 2.4.2**: 创建内容发布服务
  - `content_publish_service.rs`
  - 内容发布/下线
  - 定时发布
  - 内容版本管理

- [x] **Task 2.4.3**: 创建内容查询服务
  - `content_query_service.rs`
  - 动态字段查询
  - 全文搜索
  - 高级筛选

### 2.5 分类标签服务

- [x] **Task 2.5.1**: 创建分类管理服务
  - `cms_category_service.rs`
  - 分类CRUD
  - 分类树形结构
  - 分类排序

- [x] **Task 2.5.2**: 创建标签管理服务
  - `cms_tag_service.rs`
  - 标签CRUD
  - 标签云
  - 标签统计

### 2.6 代码生成服务

- [x] **Task 2.6.1**: 创建代码生成核心服务
  - `code_generator_service.rs`
  - Entity代码生成
  - Model代码生成
  - Args代码生成
  - Service代码生成
  - API代码生成

- [x] **Task 2.6.2**: 创建前端代码生成服务
  - `frontend_code_generator.rs`
  - Vue列表页生成
  - Vue表单页生成
  - Vue详情页生成
  - API接口文件生成

- [x] **Task 2.6.3**: 创建代码打包下载服务
  - `code_pack_service.rs`
  - 代码打包为ZIP
  - 文件树生成
  - 代码预览

---

## 阶段三：API 路由层

### 3.1 模型管理 API

- [x] **Task 3.1.1**: 创建模型管理 API 路由
  - `/api/cms/model/` 路由组
  - 模型列表、详情、新增、编辑、删除接口
  - 字段管理接口

### 3.2 表单配置 API

- [x] **Task 3.2.1**: 创建表单配置 API 路由
  - `/api/cms/form/` 路由组
  - 表单配置CRUD接口
  - 表单预览接口

### 3.3 表格配置 API

- [x] **Task 3.3.1**: 创建表格配置 API 路由
  - `/api/cms/table/` 路由组
  - 表格配置CRUD接口
  - 表格预览接口

### 3.4 内容管理 API

- [x] **Task 3.4.1**: 创建内容管理 API 路由
  - `/api/cms/content/` 路由组
  - 内容CRUD接口
  - 内容发布/下线接口
  - 内容审核接口
  - 版本管理接口

### 3.5 分类标签 API

- [x] **Task 3.5.1**: 创建分类管理 API 路由
  - `/api/cms/category/` 路由组
  - 分类CRUD接口
  - 分类树接口

- [x] **Task 3.5.2**: 创建标签管理 API 路由
  - `/api/cms/tag/` 路由组
  - 标签CRUD接口
  - 标签云接口

### 3.6 代码生成 API

- [x] **Task 3.6.1**: 创建代码生成 API 路由
  - `/api/cms/code-gen/` 路由组
  - 代码预览接口
  - 代码下载接口
  - 代码应用接口

---

## 阶段四：前端 - API 层

### 4.1 CMS API 模块

- [x] **Task 4.1.1**: 创建CMS API 模块
  - `frontend/src/api/modules/cms/`
  - `index.ts` - 导出
  - `model.ts` - 模型接口
  - `field.ts` - 字段接口
  - `content.ts` - 内容接口
  - `category.ts` - 分类接口
  - `tag.ts` - 标签接口
  - `form.ts` - 表单配置接口
  - `table.ts` - 表格配置接口
  - `codeGen.ts` - 代码生成接口

---

## 阶段五：前端 - Mock 数据层

### 5.1 Mock 数据文件

- [x] **Task 5.1.1**: 创建CMS Mock 数据
  - `frontend/src/mock/cms/`
  - 模型Mock数据
  - 字段Mock数据
  - 内容Mock数据
  - 分类Mock数据
  - 标签Mock数据

- [x] **Task 5.1.2**: 创建表单表格配置 Mock
  - 表单配置Mock
  - 表格配置Mock
  - 代码生成Mock

### 5.2 Mock 接口注册

- [x] **Task 5.2.1**: 注册CMS Mock 路由
  - 在 `frontend/src/mock/index.ts` 中注册

---

## 阶段六：前端 - 组件开发

### 6.1 模型设计器组件

- [x] **Task 6.1.1**: 创建模型设计器核心组件
  - `frontend/src/components/cms/model-designer/`
  - `ModelDesigner.vue` - 模型设计器主组件
  - `ModelForm.vue` - 模型基本信息表单
  - `ModelConfig.vue` - 模型配置组件

- [x] **Task 6.1.2**: 创建字段配置组件
  - `FieldList.vue` - 字段列表
  - `FieldEditor.vue` - 字段编辑器
  - `FieldTypeSelect.vue` - 字段类型选择器
  - `ValidationRules.vue` - 验证规则配置

### 6.2 表单构建器组件

- [x] **Task 6.2.1**: 创建表单构建器核心组件
  - `frontend/src/components/cms/form-builder/`
  - `FormBuilder.vue` - 表单构建器主组件
  - `FormPreview.vue` - 表单预览
  - `FieldLayout.vue` - 字段布局配置

### 6.3 表格构建器组件

- [x] **Task 6.3.1**: 创建表格构建器核心组件
  - `frontend/src/components/cms/table-builder/`
  - `TableBuilder.vue` - 表格构建器主组件
  - `ColumnConfig.vue` - 列配置
  - `SearchConfig.vue` - 搜索配置
  - `ActionConfig.vue` - 操作配置

### 6.4 代码生成组件

- [x] **Task 6.4.1**: 创建代码生成相关组件
  - `frontend/src/components/cms/code-gen/`
  - `CodePreview.vue` - 代码预览
  - `FileTree.vue` - 文件树
  - `CodeViewer.vue` - 代码查看器（语法高亮）

### 6.5 动态表单组件

- [x] **Task 6.5.1**: 创建动态表单渲染组件
  - `frontend/src/components/cms/dynamic-form/`
  - `DynamicForm.vue` - 动态表单主组件
  - 各字段类型组件（Text, Select, Editor, Image等）

### 6.6 动态表格组件

- [x] **Task 6.6.1**: 创建动态表格渲染组件
  - `frontend/src/components/cms/dynamic-table/`
  - `DynamicTable.vue` - 动态表格主组件
  - 列渲染器组件

---

## 阶段七：前端 - 页面开发

### 7.1 模型管理页面

- [x] **Task 7.1.1**: 创建模型列表页面
  - `frontend/src/views/cms/model/list.vue`
  - 模型列表展示
  - 模型搜索筛选
  - 模型状态切换

- [x] **Task 7.1.2**: 创建模型设计器页面
  - `frontend/src/views/cms/model/design.vue`
  - 模型基本信息配置
  - 字段配置面板
  - 模型配置面板

### 7.2 表单配置页面

- [x] **Task 7.2.1**: 创建表单配置列表页面
  - `frontend/src/views/cms/form/list.vue`

- [x] **Task 7.2.2**: 创建表单构建器页面
  - `frontend/src/views/cms/form/builder.vue`
  - 表单布局配置
  - 字段分组配置
  - 表单规则配置

### 7.3 表格配置页面

- [x] **Task 7.3.1**: 创建表格配置列表页面
  - `frontend/src/views/cms/table/list.vue`

- [x] **Task 7.3.2**: 创建表格构建器页面
  - `frontend/src/views/cms/table/builder.vue`
  - 列配置
  - 搜索筛选配置
  - 操作按钮配置

### 7.4 内容管理页面

- [x] **Task 7.4.1**: 创建内容列表页面（动态）
  - `frontend/src/views/cms/content/list.vue`
  - 根据模型动态渲染表格
  - 动态搜索筛选
  - 批量操作

- [x] **Task 7.4.2**: 创建内容表单页面（动态）
  - `frontend/src/views/cms/content/form.vue`
  - 根据模型动态渲染表单
  - 富文本编辑器集成
  - 图片上传集成

- [x] **Task 7.4.3**: 创建内容详情页面（动态）
  - `frontend/src/views/cms/content/detail.vue`
  - 根据模型动态渲染详情

- [x] **Task 7.4.4**: 创建回收站页面
  - `frontend/src/views/cms/content/recycle.vue`
  - 已删除内容列表
  - 内容恢复

### 7.5 分类管理页面

- [x] **Task 7.5.1**: 创建分类列表页面
  - `frontend/src/views/cms/category/list.vue`
  - 分类树形展示
  - 分类拖拽排序

### 7.6 标签管理页面

- [x] **Task 7.6.1**: 创建标签列表页面
  - `frontend/src/views/cms/tag/list.vue`
  - 标签列表
  - 标签云展示

### 7.7 代码生成页面

- [x] **Task 7.7.1**: 创建代码生成首页
  - `frontend/src/views/cms/code-gen/index.vue`
  - 模型选择
  - 生成配置

- [x] **Task 7.7.2**: 创建代码预览页面
  - `frontend/src/views/cms/code-gen/preview.vue`
  - 文件树
  - 代码预览
  - 下载按钮

---

## 阶段八：国际化与菜单

### 8.1 国际化

- [x] **Task 8.1.1**: 更新中文语言包
  - 添加CMS模块相关中文

- [x] **Task 8.1.2**: 更新英文语言包
  - 添加CMS模块相关英文

### 8.2 菜单与路由

- [x] **Task 8.2.1**: 添加CMS菜单
  - 菜单数据库迁移文件
  - 菜单图标配置

- [x] **Task 8.2.2**: 配置前端路由
  - 添加CMS模块路由

---

## 阶段九：权限与安全

### 9.1 权限控制

- [x] **Task 9.1.1**: 创建CMS权限
  - API权限定义
  - 按钮权限控制

### 9.2 安全机制

- [x] **Task 9.2.1**: 实现动态SQL安全
  - SQL注入防护
  - 动态表名白名单

---

## 阶段十：插件系统集成

### 10.1 插件接口

- [x] **Task 10.1.1**: 创建CMS插件接口
  - `CmsPlugin` trait定义
  - `FieldTypePlugin` trait定义
  - `WidgetPlugin` trait定义
  - `TemplatePlugin` trait定义

### 10.2 插件管理

- [x] **Task 10.2.1**: 创建CMS插件管理器
  - 插件注册
  - 插件加载
  - 插件启用/禁用

### 10.3 插件市场集成

- [x] **Task 10.3.1**: 集成插件市场
  - 与plugin-market模块集成
  - License验证集成

---

## 任务依赖关系

```
阶段一（数据库与实体）
    ↓
阶段二（业务服务）依赖 阶段一
    ↓
阶段三（API路由）依赖 阶段二
    ↓
阶段四（前端API）依赖 阶段三（接口定义）
阶段五（Mock数据）可并行
    ↓
阶段六（前端组件）依赖 阶段四、阶段五
    ↓
阶段七（前端页面）依赖 阶段六
    ↓
阶段八（国际化菜单）
阶段九（权限安全）
    ↓
阶段十（插件系统）依赖 阶段一至九
```

---

## 验收标准

1. 数据库迁移文件可正常执行
2. 动态表创建功能正常
3. 模型配置功能完整
4. 字段配置功能完整
5. 表单构建器可正常使用
6. 表格构建器可正常使用
7. 内容CRUD功能正常
8. 分类标签管理功能正常
9. 代码生成功能正常
10. 生成的代码可编译运行
11. 前端页面可正常访问
12. 权限控制正常生效
13. 代码风格符合项目规范
