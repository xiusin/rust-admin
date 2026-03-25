# 插件市场/应用市场系统 - 检查清单

## 概述

本检查清单用于验证插件市场/应用市场系统的开发完成情况。每个检查项均对应具体的功能点和技术要求。

---

## 阶段一：数据库与实体层

### 1.1 数据库迁移文件

- [ ] **p_plugin 表** - 插件表包含所有必要字段
  - [ ] id, code, name, category_id, developer_id
  - [ ] summary, description, cover_image, screenshots
  - [ ] version, price_type, verify_level
  - [ ] download_count, rating, status, is_official
  - [ ] created_at, updated_at, deleted_at
  - [ ] 索引：uk_code, idx_category, idx_developer, idx_status

- [ ] **p_plugin_version 表** - 插件版本表
  - [ ] id, plugin_id, version, changelog
  - [ ] download_url, file_hash, file_size
  - [ ] min_app_version, is_latest, status
  - [ ] 索引：idx_plugin, idx_latest

- [ ] **p_plugin_category 表** - 插件分类表
  - [ ] id, name, icon, parent_id
  - [ ] sort, plugin_count, status

- [ ] **p_plan 表** - 套餐表
  - [ ] id, plugin_id, name, description
  - [ ] period_type, period_days, price, original_price
  - [ ] features (JSON), max_devices, max_users
  - [ ] storage_limit, api_calls_limit, support_level
  - [ ] sort, status
  - [ ] 索引：idx_plugin

- [ ] **p_order 表** - 订单表
  - [ ] id, order_no, user_id, plugin_id, plan_id
  - [ ] amount, original_amount, discount_amount, coupon_id
  - [ ] payment_method, payment_time
  - [ ] status, expire_time
  - [ ] 索引：uk_order_no, idx_user, idx_plugin, idx_status, idx_expire

- [ ] **p_cart 表** - 购物车表
  - [ ] id, user_id, plugin_id, plan_id
  - [ ] 唯一索引：uk_user_plugin_plan

- [ ] **p_subscription 表** - 订阅表
  - [ ] id, user_id, plugin_id, plan_id, order_id, license_id
  - [ ] start_time, end_time, auto_renew, status
  - [ ] 索引：idx_user, idx_plugin, idx_end_time, idx_status

- [ ] **p_license 表** - License表
  - [ ] id, license_key, user_id, plugin_id, plan_id, order_id
  - [ ] device_id, device_name, device_type, os_version, app_version
  - [ ] ip_address, bind_count, max_devices
  - [ ] verify_count, last_verify_time
  - [ ] status, start_time, end_time
  - [ ] 索引：uk_license_key, idx_user, idx_plugin, idx_device, idx_status

- [ ] **p_device 表** - 设备表
  - [ ] id, user_id, license_id, device_id
  - [ ] device_name, device_type, os_version, mac_address, ip_address
  - [ ] last_active_time, status
  - [ ] 唯一索引：uk_device_license

- [ ] **p_verification_code 表** - 验证码表
  - [ ] id, code, license_id, user_id, plugin_id
  - [ ] purpose, device_hash, status, attempts
  - [ ] expire_time, verified_at
  - [ ] 索引：idx_code, idx_license, idx_expire

- [ ] **p_card 表** - 卡密表
  - [ ] id, card_no, card_pwd, plugin_id, plan_id, batch_id
  - [ ] face_value, status
  - [ ] used_user_id, used_order_id, used_time
  - [ ] expire_time
  - [ ] 索引：uk_card_no, idx_plugin, idx_batch, idx_status, idx_expire

- [ ] **p_card_batch 表** - 卡密批次表
  - [ ] id, batch_no, plugin_id, plan_id
  - [ ] total_count, used_count, price
  - [ ] expire_time, creator_id, status
  - [ ] 唯一索引：uk_batch_no

- [ ] **p_developer 表** - 开发者表
  - [ ] id, user_id, name, logo, description
  - [ ] contact, plugins_count, total_downloads, rating, status
  - [ ] 唯一索引：uk_user

