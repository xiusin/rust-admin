# 插件市场/应用市场系统 - 任务清单

## 概述

本任务清单详细规划了插件市场/应用市场系统的开发工作，按照模块化方式组织，任务之间标注了依赖关系。

---

## 阶段一：基础设施与数据模型

### 1.1 数据库层

- [ ] **Task 1.1.1**: 创建插件市场数据库迁移文件
  - 创建 `p_plugin` 插件表
  - 创建 `p_plugin_version` 插件版本表
  - 创建 `p_plugin_category` 插件分类表
  - 创建 `p_plan` 套餐表
  - 创建索引和约束

- [ ] **Task 1.1.2**: 创建交易相关数据库迁移文件
  - 创建 `p_order` 订单表
  - 创建 `p_cart` 购物车表
  - 创建 `p_subscription` 订阅表
  - 创建索引和约束

- [ ] **Task 1.1.3**: 创建License与设备数据库迁移文件
  - 创建 `p_license` License表
  - 创建 `p_device` 设备表
  - 创建 `p_verification_code` 验证码表
  - 创建索引和约束

- [ ] **Task 1.1.4**: 创建卡密系统数据库迁移文件
  - 创建 `p_card` 卡密表
  - 创建 `p_card_batch` 卡密批次表
  - 创建索引和约束

- [ ] **Task 1.1.5**: 创建开发者与评论数据库迁移文件
  - 创建 `p_developer` 开发者表
  - 创建 `p_plugin_review` 插件评论表
  - 创建索引和约束

### 1.2 Domain 实体层

- [ ] **Task 1.2.1**: 创建插件领域实体 (Entity)
  - `p_plugin.rs` - 插件实体
  - `p_plugin_version.rs` - 插件版本实体
  - `p_plugin_category.rs` - 插件分类实体
  - 实现关联关系

- [ ] **Task 1.2.2**: 创建交易领域实体
  - `p_order.rs` - 订单实体
  - `p_cart.rs` - 购物车实体
  - `p_subscription.rs` - 订阅实体

- [ ] **Task 1.2.3**: 创建License领域实体
  - `p_license.rs` - License实体
  - `p_device.rs` - 设备实体
  - `p_verification_code.rs` - 验证码实体

- [ ] **Task 1.2.4**: 创建卡密领域实体
  - `p_card.rs` - 卡密实体
  - `p_card_batch.rs` - 卡密批次实体

- [ ] **Task 1.2.5**: 创建开发者与评论领域实体
  - `p_developer.rs` - 开发者实体
  - `p_plugin_review.rs` - 插件评论实体

### 1.3 Domain 模型层 (Model)

- [ ] **Task 1.3.1**: 创建插件领域模型
  - `m_plugin.rs` - 插件列表项、详情模型
  - `m_plugin_version.rs` - 版本模型
  - `m_plugin_category.rs` - 分类模型

- [ ] **Task 1.3.2**: 创建交易领域模型
  - `m_order.rs` - 订单模型
  - `m_cart.rs` - 购物车模型
  - `m_subscription.rs` - 订阅模型

- [ ] **Task 1.3.3**: 创建License领域模型
  - `m_license.rs` - License模型
  - `m_device.rs` - 设备模型
  - `m_verification.rs` - 验证码模型

- [ ] **Task 1.3.4**: 创建卡密领域模型
  - `m_card.rs` - 卡密模型
  - `m_card_batch.rs` - 批次模型

### 1.4 Domain 参数层 (Args)

- [ ] **Task 1.4.1**: 创建插件参数结构
  - `a_plugin.rs` - 插件增删改查参数

- [ ] **Task 1.4.2**: 创建订单参数结构
  - `a_order.rs` - 订单参数

- [ ] **Task 1.4.3**: 创建License参数结构
  - `a_license.rs` - License参数

- [ ] **Task 1.4.4**: 创建卡密参数结构
  - `a_card.rs` - 卡密参数

---

## 阶段二：后端业务服务层 (Application)

### 2.1 插件服务

- [ ] **Task 2.1.1**: 创建插件分类服务
  - `plugin_category_service.rs`
  - 分类列表、树形结构

