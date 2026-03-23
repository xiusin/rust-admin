-- m20260322_018_create_config_tables.sql
-- 描述：创建配置管理相关表
-- 作者：tuoke
-- 日期：2026-03-22

-- ============================================
-- 一、微信公众号配置表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_wechat_mp_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '公众号名称',
    `app_id` varchar(50) NOT NULL COMMENT 'AppID',
    `app_secret` varchar(100) NOT NULL COMMENT 'AppSecret',
    `token` varchar(100) DEFAULT NULL COMMENT 'Token',
    `encoding_aes_key` varchar(100) DEFAULT NULL COMMENT 'EncodingAESKey',
    `mp_type` varchar(20) NOT NULL DEFAULT 'service' COMMENT '类型: service/subscribe',
    `verified` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否认证: 0否 1是',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_app_id` (`app_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='微信公众号配置表';

-- ============================================
-- 二、微信小程序配置表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_wechat_mini_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '小程序名称',
    `app_id` varchar(50) NOT NULL COMMENT 'AppID',
    `app_secret` varchar(100) NOT NULL COMMENT 'AppSecret',
    `original_id` varchar(50) DEFAULT NULL COMMENT '原始ID',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_app_id` (`app_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='微信小程序配置表';

-- ============================================
-- 三、短信配置表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_sms_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '配置名称',
    `provider` varchar(50) NOT NULL COMMENT '服务商: aliyun/tencent/huawei',
    `access_key` varchar(100) NOT NULL COMMENT 'AccessKey',
    `access_secret` varchar(100) NOT NULL COMMENT 'AccessSecret',
    `sign_name` varchar(50) NOT NULL COMMENT '短信签名',
    `region` varchar(50) DEFAULT NULL COMMENT '区域',
    `is_default` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否默认: 0否 1是',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_provider` (`provider`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='短信配置表';

-- ============================================
-- 四、短信模板表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_sms_template` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '模板名称',
    `template_code` varchar(50) NOT NULL COMMENT '模板编码',
    `template_type` varchar(20) NOT NULL COMMENT '模板类型: verification/notification/marketing',
    `content` text COMMENT '模板内容',
    `params` json DEFAULT NULL COMMENT '参数说明',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_template_code` (`template_code`),
    KEY `idx_template_type` (`template_type`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='短信模板表';

-- ============================================
-- 五、OSS存储配置表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_oss_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '配置名称',
    `storage_type` varchar(20) NOT NULL COMMENT '存储类型: aliyun/tencent/qiniu/minio',
    `endpoint` varchar(200) NOT NULL COMMENT 'Endpoint',
    `bucket` varchar(100) NOT NULL COMMENT 'Bucket名称',
    `access_key` varchar(100) NOT NULL COMMENT 'AccessKey',
    `secret_key` varchar(100) NOT NULL COMMENT 'SecretKey',
    `region` varchar(50) DEFAULT NULL COMMENT '区域',
    `domain` varchar(200) DEFAULT NULL COMMENT '自定义域名',
    `is_default` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否默认: 0否 1是',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_storage_type` (`storage_type`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='OSS存储配置表';

-- ============================================
-- 六、物流公司表
-- ============================================
CREATE TABLE IF NOT EXISTS `c_logistics_company` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(50) NOT NULL COMMENT '公司名称',
    `code` varchar(20) NOT NULL COMMENT '公司编码',
    `logo` varchar(200) DEFAULT NULL COMMENT 'Logo URL',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态: 0停用 1启用',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物流公司表';

-- ============================================
-- 七、插入测试数据
-- ============================================
-- 微信公众号配置
INSERT IGNORE INTO `c_wechat_mp_config` (`id`, `name`, `app_id`, `app_secret`, `mp_type`, `verified`, `status`, `remark`, `created_at`) VALUES
(1, '奇洛商城公众号', 'wx1234567890abcdef', 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX', 'service', 1, 1, '商城服务号', NOW()),
(2, '奇洛订阅号', 'wxabcdef1234567890', 'YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY', 'subscribe', 0, 1, '商城订阅号', NOW());

-- 微信小程序配置
INSERT IGNORE INTO `c_wechat_mini_config` (`id`, `name`, `app_id`, `app_secret`, `original_id`, `status`, `remark`, `created_at`) VALUES
(1, '奇洛商城小程序', 'wxabcdef1234567890', 'YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY', 'gh_1234567890ab', 1, '商城小程序', NOW());

-- 短信配置
INSERT IGNORE INTO `c_sms_config` (`id`, `name`, `provider`, `access_key`, `access_secret`, `sign_name`, `region`, `is_default`, `status`, `remark`, `created_at`) VALUES
(1, '阿里云短信配置', 'aliyun', 'LTAI5tXXXXXXXXXX', 'XXXXXXXXXXXXXXXXXXXX', '奇洛商城', 'cn-hangzhou', 1, 1, '主配置', NOW());

-- 短信模板
INSERT IGNORE INTO `c_sms_template` (`id`, `name`, `template_code`, `template_type`, `content`, `status`, `remark`, `created_at`) VALUES
(1, '验证码模板', 'SMS_001', 'verification', '您的验证码是${code}，有效期${expire}分钟', 1, '验证码模板', NOW()),
(2, '订单发货通知', 'SMS_002', 'notification', '您的订单已发货，快递单号：${tracking_no}', 1, '发货通知', NOW()),
(3, '订单支付成功', 'SMS_003', 'notification', '您的订单已支付成功，订单号：${order_no}', 1, '支付通知', NOW());

-- OSS配置
INSERT IGNORE INTO `c_oss_config` (`id`, `name`, `storage_type`, `endpoint`, `bucket`, `access_key`, `secret_key`, `region`, `domain`, `is_default`, `status`, `remark`, `created_at`) VALUES
(1, '阿里云OSS', 'aliyun', 'oss-cn-hangzhou.aliyuncs.com', 'qiluo-admin', 'LTAI5tXXXXXXXXXX', 'XXXXXXXXXXXXXXXXXXXX', 'cn-hangzhou', 'https://cdn.qiluo.com', 1, 1, '主存储', NOW());

-- 物流公司
INSERT IGNORE INTO `c_logistics_company` (`id`, `name`, `code`, `logo`, `status`, `sort`, `created_at`) VALUES
(1, '顺丰速运', 'SF', 'https://img.example.com/sf.png', 1, 1, NOW()),
(2, '中通快递', 'ZTO', 'https://img.example.com/zto.png', 1, 2, NOW()),
(3, '圆通速递', 'YTO', 'https://img.example.com/yto.png', 1, 3, NOW()),
(4, '韵达快递', 'YD', 'https://img.example.com/yd.png', 1, 4, NOW()),
(5, '申通快递', 'STO', 'https://img.example.com/sto.png', 1, 5, NOW()),
(6, '极兔速递', 'JTSD', 'https://img.example.com/jt.png', 1, 6, NOW()),
(7, '邮政EMS', 'EMS', 'https://img.example.com/ems.png', 1, 7, NOW()),
(8, '京东物流', 'JD', 'https://img.example.com/jd.png', 1, 8, NOW());

-- ROLLBACK:
-- DROP TABLE IF EXISTS `c_logistics_company`;
-- DROP TABLE IF EXISTS `c_oss_config`;
-- DROP TABLE IF EXISTS `c_sms_template`;
-- DROP TABLE IF EXISTS `c_sms_config`;
-- DROP TABLE IF EXISTS `c_wechat_mini_config`;
-- DROP TABLE IF EXISTS `c_wechat_mp_config`;
