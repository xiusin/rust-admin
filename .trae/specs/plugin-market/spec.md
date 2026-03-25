# 插件市场/应用市场系统 - 技术规格文档

## 1. 背景与目标

### 1.1 项目定位

构建一个现代化的**插件市场/应用市场平台**，为商户和开发者提供插件化、模块化的功能组件交易生态系统。平台支持第三方开发者发布插件、商户购买订阅、在线支付、套餐管理、版本更新等完整商业化流程，同时集成发码验证、设备绑定、代码混淆等防破解安全机制。

### 1.2 核心价值

- **商户侧**：一站式获取所需插件，无需自行开发
- **开发者侧**：插件变现渠道，降低分发成本
- **平台侧**：交易抽成，构建商业生态

---

## 2. 系统架构设计

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        前端展示层 (Vue 3)                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │
│  │  插件市场   │ │  购物车     │ │  订单管理   │ │  插件管理 │ │
│  │  PluginMall │ │  Cart      │ │  Orders    │ │  MyApps  │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                       API 网关层 (Axum)                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    统一鉴权 / 限流 / 监控                     ││
│  └─────────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────────┤
│                      业务服务层 (Application)                    │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────────────┐ │
│  │  plugin_service │ │  order_service │ │  license_service     │ │
│  │  插件服务      │ │  订单服务      │ │  授权/License服务     │ │
│  └───────────────┘ └───────────────┘ └───────────────────────┘ │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────────────┐ │
│  │ payment_service│ │  verify_service│ │  subscription_service │ │
│  │  支付服务      │ │  验证服务      │ │  订阅套餐服务         │ │
│  └───────────────┘ └───────────────┘ └───────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                      领域模型层 (Domain)                          │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────────────┐ │
│  │  Plugin       │ │  Order        │ │  License              │ │
│  │  插件实体     │ │  订单实体      │ │  License实体          │ │
│  └───────────────┘ └───────────────┘ └───────────────────────┘ │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────────────┐ │
│  │  Plan         │ │  Purchase     │ │  Verification         │ │
│  │  套餐实体     │ │  购买记录实体   │ │  验证实体             │ │
│  └───────────────┘ └───────────────┘ └───────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                      基础设施层 (Infrastructure)                   │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────────────┐ │
│  │  Redis Cache  │ │  MySQL DB     │ │  File Storage (OSS)   │ │
│  └───────────────┘ └───────────────┘ └───────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 核心数据流

```
商户搜索插件 → 查看详情 → 选择套餐 → 加入购物车/立即购买
     ↓
创建订单 → 发起支付 → 支付成功 → 生成License → 发送验证码/授权码
     ↓
插件市场下载 → 本地验证License → 插件运行
     ↓
到期提醒 → 续费/升级 → 重新验证
```

---

## 3. 功能模块详细设计

### 3.1 插件管理模块 (Plugin Management)

#### 3.1.1 插件实体设计

```rust
// 插件基础信息
struct Plugin {
    id: i64,                      // 插件唯一标识
    code: String,                  // 插件编码（唯一，用于防破解校验）
    name: String,                  // 插件名称
    category_id: i64,              // 所属分类
    developer_id: i64,             // 开发者ID
    summary: String,               // 简介
    description: Text,             // 详细描述（支持Markdown）
    cover_image: String,           // 封面图
    screenshots: Vec<String>,       // 截图列表
    version: String,               // 当前版本
    price_type: PriceType,         // 价格类型：免费/一次性/订阅
    prices: Vec<PlanPrice>,        // 各套餐价格
    tags: Vec<String>,             // 标签
    download_count: i64,            // 下载次数
    rating: f64,                   // 评分
    status: PluginStatus,          // 状态：待审核/上架/下架/回收站
    is_official: bool,             // 是否官方插件
    verify_level: VerifyLevel,     // 验证级别：无/基础/高级
    created_at: DateTime,
    updated_at: DateTime,
}

enum PriceType {
    Free,      // 免费
    OneTime,  // 一次性买断
    Subscription, // 订阅（月/年）
}

enum PluginStatus {
    Pending,   // 待审核
    Active,   // 上架
    Inactive, // 下架
    Trash,    // 回收站
}

enum VerifyLevel {
    None,     // 无验证
    Basic,    // 基础验证（设备绑定）
    Advanced, // 高级验证（设备+发码+混淆）
}
```

#### 3.1.2 插件版本管理