- [ ] **Task 2.1.2**: 创建插件基础服务
  - `plugin_service.rs`
  - 插件列表、详情、搜索
  - 插件上架、下架
  - 插件审核

- [ ] **Task 2.1.3**: 创建插件版本服务
  - `plugin_version_service.rs`
  - 版本发布、版本列表
  - 版本更新

- [ ] **Task 2.1.4**: 创建插件评价服务
  - `plugin_review_service.rs`
  - 评论列表、发表评论
  - 开发者回复

### 2.2 套餐与订阅服务

- [ ] **Task 2.2.1**: 创建套餐服务
  - `plan_service.rs`
  - 套餐CRUD
  - 套餐价格计算

- [ ] **Task 2.2.2**: 创建订阅服务
  - `subscription_service.rs`
  - 订阅创建、续费、升级
  - 订阅状态管理
  - 到期提醒触发

### 2.3 订单与购物车服务

- [ ] **Task 2.3.1**: 创建购物车服务
  - `cart_service.rs`
  - 添加购物车
  - 移除购物车
  - 清空购物车

- [ ] **Task 2.3.2**: 创建订单服务
  - `order_service.rs`
  - 创建订单（支持购物车/直接购买）
  - 订单支付流程
  - 订单取消/关闭
  - 订单列表/详情

- [ ] **Task 2.3.3**: 创建支付集成服务
  - 支付流程整合
  - 支付回调处理
  - 微信/支付宝集成

### 2.4 License与验证服务

- [ ] **Task 2.4.1**: 创建License服务
  - `license_service.rs`
  - License生成
  - License绑定设备
  - License解绑设备
  - License续费
  - License吊销

- [ ] **Task 2.4.2**: 创建设备服务
  - `device_service.rs`
  - 设备注册
  - 设备列表
  - 设备状态管理

- [ ] **Task 2.4.3**: 创建验证码服务
  - `verify_code_service.rs`
  - 验证码生成
  - 验证码发送（Mock）
  - 验证码校验
  - 防暴力破解

- [ ] **Task 2.4.4**: 创建验证核心服务
  - `verification_service.rs`
  - License验证
  - 心跳检测
  - 签名校验

### 2.5 卡密服务

- [ ] **Task 2.5.1**: 创建卡密生成服务
  - `card_generate_service.rs`
  - 批量生成卡密
  - 卡密加密存储

- [ ] **Task 2.5.2**: 创建卡密兑换服务
  - `card_redeem_service.rs`
  - 卡密校验
  - 卡密兑换流程

---

## 阶段三：API 路由层

### 3.1 插件市场 API

- [ ] **Task 3.1.1**: 创建插件市场 API 路由
  - `/api/plugin/market/` 路由组
  - 市场列表接口
  - 市场详情接口
  - 市场分类接口

- [ ] **Task 3.1.2**: 创建插件管理 API 路由
  - `/api/plugin/developer/` 路由组
  - 开发者插件CRUD
  - 版本发布接口
  - 销售统计接口

### 3.2 交易订单 API

- [ ] **Task 3.2.1**: 创建购物车 API
  - `/api/order/cart/` 路由组
  - 购物车增删改查

- [ ] **Task 3.2.2**: 创建订单 API
  - `/api/order/` 路由组
  - 订单创建、列表、详情
  - 支付接口

### 3.3 License API

- [ ] **Task 3.3.1**: 创建 License API
  - `/api/license/` 路由组
  - License列表、详情
  - 设备绑定/解绑
  - 续费接口

- [ ] **Task 3.3.2**: 创建验证 API
  - `/api/verify/` 路由组
  - 验证码发送/校验
  - License验证
  - 心跳检测

### 3.4 卡密 API

- [ ] **Task 3.4.1**: 创建卡密管理 API (后台)
  - `/api/card/admin/` 路由组
  - 卡密生成、导出

- [ ] **Task 3.4.2**: 创建卡密兑换 API
  - `/api/card/redeem/` 路由组
  - 卡密兑换接口

---

## 阶段四：前端 - API 层

### 4.1 插件市场 API

