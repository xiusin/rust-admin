-- m20260327_003_page_element_create_table.sql
-- 描述：创建页面元素表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_page_element` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `page_id` bigint(20) NOT NULL COMMENT '页面ID',
    `element_type` varchar(30) NOT NULL COMMENT '元素类型(text/image/shape/chart/table)',
    `content` json DEFAULT NULL COMMENT '内容数据',
    `style` json DEFAULT NULL COMMENT '样式数据',
    `position` json DEFAULT NULL COMMENT '位置数据{x,y}',
    `size` json DEFAULT NULL COMMENT '尺寸数据{width,height}',
    `rotation` decimal(5,2) DEFAULT 0 COMMENT '旋转角度',
    `z_index` int(11) NOT NULL DEFAULT 0 COMMENT '层级',
    `locked` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否锁定',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_page_id` (`page_id`),
    KEY `idx_element_type` (`element_type`),
    KEY `idx_z_index` (`z_index`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT页面元素表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_page_element`;
