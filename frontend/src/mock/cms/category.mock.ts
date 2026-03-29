export interface CmsCategoryTree {
  id: number;
  parentId: number | null;
  name: string;
  code: string;
  icon: string;
  sort: number;
  status: boolean;
  description: string | null;
  modelId: number | null;
  contentCount: number;
  children: CmsCategoryTree[] | null;
}

export interface CmsCategoryItem {
  id: number;
  parentId: number | null;
  parentName: string | null;
  name: string;
  code: string;
  icon: string;
  sort: number;
  status: boolean;
  description: string | null;
  modelId: number | null;
  modelName: string | null;
  contentCount: number;
  createdAt: string;
  updatedAt: string;
}

export const categoryTree: CmsCategoryTree[] = [
  {
    id: 1,
    parentId: null,
    name: "新闻中心",
    code: "news",
    icon: "Folder",
    sort: 1,
    status: true,
    description: "新闻资讯分类",
    modelId: 1,
    contentCount: 0,
    children: [
      {
        id: 2,
        parentId: 1,
        name: "公司新闻",
        code: "company-news",
        icon: "Document",
        sort: 1,
        status: true,
        description: "公司相关新闻动态",
        modelId: 1,
        contentCount: 5,
        children: null
      },
      {
        id: 3,
        parentId: 1,
        name: "行业动态",
        code: "industry-news",
        icon: "TrendCharts",
        sort: 2,
        status: true,
        description: "行业最新动态资讯",
        modelId: 1,
        contentCount: 8,
        children: null
      },
      {
        id: 4,
        parentId: 1,
        name: "技术分享",
        code: "tech-share",
        icon: "Cpu",
        sort: 3,
        status: true,
        description: "技术文章与经验分享",
        modelId: 1,
        contentCount: 12,
        children: null
      }
    ]
  },
  {
    id: 5,
    parentId: null,
    name: "产品中心",
    code: "products",
    icon: "Box",
    sort: 2,
    status: true,
    description: "产品展示分类",
    modelId: 2,
    contentCount: 0,
    children: [
      {
        id: 6,
        parentId: 5,
        name: "软件产品",
        code: "software",
        icon: "Monitor",
        sort: 1,
        status: true,
        description: "软件类产品",
        modelId: 2,
        contentCount: 15,
        children: null
      },
      {
        id: 7,
        parentId: 5,
        name: "硬件产品",
        code: "hardware",
        icon: "Cpu",
        sort: 2,
        status: true,
        description: "硬件类产品",
        modelId: 2,
        contentCount: 8,
        children: null
      },
      {
        id: 8,
        parentId: 5,
        name: "解决方案",
        code: "solutions",
        icon: "Grid",
        sort: 3,
        status: true,
        description: "行业解决方案",
        modelId: 2,
        contentCount: 6,
        children: null
      }
    ]
  },
  {
    id: 9,
    parentId: null,
    name: "资源中心",
    code: "resources",
    icon: "FolderOpened",
    sort: 3,
    status: true,
    description: "资源下载分类",
    modelId: 5,
    contentCount: 0,
    children: [
      {
        id: 10,
        parentId: 9,
        name: "产品手册",
        code: "manuals",
        icon: "Notebook",
        sort: 1,
        status: true,
        description: "产品使用手册",
        modelId: 5,
        contentCount: 20,
        children: null
      },
      {
        id: 11,
        parentId: 9,
        name: "技术文档",
        code: "tech-docs",
        icon: "Document",
        sort: 2,
        status: true,
        description: "技术相关文档",
        modelId: 5,
        contentCount: 35,
        children: null
      },
      {
        id: 12,
        parentId: 9,
        name: "软件下载",
        code: "downloads",
        icon: "Download",
        sort: 3,
        status: true,
        description: "软件资源下载",
        modelId: 5,
        contentCount: 12,
        children: null
      }
    ]
  },
  {
    id: 13,
    parentId: null,
    name: "案例展示",
    code: "cases",
    icon: "TrendCharts",
    sort: 4,
    status: true,
    description: "成功案例展示",
    modelId: 6,
    contentCount: 0,
    children: [
      {
        id: 14,
        parentId: 13,
        name: "企业案例",
        code: "enterprise-cases",
        icon: "OfficeBuilding",
        sort: 1,
        status: true,
        description: "企业级应用案例",
        modelId: 6,
        contentCount: 10,
        children: null
      },
      {
        id: 15,
        parentId: 13,
        name: "政府案例",
        code: "government-cases",
        icon: "School",
        sort: 2,
        status: true,
        description: "政府机构应用案例",
        modelId: 6,
        contentCount: 5,
        children: null
      }
    ]
  },
  {
    id: 16,
    parentId: null,
    name: "媒体中心",
    code: "media",
    icon: "PictureFilled",
    sort: 5,
    status: true,
    description: "媒体资源分类",
    modelId: 3,
    contentCount: 0,
    children: [
      {
        id: 17,
        parentId: 16,
        name: "图片素材",
        code: "images",
        icon: "Picture",
        sort: 1,
        status: true,
        description: "图片资源库",
        modelId: 3,
        contentCount: 50,
        children: null
      },
      {
        id: 18,
        parentId: 16,
        name: "视频素材",
        code: "videos",
        icon: "VideoPlay",
        sort: 2,
        status: true,
        description: "视频资源库",
        modelId: 4,
        contentCount: 25,
        children: null
      }
    ]
  }
];