```rust
struct PluginVersion {
    id: i64,
    plugin_id: i64,
    version: String,           // 版本号（语义化：1.0.0）
    changelog: String,         // 变更日志
    download_url: String,      // 下载地址
    file_hash: String,         // 文件哈希（SHA256）
    file_size: i64,            // 文件大小
    min_app_version: String,   // 最低兼容版本
    is_latest: bool,           // 是否最新
    status: VersionStatus,
    created_at: DateTime,
}
```

### 3.2 套餐订阅模块 (Plan & Subscription)

#### 3.2.1 套餐实体

```rust
struct Plan {
    id: i64,
    plugin_id: i64,
    name: String,               // 套餐名称：基础版/专业版/企业版
    description: String,        // 套餐描述
    period_type: PeriodType,    // 周期类型
    period_days: i32,           // 周期天数（月付30天，年付365天）
    price: Decimal,             // 价格
    original_price: Decimal,    // 原价
    features: Vec<Feature>,     // 功能列表
    limits: PlanLimits,         // 限制条件
    sort: i32,
    status: bool,               // 是否启用
    created_at: DateTime,
}

struct PeriodType {
    Month,   // 月付
    Quarter, // 季付
    Year,    // 年付
    Forever, // 永久
}

struct PlanLimits {
    max_devices: i32,           // 最大设备数
    max_users: i32,             // 最大用户数
    storage_limit: i64,         // 存储限制（MB）
    api_calls_limit: i64,       // API调用限制（次/天）
    support_level: SupportLevel, // 客服级别
}

struct Feature {
    code: String,               // 功能编码
    name: String,                // 功能名称
    included: bool,             // 是否包含
    limit: Option<i64>,         // 限制数量
}
```

#### 3.2.2 订阅记录

```rust
struct Subscription {
    id: i64,
    user_id: i64,
    plugin_id: i64,
    plan_id: i64,
    order_id: i64,
    start_time: DateTime,
    end_time: DateTime,         // 到期时间
    auto_renew: bool,            // 自动续费
    status: SubscriptionStatus,
    created_at: DateTime,
    updated_at: DateTime,
}

enum SubscriptionStatus {
    Active,    // 生效中
    Expired,   // 已过期
    Cancelled, // 已取消
    Pending,   // 待生效
}
```

### 3.3 订单交易模块 (Order & Payment)

#### 3.3.1 订单实体

```rust
struct Order {
    id: i64,
    order_no: String,            // 订单号
    user_id: i64,
    plugin_id: i64,
    plan_id: i64,
    amount: Decimal,             // 支付金额
    original_amount: Decimal,    // 原价金额
    discount_amount: Decimal,     // 优惠金额
    coupon_id: Option<i64>,      // 使用的优惠券
    payment_method: PaymentMethod,
    payment_time: Option<DateTime>,
    status: OrderStatus,
    expire_time: DateTime,       // 订单过期时间
    created_at: DateTime,
}

enum OrderStatus {
    Pending,    // 待支付
    Paid,       // 已支付
    Cancelled,  // 已取消
    Refunded,   // 已退款
    Expired,    // 已过期
}

enum PaymentMethod {
    Wechat,     // 微信支付
    Alipay,     // 支付宝
    Card,       // 卡密支付
    Balance,    // 余额支付
}
```

#### 3.3.2 购物车

```rust
struct CartItem {
    id: i64,
    user_id: i64,
    plugin_id: i64,
    plan_id: i64,
    created_at: DateTime,
}
```

### 3.4 License 授权模块 (License & Anti-Piracy)

#### 3.4.1 License 实体

```rust
struct License {
    id: i64,
    license_key: String,         // License密钥（UUID格式）
    user_id: i64,
    plugin_id: i64,
    plan_id: i64,
    order_id: Option<i64>,
    device_id: String,           // 设备ID
    device_info: DeviceInfo,     // 设备信息
    status: LicenseStatus,
    start_time: DateTime,
    end_time: DateTime,          // 有效期截止
    verify_count: i32,           // 验证次数
    last_verify_time: DateTime,
    bind_count: i32,             // 绑定次数
    created_at: DateTime,
    updated_at: DateTime,
}

struct DeviceInfo {
    device_id: String,           // 设备指纹
    device_name: String,         // 设备名称
    device_type: String,         // 设备类型
    os_version: String,          // 系统版本
    app_version: String,         // 应用版本
    ip_address: String,          // 注册IP
    mac_address: String,         // MAC地址
}
```

#### 3.4.2 防破解验证机制

