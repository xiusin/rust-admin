export interface CmsModelList {
  id: number;
  name: string;
  code: string;
  tableName: string;
  description: string;
  icon: string;
  isEnabled: boolean;
  isSystem: boolean;
  sort: number;
  fieldCount: number;
  createdAt: string;
}

export interface CmsFieldItem {
  id: number;
  modelId: number;
  name: string;
  code: string;
  fieldType:
    | "text"
    | "textarea"
    | "number"
    | "select"
    | "radio"
    | "checkbox"
    | "date"
    | "datetime"
    | "image"
    | "file"
    | "editor"
    | "switch";
  defaultValue: string | null;
  isRequired: boolean;
  isUnique: boolean;
  isSearchable: boolean;
  isSortable: boolean;
  isSystem: boolean;
  options: string | null;
  validation: string | null;
  placeholder: string | null;
  helpText: string | null;
  sort: number;
  createdAt: string;
}

export interface CmsModelDetail {
  id: number;
  name: string;
  code: string;
  tableName: string;
  description: string;
  icon: string;
  isEnabled: boolean;
  isSystem: boolean;
  sort: number;
  fields: CmsFieldItem[];
  createdAt: string;
  updatedAt: string;
}

export const modelList: CmsModelList[] = [
  {
    id: 1,
    name: "文章",
    code: "article",
    tableName: "cms_content_article",
    description: "文章内容模型，用于发布新闻、资讯等文本类内容",
    icon: "Document",
    isEnabled: true,
    isSystem: true,
    sort: 1,
    fieldCount: 10,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 2,
    name: "产品",
    code: "product",
    tableName: "cms_content_product",
    description: "产品内容模型，用于展示产品信息、规格参数等",
    icon: "Box",
    isEnabled: true,
    isSystem: false,
    sort: 2,
    fieldCount: 15,
    createdAt: "2024-01-02 00:00:00"
  },
  {
    id: 3,
    name: "图片",
    code: "image",
    tableName: "cms_content_image",
    description: "图片内容模型，用于图片展示、相册管理等",
    icon: "Picture",
    isEnabled: true,
    isSystem: false,
    sort: 3,
    fieldCount: 8,
    createdAt: "2024-01-03 00:00:00"
  },
  {
    id: 4,
    name: "视频",
    code: "video",
    tableName: "cms_content_video",
    description: "视频内容模型，用于视频内容管理和播放",
    icon: "VideoPlay",
    isEnabled: true,
    isSystem: false,
    sort: 4,
    fieldCount: 12,
    createdAt: "2024-01-04 00:00:00"
  },
  {
    id: 5,
    name: "下载",
    code: "download",
    tableName: "cms_content_download",
    description: "下载内容模型，用于文件资源下载管理",
    icon: "Download",
    isEnabled: true,
    isSystem: false,
    sort: 5,
    fieldCount: 9,
    createdAt: "2024-01-05 00:00:00"
  },
  {
    id: 6,
    name: "案例",
    code: "case",
    tableName: "cms_content_case",
    description: "案例内容模型，用于展示成功案例、项目案例等",
    icon: "TrendCharts",
    isEnabled: false,
    isSystem: false,
    sort: 6,
    fieldCount: 11,
    createdAt: "2024-01-06 00:00:00"
  }
];