- [ ] **p_plugin_review 表** - 插件评论表
  - [ ] id, plugin_id, user_id, rating, content
  - [ ] reply_content, reply_time, status
  - [ ] 索引：idx_plugin, idx_user

### 1.2 Domain 实体

- [ ] **p_plugin.rs** - 插件实体实现完整
  - [ ] Model 结构体完整
  - [ ] Relation 枚举定义
  - [ ] Related 实现

- [ ] **p_plugin_version.rs** - 版本实体实现完整
- [ ] **p_plugin_category.rs** - 分类实体实现完整
- [ ] **p_plan.rs** - 套餐实体实现完整
- [ ] **p_order.rs** - 订单实体实现完整
- [ ] **p_cart.rs** - 购物车实体实现完整
- [ ] **p_subscription.rs** - 订阅实体实现完整
- [ ] **p_license.rs** - License实体实现完整
- [ ] **p_device.rs** - 设备实体实现完整
- [ ] **p_verification_code.rs** - 验证码实体实现完整
- [ ] **p_card.rs** - 卡密实体实现完整
- [ ] **p_card_batch.rs** - 批次实体实现完整
- [ ] **p_developer.rs** - 开发者实体实现完整
- [ ] **p_plugin_review.rs** - 评论实体实现完整

### 1.3 Domain 模型

- [ ] **m_plugin.rs** - 插件模型
  - [ ] PluginListItem - 列表项
  - [ ] PluginDetail - 详情
  - [ ] PluginStatistics - 统计
  - [ ] PluginSearchParams - 搜索参数

- [ ] **m_order.rs** - 订单模型
  - [ ] OrderListItem
  - [ ] OrderDetail
  - [ ] OrderStatistics

- [ ] **m_license.rs** - License模型
  - [ ] LicenseListItem
  - [ ] LicenseDetail
  - [ ] VerifyRequest
  - [ ] VerifyResponse

- [ ] **m_card.rs** - 卡密模型
  - [ ] CardItem
  - [ ] CardGenerateRequest

### 1.4 Domain 参数

- [ ] **a_plugin.rs** - 插件参数完整
- [ ] **a_order.rs** - 订单参数完整
- [ ] **a_license.rs** - License参数完整
- [ ] **a_card.rs** - 卡密参数完整

---

## 阶段二：业务服务层

### 2.1 插件服务

- [ ] **plugin_category_service.rs**
  - [ ] list() - 分类列表
  - [ ] tree() - 树形结构

- [ ] **plugin_service.rs**
  - [ ] market_list() - 市场列表
  - [ ] market_detail() - 市场详情
  - [ ] search() - 搜索
  - [ ] recommend() - 推荐
  - [ ] hot() - 热门
  - [ ] developer_list() - 开发者列表
  - [ ] add() - 新增
  - [ ] edit() - 编辑
  - [ ] delete() - 删除
  - [ ] audit() - 审核

- [ ] **plugin_version_service.rs**
  - [ ] list() - 版本列表
  - [ ] detail() - 版本详情
  - [ ] publish() - 发布新版本
  - [ ] latest() - 获取最新版本

### 2.2 交易服务

- [ ] **plan_service.rs**
  - [ ] list() - 套餐列表
  - [ ] detail() - 套餐详情
  - [ ] calculate_price() - 价格计算

- [ ] **cart_service.rs**
  - [ ] list() - 购物车列表
  - [ ] add() - 添加
  - [ ] remove() - 移除
  - [ ] clear() - 清空

- [ ] **order_service.rs**
  - [ ] create() - 创建订单
  - [ ] list() - 订单列表
  - [ ] detail() - 订单详情
  - [ ] cancel() - 取消订单
  - [ ] pay() - 发起支付
  - [ ] pay_callback() - 支付回调
  - [ ] statistics() - 订单统计

- [ ] **subscription_service.rs**
  - [ ] create() - 创建订阅
  - [ ] list() - 订阅列表
  - [ ] renew() - 续费
  - [ ] upgrade() - 升级
  - [ ] cancel() - 取消
  - [ ] check_expired() - 检查过期

### 2.3 License服务