```rust
// 验证码生成与验证
struct VerificationCode {
    id: i64,
    code: String,                 // 6位数字验证码
    license_id: i64,
    user_id: i64,
    plugin_id: i64,
    purpose: VerifyPurpose,       // 验证用途
    device_hash: String,          // 设备哈希
    status: CodeStatus,
    attempts: i32,               // 尝试次数
    expire_time: DateTime,
    verified_at: Option<DateTime>,
    created_at: DateTime,
}

enum VerifyPurpose {
    Activate,     // 激活验证
    Login,        // 登录验证
    Heartbeat,    // 心跳验证
    Update,       // 更新验证
}

enum CodeStatus {
    Unused,       // 未使用
    Used,         // 已使用
    Expired,      // 已过期
    Blocked,      // 已封禁
}
```

#### 3.4.3 混淆码机制

```rust
// 代码混淆配置
struct ObfuscationConfig {
    id: i64,
    plugin_id: i64,
    version: String,
    obfuscation_level: ObfuscationLevel,
    enabled_modules: Vec<String>,  // 启用的混淆模块
    license_check_interval: i64,    // License检查间隔（秒）
    max_offline_days: i32,          // 最大离线天数
    anti_debug_enabled: bool,       // 防调试
    code_virtualization: bool,     // 代码虚拟化
}

enum ObfuscationLevel {
    None,
    Basic,    // 基础混淆
    Standard, // 标准混淆
    Advanced, // 高级混淆（包含代码虚拟化）
}
```

### 3.5 插件分类模块

```rust
struct PluginCategory {
    id: i64,
    name: String,
    icon: String,
    parent_id: Option<i64>,
    sort: i32,
    plugin_count: i64,
    status: bool,
}
```

预设分类：
- **营销工具**：优惠券、抽奖、拼团、秒杀
- **客服系统**：在线客服、工单系统、呼叫中心
- **数据分析**：数据统计、用户行为分析、报表
- **支付收款**：支付通道、账务管理、对账结算
- **物流管理**：快递查询、打印面单、仓储管理
- **会员管理**：会员体系、积分系统、等级权益
- **内容运营**：CMS、文章管理、评论系统
- **企业微信**：企微助手、裂变工具、客户管理

---

## 4. API 接口设计

### 4.1 插件市场 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/plugin/market/list` | GET | 获取插件市场列表 |
| `/plugin/market/detail/{id}` | GET | 获取插件详情 |
| `/plugin/market/categories` | GET | 获取插件分类 |
| `/plugin/market/search` | GET | 搜索插件 |
| `/plugin/market/recommend` | GET | 推荐插件 |
| `/plugin/market/hot` | GET | 热门插件 |

### 4.2 插件管理 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/plugin/developer/list` | GET | 开发者插件列表 |
| `/plugin/developer/add` | POST | 提交插件 |
| `/plugin/developer/edit` | PUT | 编辑插件 |
| `/plugin/developer/delete` | DELETE | 删除插件 |
| `/plugin/developer/version/add` | POST | 发布新版本 |
| `/plugin/developer/stats` | GET | 销售统计 |

### 4.3 交易订单 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/order/cart/list` | GET | 购物车列表 |
| `/order/cart/add` | POST | 加入购物车 |
| `/order/cart/remove` | DELETE | 移除购物车 |
| `/order/create` | POST | 创建订单 |
| `/order/list` | GET | 订单列表 |
| `/order/detail/{id}` | GET | 订单详情 |
| `/order/cancel/{id}` | POST | 取消订单 |
| `/order/pay/{id}` | POST | 发起支付 |
| `/order/pay/callback` | POST | 支付回调 |

### 4.4 License API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/license/list` | GET | 我的License列表 |
| `/license/detail/{id}` | GET | License详情 |
| `/license/bind` | POST | 绑定设备 |
| `/license/unbind` | POST | 解绑设备 |
| `/license/renew` | POST | 续费License |
| `/license/verify` | POST | 验证License |
| `/license/heartbeat` | POST | 心跳检测 |

### 4.5 验证码 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/verify/code/send` | POST | 发送验证码 |
| `/verify/code/check` | POST | 校验验证码 |
| `/verify/device/register` | POST | 注册设备 |
| `/verify/obfuscation/config` | GET | 获取混淆配置 |

### 4.6 卡密相关 API

| 接口 | 方法 | 描述 |
|------|------|------|
| `/card/generate` | POST | 生成卡密（后台） |
| `/card/redeem` | POST | 卡密兑换 |
| `/card/batch/export` | GET | 批量导出卡密 |

---

## 5. 前端页面设计

### 5.1 页面结构

