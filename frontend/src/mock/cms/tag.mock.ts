export interface CmsTagItem {
  id: number;
  name: string;
  code: string;
  color: string;
  description: string | null;
  contentCount: number;
  status: boolean;
  createdAt: string;
  updatedAt: string;
}

export const tagList: CmsTagItem[] = [
  {
    id: 1,
    name: '热门',
    code: 'hot',
    color: '#f56c6c',
    description: '热门内容标签',
    contentCount: 50,
    status: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 2,
    name: '推荐',
    code: 'recommend',
    color: '#67c23a',
    description: '推荐内容标签',
    contentCount: 30,
    status: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 3,
    name: '置顶',
    code: 'top',
    color: '#e6a23c',
    description: '置顶内容标签',
    contentCount: 15,
    status: true,
    createdAt: '2024-01-02 00:00:00',
    updatedAt: '2024-01-02 00:00:00'
  },
  {
    id: 4,
    name: '精华',
    code: 'featured',
    color: '#409eff',
    description: '精华内容标签',
    contentCount: 25,
    status: true,
    createdAt: '2024-01-02 00:00:00',
    updatedAt: '2024-01-02 00:00:00'
  },
  {
    id: 5,
    name: '新品',
    code: 'new',
    color: '#909399',
    description: '新品发布标签',
    contentCount: 18,
    status: true,
    createdAt: '2024-01-03 00:00:00',
    updatedAt: '2024-01-03 00:00:00'
  },
  {
    id: 6,
    name: '技术',
    code: 'tech',
    color: '#00d2d3',
    description: '技术相关内容',
    contentCount: 45,
    status: true,
    createdAt: '2024-01-03 00:00:00',
    updatedAt: '2024-01-03 00:00:00'
  },
  {
    id: 7,
    name: '教程',
    code: 'tutorial',
    color: '#a29bfe',
    description: '教程类内容',
    contentCount: 32,
    status: true,
    createdAt: '2024-01-04 00:00:00',
    updatedAt: '2024-01-04 00:00:00'
  },
  {
    id: 8,
    name: '案例',
    code: 'case',
    color: '#fd79a8',
    description: '案例展示标签',
    contentCount: 20,
    status: true,
    createdAt: '2024-01-04 00:00:00',
    updatedAt: '2024-01-04 00:00:00'
  },
  {
    id: 9,
    name: '行业',
    code: 'industry',
    color: '#55a3ff',
    description: '行业资讯标签',
    contentCount: 38,
    status: true,
    createdAt: '2024-01-05 00:00:00',
    updatedAt: '2024-01-05 00:00:00'
  },
  {
    id: 10,
    name: '产品',
    code: 'product',
    color: '#26de81',
    description: '产品相关标签',
    contentCount: 28,
    status: true,
    createdAt: '2024-01-05 00:00:00',
    updatedAt: '2024-01-05 00:00:00'
  },
  {
    id: 11,
    name: '人工智能',
    code: 'ai',
    color: '#eb3b5a',
    description: 'AI相关内容',
    contentCount: 22,
    status: true,
    createdAt: '2024-01-06 00:00:00',
    updatedAt: '2024-01-06 00:00:00'
  },
  {
    id: 12,
    name: '云计算',
    code: 'cloud',
    color: '#45aaf2',
    description: '云计算相关内容',
    contentCount: 16,
    status: true,
    createdAt: '2024-01-06 00:00:00',
    updatedAt: '2024-01-06 00:00:00'
  },
  {
    id: 13,
    name: '大数据',
    code: 'bigdata',
    color: '#2bcbba',
    description: '大数据相关内容',
    contentCount: 14,
    status: true,
    createdAt: '2024-01-07 00:00:00',
    updatedAt: '2024-01-07 00:00:00'
  },
  {
    id: 14,
    name: '物联网',
    code: 'iot',
    color: '#fed330',
    description: '物联网相关内容',
    contentCount: 12,
    status: true,
    createdAt: '2024-01-07 00:00:00',
    updatedAt: '2024-01-07 00:00:00'
  },
  {
    id: 15,
    name: '区块链',
    code: 'blockchain',
    color: '#fc5c65',
    description: '区块链相关内容',
    contentCount: 8,
    status: false,
    createdAt: '2024-01-08 00:00:00',
    updatedAt: '2024-01-08 00:00:00'
  }
];

export const tagCloud: CmsTagItem[] = [
  {
    id: 1,
    name: '热门',
    code: 'hot',
    color: '#f56c6c',
    description: '热门内容标签',
    contentCount: 50,
    status: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 6,
    name: '技术',
    code: 'tech',
    color: '#00d2d3',
    description: '技术相关内容',
    contentCount: 45,
    status: true,
    createdAt: '2024-01-03 00:00:00',
    updatedAt: '2024-01-03 00:00:00'
  },
  {
    id: 9,
    name: '行业',
    code: 'industry',
    color: '#55a3ff',
    description: '行业资讯标签',
    contentCount: 38,
    status: true,
    createdAt: '2024-01-05 00:00:00',
    updatedAt: '2024-01-05 00:00:00'
  },
  {
    id: 7,
    name: '教程',
    code: 'tutorial',
    color: '#a29bfe',
    description: '教程类内容',
    contentCount: 32,
    status: true,
    createdAt: '2024-01-04 00:00:00',
    updatedAt: '2024-01-04 00:00:00'
  },
  {
    id: 2,
    name: '推荐',
    code: 'recommend',
    color: '#67c23a',
    description: '推荐内容标签',
    contentCount: 30,
    status: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 10,
    name: '产品',
    code: 'product',
    color: '#26de81',
    description: '产品相关标签',
    contentCount: 28,
    status: true,
    createdAt: '2024-01-05 00:00:00',
    updatedAt: '2024-01-05 00:00:00'
  },
  {
    id: 4,
    name: '精华',
    code: 'featured',
    color: '#409eff',
    description: '精华内容标签',
    contentCount: 25,
    status: true,
    createdAt: '2024-01-02 00:00:00',
    updatedAt: '2024-01-02 00:00:00'
  },
  {
    id: 11,
    name: '人工智能',
    code: 'ai',
    color: '#eb3b5a',
    description: 'AI相关内容',
    contentCount: 22,
    status: true,
    createdAt: '2024-01-06 00:00:00',
    updatedAt: '2024-01-06 00:00:00'
  },
  {
    id: 8,
    name: '案例',
    code: 'case',
    color: '#fd79a8',
    description: '案例展示标签',
    contentCount: 20,
    status: true,
    createdAt: '2024-01-04 00:00:00',
    updatedAt: '2024-01-04 00:00:00'
  },
  {
    id: 5,
    name: '新品',
    code: 'new',
    color: '#909399',
    description: '新品发布标签',
    contentCount: 18,
    status: true,
    createdAt: '2024-01-03 00:00:00',
    updatedAt: '2024-01-03 00:00:00'
  }
];