- [ ] **license_service.rs**
  - [ ] generate() - 生成License
  - [ ] list() - License列表
  - [ ] detail() - License详情
  - [ ] bind_device() - 绑定设备
  - [ ] unbind_device() - 解绑设备
  - [ ] renew() - 续费
  - [ ] revoke() - 吊销

- [ ] **device_service.rs**
  - [ ] register() - 设备注册
  - [ ] list() - 设备列表
  - [ ] update_status() - 更新状态

- [ ] **verify_code_service.rs**
  - [ ] generate() - 生成验证码
  - [ ] send() - 发送验证码
  - [ ] check() - 校验验证码
  - [ ] block() - 封禁验证码

- [ ] **verification_service.rs**
  - [ ] verify_license() - License验证
  - [ ] heartbeat() - 心跳检测
  - [ ] validate_sign() - 签名校验

### 2.4 卡密服务

- [ ] **card_generate_service.rs**
  - [ ] generate() - 批量生成
  - [ ] export() - 导出
  - [ ] freeze() - 冻结

- [ ] **card_redeem_service.rs**
  - [ ] validate() - 校验卡密
  - [ ] redeem() - 兑换

---

## 阶段三：API路由层

### 3.1 插件市场API

- [ ] **GET /plugin/market/list** - 市场列表
  - [ ] 返回分页数据
  - [ ] 支持分类筛选
  - [ ] 支持价格筛选
  - [ ] 支持排序

- [ ] **GET /plugin/market/detail/{id}** - 市场详情
  - [ ] 返回完整插件信息
  - [ ] 返回套餐列表
  - [ ] 返回评分信息

- [ ] **GET /plugin/market/categories** - 分类列表
  - [ ] 返回树形结构
  - [ ] 返回插件数量

- [ ] **GET /plugin/market/search** - 搜索
  - [ ] 关键词搜索
  - [ ] 分页返回

- [ ] **GET /plugin/market/recommend** - 推荐插件
- [ ] **GET /plugin/market/hot** - 热门插件

### 3.2 插件管理API

- [ ] **GET /plugin/developer/list** - 开发者列表
- [ ] **POST /plugin/developer/add** - 新增插件
- [ ] **PUT /plugin/developer/edit** - 编辑插件
- [ ] **DELETE /plugin/developer/delete** - 删除插件
- [ ] **POST /plugin/developer/version/add** - 发布版本
- [ ] **GET /plugin/developer/stats** - 销售统计

### 3.3 订单API

- [ ] **GET /order/cart/list** - 购物车列表
- [ ] **POST /order/cart/add** - 添加购物车
- [ ] **DELETE /order/cart/remove** - 移除购物车
- [ ] **DELETE /order/cart/clear** - 清空购物车

- [ ] **POST /order/create** - 创建订单
- [ ] **GET /order/list** - 订单列表
- [ ] **GET /order/detail/{id}** - 订单详情
- [ ] **POST /order/cancel/{id}** - 取消订单
- [ ] **POST /order/pay/{id}** - 发起支付
- [ ] **POST /order/pay/callback** - 支付回调

### 3.4 License API

- [ ] **GET /license/list** - License列表
- [ ] **GET /license/detail/{id}** - License详情
- [ ] **POST /license/bind** - 绑定设备
- [ ] **POST /license/unbind** - 解绑设备
- [ ] **POST /license/renew** - 续费
- [ ] **POST /license/verify** - 验证License
- [ ] **POST /license/heartbeat** - 心跳

### 3.5 验证码API

- [ ] **POST /verify/code/send** - 发送验证码
- [ ] **POST /verify/code/check** - 校验验证码
- [ ] **POST /verify/device/register** - 注册设备
- [ ] **GET /verify/obfuscation/config** - 混淆配置

### 3.6 卡密API

- [ ] **POST /card/generate** - 生成卡密
- [ ] **GET /card/batch/list** - 批次列表
- [ ] **GET /card/export** - 导出卡密
- [ ] **POST /card/redeem** - 兑换卡密

---

## 阶段四：前端API模块

### 4.1 插件市场API