```
├── plugin-market/              # 插件市场
│   ├── index.vue               # 市场首页（分类展示）
│   ├── list.vue                # 插件列表页
│   ├── detail.vue              # 插件详情页
│   └── search.vue              # 搜索结果页
│
├── plugin-my/                  # 我的插件
│   ├── purchased.vue           # 已购插件
│   ├── subscription.vue        # 订阅管理
│   └── license.vue             # License管理
│
├── plugin-developer/            # 开发者中心
│   ├── dashboard.vue           # 开发者仪表盘
│   ├── plugin-list.vue        # 插件管理
│   ├── plugin-edit.vue        # 插件编辑
│   ├── version.vue             # 版本管理
│   └── sales.vue               # 销售统计
│
├── order/                      # 订单中心
│   ├── cart.vue                # 购物车
│   ├── checkout.vue            # 结账页面
│   ├── list.vue                # 订单列表
│   └── detail.vue              # 订单详情
│
└── verify/                     # 验证中心
    ├── activate.vue            # 激活页面
    ├── device.vue              # 设备管理
    └── card-redeem.vue         # 卡密兑换
```

### 5.2 关键页面设计要点

#### 5.2.1 插件市场首页

```
┌─────────────────────────────────────────────────────────────────┐
│  🔍 搜索插件...                        [购物车(3)] [我的插件]    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │  营销   │ │  客服   │ │ 数据分析│ │  支付   │ │  物流   │  │
│  │  工具   │ │  系统   │ │         │ │  收款   │ │  管理   │  │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │
│                                                                 │
│  ─────────── 为您推荐 ───────────                              │
│                                                                 │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────┐        │
│  │   [封面图]    │ │   [封面图]    │ │   [封面图]    │        │
│  │               │ │               │ │               │        │
│  │  插件名称     │ │  插件名称     │ │  插件名称     │        │
│  │  ★★★★☆ 4.5  │ │  ★★★★★ 5.0   │ │  ★★★☆☆ 3.8  │        │
│  │  ¥299/年 起  │ │  免费        │ │  ¥999/永久   │        │
│  │  下载 1.2k   │ │  下载 5.6k   │ │  下载 890    │        │
│  └───────────────┘ └───────────────┘ └───────────────┘        │
│                                                                 │
│  ─────────── 热门插件 ───────────                              │
│  ...                                                            │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.2.2 插件详情页

```
┌─────────────────────────────────────────────────────────────────┐
│  ← 返回                                                       │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  插件名称                          [★★★★★]  │
│  │              │  插件简介：一句话描述插件核心功能          │
│  │   封面图     │  开发者：XXX官方  |  版本：2.1.0           │
│  │              │  分类：营销工具 > 优惠券                   │
│  └──────────────┘  标签：#官方 #热门 #稳定                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ [基本信息] [套餐价格] [使用教程] [更新日志] [评论(128)]     ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│  套餐价格                                                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐              │
│  │  基础版     │ │  专业版      │ │  企业版     │              │
│  │             │ │  ⭐推荐      │ │             │              │
│  │  ¥99/月     │ │  ¥299/月    │ │  ¥799/月    │              │
│  │  ¥990/年    │ │  ¥2990/年   │ │  ¥7990/年   │              │
│  │  ¥1999/永久 │ │  ¥5999/永久 │ │  ¥15999/永  │              │
│  │             │ │             │ │             │              │
│  │  □单设备    │ │  □5设备     │ │  □无限设备  │              │
│  │  □基础功能  │ │  □高级功能  │ │  □全功能   │              │
│  │  □邮件支持  │ │  □在线支持  │ │  □专属客服  │              │
│  │             │ │             │ │             │              │
│  │ [加入购物车]│ │ [加入购物车]│ │ [加入购物车]│              │
│  │ [立即购买] │ │ [立即购买]  │ │ [立即购买] │              │
│  └─────────────┘ └─────────────┘ └─────────────┘              │
│                                                                 │
│  ┌─────────────────────────────┐                               │
│  │  限时优惠：全场8折  优惠码:  │ ← 显示复制                   │
│  └─────────────────────────────┘                               │
├─────────────────────────────────────────────────────────────────┤
│                                    [加入购物车]  [立即购买]     │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.2.3 插件管理页面