- [ ] **Task 4.1.1**: 创建插件市场 API 模块
  - `frontend/src/api/modules/plugin-market/`
  - `index.ts` - 导出
  - `market.ts` - 市场接口

- [ ] **Task 4.1.2**: 创建插件管理 API 模块
  - `frontend/src/api/modules/plugin-developer/`
  - `index.ts`
  - `plugin.ts` - 插件管理接口
  - `version.ts` - 版本管理接口

### 4.2 订单 API

- [ ] **Task 4.2.1**: 创建订单 API 模块
  - `frontend/src/api/modules/order/`
  - `index.ts`
  - `cart.ts` - 购物车接口
  - `order.ts` - 订单接口

### 4.3 License API

- [ ] **Task 4.3.1**: 创建 License API 模块
  - `frontend/src/api/modules/license/`
  - `index.ts`
  - `license.ts` - License接口
  - `device.ts` - 设备接口
  - `verify.ts` - 验证接口

### 4.4 卡密 API

- [ ] **Task 4.4.1**: 创建卡密 API 模块
  - `frontend/src/api/modules/card/`
  - `index.ts`
  - `card.ts` - 卡密接口

---

## 阶段五：前端 - Mock 数据层

### 5.1 Mock 数据文件

- [ ] **Task 5.1.1**: 创建插件市场 Mock 数据
  - `frontend/src/mock/plugin/market.ts`
  - 插件列表Mock
  - 插件详情Mock
  - 分类Mock

- [ ] **Task 5.1.2**: 创建订单 Mock 数据
  - `frontend/src/mock/order/`
  - 购物车Mock
  - 订单Mock
  - 支付Mock

- [ ] **Task 5.1.3**: 创建 License Mock 数据
  - `frontend/src/mock/license/`
  - License列表Mock
  - 验证结果Mock
  - 设备列表Mock

- [ ] **Task 5.1.4**: 创建卡密 Mock 数据
  - `frontend/src/mock/card/`
  - 卡密生成Mock
  - 卡密兑换Mock

### 5.2 Mock 接口注册

- [ ] **Task 5.2.1**: 注册插件市场 Mock 路由
  - 在 `frontend/src/mock/index.ts` 中注册

- [ ] **Task 5.2.2**: 注册订单 Mock 路由
  - 在 `frontend/src/mock/index.ts` 中注册

- [ ] **Task 5.2.3**: 注册 License Mock 路由
  - 在 `frontend/src/mock/index.ts` 中注册

- [ ] **Task 5.2.4**: 注册卡密 Mock 路由
  - 在 `frontend/src/mock/index.ts` 中注册

---

## 阶段六：前端 - 页面开发

### 6.1 插件市场页面

- [ ] **Task 6.1.1**: 创建插件市场首页
  - `frontend/src/views/plugin-market/index.vue`
  - 分类展示
  - 推荐插件
  - 热门插件

- [ ] **Task 6.1.2**: 创建插件列表页
  - `frontend/src/views/plugin-market/list.vue`
  - 分类筛选
  - 价格筛选
  - 排序功能
  - 分页

- [ ] **Task 6.1.3**: 创建插件详情页
  - `frontend/src/views/plugin-market/detail.vue`
  - 基本信息Tab
  - 套餐选择Tab
  - 使用教程Tab
  - 更新日志Tab
  - 评论Tab
  - 立即购买/加入购物车

- [ ] **Task 6.1.4**: 创建插件搜索页
  - `frontend/src/views/plugin-market/search.vue`
  - 搜索结果展示

### 6.2 我的插件页面

- [ ] **Task 6.2.1**: 创建已购插件页面
  - `frontend/src/views/plugin-my/purchased.vue`
  - 已购插件列表
  - 续费入口
  - 升级入口

- [ ] **Task 6.2.2**: 创建订阅管理页面
  - `frontend/src/views/plugin-my/subscription.vue`
  - 订阅列表
  - 自动续费设置
  - 到期提醒设置

- [ ] **Task 6.2.3**: 创建License管理页面
  - `frontend/src/views/plugin-my/license.vue`
  - License列表
  - 设备绑定管理
  - 绑定/解绑设备

### 6.3 开发者中心页面

