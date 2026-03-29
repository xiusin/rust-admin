export interface CmsContentList {
  id: number;
  modelId: number;
  modelName: string;
  categoryId: number;
  categoryName: string;
  title: string;
  thumbnail: string | null;
  status: "draft" | "published" | "unpublished" | "archived";
  publishTime: string | null;
  viewCount: number;
  isTop: boolean;
  isRecommend: boolean;
  author: string;
  createdAt: string;
  updatedAt: string;
}

export interface CmsContentDetail {
  id: number;
  modelId: number;
  modelName: string;
  categoryId: number;
  categoryName: string;
  title: string;
  subtitle: string | null;
  thumbnail: string | null;
  content: string;
  summary: string | null;
  author: string | null;
  source: string | null;
  status: "draft" | "published" | "unpublished" | "archived";
  publishTime: string | null;
  viewCount: number;
  isTop: boolean;
  isRecommend: boolean;
  tags: string[];
  seoTitle: string | null;
  seoKeywords: string | null;
  seoDescription: string | null;
  createdAt: string;
  updatedAt: string;
  createdBy: string;
  updatedBy: string;
}

export const contentList: CmsContentList[] = [
  {
    id: 1,
    modelId: 1,
    modelName: "文章",
    categoryId: 2,
    categoryName: "公司新闻",
    title: "公司成功完成A轮融资，估值突破10亿元",
    thumbnail: "/uploads/2024/01/finance-news.jpg",
    status: "published",
    publishTime: "2024-01-15 10:00:00",
    viewCount: 1520,
    isTop: true,
    isRecommend: true,
    author: "张三",
    createdAt: "2024-01-15 08:30:00",
    updatedAt: "2024-01-15 10:00:00"
  },
  {
    id: 2,
    modelId: 1,
    modelName: "文章",
    categoryId: 2,
    categoryName: "公司新闻",
    title: "我司荣获2024年度最佳创新企业奖",
    thumbnail: "/uploads/2024/01/award.jpg",
    status: "published",
    publishTime: "2024-01-20 14:30:00",
    viewCount: 890,
    isTop: false,
    isRecommend: true,
    author: "李四",
    createdAt: "2024-01-20 10:00:00",
    updatedAt: "2024-01-20 14:30:00"
  },
  {
    id: 3,
    modelId: 1,
    modelName: "文章",
    categoryId: 3,
    categoryName: "行业动态",
    title: "2024年人工智能行业发展趋势分析报告",
    thumbnail: "/uploads/2024/02/ai-trend.jpg",
    status: "published",
    publishTime: "2024-02-01 09:00:00",
    viewCount: 2350,
    isTop: true,
    isRecommend: true,
    author: "王五",
    createdAt: "2024-02-01 08:00:00",
    updatedAt: "2024-02-01 09:00:00"
  },
  {
    id: 4,
    modelId: 1,
    modelName: "文章",
    categoryId: 3,
    categoryName: "行业动态",
    title: "云计算市场格局变化：巨头竞争加剧",
    thumbnail: "/uploads/2024/02/cloud-market.jpg",
    status: "published",
    publishTime: "2024-02-10 11:00:00",
    viewCount: 680,
    isTop: false,
    isRecommend: false,
    author: "赵六",
    createdAt: "2024-02-10 09:00:00",
    updatedAt: "2024-02-10 11:00:00"
  },
  {
    id: 5,
    modelId: 1,
    modelName: "文章",
    categoryId: 4,
    categoryName: "技术分享",
    title: "Rust语言在系统编程中的优势与实践",
    thumbnail: "/uploads/2024/02/rust-programming.jpg",
    status: "published",
    publishTime: "2024-02-15 15:00:00",
    viewCount: 1890,
    isTop: false,
    isRecommend: true,
    author: "孙七",
    createdAt: "2024-02-15 12:00:00",
    updatedAt: "2024-02-15 15:00:00"
  },
  {
    id: 6,
    modelId: 1,
    modelName: "文章",
    categoryId: 4,
    categoryName: "技术分享",
    title: "Vue 3 Composition API最佳实践指南",
    thumbnail: "/uploads/2024/02/vue3-guide.jpg",
    status: "draft",
    publishTime: null,
    viewCount: 0,
    isTop: false,
    isRecommend: false,
    author: "周八",
    createdAt: "2024-02-20 10:00:00",
    updatedAt: "2024-02-20 10:00:00"
  },
  {
    id: 7,
    modelId: 2,
    modelName: "产品",
    categoryId: 5,
    categoryName: "软件产品",
    title: "企业级内容管理系统 v3.0",
    thumbnail: "/uploads/2024/01/cms-product.jpg",
    status: "published",
    publishTime: "2024-01-10 10:00:00",
    viewCount: 3200,
    isTop: true,
    isRecommend: true,
    author: "产品部",
    createdAt: "2024-01-10 08:00:00",
    updatedAt: "2024-01-10 10:00:00"
  },
  {
    id: 8,
    modelId: 2,
    modelName: "产品",
    categoryId: 5,
    categoryName: "软件产品",
    title: "智能数据分析平台",
    thumbnail: "/uploads/2024/01/data-platform.jpg",
    status: "published",
    publishTime: "2024-01-12 14:00:00",
    viewCount: 2100,
    isTop: false,
    isRecommend: true,
    author: "产品部",
    createdAt: "2024-01-12 10:00:00",
    updatedAt: "2024-01-12 14:00:00"
  },
  {
    id: 9,
    modelId: 2,
    modelName: "产品",
    categoryId: 6,
    categoryName: "硬件产品",
    title: "智能物联网网关设备",
    thumbnail: "/uploads/2024/02/iot-gateway.jpg",
    status: "published",
    publishTime: "2024-02-05 09:00:00",
    viewCount: 1560,
    isTop: false,
    isRecommend: false,
    author: "硬件部",
    createdAt: "2024-02-05 08:00:00",
    updatedAt: "2024-02-05 09:00:00"
  },
  {
    id: 10,
    modelId: 1,
    modelName: "文章",
    categoryId: 2,
    categoryName: "公司新闻",
    title: "公司年度总结大会顺利召开",
    thumbnail: "/uploads/2024/02/annual-meeting.jpg",
    status: "unpublished",
    publishTime: null,
    viewCount: 0,
    isTop: false,
    isRecommend: false,
    author: "行政部",
    createdAt: "2024-02-25 16:00:00",
    updatedAt: "2024-02-25 16:00:00"
  },
  {
    id: 11,
    modelId: 1,
    modelName: "文章",
    categoryId: 3,
    categoryName: "行业动态",
    title: "区块链技术在供应链管理中的应用前景",
    thumbnail: "/uploads/2024/03/blockchain-supply.jpg",
    status: "published",
    publishTime: "2024-03-01 10:00:00",
    viewCount: 920,
    isTop: false,
    isRecommend: false,
    author: "研究部",
    createdAt: "2024-03-01 08:00:00",
    updatedAt: "2024-03-01 10:00:00"
  },
  {
    id: 12,
    modelId: 1,
    modelName: "文章",
    categoryId: 4,
    categoryName: "技术分享",
    title: "微服务架构设计模式与实践",
    thumbnail: "/uploads/2024/03/microservice.jpg",
    status: "archived",
    publishTime: "2024-01-05 11:00:00",
    viewCount: 450,
    isTop: false,
    isRecommend: false,
    author: "架构组",
    createdAt: "2024-01-05 09:00:00",
    updatedAt: "2024-03-05 10:00:00"
  }
];

