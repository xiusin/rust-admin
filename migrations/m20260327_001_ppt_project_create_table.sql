-- m20260327_001_ppt_project_create_table.sql
-- 描述：创建PPT项目表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_project` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `title` varchar(200) NOT NULL COMMENT '项目标题',
    `description` text COMMENT '项目描述',
    `source_type` varchar(20) NOT NULL DEFAULT 'md' COMMENT '来源类型(ppt/word/pdf/md)',
    `source_file` varchar(500) DEFAULT NULL COMMENT '源文件路径',
    `style_template_id` bigint(20) DEFAULT NULL COMMENT '风格模板ID',
    `industry` varchar(50) DEFAULT NULL COMMENT '识别的行业',
    `industry_confidence` decimal(5,4) DEFAULT NULL COMMENT '行业置信度',
    `status` varchar(20) NOT NULL DEFAULT 'draft' COMMENT '状态(draft/generating/completed)',
    `metadata` json DEFAULT NULL COMMENT '元数据',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT项目表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_project`;
