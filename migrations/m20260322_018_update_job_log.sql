-- m20260322_018_update_job_log.sql
-- 描述：更新任务日志表字段，添加任务名称、分组等字段
-- 作者：tuoke
-- 日期：2026-03-22

-- 删除旧表重新创建（因为字段变更较大）
DROP TABLE IF EXISTS `sys_job_log`;

-- 创建新的任务日志表
CREATE TABLE `sys_job_log` (
    `log_id` bigint(20) NOT NULL COMMENT '日志ID',
    `job_id` bigint(20) NOT NULL COMMENT '任务ID',
    `job_name` varchar(100) NOT NULL DEFAULT '' COMMENT '任务名称',
    `job_group` varchar(50) NOT NULL DEFAULT '' COMMENT '任务分组',
    `run_count` int(11) NOT NULL DEFAULT '0' COMMENT '执行次数',
    `job_message` text COMMENT '执行结果消息',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '执行状态（0成功 1失败）',
    `elapsed_time` int(11) NOT NULL DEFAULT '0' COMMENT '执行耗时(毫秒)',
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`log_id`),
    KEY `idx_job_id` (`job_id`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='定时任务日志表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `sys_job_log`;
