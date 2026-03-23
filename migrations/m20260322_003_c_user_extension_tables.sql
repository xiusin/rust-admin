-- =====================================================
-- 用户模块扩展表结构
-- 创建日期: 2026-03-22
-- 描述: 第三方授权绑定、用户等级、用户标签、用户禁用
-- =====================================================

-- 1. 第三方授权绑定表
CREATE TABLE IF NOT EXISTS `c_user_oauth` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `oauth_type` varchar(20) NOT NULL COMMENT '授权类型: wechat/github/qq/weibo/apple',
    `oauth_id` varchar(128) NOT NULL COMMENT '第三方平台用户ID',
    `oauth_name` varchar(100) DEFAULT NULL COMMENT '第三方平台用户名',
    `oauth_avatar` varchar(500) DEFAULT NULL COMMENT '第三方平台头像',
    `oauth_token` text DEFAULT NULL COMMENT '第三方访问令牌(加密存储)',
    `refresh_token` text DEFAULT NULL COMMENT '刷新令牌(加密存储)',
    `token_expires_at` datetime DEFAULT NULL COMMENT '令牌过期时间',
    `bind_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '绑定时间',
    `unbind_at` datetime DEFAULT NULL COMMENT '解绑时间',
    `is_primary` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否主绑定账号',
    `status` varchar(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/unbind',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_oauth_type_id` (`oauth_type`, `oauth_id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_oauth_type` (`oauth_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户第三方授权绑定表';

-- 2. 用户等级配置表
CREATE TABLE IF NOT EXISTS `c_user_level_config` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `level` int(11) NOT NULL COMMENT '等级',
    `level_name` varchar(50) NOT NULL COMMENT '等级名称',
    `min_exp` int(11) NOT NULL DEFAULT 0 COMMENT '升级所需最小经验值',
    `max_exp` int(11) DEFAULT NULL COMMENT '当前等级最大经验值(为空表示无上限)',
    `icon` varchar(500) DEFAULT NULL COMMENT '等级图标',
    `color` varchar(20) DEFAULT NULL COMMENT '等级颜色',
    `discount_rate` decimal(5,2) DEFAULT 100.00 COMMENT '折扣率(百分比)',
    `privileges` json DEFAULT NULL COMMENT '等级特权配置',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `sort_order` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_level` (`level`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户等级配置表';

-- 3. 用户等级表
CREATE TABLE IF NOT EXISTS `c_user_level` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `level` int(11) NOT NULL DEFAULT 1 COMMENT '当前等级',
    `exp` int(11) NOT NULL DEFAULT 0 COMMENT '当前经验值',
    `total_exp` bigint(20) NOT NULL DEFAULT 0 COMMENT '累计经验值',
    `level_up_at` datetime DEFAULT NULL COMMENT '最近升级时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_consumer_id` (`consumer_id`),
    KEY `idx_level` (`level`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户等级表';

-- 4. 用户等级变动记录表
CREATE TABLE IF NOT EXISTS `c_user_level_record` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `old_level` int(11) DEFAULT NULL COMMENT '原等级',
    `new_level` int(11) NOT NULL COMMENT '新等级',
    `old_exp` int(11) DEFAULT NULL COMMENT '原经验值',
    `new_exp` int(11) NOT NULL COMMENT '新经验值',
    `exp_change` int(11) NOT NULL COMMENT '经验值变化量',
    `change_type` varchar(20) NOT NULL COMMENT '变动类型: level_up/level_down/exp_add/exp_reduce',
    `source` varchar(50) DEFAULT NULL COMMENT '来源: order/login/activity/manual',
    `source_id` varchar(64) DEFAULT NULL COMMENT '来源ID',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户等级变动记录表';

-- 5. 用户标签表
CREATE TABLE IF NOT EXISTS `c_user_tag` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(50) NOT NULL COMMENT '标签名称',
    `tag_type` varchar(20) NOT NULL DEFAULT 'custom' COMMENT '标签类型: system/custom',
    `category` varchar(50) DEFAULT NULL COMMENT '标签分类',
    `color` varchar(20) DEFAULT NULL COMMENT '标签颜色',
    `icon` varchar(100) DEFAULT NULL COMMENT '标签图标',
    `description` varchar(500) DEFAULT NULL COMMENT '标签描述',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `sort_order` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_by` bigint(20) DEFAULT NULL COMMENT '创建人',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name` (`name`),
    KEY `idx_tag_type` (`tag_type`),
    KEY `idx_category` (`category`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户标签表';

-- 6. 用户标签关联表
CREATE TABLE IF NOT EXISTS `c_user_tag_relation` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `tag_id` bigint(20) NOT NULL COMMENT '标签ID',
    `source` varchar(50) DEFAULT NULL COMMENT '来源: manual/auto',
    `source_desc` varchar(200) DEFAULT NULL COMMENT '来源描述',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_consumer_tag` (`consumer_id`, `tag_id`),
    KEY `idx_tag_id` (`tag_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户标签关联表';

-- 7. 用户禁用记录表
CREATE TABLE IF NOT EXISTS `c_user_ban` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `ban_type` varchar(20) NOT NULL COMMENT '禁用类型: temporary/permanent',
    `reason` varchar(500) NOT NULL COMMENT '禁用原因',
    `start_at` datetime NOT NULL COMMENT '禁用开始时间',
    `end_at` datetime DEFAULT NULL COMMENT '禁用结束时间(永久禁用为空)',
    `banned_by` bigint(20) DEFAULT NULL COMMENT '禁用操作人',
    `unban_at` datetime DEFAULT NULL COMMENT '解禁时间',
    `unban_by` bigint(20) DEFAULT NULL COMMENT '解禁操作人',
    `unban_reason` varchar(500) DEFAULT NULL COMMENT '解禁原因',
    `status` varchar(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/expired/cancelled',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_status` (`status`),
    KEY `idx_end_at` (`end_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户禁用记录表';

-- 8. 消费统计表
CREATE TABLE IF NOT EXISTS `c_consumer_statistics` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `total_consume` decimal(15,2) NOT NULL DEFAULT 0.00 COMMENT '累计消费金额',
    `month_consume` decimal(15,2) NOT NULL DEFAULT 0.00 COMMENT '本月消费金额',
    `year_consume` decimal(15,2) NOT NULL DEFAULT 0.00 COMMENT '本年消费金额',
    `order_count` int(11) NOT NULL DEFAULT 0 COMMENT '订单总数',
    `refund_count` int(11) NOT NULL DEFAULT 0 COMMENT '退款总数',
    `refund_amount` decimal(15,2) NOT NULL DEFAULT 0.00 COMMENT '累计退款金额',
    `last_order_at` datetime DEFAULT NULL COMMENT '最近下单时间',
    `statistics_month` varchar(7) DEFAULT NULL COMMENT '统计月份(YYYY-MM)',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_consumer_month` (`consumer_id`, `statistics_month`),
    KEY `idx_statistics_month` (`statistics_month`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='消费统计表';

-- 插入默认等级配置
INSERT INTO `c_user_level_config` (`id`, `level`, `level_name`, `min_exp`, `max_exp`, `color`, `discount_rate`, `sort_order`) VALUES
(1, 1, '普通会员', 0, 100, '#999999', 100.00, 1),
(2, 2, '铜牌会员', 100, 500, '#CD7F32', 98.00, 2),
(3, 3, '银牌会员', 500, 2000, '#C0C0C0', 95.00, 3),
(4, 4, '金牌会员', 2000, 5000, '#FFD700', 92.00, 4),
(5, 5, '白金会员', 5000, 10000, '#E5E4E2', 88.00, 5),
(6, 6, '钻石会员', 10000, NULL, '#B9F2FF', 85.00, 6);

-- 插入默认系统标签
INSERT INTO `c_user_tag` (`id`, `name`, `tag_type`, `category`, `color`, `description`, `sort_order`) VALUES
(1, '新用户', 'system', 'lifecycle', '#52c41a', '注册30天内的新用户', 1),
(2, '活跃用户', 'system', 'lifecycle', '#1890ff', '近期活跃的用户', 2),
(3, '高价值用户', 'system', 'value', '#faad14', '消费金额较高的用户', 3),
(4, 'VIP用户', 'system', 'value', '#eb2f96', 'VIP等级用户', 4),
(5, '风险用户', 'system', 'risk', '#ff4d4f', '存在风险行为的用户', 5);
