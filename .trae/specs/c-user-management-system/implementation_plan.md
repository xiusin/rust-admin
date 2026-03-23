# 用户管理系统开发方案

## 项目概述

基于 Kiro C端用户管理系统设计方案，将其转换为项目现有技术栈（Rust + Axum + Sea-ORM + Vue3）的后台管理模块。

**设计来源**: `/Users/tuoke/Desktop/qiluo_admin/.trae/specs/c-user-management-system`

**核心约束**:
- 不引入新中间件（如 Kafka），使用 Redis 实现事件驱动
- 保持业务完整性，将 Go 实现转换为 Rust 实现
- 忽略单元测试任务，确保业务逻辑完整实现

## 技术栈映射

| 原设计（Go） | 项目实现（Rust） |
|-------------|----------------|
| Kratos 框架 | Axum 0.8.x |
| gRPC + HTTP | HTTP REST API (Axum) |
| Ent ORM | Sea-ORM 1.1.x |
| Kafka EventBus | Redis Pub/Sub + 内存事件总线 |
| Go uuid | Rust uuid |

## 模块架构

```
src/
├── domain/
│   ├── entity/              # 数据库实体
│   │   ├── consumer.rs      # C端用户 (重命名为 consumer 而非 c_user)
│   │   ├── login_log.rs     # 登录日志
│   │   ├── sms_log.rs       # 短信日志
│   │   ├── payment_order.rs  # 支付订单
│   │   ├── finance_account.rs # 财务账户
│   │   ├── finance_transaction.rs # 财务流水
│   │   ├── media_file.rs    # 媒体文件
│   │   ├── logistics_tracking.rs # 物流跟踪
│   │   └── freight_template.rs # 运费模板
│   ├── model/               # 领域模型
│   └── args/                # 请求参数
├── application/
│   ├── consumer/            # 用户服务
│   │   ├── mod.rs
│   │   └── consumer_service.rs
│   ├── sms/                 # 短信服务
│   ├── payment/             # 支付服务
│   ├── finance/             # 财务服务
│   ├── wechat/              # 微信服务
│   ├── media/               # 媒体服务
│   ├── logistics/           # 物流服务
│   └── freight/             # 运费服务
├── infrastructure/
│   ├── sms/                 # 短信服务商实现
│   │   ├── aliyun.rs       # 阿里云短信
│   │   └── tencent.rs       # 腾讯云短信
│   ├── payment/             # 支付服务商实现
│   │   ├── wechat.rs        # 微信支付
│   │   ├── alipay.rs        # 支付宝
│   │   └── yeepay.rs        # 易宝支付
│   ├── oss/                 # 对象存储实现
│   │   ├── aliyun.rs        # 阿里云OSS
│   │   └── tencent.rs       # 腾讯云COS
│   ├── logistics/           # 物流服务商实现
│   │   └── kdniao.rs        # 快递鸟
│   └── eventbus/            # 事件总线 (基于 Redis)
│       ├── mod.rs
│       └── handlers.rs
└── api/
    ├── consumer.rs          # 用户相关接口
    ├── sms.rs               # 短信接口
    ├── payment.rs           # 支付接口
    ├── finance.rs           # 财务接口
    ├── wechat.rs            # 微信接口
    ├── media.rs             # 媒体接口
    ├── logistics.rs         # 物流接口
    └── freight.rs           # 运费接口
```

## 数据库设计

### 1. c_consumer (C端用户表)

