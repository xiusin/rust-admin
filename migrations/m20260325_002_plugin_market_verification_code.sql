-- m20260325_002_plugin_market_verification_code.sql
-- 描述：插件市场验证码表
-- 作者：tuoke
-- 日期：2026-03-25

CREATE TABLE IF NOT EXISTS `p_plugin_verification_code` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `code` varchar(16) NOT NULL DEFAULT '' COMMENT '验证码',
    `license_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '许可证ID',
    `user_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户ID',
    `plugin_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '插件ID',
    `purpose` tinyint(1) NOT NULL DEFAULT 0 COMMENT '用途（0激活 1重置）',
    `device_hash` varchar(128) DEFAULT NULL COMMENT '设备哈希',
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='插件市场验证码表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `p_plugin_verification_code`;