```
┌─────────────────────────────────────────────────────────────────┐
│  我的插件                                    [筛选▼] [排序▼]    │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ □ 全选                                                         ││
│  └─────────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ □ [封面] 插件名称                    到期时间      状态      ││
│  │   开发者：XXX        套餐：专业版   2024-12-31    正常      ││
│  │   版本：2.1.0        设备：3/5      [续费] [升级] [详情]    ││
│  └─────────────────────────────────────────────────────────────┘│
│  ...                                                            │
├─────────────────────────────────────────────────────────────────┤
│  共 5 个插件                        ◀ 1 2 3 4 5 ▶              │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. 防破解安全机制

### 6.1 多级验证体系

#### Level 1: 基础验证（设备绑定）

```rust
// 设备注册流程
1. 客户端首次运行生成设备指纹（Fingerprint）
2. 发送设备信息到服务器进行注册
3. 服务器返回 License Key
4. 客户端本地存储 License Key
5. 每次启动验证设备指纹 + License Key
```

#### Level 2: 验证码验证（发码机制）

```rust
// 验证码激活流程
1. 商户在平台购买插件 → 生成订单
2. 商户填写激活码申请 → 系统发送6位验证码
3. 商户在插件端输入验证码
4. 服务器验证：
   - 验证码有效性
   - 设备指纹哈希匹配
   - 时间窗口（5分钟内有效）
   - 尝试次数限制（最多3次）
5. 验证通过 → 绑定 License 与设备
```

#### Level 3: 代码混淆 + 反调试

```rust
// 高级防护
1. 核心代码段虚拟化
2. 动态花指令插入
3. 控制流扁平化
4. 反调试检测：
   - 检测 Debugger
   - 检测 Hook 框架
   - 检测模拟器
   - 检测 ROOT/越狱
5. 加密通信（TLS证书锁定）
```

### 6.2 防破解关键措施

| 措施 | 说明 |
|------|------|
| **设备指纹** | 基于硬件信息生成唯一设备ID，防止多设备共享 |
| **License Key** | 使用UUID格式，服务器可控，随时可吊销 |
| **验证码机制** | 6位数字，一次性使用，5分钟有效期 |
| **时间戳校验** | 防止本地时间篡改 |
| **签名校验** | 下载文件使用RSA签名，防篡改 |
| **心跳检测** | 定期与服务器通信，检测异常 |
| **离线限制** | 最大离线天数控制，防止破解后完全离线使用 |
| **代码混淆** | 核心逻辑使用虚拟化保护 |

### 6.3 卡密系统设计

```rust
struct Card密 {
    id: i64,
    card_no: String,             // 卡号
    card_pwd: String,            // 卡密（加密存储）
    plugin_id: i64,
    plan_id: i64,
    batch_id: i64,               // 批次号
    face_value: Decimal,         // 面值
    status: CardStatus,
    used_user_id: Option<i64>,
    used_time: Option<DateTime>,
    expire_time: DateTime,       // 卡密过期时间
    created_at: DateTime,
}