```sql
CREATE TABLE `c_consumer` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `phone` varchar(20) NOT NULL COMMENT '手机号',
    `email` varchar(100) DEFAULT NULL COMMENT '邮箱',
    `nickname` varchar(50) DEFAULT NULL COMMENT '昵称',
    `avatar` varchar(500) DEFAULT NULL COMMENT '头像URL',
    `password_hash` varchar(255) NOT NULL COMMENT '密码哈希',
    `wechat_openid` varchar(100) DEFAULT NULL COMMENT '微信OpenID',
    `wechat_unionid` varchar(100) DEFAULT NULL COMMENT '微信UnionID',
    `status` enum('normal','locked','deactivated') DEFAULT 'normal' COMMENT '状态',
    `risk_score` int(11) DEFAULT 0 COMMENT '风险评分 0-100',
    `login_fail_count` int(11) DEFAULT 0 COMMENT '登录失败次数',
    `locked_until` datetime DEFAULT NULL COMMENT '锁定截止时间',
    `last_login_at` datetime DEFAULT NULL COMMENT '最后登录时间',
    `last_login_ip` varchar(50) DEFAULT NULL COMMENT '最后登录IP',
    `deactivated_at` datetime DEFAULT NULL COMMENT '注销时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tenant_phone` (`tenant_id`, `phone`),
    KEY `idx_tenant_wechat_openid` (`tenant_id`, `wechat_openid`),
    KEY `idx_tenant_status` (`tenant_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端用户表';
```

### 2. c_login_log (登录日志表)

```sql
CREATE TABLE `c_login_log` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `phone` varchar(20) NOT NULL COMMENT '手机号',
    `login_type` enum('phone','wechat') NOT NULL COMMENT '登录方式',
    `success` tinyint(1) NOT NULL COMMENT '是否成功',
    `fail_reason` varchar(200) DEFAULT NULL COMMENT '失败原因',
    `ip_address` varchar(50) NOT NULL COMMENT 'IP地址',
    `user_agent` varchar(500) DEFAULT NULL COMMENT 'User Agent',
    `device_type` varchar(50) DEFAULT NULL COMMENT '设备类型',
    `login_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '登录时间',
    PRIMARY KEY (`id`),
    KEY `idx_tenant_consumer_login_at` (`tenant_id`, `consumer_id`, `login_at`),
    KEY `idx_tenant_phone_login_at` (`tenant_id`, `phone`, `login_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='登录日志表';
```

### 3. c_sms_log (短信日志表)

```sql
CREATE TABLE `c_sms_log` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `phone` varchar(20) NOT NULL COMMENT '手机号',
    `sms_type` enum('verification','notification') NOT NULL COMMENT '短信类型',
    `content` varchar(500) NOT NULL COMMENT '短信内容',
    `code` varchar(10) DEFAULT NULL COMMENT '验证码',
    `channel` enum('aliyun','tencent') NOT NULL COMMENT '短信通道',
    `status` enum('pending','success','failed') NOT NULL COMMENT '发送状态',
    `error_message` varchar(200) DEFAULT NULL COMMENT '错误信息',
    `sent_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '发送时间',
    `expires_at` datetime DEFAULT NULL COMMENT '过期时间',
    PRIMARY KEY (`id`),
    KEY `idx_tenant_phone_sent_at` (`tenant_id`, `phone`, `sent_at`),
    KEY `idx_tenant_sms_type_status` (`tenant_id`, `sms_type`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='短信日志表';
```

### 4. c_payment_order (支付订单表)

```sql
CREATE TABLE `c_payment_order` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `order_no` varchar(64) NOT NULL COMMENT '订单号',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `payment_method` enum('wechat','alipay','yeepay') NOT NULL COMMENT '支付方式',
    `payment_type` enum('app','h5','mini','qrcode') NOT NULL COMMENT '支付类型',
    `amount` decimal(10,2) NOT NULL COMMENT '支付金额',
    `status` enum('pending','success','failed','closed','refunded') DEFAULT 'pending' COMMENT '订单状态',
    `transaction_id` varchar(100) DEFAULT NULL COMMENT '第三方交易号',
    `callback_data` varchar(2000) DEFAULT NULL COMMENT '回调数据',
    `paid_at` datetime DEFAULT NULL COMMENT '支付时间',
    `closed_at` datetime DEFAULT NULL COMMENT '关闭时间',
    `expires_at` datetime NOT NULL COMMENT '过期时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tenant_order_no` (`tenant_id`, `order_no`),
    KEY `idx_tenant_consumer_status` (`tenant_id`, `consumer_id`, `status`),
    KEY `idx_transaction_id` (`transaction_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='支付订单表';
```

### 5. c_finance_account (财务账户表)

```sql
CREATE TABLE `c_finance_account` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `balance` decimal(12,2) DEFAULT '0.00' COMMENT '账户余额',
    `frozen_balance` decimal(12,2) DEFAULT '0.00' COMMENT '冻结余额',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tenant_consumer` (`tenant_id`, `consumer_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='财务账户表';
```

### 6. c_finance_transaction (财务流水表)

```sql
CREATE TABLE `c_finance_transaction` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `transaction_no` varchar(64) NOT NULL COMMENT '流水号',
    `transaction_type` enum('recharge','consume','withdraw','refund') NOT NULL COMMENT '交易类型',
    `amount` decimal(10,2) NOT NULL COMMENT '交易金额',
    `balance_before` decimal(12,2) NOT NULL COMMENT '交易前余额',
    `balance_after` decimal(12,2) NOT NULL COMMENT '交易后余额',
    `description` varchar(200) DEFAULT NULL COMMENT '交易描述',
    `related_order_no` varchar(64) DEFAULT NULL COMMENT '关联订单号',
    `operator_id` bigint(20) DEFAULT NULL COMMENT '操作人ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_transaction_no` (`transaction_no`),
    KEY `idx_tenant_consumer_created_at` (`tenant_id`, `consumer_id`, `created_at`),
    KEY `idx_tenant_transaction_type_created_at` (`tenant_id`, `transaction_type`, `created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='财务流水表';
```

### 7. c_media_file (媒体文件表)

```sql
CREATE TABLE `c_media_file` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '上传者ID',
    `file_name` varchar(255) NOT NULL COMMENT '文件名',
    `file_type` enum('image','video') NOT NULL COMMENT '文件类型',
    `file_format` varchar(20) NOT NULL COMMENT '文件格式',
    `file_size` bigint(20) NOT NULL COMMENT '文件大小（字节）',
    `file_url` varchar(500) NOT NULL COMMENT '文件URL',
    `thumbnail_url` varchar(500) DEFAULT NULL COMMENT '缩略图URL',
    `oss_bucket` varchar(100) NOT NULL COMMENT 'OSS Bucket',
    `oss_key` varchar(500) NOT NULL COMMENT 'OSS Key',
    `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_tenant_consumer_deleted` (`tenant_id`, `consumer_id`, `is_deleted`),
    KEY `idx_tenant_file_type_created_at` (`tenant_id`, `file_type`, `created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='媒体文件表';
```

### 8. c_logistics_tracking (物流跟踪表)

```sql
CREATE TABLE `c_logistics_tracking` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `tracking_no` varchar(100) NOT NULL COMMENT '运单号',
    `courier_company` varchar(50) NOT NULL COMMENT '快递公司',
    `status` enum('pending','in_transit','delivering','delivered') DEFAULT 'pending' COMMENT '物流状态',
    `tracking_info` json DEFAULT NULL COMMENT '物流轨迹',
    `last_updated_at` datetime DEFAULT NULL COMMENT '最后更新时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tenant_tracking_no` (`tenant_id`, `tracking_no`),
    KEY `idx_tenant_status` (`tenant_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物流跟踪表';
```

### 9. c_freight_template (运费模板表)

```sql
CREATE TABLE `c_freight_template` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `tenant_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '租户ID',
    `name` varchar(100) NOT NULL COMMENT '模板名称',
    `calculation_type` enum('by_weight','by_distance') NOT NULL COMMENT '计算方式',
    `first_weight` decimal(8,3) DEFAULT NULL COMMENT '首重（kg）',
    `first_price` decimal(10,2) DEFAULT NULL COMMENT '首重价格',
    `additional_weight` decimal(8,3) DEFAULT NULL COMMENT '续重（kg）',
    `additional_price` decimal(10,2) DEFAULT NULL COMMENT '续重价格',
    `region_rules` json DEFAULT NULL COMMENT '地区规则',
    `free_shipping_rules` json DEFAULT NULL COMMENT '包邮规则',
    `is_active` tinyint(1) DEFAULT 1 COMMENT '是否启用',
    `created_by` bigint(20) DEFAULT NULL COMMENT '创建者ID',
    `updated_by` bigint(20) DEFAULT NULL COMMENT '更新者ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_tenant_is_active` (`tenant_id`, `is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='运费模板表';
```

## 数据库迁移文件

迁移文件将按以下格式创建:
- `m{YYYYMMDD}_{序号}_{模块}_{操作类型}.sql`

## API 接口设计

### 1. 用户服务 (Consumer Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| POST | /api/consumer/register | 手机号注册 |
| POST | /api/consumer/login | 手机号登录 |
| POST | /api/consumer/login/wechat | 微信登录 |
| GET | /api/consumer/info | 获取用户信息 |
| PUT | /api/consumer/update | 更新用户信息 |
| PUT | /api/consumer/phone | 更新手机号 |
| PUT | /api/consumer/email | 更新邮箱 |
| POST | /api/consumer/avatar | 上传头像 |
| POST | /api/consumer/deactivate | 注销账户 |
| GET | /api/consumer/login-logs | 查询登录日志 |
| GET | /api/consumer/list | 查询用户列表(管理员) |

### 2. 短信服务 (SMS Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| POST | /api/sms/send-code | 发送验证码 |
| POST | /api/sms/verify-code | 验证验证码 |
| POST | /api/sms/send-notification | 发送通知短信 |
| GET | /api/sms/logs | 查询短信日志 |

### 3. 支付服务 (Payment Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| POST | /api/payment/create | 创建支付订单 |
| GET | /api/payment/:order_no | 查询支付订单 |
| GET | /api/payment/status/:order_no | 查询支付结果 |
| POST | /api/payment/refund | 申请退款 |
| GET | /api/payment/refund/:order_no | 查询退款状态 |
| POST | /api/payment/callback/:method | 支付回调 |
| GET | /api/payment/list | 查询支付流水 |

### 4. 财务服务 (Finance Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| GET | /api/finance/account | 获取账户余额 |
| POST | /api/finance/recharge | 充值 |
| POST | /api/finance/withdraw | 申请提现 |
| POST | /api/finance/withdraw/approve | 审核提现 |
| GET | /api/finance/transactions | 查询财务流水 |
| GET | /api/finance/export | 导出财务流水 |

### 5. 微信服务 (Wechat Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| GET | /api/wechat/auth-url | 获取微信授权URL |
| GET | /api/wechat/callback | 微信授权回调 |
| GET | /api/wechat/user-info | 获取微信用户信息 |
| POST | /api/wechat/template-message | 发送模板消息 |
| POST | /api/wechat/mini/login | 小程序登录 |

### 6. 媒体服务 (Media Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| POST | /api/media/upload-url | 生成上传预签名URL |
| POST | /api/media/confirm | 确认上传完成 |
| GET | /api/media/:id | 获取媒体文件 |
| GET | /api/media/list | 查询媒体文件列表 |
| DELETE | /api/media/:id | 删除媒体文件 |

### 7. 物流服务 (Logistics Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| GET | /api/logistics/query | 查询物流信息 |
| POST | /api/logistics/subscribe | 订阅物流状态 |
| GET | /api/logistics/history | 查询物流历史 |

### 8. 运费服务 (Freight Service)

| 方法 | 路径 | 描述 |
|-----|------|------|
| POST | /api/freight/calculate | 计算运费 |
| POST | /api/freight/template | 创建运费模板 |
| PUT | /api/freight/template/:id | 更新运费模板 |
| GET | /api/freight/template/:id | 查询运费模板 |
| GET | /api/freight/templates | 查询运费模板列表 |

## 事件驱动设计

### 使用 Redis Pub/Sub 替代 Kafka

```rust
// 事件类型定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    UserRegistered { consumer_id: i64, phone: String, tenant_id: i64 },
    PaymentSuccess { order_no: String, consumer_id: i64, amount: Decimal, tenant_id: i64 },
    LogisticsStatusChanged { tracking_no: String, status: String, tenant_id: i64 },
}

// 事件总线实现
pub struct EventBus {
    redis: RedisPool,
}

impl EventBus {
    pub async fn publish(&self, event: Event) -> Result<()> {
        let channel = match &event {
            Event::UserRegistered { .. } => "user.registered",
            Event::PaymentSuccess { .. } => "payment.success",
            Event::LogisticsStatusChanged { .. } => "logistics.status_changed",
        };
        let payload = serde_json::to_string(&event)?;
        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(&payload)
            .query_async(&mut self.redis.acquire().await?)
            .await?;
        Ok(())
    }
}

// 事件处理器
pub struct EventHandlers {
    finance_service: FinanceService,
}

impl EventHandlers {
    pub async fn handle_payment_success(&self, payload: PaymentSuccessPayload) -> Result<()> {
        // 充值用户余额
        self.finance_service.recharge_from_payment(payload).await
    }
}
```

## 开发任务清单

### 阶段一：基础设施 (Task 1-3)

- [ ] 1.1 创建数据库迁移文件
- [ ] 1.2 实现 Domain 层实体 (c_consumer, c_login_log, c_sms_log, c_payment_order, c_finance_account, c_finance_transaction, c_media_file, c_logistics_tracking, c_freight_template)
- [ ] 1.3 创建 Sea-ORM Entity 代码

### 阶段二：核心服务 (Task 4-11)

- [ ] 2.1 实现 Consumer Service (用户服务)
  - 用户注册、登录、信息管理
  - 登录日志记录
  - 账户锁定和风险评分
- [ ] 2.2 实现 SMS Service (短信服务)
  - 验证码发送和验证
  - 频率限制
  - 阿里云/腾讯云通道
- [ ] 2.3 实现 Payment Service (支付服务)
  - 支付订单创建
  - 支付回调处理
  - 退款操作
- [ ] 2.4 实现 Finance Service (财务服务)
  - 账户余额管理
  - 充值、提现
  - 财务流水
- [ ] 2.5 实现 Wechat Service (微信服务)
  - OAuth 登录
  - 模板消息
  - 小程序登录
- [ ] 2.6 实现 Media Service (媒体服务)
  - 文件上传
  - 预签名URL
  - 缩略图生成
- [ ] 2.7 实现 Logistics Service (物流服务)
  - 物流查询
  - 物流跟踪
- [ ] 2.8 实现 Freight Service (运费服务)
  - 运费计算
  - 运费模板管理

### 阶段三：事件驱动 (Task 12)

- [ ] 3.1 实现 Redis EventBus
- [ ] 3.2 实现事件订阅和处理

### 阶段四：API 层 (Task 13)

- [ ] 4.1 实现 API 路由
- [ ] 4.2 实现请求验证
- [ ] 4.3 实现响应格式化

### 阶段五：前端 (Task 14)

- [ ] 5.1 创建前端 API 模块
- [ ] 5.2 创建前端页面组件
- [ ] 5.3 配置路由

## 前端页面结构

```
EcomAdmin/src/views/
├── consumer/
│   ├── register.vue        # 用户注册
│   ├── login.vue           # 用户登录
│   ├── profile.vue         # 个人信息
│   ├── avatar.vue          # 头像上传
│   ├── security.vue         # 安全设置
│   └── login-logs.vue      # 登录日志
├── finance/
│   ├── account.vue         # 账户余额
│   ├── recharge.vue        # 充值
│   ├── withdraw.vue        # 提现
│   └── transactions.vue    # 财务流水
├── media/
│   ├── upload.vue          # 文件上传
│   └── list.vue            # 文件列表
├── logistics/
│   ├── query.vue           # 物流查询
│   └── tracking.vue        # 物流跟踪
└── freight/
    ├── calculator.vue       # 运费计算
    └── templates.vue        # 运费模板
```

## 关键实现细节

### 1. 多租户隔离

所有查询自动添加 `tenant_id` 过滤条件，从 JWT token 中提取当前租户ID。

### 2. 密码安全

使用 argon2 算法加密（项目已有），而非 bcrypt。

### 3. 验证码存储

使用 Redis 存储验证码，设置 5 分钟过期时间。

### 4. 支付回调签名验证

验证微信/支付宝回调签名的有效性。

### 5. 金额精度

使用 `rust_decimal` crate 处理金额计算，确保精确到分。

### 6. 物流缓存

使用 Redis 缓存物流查询结果，30 分钟过期。

## 依赖项

需要添加以下依赖到 `Cargo.toml`:

```toml
# 已有
argon2 = "0.5.3"
redis = { version = "0.32", features = ["aio", "default", "tokio-comp", "connection-manager"] }

# 需要添加
rust_decimal = { version = "1.33", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
base64 = "0.22"
image = "0.25"
qrcode = "0.14"
```

## 注意事项

1. **无 C端模块**: 所有功能作为总后台模块开发，不存在独立的 C端服务
2. **事件驱动**: 使用 Redis Pub/Sub 替代 Kafka
3. **租户隔离**: 所有数据表包含 tenant_id 字段
4. **业务完整性**: 不简化任何功能，确保所有业务流程完整覆盖
5. **精度保证**: 金额计算使用 decimal 类型