- [ ] **Task 6.3.1**: 创建开发者仪表盘
  - `frontend/src/views/plugin-developer/dashboard.vue`
  - 销售概览
  - 下载统计
  - 收入统计

- [ ] **Task 6.3.2**: 创建插件管理列表
  - `frontend/src/views/plugin-developer/plugin-list.vue`
  - 插件CRUD
  - 状态管理

- [ ] **Task 6.3.3**: 创建插件编辑页面
  - `frontend/src/views/plugin-developer/plugin-edit.vue`
  - 插件信息编辑
  - 版本发布

- [ ] **Task 6.3.4**: 创建版本管理页面
  - `frontend/src/views/plugin-developer/version.vue`
  - 版本列表
  - 版本发布

- [ ] **Task 6.3.5**: 创建销售统计页面
  - `frontend/src/views/plugin-developer/sales.vue`
  - 订单统计
  - 收入报表
  - 导出功能

### 6.4 订单中心页面

- [ ] **Task 6.4.1**: 创建购物车页面
  - `frontend/src/views/order/cart.vue`
  - 购物车列表
  - 选择套餐
  - 删除/结算

- [ ] **Task 6.4.2**: 创建结账页面
  - `frontend/src/views/order/checkout.vue`
  - 订单确认
  - 优惠码输入
  - 支付方式选择

- [ ] **Task 6.4.3**: 创建订单列表页面
  - `frontend/src/views/order/list.vue`
  - 订单状态筛选
  - 订单列表

- [ ] **Task 6.4.4**: 创建订单详情页面
  - `frontend/src/views/order/detail.vue`
  - 订单信息
  - 支付信息
  - License信息

### 6.5 验证中心页面

- [ ] **Task 6.5.1**: 创建插件激活页面
  - `frontend/src/views/verify/activate.vue`
  - 验证码输入
  - 设备信息展示

- [ ] **Task 6.5.2**: 创建设备管理页面
  - `frontend/src/views/verify/device.vue`
  - 设备列表
  - 设备状态
  - 解绑设备

- [ ] **Task 6.5.3**: 创建卡密兑换页面
  - `frontend/src/views/verify/card-redeem.vue`
  - 卡密输入
  - 兑换结果

---

## 阶段七：国际化与菜单

### 7.1 国际化

- [ ] **Task 7.1.1**: 更新中文语言包
  - 添加插件市场相关中文
  - 添加订单相关中文
  - 添加License相关中文

- [ ] **Task 7.1.2**: 更新英文语言包
  - 添加插件市场相关英文
  - 添加订单相关英文
  - 添加License相关英文

### 7.2 菜单与路由

- [ ] **Task 7.2.1**: 添加插件市场菜单
  - 菜单数据库迁移文件
  - 菜单图标配置

- [ ] **Task 7.2.2**: 配置前端路由
  - 添加插件市场路由
  - 添加订单路由
  - 添加License路由
  - 添加验证路由

---

## 阶段八：权限与安全

### 8.1 权限控制

- [ ] **Task 8.1.1**: 创建插件市场权限
  - API权限定义
  - 按钮权限控制

- [ ] **Task 8.1.2**: 创建开发者权限
  - 开发者角色权限
  - 插件发布权限

### 8.2 安全机制

- [ ] **Task 8.2.1**: 实现签名验证
  - 验证请求签名算法
  - 签名校验中间件

- [ ] **Task 8.2.2**: 实现限流防刷
  - 验证码接口限流
  - 验证接口限流

---

## 任务依赖关系

```
阶段一（基础设施）
    ↓
阶段二（业务服务）依赖 阶段一
    ↓
阶段三（API路由）依赖 阶段二
    ↓
阶段四（前端API）依赖 阶段三（接口定义）
阶段五（Mock数据）可并行
    ↓
阶段六（前端页面）依赖 阶段四、阶段五
    ↓
阶段七（国际化菜单）
阶段八（权限安全）
```

---

## 验收标准

1. 数据库迁移文件可正常执行
2. 所有API接口返回符合定义的响应格式
3. 前端页面可正常访问，数据正常展示
4. Mock接口可切换
5. 权限控制正常生效
6. 代码风格符合项目规范