export interface GeneratedFile {
  filePath: string;
  fileName: string;
  content: string;
  language: 'rust' | 'vue' | 'typescript' | 'javascript' | 'sql' | 'json';
  size: number;
  createdAt: string;
}

export interface FileTreeNode {
  name: string;
  path: string;
  isDir: boolean;
  children: FileTreeNode[] | null;
}

export interface CodeGenConfig {
  modelId: number;
  modelName: string;
  tableName: string;
  generateBackend: boolean;
  generateFrontend: boolean;
  backendOptions: {
    generateEntity: boolean;
    generateModel: boolean;
    generateArgs: boolean;
    generateService: boolean;
    generateApi: boolean;
  };
  frontendOptions: {
    generateApi: boolean;
    generateViews: boolean;
    generateComponents: boolean;
  };
  author: string;
  moduleName: string;
}

export const generatedFiles: GeneratedFile[] = [
  {
    filePath: 'backend/entity/cms_content_article.rs',
    fileName: 'cms_content_article.rs',
    content: `use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cms_content_article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub model_id: i64,
    pub category_id: i64,
    pub title: String,
    #[sea_orm(nullable)]
    pub subtitle: Option<String>,
    #[sea_orm(nullable)]
    pub thumbnail: Option<String>,
    pub content: String,
    #[sea_orm(nullable)]
    pub summary: Option<String>,
    #[sea_orm(nullable)]
    pub author: Option<String>,
    #[sea_orm(nullable)]
    pub source: Option<String>,
    pub status: String,
    #[sea_orm(nullable)]
    pub publish_time: Option<DateTime>,
    pub view_count: i32,
    pub is_top: bool,
    pub is_recommend: bool,
    #[sea_orm(nullable)]
    pub seo_title: Option<String>,
    #[sea_orm(nullable)]
    pub seo_keywords: Option<String>,
    #[sea_orm(nullable)]
    pub seo_description: Option<String>,
    #[sea_orm(nullable)]
    pub created_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub updated_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cms_model::Entity",
        from = "Column::ModelId",
        to = "super::cms_model::Column::Id"
    )]
    Model,
    #[sea_orm(
        belongs_to = "super::cms_category::Entity",
        from = "Column::CategoryId",
        to = "super::cms_category::Column::Id"
    )]
    Category,
}

impl Related<super::cms_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl Related<super::cms_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}`,
    language: 'rust',
    size: 2048,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'backend/model/m_cms_article.rs',
    fileName: 'm_cms_article.rs',
    content: `use serde::{Deserialize, Serialize};
use sea_orm::FromQueryResult;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct CmsArticleList {
    pub id: i64,
    pub model_id: i64,
    pub model_name: String,
    pub category_id: i64,
    pub category_name: String,
    pub title: String,
    pub thumbnail: Option<String>,
    pub status: String,
    pub publish_time: Option<DateTime>,
    pub view_count: i32,
    pub is_top: bool,
    pub is_recommend: bool,
    pub author: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsArticleDetail {
    pub id: i64,
    pub model_id: i64,
    pub model_name: String,
    pub category_id: i64,
    pub category_name: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub thumbnail: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub status: String,
    pub publish_time: Option<DateTime>,
    pub view_count: i32,
    pub is_top: bool,
    pub is_recommend: bool,
    pub tags: Vec<String>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub created_by: String,
    pub updated_by: String,
}`,
    language: 'rust',
    size: 1536,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'backend/args/a_cms_article.rs',
    fileName: 'a_cms_article.rs',
    content: `use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CmsArticleAddReq {
    #[validate(length(min = 1, max = 200, message = "标题长度必须在1-200之间"))]
    pub title: String,
    #[validate(length(max = 200, message = "副标题长度不能超过200"))]
    pub subtitle: Option<String>,
    pub category_id: i64,
    pub thumbnail: Option<String>,
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    #[validate(length(max = 500, message = "摘要长度不能超过500"))]
    pub summary: Option<String>,
    #[validate(length(max = 50, message = "作者长度不能超过50"))]
    pub author: Option<String>,
    #[validate(length(max = 100, message = "来源长度不能超过100"))]
    pub source: Option<String>,
    pub status: String,
    pub publish_time: Option<DateTime>,
    pub is_top: Option<bool>,
    pub is_recommend: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CmsArticleEditReq {
    pub id: i64,
    #[validate(length(min = 1, max = 200, message = "标题长度必须在1-200之间"))]
    pub title: String,
    #[validate(length(max = 200, message = "副标题长度不能超过200"))]
    pub subtitle: Option<String>,
    pub category_id: i64,
    pub thumbnail: Option<String>,
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    #[validate(length(max = 500, message = "摘要长度不能超过500"))]
    pub summary: Option<String>,
    #[validate(length(max = 50, message = "作者长度不能超过50"))]
    pub author: Option<String>,
    #[validate(length(max = 100, message = "来源长度不能超过100"))]
    pub source: Option<String>,
    pub status: String,
    pub publish_time: Option<DateTime>,
    pub is_top: Option<bool>,
    pub is_recommend: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsArticleSearchReq {
    pub title: Option<String>,
    pub category_id: Option<i64>,
    pub model_id: Option<i64>,
    pub status: Option<String>,
    pub publish_time_start: Option<DateTime>,
    pub publish_time_end: Option<DateTime>,
    pub is_top: Option<bool>,
    pub is_recommend: Option<bool>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}`,
    language: 'rust',
    size: 2560,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'backend/service/cms_article_service.rs',
    fileName: 'cms_article_service.rs',
    content: `use crate::domain::entity::cms_content_article;
use crate::domain::model::m_cms_article::*;
use crate::domain::args::a_cms_article::*;
use crate::common::result::Result;
use crate::common::pagination::PageData;

pub async fn list(params: CmsArticleSearchReq) -> Result<PageData<CmsArticleList>> {
    // 实现列表查询逻辑
    todo!()
}

pub async fn detail(id: i64) -> Result<CmsArticleDetail> {
    // 实现详情查询逻辑
    todo!()
}

pub async fn add(params: CmsArticleAddReq) -> Result<i64> {
    // 实现新增逻辑
    todo!()
}

pub async fn edit(params: CmsArticleEditReq) -> Result<()> {
    // 实现编辑逻辑
    todo!()
}

pub async fn delete(ids: Vec<i64>) -> Result<()> {
    // 实现删除逻辑
    todo!()
}

pub async fn publish(id: i64) -> Result<()> {
    // 实现发布逻辑
    todo!()
}

pub async fn unpublish(id: i64) -> Result<()> {
    // 实现取消发布逻辑
    todo!()
}`,
    language: 'rust',
    size: 1024,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'backend/api/cms_article.rs',
    fileName: 'cms_article.rs',
    content: `use axum::{
    extract::Query,
    routing::{get, post, put, delete},
    Json, Router,
};
use crate::domain::args::a_cms_article::*;
use crate::application::cms_article_service;
use crate::common::result::Result;
use crate::common::pagination::PageData;

pub fn cms_article_routes() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/detail/:id", get(detail))
        .route("/add", post(add))
        .route("/edit", put(edit))
        .route("/delete", delete(delete))
        .route("/publish/:id", post(publish))
        .route("/unpublish/:id", post(unpublish))
}

async fn list(Query(params): Query<CmsArticleSearchReq>) -> Json<Result<PageData<CmsArticleList>>> {
    match cms_article_service::list(params).await {
        Ok(data) => Json(Result::success(data)),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn detail(id: i64) -> Json<Result<CmsArticleDetail>> {
    match cms_article_service::detail(id).await {
        Ok(data) => Json(Result::success(data)),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn add(Json(params): Json<CmsArticleAddReq>) -> Json<Result<i64>> {
    match cms_article_service::add(params).await {
        Ok(id) => Json(Result::success(id)),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn edit(Json(params): Json<CmsArticleEditReq>) -> Json<Result<()>> {
    match cms_article_service::edit(params).await {
        Ok(_) => Json(Result::success(())),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn delete(Json(ids): Json<Vec<i64>>) -> Json<Result<()>> {
    match cms_article_service::delete(ids).await {
        Ok(_) => Json(Result::success(())),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn publish(id: i64) -> Json<Result<()>> {
    match cms_article_service::publish(id).await {
        Ok(_) => Json(Result::success(())),
        Err(e) => Json(Result::error(e.to_string())),
    }
}

async fn unpublish(id: i64) -> Json<Result<()>> {
    match cms_article_service::unpublish(id).await {
        Ok(_) => Json(Result::success(())),
        Err(e) => Json(Result::error(e.to_string())),
    }
}`,
    language: 'rust',
    size: 2048,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'frontend/api/cms/article.ts',
    fileName: 'article.ts',
    content: `import request from '@/utils/request';

export interface ArticleListParams {
  title?: string;
  categoryId?: number;
  modelId?: number;
  status?: string;
  publishTimeStart?: string;
  publishTimeEnd?: string;
  isTop?: boolean;
  isRecommend?: boolean;
  pageNum?: number;
  pageSize?: number;
}

export interface ArticleAddParams {
  title: string;
  subtitle?: string;
  categoryId: number;
  thumbnail?: string;
  content: string;
  summary?: string;
  author?: string;
  source?: string;
  status: string;
  publishTime?: string;
  isTop?: boolean;
  isRecommend?: boolean;
  tags?: string[];
  seoTitle?: string;
  seoKeywords?: string;
  seoDescription?: string;
}

export interface ArticleEditParams extends ArticleAddParams {
  id: number;
}

export function getArticleList(params: ArticleListParams) {
  return request({
    url: '/api/cms/article/list',
    method: 'get',
    params
  });
}

export function getArticleDetail(id: number) {
  return request({
    url: \`/api/cms/article/detail/\${id}\`,
    method: 'get'
  });
}

export function addArticle(data: ArticleAddParams) {
  return request({
    url: '/api/cms/article/add',
    method: 'post',
    data
  });
}

export function editArticle(data: ArticleEditParams) {
  return request({
    url: '/api/cms/article/edit',
    method: 'put',
    data
  });
}

export function deleteArticle(ids: number[]) {
  return request({
    url: '/api/cms/article/delete',
    method: 'delete',
    data: ids
  });
}

export function publishArticle(id: number) {
  return request({
    url: \`/api/cms/article/publish/\${id}\`,
    method: 'post'
  });
}

export function unpublishArticle(id: number) {
  return request({
    url: \`/api/cms/article/unpublish/\${id}\`,
    method: 'post'
  });
}`,
    language: 'typescript',
    size: 2048,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'frontend/views/cms/article/list.vue',
    fileName: 'list.vue',
    content: `<template>
  <div class="app-container">
    <pro-table
      ref="tableRef"
      :columns="columns"
      :request-api="getArticleList"
      :search-config="searchConfig"
      :toolbar-config="toolbarConfig"
      :row-actions="rowActions"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import { getArticleList, deleteArticle, publishArticle, unpublishArticle } from '@/api/cms/article';

const router = useRouter();
const tableRef = ref();

const columns = ref([
  { prop: 'id', label: 'ID', width: 80 },
  { prop: 'thumbnail', label: '缩略图', width: 100, type: 'image' },
  { prop: 'title', label: '标题', minWidth: 200 },
  { prop: 'categoryName', label: '所属分类', width: 120 },
  { prop: 'status', label: '状态', width: 100, type: 'tag' },
  { prop: 'viewCount', label: '阅读量', width: 100 },
  { prop: 'author', label: '作者', width: 100 },
  { prop: 'publishTime', label: '发布时间', width: 160 },
  { prop: 'actions', label: '操作', width: 200, fixed: 'right' }
]);

const searchConfig = {
  fields: [
    { field: 'title', label: '标题', type: 'text' },
    { field: 'categoryId', label: '分类', type: 'tree-select' },
    { field: 'status', label: '状态', type: 'select', options: [] },
    { field: 'publishTime', label: '发布时间', type: 'daterange' }
  ]
};

const toolbarConfig = {
  buttons: [
    { key: 'create', label: '新增', type: 'primary', icon: 'Plus' },
    { key: 'delete', label: '批量删除', type: 'danger', icon: 'Delete' }
  ]
};

const rowActions = [
  { key: 'edit', label: '编辑', type: 'primary' },
  { key: 'publish', label: '发布', type: 'success' },
  { key: 'delete', label: '删除', type: 'danger' }
];

const handleCreate = () => {
  router.push('/cms/article/create');
};

const handleEdit = (row: any) => {
  router.push(\`/cms/article/edit/\${row.id}\`);
};

const handleDelete = async (ids: number[]) => {
  await ElMessageBox.confirm('确定要删除选中的内容吗？', '提示', {
    type: 'warning'
  });
  await deleteArticle(ids);
  ElMessage.success('删除成功');
  tableRef.value?.refresh();
};

const handlePublish = async (row: any) => {
  await publishArticle(row.id);
  ElMessage.success('发布成功');
  tableRef.value?.refresh();
};
</script>`,
    language: 'vue',
    size: 3072,
    createdAt: '2024-01-01 00:00:00'
  },
  {
    filePath: 'migrations/m20240101_001_cms_article_create_table.sql',
    fileName: 'm20240101_001_cms_article_create_table.sql',
    content: `-- 创建文章内容表
CREATE TABLE \`cms_content_article\` (
  \`id\` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  \`model_id\` bigint(20) NOT NULL COMMENT '模型ID',
  \`category_id\` bigint(20) NOT NULL COMMENT '分类ID',
  \`title\` varchar(200) NOT NULL COMMENT '标题',
  \`subtitle\` varchar(200) DEFAULT NULL COMMENT '副标题',
  \`thumbnail\` varchar(500) DEFAULT NULL COMMENT '缩略图',
  \`content\` longtext NOT NULL COMMENT '内容',
  \`summary\` varchar(500) DEFAULT NULL COMMENT '摘要',
  \`author\` varchar(50) DEFAULT NULL COMMENT '作者',
  \`source\` varchar(100) DEFAULT NULL COMMENT '来源',
  \`status\` varchar(20) NOT NULL DEFAULT 'draft' COMMENT '状态：draft-草稿，published-已发布，unpublished-未发布，archived-已归档',
  \`publish_time\` datetime DEFAULT NULL COMMENT '发布时间',
  \`view_count\` int(11) NOT NULL DEFAULT '0' COMMENT '阅读量',
  \`is_top\` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否置顶：0-否，1-是',
  \`is_recommend\` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否推荐：0-否，1-是',
  \`seo_title\` varchar(100) DEFAULT NULL COMMENT 'SEO标题',
  \`seo_keywords\` varchar(200) DEFAULT NULL COMMENT 'SEO关键词',
  \`seo_description\` varchar(300) DEFAULT NULL COMMENT 'SEO描述',
  \`created_at\` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  \`updated_at\` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  \`deleted_at\` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (\`id\`),
  KEY \`idx_model_id\` (\`model_id\`),
  KEY \`idx_category_id\` (\`category_id\`),
  KEY \`idx_status\` (\`status\`),
  KEY \`idx_publish_time\` (\`publish_time\`),
  KEY \`idx_created_at\` (\`created_at\`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章内容表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS \`cms_content_article\`;`,
    language: 'sql',
    size: 2048,
    createdAt: '2024-01-01 00:00:00'
  }
];

