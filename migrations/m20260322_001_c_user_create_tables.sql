-- m20260322_001_c_user_create_tables.sql
-- 描述：C端用户管理系统表结构（无租户字段）
-- 作者：tuoke
-- 日期：2026-03-22

-- 创建消费者表
CREATE TABLE IF NOT EXISTS `c_consumer` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
    `email` varchar(100) DEFAULT NULL COMMENT '邮箱',
    `nickname` varchar(50) DEFAULT NULL COMMENT '昵称',
    `avatar` varchar(255) DEFAULT NULL COMMENT '头像URL',
    `password_hash` varchar(255) NOT NULL COMMENT '密码哈希',
    `wechat_openid` varchar(100) DEFAULT NULL COMMENT '微信OpenID',
    `wechat_unionid` varchar(100) DEFAULT NULL COMMENT '微信UnionID',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用 2锁定）',
    `risk_score` int(11) NOT NULL DEFAULT 0 COMMENT '风险评分',
    `login_fail_count` int(11) NOT NULL DEFAULT 0 COMMENT '登录失败次数',
    `locked_until` datetime DEFAULT NULL COMMENT '锁定截止时间',
    `last_login_at` datetime DEFAULT NULL COMMENT '最后登录时间',
    `last_login_ip` varchar(50) DEFAULT NULL COMMENT '最后登录IP',
    `deactivated_at` datetime DEFAULT NULL COMMENT '注销时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_phone` (`phone`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='消费者表';

-- 创建登录日志表
CREATE TABLE IF NOT EXISTS `c_login_log` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '消费者ID',
    `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
    `login_type` varchar(20) NOT NULL COMMENT '登录类型（password/sms/wechat）',
    `success` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否成功',
    `fail_reason` varchar(255) DEFAULT NULL COMMENT '失败原因',
    `ip_address` varchar(50) DEFAULT NULL COMMENT 'IP地址',
    `user_agent` varchar(500) DEFAULT NULL COMMENT 'User-Agent',
    `device_type` varchar(20) DEFAULT NULL COMMENT '设备类型',
    `login_at` datetime NOT NULL COMMENT '登录时间',
    PRIMARY KEY (`id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_login_at` (`login_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端登录日志表';

-- 创建财务账户表
CREATE TABLE IF NOT EXISTS `c_finance_account` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '消费者ID',
    `balance` decimal(20,2) NOT NULL DEFAULT 0.00 COMMENT '账户余额',
    `frozen_balance` decimal(20,2) NOT NULL DEFAULT 0.00 COMMENT '冻结金额',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_consumer` (`consumer_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端财务账户表';

