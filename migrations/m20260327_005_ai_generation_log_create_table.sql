-- m20260327_005_ai_generation_log_create_table.sql
-- 描述：创建AI生成日志表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_ai_generation_log` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `project_id` bigint(20) DEFAULT NULL COMMENT '项目ID',
    `task_type` varchar(50) NOT NULL COMMENT '任务类型',
    `ai_provider` varchar(50) NOT NULL COMMENT 'AI服务商',
    `model` varchar(100) NOT NULL COMMENT '模型名称',
    `input_data` json DEFAULT NULL COMMENT '输入数据',
    `output_data` json DEFAULT NULL COMMENT '输出数据',
    `tokens_used` int(11) NOT NULL DEFAULT 0 COMMENT '使用token数',
    `cost` decimal(10,4) DEFAULT NULL COMMENT '成本',
    `duration_ms` int(11) NOT NULL DEFAULT 0 COMMENT '耗时(毫秒)',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态(pending/processing/completed/failed)',
    `error_message` text DEFAULT NULL COMMENT '错误信息',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_project_id` (`project_id`),
    KEY `idx_task_type` (`task_type`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT AI生成日志表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_ai_generation_log`;
