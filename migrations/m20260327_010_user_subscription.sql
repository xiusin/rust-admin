-- m20260327_010_user_subscription.sql
-- 描述：创建PPT用户订阅表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_user_subscription` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
    `order_id` bigint(20) DEFAULT NULL COMMENT '订单ID',
    `started_at` datetime NOT NULL COMMENT '开始时间',
    `expires_at` datetime NOT NULL COMMENT '过期时间',
    `ai_credits_total` int(11) NOT NULL DEFAULT 0 COMMENT 'AI额度总量',
    `ai_credits_used` int(11) NOT NULL DEFAULT 0 COMMENT 'AI额度已使用',
    `ai_credits_remaining` int(11) NOT NULL DEFAULT 0 COMMENT 'AI额度剩余',
    `projects_count` int(11) NOT NULL DEFAULT 0 COMMENT '项目数量',
    `auto_renew` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否自动续费',
    `status` varchar(20) NOT NULL DEFAULT 'active' COMMENT '状态(active/expired/cancelled)',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plan_id` (`plan_id`),
    KEY `idx_order_id` (`order_id`),
    KEY `idx_status` (`status`),
    KEY `idx_expires_at` (`expires_at`),
    CONSTRAINT `fk_user_subscription_user` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`) ON DELETE CASCADE,
    CONSTRAINT `fk_user_subscription_plan` FOREIGN KEY (`plan_id`) REFERENCES `ppt_subscription_plan` (`id`) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT用户订阅表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_user_subscription`;
