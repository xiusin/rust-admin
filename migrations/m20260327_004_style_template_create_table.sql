-- m20260327_004_style_template_create_table.sql
-- 描述：创建风格模板表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_style_template` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '模板名称',
    `industry` varchar(50) DEFAULT NULL COMMENT '适用行业',
    `color_scheme` json DEFAULT NULL COMMENT '颜色方案',
    `font_scheme` json DEFAULT NULL COMMENT '字体方案',
    `layout_rules` json DEFAULT NULL COMMENT '布局规则',
    `component_styles` json DEFAULT NULL COMMENT '组件样式',
    `preview_url` varchar(500) DEFAULT NULL COMMENT '预览图URL',
    `usage_count` int(11) NOT NULL DEFAULT 0 COMMENT '使用次数',
    `is_system` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否系统模板',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态(0禁用1启用)',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_industry` (`industry`),
    KEY `idx_usage_count` (`usage_count`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT风格模板表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_style_template`;
