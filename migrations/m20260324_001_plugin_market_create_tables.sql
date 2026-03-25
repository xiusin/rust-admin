-- m20260324_001_plugin_market_create_tables.sql
-- 描述：插件市场/应用市场核心表结构
-- 作者：tuoke
-- 日期：2026-03-24

-- =====================================================
-- 1. 插件分类表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_plugin_category` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '分类名称',
    `icon` varchar(100) DEFAULT NULL COMMENT '分类图标',
    `parent_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '父分类ID，0为顶级分类',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `plugin_count` int(11) NOT NULL DEFAULT 0 COMMENT '插件数量',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_parent_id` (`parent_id`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件分类表';

-- =====================================================
-- 2. 开发者表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_developer` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '开发者名称',
    `logo` varchar(500) DEFAULT NULL COMMENT '开发者Logo',
    `description` text COMMENT '开发者描述',
    `contact` varchar(100) DEFAULT NULL COMMENT '联系方式',
    `plugins_count` int(11) NOT NULL DEFAULT 0 COMMENT '插件数量',
    `total_downloads` bigint(20) NOT NULL DEFAULT 0 COMMENT '总下载量',
    `rating` decimal(3,2) NOT NULL DEFAULT 5.00 COMMENT '平均评分',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='开发者表';

-- =====================================================
-- 3. 插件表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_plugin` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `code` varchar(64) NOT NULL DEFAULT '' COMMENT '插件编码（唯一）',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '插件名称',
    `category_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '分类ID',
    `developer_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '开发者ID',
    `summary` varchar(255) DEFAULT NULL COMMENT '简介',
    `description` text COMMENT '详细描述',
    `cover_image` varchar(500) DEFAULT NULL COMMENT '封面图',
    `screenshots` json DEFAULT NULL COMMENT '截图列表',
    `version` varchar(32) NOT NULL DEFAULT '1.0.0' COMMENT '当前版本',
    `price_type` tinyint(1) NOT NULL DEFAULT 0 COMMENT '价格类型（0免费 1一次性 2订阅）',
    `verify_level` tinyint(1) NOT NULL DEFAULT 0 COMMENT '验证级别（0无 1基础 2高级）',
    `download_count` int(11) NOT NULL DEFAULT 0 COMMENT '下载次数',
    `rating` decimal(3,2) NOT NULL DEFAULT 5.00 COMMENT '评分',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '状态（0待审核 1上架 2下架 3回收站）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `tags` json DEFAULT NULL COMMENT '标签',
    `is_official` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否官方插件（0否 1是）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_category_id` (`category_id`),
    KEY `idx_developer_id` (`developer_id`),
    KEY `idx_status` (`status`),
    KEY `idx_rating` (`rating`),
    KEY `idx_download_count` (`download_count`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件表';

-- =====================================================
-- 4. 插件版本表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_plugin_version` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `version` varchar(32) NOT NULL DEFAULT '' COMMENT '版本号',
    `changelog` text COMMENT '变更日志',
    `download_url` varchar(500) NOT NULL DEFAULT '' COMMENT '下载链接',
    `file_hash` varchar(64) NOT NULL DEFAULT '' COMMENT '文件哈希（SHA256）',
    `file_size` bigint(20) NOT NULL DEFAULT 0 COMMENT '文件大小（字节）',
    `min_app_version` varchar(32) DEFAULT NULL COMMENT '最低兼容版本',
    `is_latest` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否最新版本（0否 1是）',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_is_latest` (`is_latest`),
    KEY `idx_version` (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件版本表';

-- =====================================================
-- 5. 套餐表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_plan` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '套餐名称（如：基础版、专业版、企业版）',
    `description` varchar(255) DEFAULT NULL COMMENT '套餐描述',
    `period_type` tinyint(1) NOT NULL DEFAULT 0 COMMENT '周期类型（0月 1季 2年 3永久）',
    `period_days` int(11) NOT NULL DEFAULT 0 COMMENT '周期天数',
    `price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '价格',
    `original_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '原价',
    `features` json DEFAULT NULL COMMENT '功能列表',
    `max_devices` int(11) NOT NULL DEFAULT 1 COMMENT '最大设备数',
    `max_users` int(11) NOT NULL DEFAULT 0 COMMENT '最大用户数（0表示不限）',
    `storage_limit` bigint(20) NOT NULL DEFAULT 0 COMMENT '存储限制（MB，0表示不限）',
    `api_calls_limit` bigint(20) NOT NULL DEFAULT 0 COMMENT 'API限制（次/天，0表示不限）',
    `support_level` tinyint(1) NOT NULL DEFAULT 0 COMMENT '支持级别（0基础 1高级 2专属）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='套餐表';

-- =====================================================
-- 6. 购物车表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_cart` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_plugin_plan` (`user_id`, `plugin_id`, `plan_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plugin_id` (`plugin_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购物车表';

-- =====================================================
-- 7. 订单表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_order` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `order_no` varchar(32) NOT NULL DEFAULT '' COMMENT '订单号',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `amount` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '支付金额',
    `original_amount` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '原价金额',
    `discount_amount` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '优惠金额',
    `coupon_id` bigint(20) DEFAULT NULL COMMENT '优惠券ID',
    `payment_method` tinyint(1) NOT NULL DEFAULT 0 COMMENT '支付方式（0微信 1支付宝 2卡密 3余额）',
    `payment_time` datetime DEFAULT NULL COMMENT '支付时间',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '状态（0待支付 1已支付 2已取消 3已退款 4已过期）',
    `expire_time` datetime NOT NULL COMMENT '订单过期时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_no` (`order_no`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_status` (`status`),
    KEY `idx_expire_time` (`expire_time`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单表';

-- =====================================================
-- 8. 订阅表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_subscription` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `order_id` bigint(20) DEFAULT NULL COMMENT '订单ID',
    `license_id` bigint(20) DEFAULT NULL COMMENT 'License ID',
    `start_time` datetime NOT NULL COMMENT '开始时间',
    `end_time` datetime NOT NULL COMMENT '到期时间',
    `auto_renew` tinyint(1) NOT NULL DEFAULT 0 COMMENT '自动续费（0关闭 1开启）',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0待生效 1生效中 2已过期 3已取消）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_end_time` (`end_time`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订阅表';

-- =====================================================
-- 9. License表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_license` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `license_key` varchar(64) NOT NULL DEFAULT '' COMMENT 'License密钥（UUID格式）',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `order_id` bigint(20) DEFAULT NULL COMMENT '订单ID',
    `device_id` varchar(128) NOT NULL DEFAULT '' COMMENT '主设备ID（设备指纹）',
    `device_name` varchar(100) DEFAULT NULL COMMENT '设备名称',
    `device_type` varchar(50) DEFAULT NULL COMMENT '设备类型',
    `os_version` varchar(50) DEFAULT NULL COMMENT '系统版本',
    `app_version` varchar(32) DEFAULT NULL COMMENT '应用版本',
    `ip_address` varchar(50) DEFAULT NULL COMMENT '注册IP',
    `bind_count` int(11) NOT NULL DEFAULT 0 COMMENT '绑定次数',
    `max_devices` int(11) NOT NULL DEFAULT 1 COMMENT '最大设备数',
    `verify_count` int(11) NOT NULL DEFAULT 0 COMMENT '验证次数',
    `last_verify_time` datetime DEFAULT NULL COMMENT '最后验证时间',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用 2过期 3注销）',
    `start_time` datetime NOT NULL COMMENT '有效期开始',
    `end_time` datetime NOT NULL COMMENT '有效期截止',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_license_key` (`license_key`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_device_id` (`device_id`),
    KEY `idx_status` (`status`),
    KEY `idx_end_time` (`end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='License表';

-- =====================================================
-- 10. 设备表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_device` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `license_id` bigint(20) NOT NULL DEFAULT 0 COMMENT 'License ID',
    `device_id` varchar(128) NOT NULL DEFAULT '' COMMENT '设备ID（设备指纹）',
    `device_name` varchar(100) DEFAULT NULL COMMENT '设备名称',
    `device_type` varchar(50) DEFAULT NULL COMMENT '设备类型',
    `os_version` varchar(50) DEFAULT NULL COMMENT '系统版本',
    `mac_address` varchar(64) DEFAULT NULL COMMENT 'MAC地址',
    `ip_address` varchar(50) DEFAULT NULL COMMENT 'IP地址',
    `last_active_time` datetime DEFAULT NULL COMMENT '最后活跃时间',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0离线 1在线）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_license_device` (`license_id`, `device_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_device_id` (`device_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备表';

-- =====================================================
-- 11. 验证码表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_verification_code` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `code` varchar(10) NOT NULL DEFAULT '' COMMENT '验证码（6位数字）',
    `license_id` bigint(20) NOT NULL DEFAULT 0 COMMENT 'License ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `purpose` tinyint(1) NOT NULL DEFAULT 0 COMMENT '用途（0激活 1登录 2心跳 3更新）',
    `device_hash` varchar(64) DEFAULT NULL COMMENT '设备哈希',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '状态（0未用 1已用 2过期 3封禁）',
    `attempts` int(11) NOT NULL DEFAULT 0 COMMENT '尝试次数',
    `expire_time` datetime NOT NULL COMMENT '过期时间',
    `verified_at` datetime DEFAULT NULL COMMENT '验证时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_code` (`code`),
    KEY `idx_license_id` (`license_id`),
    KEY `idx_expire_time` (`expire_time`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='验证码表';

-- =====================================================
-- 12. 卡密批次表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_card_batch` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `batch_no` varchar(32) NOT NULL DEFAULT '' COMMENT '批次号',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `total_count` int(11) NOT NULL DEFAULT 0 COMMENT '生成数量',
    `used_count` int(11) NOT NULL DEFAULT 0 COMMENT '已使用数量',
    `price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '卡密售价',
    `expire_time` datetime NOT NULL COMMENT '卡密过期时间',
    `creator_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '创建人',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_batch_no` (`batch_no`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_plan_id` (`plan_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='卡密批次表';

-- =====================================================
-- 13. 卡密表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_card` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `card_no` varchar(32) NOT NULL DEFAULT '' COMMENT '卡号',
    `card_pwd` varchar(64) NOT NULL DEFAULT '' COMMENT '卡密',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `plan_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '套餐ID',
    `batch_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '批次ID',
    `face_value` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '面值（代表使用期限）',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '状态（0未使用 1已使用 2已过期 3已冻结）',
    `used_user_id` bigint(20) DEFAULT NULL COMMENT '使用用户ID',
    `used_order_id` bigint(20) DEFAULT NULL COMMENT '使用订单ID',
    `used_time` datetime DEFAULT NULL COMMENT '使用时间',
    `expire_time` datetime NOT NULL COMMENT '卡密过期时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_card_no` (`card_no`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_batch_id` (`batch_id`),
    KEY `idx_status` (`status`),
    KEY `idx_expire_time` (`expire_time`),
    KEY `idx_used_user_id` (`used_user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='卡密表';

-- =====================================================
-- 14. 插件评论表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_plugin_review` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `rating` tinyint(1) NOT NULL DEFAULT 5 COMMENT '评分（1-5）',
    `content` text COMMENT '评论内容',
    `reply_content` text COMMENT '开发者回复内容',
    `reply_time` datetime DEFAULT NULL COMMENT '回复时间',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0隐藏 1显示）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_plugin_id` (`plugin_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_status` (`status`),
    KEY `idx_rating` (`rating`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件评论表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `p_plugin_review`;
-- DROP TABLE IF EXISTS `p_card`;
-- DROP TABLE IF EXISTS `p_card_batch`;
-- DROP TABLE IF EXISTS `p_verification_code`;
-- DROP TABLE IF EXISTS `p_device`;
-- DROP TABLE IF EXISTS `p_license`;
-- DROP TABLE IF EXISTS `p_subscription`;
-- DROP TABLE IF EXISTS `p_order`;
-- DROP TABLE IF EXISTS `p_cart`;
-- DROP TABLE IF EXISTS `p_plan`;
-- DROP TABLE IF EXISTS `p_plugin_version`;
-- DROP TABLE IF EXISTS `p_plugin`;
-- DROP TABLE IF EXISTS `p_developer`;
-- DROP TABLE IF EXISTS `p_plugin_category`;