export const fieldList: CmsFieldItem[] = [
  {
    id: 1,
    modelId: 1,
    name: "标题",
    code: "title",
    fieldType: "text",
    defaultValue: null,
    isRequired: true,
    isUnique: false,
    isSearchable: true,
    isSortable: true,
    isSystem: true,
    options: null,
    validation: '{"max": 200}',
    placeholder: "请输入标题",
    helpText: "标题长度不超过200字符",
    sort: 1,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 2,
    modelId: 1,
    name: "副标题",
    code: "subtitle",
    fieldType: "text",
    defaultValue: null,
    isRequired: false,
    isUnique: false,
    isSearchable: true,
    isSortable: false,
    isSystem: false,
    options: null,
    validation: '{"max": 200}',
    placeholder: "请输入副标题",
    helpText: null,
    sort: 2,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 3,
    modelId: 1,
    name: "缩略图",
    code: "thumbnail",
    fieldType: "image",
    defaultValue: null,
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: false,
    isSystem: false,
    options: '{"maxSize": 2048, "accept": ".jpg,.jpeg,.png,.gif"}',
    validation: null,
    placeholder: null,
    helpText: "支持jpg、png、gif格式，最大2MB",
    sort: 3,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 4,
    modelId: 1,
    name: "内容",
    code: "content",
    fieldType: "editor",
    defaultValue: null,
    isRequired: true,
    isUnique: false,
    isSearchable: true,
    isSortable: false,
    isSystem: true,
    options: null,
    validation: null,
    placeholder: "请输入内容",
    helpText: null,
    sort: 4,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 5,
    modelId: 1,
    name: "摘要",
    code: "summary",
    fieldType: "textarea",
    defaultValue: null,
    isRequired: false,
    isUnique: false,
    isSearchable: true,
    isSortable: false,
    isSystem: false,
    options: null,
    validation: '{"max": 500}',
    placeholder: "请输入摘要",
    helpText: "摘要长度不超过500字符",
    sort: 5,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 6,
    modelId: 1,
    name: "作者",
    code: "author",
    fieldType: "text",
    defaultValue: "管理员",
    isRequired: false,
    isUnique: false,
    isSearchable: true,
    isSortable: true,
    isSystem: false,
    options: null,
    validation: '{"max": 50}',
    placeholder: "请输入作者",
    helpText: null,
    sort: 6,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 7,
    modelId: 1,
    name: "来源",
    code: "source",
    fieldType: "text",
    defaultValue: null,
    isRequired: false,
    isUnique: false,
    isSearchable: true,
    isSortable: false,
    isSystem: false,
    options: null,
    validation: '{"max": 100}',
    placeholder: "请输入来源",
    helpText: null,
    sort: 7,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 8,
    modelId: 1,
    name: "阅读量",
    code: "view_count",
    fieldType: "number",
    defaultValue: "0",
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: true,
    isSystem: true,
    options: null,
    validation: '{"min": 0}',
    placeholder: null,
    helpText: null,
    sort: 8,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 9,
    modelId: 1,
    name: "是否置顶",
    code: "is_top",
    fieldType: "switch",
    defaultValue: "0",
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: true,
    isSystem: false,
    options: null,
    validation: null,
    placeholder: null,
    helpText: null,
    sort: 9,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 10,
    modelId: 1,
    name: "是否推荐",
    code: "is_recommend",
    fieldType: "switch",
    defaultValue: "0",
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: true,
    isSystem: false,
    options: null,
    validation: null,
    placeholder: null,
    helpText: null,
    sort: 10,
    createdAt: "2024-01-01 00:00:00"
  },
  {
    id: 11,
    modelId: 2,
    name: "产品名称",
    code: "product_name",
    fieldType: "text",
    defaultValue: null,
    isRequired: true,
    isUnique: true,
    isSearchable: true,
    isSortable: true,
    isSystem: true,
    options: null,
    validation: '{"max": 100}',
    placeholder: "请输入产品名称",
    helpText: null,
    sort: 1,
    createdAt: "2024-01-02 00:00:00"
  },
  {
    id: 12,
    modelId: 2,
    name: "产品编码",
    code: "product_code",
    fieldType: "text",
    defaultValue: null,
    isRequired: true,
    isUnique: true,
    isSearchable: true,
    isSortable: true,
    isSystem: true,
    options: null,
    validation: '{"max": 50}',
    placeholder: "请输入产品编码",
    helpText: null,
    sort: 2,
    createdAt: "2024-01-02 00:00:00"
  },
  {
    id: 13,
    modelId: 2,
    name: "产品价格",
    code: "price",
    fieldType: "number",
    defaultValue: "0",
    isRequired: true,
    isUnique: false,
    isSearchable: false,
    isSortable: true,
    isSystem: false,
    options: null,
    validation: '{"min": 0, "precision": 2}',
    placeholder: "请输入价格",
    helpText: "支持两位小数",
    sort: 3,
    createdAt: "2024-01-02 00:00:00"
  },
  {
    id: 14,
    modelId: 2,
    name: "产品库存",
    code: "stock",
    fieldType: "number",
    defaultValue: "0",
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: true,
    isSystem: false,
    options: null,
    validation: '{"min": 0}',
    placeholder: "请输入库存数量",
    helpText: null,
    sort: 4,
    createdAt: "2024-01-02 00:00:00"
  },
  {
    id: 15,
    modelId: 2,
    name: "产品图片",
    code: "images",
    fieldType: "image",
    defaultValue: null,
    isRequired: false,
    isUnique: false,
    isSearchable: false,
    isSortable: false,
    isSystem: false,
    options: '{"maxSize": 5120, "accept": ".jpg,.jpeg,.png", "multiple": true}',
    validation: null,
    placeholder: null,
    helpText: "支持多图上传，每张最大5MB",
    sort: 5,
    createdAt: "2024-01-02 00:00:00"
  }
];

export const modelDetail: CmsModelDetail = {
  id: 1,
  name: "文章",
  code: "article",
  tableName: "cms_content_article",
  description: "文章内容模型，用于发布新闻、资讯等文本类内容",
  icon: "Document",
  isEnabled: true,
  isSystem: true,
  sort: 1,
  fields: fieldList.filter(f => f.modelId === 1),
  createdAt: "2024-01-01 00:00:00",
  updatedAt: "2024-03-15 10:30:00"
};