export const contentDetail: CmsContentDetail = {
  id: 1,
  modelId: 1,
  modelName: "文章",
  categoryId: 2,
  categoryName: "公司新闻",
  title: "公司成功完成A轮融资，估值突破10亿元",
  subtitle: "开启新一轮发展征程",
  thumbnail: "/uploads/2024/01/finance-news.jpg",
  content: `<h2>融资概况</h2>
<p>2024年1月15日，我司正式宣布完成A轮融资，融资金额达2亿元人民币，公司估值突破10亿元大关。本轮融资由知名投资机构领投，多家知名投资机构跟投。</p>

<h2>资金用途</h2>
<p>本轮融资资金将主要用于以下几个方面：</p>
<ul>
  <li>产品研发：加大研发投入，提升产品核心竞争力</li>
  <li>市场拓展：扩大市场份额，开拓海外市场</li>
  <li>团队建设：引进高端人才，打造一流团队</li>
  <li>基础设施：完善基础设施建设，提升服务能力</li>
</ul>

<h2>未来展望</h2>
<p>公司CEO表示："本轮融资的完成标志着公司进入了一个新的发展阶段。我们将继续秉承'技术创新、服务至上'的理念，为客户提供更优质的产品和服务。"</p>

<p>未来，公司将继续深耕企业级软件市场，致力于成为行业领先的企业数字化解决方案提供商。</p>`,
  summary:
    "2024年1月15日，我司正式宣布完成A轮融资，融资金额达2亿元人民币，公司估值突破10亿元大关。本轮融资将用于产品研发、市场拓展、团队建设等方面。",
  author: "张三",
  source: "公司官网",
  status: "published",
  publishTime: "2024-01-15 10:00:00",
  viewCount: 1520,
  isTop: true,
  isRecommend: true,
  tags: ["融资", "公司新闻", "企业发展"],
  seoTitle: "公司完成A轮融资，估值突破10亿元 - 公司新闻",
  seoKeywords: "A轮融资,企业融资,公司估值,企业发展",
  seoDescription: "我司成功完成A轮融资，融资金额达2亿元人民币，公司估值突破10亿元，开启新一轮发展征程。",
  createdAt: "2024-01-15 08:30:00",
  updatedAt: "2024-01-15 10:00:00",
  createdBy: "张三",
  updatedBy: "张三"
};