- [ ] **market.ts** - 市场接口
  - [ ] marketList() - 市场列表
  - [ ] marketDetail() - 市场详情
  - [ ] categories() - 分类
  - [ ] search() - 搜索
  - [ ] recommend() - 推荐
  - [ ] hot() - 热门

- [ ] **plugin.ts** - 插件管理接口
  - [ ] developerList()
  - [ ] add()
  - [ ] edit()
  - [ ] remove()
  - [ ] versionAdd()

### 4.2 订单API

- [ ] **cart.ts** - 购物车接口
  - [ ] cartList()
  - [ ] cartAdd()
  - [ ] cartRemove()
  - [ ] cartClear()

- [ ] **order.ts** - 订单接口
  - [ ] create()
  - [ ] list()
  - [ ] detail()
  - [ ] cancel()
  - [ ] pay()
  - [ ] callback()

### 4.3 License API

- [ ] **license.ts** - License接口
  - [ ] licenseList()
  - [ ] licenseDetail()
  - [ ] bindDevice()
  - [ ] unbindDevice()
  - [ ] renew()

- [ ] **device.ts** - 设备接口
  - [ ] deviceList()
  - [ ] deviceRegister()
  - [ ] deviceUpdate()

- [ ] **verify.ts** - 验证接口
  - [ ] sendCode()
  - [ ] checkCode()
  - [ ] verify()
  - [ ] heartbeat()

### 4.4 卡密API

- [ ] **card.ts** - 卡密接口
  - [ ] generate()
  - [ ] batchList()
  - [ ] export()
  - [ ] redeem()

---

## 阶段五：Mock数据

### 5.1 Mock接口

- [ ] **插件市场Mock**
  - [ ] 插件列表Mock数据
  - [ ] 插件详情Mock数据
  - [ ] 分类Mock数据
  - [ ] 推荐Mock数据
  - [ ] 热门Mock数据

- [ ] **订单Mock**
  - [ ] 购物车Mock数据
  - [ ] 订单Mock数据
  - [ ] 支付Mock

- [ ] **License Mock**
  - [ ] License列表Mock
  - [ ] 验证结果Mock
  - [ ] 设备列表Mock

- [ ] **卡密Mock**
  - [ ] 卡密生成Mock
  - [ ] 卡密兑换Mock

### 5.2 Mock路由注册

- [ ] Mock路由正确注册到index.ts
- [ ] 可通过环境变量切换Mock/真实API

---

## 阶段六：前端页面

### 6.1 插件市场页面

- [ ] **index.vue** - 市场首页
  - [ ] 分类展示
  - [ ] 推荐插件
  - [ ] 热门插件
  - [ ] 搜索框

- [ ] **list.vue** - 插件列表
  - [ ] 分类筛选
  - [ ] 价格筛选
  - [ ] 排序
  - [ ] 分页

- [ ] **detail.vue** - 插件详情
  - [ ] 基本信息Tab
  - [ ] 套餐选择Tab
  - [ ] 使用教程Tab
  - [ ] 更新日志Tab
  - [ ] 评论Tab
  - [ ] 购买按钮

- [ ] **search.vue** - 搜索页
  - [ ] 搜索结果
  - [ ] 筛选排序

### 6.2 我的插件页面

- [ ] **purchased.vue** - 已购插件
  - [ ] 已购列表
  - [ ] 续费入口
  - [ ] 升级入口

- [ ] **subscription.vue** - 订阅管理
  - [ ] 订阅列表
  - [ ] 自动续费开关
  - [ ] 到期提醒

- [ ] **license.vue** - License管理
  - [ ] License列表
  - [ ] 设备绑定管理
  - [ ] 解绑功能

### 6.3 开发者中心页面

- [ ] **dashboard.vue** - 开发者仪表盘
  - [ ] 销售概览
  - [ ] 下载统计
  - [ ] 收入图表

- [ ] **plugin-list.vue** - 插件管理
  - [ ] 插件CRUD
  - [ ] 状态切换

- [ ] **plugin-edit.vue** - 插件编辑
  - [ ] 信息编辑
  - [ ] 版本发布

- [ ] **version.vue** - 版本管理
  - [ ] 版本列表
  - [ ] 发布记录

