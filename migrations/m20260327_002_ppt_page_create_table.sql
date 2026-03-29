-- m20260327_002_ppt_page_create_table.sql
-- 描述：创建PPT页面表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_page` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `project_id` bigint(20) NOT NULL COMMENT '项目ID',
    `page_order` int(11) NOT NULL DEFAULT 0 COMMENT '页面顺序',
    `page_type` varchar(20) NOT NULL DEFAULT 'content' COMMENT '页面类型(title/toc/content/chart/end)',
    `title` varchar(200) DEFAULT NULL COMMENT '页面标题',
    `layout_config` json DEFAULT NULL COMMENT '布局配置',
    `style_config` json DEFAULT NULL COMMENT '样式配置',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_project_id` (`project_id`),
    KEY `idx_page_order` (`page_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT页面表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_page`;