export const fileTree: FileTreeNode = {
  name: 'generated',
  path: '',
  isDir: true,
  children: [
    {
      name: 'backend',
      path: 'backend',
      isDir: true,
      children: [
        {
          name: 'entity',
          path: 'backend/entity',
          isDir: true,
          children: [
            {
              name: 'cms_content_article.rs',
              path: 'backend/entity/cms_content_article.rs',
              isDir: false,
              children: null
            }
          ]
        },
        {
          name: 'model',
          path: 'backend/model',
          isDir: true,
          children: [
            {
              name: 'm_cms_article.rs',
              path: 'backend/model/m_cms_article.rs',
              isDir: false,
              children: null
            }
          ]
        },
        {
          name: 'args',
          path: 'backend/args',
          isDir: true,
          children: [
            {
              name: 'a_cms_article.rs',
              path: 'backend/args/a_cms_article.rs',
              isDir: false,
              children: null
            }
          ]
        },
        {
          name: 'service',
          path: 'backend/service',
          isDir: true,
          children: [
            {
              name: 'cms_article_service.rs',
              path: 'backend/service/cms_article_service.rs',
              isDir: false,
              children: null
            }
          ]
        },
        {
          name: 'api',
          path: 'backend/api',
          isDir: true,
          children: [
            {
              name: 'cms_article.rs',
              path: 'backend/api/cms_article.rs',
              isDir: false,
              children: null
            }
          ]
        }
      ]
    },
    {
      name: 'frontend',
      path: 'frontend',
      isDir: true,
      children: [
        {
          name: 'api',
          path: 'frontend/api',
          isDir: true,
          children: [
            {
              name: 'cms',
              path: 'frontend/api/cms',
              isDir: true,
              children: [
                {
                  name: 'article.ts',
                  path: 'frontend/api/cms/article.ts',
                  isDir: false,
                  children: null
                }
              ]
            }
          ]
        },
        {
          name: 'views',
          path: 'frontend/views',
          isDir: true,
          children: [
            {
              name: 'cms',
              path: 'frontend/views/cms',
              isDir: true,
              children: [
                {
                  name: 'article',
                  path: 'frontend/views/cms/article',
                  isDir: true,
                  children: [
                    {
                      name: 'list.vue',
                      path: 'frontend/views/cms/article/list.vue',
                      isDir: false,
                      children: null
                    }
                  ]
                }
              ]
            }
          ]
        }
      ]
    },
    {
      name: 'migrations',
      path: 'migrations',
      isDir: true,
      children: [
        {
          name: 'm20240101_001_cms_article_create_table.sql',
          path: 'migrations/m20240101_001_cms_article_create_table.sql',
          isDir: false,
          children: null
        }
      ]
    }
  ]
};

export const codeGenConfig: CodeGenConfig = {
  modelId: 1,
  modelName: '文章',
  tableName: 'cms_content_article',
  generateBackend: true,
  generateFrontend: true,
  backendOptions: {
    generateEntity: true,
    generateModel: true,
    generateArgs: true,
    generateService: true,
    generateApi: true
  },
  frontendOptions: {
    generateApi: true,
    generateViews: true,
    generateComponents: false
  },
  author: 'xiusin',
  moduleName: 'cms'
};