-- 创建财务流水表
CREATE TABLE IF NOT EXISTS `c_finance_transaction` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '消费者ID',
    `transaction_no` varchar(50) NOT NULL COMMENT '交易流水号',
    `transaction_type` varchar(20) NOT NULL COMMENT '交易类型（recharge/withdraw/consume/refund）',
    `amount` decimal(20,2) NOT NULL COMMENT '交易金额',
    `balance_before` decimal(20,2) NOT NULL COMMENT '交易前余额',
    `balance_after` decimal(20,2) NOT NULL COMMENT '交易后余额',
    `description` varchar(500) DEFAULT NULL COMMENT '描述',
    `related_order_no` varchar(50) DEFAULT NULL COMMENT '关联订单号',
    `operator_id` bigint(20) DEFAULT NULL COMMENT '操作员ID',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_transaction_no` (`transaction_no`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端财务流水表';

-- 创建支付订单表
CREATE TABLE IF NOT EXISTS `c_payment_order` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `order_no` varchar(50) NOT NULL COMMENT '订单号',
    `consumer_id` bigint(20) NOT NULL COMMENT '消费者ID',
    `payment_method` varchar(20) NOT NULL COMMENT '支付方式（wechat/alipay/yeepay）',
    `payment_type` varchar(10) NOT NULL COMMENT '支付类型（app/h5/mini/qrcode）',
    `amount` decimal(20,2) NOT NULL COMMENT '支付金额',
    `status` varchar(20) NOT NULL COMMENT '状态（pending/success/failed/closed/expired）',
    `transaction_id` varchar(100) DEFAULT NULL COMMENT '第三方交易ID',
    `callback_data` text DEFAULT NULL COMMENT '回调数据',
    `paid_at` datetime DEFAULT NULL COMMENT '支付时间',
    `closed_at` datetime DEFAULT NULL COMMENT '关闭时间',
    `expires_at` datetime DEFAULT NULL COMMENT '过期时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_no` (`order_no`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端支付订单表';

-- 创建短信日志表
CREATE TABLE IF NOT EXISTS `c_sms_log` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `phone` varchar(20) NOT NULL COMMENT '手机号',
    `sms_type` varchar(20) NOT NULL COMMENT '短信类型（verification/notification/marketing）',
    `content` text DEFAULT NULL COMMENT '短信内容',
    `code` varchar(10) DEFAULT NULL COMMENT '验证码',
    `channel` varchar(20) NOT NULL COMMENT '发送渠道',
    `status` varchar(20) NOT NULL COMMENT '状态（sent/failed/verified/expired）',
    `error_message` varchar(255) DEFAULT NULL COMMENT '错误信息',
    `sent_at` datetime DEFAULT NULL COMMENT '发送时间',
    `expires_at` datetime DEFAULT NULL COMMENT '过期时间',
    PRIMARY KEY (`id`),
    KEY `idx_phone` (`phone`),
    KEY `idx_sent_at` (`sent_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端短信日志表';

-- 创建媒体文件表
CREATE TABLE IF NOT EXISTS `c_media_file` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '上传者ID',
    `file_name` varchar(255) NOT NULL COMMENT '文件名',
    `file_type` varchar(20) NOT NULL COMMENT '文件类型（image/video/audio/document）',
    `file_format` varchar(20) DEFAULT NULL COMMENT '文件格式',
    `file_size` bigint(20) NOT NULL COMMENT '文件大小（字节）',
    `file_url` varchar(500) DEFAULT NULL COMMENT '文件URL',
    `thumbnail_url` varchar(500) DEFAULT NULL COMMENT '缩略图URL',
    `oss_bucket` varchar(100) DEFAULT NULL COMMENT 'OSS Bucket',
    `oss_key` varchar(255) DEFAULT NULL COMMENT 'OSS Key',
    `is_deleted` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否删除',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_file_type` (`file_type`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端媒体文件表';

-- 创建运费模板表
CREATE TABLE IF NOT EXISTS `c_freight_template` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '模板名称',
    `calculation_type` varchar(20) NOT NULL COMMENT '计算类型（by_weight/by_distance）',
    `first_weight` decimal(10,2) DEFAULT NULL COMMENT '首重/首件',
    `first_price` decimal(10,2) DEFAULT NULL COMMENT '首重/首件价格',
    `additional_weight` decimal(10,2) DEFAULT NULL COMMENT '续重/续件',
    `additional_price` decimal(10,2) DEFAULT NULL COMMENT '续重/续件价格',
    `region_rules` json DEFAULT NULL COMMENT '区域规则',
    `free_shipping_rules` json DEFAULT NULL COMMENT '包邮规则',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `created_by` bigint(20) DEFAULT NULL COMMENT '创建者',
    `updated_by` bigint(20) DEFAULT NULL COMMENT '更新者',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端运费模板表';

-- 创建物流追踪表
CREATE TABLE IF NOT EXISTS `c_logistics_tracking` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `tracking_no` varchar(50) NOT NULL COMMENT '物流单号',
    `courier_company` varchar(50) DEFAULT NULL COMMENT '快递公司',
    `status` varchar(20) NOT NULL COMMENT '状态（pending/subscribed/in_transit/delivered/exception）',
    `tracking_info` json DEFAULT NULL COMMENT '物流详情',
    `last_updated_at` datetime DEFAULT NULL COMMENT '最后更新时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tracking_no` (`tracking_no`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='C端物流追踪表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `c_logistics_tracking`;
-- DROP TABLE IF EXISTS `c_freight_template`;
-- DROP TABLE IF EXISTS `c_media_file`;
-- DROP TABLE IF EXISTS `c_sms_log`;
-- DROP TABLE IF EXISTS `c_payment_order`;
-- DROP TABLE IF EXISTS `c_finance_transaction`;
-- DROP TABLE IF EXISTS `c_finance_account`;
-- DROP TABLE IF EXISTS `c_login_log`;
-- DROP TABLE IF EXISTS `c_consumer`;