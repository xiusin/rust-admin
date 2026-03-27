import type { MockMethod } from "vite-plugin-mock";
import { resultSuccess, resultError } from "../_utils";

const categories = [
  {
    id: 1,
    name: '营销工具',
    icon: 'icon-promotion',
    parentId: 0,
    sort: 1,
    pluginCount: 25,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 11, name: '优惠券', icon: 'icon-coupon', parentId: 1, sort: 1, pluginCount: 8, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 12, name: '秒杀', icon: 'icon-seckill', parentId: 1, sort: 2, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 13, name: '拼团', icon: 'icon-group', parentId: 1, sort: 3, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 14, name: '抽奖', icon: 'icon-lottery', parentId: 1, sort: 4, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 2,
    name: '客服系统',
    icon: 'icon-customer',
    parentId: 0,
    sort: 2,
    pluginCount: 18,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 21, name: '在线客服', icon: 'icon-chat', parentId: 2, sort: 1, pluginCount: 8, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 22, name: '工单系统', icon: 'icon-workorder', parentId: 2, sort: 2, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 23, name: '呼叫中心', icon: 'icon-call', parentId: 2, sort: 3, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 3,
    name: '数据分析',
    icon: 'icon-chart',
    parentId: 0,
    sort: 3,
    pluginCount: 15,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 31, name: '数据统计', icon: 'icon-statistics', parentId: 3, sort: 1, pluginCount: 8, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 32, name: '用户分析', icon: 'icon-user', parentId: 3, sort: 2, pluginCount: 4, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 33, name: '报表中心', icon: 'icon-report', parentId: 3, sort: 3, pluginCount: 3, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 4,
    name: '支付收款',
    icon: 'icon-payment',
    parentId: 0,
    sort: 4,
    pluginCount: 12,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 41, name: '支付通道', icon: 'icon-gateway', parentId: 4, sort: 1, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 42, name: '账务管理', icon: 'icon-account', parentId: 4, sort: 2, pluginCount: 4, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 43, name: '对账结算', icon: 'icon-reconcile', parentId: 4, sort: 3, pluginCount: 2, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 5,
    name: '内容运营',
    icon: 'icon-content',
    parentId: 0,
    sort: 5,
    pluginCount: 20,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 51, name: 'CMS', icon: 'icon-cms', parentId: 5, sort: 1, pluginCount: 8, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 52, name: '文章管理', icon: 'icon-article', parentId: 5, sort: 2, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 53, name: '评论系统', icon: 'icon-comment', parentId: 5, sort: 3, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 6,
    name: '物流管理',
    icon: 'icon-logistics',
    parentId: 0,
    sort: 6,
    pluginCount: 16,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 61, name: '快递查询', icon: 'icon-express', parentId: 6, sort: 1, pluginCount: 6, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 62, name: '打印面单', icon: 'icon-print', parentId: 6, sort: 2, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 63, name: '仓储管理', icon: 'icon-warehouse', parentId: 6, sort: 3, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 7,
    name: '会员管理',
    icon: 'icon-member',
    parentId: 0,
    sort: 7,
    pluginCount: 14,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 71, name: '会员体系', icon: 'icon-vip', parentId: 7, sort: 1, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 72, name: '积分系统', icon: 'icon-point', parentId: 7, sort: 2, pluginCount: 4, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 73, name: '等级权益', icon: 'icon-level', parentId: 7, sort: 3, pluginCount: 5, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
  {
    id: 8,
    name: '企业微信',
    icon: 'icon-wework',
    parentId: 0,
    sort: 8,
    pluginCount: 10,
    status: 1,
    createdAt: '2024-01-01T00:00:00Z',
    children: [
      { id: 81, name: '企微助手', icon: 'icon-assistant', parentId: 8, sort: 1, pluginCount: 4, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 82, name: '裂变工具', icon: 'icon-fission', parentId: 8, sort: 2, pluginCount: 3, status: 1, createdAt: '2024-01-01T00:00:00Z' },
      { id: 83, name: '客户管理', icon: 'icon-crm', parentId: 8, sort: 3, pluginCount: 3, status: 1, createdAt: '2024-01-01T00:00:00Z' },
    ],
  },
];

const developers = [
  {
    id: 1,
    userId: 1,
    name: '奇络科技',
    logo: 'https://picsum.photos/100/100?random=10',
    contact: 'contact@qiluo.com',
    phone: '138****8888',
    email: 'dev@qiluo.com',
    description: '专业的企业级插件开发商，致力于为商户提供高质量的商业解决方案。我们拥有丰富的电商系统开发经验，已服务超过10000+商户。',
    pluginCount: 15,
    totalSales: 125680,
    totalDownloads: 45600,
    rating: 4.9,
    status: 1,
    verified: true,
    verifiedAt: '2024-01-15T10:00:00Z',
    bankName: '招商银行',
    bankAccount: '**** **** **** 8888',
    balance: 56800.50,
    frozenBalance: 0,
    totalIncome: 125680.50,
    withdrawnAmount: 68880.00,
    createdAt: '2024-01-01T00:00:00Z',
    updatedAt: '2024-03-25T10:00:00Z',
  },
  {
    id: 2,
    userId: 2,
    name: '云商科技',
    logo: 'https://picsum.photos/100/100?random=20',
    contact: 'contact@yunshang.com',
    phone: '139****9999',
    email: 'dev@yunshang.com',
    description: '专注电商领域插件开发，提供营销、客服、数据分析等全方位解决方案。',
    pluginCount: 8,
    totalSales: 45600,
    totalDownloads: 12300,
    rating: 4.7,
    status: 1,
    verified: true,
    verifiedAt: '2024-01-20T14:00:00Z',
    bankName: '工商银行',
    bankAccount: '**** **** **** 6666',
    balance: 23450.00,
    frozenBalance: 0,
    totalIncome: 45600.00,
    withdrawnAmount: 22150.00,
    createdAt: '2024-01-15T00:00:00Z',
    updatedAt: '2024-03-20T15:30:00Z',
  },
  {
    id: 3,
    userId: 3,
    name: '通讯先锋',
    logo: 'https://picsum.photos/100/100?random=30',
    contact: 'support@tongxun.com',
    phone: '137****7777',
    email: 'dev@tongxun.com',
    description: '专注短信、通知类插件开发，提供稳定可靠的通讯解决方案。',
    pluginCount: 5,
    totalSales: 28900,
    totalDownloads: 8600,
    rating: 4.6,
    status: 1,
    verified: false,
    verifiedAt: null,
    bankName: '建设银行',
    bankAccount: '**** **** **** 5555',
    balance: 15600.00,
    frozenBalance: 0,
    totalIncome: 28900.00,
    withdrawnAmount: 13300.00,
    createdAt: '2024-02-01T00:00:00Z',
    updatedAt: '2024-03-15T09:20:00Z',
  },
  {
    id: 4,
    userId: 4,
    name: 'AI智能科技',
    logo: 'https://picsum.photos/100/100?random=40',
    contact: 'ai@aitech.com',
    phone: '136****6666',
    email: 'dev@aitech.com',
    description: '基于大模型的智能解决方案提供商，专注于AI客服、智能推荐等领域。',
    pluginCount: 3,
    totalSales: 89500,
    totalDownloads: 15200,
    rating: 4.9,
    status: 1,
    verified: true,
    verifiedAt: '2024-02-10T11:00:00Z',
    bankName: '中国银行',
    bankAccount: '**** **** **** 4444',
    balance: 45600.00,
    frozenBalance: 0,
    totalIncome: 89500.00,
    withdrawnAmount: 43900.00,
    createdAt: '2024-03-01T00:00:00Z',
    updatedAt: '2024-03-25T14:00:00Z',
  },
];

const plugins = [
  {
    id: 1,
    code: 'plugin_coupon',
    name: '智能优惠券',
    categoryId: 11,
    categoryName: '优惠券',
    parentCategoryName: '营销工具',
    developerId: 1,
    developerName: '奇络科技',
    developerLogo: 'https://picsum.photos/100/100?random=10',
    developerVerified: true,
    summary: '多场景优惠券发放与核销，支持满减、折扣、新人专享等多种类型，助力商家精准营销',
    description: `## 功能特性

### 多种优惠券类型
- **满减券**：满100减20，灵活设置门槛
- **折扣券**：全场8折，支持指定商品
- **现金券**：直接抵扣，无门槛使用
- **新人券**：新用户专享，提升转化
- **会员券**：会员专属，增强粘性

### 发放方式
- 批量发放：支持Excel导入用户列表
- 活动领取：配置活动页面自动领取
- 码券兑换：生成兑换码线下发放
- 分享裂变：分享后获得优惠券

### 核销管理
- 扫码核销：支持微信/支付宝扫码
- 手动核销：输入券码核销
- 自动核销：订单支付自动使用

### 数据统计
- 发放统计：发放量、领取量、使用量
- 转化分析：核销率、复购率、ROI
- 用户画像：领券用户分析`,
    coverImage: 'https://picsum.photos/400/300?random=1',
    screenshots: [
      'https://picsum.photos/800/600?random=11',
      'https://picsum.photos/800/600?random=12',
      'https://picsum.photos/800/600?random=13',
      'https://picsum.photos/800/600?random=14',
    ],
    version: '2.1.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 99,
    originalPrice: 199,
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    rating: 4.8,
    reviewCount: 128,
    downloadCount: 2560,
    activeUsers: 1856,
    status: 1,
    statusName: '上架',
    tags: ['官方', '热门', '稳定'],
    isOfficial: 1,
    minAppVersion: '1.0.0',
    updatedAt: '2024-03-01T15:20:00Z',
    createdAt: '2024-01-15T10:30:00Z',
  },
  {
    id: 2,
    code: 'plugin_seckill',
    name: '限时秒杀',
    categoryId: 12,
    categoryName: '秒杀',
    parentCategoryName: '营销工具',
    developerId: 2,
    developerName: '云商科技',
    developerLogo: 'https://picsum.photos/100/100?random=20',
    developerVerified: true,
    summary: '专业的秒杀活动工具，支持预热提醒、库存锁定、倒计时等功能，万级QPS高并发支持',
    description: `## 功能特性

### 活动预热
- 预告展示：活动开始前展示预告页
- 预约提醒：用户预约，开始前短信/推送提醒
- 倒计时：精确到秒的倒计时显示
- 分享裂变：分享活动获取优先抢购权

### 高并发支持
- 库存预热：活动开始前预热库存到Redis
- 限流控制：令牌桶算法控制请求流量
- 队列削峰：异步处理订单请求
- 分布式锁：防止超卖问题

### 数据监控
- 实时大屏：活动数据实时展示
- 流量监控：QPS、响应时间监控
- 库存预警：库存不足自动告警
- 异常检测：自动识别异常行为`,
    coverImage: 'https://picsum.photos/400/300?random=2',
    screenshots: [
      'https://picsum.photos/800/600?random=21',
      'https://picsum.photos/800/600?random=22',
      'https://picsum.photos/800/600?random=23',
    ],
    version: '1.5.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 199,
    originalPrice: 299,
    verifyLevel: 2,
    verifyLevelName: '高级验证',
    rating: 4.9,
    reviewCount: 86,
    downloadCount: 1890,
    activeUsers: 1456,
    status: 1,
    statusName: '上架',
    tags: ['热门', '高评分', '高并发'],
    isOfficial: 0,
    minAppVersion: '1.2.0',
    updatedAt: '2024-03-10T09:15:00Z',
    createdAt: '2024-02-20T14:20:00Z',
  },
  {
    id: 3,
    code: 'plugin_qrcode',
    name: '二维码生成器',
    categoryId: 51,
    categoryName: 'CMS',
    parentCategoryName: '内容运营',
    developerId: 1,
    developerName: '奇络科技',
    developerLogo: 'https://picsum.photos/100/100?random=10',
    developerVerified: true,
    summary: '支持生成各种类型的二维码，包括URL、文本、名片、WiFi等，支持批量生成和样式自定义',
    description: `## 功能特性

### 多种二维码类型
- **URL二维码**：网址链接直接扫码访问
- **文本二维码**：任意文本内容编码
- **名片二维码**：vCard格式，扫码即存通讯录
- **WiFi二维码**：扫码直接连接WiFi
- **支付二维码**：微信/支付宝收款码

### 样式自定义
- 颜色设置：前景色、背景色自定义
- Logo嵌入：中心嵌入Logo图片
- 边框样式：多种边框样式可选
- 尺寸调整：支持多种尺寸输出

### 批量生成
- Excel导入：批量导入数据生成
- 模板配置：预设模板快速生成
- 批量导出：打包下载所有二维码`,
    coverImage: 'https://picsum.photos/400/300?random=3',
    screenshots: [
      'https://picsum.photos/800/600?random=31',
      'https://picsum.photos/800/600?random=32',
    ],
    version: '1.0.0',
    priceType: 0,
    priceTypeName: '免费',
    price: 0,
    originalPrice: 0,
    verifyLevel: 0,
    verifyLevelName: '无验证',
    rating: 4.5,
    reviewCount: 256,
    downloadCount: 5600,
    activeUsers: 4200,
    status: 1,
    statusName: '上架',
    tags: ['免费', '实用', '官方'],
    isOfficial: 1,
    minAppVersion: '1.0.0',
    updatedAt: '2024-03-01T09:00:00Z',
    createdAt: '2024-03-01T09:00:00Z',
  },
  {
    id: 4,
    code: 'plugin_sms',
    name: '短信通知',
    categoryId: 21,
    categoryName: '在线客服',
    parentCategoryName: '客服系统',
    developerId: 3,
    developerName: '通讯先锋',
    developerLogo: 'https://picsum.photos/100/100?random=30',
    developerVerified: false,
    summary: '支持短信验证码、通知短信、营销短信等多种发送场景，智能路由自动选择最优通道',
    description: `## 功能特性

### 多种短信类型
- **验证码短信**：用于注册、登录、找回密码
- **通知短信**：订单通知、物流通知、系统通知
- **营销短信**：促销活动、会员关怀、新品推荐

### 智能路由
- 多通道支持：阿里云、腾讯云、华为云等
- 自动切换：通道异常自动切换备用通道
- 智能选择：根据运营商自动选择最优通道

### 模板管理
- 变量支持：支持动态变量替换
- 审核管理：模板审核状态跟踪
- 签名管理：多签名支持`,
    coverImage: 'https://picsum.photos/400/300?random=4',
    screenshots: [
      'https://picsum.photos/800/600?random=41',
      'https://picsum.photos/800/600?random=42',
    ],
    version: '3.2.0',
    priceType: 1,
    priceTypeName: '一次性',
    price: 599,
    originalPrice: 999,
    verifyLevel: 2,
    verifyLevelName: '高级验证',
    rating: 4.6,
    reviewCount: 64,
    downloadCount: 1230,
    activeUsers: 986,
    status: 1,
    statusName: '上架',
    tags: ['稳定', '高性价比', '通讯'],
    isOfficial: 0,
    minAppVersion: '1.0.0',
    updatedAt: '2024-02-28T16:30:00Z',
    createdAt: '2024-01-08T11:15:00Z',
  },
  {
    id: 5,
    code: 'plugin_analysis',
    name: '数据统计分析',
    categoryId: 31,
    categoryName: '数据统计',
    parentCategoryName: '数据分析',
    developerId: 1,
    developerName: '奇络科技',
    developerLogo: 'https://picsum.photos/100/100?random=10',
    developerVerified: true,
    summary: '全方位数据统计分析，支持自定义报表、数据可视化、多维度分析，助力数据驱动决策',
    description: `## 功能特性

### 多维度分析
- **用户分析**：新增用户、活跃用户、留存分析
- **订单分析**：订单量、成交额、转化漏斗
- **商品分析**：销量排行、库存预警、动销率
- **渠道分析**：来源渠道、渠道效果对比

### 自定义报表
- 拖拽配置：可视化拖拽配置报表
- 定时推送：报表定时发送到邮箱
- 导出功能：支持Excel、PDF导出

### 数据可视化
- 多种图表：折线图、柱状图、饼图、漏斗图
- 实时大屏：数据实时更新展示
- 仪表盘：自定义数据仪表盘`,
    coverImage: 'https://picsum.photos/400/300?random=5',
    screenshots: [
      'https://picsum.photos/800/600?random=51',
      'https://picsum.photos/800/600?random=52',
      'https://picsum.photos/800/600?random=53',
    ],
    version: '2.0.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 399,
    originalPrice: 599,
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    rating: 4.7,
    reviewCount: 92,
    downloadCount: 980,
    activeUsers: 756,
    status: 1,
    statusName: '上架',
    tags: ['官方', '数据', '可视化'],
    isOfficial: 1,
    minAppVersion: '1.5.0',
    updatedAt: '2024-03-05T11:20:00Z',
    createdAt: '2024-02-12T16:45:00Z',
  },
  {
    id: 6,
    code: 'plugin_robot',
    name: 'AI智能客服',
    categoryId: 21,
    categoryName: '在线客服',
    parentCategoryName: '客服系统',
    developerId: 4,
    developerName: 'AI智能科技',
    developerLogo: 'https://picsum.photos/100/100?random=40',
    developerVerified: true,
    summary: '基于大模型的智能客服系统，支持多轮对话、意图识别、知识库管理，实现7x24小时自动服务',
    description: `## 功能特性

### 智能对话
- **多轮对话**：支持上下文理解的多轮对话
- **意图识别**：自动识别用户意图，精准响应
- **情感分析**：识别用户情绪，智能转人工
- **多语言支持**：支持中英文等多语言

### 知识库管理
- 问答配置：常见问题快速配置
- 知识图谱：构建知识关联网络
- 自动学习：从对话中自动学习

### 人机协作
- 智能转人工：复杂问题自动转人工
- 会话记录：完整会话历史记录
- 效率统计：客服效率数据分析`,
    coverImage: 'https://picsum.photos/400/300?random=6',
    screenshots: [
      'https://picsum.photos/800/600?random=61',
      'https://picsum.photos/800/600?random=62',
    ],
    version: '1.0.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 799,
    originalPrice: 1299,
    verifyLevel: 2,
    verifyLevelName: '高级验证',
    rating: 4.9,
    reviewCount: 156,
    downloadCount: 2100,
    activeUsers: 1680,
    status: 1,
    statusName: '上架',
    tags: ['AI', '热门', '智能', '高评分'],
    isOfficial: 0,
    minAppVersion: '1.8.0',
    updatedAt: '2024-03-15T08:30:00Z',
    createdAt: '2024-03-15T08:30:00Z',
  },
  {
    id: 7,
    code: 'plugin_group_buy',
    name: '拼团活动',
    categoryId: 13,
    categoryName: '拼团',
    parentCategoryName: '营销工具',
    developerId: 2,
    developerName: '云商科技',
    developerLogo: 'https://picsum.photos/100/100?random=20',
    developerVerified: true,
    summary: '支持多人拼团、老带新团、阶梯团等多种拼团模式，助力社交裂变增长',
    description: `## 功能特性

### 多种拼团模式
- **普通拼团**：多人成团享受优惠价
- **老带新团**：老用户邀新用户成团
- **阶梯团**：人数越多价格越低
- **抽奖团**：拼团成功参与抽奖

### 社交裂变
- 分享激励：分享获得额外优惠
- 邀请奖励：邀请好友参团奖励
- 拼团排行：拼团成功排行榜

### 数据分析
- 成团率分析：拼团成功率统计
- 用户画像：参团用户分析
- 效果追踪：活动ROI分析`,
    coverImage: 'https://picsum.photos/400/300?random=7',
    screenshots: [
      'https://picsum.photos/800/600?random=71',
      'https://picsum.photos/800/600?random=72',
    ],
    version: '1.2.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 149,
    originalPrice: 249,
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    rating: 4.6,
    reviewCount: 78,
    downloadCount: 1650,
    activeUsers: 1280,
    status: 1,
    statusName: '上架',
    tags: ['社交', '裂变', '营销'],
    isOfficial: 0,
    minAppVersion: '1.0.0',
    updatedAt: '2024-03-12T14:00:00Z',
    createdAt: '2024-02-25T10:30:00Z',
  },
  {
    id: 8,
    code: 'plugin_lottery',
    name: '幸运抽奖',
    categoryId: 14,
    categoryName: '抽奖',
    parentCategoryName: '营销工具',
    developerId: 1,
    developerName: '奇络科技',
    developerLogo: 'https://picsum.photos/100/100?random=10',
    developerVerified: true,
    summary: '支持大转盘、九宫格、刮刮卡等多种抽奖形式，丰富的奖品配置和防刷机制',
    description: `## 功能特性

### 多种抽奖形式
- **大转盘**：经典转盘抽奖
- **九宫格**：九宫格翻牌抽奖
- **刮刮卡**：模拟刮奖体验
- **砸金蛋**：趣味砸蛋抽奖

### 奖品配置
- 实物奖品：配置实物奖品发放
- 虚拟奖品：优惠券、积分、会员等
- 概率设置：灵活设置中奖概率
- 库存管理：奖品库存实时管理

### 防刷机制
- 频率限制：限制抽奖频率
- 黑名单：异常用户黑名单
- 验证码：可疑操作需验证`,
    coverImage: 'https://picsum.photos/400/300?random=8',
    screenshots: [
      'https://picsum.photos/800/600?random=81',
      'https://picsum.photos/800/600?random=82',
    ],
    version: '1.3.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 129,
    originalPrice: 199,
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    rating: 4.5,
    reviewCount: 112,
    downloadCount: 2300,
    activeUsers: 1780,
    status: 1,
    statusName: '上架',
    tags: ['官方', '营销', '互动'],
    isOfficial: 1,
    minAppVersion: '1.0.0',
    updatedAt: '2024-03-08T16:45:00Z',
    createdAt: '2024-02-18T09:20:00Z',
  },
  {
    id: 9,
    code: 'plugin_express',
    name: '快递查询',
    categoryId: 61,
    categoryName: '快递查询',
    parentCategoryName: '物流管理',
    developerId: 2,
    developerName: '云商科技',
    developerLogo: 'https://picsum.photos/100/100?random=20',
    developerVerified: true,
    summary: '支持100+快递公司实时查询，自动识别快递单号，提供物流轨迹追踪和到达提醒',
    description: `## 功能特性

### 快递查询
- **实时查询**：对接快递100、快递鸟等接口
- **自动识别**：自动识别快递公司
- **批量查询**：支持批量单号查询
- **轨迹追踪**：完整物流轨迹展示

### 到达提醒
- 短信提醒：包裹到达短信通知
- 微信推送：微信模板消息推送
- 签收确认：签收后状态更新

### 数据统计
- 发货统计：发货量、快递公司分布
- 时效分析：平均配送时效
- 异常监控：异常包裹预警`,
    coverImage: 'https://picsum.photos/400/300?random=9',
    screenshots: [
      'https://picsum.photos/800/600?random=91',
    ],
    version: '2.0.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 79,
    originalPrice: 129,
    verifyLevel: 0,
    verifyLevelName: '无验证',
    rating: 4.4,
    reviewCount: 89,
    downloadCount: 3100,
    activeUsers: 2450,
    status: 1,
    statusName: '上架',
    tags: ['物流', '实用', '稳定'],
    isOfficial: 0,
    minAppVersion: '1.0.0',
    updatedAt: '2024-03-18T11:30:00Z',
    createdAt: '2024-01-20T14:00:00Z',
  },
  {
    id: 10,
    code: 'plugin_vip',
    name: '会员体系',
    categoryId: 71,
    categoryName: '会员体系',
    parentCategoryName: '会员管理',
    developerId: 1,
    developerName: '奇络科技',
    developerLogo: 'https://picsum.photos/100/100?random=10',
    developerVerified: true,
    summary: '完整的会员等级体系，支持成长值、积分、权益配置，提升用户粘性和复购率',
    description: `## 功能特性

### 等级体系
- **多级会员**：普通、银卡、金卡、钻石等
- **成长值**：消费、签到、互动获取成长值
- **自动升级**：成长值达标自动升级
- **保级机制**：年度消费保级

### 权益配置
- 专属折扣：不同等级不同折扣
- 积分倍率：高等级积分翻倍
- 专属客服：VIP专属客服通道
- 生日特权：生日专属优惠

### 积分系统
- 积分获取：消费、签到、分享
- 积分兑换：积分商城兑换
- 积分过期：积分有效期管理`,
    coverImage: 'https://picsum.photos/400/300?random=10',
    screenshots: [
      'https://picsum.photos/800/600?random=101',
      'https://picsum.photos/800/600?random=102',
    ],
    version: '1.5.0',
    priceType: 2,
    priceTypeName: '订阅',
    price: 299,
    originalPrice: 499,
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    rating: 4.8,
    reviewCount: 145,
    downloadCount: 1890,
    activeUsers: 1520,
    status: 1,
    statusName: '上架',
    tags: ['官方', '会员', '运营'],
    isOfficial: 1,
    minAppVersion: '1.3.0',
    updatedAt: '2024-03-20T09:15:00Z',
    createdAt: '2024-02-05T11:00:00Z',
  },
];

const plans = [
  {
    id: 1,
    pluginId: 1,
    name: '基础版',
    description: '适合个人开发者或小型店铺使用，满足基本优惠券管理需求',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 99,
    originalPrice: 99,
    discount: 0,
    features: [
      { code: 'basic', name: '基础功能', included: true, description: '包含优惠券创建、发放、核销等基础功能' },
      { code: 'template', name: '优惠券模板', included: true, limit: 5, description: '最多创建5个优惠券模板' },
      { code: 'batch', name: '批量发放', included: true, limit: 100, description: '单次最多发放100张' },
      { code: 'api', name: 'API接口', included: false, description: '不提供API接口' },
      { code: 'priority', name: '优先客服', included: false, description: '普通客服支持' },
      { code: 'custom', name: '定制开发', included: false, description: '不支持定制' },
    ],
    maxDevices: 1,
    maxUsers: 0,
    storageLimit: 100,
    apiCallsLimit: 0,
    supportLevel: 0,
    supportLevelName: '基础支持',
    recommended: false,
    sort: 1,
    status: 1,
  },
  {
    id: 2,
    pluginId: 1,
    name: '专业版',
    description: '适合中型商家使用，功能全面，性价比高',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 299,
    originalPrice: 399,
    discount: 100,
    features: [
      { code: 'basic', name: '基础功能', included: true, description: '包含所有基础功能' },
      { code: 'template', name: '优惠券模板', included: true, limit: -1, description: '无限优惠券模板' },
      { code: 'batch', name: '批量发放', included: true, limit: 1000, description: '单次最多发放1000张' },
      { code: 'api', name: 'API接口', included: true, limit: 10000, description: '每月10000次API调用' },
      { code: 'priority', name: '优先客服', included: true, description: '工作日4小时内响应' },
      { code: 'custom', name: '定制开发', included: false, description: '不支持定制' },
    ],
    maxDevices: 5,
    maxUsers: 0,
    storageLimit: 1024,
    apiCallsLimit: 10000,
    supportLevel: 1,
    supportLevelName: '高级支持',
    recommended: true,
    sort: 2,
    status: 1,
  },
  {
    id: 3,
    pluginId: 1,
    name: '企业版',
    description: '适合大型企业使用，全功能支持，专属服务',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 799,
    originalPrice: 999,
    discount: 200,
    features: [
      { code: 'basic', name: '基础功能', included: true, description: '包含所有功能' },
      { code: 'template', name: '优惠券模板', included: true, limit: -1, description: '无限优惠券模板' },
      { code: 'batch', name: '批量发放', included: true, limit: -1, description: '无限制批量发放' },
      { code: 'api', name: 'API接口', included: true, limit: -1, description: '无限API调用' },
      { code: 'priority', name: '专属客服', included: true, description: '专属客服经理，2小时内响应' },
      { code: 'custom', name: '定制开发', included: true, description: '支持功能定制开发' },
    ],
    maxDevices: -1,
    maxUsers: -1,
    storageLimit: -1,
    apiCallsLimit: -1,
    supportLevel: 2,
    supportLevelName: '专属客服',
    recommended: false,
    sort: 3,
    status: 1,
  },
  {
    id: 4,
    pluginId: 2,
    name: '基础版',
    description: '适合小型活动使用',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 199,
    originalPrice: 199,
    discount: 0,
    features: [
      { code: 'basic', name: '基础秒杀功能', included: true },
      { code: 'concurrent', name: '并发支持', included: true, limit: 1000, description: '支持1000 QPS' },
      { code: 'monitor', name: '实时监控', included: true },
    ],
    maxDevices: 1,
    maxUsers: 0,
    storageLimit: 100,
    apiCallsLimit: 0,
    supportLevel: 0,
    supportLevelName: '基础支持',
    recommended: false,
    sort: 1,
    status: 1,
  },
  {
    id: 5,
    pluginId: 2,
    name: '专业版',
    description: '适合中型活动，高并发支持',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 499,
    originalPrice: 699,
    discount: 200,
    features: [
      { code: 'basic', name: '基础秒杀功能', included: true },
      { code: 'concurrent', name: '并发支持', included: true, limit: 5000, description: '支持5000 QPS' },
      { code: 'monitor', name: '实时监控', included: true },
      { code: 'alert', name: '异常告警', included: true },
    ],
    maxDevices: 3,
    maxUsers: 0,
    storageLimit: 512,
    apiCallsLimit: 5000,
    supportLevel: 1,
    supportLevelName: '高级支持',
    recommended: true,
    sort: 2,
    status: 1,
  },
  {
    id: 6,
    pluginId: 2,
    name: '企业版',
    description: '适合大型活动，万级并发',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 999,
    originalPrice: 1499,
    discount: 500,
    features: [
      { code: 'basic', name: '基础秒杀功能', included: true },
      { code: 'concurrent', name: '并发支持', included: true, limit: 10000, description: '支持10000+ QPS' },
      { code: 'monitor', name: '实时监控', included: true },
      { code: 'alert', name: '异常告警', included: true },
      { code: 'custom', name: '定制开发', included: true },
    ],
    maxDevices: -1,
    maxUsers: -1,
    storageLimit: -1,
    apiCallsLimit: -1,
    supportLevel: 2,
    supportLevelName: '专属客服',
    recommended: false,
    sort: 3,
    status: 1,
  },
  {
    id: 7,
    pluginId: 3,
    name: '免费版',
    description: '免费使用基础功能',
    periodType: 3,
    periodTypeName: '永久',
    periodDays: -1,
    price: 0,
    originalPrice: 0,
    discount: 0,
    features: [
      { code: 'basic', name: '基础二维码生成', included: true },
      { code: 'template', name: '模板数量', included: true, limit: 10 },
      { code: 'batch', name: '批量生成', included: false },
      { code: 'stats', name: '扫码统计', included: false },
    ],
    maxDevices: 1,
    maxUsers: 0,
    storageLimit: 50,
    apiCallsLimit: 100,
    supportLevel: 0,
    supportLevelName: '社区支持',
    recommended: false,
    sort: 1,
    status: 1,
  },
  {
    id: 8,
    pluginId: 6,
    name: '体验版',
    description: '适合小型客服团队试用',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 299,
    originalPrice: 399,
    discount: 100,
    features: [
      { code: 'chat', name: '智能对话', included: true, limit: 1000, description: '每月1000次对话' },
      { code: 'knowledge', name: '知识库', included: true, limit: 100, description: '最多100条知识' },
      { code: 'transfer', name: '转人工', included: true },
    ],
    maxDevices: 1,
    maxUsers: 3,
    storageLimit: 100,
    apiCallsLimit: 1000,
    supportLevel: 0,
    supportLevelName: '基础支持',
    recommended: false,
    sort: 1,
    status: 1,
  },
  {
    id: 9,
    pluginId: 6,
    name: '标准版',
    description: '适合中型客服团队',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 799,
    originalPrice: 999,
    discount: 200,
    features: [
      { code: 'chat', name: '智能对话', included: true, limit: 10000, description: '每月10000次对话' },
      { code: 'knowledge', name: '知识库', included: true, limit: 500, description: '最多500条知识' },
      { code: 'transfer', name: '转人工', included: true },
      { code: 'analytics', name: '数据分析', included: true },
    ],
    maxDevices: 3,
    maxUsers: 10,
    storageLimit: 512,
    apiCallsLimit: 10000,
    supportLevel: 1,
    supportLevelName: '高级支持',
    recommended: true,
    sort: 2,
    status: 1,
  },
  {
    id: 10,
    pluginId: 6,
    name: '企业版',
    description: '适合大型客服中心',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 1999,
    originalPrice: 2999,
    discount: 1000,
    features: [
      { code: 'chat', name: '智能对话', included: true, limit: -1, description: '无限对话' },
      { code: 'knowledge', name: '知识库', included: true, limit: -1, description: '无限知识条数' },
      { code: 'transfer', name: '转人工', included: true },
      { code: 'analytics', name: '数据分析', included: true },
      { code: 'custom', name: '定制训练', included: true, description: '支持自定义模型训练' },
    ],
    maxDevices: -1,
    maxUsers: -1,
    storageLimit: -1,
    apiCallsLimit: -1,
    supportLevel: 2,
    supportLevelName: '专属客服',
    recommended: false,
    sort: 3,
    status: 1,
  },
];

const versions = [
  {
    id: 1,
    pluginId: 1,
    version: '2.1.0',
    changelog: `## 更新内容

### 新功能
- 新增批量导入优惠券功能，支持Excel导入
- 新增优惠券分享海报自动生成
- 新增优惠券核销语音播报功能

### 优化
- 优化优惠券核销性能，响应时间降低50%
- 优化移动端体验，适配更多机型
- 优化数据统计图表展示

### 修复
- 修复批量发放时部分用户未收到的问题
- 修复优惠券过期提醒时间不准确的问题`,
    downloadUrl: 'https://download.example.com/plugin_coupon_2.1.0.zip',
    fileHash: 'sha256:abc123def456ghi789jkl012mno345pqr678stu901vwx234yz',
    fileSize: 2048576,
    fileSizeDisplay: '2.0 MB',
    minAppVersion: '1.0.0',
    isLatest: 1,
    status: 1,
    downloadCount: 1256,
    createdAt: '2024-03-01T15:20:00Z',
  },
  {
    id: 2,
    pluginId: 1,
    version: '2.0.0',
    changelog: `## 更新内容

### 重大更新
- 全新UI设计，更加现代化
- 新增数据统计功能
- 新增优惠券模板功能

### 优化
- 性能优化，加载速度提升30%
- 数据库查询优化`,
    downloadUrl: 'https://download.example.com/plugin_coupon_2.0.0.zip',
    fileHash: 'sha256:def789ghi012jkl345mno678pqr901stu234vwx567yz890abc',
    fileSize: 1948576,
    fileSizeDisplay: '1.9 MB',
    minAppVersion: '1.0.0',
    isLatest: 0,
    status: 1,
    downloadCount: 890,
    createdAt: '2024-02-15T10:00:00Z',
  },
  {
    id: 3,
    pluginId: 2,
    version: '1.5.0',
    changelog: `## 更新内容

### 新功能
- 新增活动预热功能
- 新增预约提醒功能
- 新增活动数据大屏

### 优化
- 高并发场景优化
- 库存锁定机制改进`,
    downloadUrl: 'https://download.example.com/plugin_seckill_1.5.0.zip',
    fileHash: 'sha256:jkl345mno678pqr901stu234vwx567yz890abc123def456ghi',
    fileSize: 3148576,
    fileSizeDisplay: '3.0 MB',
    minAppVersion: '1.2.0',
    isLatest: 1,
    status: 1,
    downloadCount: 678,
    createdAt: '2024-03-10T09:15:00Z',
  },
  {
    id: 4,
    pluginId: 3,
    version: '1.0.0',
    changelog: `## 首次发布
- 支持多种二维码类型生成
- 支持样式自定义
- 支持批量生成`,
    downloadUrl: 'https://download.example.com/plugin_qrcode_1.0.0.zip',
    fileHash: 'sha256:mno678pqr901stu234vwx567yz890abc123def456ghi012jkl',
    fileSize: 524288,
    fileSizeDisplay: '512 KB',
    minAppVersion: '1.0.0',
    isLatest: 1,
    status: 1,
    downloadCount: 2345,
    createdAt: '2024-03-01T09:00:00Z',
  },
  {
    id: 5,
    pluginId: 6,
    version: '1.0.0',
    changelog: `## 首次发布
- 基于大模型的智能对话
- 多轮对话支持
- 知识库管理
- 人机协作`,
    downloadUrl: 'https://download.example.com/plugin_robot_1.0.0.zip',
    fileHash: 'sha256:pqr901stu234vwx567yz890abc123def456ghi012jkl345mno',
    fileSize: 5242880,
    fileSizeDisplay: '5.0 MB',
    minAppVersion: '1.8.0',
    isLatest: 1,
    status: 1,
    downloadCount: 1567,
    createdAt: '2024-03-15T08:30:00Z',
  },
];

const cartItems = [
  {
    id: 1,
    userId: 1,
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    pluginCover: 'https://picsum.photos/200/150?random=1',
    pluginVersion: '2.1.0',
    developerName: '奇络科技',
    developerId: 1,
    planId: 2,
    planName: '专业版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 299,
    originalPrice: 399,
    discount: 100,
    maxDevices: 5,
    selected: true,
    createdAt: '2024-03-20T10:30:00Z',
  },
  {
    id: 2,
    userId: 1,
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    pluginCover: 'https://picsum.photos/200/150?random=2',
    pluginVersion: '1.5.0',
    developerName: '云商科技',
    developerId: 2,
    planId: 5,
    planName: '专业版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 499,
    originalPrice: 699,
    discount: 200,
    maxDevices: 3,
    selected: true,
    createdAt: '2024-03-20T11:00:00Z',
  },
  {
    id: 3,
    userId: 1,
    pluginId: 6,
    pluginName: 'AI智能客服',
    pluginCode: 'plugin_robot',
    pluginCover: 'https://picsum.photos/200/150?random=6',
    pluginVersion: '1.0.0',
    developerName: 'AI智能科技',
    developerId: 4,
    planId: 9,
    planName: '标准版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 799,
    originalPrice: 999,
    discount: 200,
    maxDevices: 3,
    selected: false,
    createdAt: '2024-03-21T09:15:00Z',
  },
];

const orders = [
  {
    id: 1,
    orderNo: 'PLM202603201030000001',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    pluginCover: 'https://picsum.photos/200/150?random=1',
    developerId: 1,
    developerName: '奇络科技',
    planId: 2,
    planName: '专业版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    quantity: 1,
    amount: 299,
    originalAmount: 399,
    discountAmount: 100,
    couponId: null,
    couponCode: null,
    couponName: null,
    paymentMethod: 1,
    paymentMethodName: '微信支付',
    paymentNo: 'WX4200001234567890',
    paymentTime: '2024-03-20T10:35:00Z',
    status: 1,
    statusName: '已支付',
    expireTime: '2024-03-20T10:45:00Z',
    licenseId: 1,
    subscriptionId: 1,
    remark: null,
    createdAt: '2024-03-20T10:30:00Z',
    updatedAt: '2024-03-20T10:35:00Z',
  },
  {
    id: 2,
    orderNo: 'PLM202603191520000002',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 3,
    pluginName: '二维码生成器',
    pluginCode: 'plugin_qrcode',
    pluginCover: 'https://picsum.photos/200/150?random=3',
    developerId: 1,
    developerName: '奇络科技',
    planId: 7,
    planName: '免费版',
    periodType: 3,
    periodTypeName: '永久',
    periodDays: -1,
    quantity: 1,
    amount: 0,
    originalAmount: 0,
    discountAmount: 0,
    couponId: null,
    couponCode: null,
    couponName: null,
    paymentMethod: 4,
    paymentMethodName: '免费',
    paymentNo: null,
    paymentTime: null,
    status: 1,
    statusName: '已完成',
    expireTime: null,
    licenseId: 3,
    subscriptionId: null,
    remark: null,
    createdAt: '2024-03-19T15:20:00Z',
    updatedAt: '2024-03-19T15:20:00Z',
  },
  {
    id: 3,
    orderNo: 'PLM202603181000000003',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 5,
    pluginName: '数据统计分析',
    pluginCode: 'plugin_analysis',
    pluginCover: 'https://picsum.photos/200/150?random=5',
    developerId: 1,
    developerName: '奇络科技',
    planId: 5,
    planName: '专业版',
    periodType: 2,
    periodTypeName: '年付',
    periodDays: 365,
    quantity: 1,
    amount: 2999,
    originalAmount: 3999,
    discountAmount: 1000,
    couponId: 1,
    couponCode: 'NEWYEAR2024',
    couponName: '新年特惠券',
    paymentMethod: 2,
    paymentMethodName: '支付宝',
    paymentNo: 'ALI2024031810001234567890',
    paymentTime: '2024-03-18T10:05:00Z',
    status: 1,
    statusName: '已支付',
    expireTime: '2025-03-18T10:05:00Z',
    licenseId: 2,
    subscriptionId: 2,
    remark: '年付优惠',
    createdAt: '2024-03-18T10:00:00Z',
    updatedAt: '2024-03-18T10:05:00Z',
  },
  {
    id: 4,
    orderNo: 'PLM202603171430000004',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    pluginCover: 'https://picsum.photos/200/150?random=2',
    developerId: 2,
    developerName: '云商科技',
    planId: 4,
    planName: '基础版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    quantity: 1,
    amount: 199,
    originalAmount: 199,
    discountAmount: 0,
    couponId: null,
    couponCode: null,
    couponName: null,
    paymentMethod: 3,
    paymentMethodName: '卡密支付',
    paymentNo: 'CARD-ABCD-1234-EFGH',
    paymentTime: '2024-03-17T14:35:00Z',
    status: 1,
    statusName: '已支付',
    expireTime: '2024-04-17T14:35:00Z',
    licenseId: 4,
    subscriptionId: 3,
    remark: '卡密兑换',
    createdAt: '2024-03-17T14:30:00Z',
    updatedAt: '2024-03-17T14:35:00Z',
  },
  {
    id: 5,
    orderNo: 'PLM202603161200000005',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 6,
    pluginName: 'AI智能客服',
    pluginCode: 'plugin_robot',
    pluginCover: 'https://picsum.photos/200/150?random=6',
    developerId: 4,
    developerName: 'AI智能科技',
    planId: 8,
    planName: '体验版',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    quantity: 1,
    amount: 299,
    originalAmount: 399,
    discountAmount: 100,
    couponId: null,
    couponCode: null,
    couponName: null,
    paymentMethod: 1,
    paymentMethodName: '微信支付',
    paymentNo: null,
    paymentTime: null,
    status: 0,
    statusName: '待支付',
    expireTime: '2024-03-16T12:30:00Z',
    licenseId: null,
    subscriptionId: null,
    remark: null,
    createdAt: '2024-03-16T12:00:00Z',
    updatedAt: '2024-03-16T12:00:00Z',
  },
  {
    id: 6,
    orderNo: 'PLM202603151000000006',
    userId: 1,
    userName: '张三',
    userPhone: '138****8888',
    pluginId: 4,
    pluginName: '短信通知',
    pluginCode: 'plugin_sms',
    pluginCover: 'https://picsum.photos/200/150?random=4',
    developerId: 3,
    developerName: '通讯先锋',
    planId: 11,
    planName: '标准版',
    periodType: 1,
    periodTypeName: '一次性',
    periodDays: -1,
    quantity: 1,
    amount: 599,
    originalAmount: 999,
    discountAmount: 400,
    couponId: null,
    couponCode: null,
    couponName: null,
    paymentMethod: 1,
    paymentMethodName: '微信支付',
    paymentNo: 'WX4200009876543210',
    paymentTime: '2024-03-15T10:05:00Z',
    status: 2,
    statusName: '已取消',
    expireTime: '2024-03-15T10:30:00Z',
    licenseId: null,
    subscriptionId: null,
    remark: '用户主动取消',
    createdAt: '2024-03-15T10:00:00Z',
    updatedAt: '2024-03-15T10:10:00Z',
  },
];

const subscriptions = [
  {
    id: 1,
    userId: 1,
    userName: '张三',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    pluginCover: 'https://picsum.photos/200/150?random=1',
    developerId: 1,
    developerName: '奇络科技',
    planId: 2,
    planName: '专业版',
    orderId: 1,
    orderNo: 'PLM202603201030000001',
    licenseId: 1,
    licenseKey: '550e8400-e29b-41d4-a716-446655440001',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    startTime: '2024-03-20T10:35:00Z',
    endTime: '2024-04-20T10:35:00Z',
    autoRenew: 1,
    renewPrice: 299,
    status: 1,
    statusName: '有效',
    daysRemaining: 25,
    usedDevices: 2,
    maxDevices: 5,
    createdAt: '2024-03-20T10:35:00Z',
    updatedAt: '2024-03-25T14:30:00Z',
  },
  {
    id: 2,
    userId: 1,
    userName: '张三',
    pluginId: 5,
    pluginName: '数据统计分析',
    pluginCode: 'plugin_analysis',
    pluginCover: 'https://picsum.photos/200/150?random=5',
    developerId: 1,
    developerName: '奇络科技',
    planId: 5,
    planName: '专业版',
    orderId: 3,
    orderNo: 'PLM202603181000000003',
    licenseId: 2,
    licenseKey: '550e8400-e29b-41d4-a716-446655440002',
    periodType: 2,
    periodTypeName: '年付',
    periodDays: 365,
    startTime: '2024-03-18T10:05:00Z',
    endTime: '2025-03-18T10:05:00Z',
    autoRenew: 0,
    renewPrice: 2999,
    status: 1,
    statusName: '有效',
    daysRemaining: 358,
    usedDevices: 1,
    maxDevices: 5,
    createdAt: '2024-03-18T10:05:00Z',
    updatedAt: '2024-03-25T16:45:00Z',
  },
  {
    id: 3,
    userId: 1,
    userName: '张三',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    pluginCover: 'https://picsum.photos/200/150?random=2',
    developerId: 2,
    developerName: '云商科技',
    planId: 4,
    planName: '基础版',
    orderId: 4,
    orderNo: 'PLM202603171430000004',
    licenseId: 4,
    licenseKey: '550e8400-e29b-41d4-a716-446655440004',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    startTime: '2024-03-17T14:35:00Z',
    endTime: '2024-04-17T14:35:00Z',
    autoRenew: 0,
    renewPrice: 199,
    status: 1,
    statusName: '有效',
    daysRemaining: 22,
    usedDevices: 1,
    maxDevices: 1,
    createdAt: '2024-03-17T14:35:00Z',
    updatedAt: '2024-03-24T10:20:00Z',
  },
  {
    id: 4,
    userId: 1,
    userName: '张三',
    pluginId: 3,
    pluginName: '二维码生成器',
    pluginCode: 'plugin_qrcode',
    pluginCover: 'https://picsum.photos/200/150?random=3',
    developerId: 1,
    developerName: '奇络科技',
    planId: 7,
    planName: '免费版',
    orderId: 2,
    orderNo: 'PLM202603191520000002',
    licenseId: 3,
    licenseKey: '550e8400-e29b-41d4-a716-446655440003',
    periodType: 3,
    periodTypeName: '永久',
    periodDays: -1,
    startTime: '2024-03-19T15:20:00Z',
    endTime: null,
    autoRenew: 0,
    renewPrice: 0,
    status: 1,
    statusName: '永久有效',
    daysRemaining: -1,
    usedDevices: 1,
    maxDevices: 1,
    createdAt: '2024-03-19T15:20:00Z',
    updatedAt: '2024-03-19T15:20:00Z',
  },
  {
    id: 5,
    userId: 1,
    userName: '张三',
    pluginId: 8,
    pluginName: '幸运抽奖',
    pluginCode: 'plugin_lottery',
    pluginCover: 'https://picsum.photos/200/150?random=8',
    developerId: 1,
    developerName: '奇络科技',
    planId: 12,
    planName: '专业版',
    orderId: null,
    orderNo: null,
    licenseId: 5,
    licenseKey: '550e8400-e29b-41d4-a716-446655440005',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    startTime: '2024-02-20T09:00:00Z',
    endTime: '2024-03-20T09:00:00Z',
    autoRenew: 0,
    renewPrice: 129,
    status: 2,
    statusName: '已过期',
    daysRemaining: 0,
    usedDevices: 1,
    maxDevices: 3,
    createdAt: '2024-02-20T09:00:00Z',
    updatedAt: '2024-03-20T09:00:00Z',
  },
];

const licenses = [
  {
    id: 1,
    licenseKey: '550e8400-e29b-41d4-a716-446655440001',
    userId: 1,
    userName: '张三',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 2,
    planName: '专业版',
    orderId: 1,
    orderNo: 'PLM202603201030000001',
    developerId: 1,
    developerName: '奇络科技',
    primaryDeviceId: 'device-001',
    primaryDeviceName: '主服务器',
    deviceType: 'server',
    osVersion: 'CentOS 7.9',
    appVersion: '2.1.0',
    ipAddress: '192.168.1.100',
    macAddress: '00:1A:2B:3C:4D:5E',
    bindCount: 2,
    maxDevices: 5,
    verifyCount: 156,
    lastVerifyTime: '2024-03-25T14:30:00Z',
    lastVerifyIp: '192.168.1.100',
    status: 1,
    statusName: '启用',
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    startTime: '2024-03-20T10:35:00Z',
    endTime: '2024-04-20T10:35:00Z',
    isExpired: false,
    daysRemaining: 25,
    features: ['basic', 'template', 'batch', 'api', 'priority'],
    createdAt: '2024-03-20T10:35:00Z',
    updatedAt: '2024-03-25T14:30:00Z',
  },
  {
    id: 2,
    licenseKey: '550e8400-e29b-41d4-a716-446655440002',
    userId: 1,
    userName: '张三',
    pluginId: 5,
    pluginName: '数据统计分析',
    pluginCode: 'plugin_analysis',
    planId: 5,
    planName: '专业版',
    orderId: 3,
    orderNo: 'PLM202603181000000003',
    developerId: 1,
    developerName: '奇络科技',
    primaryDeviceId: 'device-002',
    primaryDeviceName: '数据分析服务器',
    deviceType: 'server',
    osVersion: 'Ubuntu 20.04',
    appVersion: '2.0.0',
    ipAddress: '192.168.1.101',
    macAddress: '00:1A:2B:3C:4D:5F',
    bindCount: 1,
    maxDevices: 5,
    verifyCount: 89,
    lastVerifyTime: '2024-03-25T16:45:00Z',
    lastVerifyIp: '192.168.1.101',
    status: 1,
    statusName: '启用',
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    startTime: '2024-03-18T10:05:00Z',
    endTime: '2025-03-18T10:05:00Z',
    isExpired: false,
    daysRemaining: 358,
    features: ['basic', 'template', 'batch', 'api', 'priority'],
    createdAt: '2024-03-18T10:05:00Z',
    updatedAt: '2024-03-25T16:45:00Z',
  },
  {
    id: 3,
    licenseKey: '550e8400-e29b-41d4-a716-446655440003',
    userId: 1,
    userName: '张三',
    pluginId: 3,
    pluginName: '二维码生成器',
    pluginCode: 'plugin_qrcode',
    planId: 7,
    planName: '免费版',
    orderId: 2,
    orderNo: 'PLM202603191520000002',
    developerId: 1,
    developerName: '奇络科技',
    primaryDeviceId: 'device-003',
    primaryDeviceName: '开发机',
    deviceType: 'desktop',
    osVersion: 'Windows 11',
    appVersion: '1.0.0',
    ipAddress: '192.168.1.103',
    macAddress: '00:1A:2B:3C:4D:60',
    bindCount: 1,
    maxDevices: 1,
    verifyCount: 23,
    lastVerifyTime: '2024-03-24T11:20:00Z',
    lastVerifyIp: '192.168.1.103',
    status: 1,
    statusName: '启用',
    verifyLevel: 0,
    verifyLevelName: '无验证',
    startTime: '2024-03-19T15:20:00Z',
    endTime: null,
    isExpired: false,
    daysRemaining: -1,
    features: ['basic', 'template'],
    createdAt: '2024-03-19T15:20:00Z',
    updatedAt: '2024-03-24T11:20:00Z',
  },
  {
    id: 4,
    licenseKey: '550e8400-e29b-41d4-a716-446655440004',
    userId: 1,
    userName: '张三',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    planId: 4,
    planName: '基础版',
    orderId: 4,
    orderNo: 'PLM202603171430000004',
    developerId: 2,
    developerName: '云商科技',
    primaryDeviceId: 'device-004',
    primaryDeviceName: '活动服务器',
    deviceType: 'server',
    osVersion: 'CentOS 8',
    appVersion: '1.5.0',
    ipAddress: '192.168.1.104',
    macAddress: '00:1A:2B:3C:4D:61',
    bindCount: 1,
    maxDevices: 1,
    verifyCount: 45,
    lastVerifyTime: '2024-03-25T09:15:00Z',
    lastVerifyIp: '192.168.1.104',
    status: 1,
    statusName: '启用',
    verifyLevel: 2,
    verifyLevelName: '高级验证',
    startTime: '2024-03-17T14:35:00Z',
    endTime: '2024-04-17T14:35:00Z',
    isExpired: false,
    daysRemaining: 22,
    features: ['basic', 'concurrent', 'monitor'],
    createdAt: '2024-03-17T14:35:00Z',
    updatedAt: '2024-03-25T09:15:00Z',
  },
  {
    id: 5,
    licenseKey: '550e8400-e29b-41d4-a716-446655440005',
    userId: 1,
    userName: '张三',
    pluginId: 8,
    pluginName: '幸运抽奖',
    pluginCode: 'plugin_lottery',
    planId: 12,
    planName: '专业版',
    orderId: null,
    orderNo: null,
    developerId: 1,
    developerName: '奇络科技',
    primaryDeviceId: 'device-005',
    primaryDeviceName: '测试服务器',
    deviceType: 'server',
    osVersion: 'Ubuntu 18.04',
    appVersion: '1.3.0',
    ipAddress: '192.168.1.105',
    macAddress: '00:1A:2B:3C:4D:62',
    bindCount: 1,
    maxDevices: 3,
    verifyCount: 12,
    lastVerifyTime: '2024-03-19T16:30:00Z',
    lastVerifyIp: '192.168.1.105',
    status: 2,
    statusName: '已过期',
    verifyLevel: 1,
    verifyLevelName: '基础验证',
    startTime: '2024-02-20T09:00:00Z',
    endTime: '2024-03-20T09:00:00Z',
    isExpired: true,
    daysRemaining: 0,
    features: ['basic', 'template', 'batch'],
    createdAt: '2024-02-20T09:00:00Z',
    updatedAt: '2024-03-20T09:00:00Z',
  },
];

const devices = [
  {
    id: 1,
    userId: 1,
    licenseId: 1,
    licenseKey: '550e8400-e29b-41d4-a716-446655440001',
    pluginId: 1,
    pluginName: '智能优惠券',
    deviceId: 'device-001',
    deviceName: '主服务器',
    deviceType: 'server',
    deviceTypeName: '服务器',
    osVersion: 'CentOS 7.9',
    appVersion: '2.1.0',
    macAddress: '00:1A:2B:3C:4D:5E',
    ipAddress: '192.168.1.100',
    cpuInfo: 'Intel Xeon E5-2680 v4',
    memoryInfo: '32GB',
    diskInfo: '500GB SSD',
    registerTime: '2024-03-20T10:40:00Z',
    lastActiveTime: '2024-03-25T14:30:00Z',
    activeDuration: 5,
    status: 1,
    statusName: '在线',
    online: true,
    createdAt: '2024-03-20T10:40:00Z',
  },
  {
    id: 2,
    userId: 1,
    licenseId: 1,
    licenseKey: '550e8400-e29b-41d4-a716-446655440001',
    pluginId: 1,
    pluginName: '智能优惠券',
    deviceId: 'device-002',
    deviceName: '测试服务器',
    deviceType: 'server',
    deviceTypeName: '服务器',
    osVersion: 'Ubuntu 20.04',
    appVersion: '2.1.0',
    macAddress: '00:1A:2B:3C:4D:5F',
    ipAddress: '192.168.1.102',
    cpuInfo: 'Intel Core i7-9700K',
    memoryInfo: '16GB',
    diskInfo: '256GB SSD',
    registerTime: '2024-03-21T09:15:00Z',
    lastActiveTime: '2024-03-24T18:20:00Z',
    activeDuration: 3,
    status: 0,
    statusName: '离线',
    online: false,
    createdAt: '2024-03-21T09:15:00Z',
  },
  {
    id: 3,
    userId: 1,
    licenseId: 2,
    licenseKey: '550e8400-e29b-41d4-a716-446655440002',
    pluginId: 5,
    pluginName: '数据统计分析',
    deviceId: 'device-002',
    deviceName: '数据分析服务器',
    deviceType: 'server',
    deviceTypeName: '服务器',
    osVersion: 'Ubuntu 20.04',
    appVersion: '2.0.0',
    macAddress: '00:1A:2B:3C:4D:5F',
    ipAddress: '192.168.1.101',
    cpuInfo: 'Intel Xeon Gold 6248',
    memoryInfo: '64GB',
    diskInfo: '1TB SSD',
    registerTime: '2024-03-18T10:10:00Z',
    lastActiveTime: '2024-03-25T16:45:00Z',
    activeDuration: 7,
    status: 1,
    statusName: '在线',
    online: true,
    createdAt: '2024-03-18T10:10:00Z',
  },
  {
    id: 4,
    userId: 1,
    licenseId: 3,
    licenseKey: '550e8400-e29b-41d4-a716-446655440003',
    pluginId: 3,
    pluginName: '二维码生成器',
    deviceId: 'device-003',
    deviceName: '开发机',
    deviceType: 'desktop',
    deviceTypeName: '桌面电脑',
    osVersion: 'Windows 11',
    appVersion: '1.0.0',
    macAddress: '00:1A:2B:3C:4D:60',
    ipAddress: '192.168.1.103',
    cpuInfo: 'AMD Ryzen 9 5900X',
    memoryInfo: '32GB',
    diskInfo: '1TB NVMe',
    registerTime: '2024-03-19T15:25:00Z',
    lastActiveTime: '2024-03-24T11:20:00Z',
    activeDuration: 5,
    status: 0,
    statusName: '离线',
    online: false,
    createdAt: '2024-03-19T15:25:00Z',
  },
  {
    id: 5,
    userId: 1,
    licenseId: 4,
    licenseKey: '550e8400-e29b-41d4-a716-446655440004',
    pluginId: 2,
    pluginName: '限时秒杀',
    deviceId: 'device-004',
    deviceName: '活动服务器',
    deviceType: 'server',
    deviceTypeName: '服务器',
    osVersion: 'CentOS 8',
    appVersion: '1.5.0',
    macAddress: '00:1A:2B:3C:4D:61',
    ipAddress: '192.168.1.104',
    cpuInfo: 'Intel Xeon Platinum 8280',
    memoryInfo: '128GB',
    diskInfo: '2TB SSD',
    registerTime: '2024-03-17T14:40:00Z',
    lastActiveTime: '2024-03-25T09:15:00Z',
    activeDuration: 8,
    status: 1,
    statusName: '在线',
    online: true,
    createdAt: '2024-03-17T14:40:00Z',
  },
];

const cardBatches = [
  {
    id: 1,
    batchNo: 'BATCH20260315001',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 2,
    planName: '专业版',
    developerId: 1,
    developerName: '奇络科技',
    totalCount: 100,
    usedCount: 45,
    unusedCount: 55,
    frozenCount: 0,
    price: 299,
    faceValue: 299,
    expireDays: 30,
    expireTime: '2024-06-15T00:00:00Z',
    status: 1,
    statusName: '正常',
    creatorId: 1,
    creatorName: '管理员',
    remark: '春季促销活动卡密',
    createdAt: '2024-03-15T10:00:00Z',
  },
  {
    id: 2,
    batchNo: 'BATCH20260318002',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    planId: 5,
    planName: '专业版',
    developerId: 2,
    developerName: '云商科技',
    totalCount: 50,
    usedCount: 12,
    unusedCount: 38,
    frozenCount: 0,
    price: 499,
    faceValue: 499,
    expireDays: 30,
    expireTime: '2024-06-18T00:00:00Z',
    status: 1,
    statusName: '正常',
    creatorId: 1,
    creatorName: '管理员',
    remark: '渠道合作卡密',
    createdAt: '2024-03-18T14:30:00Z',
  },
  {
    id: 3,
    batchNo: 'BATCH20260320003',
    pluginId: 6,
    pluginName: 'AI智能客服',
    pluginCode: 'plugin_robot',
    planId: 8,
    planName: '体验版',
    developerId: 4,
    developerName: 'AI智能科技',
    totalCount: 200,
    usedCount: 89,
    unusedCount: 111,
    frozenCount: 0,
    price: 299,
    faceValue: 299,
    expireDays: 30,
    expireTime: '2024-06-20T00:00:00Z',
    status: 1,
    statusName: '正常',
    creatorId: 1,
    creatorName: '管理员',
    remark: '新用户试用卡密',
    createdAt: '2024-03-20T09:00:00Z',
  },
  {
    id: 4,
    batchNo: 'BATCH20260310004',
    pluginId: 5,
    pluginName: '数据统计分析',
    pluginCode: 'plugin_analysis',
    planId: 5,
    planName: '专业版',
    developerId: 1,
    developerName: '奇络科技',
    totalCount: 30,
    usedCount: 30,
    unusedCount: 0,
    frozenCount: 0,
    price: 399,
    faceValue: 399,
    expireDays: 30,
    expireTime: '2024-06-10T00:00:00Z',
    status: 1,
    statusName: '正常',
    creatorId: 1,
    creatorName: '管理员',
    remark: 'VIP客户专属卡密',
    createdAt: '2024-03-10T11:00:00Z',
  },
  {
    id: 5,
    batchNo: 'BATCH20260305005',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 1,
    planName: '基础版',
    developerId: 1,
    developerName: '奇络科技',
    totalCount: 500,
    usedCount: 450,
    unusedCount: 50,
    frozenCount: 0,
    price: 99,
    faceValue: 99,
    expireDays: 30,
    expireTime: '2024-06-05T00:00:00Z',
    status: 1,
    statusName: '正常',
    creatorId: 1,
    creatorName: '管理员',
    remark: '大规模推广卡密',
    createdAt: '2024-03-05T16:30:00Z',
  },
];

const cards = [
  {
    id: 1,
    batchId: 1,
    batchNo: 'BATCH20260315001',
    cardNo: 'CARD-ABCD-1234-EFGH',
    cardPwd: 'PWD5678',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 2,
    planName: '专业版',
    price: 299,
    faceValue: 299,
    expireDays: 30,
    status: 0,
    statusName: '未使用',
    usedBy: null,
    usedById: null,
    usedAt: null,
    usedOrderId: null,
    expireTime: '2024-06-15T00:00:00Z',
    createdAt: '2024-03-15T10:00:00Z',
  },
  {
    id: 2,
    batchId: 1,
    batchNo: 'BATCH20260315001',
    cardNo: 'CARD-IJKL-5678-MNOP',
    cardPwd: 'PWD9012',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 2,
    planName: '专业版',
    price: 299,
    faceValue: 299,
    expireDays: 30,
    status: 1,
    statusName: '已使用',
    usedBy: '张三',
    usedById: 1,
    usedAt: '2024-03-20T15:30:00Z',
    usedOrderId: 10,
    expireTime: '2024-06-15T00:00:00Z',
    createdAt: '2024-03-15T10:00:00Z',
  },
  {
    id: 3,
    batchId: 1,
    batchNo: 'BATCH20260315001',
    cardNo: 'CARD-QRST-9012-UVWX',
    cardPwd: 'PWD3456',
    pluginId: 1,
    pluginName: '智能优惠券',
    pluginCode: 'plugin_coupon',
    planId: 2,
    planName: '专业版',
    price: 299,
    faceValue: 299,
    expireDays: 30,
    status: 0,
    statusName: '未使用',
    usedBy: null,
    usedById: null,
    usedAt: null,
    usedOrderId: null,
    expireTime: '2024-06-15T00:00:00Z',
    createdAt: '2024-03-15T10:00:00Z',
  },
  {
    id: 4,
    batchId: 2,
    batchNo: 'BATCH20260318002',
    cardNo: 'CARD-YZAB-3456-CDEF',
    cardPwd: 'PWD7890',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    planId: 5,
    planName: '专业版',
    price: 499,
    faceValue: 499,
    expireDays: 30,
    status: 0,
    statusName: '未使用',
    usedBy: null,
    usedById: null,
    usedAt: null,
    usedOrderId: null,
    expireTime: '2024-06-18T00:00:00Z',
    createdAt: '2024-03-18T14:30:00Z',
  },
  {
    id: 5,
    batchId: 2,
    batchNo: 'BATCH20260318002',
    cardNo: 'CARD-GHIJ-7890-KLMN',
    cardPwd: 'PWD1234',
    pluginId: 2,
    pluginName: '限时秒杀',
    pluginCode: 'plugin_seckill',
    planId: 5,
    planName: '专业版',
    price: 499,
    faceValue: 499,
    expireDays: 30,
    status: 2,
    statusName: '已过期',
    usedBy: null,
    usedById: null,
    usedAt: null,
    usedOrderId: null,
    expireTime: '2024-03-18T00:00:00Z',
    createdAt: '2024-03-18T14:30:00Z',
  },
  {
    id: 6,
    batchId: 3,
    batchNo: 'BATCH20260320003',
    cardNo: 'CARD-OPQR-1234-STUV',
    cardPwd: 'PWD5678',
    pluginId: 6,
    pluginName: 'AI智能客服',
    pluginCode: 'plugin_robot',
    planId: 8,
    planName: '体验版',
    price: 299,
    faceValue: 299,
    expireDays: 30,
    status: 3,
    statusName: '已冻结',
    usedBy: null,
    usedById: null,
    usedAt: null,
    usedOrderId: null,
    expireTime: '2024-06-20T00:00:00Z',
    createdAt: '2024-03-20T09:00:00Z',
  },
];

const reviews = [
  {
    id: 1,
    pluginId: 1,
    userId: 1,
    userName: '张三',
    userAvatar: 'https://picsum.photos/50/50?random=100',
    userLevel: 'VIP会员',
    rating: 5,
    content: '非常好用的插件，功能强大，客服响应也很及时！优惠券发放和核销都很方便，数据统计也很详细，强烈推荐！',
    images: [],
    likeCount: 23,
    replyContent: '感谢您的支持，我们会继续努力！如有任何问题欢迎随时联系客服。',
    replyTime: '2024-03-22T10:00:00Z',
    status: 1,
    helpful: true,
    createdAt: '2024-03-21T15:30:00Z',
  },
  {
    id: 2,
    pluginId: 1,
    userId: 2,
    userName: '李四',
    userAvatar: 'https://picsum.photos/50/50?random=101',
    userLevel: '普通会员',
    rating: 4,
    content: '功能不错，就是价格稍微有点贵。希望能推出更多优惠活动。',
    images: [],
    likeCount: 8,
    replyContent: '感谢您的反馈，我们会考虑推出更多优惠活动，敬请关注！',
    replyTime: '2024-03-24T09:30:00Z',
    status: 1,
    helpful: false,
    createdAt: '2024-03-23T09:15:00Z',
  },
  {
    id: 3,
    pluginId: 1,
    userId: 3,
    userName: '王五',
    userAvatar: 'https://picsum.photos/50/50?random=102',
    userLevel: '企业会员',
    rating: 5,
    content: '企业版功能很全面，API接口对接也很方便，技术支持响应很快，帮我们解决了不少问题。',
    images: [],
    likeCount: 15,
    replyContent: '感谢您对企业的信任，我们会持续优化产品，为您提供更好的服务！',
    replyTime: '2024-03-20T14:00:00Z',
    status: 1,
    helpful: true,
    createdAt: '2024-03-19T16:45:00Z',
  },
  {
    id: 4,
    pluginId: 2,
    userId: 1,
    userName: '张三',
    userAvatar: 'https://picsum.photos/50/50?random=100',
    userLevel: 'VIP会员',
    rating: 5,
    content: '秒杀活动必备！高并发支持很稳定，活动期间没有任何问题，数据监控也很直观。',
    images: [],
    likeCount: 32,
    replyContent: '感谢您的认可，高并发是我们重点优化的方向，会持续提升性能！',
    replyTime: '2024-03-18T11:00:00Z',
    status: 1,
    helpful: true,
    createdAt: '2024-03-17T10:20:00Z',
  },
  {
    id: 5,
    pluginId: 6,
    userId: 2,
    userName: '李四',
    userAvatar: 'https://picsum.photos/50/50?random=101',
    userLevel: '普通会员',
    rating: 5,
    content: 'AI客服太智能了，能准确理解用户问题，自动回复率很高，大大减轻了客服压力！',
    images: [],
    likeCount: 45,
    replyContent: '感谢您的使用，我们正在持续优化AI模型，未来会更智能！',
    replyTime: '2024-03-22T15:30:00Z',
    status: 1,
    helpful: true,
    createdAt: '2024-03-21T14:00:00Z',
  },
  {
    id: 6,
    pluginId: 5,
    userId: 1,
    userName: '张三',
    userAvatar: 'https://picsum.photos/50/50?random=100',
    userLevel: 'VIP会员',
    rating: 4,
    content: '数据分析功能很强大，报表自定义很灵活，就是学习成本稍微有点高。',
    images: [],
    likeCount: 12,
    replyContent: '感谢您的反馈，我们正在制作更详细的使用教程，帮助用户快速上手！',
    replyTime: '2024-03-19T10:00:00Z',
    status: 1,
    helpful: false,
    createdAt: '2024-03-18T09:30:00Z',
  },
];

const developerStats = {
  overview: {
    totalRevenue: 125680.50,
    monthRevenue: 28560.00,
    totalOrders: 356,
    monthOrders: 89,
    totalDownloads: 4560,
    monthDownloads: 1230,
    totalPlugins: 3,
    activePlugins: 3,
    avgRating: 4.8,
    totalReviews: 156,
  },
  recentOrders: [
    { id: 101, orderNo: 'PLM202603251000000101', pluginName: '智能优惠券', planName: '专业版', amount: 299, userName: '用户A', createdAt: '2024-03-25T10:00:00Z' },
    { id: 102, orderNo: 'PLM202603250930000102', pluginName: '数据统计分析', planName: '专业版', amount: 399, userName: '用户B', createdAt: '2024-03-25T09:30:00Z' },
    { id: 103, orderNo: 'PLM202603250900000103', pluginName: '智能优惠券', planName: '企业版', amount: 799, userName: '用户C', createdAt: '2024-03-25T09:00:00Z' },
  ],
  topPlugins: [
    { id: 1, name: '智能优惠券', downloads: 2560, revenue: 68500.00, rating: 4.8, orderCount: 189 },
    { id: 5, name: '数据统计分析', downloads: 980, revenue: 38900.00, rating: 4.7, orderCount: 98 },
    { id: 8, name: '幸运抽奖', downloads: 2300, revenue: 18280.50, rating: 4.5, orderCount: 69 },
  ],
  revenueChart: [
    { date: '2024-03-01', revenue: 3560, orders: 12 },
    { date: '2024-03-02', revenue: 4280, orders: 15 },
    { date: '2024-03-03', revenue: 5120, orders: 18 },
    { date: '2024-03-04', revenue: 3890, orders: 13 },
    { date: '2024-03-05', revenue: 6780, orders: 22 },
    { date: '2024-03-06', revenue: 5430, orders: 19 },
    { date: '2024-03-07', revenue: 7120, orders: 25 },
    { date: '2024-03-08', revenue: 8560, orders: 28 },
    { date: '2024-03-09', revenue: 6230, orders: 21 },
    { date: '2024-03-10', revenue: 9120, orders: 32 },
    { date: '2024-03-11', revenue: 7890, orders: 27 },
    { date: '2024-03-12', revenue: 10230, orders: 35 },
    { date: '2024-03-13', revenue: 8560, orders: 29 },
    { date: '2024-03-14', revenue: 11230, orders: 38 },
    { date: '2024-03-15', revenue: 12560, orders: 42 },
    { date: '2024-03-16', revenue: 9870, orders: 33 },
    { date: '2024-03-17', revenue: 13240, orders: 45 },
    { date: '2024-03-18', revenue: 14560, orders: 48 },
    { date: '2024-03-19', revenue: 11890, orders: 40 },
    { date: '2024-03-20', revenue: 15670, orders: 52 },
    { date: '2024-03-21', revenue: 12340, orders: 41 },
    { date: '2024-03-22', revenue: 16780, orders: 55 },
    { date: '2024-03-23', revenue: 14560, orders: 48 },
    { date: '2024-03-24', revenue: 17890, orders: 59 },
    { date: '2024-03-25', revenue: 19230, orders: 63 },
  ],
  downloadChart: [
    { date: '2024-03-01', downloads: 156 },
    { date: '2024-03-02', downloads: 189 },
    { date: '2024-03-03', downloads: 234 },
    { date: '2024-03-04', downloads: 178 },
    { date: '2024-03-05', downloads: 267 },
    { date: '2024-03-06', downloads: 245 },
    { date: '2024-03-07', downloads: 312 },
  ],
};

const withdrawRecords = [
  { id: 1, amount: 10000.00, status: 1, statusName: '已到账', bankName: '招商银行', bankAccount: '****8888', applyTime: '2024-03-01T10:00:00Z', completeTime: '2024-03-02T15:30:00Z' },
  { id: 2, amount: 15000.00, status: 1, statusName: '已到账', bankName: '招商银行', bankAccount: '****8888', applyTime: '2024-03-10T14:00:00Z', completeTime: '2024-03-11T16:20:00Z' },
  { id: 3, amount: 20000.00, status: 0, statusName: '处理中', bankName: '招商银行', bankAccount: '****8888', applyTime: '2024-03-25T09:00:00Z', completeTime: null },
  { id: 4, amount: 8000.00, status: 1, statusName: '已到账', bankName: '招商银行', bankAccount: '****8888', applyTime: '2024-02-20T11:30:00Z', completeTime: '2024-02-21T14:00:00Z' },
  { id: 5, amount: 15880.00, status: 1, statusName: '已到账', bankName: '招商银行', bankAccount: '****8888', applyTime: '2024-02-10T16:00:00Z', completeTime: '2024-02-11T18:30:00Z' },
];

const verificationCodes: Record<string, { code: string; expireTime: number; attempts: number }> = {};

const mockPluginMarket: MockMethod[] = [
  {
    url: "/plugin/category/list",
    method: "get",
    response: () => resultSuccess(categories.filter(c => c.parentId === 0)),
  },
  {
    url: "/plugin/category/tree",
    method: "get",
    response: () => resultSuccess(categories),
  },
  {
    url: "/plugin/category/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      const findCategory = (cats: any[]): any => {
        for (const cat of cats) {
          if (cat.id === id) return cat;
          if (cat.children) {
            const found = findCategory(cat.children);
            if (found) return found;
          }
        }
        return null;
      };
      return resultSuccess(findCategory(categories));
    },
  },
  {
    url: "/plugin/category/add",
    method: "post",
    response: () => resultSuccess(Date.now()),
  },
  {
    url: "/plugin/category/edit",
    method: "put",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/category/delete",
    method: "delete",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/market/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...plugins];
      if (params.categoryId) {
        const catId = parseInt(params.categoryId);
        list = list.filter(p => p.categoryId === catId || p.parentCategoryName === categories.find(c => c.id === catId)?.name);
      }
      if (params.keyword) {
        const keyword = params.keyword.toLowerCase();
        list = list.filter(p => 
          p.name.toLowerCase().includes(keyword) || 
          p.summary.toLowerCase().includes(keyword) ||
          p.tags.some((t: string) => t.toLowerCase().includes(keyword))
        );
      }
      if (params.priceType !== undefined && params.priceType !== '') {
        list = list.filter(p => p.priceType === parseInt(params.priceType));
      }
      if (params.isOfficial !== undefined && params.isOfficial !== '') {
        list = list.filter(p => p.isOfficial === parseInt(params.isOfficial));
      }
      if (params.verifyLevel !== undefined && params.verifyLevel !== '') {
        list = list.filter(p => p.verifyLevel === parseInt(params.verifyLevel));
      }
      if (params.minRating !== undefined && params.minRating !== '') {
        list = list.filter(p => p.rating >= parseFloat(params.minRating));
      }
      if (params.sortBy === 'download') {
        list.sort((a, b) => b.downloadCount - a.downloadCount);
      } else if (params.sortBy === 'rating') {
        list.sort((a, b) => b.rating - a.rating);
      } else if (params.sortBy === 'price_asc') {
        list.sort((a, b) => a.price - b.price);
      } else if (params.sortBy === 'price_desc') {
        list.sort((a, b) => b.price - a.price);
      } else if (params.sortBy === 'newest') {
        list.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      const pagedList = list.slice(start, start + pageSize);
      return resultSuccess({
        list: pagedList,
        total: list.length,
        pageNum,
        pageSize,
        totalPages: Math.ceil(list.length / pageSize),
      });
    },
  },
  {
    url: "/plugin/market/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      const plugin = plugins.find(p => p.id === id);
      if (plugin) {
        const pluginPlans = plans.filter(p => p.pluginId === id);
        const pluginVersions = versions.filter(v => v.pluginId === id);
        const pluginReviews = reviews.filter(r => r.pluginId === id);
        const developer = developers.find(d => d.id === plugin.developerId);
        return resultSuccess({
          ...plugin,
          plans: pluginPlans,
          versions: pluginVersions,
          reviews: pluginReviews.slice(0, 5),
          reviewStats: {
            total: pluginReviews.length,
            avgRating: plugin.rating,
            ratingDistribution: {
              5: pluginReviews.filter(r => r.rating === 5).length,
              4: pluginReviews.filter(r => r.rating === 4).length,
              3: pluginReviews.filter(r => r.rating === 3).length,
              2: pluginReviews.filter(r => r.rating === 2).length,
              1: pluginReviews.filter(r => r.rating === 1).length,
            },
          },
          developer: developer || null,
          relatedPlugins: plugins.filter(p => p.categoryId === plugin.categoryId && p.id !== id).slice(0, 4),
        });
      }
      return resultError(null, '插件不存在', 404);
    },
  },
  {
    url: "/plugin/market/search",
    method: "get",
    response: ({ params }: any) => {
      const keyword = (params.keyword || '').toLowerCase();
      const list = plugins.filter(p => 
        p.name.toLowerCase().includes(keyword) || 
        p.summary.toLowerCase().includes(keyword) ||
        p.tags.some((t: string) => t.toLowerCase().includes(keyword)) ||
        p.developerName.toLowerCase().includes(keyword)
      );
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
        keywords: keyword ? keyword.split(' ') : [],
      });
    },
  },
  {
    url: "/plugin/market/recommend",
    method: "get",
    response: ({ params }: any) => {
      const limit = parseInt(params.limit) || 4;
      const recommendPlugins = plugins
        .filter(p => p.isOfficial === 1 || p.rating >= 4.8)
        .slice(0, limit);
      return resultSuccess(recommendPlugins);
    },
  },
  {
    url: "/plugin/market/hot",
    method: "get",
    response: ({ params }: any) => {
      const limit = parseInt(params.limit) || 4;
      const hotPlugins = [...plugins]
        .sort((a, b) => b.downloadCount - a.downloadCount)
        .slice(0, limit);
      return resultSuccess(hotPlugins);
    },
  },
  {
    url: "/plugin/market/categories",
    method: "get",
    response: () => resultSuccess(categories),
  },
  {
    url: "/plugin/developer/list",
    method: "get",
    response: ({ params }: any) => {
      const developerId = parseInt(params.developerId) || 1;
      let list = plugins.filter(p => p.developerId === developerId);
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(p => p.status === parseInt(params.status));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/developer/add",
    method: "post",
    response: () => resultSuccess({ id: Date.now() }),
  },
  {
    url: "/plugin/developer/edit",
    method: "put",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/developer/delete",
    method: "delete",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/developer/audit",
    method: "post",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/developer/stats",
    method: "get",
    response: () => resultSuccess(developerStats),
  },
  {
    url: "/plugin/developer/register",
    method: "post",
    response: () => resultSuccess({ id: Date.now() }),
  },
  {
    url: "/plugin/developer/profile",
    method: "get",
    response: () => resultSuccess(developers[0]),
  },
  {
    url: "/plugin/developer/update",
    method: "put",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/developer/version/add",
    method: "post",
    response: () => resultSuccess({ id: Date.now() }),
  },
  {
    url: "/plugin/version/list",
    method: "get",
    response: ({ params }: any) => {
      const pluginId = parseInt(params.pluginId);
      return resultSuccess(versions.filter(v => v.pluginId === pluginId));
    },
  },
  {
    url: "/plugin/version/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      return resultSuccess(versions.find(v => v.id === id) || null);
    },
  },
  {
    url: "/plugin/version/latest",
    method: "get",
    response: ({ params }: any) => {
      const pluginId = parseInt(params.pluginId);
      return resultSuccess(versions.find(v => v.pluginId === pluginId && v.isLatest === 1) || null);
    },
  },
  {
    url: "/plugin/plan/list",
    method: "get",
    response: ({ params }: any) => {
      const pluginId = parseInt(params.pluginId);
      return resultSuccess(plans.filter(p => p.pluginId === pluginId));
    },
  },
  {
    url: "/plugin/plan/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      return resultSuccess(plans.find(p => p.id === id) || null);
    },
  },
  {
    url: "/plugin/plan/add",
    method: "post",
    response: () => resultSuccess({ id: Date.now() }),
  },
  {
    url: "/plugin/plan/edit",
    method: "put",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/plan/delete",
    method: "delete",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/cart/list",
    method: "get",
    response: ({ params }: any) => {
      const userId = params.userId ? parseInt(params.userId) : 1;
      const items = cartItems.filter(c => c.userId === userId);
      const totalAmount = items.reduce((sum, item) => sum + (item.selected ? item.price : 0), 0);
      const totalOriginalAmount = items.reduce((sum, item) => sum + (item.selected ? item.originalPrice : 0), 0);
      return resultSuccess({
        list: items,
        total: items.length,
        selectedCount: items.filter(i => i.selected).length,
        totalAmount,
        totalOriginalAmount,
        totalDiscount: totalOriginalAmount - totalAmount,
      });
    },
  },
  {
    url: "/plugin/cart/add",
    method: "post",
    response: ({ body }: any) => {
      const plugin = plugins.find(p => p.id === body.pluginId);
      const planItem = plans.find(p => p.id === body.planId);
      if (!plugin || !planItem) {
        return resultError(null, '插件或套餐不存在', 400);
      }
      return resultSuccess({ id: Date.now() });
    },
  },
  {
    url: "/plugin/cart/remove",
    method: "delete",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/cart/clear",
    method: "delete",
    response: () => resultSuccess(null),
  },
  {
    url: "/plugin/cart/update",
    method: "put",
    response: ({ body }: any) => {
      const item = cartItems.find(c => c.id === body.id);
      if (item) {
        item.selected = body.selected;
      }
      return resultSuccess(null);
    },
  },
  {
    url: "/plugin/order/create",
    method: "post",
    response: ({ body }: any) => {
      const plugin = plugins.find(p => p.id === body.pluginId);
      const planItem = plans.find(p => p.id === body.planId);
      if (!plugin || !planItem) {
        return resultError(null, '插件或套餐不存在', 400);
      }
      const orderNo = `PLM${Date.now()}${Math.floor(Math.random() * 10000).toString().padStart(4, '0')}`;
      return resultSuccess({
        id: Date.now(),
        orderNo,
        pluginId: body.pluginId,
        pluginName: plugin.name,
        planId: body.planId,
        planName: planItem.name,
        amount: planItem.price,
        originalAmount: planItem.originalPrice,
        discountAmount: planItem.originalPrice - planItem.price,
        status: 0,
        statusName: '待支付',
        expireTime: new Date(Date.now() + 15 * 60 * 1000).toISOString(),
        createdAt: new Date().toISOString(),
      });
    },
  },
  {
    url: "/plugin/order/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...orders];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(o => o.status === parseInt(params.status));
      }
      if (params.pluginId) {
        list = list.filter(o => o.pluginId === parseInt(params.pluginId));
      }
      if (params.orderNo) {
        list = list.filter(o => o.orderNo.includes(params.orderNo));
      }
      if (params.startDate) {
        list = list.filter(o => new Date(o.createdAt) >= new Date(params.startDate));
      }
      if (params.endDate) {
        list = list.filter(o => new Date(o.createdAt) <= new Date(params.endDate));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
        statusCount: {
          pending: orders.filter(o => o.status === 0).length,
          paid: orders.filter(o => o.status === 1).length,
          cancelled: orders.filter(o => o.status === 2).length,
          refunded: orders.filter(o => o.status === 3).length,
        },
      });
    },
  },
  {
    url: "/plugin/order/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      const order = orders.find(o => o.id === id);
      if (order) {
        const license = licenses.find(l => l.orderId === id);
        const subscription = subscriptions.find(s => s.orderId === id);
        return resultSuccess({
          ...order,
          license,
          subscription,
        });
      }
      return resultSuccess(null);
    },
  },
  {
    url: "/plugin/order/cancel",
    method: "post",
    response: ({ body }: any) => {
      const id = body.id || body.orderId;
      const order = orders.find(o => o.id === id);
      if (order && order.status === 0) {
        order.status = 2;
        order.statusName = '已取消';
        return resultSuccess({ message: '订单已取消' });
      }
      return resultError(null, '订单无法取消', 400);
    },
  },
  {
    url: "/plugin/order/pay",
    method: "post",
    response: ({ body }: any) => {
      const id = body.id || body.orderId;
      const order = orders.find(o => o.id === id);
      if (order) {
        return resultSuccess({
          orderNo: order.orderNo,
          amount: order.amount,
          qrCode: `weixin://wxpay/bizpayurl?pr=${order.orderNo}`,
          payUrl: `https://wxpay.example.com/pay/${order.orderNo}`,
          expireTime: new Date(Date.now() + 15 * 60 * 1000).toISOString(),
        });
      }
      return resultError(null, '订单不存在', 404);
    },
  },
  {
    url: "/plugin/order/payCallback",
    method: "post",
    response: ({ body }: any) => {
      const order = orders.find(o => o.id === body.orderId);
      if (order) {
        order.status = 1;
        order.statusName = '已支付';
        order.paymentTime = new Date().toISOString();
        order.paymentNo = `PAY${Date.now()}`;
        return resultSuccess({
          success: true,
          orderNo: order.orderNo,
          licenseId: Date.now(),
          subscriptionId: Date.now() + 1,
        });
      }
      return resultError(null, '订单不存在', 404);
    },
  },
  {
    url: "/plugin/subscription/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...subscriptions];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(s => s.status === parseInt(params.status));
      }
      if (params.pluginId) {
        list = list.filter(s => s.pluginId === parseInt(params.pluginId));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
        expiringCount: subscriptions.filter(s => s.daysRemaining > 0 && s.daysRemaining <= 7).length,
      });
    },
  },
  {
    url: "/plugin/subscription/renew",
    method: "post",
    response: ({ body }: any) => {
      const subscription = subscriptions.find(s => s.id === body.subscriptionId);
      if (subscription && subscription.endTime) {
        return resultSuccess({
          success: true,
          newEndTime: new Date(new Date(subscription.endTime).getTime() + body.extendDays * 24 * 60 * 60 * 1000).toISOString(),
        });
      }
      return resultError(null, '订阅不存在', 404);
    },
  },
  {
    url: "/plugin/subscription/cancel",
    method: "post",
    response: ({ body }: any) => {
      const id = body.id || body.subscriptionId;
      const subscription = subscriptions.find(s => s.id === id);
      if (subscription) {
        subscription.autoRenew = 0;
        return resultSuccess({ message: '已关闭自动续费' });
      }
      return resultError(null, '订阅不存在', 404);
    },
  },
  {
    url: "/plugin/license/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...licenses];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(l => l.status === parseInt(params.status));
      }
      if (params.pluginId) {
        list = list.filter(l => l.pluginId === parseInt(params.pluginId));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
        activeCount: licenses.filter(l => l.status === 1).length,
        expiredCount: licenses.filter(l => l.status === 2).length,
      });
    },
  },
  {
    url: "/plugin/license/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      const license = licenses.find(l => l.id === id);
      if (license) {
        const licenseDevices = devices.filter(d => d.licenseId === id);
        const subscription = subscriptions.find(s => s.licenseId === id);
        return resultSuccess({
          ...license,
          devices: licenseDevices,
          subscription,
          verifyHistory: [
            { time: '2024-03-25T14:30:00Z', ip: '192.168.1.100', result: 'success' },
            { time: '2024-03-24T18:20:00Z', ip: '192.168.1.100', result: 'success' },
            { time: '2024-03-23T10:15:00Z', ip: '192.168.1.100', result: 'success' },
          ],
        });
      }
      return resultSuccess(null);
    },
  },
  {
    url: "/plugin/license/bind",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.id === body.licenseId);
      if (license) {
        if (license.bindCount >= license.maxDevices && license.maxDevices !== -1) {
          return resultError(null, '已达到最大设备绑定数', 400);
        }
        const newDevice = {
          id: Date.now(),
          userId: license.userId,
          licenseId: body.licenseId,
          deviceId: body.deviceId,
          deviceName: body.deviceName || '新设备',
          deviceType: body.deviceType || 'unknown',
          osVersion: body.osVersion || '',
          appVersion: body.appVersion || '',
          macAddress: body.macAddress || '',
          ipAddress: body.ipAddress || '',
          registerTime: new Date().toISOString(),
          lastActiveTime: new Date().toISOString(),
          status: 1,
          statusName: '在线',
          createdAt: new Date().toISOString(),
        };
        devices.push(newDevice as any);
        license.bindCount += 1;
        return resultSuccess({
          success: true,
          device: newDevice,
          bindCount: license.bindCount,
          maxDevices: license.maxDevices,
        });
      }
      return resultError(null, '许可证不存在', 404);
    },
  },
  {
    url: "/plugin/license/unbind",
    method: "post",
    response: ({ body }: any) => {
      const deviceIndex = devices.findIndex(d => d.licenseId === body.licenseId && d.deviceId === body.deviceId);
      if (deviceIndex > -1) {
        devices.splice(deviceIndex, 1);
        const license = licenses.find(l => l.id === body.licenseId);
        if (license) {
          license.bindCount = Math.max(0, license.bindCount - 1);
        }
        return resultSuccess({ message: '设备已解绑' });
      }
      return resultError(null, '设备不存在', 404);
    },
  },
  {
    url: "/plugin/license/renew",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.id === body.licenseId);
      if (license && license.endTime) {
        const newEndTime = new Date(new Date(license.endTime).getTime() + body.extendDays * 24 * 60 * 60 * 1000);
        license.endTime = newEndTime.toISOString();
        license.isExpired = false;
        license.daysRemaining = Math.ceil((newEndTime.getTime() - Date.now()) / (24 * 60 * 60 * 1000));
        return resultSuccess({
          success: true,
          newEndTime: license.endTime,
          daysRemaining: license.daysRemaining,
        });
      }
      return resultError(null, '许可证不存在', 404);
    },
  },
  {
    url: "/plugin/license/revoke",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.id === body.licenseId);
      if (license) {
        license.status = 3;
        license.statusName = '已注销';
        return resultSuccess({ message: '许可证已注销' });
      }
      return resultError(null, '许可证不存在', 404);
    },
  },
  {
    url: "/plugin/license/verify",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.licenseKey === body.licenseKey);
      if (license) {
        if (license.status !== 1) {
          return resultSuccess({
            valid: false,
            message: '许可证已禁用或过期',
            code: 'LICENSE_DISABLED',
          });
        }
        if (license.isExpired) {
          return resultSuccess({
            valid: false,
            message: '许可证已过期',
            code: 'LICENSE_EXPIRED',
            expireTime: license.endTime,
          });
        }
        const device = devices.find(d => d.licenseId === license.id && d.deviceId === body.deviceId);
        if (!device && license.bindCount >= license.maxDevices && license.maxDevices !== -1) {
          return resultSuccess({
            valid: false,
            message: '设备数量已达上限',
            code: 'DEVICE_LIMIT_EXCEEDED',
            maxDevices: license.maxDevices,
            currentDevices: license.bindCount,
          });
        }
        license.verifyCount += 1;
        license.lastVerifyTime = new Date().toISOString();
        license.lastVerifyIp = body.deviceInfo?.ipAddress || 'unknown';
        return resultSuccess({
          valid: true,
          licenseKey: license.licenseKey,
          pluginId: license.pluginId,
          pluginName: license.pluginName,
          planId: license.planId,
          planName: license.planName,
          expireTime: license.endTime,
          maxDevices: license.maxDevices,
          features: license.features,
          verifyLevel: license.verifyLevel,
          message: null,
        });
      }
      return resultSuccess({
        valid: false,
        message: '许可证不存在或已失效',
        code: 'LICENSE_NOT_FOUND',
      });
    },
  },
  {
    url: "/plugin/license/heartbeat",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.licenseKey === body.licenseKey);
      if (license && license.status === 1 && !license.isExpired) {
        const device = devices.find(d => d.licenseId === license.id && d.deviceId === body.deviceId);
        if (device) {
          device.lastActiveTime = new Date().toISOString();
          device.status = 1;
        }
        return resultSuccess({
          valid: true,
          message: null,
          serverTime: new Date().toISOString(),
        });
      }
      return resultSuccess({
        valid: false,
        message: '许可证无效或已过期',
        action: 'REAUTH',
      });
    },
  },
  {
    url: "/plugin/device/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...devices];
      if (params.licenseId) {
        list = list.filter(d => d.licenseId === parseInt(params.licenseId));
      }
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(d => d.status === parseInt(params.status));
      }
      return resultSuccess({
        list,
        total: list.length,
        onlineCount: devices.filter(d => d.status === 1).length,
        offlineCount: devices.filter(d => d.status === 0).length,
      });
    },
  },
  {
    url: "/plugin/verify/device/register",
    method: "post",
    response: ({ body }: any) => {
      const license = licenses.find(l => l.id === body.licenseId);
      if (!license) {
        return resultError(null, '许可证不存在', 404);
      }
      const existingDevice = devices.find(d => d.licenseId === body.licenseId && d.deviceId === body.deviceId);
      if (existingDevice) {
        existingDevice.lastActiveTime = new Date().toISOString();
        existingDevice.status = 1;
        return resultSuccess({
          success: true,
          device: existingDevice,
          isNew: false,
        });
      }
      const newDevice = {
        id: Date.now(),
        userId: license.userId,
        licenseId: body.licenseId,
        deviceId: body.deviceId,
        deviceName: body.deviceInfo?.deviceName || '新设备',
        deviceType: body.deviceInfo?.deviceType || 'unknown',
        osVersion: body.deviceInfo?.osVersion || '',
        macAddress: body.deviceInfo?.macAddress || '',
        ipAddress: body.ipAddress || '',
        registerTime: new Date().toISOString(),
        lastActiveTime: new Date().toISOString(),
        status: 1,
        statusName: '在线',
        createdAt: new Date().toISOString(),
      };
      devices.push(newDevice as any);
      return resultSuccess({
        success: true,
        device: newDevice,
        isNew: true,
      });
    },
  },
  {
    url: "/plugin/device/unbind",
    method: "post",
    response: ({ body }: any) => {
      const deviceIndex = devices.findIndex(d => d.id === body.deviceId);
      if (deviceIndex > -1) {
        const device = devices[deviceIndex];
        const license = licenses.find(l => l.id === device.licenseId);
        if (license) {
          license.bindCount = Math.max(0, license.bindCount - 1);
        }
        devices.splice(deviceIndex, 1);
        return resultSuccess({ message: '设备已解绑' });
      }
      return resultError(null, '设备不存在', 404);
    },
  },
  {
    url: "/plugin/verify/code/send",
    method: "post",
    response: ({ body }: any) => {
      const code = Math.floor(100000 + Math.random() * 900000).toString();
      const key = `${body.licenseId}_${body.purpose}`;
      verificationCodes[key] = {
        code,
        expireTime: Date.now() + 5 * 60 * 1000,
        attempts: 0,
      };
      return resultSuccess({
        success: true,
        code,
        expireIn: 300,
        message: '验证码已发送',
      });
    },
  },
  {
    url: "/plugin/verify/code/check",
    method: "post",
    response: ({ body }: any) => {
      const key = `${body.licenseId}_0`;
      const storedCode = verificationCodes[key];
      if (!storedCode) {
        return resultSuccess({
          success: false,
          message: '验证码已过期，请重新获取',
        });
      }
      if (storedCode.attempts >= 3) {
        delete verificationCodes[key];
        return resultSuccess({
          success: false,
          message: '验证码尝试次数过多，请重新获取',
        });
      }
      if (Date.now() > storedCode.expireTime) {
        delete verificationCodes[key];
        return resultSuccess({
          success: false,
          message: '验证码已过期',
        });
      }
      if (body.code === storedCode.code) {
        delete verificationCodes[key];
        return resultSuccess({
          success: true,
          message: '验证成功',
        });
      }
      storedCode.attempts += 1;
      return resultSuccess({
        success: false,
        message: `验证码错误，还剩 ${3 - storedCode.attempts} 次机会`,
        attemptsLeft: 3 - storedCode.attempts,
      });
    },
  },
  {
    url: "/plugin/verify/obfuscation/config",
    method: "get",
    response: ({ params }: any) => {
      const plugin = plugins.find(p => p.id === parseInt(params.pluginId));
      if (plugin) {
        return resultSuccess({
          enabled: plugin.verifyLevel > 0,
          level: plugin.verifyLevel,
          config: {
            codeObfuscation: plugin.verifyLevel >= 1,
            stringEncryption: plugin.verifyLevel >= 1,
            controlFlowFlattening: plugin.verifyLevel >= 2,
            deadCodeInjection: plugin.verifyLevel >= 2,
            antiDebug: plugin.verifyLevel >= 2,
            codeVirtualization: plugin.verifyLevel >= 2,
          },
          settings: {
            licenseCheckInterval: 3600,
            maxOfflineDays: 7,
            heartbeatInterval: 300,
          },
        });
      }
      return resultSuccess({
        enabled: false,
        level: 0,
        config: {},
        settings: {},
      });
    },
  },
  {
    url: "/plugin/card/generate",
    method: "post",
    response: ({ body }: any) => {
      const batchNo = `BATCH${Date.now()}${Math.floor(Math.random() * 1000).toString().padStart(3, '0')}`;
      return resultSuccess({
        batchId: Date.now(),
        batchNo,
        count: body.count,
        pluginId: body.pluginId,
        planId: body.planId,
        price: body.price,
        expireDays: body.expireDays,
      });
    },
  },
  {
    url: "/plugin/card/batch/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...cardBatches];
      if (params.pluginId) {
        list = list.filter(b => b.pluginId === parseInt(params.pluginId));
      }
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(b => b.status === parseInt(params.status));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/card/batch/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const id = parseInt(params.id);
      const batch = cardBatches.find(b => b.id === id);
      if (batch) {
        const batchCards = cards.filter(c => c.batchId === id);
        return resultSuccess({
          ...batch,
          cards: batchCards.slice(0, 20),
          cardTotal: batchCards.length,
          statusDistribution: {
            unused: batchCards.filter(c => c.status === 0).length,
            used: batchCards.filter(c => c.status === 1).length,
            expired: batchCards.filter(c => c.status === 2).length,
            frozen: batchCards.filter(c => c.status === 3).length,
          },
        });
      }
      return resultSuccess(null);
    },
  },
  {
    url: "/plugin/card/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...cards];
      if (params.batchId) {
        list = list.filter(c => c.batchId === parseInt(params.batchId));
      }
      if (params.pluginId) {
        list = list.filter(c => c.pluginId === parseInt(params.pluginId));
      }
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(c => c.status === parseInt(params.status));
      }
      if (params.cardNo) {
        list = list.filter(c => c.cardNo.includes(params.cardNo));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/card/redeem",
    method: "post",
    response: ({ body }: any) => {
      const card = cards.find(c => c.cardNo === body.cardNo && c.cardPwd === body.cardPwd);
      if (!card) {
        return resultError(null, '卡号或卡密错误', 400);
      }
      if (card.status === 1) {
        return resultError(null, '该卡密已被使用', 400);
      }
      if (card.status === 2) {
        return resultError(null, '该卡密已过期', 400);
      }
      if (card.status === 3) {
        return resultError(null, '该卡密已被冻结', 400);
      }
      const orderNo = `PLM${Date.now()}${Math.floor(Math.random() * 10000).toString().padStart(4, '0')}`;
      const licenseKey = `${Date.now().toString(36)}-${Math.random().toString(36).substr(2, 4)}-${Math.random().toString(36).substr(2, 4)}-${Math.random().toString(36).substr(2, 4)}`;
      card.status = 1;
      card.statusName = '已使用';
      card.usedBy = '张三';
      card.usedById = 1;
      card.usedAt = new Date().toISOString();
      card.usedOrderId = Date.now();
      return resultSuccess({
        success: true,
        orderNo,
        licenseKey,
        pluginId: card.pluginId,
        pluginName: card.pluginName,
        planId: card.planId,
        planName: card.planName,
        expireDays: card.expireDays,
        message: '卡密兑换成功',
      });
    },
  },
  {
    url: "/plugin/card/freeze",
    method: "post",
    response: ({ body }: any) => {
      const card = cards.find(c => c.id === body.cardId);
      if (card) {
        card.status = 3;
        card.statusName = '已冻结';
        return resultSuccess({ message: '卡密已冻结' });
      }
      return resultError(null, '卡密不存在', 404);
    },
  },
  {
    url: "/plugin/card/unfreeze",
    method: "post",
    response: ({ body }: any) => {
      const card = cards.find(c => c.id === body.cardId);
      if (card) {
        card.status = 0;
        card.statusName = '未使用';
        return resultSuccess({ message: '卡密已解冻' });
      }
      return resultError(null, '卡密不存在', 404);
    },
  },
  {
    url: "/plugin/card/batch/export",
    method: "get",
    response: ({ params }: any) => {
      const batch = cardBatches.find(b => b.id === parseInt(params.batchId));
      if (batch) {
        const batchCards = cards.filter(c => c.batchId === batch.id);
        return resultSuccess({
          batchNo: batch.batchNo,
          count: batchCards.length,
          cards: batchCards.map(c => ({
            cardNo: c.cardNo,
            cardPwd: c.cardPwd,
            status: c.statusName,
            expireTime: c.expireTime,
          })),
          exportTime: new Date().toISOString(),
        });
      }
      return resultError(null, '批次不存在', 404);
    },
  },
  {
    url: "/plugin/review/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...reviews];
      if (params.pluginId) {
        list = list.filter(r => r.pluginId === parseInt(params.pluginId));
      }
      if (params.rating) {
        list = list.filter(r => r.rating === parseInt(params.rating));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/review/add",
    method: "post",
    response: () => resultSuccess({ id: Date.now() }),
  },
  {
    url: "/plugin/review/helpful",
    method: "post",
    response: ({ body }: any) => {
      const review = reviews.find(r => r.id === body.reviewId);
      if (review) {
        review.likeCount += 1;
        return resultSuccess({ likeCount: review.likeCount });
      }
      return resultError(null, '评论不存在', 404);
    },
  },
  {
    url: "/plugin/review/reply",
    method: "post",
    response: ({ body }: any) => {
      const review = reviews.find(r => r.id === body.reviewId);
      if (review) {
        review.replyContent = body.content;
        review.replyTime = new Date().toISOString();
        return resultSuccess({ message: '回复成功' });
      }
      return resultError(null, '评论不存在', 404);
    },
  },
  {
    url: "/plugin/developer/income/overview",
    method: "get",
    response: () => resultSuccess({
      totalIncome: 125680.50,
      monthIncome: 28560.00,
      todayIncome: 1256.00,
      withdrawable: 56800.50,
      frozen: 0,
      withdrawn: 68880.00,
      pendingWithdraw: 20000.00,
      incomeTrend: [
        { date: '2024-03-01', amount: 3560 },
        { date: '2024-03-02', amount: 4280 },
        { date: '2024-03-03', amount: 5120 },
        { date: '2024-03-04', amount: 3890 },
        { date: '2024-03-05', amount: 6780 },
        { date: '2024-03-06', amount: 5430 },
        { date: '2024-03-07', amount: 7120 },
      ],
      pluginIncome: [
        { pluginId: 1, pluginName: '智能优惠券', income: 68500.00, orders: 189 },
        { pluginId: 5, pluginName: '数据统计分析', income: 38900.00, orders: 98 },
        { pluginId: 8, pluginName: '幸运抽奖', income: 18280.50, orders: 69 },
      ],
    }),
  },
  {
    url: "/plugin/developer/income/list",
    method: "get",
    response: ({ params }: any) => {
      const list = [
        { id: 1, type: 1, typeName: '插件销售', amount: 299, pluginName: '智能优惠券', orderNo: 'PLM20260325001', status: 1, createdAt: '2024-03-25T10:00:00Z' },
        { id: 2, type: 1, typeName: '插件销售', amount: 399, pluginName: '数据统计分析', orderNo: 'PLM20260325002', status: 1, createdAt: '2024-03-25T09:30:00Z' },
        { id: 3, type: 1, typeName: '插件销售', amount: 799, pluginName: '智能优惠券', orderNo: 'PLM20260325003', status: 1, createdAt: '2024-03-25T09:00:00Z' },
        { id: 4, type: 2, typeName: '续费收入', amount: 299, pluginName: '智能优惠券', orderNo: 'PLM20260324001', status: 1, createdAt: '2024-03-24T16:00:00Z' },
        { id: 5, type: 3, typeName: '退款扣除', amount: -299, pluginName: '数据统计分析', orderNo: 'PLM20260324002', status: 1, createdAt: '2024-03-24T14:00:00Z' },
      ];
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/developer/withdraw/apply",
    method: "post",
    response: ({ body }: any) => {
      return resultSuccess({
        success: true,
        withdrawId: Date.now(),
        amount: body.amount,
        status: 0,
        statusName: '处理中',
        applyTime: new Date().toISOString(),
        message: '提现申请已提交，预计1-3个工作日到账',
      });
    },
  },
  {
    url: "/plugin/developer/withdraw/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...withdrawRecords];
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/developer/withdraw/detail/:id",
    method: "get",
    response: ({ params }: any) => {
      const record = withdrawRecords.find(r => r.id === parseInt(params.id));
      return resultSuccess(record || null);
    },
  },
  {
    url: "/plugin/developer/bank/info",
    method: "get",
    response: () => resultSuccess({
      bankName: '招商银行',
      bankCode: 'CMB',
      bankAccount: '**** **** **** 8888',
      accountName: '奇络科技',
      branchName: '深圳南山支行',
      status: 1,
      verified: true,
    }),
  },
  {
    url: "/plugin/developer/bank/update",
    method: "post",
    response: () => resultSuccess({ message: '银行账户信息已更新' }),
  },
  {
    url: "/plugin/admin/overview",
    method: "get",
    response: () => resultSuccess({
      totalPlugins: 156,
      activePlugins: 142,
      totalDevelopers: 45,
      verifiedDevelopers: 38,
      totalOrders: 3560,
      todayOrders: 89,
      totalRevenue: 1256800.00,
      monthRevenue: 285600.00,
      totalUsers: 12560,
      activeUsers: 8960,
      pendingReviews: 12,
      pendingAudits: 5,
      recentTrend: [
        { date: '2024-03-01', orders: 120, revenue: 35600 },
        { date: '2024-03-02', orders: 135, revenue: 42800 },
        { date: '2024-03-03', orders: 142, revenue: 51200 },
        { date: '2024-03-04', orders: 128, revenue: 38900 },
        { date: '2024-03-05', orders: 156, revenue: 67800 },
        { date: '2024-03-06', orders: 148, revenue: 54300 },
        { date: '2024-03-07', orders: 165, revenue: 71200 },
      ],
      categoryDistribution: [
        { name: '营销工具', count: 45, percentage: 28.8 },
        { name: '客服系统', count: 32, percentage: 20.5 },
        { name: '数据分析', count: 28, percentage: 17.9 },
        { name: '支付收款', count: 22, percentage: 14.1 },
        { name: '其他', count: 29, percentage: 18.7 },
      ],
    }),
  },
  {
    url: "/plugin/admin/plugin/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...plugins];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(p => p.status === parseInt(params.status));
      }
      if (params.verifyLevel !== undefined && params.verifyLevel !== '') {
        list = list.filter(p => p.verifyLevel === parseInt(params.verifyLevel));
      }
      if (params.keyword) {
        const keyword = params.keyword.toLowerCase();
        list = list.filter(p => 
          p.name.toLowerCase().includes(keyword) || 
          p.code.toLowerCase().includes(keyword)
        );
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/admin/plugin/audit",
    method: "post",
    response: ({ body }: any) => {
      const plugin = plugins.find(p => p.id === body.pluginId);
      if (plugin) {
        plugin.status = body.status;
        plugin.statusName = body.status === 1 ? '上架' : '已下架';
        return resultSuccess({ message: '审核完成' });
      }
      return resultError(null, '插件不存在', 404);
    },
  },
  {
    url: "/plugin/admin/plugin/offline",
    method: "post",
    response: ({ body }: any) => {
      const plugin = plugins.find(p => p.id === body.pluginId);
      if (plugin) {
        plugin.status = 0;
        plugin.statusName = '已下架';
        return resultSuccess({ message: '插件已下架' });
      }
      return resultError(null, '插件不存在', 404);
    },
  },
  {
    url: "/plugin/admin/developer/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...developers];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(d => d.status === parseInt(params.status));
      }
      if (params.verified !== undefined && params.verified !== '') {
        list = list.filter(d => d.verified === (params.verified === 'true' || params.verified === '1'));
      }
      if (params.keyword) {
        const keyword = params.keyword.toLowerCase();
        list = list.filter(d => 
          d.name.toLowerCase().includes(keyword) || 
          d.contact.toLowerCase().includes(keyword)
        );
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/admin/developer/verify",
    method: "post",
    response: ({ body }: any) => {
      const developer = developers.find(d => d.id === body.developerId);
      if (developer) {
        developer.verified = body.verified;
        developer.verifiedAt = body.verified ? new Date().toISOString() : null;
        return resultSuccess({ message: body.verified ? '已通过认证' : '已取消认证' });
      }
      return resultError(null, '开发者不存在', 404);
    },
  },
  {
    url: "/plugin/admin/order/list",
    method: "get",
    response: ({ params }: any) => {
      let list = [...orders];
      if (params.status !== undefined && params.status !== '') {
        list = list.filter(o => o.status === parseInt(params.status));
      }
      if (params.orderNo) {
        list = list.filter(o => o.orderNo.includes(params.orderNo));
      }
      if (params.startDate) {
        list = list.filter(o => new Date(o.createdAt) >= new Date(params.startDate));
      }
      if (params.endDate) {
        list = list.filter(o => new Date(o.createdAt) <= new Date(params.endDate));
      }
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/admin/order/refund",
    method: "post",
    response: ({ body }: any) => {
      const order = orders.find(o => o.id === body.orderId);
      if (order) {
        order.status = 3;
        order.statusName = '已退款';
        return resultSuccess({ message: '退款成功' });
      }
      return resultError(null, '订单不存在', 404);
    },
  },
  {
    url: "/plugin/admin/statistics/sales",
    method: "get",
    response: ({ params }: any) => {
      const days = parseInt(params.days) || 7;
      const data = [];
      for (let i = days - 1; i >= 0; i--) {
        const date = new Date();
        date.setDate(date.getDate() - i);
        data.push({
          date: date.toISOString().split('T')[0],
          orders: Math.floor(Math.random() * 100) + 50,
          revenue: Math.floor(Math.random() * 50000) + 20000,
          newUsers: Math.floor(Math.random() * 200) + 100,
        });
      }
      return resultSuccess({
        data,
        summary: {
          totalOrders: data.reduce((sum, d) => sum + d.orders, 0),
          totalRevenue: data.reduce((sum, d) => sum + d.revenue, 0),
          totalNewUsers: data.reduce((sum, d) => sum + d.newUsers, 0),
          avgOrdersPerDay: Math.floor(data.reduce((sum, d) => sum + d.orders, 0) / days),
          avgRevenuePerDay: Math.floor(data.reduce((sum, d) => sum + d.revenue, 0) / days),
        },
      });
    },
  },
  {
    url: "/plugin/admin/statistics/plugins",
    method: "get",
    response: () => resultSuccess({
      topDownloads: plugins.slice(0, 10).map(p => ({
        id: p.id,
        name: p.name,
        downloads: p.downloadCount,
        revenue: Math.floor(Math.random() * 100000) + 10000,
      })),
      topRevenue: plugins.slice(0, 10).map(p => ({
        id: p.id,
        name: p.name,
        revenue: Math.floor(Math.random() * 100000) + 10000,
        orders: Math.floor(Math.random() * 500) + 100,
      })),
      topRated: plugins
        .sort((a, b) => b.rating - a.rating)
        .slice(0, 10)
        .map(p => ({
          id: p.id,
          name: p.name,
          rating: p.rating,
          reviewCount: p.reviewCount,
        })),
    }),
  },
  {
    url: "/plugin/admin/statistics/developers",
    method: "get",
    response: () => resultSuccess({
      topDevelopers: developers.slice(0, 10).map(d => ({
        id: d.id,
        name: d.name,
        pluginCount: d.pluginCount,
        totalSales: d.totalSales,
        revenue: Math.floor(Math.random() * 200000) + 50000,
      })),
      newDevelopers: [
        { id: 1, name: '新开发者A', pluginCount: 1, createdAt: '2024-03-25T10:00:00Z' },
        { id: 2, name: '新开发者B', pluginCount: 2, createdAt: '2024-03-24T15:00:00Z' },
        { id: 3, name: '新开发者C', pluginCount: 1, createdAt: '2024-03-23T09:00:00Z' },
      ],
    }),
  },
  {
    url: "/plugin/admin/report/generate",
    method: "post",
    response: ({ body }: any) => {
      return resultSuccess({
        reportId: Date.now(),
        reportType: body.reportType,
        status: 0,
        statusName: '生成中',
        message: '报表生成任务已创建，请稍后在报表列表中查看',
      });
    },
  },
  {
    url: "/plugin/admin/report/list",
    method: "get",
    response: ({ params }: any) => {
      const list = [
        { id: 1, reportType: 'sales', reportTypeName: '销售报表', period: '2024-03', status: 1, statusName: '已完成', fileUrl: '/reports/sales_202403.xlsx', createdAt: '2024-03-25T10:00:00Z' },
        { id: 2, reportType: 'plugin', reportTypeName: '插件报表', period: '2024-03', status: 1, statusName: '已完成', fileUrl: '/reports/plugin_202403.xlsx', createdAt: '2024-03-24T15:00:00Z' },
        { id: 3, reportType: 'developer', reportTypeName: '开发者报表', period: '2024-03', status: 0, statusName: '生成中', fileUrl: null, createdAt: '2024-03-25T14:00:00Z' },
      ];
      const pageNum = parseInt(params.pageNum) || 1;
      const pageSize = parseInt(params.pageSize) || 10;
      const start = (pageNum - 1) * pageSize;
      return resultSuccess({
        list: list.slice(start, start + pageSize),
        total: list.length,
        pageNum,
        pageSize,
      });
    },
  },
  {
    url: "/plugin/admin/report/download",
    method: "get",
    response: ({ params }: any) => {
      return resultSuccess({
        downloadUrl: `/reports/report_${params.reportId}.xlsx`,
        fileName: `report_${params.reportId}.xlsx`,
        fileSize: 1024000,
      });
    },
  },
];

export default mockPluginMarket;