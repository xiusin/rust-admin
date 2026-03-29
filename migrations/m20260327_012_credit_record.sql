-- m20260327_012_credit_record.sql
-- 描述：创建PPT额度记录表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_credit_record` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `subscription_id` bigint(20) DEFAULT NULL COMMENT '订阅ID',
    `project_id` bigint(20) DEFAULT NULL COMMENT '项目ID',
    `task_type` varchar(50) NOT NULL COMMENT '任务类型(generate/edit/export/api)',
    `amount` int(11) NOT NULL COMMENT '额度变动(正数增加,负数减少)',
    `balance_before` int(11) NOT NULL COMMENT '变动前余额',
    `balance_after` int(11) NOT NULL COMMENT '变动后余额',
    `source` varchar(20) NOT NULL COMMENT '来源(subscription/purchase/gift/refund/consume)',
    `description` varchar(500) DEFAULT NULL COMMENT '描述',
    `metadata` json DEFAULT NULL COMMENT '元数据',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_subscription_id` (`subscription_id`),
    KEY `idx_project_id` (`project_id`),
    KEY `idx_task_type` (`task_type`),
    KEY `idx_source` (`source`),
    KEY `idx_created_at` (`created_at`),
    CONSTRAINT `fk_credit_record_user` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT额度记录表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_credit_record`;
