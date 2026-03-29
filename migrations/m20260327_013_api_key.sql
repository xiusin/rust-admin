-- m20260327_013_api_key.sql
-- 描述：创建PPT API密钥表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_api_key` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `name` varchar(100) NOT NULL COMMENT '密钥名称',
    `api_key` varchar(64) NOT NULL COMMENT 'API密钥',
    `api_secret` varchar(128) NOT NULL COMMENT 'API密钥密文',
    `permissions` json DEFAULT NULL COMMENT '权限列表',
    `rate_limit` int(11) NOT NULL DEFAULT 100 COMMENT '速率限制(次/分钟)',
    `daily_limit` int(11) NOT NULL DEFAULT 1000 COMMENT '每日限制(次)',
    `daily_used` int(11) NOT NULL DEFAULT 0 COMMENT '今日已用次数',
    `total_requests` bigint(20) NOT NULL DEFAULT 0 COMMENT '总请求数',
    `success_requests` bigint(20) NOT NULL DEFAULT 0 COMMENT '成功请求数',
    `failed_requests` bigint(20) NOT NULL DEFAULT 0 COMMENT '失败请求数',
    `last_used_at` datetime DEFAULT NULL COMMENT '最后使用时间',
    `last_reset_at` datetime DEFAULT NULL COMMENT '最后重置时间',
    `expires_at` datetime DEFAULT NULL COMMENT '过期时间',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_api_key` (`api_key`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_is_active` (`is_active`),
    KEY `idx_expires_at` (`expires_at`),
    CONSTRAINT `fk_api_key_user` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT API密钥表';

CREATE TABLE IF NOT EXISTS `ppt_api_usage_log` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `api_key_id` bigint(20) NOT NULL COMMENT 'API密钥ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `endpoint` varchar(200) NOT NULL COMMENT '请求端点',
    `method` varchar(10) NOT NULL COMMENT '请求方法',
    `status_code` int(11) NOT NULL COMMENT 'HTTP状态码',
    `response_time_ms` int(11) NOT NULL COMMENT '响应时间(毫秒)',
    `tokens_used` int(11) NOT NULL DEFAULT 0 COMMENT '消耗Token数',
    `error_message` text COMMENT '错误信息',
    `ip_address` varchar(50) DEFAULT NULL COMMENT 'IP地址',
    `user_agent` varchar(500) DEFAULT NULL COMMENT '用户代理',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_api_key_id` (`api_key_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_endpoint` (`endpoint`),
    KEY `idx_created_at` (`created_at`),
    CONSTRAINT `fk_api_usage_log_key` FOREIGN KEY (`api_key_id`) REFERENCES `ppt_api_key` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT API使用日志表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_api_usage_log`;
-- DROP TABLE IF EXISTS `ppt_api_key`;