- [ ] **sales.vue** - 销售统计
  - [ ] 订单统计
  - [ ] 收入报表

### 6.4 订单中心页面

- [ ] **cart.vue** - 购物车
  - [ ] 购物车列表
  - [ ] 删除
  - [ ] 结算

- [ ] **checkout.vue** - 结账页
  - [ ] 订单确认
  - [ ] 优惠码
  - [ ] 支付选择

- [ ] **list.vue** - 订单列表
  - [ ] 状态筛选
  - [ ] 订单列表

- [ ] **detail.vue** - 订单详情
  - [ ] 订单信息
  - [ ] License信息

### 6.5 验证中心页面

- [ ] **activate.vue** - 激活页面
  - [ ] 验证码输入
  - [ ] 设备信息

- [ ] **device.vue** - 设备管理
  - [ ] 设备列表
  - [ ] 解绑

- [ ] **card-redeem.vue** - 卡密兑换
  - [ ] 卡密输入
  - [ ] 兑换结果

---

## 阶段七：国际化与菜单

### 7.1 国际化

- [ ] **中文语言包** - 插件市场相关中文
  - [ ] 插件相关
  - [ ] 订单相关
  - [ ] License相关
  - [ ] 卡密相关

- [ ] **英文语言包** - 插件市场相关英文
  - [ ] 插件相关
  - [ ] 订单相关
  - [ ] License相关
  - [ ] 卡密相关

### 7.2 菜单

- [ ] **菜单迁移文件**
  - [ ] 插件市场菜单
  - [ ] 订单中心菜单
  - [ ] 开发者中心菜单
  - [ ] 验证中心菜单

- [ ] **前端路由配置**
  - [ ] 插件市场路由
  - [ ] 订单路由
  - [ ] License路由

---

## 阶段八：权限与安全

### 8.1 权限

- [ ] **API权限定义**
  - [ ] 插件市场权限
  - [ ] 开发者权限
  - [ ] License权限

- [ ] **按钮权限**
  - [ ] 购买按钮权限
  - [ ] 开发者操作权限

### 8.2 安全

- [ ] **签名验证**
  - [ ] 签名算法实现
  - [ ] 签名校验中间件

- [ ] **限流**
  - [ ] 验证码接口限流
  - [ ] 验证接口限流

---

## 功能完整性检查

### 核心业务流程

- [ ] **购买流程** - 从浏览到购买的完整流程
  - [ ] 插件浏览
  - [ ] 套餐选择
  - [ ] 加入购物车/直接购买
  - [ ] 订单创建
  - [ ] 支付
  - [ ] License生成

- [ ] **订阅流程** - 订阅管理完整流程
  - [ ] 订阅创建
  - [ ] 订阅续费
  - [ ] 订阅升级
  - [ ] 订阅取消

- [ ] **License流程** - License管理完整流程
  - [ ] License验证
  - [ ] 设备绑定
  - [ ] 心跳检测
  - [ ] 离线限制

- [ ] **卡密流程** - 卡密系统完整流程
  - [ ] 卡密生成
  - [ ] 卡密导出
  - [ ] 卡密兑换
  - [ ] 兑换验证

- [ ] **防破解流程** - 安全验证完整流程
  - [ ] 设备指纹
  - [ ] 验证码激活
  - [ ] License验证
  - [ ] 心跳检测

---

## 代码质量检查

### 代码风格

- [ ] **后端代码**
  - [ ] 符合Rust代码规范
  - [ ] 错误处理正确
  - [ ] 异步编程规范

- [ ] **前端代码**
  - [ ] 符合Vue3 Composition API规范
  - [ ] TypeScript类型完整
  - [ ] 命名规范

### 测试覆盖

- [ ] **API接口测试**
  - [ ] 插件市场接口测试
  - [ ] 订单接口测试
  - [ ] License接口测试

---

## 最终验收

- [ ] 所有数据库迁移可正常执行
- [ ] 所有API接口可正常调用
- [ ] 前端页面可正常访问
- [ ] Mock接口与真实接口可切换
- [ ] 权限控制正常生效
- [ ] 插件完整业务流程可跑通
- [ ] 代码风格符合项目规范