enum CardStatus {
    Unused,   // 未使用
    Used,     // 已使用
    Expired,  // 已过期
    Frozen,   // 已冻结
}
```

---

## 7. 数据库设计

### 7.1 核心表结构

```sql
-- 插件表
CREATE TABLE `p_plugin` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `code` varchar(64) NOT NULL COMMENT '插件编码',
  `name` varchar(100) NOT NULL COMMENT '插件名称',
  `category_id` bigint(20) NOT NULL COMMENT '分类ID',
  `developer_id` bigint(20) NOT NULL COMMENT '开发者ID',
  `summary` varchar(255) DEFAULT NULL COMMENT '简介',
  `description` text COMMENT '详细描述',
  `cover_image` varchar(500) DEFAULT NULL COMMENT '封面图',
  `screenshots` json DEFAULT NULL COMMENT '截图列表',
  `version` varchar(32) NOT NULL DEFAULT '1.0.0' COMMENT '当前版本',
  `price_type` tinyint(1) NOT NULL DEFAULT '0' COMMENT '价格类型：0免费1一次性2订阅',
  `verify_level` tinyint(1) NOT NULL DEFAULT '0' COMMENT '验证级别：0无1基础2高级',
  `download_count` int(11) NOT NULL DEFAULT '0' COMMENT '下载次数',
  `rating` decimal(3,2) NOT NULL DEFAULT '5.00' COMMENT '评分',
  `status` tinyint(1) NOT NULL DEFAULT '0' COMMENT '状态：0待审核1上架2下架3回收站',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `tags` json DEFAULT NULL COMMENT '标签',
  `is_official` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否官方',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_code` (`code`),
  KEY `idx_category` (`category_id`),
  KEY `idx_developer` (`developer_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件表';

-- 插件版本表
CREATE TABLE `p_plugin_version` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `version` varchar(32) NOT NULL COMMENT '版本号',
  `changelog` text COMMENT '变更日志',
  `download_url` varchar(500) NOT NULL COMMENT '下载链接',
  `file_hash` varchar(64) NOT NULL COMMENT '文件哈希',
  `file_size` bigint(20) NOT NULL COMMENT '文件大小',
  `min_app_version` varchar(32) DEFAULT NULL COMMENT '最低兼容版本',
  `is_latest` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否最新',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_latest` (`plugin_id`, `is_latest`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件版本表';

-- 套餐表
CREATE TABLE `p_plan` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `name` varchar(50) NOT NULL COMMENT '套餐名称',
  `description` varchar(255) DEFAULT NULL COMMENT '套餐描述',
  `period_type` tinyint(1) NOT NULL COMMENT '周期类型：0月1季2年3永久',
  `period_days` int(11) NOT NULL COMMENT '周期天数',
  `price` decimal(10,2) NOT NULL COMMENT '价格',
  `original_price` decimal(10,2) NOT NULL COMMENT '原价',
  `features` json DEFAULT NULL COMMENT '功能列表',
  `max_devices` int(11) NOT NULL DEFAULT '1' COMMENT '最大设备数',
  `max_users` int(11) NOT NULL DEFAULT '0' COMMENT '最大用户数',
  `storage_limit` bigint(20) NOT NULL DEFAULT '0' COMMENT '存储限制(MB)',
  `api_calls_limit` bigint(20) NOT NULL DEFAULT '0' COMMENT 'API限制(次/天)',
  `support_level` tinyint(1) NOT NULL DEFAULT '0' COMMENT '支持级别：0基础1高级2专属',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_plugin` (`plugin_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='套餐表';

-- 订单表
CREATE TABLE `p_order` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `order_no` varchar(32) NOT NULL COMMENT '订单号',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `amount` decimal(10,2) NOT NULL COMMENT '支付金额',
  `original_amount` decimal(10,2) NOT NULL COMMENT '原价金额',
  `discount_amount` decimal(10,2) NOT NULL DEFAULT '0.00' COMMENT '优惠金额',
  `coupon_id` bigint(20) DEFAULT NULL COMMENT '优惠券ID',
  `payment_method` tinyint(1) NOT NULL COMMENT '支付方式：0微信1支付宝2卡密3余额',
  `payment_time` datetime DEFAULT NULL COMMENT '支付时间',
  `status` tinyint(1) NOT NULL DEFAULT '0' COMMENT '状态：0待支付1已支付2已取消3已退款4已过期',
  `expire_time` datetime NOT NULL COMMENT '订单过期时间',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_order_no` (`order_no`),
  KEY `idx_user` (`user_id`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_status` (`status`),
  KEY `idx_expire` (`expire_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单表';

-- 订阅表
CREATE TABLE `p_subscription` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `order_id` bigint(20) DEFAULT NULL COMMENT '订单ID',
  `license_id` bigint(20) DEFAULT NULL COMMENT 'License ID',
  `start_time` datetime NOT NULL COMMENT '开始时间',
  `end_time` datetime NOT NULL COMMENT '到期时间',
  `auto_renew` tinyint(1) NOT NULL DEFAULT '0' COMMENT '自动续费：0关闭1开启',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0待生效1生效中2已过期3已取消',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_user` (`user_id`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_end_time` (`end_time`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订阅表';

-- License表
CREATE TABLE `p_license` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `license_key` varchar(64) NOT NULL COMMENT 'License密钥',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `order_id` bigint(20) DEFAULT NULL COMMENT '订单ID',
  `device_id` varchar(128) NOT NULL COMMENT '设备ID',
  `device_name` varchar(100) DEFAULT NULL COMMENT '设备名称',
  `device_type` varchar(50) DEFAULT NULL COMMENT '设备类型',
  `os_version` varchar(50) DEFAULT NULL COMMENT '系统版本',
  `app_version` varchar(32) DEFAULT NULL COMMENT '应用版本',
  `ip_address` varchar(50) DEFAULT NULL COMMENT '注册IP',
  `bind_count` int(11) NOT NULL DEFAULT '0' COMMENT '绑定次数',
  `max_devices` int(11) NOT NULL DEFAULT '1' COMMENT '最大设备数',
  `verify_count` int(11) NOT NULL DEFAULT '0' COMMENT '验证次数',
  `last_verify_time` datetime DEFAULT NULL COMMENT '最后验证时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用2过期3注销',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_license_key` (`license_key`),
  KEY `idx_user` (`user_id`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_device` (`device_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='License表';

-- 设备表
CREATE TABLE `p_device` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `license_id` bigint(20) NOT NULL COMMENT 'License ID',
  `device_id` varchar(128) NOT NULL COMMENT '设备ID',
  `device_name` varchar(100) DEFAULT NULL COMMENT '设备名称',
  `device_type` varchar(50) DEFAULT NULL COMMENT '设备类型',
  `os_version` varchar(50) DEFAULT NULL COMMENT '系统版本',
  `mac_address` varchar(64) DEFAULT NULL COMMENT 'MAC地址',
  `ip_address` varchar(50) DEFAULT NULL COMMENT 'IP地址',
  `last_active_time` datetime DEFAULT NULL COMMENT '最后活跃时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0离线1在线',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_device_license` (`license_id`, `device_id`),
  KEY `idx_user` (`user_id`),
  KEY `idx_device` (`device_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备表';

-- 验证码表
CREATE TABLE `p_verification_code` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `code` varchar(10) NOT NULL COMMENT '验证码',
  `license_id` bigint(20) NOT NULL COMMENT 'License ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `purpose` tinyint(1) NOT NULL COMMENT '用途：0激活1登录2心跳3更新',
  `device_hash` varchar(64) DEFAULT NULL COMMENT '设备哈希',
  `status` tinyint(1) NOT NULL DEFAULT '0' COMMENT '状态：0未用1已用2过期3封禁',
  `attempts` int(11) NOT NULL DEFAULT '0' COMMENT '尝试次数',
  `expire_time` datetime NOT NULL COMMENT '过期时间',
  `verified_at` datetime DEFAULT NULL COMMENT '验证时间',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_code` (`code`),
  KEY `idx_license` (`license_id`),
  KEY `idx_expire` (`expire_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='验证码表';

-- 卡密表
CREATE TABLE `p_card` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `card_no` varchar(32) NOT NULL COMMENT '卡号',
  `card_pwd` varchar(64) NOT NULL COMMENT '卡密',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `batch_id` bigint(20) NOT NULL COMMENT '批次ID',
  `face_value` decimal(10,2) NOT NULL COMMENT '面值',
  `status` tinyint(1) NOT NULL DEFAULT '0' COMMENT '状态：0未使用1已使用2已过期3已冻结',
  `used_user_id` bigint(20) DEFAULT NULL COMMENT '使用用户ID',
  `used_order_id` bigint(20) DEFAULT NULL COMMENT '使用订单ID',
  `used_time` datetime DEFAULT NULL COMMENT '使用时间',
  `expire_time` datetime NOT NULL COMMENT '卡密过期时间',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_card_no` (`card_no`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_batch` (`batch_id`),
  KEY `idx_status` (`status`),
  KEY `idx_expire` (`expire_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='卡密表';

-- 卡密批次表
CREATE TABLE `p_card_batch` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `batch_no` varchar(32) NOT NULL COMMENT '批次号',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `total_count` int(11) NOT NULL COMMENT '生成数量',
  `used_count` int(11) NOT NULL DEFAULT '0' COMMENT '已使用数量',
  `price` decimal(10,2) NOT NULL COMMENT '卡密售价',
  `expire_time` datetime NOT NULL COMMENT '卡密过期时间',
  `creator_id` bigint(20) NOT NULL COMMENT '创建人',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_batch_no` (`batch_no`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='卡密批次表';

-- 购物车表
CREATE TABLE `p_cart` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_user_plugin_plan` (`user_id`, `plugin_id`, `plan_id`),
  KEY `idx_user` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购物车表';

-- 插件分类表
CREATE TABLE `p_plugin_category` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `name` varchar(50) NOT NULL COMMENT '分类名称',
  `icon` varchar(100) DEFAULT NULL COMMENT '分类图标',
  `parent_id` bigint(20) DEFAULT NULL COMMENT '父分类ID',
  `sort` int(11) NOT NULL DEFAULT '0' COMMENT '排序',
  `plugin_count` int(11) NOT NULL DEFAULT '0' COMMENT '插件数量',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件分类表';

-- 开发者表
CREATE TABLE `p_developer` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `name` varchar(100) NOT NULL COMMENT '开发者名称',
  `logo` varchar(500) DEFAULT NULL COMMENT '开发者Logo',
  `description` text COMMENT '开发者描述',
  `contact` varchar(100) DEFAULT NULL COMMENT '联系方式',
  `plugins_count` int(11) NOT NULL DEFAULT '0' COMMENT '插件数量',
  `total_downloads` bigint(20) NOT NULL DEFAULT '0' COMMENT '总下载量',
  `rating` decimal(3,2) NOT NULL DEFAULT '5.00' COMMENT '平均评分',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0禁用1启用',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_user` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='开发者表';

-- 插件评论表
CREATE TABLE `p_plugin_review` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `plugin_id` bigint(20) NOT NULL COMMENT '插件ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `rating` tinyint(1) NOT NULL COMMENT '评分1-5',
  `content` text COMMENT '评论内容',
  `reply_content` text COMMENT '回复内容',
  `reply_time` datetime DEFAULT NULL COMMENT '回复时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '状态：0隐藏1显示',
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_plugin` (`plugin_id`),
  KEY `idx_user` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件评论表';
```

---

## 8. 技术实现要点

### 8.1 License 验证算法

```rust
// 验证请求签名生成
fn generate_verify_sign(license_key: &str, device_id: &str, timestamp: i64, nonce: &str) -> String {
    let data = format!("{}:{}:{}:{}", license_key, device_id, timestamp, nonce);
    let key = calculate_md5(license_key);
    hmac_sha256(&key, data.as_bytes())
}

// 验证码生成
fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(100000..999999);
    code.to_string()
}

// 设备指纹生成（客户端）
fn generate_device_fingerprint() -> String {
    // 组合设备信息生成哈希
    let device_info = format!(
        "{}-{}-{}-{}",
        get_hardware_id(),      // 硬件ID
        get_mac_address(),       // MAC地址
        get_disk_serial(),      // 磁盘序列号
        get_cpu_id()            // CPU序列号
    );
    calculate_sha256(device_info)
}
```

### 8.2 订单号生成规则

```
格式：PLM + 时间戳(14位) + 随机数(6位)
示例：PLM20260324143052001234

含义：
- PLM：Plugin Market 缩写
- 20260324143052：2026年03月24日 14:30:52
- 001234：6位随机数
```

### 8.3 卡密生成规则

```
格式：XXXX-XXXX-XXXX-XXXX（16位字母数字）
示例：A3F7-K9M2-N5P8-L4E1

规则：
- 使用大写字母 + 数字（去除易混淆字符：0,O,1,I,L）
- 每4位用-分隔
- 存储时使用AES加密
- 查询时按 card_no 索引
```

---

## 9. Mock 接口设计

### 9.1 插件市场 Mock 数据

```typescript
// 插件市场列表
GET /mock/plugin/market/list
Response: {
  code: 200,
  data: {
    list: [
      {
        id: 1,
        code: "plugin_coupon",
        name: "智能优惠券",
        summary: "多场景优惠券发放与核销",
        coverImage: "https://...",
        priceType: 2,
        price: 99,
        rating: 4.8,
        downloadCount: 2560,
        categoryName: "营销工具",
        tags: ["官方", "热门"],
        isFree: false
      }
    ],
    total: 100
  }
}
```

### 9.2 订单 Mock

```typescript
// 创建订单
POST /mock/order/create
Request: {
  pluginId: 1,
  planId: 2,
  paymentMethod: "wechat"
}
Response: {
  code: 200,
  data: {
    orderNo: "PLM20260324143052001234",
    amount: 299,
    qrCode: "weixin://..."
  }
}
```

### 9.3 License Mock

```typescript
// 验证License
POST /mock/license/verify
Request: {
  licenseKey: "xxxx-xxxx-xxxx-xxxx",
  deviceId: "device_fingerprint_hash",
  deviceInfo: {...},
  timestamp: 1711274400,
  sign: "hmac_signature"
}
Response: {
  code: 200,
  data: {
    valid: true,
    expireTime: "2027-03-24",
    maxDevices: 5,
    features: ["coupon_basic", "coupon_advanced"]
  }
}
```

---

## 10. 安全考虑

### 10.1 数据安全

- License Key 存储使用加盐哈希
- 卡密数据库加密存储
- 设备指纹不可逆哈希
- 通信使用 HTTPS + 证书锁定

### 10.2 防欺诈

- 同一 License 同时验证次数限制
- 设备绑定数量限制
- 验证码尝试次数限制（3次）
- 异地登录检测
- 行为分析（频繁验证/大量绑定）

### 10.3 防破解

- 客户端代码混淆（JavaScript 混淆）
- 核心算法迁移到服务端
- 定期更新验证逻辑
- 反调试检测（Debugger, Hook, Emulator）

---

## 11. 影响范围

### 11.1 新增模块

- **后端**：plugin、order、license、verify、card 五个新模块
- **前端**：plugin-market、plugin-my、plugin-developer、order、verify 五个新页面
- **数据库**：10+ 张新表
- **API**：30+ 个新接口

### 11.2 系统改造

- 菜单权限体系改造
- 用户角色新增「开发者」角色
- 支付模块集成
- 通知模块集成（邮件/短信）

---

## 12. 项目规划

整体项目分为三个阶段：

**第一阶段（基础功能）**：插件市场展示、购买、订阅
**第二阶段（授权管理）**：License、设备绑定、验证
**第三阶段（高级功能）**：卡密系统、开发者中心、数据分析