export const categoryList: CmsCategoryItem[] = [
  {
    id: 1,
    parentId: null,
    parentName: null,
    name: "新闻中心",
    code: "news",
    icon: "Folder",
    sort: 1,
    status: true,
    description: "新闻资讯分类",
    modelId: 1,
    modelName: "文章",
    contentCount: 0,
    createdAt: "2024-01-01 00:00:00",
    updatedAt: "2024-01-01 00:00:00"
  },
  {
    id: 2,
    parentId: 1,
    parentName: "新闻中心",
    name: "公司新闻",
    code: "company-news",
    icon: "Document",
    sort: 1,
    status: true,
    description: "公司相关新闻动态",
    modelId: 1,
    modelName: "文章",
    contentCount: 5,
    createdAt: "2024-01-01 00:00:00",
    updatedAt: "2024-01-01 00:00:00"
  },
  {
    id: 3,
    parentId: 1,
    parentName: "新闻中心",
    name: "行业动态",
    code: "industry-news",
    icon: "TrendCharts",
    sort: 2,
    status: true,
    description: "行业最新动态资讯",
    modelId: 1,
    modelName: "文章",
    contentCount: 8,
    createdAt: "2024-01-01 00:00:00",
    updatedAt: "2024-01-01 00:00:00"
  },
  {
    id: 4,
    parentId: 1,
    parentName: "新闻中心",
    name: "技术分享",
    code: "tech-share",
    icon: "Cpu",
    sort: 3,
    status: true,
    description: "技术文章与经验分享",
    modelId: 1,
    modelName: "文章",
    contentCount: 12,
    createdAt: "2024-01-01 00:00:00",
    updatedAt: "2024-01-01 00:00:00"
  },
  {
    id: 5,
    parentId: null,
    parentName: null,
    name: "产品中心",
    code: "products",
    icon: "Box",
    sort: 2,
    status: true,
    description: "产品展示分类",
    modelId: 2,
    modelName: "产品",
    contentCount: 0,
    createdAt: "2024-01-02 00:00:00",
    updatedAt: "2024-01-02 00:00:00"
  },
  {
    id: 6,
    parentId: 5,
    parentName: "产品中心",
    name: "软件产品",
    code: "software",
    icon: "Monitor",
    sort: 1,
    status: true,
    description: "软件类产品",
    modelId: 2,
    modelName: "产品",
    contentCount: 15,
    createdAt: "2024-01-02 00:00:00",
    updatedAt: "2024-01-02 00:00:00"
  },
  {
    id: 7,
    parentId: 5,
    parentName: "产品中心",
    name: "硬件产品",
    code: "hardware",
    icon: "Cpu",
    sort: 2,
    status: true,
    description: "硬件类产品",
    modelId: 2,
    modelName: "产品",
    contentCount: 8,
    createdAt: "2024-01-02 00:00:00",
    updatedAt: "2024-01-02 00:00:00"
  },
  {
    id: 8,
    parentId: 5,
    parentName: "产品中心",
    name: "解决方案",
    code: "solutions",
    icon: "Grid",
    sort: 3,
    status: true,
    description: "行业解决方案",
    modelId: 2,
    modelName: "产品",
    contentCount: 6,
    createdAt: "2024-01-02 00:00:00",
    updatedAt: "2024-01-02 00:00:00"
  }
];
